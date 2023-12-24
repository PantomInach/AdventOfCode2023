use minilp::Problem;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug)]
struct Velocity {
    vx: f64,
    vy: f64,
    vz: f64,
}

#[derive(Debug)]
struct Hailstorm {
    pos: Position,
    vel: Velocity,
}

impl FromStr for Hailstorm {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos_str, vel_str) = s.split_once(" @ ").unwrap();
        let pos_split: Vec<_> = pos_str.split(", ").flat_map(|n| n.parse::<f64>()).collect();
        let pos = Position {
            x: *pos_split.first().unwrap(),
            y: *pos_split.get(1).unwrap(),
            z: *pos_split.get(2).unwrap(),
        };
        let vel_split: Vec<_> = vel_str.split(", ").flat_map(|n| n.parse::<f64>()).collect();
        let vel = Velocity {
            vx: *vel_split.first().unwrap(),
            vy: *vel_split.get(1).unwrap(),
            vz: *vel_split.get(2).unwrap(),
        };
        Ok(Hailstorm { pos, vel })
    }
}

impl Hailstorm {
    fn intersetion_xy(&self, other: &Hailstorm, lower: f64, upper: f64) -> bool {
        let mut problem = Problem::new(minilp::OptimizationDirection::Minimize);
        let t1 = problem.add_var(0.0, (0.0, f64::INFINITY));
        let t2 = problem.add_var(0.0, (0.0, f64::INFINITY));
        let constant = problem.add_var(0.0, (1.0, 1.0));

        problem.add_constraint(
            &[
                (t1, self.vel.vx),
                (t2, -other.vel.vx),
                (constant, self.pos.x - other.pos.x),
            ],
            minilp::ComparisonOp::Eq,
            0.0,
        );
        problem.add_constraint(
            &[
                (t1, self.vel.vy),
                (t2, -other.vel.vy),
                (constant, self.pos.y - other.pos.y),
            ],
            minilp::ComparisonOp::Eq,
            0.0,
        );
        problem
            .solve()
            .is_ok_and(|sol| self.inbound_xy_at(sol[t1], lower, upper))
    }

    fn pos_at_time(&self, t: f64) -> Position {
        Position {
            x: self.pos.x + self.vel.vx * t,
            y: self.pos.y + self.vel.vy * t,
            z: self.pos.z + self.vel.vz * t,
        }
    }

    fn inbound_xy_at(&self, t: f64, lower: f64, upper: f64) -> bool {
        let pos = self.pos_at_time(t);
        lower <= pos.x && pos.x <= upper && lower <= pos.y && pos.y <= upper
    }
}

fn num_intersections_xy(input: &str, lower: f64, upper: f64) -> u64 {
    let hailstorms: Vec<Hailstorm> = input.lines().flat_map(|l| Hailstorm::from_str(l)).collect();
    (0..hailstorms.len())
        .map(|i| {
            (i + 1..hailstorms.len())
                .filter(|j| {
                    hailstorms.get(i).unwrap().intersetion_xy(
                        hailstorms.get(*j).unwrap(),
                        lower,
                        upper,
                    )
                })
                .count()
        })
        .sum::<usize>() as u64
}

pub fn process_part1(input: &str) -> u64 {
    num_intersections_xy(
        input,
        200_000_000_000_000 as i64 as f64,
        400_000_000_000_000 as i64 as f64,
    )
}

pub fn process_part2(input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3";
        assert_eq!(2_u64, num_intersections_xy(input, 7.0, 27.0));
    }

    #[test]
    fn test_process_part2() {
        let input = "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3";
        assert_eq!(47_u64, process_part2(input));
    }
}
