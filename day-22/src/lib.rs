use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Coordinates {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Coordinates {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<usize> = s.split(',').flat_map(|n| n.parse::<usize>()).collect();
        Ok(Coordinates {
            x: *nums.first().unwrap(),
            y: *nums.get(1).unwrap(),
            z: *nums.last().unwrap(),
        })
    }
}

impl Coordinates {
    fn coords(&self) -> [usize; 3] {
        [self.x, self.y, self.z]
    }

    fn move_by(&self, x: i64, y: i64, z: i64) -> Coordinates {
        Coordinates {
            x: (self.x as i64 + x) as usize,
            y: (self.y as i64 + y) as usize,
            z: (self.z as i64 + z) as usize,
        }
    }

    fn one_down(&self) -> Coordinates {
        Coordinates {
            x: self.x,
            y: self.y,
            z: self.z.saturating_sub(1),
        }
    }

    fn one_up(&self) -> Coordinates {
        Coordinates {
            x: self.x,
            y: self.y,
            z: self.z.saturating_add(1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    start: Coordinates,
    end: Coordinates,
}

impl FromStr for Brick {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start_str, end_str) = s.split_once('~').unwrap();
        Ok(Brick {
            start: Coordinates::from_str(start_str).unwrap(),
            end: Coordinates::from_str(end_str).unwrap(),
        })
    }
}

impl Brick {
    fn max_x(&self) -> usize {
        self.start.x.max(self.end.x)
    }

    fn max_y(&self) -> usize {
        self.start.y.max(self.end.y)
    }

    fn max_z(&self) -> usize {
        self.start.z.max(self.end.z)
    }

    fn move_down(&mut self) {
        self.start = self.start.one_down();
        self.end = self.end.one_down();
    }

    fn coords_iter(&self) -> Vec<Coordinates> {
        let coords_diff: Vec<i64> = self
            .start
            .coords()
            .iter()
            .zip(self.end.coords())
            .map(|(a, b)| b as i64 - *a as i64)
            .collect();
        let only_diff_op = coords_diff
            .iter()
            .find_map(|n| (*n != 0_i64).then_some(n.abs()));
        if let Some(only_diff) = only_diff_op {
            let moving_vec: Vec<i64> = coords_diff
                .iter()
                .map(|n| if *n != 0_i64 { n.signum() } else { 0 })
                .collect();
            (0..=only_diff)
                .map(|step| {
                    self.start.move_by(
                        moving_vec.first().unwrap() * step,
                        moving_vec.get(1).unwrap() * step,
                        moving_vec.get(2).unwrap() * step,
                    )
                })
                .collect()
        } else {
            vec![self.start]
        }
    }
}

#[derive(Clone)]
struct CubeGrid {
    bricks: Vec<Brick>,
    cube_grid: Vec<Vec<Vec<Option<usize>>>>,
}

impl FromStr for CubeGrid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bricks: Vec<Brick> = s.lines().flat_map(Brick::from_str).collect();
        // bricks.iter().for_each(|b| println!("{:?}", b));
        let max_x = bricks.iter().map(|b| b.max_x()).max().unwrap();
        let max_y = bricks.iter().map(|b| b.max_y()).max().unwrap();
        let max_z = bricks.iter().map(|b| b.max_z()).max().unwrap();
        // println!("{} {} {}", max_x, max_y, max_z);
        let mut cube_grid: Vec<Vec<Vec<Option<usize>>>> =
            vec![vec![vec![None; max_x + 1]; max_y + 1]; max_z + 1];

        bricks
            .iter()
            .enumerate()
            .flat_map(|(index, b)| {
                b.coords_iter()
                    .into_iter()
                    .map(|c| (index, c))
                    .collect::<Vec<(usize, Coordinates)>>()
            })
            .for_each(|(index, coords)| {
                cube_grid[coords.z][coords.y][coords.x] = Some(index);
            });

        Ok(CubeGrid { bricks, cube_grid })
    }
}

impl CubeGrid {
    fn get(&self, coords: &Coordinates) -> Option<usize> {
        *self
            .cube_grid
            .get(coords.z)
            .and_then(|square| square.get(coords.y))
            .and_then(|l| l.get(coords.x))
            .unwrap_or(&None)
    }

    fn fall_by_one(&mut self) -> Option<Vec<usize>> {
        let mut brick_falled: Vec<usize> = vec![];
        for index in 0..self.bricks.len() {
            if self.brick_can_fall(index) {
                brick_falled.push(index);
                self.move_down(index);
                if let Some(b) = self.bricks.get_mut(index) {
                    b.move_down()
                }
                // println!("{:?}", self.bricks.get(index).unwrap());
            }
        }
        if brick_falled.is_empty() {
            None
        } else {
            Some(brick_falled.into_iter().unique().collect_vec())
        }
    }

