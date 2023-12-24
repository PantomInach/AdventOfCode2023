use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_by(&self, x: i64, y: i64) -> Position {
        Position {
            x: (self.x as i64).saturating_add(x) as usize,
            y: (self.y as i64).saturating_add(y) as usize,
        }
    }

    fn neighbors(&self) -> [Position; 4] {
        [
            self.move_by(0, 1),
            self.move_by(0, -1),
            self.move_by(-1, 0),
            self.move_by(1, 0),
        ]
    }

    fn neighbors_with_dir(&self) -> [(Position, Direction); 4] {
        [
            (self.move_by(0, 1), Direction::Down),
            (self.move_by(0, -1), Direction::Up),
            (self.move_by(-1, 0), Direction::Left),
            (self.move_by(1, 0), Direction::Right),
        ]
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Slop(Direction),
    Forest,
    Path,
    Visited,
    Goal,
}

struct Trail {
    map: Vec<Vec<Tile>>,
}

impl FromStr for Trail {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map: Vec<Vec<Tile>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => Tile::Forest,
                        '.' => Tile::Path,
                        '>' => Tile::Slop(Direction::Right),
                        '<' => Tile::Slop(Direction::Left),
                        '^' => Tile::Slop(Direction::Up),
                        'v' => Tile::Slop(Direction::Down),
                        _ => unreachable!("Should not be in input. {:?}", c),
                    })
                    .collect()
            })
            .collect();
        map.last_mut().unwrap().iter_mut().for_each(|t| {
            if t == &mut Tile::Path {
                *t = Tile::Goal;
            }
        });
        Ok(Trail { map })
    }
}

impl Trail {
    fn build_graph(&mut self) -> Graph {
        let x = self
            .map
            .first()
            .unwrap()
            .iter()
            .position(|t| t == &Tile::Path)
            .unwrap();
        let starting_position = Position { x, y: 0 };
        let x = self
            .map
            .last()
            .unwrap()
            .iter()
            .position(|t| t == &Tile::Goal)
            .unwrap();
        let goal = Position {
            x,
            y: self.map.len() - 1,
        };

        let (intersection, dist) = self.traverse_till_intersection(&starting_position);
        let mut edges = self.handle_intersection(&intersection);
        edges.push((starting_position, intersection, dist));

        let nodes: Vec<Position> = edges
            .iter()
            .flat_map(|(u, v, _)| vec![*u, *v])
            .unique()
            .collect();
        let edges: HashMap<(Position, Position), usize> =
            edges.into_iter().map(|(u, v, d)| ((u, v), d)).collect();

        Graph {
            nodes,
            edges,
            start: starting_position,
            goal,
        }
    }

    fn set_visited(&mut self, pos: &Position) {
        if let Some(f) = self.map.get_mut(pos.y).and_then(|v| v.get_mut(pos.x)) {
            *f = Tile::Visited;
        }
    }

    fn get(&self, pos: &Position) -> Option<Tile> {
        self.map.get(pos.y).and_then(|v| v.get(pos.x).copied())
    }

    fn get_path_neighbors(&self, pos: &Position) -> Vec<Position> {
        pos.neighbors()
            .into_iter()
            .filter(|p| {
                let tile = self.get(p);
                tile == Some(Tile::Path)
                    || matches!(tile, Some(Tile::Slop(_)))
                    || tile == Some(Tile::Goal)
            })
            .collect()
    }

    fn is_intesection(&self, pos: &Position) -> bool {
        self.get(pos) == Some(Tile::Goal)
            || self
                .get_neighbors_tile(pos)
                .iter()
                .filter(|t| t == &&Tile::Visited || matches!(t, Tile::Slop(_)) || t == &&Tile::Goal)
                .count()
                >= 3
    }

    fn get_neighbors_tile(&self, pos: &Position) -> Vec<Tile> {
        pos.neighbors().iter().flat_map(|p| self.get(p)).collect()
    }

    /// Traverses from the given position in  direction until another intersection is reached.
    /// Each traversed tile will be marked visited.
    /// It returns the intersection position and how many tiles were walked. This includes itself.
    fn traverse_till_intersection(&mut self, pos: &Position) -> (Position, usize) {
        let mut walked: usize = 1;
        let mut position: Position = *pos;
        let mut path_neighbors = self.get_path_neighbors(&position);
        while !self.is_intesection(&position) {
            // if self.get(&position) == Some(Tile::Goal) {
            //     walked += 1;
            // }
            self.set_visited(&position);

            walked += 1;
            position = *path_neighbors.first().unwrap();
            path_neighbors = self.get_path_neighbors(&position);
        }

        (position, walked)
    }

