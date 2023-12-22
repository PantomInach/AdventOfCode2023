use num::integer::lcm;
use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

trait Gate {
    fn trigger(&mut self) -> Option<bool>;

    fn recive(&mut self, pulse: bool, from: String);

    fn get_outputs(&self) -> &Vec<String>;

    fn set_inputs(&mut self, inputs: &[String]);
}

#[derive(Debug)]
struct Broadcaster {
    outputs: Vec<String>,
}

impl Gate for Broadcaster {
    fn trigger(&mut self) -> Option<bool> {
        Some(false)
    }

    fn recive(&mut self, _: bool, _: String) {}

    fn get_outputs(&self) -> &Vec<String> {
        &self.outputs
    }

    fn set_inputs(&mut self, _: &[String]) {}
}

#[derive(Debug)]
struct FlipFlop {
    state: bool,
    will_fire: bool,
    outputs: Vec<String>,
}

impl Gate for FlipFlop {
    fn trigger(&mut self) -> Option<bool> {
        if self.will_fire {
            self.will_fire = false;
            Some(self.state)
        } else {
            None
        }
    }

    fn recive(&mut self, pulse: bool, _: String) {
        if !pulse {
            self.state = !self.state;
            self.will_fire = true;
        }
    }

    fn get_outputs(&self) -> &Vec<String> {
        &self.outputs
    }

    fn set_inputs(&mut self, _: &[String]) {}
}

#[derive(Debug)]
struct Conjunction {
    memory: HashMap<String, bool>,
    outputs: Vec<String>,
}

impl Gate for Conjunction {
    fn trigger(&mut self) -> Option<bool> {
        Some(!self.memory.iter().all(|(_, x)| *x))
    }

    fn recive(&mut self, pulse: bool, from: String) {
        self.memory.insert(from, pulse);
    }

    fn get_outputs(&self) -> &Vec<String> {
        &self.outputs
    }
    fn set_inputs(&mut self, inputs: &[String]) {
        self.memory = inputs.iter().map(|s| (s.clone(), false)).collect();
    }
}

struct Network {
    modules: HashMap<String, Box<dyn Gate>>,
    low_pulses: u64,
    high_pulses: u64,
}

impl FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
        let mut modules: HashMap<String, Box<dyn Gate>> = s
            .lines()
            .map(|l| -> (String, Box<dyn Gate>) {
                let (name_str, outputs_str) = l.split_once(" -> ").unwrap();
                let outputs = outputs_str
                    .split(", ")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                let mut name = name_str.get(1..).unwrap().to_string();
                if name == "roadcaster" {
                    name = "broadcaster".to_string();
                }
                outputs.iter().for_each(|out| {
                    if let Some(v) = inputs.get_mut(out) {
                        v.push(name.clone());
                    } else {
                        inputs.insert(out.to_string(), vec![name.clone()]);
                    }
                });
                match name_str.get(0..1) {
                    Some("b") => ("broadcaster".to_string(), Box::new(Broadcaster { outputs })),
                    Some("%") => (
                        name,
                        Box::new(FlipFlop {
                            outputs,
                            state: false,
                            will_fire: false,
                        }),
                    ),
                    Some("&") => (
                        name,
                        Box::new(Conjunction {
                            memory: HashMap::new(),
                            outputs,
                        }),
                    ),
                    _ => unreachable!("Invalid line given."),
                }
            })
            .collect();
        modules
            .iter_mut()
            .for_each(|(s, g)| g.set_inputs(inputs.get_mut(s).unwrap_or(&mut vec![])));
        Ok(Network {
            modules,
            low_pulses: 0,
            high_pulses: 0,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Signal {
    Recive((String, bool, String)),
    Trigger(String),
}

impl Network {
    fn once(&mut self, listen_for: Option<String>) -> bool {
        self.low_pulses += 1;
        let mut queue = VecDeque::new();
        queue.push_front(Signal::Trigger("broadcaster".to_string()));
        while let Some(signal) = queue.pop_front() {
            match signal {
                Signal::Recive((name, pulse, from)) => {
                    if listen_for.clone().is_some_and(|s| s == name) && !pulse {
                        return true;
                    }
                    if pulse {
                        self.high_pulses += 1;
                    } else {
                        self.low_pulses += 1;
                    }
                    if let Some(module) = self.modules.get_mut(&name) {
                        module.recive(pulse, from);
                    }
                }
                Signal::Trigger(name) => {
                    if let Some(module) = self.modules.get_mut(&name) {
                        if let Some(pulse) = module.trigger() {
                            module.get_outputs().iter().for_each(|s| {
                                queue.push_front(Signal::Recive((
                                    s.to_string(),
                                    pulse,
                                    name.to_string(),
                                )));
                                let trigger = Signal::Trigger(s.to_string());
                                queue.push_back(Signal::Trigger(s.to_string()));
                                if !queue.contains(&trigger) {}
                            })
                        }
                    }
                }
            }
        }
        false
    }

    fn get_first_high(&mut self, module: String) -> u64 {
        (1..).find(|_| self.once(Some(module.clone()))).unwrap()
    }
}

pub fn process_part1(input: &str) -> u64 {
    let mut network = Network::from_str(input).unwrap();
    // (0..1).for_each(|_| network.once());
    (0..1000).for_each(|_| {
        network.once(None);
    });
    network.high_pulses * network.low_pulses
}

pub fn process_part2(input: &str) -> u64 {
    ["js", "qs", "dt", "ts"]
        .iter()
        .map(|s| {
            Network::from_str(input)
                .unwrap()
                .get_first_high(s.to_string())
        })
        .reduce(lcm)
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(32000000_u64, process_part1(input));
    }

    #[test]
    fn test_process_part1_v2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(11687500_u64, process_part1(input));
    }

    #[test]
    fn test_process_part1_simple() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        let mut network = Network::from_str(input).unwrap();
        network.once(None);
        assert_eq!(network.low_pulses, 8);
        assert_eq!(network.high_pulses, 4);
    }

    #[test]
    fn test_process_part1_simple_v2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        let mut network = Network::from_str(input).unwrap();
        network.once(None);
        assert_eq!(network.low_pulses, 4);
        assert_eq!(network.high_pulses, 4);
        network.once(None);
        assert_eq!(network.low_pulses, 8);
        assert_eq!(network.high_pulses, 6);
        network.once(None);
        assert_eq!(network.low_pulses, 13);
        assert_eq!(network.high_pulses, 9);
        network.once(None);
        assert_eq!(network.low_pulses, 17);
        assert_eq!(network.high_pulses, 11);
    }

    #[test]
    fn test_flip_flop() {
        let mut ff = FlipFlop {
            state: false,
            will_fire: false,
            outputs: vec!["test".to_string()],
        };
        ff.recive(false, "b".to_string());
        assert_eq!(ff.trigger(), Some(true));
    }
}
