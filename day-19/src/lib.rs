use std::{collections::HashMap, str::FromStr, u64};

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values = s[1..s.len() - 1]
            .split(',')
            .map(|x| x[2..].parse::<u64>().unwrap());
        Ok(Part {
            x: values.next().unwrap(),
            m: values.next().unwrap(),
            a: values.next().unwrap(),
            s: values.next().unwrap(),
        })
    }
}

#[derive(Debug)]
enum Comparison {
    Greater,
    Less,
}

impl Comparison {
    fn compare(&self, x1: u64, x2: u64) -> bool {
        match self {
            Comparison::Greater => x1 > x2,
            Comparison::Less => x1 < x2,
        }
    }
}

#[derive(Eq, PartialEq, Debug, Clone)]
enum NextWorkflow {
    Accept,
    Reject,
    Workflow(String),
}

impl FromStr for NextWorkflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => NextWorkflow::Accept,
            "R" => NextWorkflow::Reject,
            _ => NextWorkflow::Workflow(s.to_string()),
        })
    }
}

#[derive(Debug)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn get_from(&self, part: &Part) -> u64 {
        match self {
            Category::X => part.x,
            Category::M => part.m,
            Category::A => part.a,
            Category::S => part.s,
        }
    }
}

#[derive(Debug)]
struct Condition {
    compare_with: Category,
    comparison: Comparison,
    value: u64,
    next: NextWorkflow,
}

impl FromStr for Condition {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let compare_with = match s.get(0..=0).unwrap() {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => unimplemented!("Can't match category"),
        };
        let comparison = match s.get(1..=1).unwrap() {
            "<" => Comparison::Less,
            ">" => Comparison::Greater,
            _ => unimplemented!("No such comparision"),
        };
        let (val, n) = s.split_once(':').unwrap();
        let value = u64::from_str_radix(&val[2..], 10).unwrap();
        let next = NextWorkflow::from_str(n).unwrap();
        Ok(Condition {
            compare_with,
            comparison,
            value,
            next,
        })
    }
}

impl Condition {
    fn compare(&self, part: &Part) -> Option<&NextWorkflow> {
        let compare_value: u64 = self.compare_with.get_from(part);
        self.comparison
            .compare(compare_value, self.value)
            .then_some(&self.next)
    }
}

#[derive(Debug)]
struct Workflow {
    label: String,
    conditions: Vec<Condition>,
    next: NextWorkflow,
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lab, remains) = s.split_once('{').unwrap();
        let label = lab.to_string();
        let mut comps: Vec<&str> = remains.split(',').collect();
        let next_str = comps.pop().unwrap();
        let next = NextWorkflow::from_str(&next_str[..next_str.len() - 1]).unwrap();
        let conditions = comps
            .into_iter()
            .flat_map(Condition::from_str)
            .collect::<Vec<Condition>>();
        Ok(Workflow {
            label,
            conditions,
            next,
        })
    }
}

impl Workflow {
    fn process(&self, part: &Part) -> &NextWorkflow {
        &self
            .conditions
            .iter()
            .find_map(|cond| cond.compare(part))
            .unwrap_or(&self.next)
    }
}

struct Puzzle {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl FromStr for Puzzle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (workflows_str, parts_str) = s.split_once("\n\n").unwrap();
        let workflows: HashMap<String, Workflow> = workflows_str
            .split('\n')
            .flat_map(Workflow::from_str)
            .map(|w| (w.label.clone(), w))
            .collect();
        let parts = parts_str
            .trim()
            .split('\n')
            .flat_map(Part::from_str)
            .collect();
        Ok(Puzzle { workflows, parts })
    }
}

impl Puzzle {
    fn count_accepted(&self) -> u64 {
        self.parts
            .iter()
            .filter(|p| self.accepted(p))
            .map(|p| p.x + p.m + p.a + p.s)
            .sum::<u64>()
    }

    fn accepted(&self, part: &Part) -> bool {
        let worklow = self.workflows.get("in").unwrap();
        let mut next = worklow.process(part);
        while let NextWorkflow::Workflow(label) = next {
            next = self.workflows.get(label).unwrap().process(part);
        }
        match next {
            NextWorkflow::Accept => true,
            NextWorkflow::Reject => false,
            NextWorkflow::Workflow(_) => false,
        }
    }
}

pub fn process_part1(input: &str) -> u64 {
    Puzzle::from_str(input).unwrap().count_accepted()
}

pub fn process_part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(19114_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(167409079868000_u64, process_part2(input));
    }
}
