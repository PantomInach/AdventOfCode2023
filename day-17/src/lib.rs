use std::{
    collections::{BinaryHeap, HashSet},
    str::FromStr,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn go(&self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y.wrapping_sub(1),
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Position {
                x: self.x.wrapping_sub(1),
                y: self.y,
            },
            Direction::Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct State {
    cur_heat_loss: u64,
    position: Position,
    last_dir: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cur_heat_loss.cmp(&other.cur_heat_loss).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cur_heat_loss.partial_cmp(&self.cur_heat_loss)
    }
}

struct Graph {
    heat_loss: Vec<Vec<u64>>,
}

impl FromStr for Graph {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let heat_loss = s
            .lines()
            .map(|l| l.chars().map(|c| c as u64 - '0' as u64).collect())
            .collect();
        Ok(Graph { heat_loss })
    }
}

impl Graph {
    fn in_bound(&self, pos: Position) -> bool {
        self.heat_loss
            .get(pos.y)
            .and_then(|v| v.get(pos.x))
            .is_some()
    }

    fn dijkstra(&self, min_dist: usize, max_dist: usize) -> Option<u64> {
        let mut visited: HashSet<(Position, Direction, usize)> = HashSet::new();
        let mut heap: BinaryHeap<State> = BinaryHeap::new();

        heap.push(State {
            cur_heat_loss: 0,
            position: Position { x: 0, y: 0 },
            last_dir: Direction::Right,
        });
        heap.push(State {
            cur_heat_loss: 0,
            position: Position { x: 0, y: 0 },
            last_dir: Direction::Down,
        });
        let goal = Position {
            x: self.heat_loss.first().unwrap().len() - 1,
            y: self.heat_loss.len() - 1,
        };

        while let Some(State {
            cur_heat_loss,
            position,
            last_dir,
        }) = heap.pop()
        {
            if position == goal {
                return Some(cur_heat_loss);
            }
            for dir in [
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ] {
                if dir == last_dir || dir == last_dir.opposite() {
                    continue;
                }
                let mut lost_heat = 0;
                let mut new_pos = position;
                for dist in 1..=max_dist {
                    new_pos = new_pos.go(dir);
                    if !self.in_bound(new_pos) {
                        break;
                    }
                    lost_heat += self.heat_loss[new_pos.y][new_pos.x];
                    if min_dist > dist {
                        continue;
                    }
                    let next = State {
                        cur_heat_loss: cur_heat_loss + lost_heat,
                        position: new_pos,
                        last_dir: dir,
                    };
                    if visited.insert((new_pos, dir, dist)) {
                        heap.push(next);
                    }
                }
            }
        }
        None
    }
}

pub fn process_part1(input: &str) -> u64 {
    Graph::from_str(input).unwrap().dijkstra(1, 3).unwrap()
}

pub fn process_part2(input: &str) -> u64 {
    Graph::from_str(input).unwrap().dijkstra(4, 10).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(102_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(94_u64, process_part2(input));
    }
}