    fn brick_can_fall(&self, index: usize) -> bool {
        self.bricks
            .get(index)
            .map(|brick| {
                if brick.start.z == 0 || brick.end.z == 0 {
                    return false;
                }
                brick.coords_iter().iter().all(|coord| {
                    if let Some(other) = self.get(&coord.one_down()) {
                        other == index
                    } else {
                        true
                    }
                })
            })
            .unwrap_or(false)
    }

    fn move_down(&mut self, index: usize) {
        if let Some(brick) = self.bricks.get(index) {
            let prev_entries: Vec<Option<usize>> = brick
                .coords_iter()
                .iter()
                .map(|coord| self.get(coord))
                .collect();
            brick
                .coords_iter()
                .iter()
                .enumerate()
                .for_each(|(i, coords)| {
                    let one_down = coords.one_down();
                    self.cube_grid[one_down.z][one_down.y][one_down.x] =
                        *prev_entries.get(i).unwrap();
                    self.cube_grid[coords.z][coords.y][coords.x] = None;
                });
        }
    }

    fn let_it_fall(&mut self) -> Vec<usize> {
        let mut fallen_bricks: Vec<usize> = vec![];
        while let Some(fallen) = self.fall_by_one() {
            fallen_bricks.extend(fallen);
        }
        fallen_bricks.into_iter().unique().collect_vec()
    }

    fn supported_by(&self, index: usize) -> Option<Vec<usize>> {
        self.bricks.get(index).map(|brick| {
            brick
                .coords_iter()
                .iter()
                .flat_map(|coords| self.get(&coords.one_down()))
                .filter(|i| *i != index)
                .unique()
                .collect()
        })
    }

    fn supports(&self, index: usize) -> Option<Vec<usize>> {
        self.bricks.get(index).map(|brick| {
            brick
                .coords_iter()
                .iter()
                .flat_map(|coords| self.get(&coords.one_up()))
                .filter(|i| *i != index)
                .unique()
                .collect()
        })
    }

    fn remove_brick(&mut self, index: usize) {
        if let Some(brick) = self.bricks.get(index) {
            brick
                .coords_iter()
                .iter()
                .for_each(|coord| self.cube_grid[coord.z][coord.y][coord.x] = None);
        }
    }
}

pub fn process_part1(input: &str) -> u64 {
    let mut cube_grid = CubeGrid::from_str(input).unwrap();
    cube_grid.let_it_fall();
    let supported_by: HashMap<usize, Vec<usize>> = (0..cube_grid.bricks.len())
        .map(|index| (index, cube_grid.supported_by(index).unwrap_or(vec![])))
        .collect();
    (0..cube_grid.bricks.len())
        .filter(|i| {
            if let Some(ys) = cube_grid.supports(*i) {
                ys.iter()
                    .all(|y| supported_by.get(y).is_some_and(|l| l.len() >= 2))
            } else {
                true
            }
        })
        .count() as u64
}

pub fn process_part2(input: &str) -> u64 {
    // This is just a brute force approche.
    // It would be faster to create a graph describing which bricks depend on which bricks. Then
    // removing a brick would result in a seperate component in the graph, in which all bricks
    // would fall.
    let mut cube_grid = CubeGrid::from_str(input).unwrap();
    cube_grid.let_it_fall();
    (0..cube_grid.bricks.len())
        .flat_map(|index| {
            let mut cgc = cube_grid.clone();
            cgc.remove_brick(index);
            cgc.let_it_fall()
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(5_u64, process_part1(input));
    }

    #[test]
    fn test_coords_iter() {
        let coord1 = Coordinates { x: 1, y: 1, z: 0 };
        let coord2 = Coordinates { x: 1, y: 1, z: 4 };
        let brick = Brick {
            start: coord1,
            end: coord2,
        };
        assert_eq!(
            brick.coords_iter(),
            vec![
                Coordinates { x: 1, y: 1, z: 0 },
                Coordinates { x: 1, y: 1, z: 1 },
                Coordinates { x: 1, y: 1, z: 2 },
                Coordinates { x: 1, y: 1, z: 3 },
                Coordinates { x: 1, y: 1, z: 4 },
            ]
        );
        let brick = Brick {
            start: coord2,
            end: coord1,
        };
        assert_eq!(
            brick.coords_iter(),
            vec![
                Coordinates { x: 1, y: 1, z: 4 },
                Coordinates { x: 1, y: 1, z: 3 },
                Coordinates { x: 1, y: 1, z: 2 },
                Coordinates { x: 1, y: 1, z: 1 },
                Coordinates { x: 1, y: 1, z: 0 },
            ]
        );
    }

    #[test]
    fn test_process_part2() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(7_u64, process_part2(input));
    }
}