    fn handle_intersection(&mut self, pos: &Position) -> Vec<(Position, Position, usize)> {
        if self.get(pos) != Some(Tile::Path) {
            return vec![];
        }
        self.set_visited(pos);

        let mut new_intersection: HashSet<Position> = HashSet::new();

        let mut edges: Vec<(Position, Position, usize)> = pos
            .neighbors_with_dir()
            .into_iter()
            .flat_map(|(p, dir)| {
                if let Some(Tile::Slop(slop_dir)) = self.get(&p) {
                    let (next_intersection, walked) = self.traverse_till_intersection(&p);
                    new_intersection.insert(next_intersection);
                    if slop_dir == dir {
                        Some((*pos, next_intersection, walked))
                    } else {
                        Some((next_intersection, *pos, walked))
                    }
                } else {
                    None
                }
            })
            .collect();

        edges.extend(
            new_intersection
                .iter()
                .flat_map(|p| self.handle_intersection(p)),
        );

        edges
    }
}

struct Graph {
    nodes: Vec<Position>,
    edges: HashMap<(Position, Position), usize>,
    start: Position,
    goal: Position,
}

impl Graph {
    fn neighbors(&self, pos: &Position) -> Vec<Position> {
        self.edges
            .keys()
            .filter_map(|(u, v)| (u == pos).then_some(*v))
            .collect()
    }

    fn undirected_neighbors(&self, pos: &Position) -> Vec<Position> {
        self.edges
            .keys()
            .filter_map(|(u, v)| {
                let e1 = (u == pos).then_some(*v);
                let e2 = (v == pos).then_some(*u);
                e1.or(e2)
            })
            .collect()
    }

    fn undirected_weight(&self, u: Position, v: Position) -> Option<usize> {
        self.edges.get(&(u, v)).or(self.edges.get(&(v, u))).copied()
    }

    fn topological_sorting_util(
        &self,
        v: Position,
        visited: &mut HashSet<Position>,
        stack: &mut Vec<Position>,
    ) {
        visited.insert(v);
        self.neighbors(&v).iter().for_each(|p| {
            if !visited.contains(p) {
                self.topological_sorting_util(*p, visited, stack)
            }
        });
        stack.push(v);
    }

    fn max_path(&self) -> Option<u64> {
        let mut dist: HashMap<Position, Option<usize>> = HashMap::new();
        let mut visited: HashSet<Position> = HashSet::new();
        let mut stack: Vec<Position> = Vec::new();
        self.nodes.iter().for_each(|v| {
            if !visited.contains(v) {
                self.topological_sorting_util(*v, &mut visited, &mut stack)
            }
        });

        dist.insert(self.start, Some(0));

        while let Some(u) = stack.pop() {
            if let Some(Some(d)) = dist.clone().get(&u) {
                self.neighbors(&u).iter().for_each(|v| {
                    let dv = dist.get(v).unwrap_or(&None);
                    let weight = self.edges.get(&(u, *v)).unwrap_or(&0);
                    if dv.is_none() || dv.unwrap() < d + weight {
                        dist.insert(*v, Some(d + weight));
                    }
                });
            }
        }
        dist.get(&self.goal).unwrap_or(&None).map(|x| x as u64)
    }

    fn brute_force_paths(
        &self,
        start: Position,
        visited: &mut HashSet<Position>,
        total: usize,
    ) -> Option<usize> {
        if visited.contains(&start) {
            return None;
        }
        if start == self.goal {
            return Some(total);
        }
        visited.insert(start);
        let res = self
            .undirected_neighbors(&start)
            .iter()
            .flat_map(|v| {
                let ew = self.undirected_weight(start, *v).unwrap();
                self.brute_force_paths(*v, visited, total + ew)
            })
            .max();

        visited.remove(&start);
        res
    }
}

pub fn process_part1(input: &str) -> u64 {
    let mut trail = Trail::from_str(input).unwrap();
    let graph = trail.build_graph();
    graph.max_path().unwrap() - 1
}

pub fn process_part2(input: &str) -> u64 {
    let mut trail = Trail::from_str(input).unwrap();
    let graph = trail.build_graph();
    graph
        .brute_force_paths(graph.start, &mut HashSet::new(), 0)
        .unwrap() as u64
        - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(94_u64, process_part1(input));
    }

    #[test]
    fn test_process_part1_v2() {
        let input = "#.###
#.>.#
#v#v#
#.>.#
###v#
###.#";
        let mut trail = Trail::from_str(input).unwrap();
        let graph = trail.build_graph();

        assert_eq!(8_u64, graph.max_path().unwrap())
    }

    #[test]
    fn test_process_part2() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(154_u64, process_part2(input));
    }
}
