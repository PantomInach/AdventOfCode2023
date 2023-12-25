use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Debug)]
struct Velocity {
    vx: i64,
    vy: i64,
    vz: i64,
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
        let pos_split: Vec<_> = pos_str.split(", ").flat_map(|n| n.parse::<i64>()).collect();
        let pos = Position {
            x: *pos_split.first().unwrap(),
            y: *pos_split.get(1).unwrap(),
            z: *pos_split.get(2).unwrap(),
        };
        let vel_split: Vec<_> = vel_str.split(", ").flat_map(|n| n.parse::<i64>()).collect();
        let vel = Velocity {
            vx: *vel_split.first().unwrap(),
            vy: *vel_split.get(1).unwrap(),
            vz: *vel_split.get(2).unwrap(),
        };
        Ok(Hailstorm { pos, vel })
    }
}

impl Hailstorm {
    fn intersetion_xy(&self, other: &Hailstorm, lower: i64, upper: i64) -> bool {
        // One hailstorm in the xy-plan =^= (px + vx * t, py + vy * t) can be described by a line
        // in the form y = mx + t.
        // For t = 0 => (px, py); t = 1 => (px + vx, py + vy)
        // It follows m = delta_y / delta_x = vy / vx
        // For (px, py), we can deduce y = mx + t => py = m px + t => t = py - m px
        // With this we get the line.
        // Then check where the two hailstorm lines intersect.
        let m1 = self.vel.vy as f64 / self.vel.vx as f64;
        let m2 = other.vel.vy as f64 / other.vel.vx as f64;
        let t1 = self.pos.y as f64 - self.pos.x as f64 * m1;
        let t2 = other.pos.y as f64 - other.pos.x as f64 * m2;
        if m1 == m2 {
            return false;
        }
        let x = (t2 - t1) / (m1 - m2);
        let y = m1 * x + t1;
        if ((x < self.pos.x as f64) && (self.vel.vx > 0))
            || ((x > self.pos.x as f64) && (self.vel.vx < 0))
            || ((x < other.pos.x as f64) && (other.vel.vx > 0))
            || ((x > other.pos.x as f64) && (other.vel.vx < 0))
        {
            return false;
        }
        lower as f64 <= x && x <= upper as f64 && lower as f64 <= y && y <= upper as f64
    }
}

fn num_intersections_xy(input: &str, lower: i64, upper: i64) -> u64 {
    let hailstorms: Vec<Hailstorm> = input.lines().flat_map(Hailstorm::from_str).collect();
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
    num_intersections_xy(input, 200_000_000_000_000_i64, 400_000_000_000_000_i64)
}

fn possible_values(delta: i64, v: i64, lower: i64, upper: i64) -> HashSet<i64> {
    (lower..=upper)
        .filter(|v2| v2 != &v && delta % (v2 - v) == 0)
        .collect()
}

pub fn process_part2(input: &str) -> u64 {
    let hailstorms: Vec<Hailstorm> = input.lines().flat_map(Hailstorm::from_str).collect();
    let lower = -1000;
    let upper = 1000;
    let mut possible_x: HashSet<i64> = (lower..=upper).collect();
    let mut possible_y: HashSet<i64> = (lower..=upper).collect();
    let mut possible_z: HashSet<i64> = (lower..=upper).collect();

    (0..hailstorms.len()).for_each(|i| {
        let h1 = hailstorms.get(i).unwrap();
        (i + 1..hailstorms.len()).for_each(|j| {
            let h2 = hailstorms.get(j).unwrap();
            if h1.vel.vx == h2.vel.vx && h1.pos.x != h2.pos.x {
                let new_x = possible_values(h1.pos.x - h2.pos.x, h1.vel.vx, lower, upper);
                if !new_x.is_empty() {
                    possible_x = possible_x.intersection(&new_x).cloned().collect();
                }
            }
            if h1.vel.vy == h2.vel.vy && h1.pos.y != h2.pos.y {
                possible_y = possible_y
                    .intersection(&possible_values(
                        h1.pos.y - h2.pos.y,
                        h1.vel.vy,
                        lower,
                        upper,
                    ))
                    .cloned()
                    .collect();
            }
            if h1.vel.vz == h2.vel.vz && h1.pos.z != h2.pos.z {
                possible_z = possible_z
                    .intersection(&possible_values(
                        h1.pos.z - h2.pos.z,
                        h1.vel.vz,
                        lower,
                        upper,
                    ))
                    .cloned()
                    .collect();
            }
        });
    });

    let vx = **possible_x.iter().collect::<Vec<&i64>>().first().unwrap();
    let vy = **possible_y.iter().collect::<Vec<&i64>>().first().unwrap();
    let vz = **possible_z.iter().collect::<Vec<&i64>>().first().unwrap();

    // When subtracting the wanted rock throw from two intersection hailstorms, then the still
    // intersect. Consider the xy-projection. This results in (x1 + vx1 * t - x - vx * t, y1 + vy1 * t - y - vy * t)
    // and (x2 + vx2 * t - x - vx * t, y2 + vy2 * t - y - vy * t).
    // The intersection point of this gives the origin x and y coordinates of the rock throw.
    let h1 = hailstorms.first().unwrap();
    let h2 = hailstorms.get(1).unwrap();

    let m1 = (h1.vel.vy - vy) as f64 / (h1.vel.vx - vx) as f64;
    let m2 = (h2.vel.vy - vy) as f64 / (h2.vel.vx - vx) as f64;
    let t1 = h1.pos.y as f64 - h1.pos.x as f64 * m1;
    let t2 = h2.pos.y as f64 - h2.pos.x as f64 * m2;
    let x = (t2 - t1) / (m1 - m2);
    let y = m1 * x + t1;

    // Now we calculate the intersection time of the x-achses of the stone throw and one of the
    // hailstorms. With this time, we can calculate the missing z coordinates, from which the stone
    // originates.
    let time = (x - h1.pos.x as f64) / (h1.vel.vx - vx) as f64;
    let z = h1.pos.z as f64 + (h1.vel.vz - vz) as f64 * time;
    (x + y + z).round() as u64
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
        assert_eq!(2_u64, num_intersections_xy(input, 7, 27));
    }
}
