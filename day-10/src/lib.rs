use std::collections::HashSet;

type WalkOutput = Option<(u64, HashSet<(i64, i64)>, HashSet<(i64, i64)>)>;

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug)]
enum Turn {
    Other,
    Left,
    Right,
}

impl Turn {
    fn opposite(self) -> Turn {
        match self {
            Turn::Other => Turn::Other,
            Turn::Left => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

impl Direction {
    fn manipulation(&self) -> (i64, i64) {
        match self {
            Direction::North => (-1, 0),
            Direction::East => (0, 1),
            Direction::South => (1, 0),
            Direction::West => (0, -1),
        }
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }

    fn turn(self, going_to: &Direction) -> Turn {
        match self {
            Direction::North => match going_to {
                Direction::North => Turn::Other,
                Direction::West => Turn::Left,
                Direction::East => Turn::Right,
                Direction::South => Turn::Other,
            },
            Direction::East => match going_to {
                Direction::North => Turn::Left,
                Direction::East => Turn::Other,
                Direction::South => Turn::Right,
                Direction::West => Turn::Other,
            },
            Direction::South => self.opposite().turn(going_to).opposite(),
            Direction::West => self.opposite().turn(going_to).opposite(),
        }
    }

    fn neighbor_dirs(&self, going_to: &Direction) -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ]
        .into_iter()
        .filter(|d| d != self && d != going_to)
        .collect()
    }
}

const POSSIBLE_DIRECTIONS: [(char, (Direction, Direction)); 6] = [
    ('|', (Direction::North, Direction::South)),
    ('-', (Direction::East, Direction::West)),
    ('L', (Direction::North, Direction::East)),
    ('J', (Direction::North, Direction::West)),
    ('7', (Direction::West, Direction::South)),
    ('F', (Direction::East, Direction::South)),
];

fn get_dirs(c: &char) -> Option<&(Direction, Direction)> {
    POSSIBLE_DIRECTIONS
        .iter()
        .find(|(dir, _)| c == dir)
        .map(|(_, x)| x)
}

fn get_next_dir(d1: &Direction, d2: &Direction, came_from: &Direction) -> Option<Direction> {
    if came_from != d1 && came_from != d2 {
        None
    } else if came_from == d1 {
        Some(d2.clone())
    } else {
        Some(d1.clone())
    }
}

fn init_field(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|l| l.chars().collect()).collect()
}

fn in_bound(field: &[Vec<char>], x: i64, y: i64) -> bool {
    field
        .get(x as usize)
        .and_then(|v| v.get(y as usize))
        .is_some()
}

fn field_neighbors(field: &[Vec<char>], x: i64, y: i64) -> Vec<(i64, i64)> {
    [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ]
    .iter()
    .map(|dir| dir.manipulation())
    .map(|(dx, dy)| (x + dx, y + dy))
    .filter(|(cx, cy)| in_bound(field, *cx, *cy))
    .collect()
}

fn walk(
    field: &[Vec<char>],
    x: i64,
    y: i64,
    came_from: Direction,
    finish: (i64, i64),
    count: bool,
) -> WalkOutput {
    let (mut cx, mut cy) = (x, y);
    let mut dir: Direction = came_from;
    let mut i = 1;

    let mut r_turns: usize = 0;
    let mut l_turns: usize = 0;
    let mut left_nodes: HashSet<(i64, i64)> = HashSet::new();
    let mut right_nodes: HashSet<(i64, i64)> = HashSet::new();
    let mut on_cycle: HashSet<(i64, i64)> = HashSet::new();

    while (cx, cy) != finish {
        if let Some(new_dir) = field
            .get(cx as usize)
            .and_then(|v| v.get(cy as usize))
            .and_then(get_dirs)
            .and_then(|(d1, d2)| get_next_dir(d1, d2, &dir))
        {
            let (delta_x, delta_y) = new_dir.manipulation();

            if count {
                on_cycle.insert((cx, cy));
                let neighbor_dirs = dir.neighbor_dirs(&new_dir);
                match dir.clone().turn(&new_dir) {
                    Turn::Right => {
                        r_turns += 1;
                        neighbor_dirs
                            .iter()
                            .map(|d| d.manipulation())
                            .for_each(|(dx, dy)| {
                                if in_bound(field, cx + dx, cy + dy) {
                                    left_nodes.insert((cx + dx, cy + dy));
                                }
                            })
                    }
                    Turn::Left => {
                        l_turns += 1;
                        neighbor_dirs
                            .iter()
                            .map(|d| d.manipulation())
                            .for_each(|(dx, dy)| {
                                if in_bound(field, cx + dx, cy + dy) {
                                    right_nodes.insert((cx + dx, cy + dy));
                                }
                            })
                    }
                    Turn::Other => neighbor_dirs
                        .iter()
                        .for_each(|d| match dir.clone().turn(d) {
                            Turn::Other => unimplemented!("Not possible"),
                            Turn::Left => {
                                let (dx, dy) = d.manipulation();
                                if in_bound(field, cx + dx, cy + dy) {
                                    left_nodes.insert((cx + dx, cy + dy));
                                }
                            }
                            Turn::Right => {
                                let (dx, dy) = d.manipulation();
                                if in_bound(field, cx + dx, cy + dy) {
                                    right_nodes.insert((cx + dx, cy + dy));
                                }
                            }
                        }),
                };
            }

            (cx, cy) = (cx + delta_x, cy + delta_y);
            dir = new_dir.opposite();
        } else {
            return None;
        }
        i += 1;
    }
    let insight_nodes = if r_turns > l_turns {
        right_nodes
    } else {
        left_nodes
    }
    .into_iter()
    .filter(|point| !on_cycle.contains(point))
    .collect();
    Some((i, insight_nodes, on_cycle))
}

