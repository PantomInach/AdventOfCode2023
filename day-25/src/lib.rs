use rand::seq::SliceRandom;
use std::str::FromStr;

#[derive(Clone)]
struct Graph {
    contraction_count: Vec<usize>,
    edges: Vec<(usize, usize)>,
}

impl FromStr for Graph {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let string_edges: Vec<(&str, &str)> = s
            .lines()
            .flat_map(|l| {
                let (u, vs) = l.split_once(": ").unwrap();
                vs.split_whitespace().map(move |v| (u, v))
            })
            .collect();
        let mut names = vec![];
        string_edges.iter().for_each(|(u, v)| {
            names.push(*u);
            names.push(*v);
        });
        names.sort_unstable();
        names.dedup();
        let edges = string_edges
            .iter()
            .map(|(u_str, v_str)| {
                (
                    names.iter().position(|x| u_str == x).unwrap(),
                    names.iter().position(|x| v_str == x).unwrap(),
                )
            })
            .collect();

        Ok(Graph {
            contraction_count: vec![1; names.len()],
            edges,
        })
    }
}

impl Graph {
    fn contract(&mut self, u: usize, v: usize) {
        let v_count = self.contraction_count.get(v).unwrap().clone();
        if let Some(us) = self.contraction_count.get_mut(u) {
            *us += v_count;
        }
        if let Some(vs) = self.contraction_count.get_mut(v) {
            *vs = 0;
        }
        self.edges.retain_mut(|(r, s)| {
            if *r == v {
                *r = u;
            } else if *s == v {
                *s = u;
            }
            r != s
        });
    }

    fn karger(&mut self) {
        let mut rng = rand::thread_rng();
        (0..self.contraction_count.len() - 2).for_each(|_| {
            let (u, v) = self.edges.choose(&mut rng).unwrap();
            self.contract(*u, *v);
        });
    }

    fn karger_three_cut(&self) -> u64 {
        loop {
            let mut g = self.clone();
            g.karger();
            if g.edges.len() == 3 {
                return g
                    .contraction_count
                    .iter()
                    .filter(|x| x != &&0)
                    .product::<usize>() as u64;
            }
        }
    }
}

pub fn process_part1(input: &str) -> u64 {
    Graph::from_str(input).unwrap().karger_three_cut()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!(54_u64, process_part1(input));
    }
}