pub fn process_part1(input: &str) -> u64 {
    let field = init_field(input);
    let (x, y) = field
        .iter()
        .enumerate()
        .find(|(_, l)| l.iter().enumerate().any(|(_, c)| c == &'S'))
        .and_then(|(x, l)| {
            l.iter()
                .enumerate()
                .find(|(_, c)| c == &&'S')
                .map(|(y, _)| (x, y))
        })
        .unwrap();

    *[
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ]
    .iter()
    .flat_map(|dir| {
        let (dx, dy) = dir.manipulation();
        walk(
            &field,
            x as i64 + dx,
            y as i64 + dy,
            dir.opposite(),
            (x as i64, y as i64),
            false,
        )
    })
    .map(|(n, _, _)| n)
    .collect::<Vec<u64>>()
    .first()
    .unwrap()
        / 2
}

pub fn process_part2(input: &str) -> u64 {
    let field = init_field(input);
    let (x, y) = field
        .iter()
        .enumerate()
        .find(|(_, l)| l.iter().enumerate().any(|(_, c)| c == &'S'))
        .and_then(|(x, l)| {
            l.iter()
                .enumerate()
                .find(|(_, c)| c == &&'S')
                .map(|(y, _)| (x, y))
        })
        .unwrap();

    let (_, in_nodes, on_cycle) = [
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ]
    .iter()
    .flat_map(|dir| {
        let (dx, dy) = dir.manipulation();
        walk(
            &field,
            x as i64 + dx,
            y as i64 + dy,
            dir.opposite(),
            (x as i64, y as i64),
            true,
        )
    })
    .collect::<Vec<_>>()
    .first()
    .unwrap()
    .clone();
    let mut insight_nodes = in_nodes;
    let mut queue: Vec<(i64, i64)> = insight_nodes.clone().into_iter().collect();
    while !queue.is_empty() {
        let (x, y) = &queue.pop().unwrap();
        queue.extend(
            field_neighbors(&field, *x, *y)
                .iter()
                .filter(|point| !insight_nodes.contains(point) && !on_cycle.contains(point)),
        );
        insight_nodes.insert((*x, *y));
    }
    if insight_nodes.contains(&(x as i64, y as i64)) {
        insight_nodes.remove(&(x as i64, y as i64));
    };
    insight_nodes.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1_v0() {
        let input = "FS
LJ";
        assert_eq!(2_u64, process_part1(input));
    }

    #[test]
    fn test_process_part1_v1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(4_u64, process_part1(input));
    }

    #[test]
    fn test_process_part1_v2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(8_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(4_u64, process_part2(input));
    }

    #[test]
    fn test_process_part2_v2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(8_u64, process_part2(input));
    }

    #[test]
    fn test_process_part2_v3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(10_u64, process_part2(input));
    }

    #[test]
    fn test_process_part2_v0() {
        let input = "S-7
|.|
L-J";
        assert_eq!(1_u64, process_part2(input));
    }

    #[test]
    fn test_process_part2_v4() {
        let input = "S-7.
|.|.
|.|.
L-J.";
        assert_eq!(2_u64, process_part2(input));
    }

    #[test]
    fn test_process_part2_v5() {
        let input = "S---7.
|...|.
|...|.
|...|.
L---J.";
        assert_eq!(9_u64, process_part2(input));
    }

    #[test]
    fn test_process_part2_v6() {
        let input = "S---7.
|F-7|.
||-||.
|L-J|.
L---J.";
        assert_eq!(9_u64, process_part2(input));
    }

    #[test]
    fn test_neighbors() {
        assert_eq!(
            Direction::North.neighbor_dirs(&Direction::East),
            vec![Direction::West, Direction::South]
        );
        assert_eq!(
            Direction::North.neighbor_dirs(&Direction::South),
            vec![Direction::West, Direction::East]
        );
    }
}
