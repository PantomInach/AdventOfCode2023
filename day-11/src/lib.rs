use std::str::FromStr;

use itertools::Itertools;

#[derive(PartialEq, Clone)]
enum Bit {
    Galaxy,
    Space,
}

impl Bit {
    fn from_char(s: &char) -> Result<Self, String> {
        match s {
            '.' => Ok(Bit::Space),
            '#' => Ok(Bit::Galaxy),
            _ => Err(format!("Can't parse '{:?}' into Bit", s)),
        }
    }
}

struct UniversumMap {
    image: Vec<Vec<Bit>>,
    galaxy_cords: Vec<(usize, usize)>,
}

fn get_galaxy_cords(image: &[Vec<Bit>]) -> Vec<(usize, usize)> {
    image
        .iter()
        .enumerate()
        .flat_map(|(x, column)| {
            column
                .iter()
                .enumerate()
                .filter(|(_, b)| **b == Bit::Galaxy)
                .map(|(y, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect()
}

impl FromStr for UniversumMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (expand_map, insert_row_at, insert_column_at) =
            UniversumMap::universum_map_unexpanded(s);
        let mut expand = expand_map.image;

        let mut columns_inserted: usize = 0;
        let size = expand.first().unwrap().len();
        insert_column_at.iter().for_each(|p| {
            expand.insert(columns_inserted + p, vec![Bit::Space; size]);
            columns_inserted += 1;
        });

        let mut rows_inserted: usize = 0;
        insert_row_at.iter().for_each(|y| {
            (0..expand.len()).for_each(|x| {
                expand
                    .get_mut(x)
                    .unwrap()
                    .insert(y + rows_inserted, Bit::Space);
            });
            rows_inserted += 1;
        });

        let galaxy_cords: Vec<(usize, usize)> = get_galaxy_cords(&expand);

        Ok(UniversumMap {
            image: expand,
            galaxy_cords,
        })
    }
}

impl UniversumMap {
    fn universum_map_unexpanded(s: &str) -> (UniversumMap, Vec<usize>, Vec<usize>) {
        let unexpanded = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| Bit::from_char(&c))
                    .collect::<Result<Vec<Bit>, String>>()
            })
            .collect::<Result<Vec<Vec<Bit>>, String>>()
            .unwrap();

        let insert_column_at: Vec<usize> = unexpanded
            .iter()
            .enumerate()
            .filter(|(_, c)| c.iter().all(|b| *b == Bit::Space))
            .map(|(i, _)| i)
            .collect();

        let insert_row_at: Vec<usize> = (0..unexpanded.first().unwrap().len())
            .filter(|y| {
                (0..unexpanded.len()).all(|x| {
                    unexpanded
                        .get(x)
                        .and_then(|row| row.get(*y))
                        .is_some_and(|b| *b == Bit::Space)
                })
            })
            .collect();

        let galaxy_cords: Vec<(usize, usize)> = get_galaxy_cords(&unexpanded);

        (
            UniversumMap {
                image: unexpanded,
                galaxy_cords,
            },
            insert_row_at,
            insert_column_at,
        )
    }
}

pub fn process_part1(input: &str) -> u64 {
    let universum = UniversumMap::from_str(input).unwrap();
    universum
        .galaxy_cords
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1))
        .sum::<usize>() as u64
}

pub fn process_part2(input: &str) -> u64 {
    process_part2_with_factor(input, 1_000_000)
}

fn process_part2_with_factor(input: &str, factor: usize) -> u64 {
    let (universum_map, scale_x, scale_y) = UniversumMap::universum_map_unexpanded(input);
    universum_map
        .galaxy_cords
        .iter()
        .tuple_combinations()
        .map(|(g1, g2)| -> usize {
            let passed_y: Vec<&usize> = scale_y
                .iter()
                .filter(|&&x| g1.0 <= x && x <= g2.0 || g2.0 <= x && x <= g1.0)
                .collect();
            let passed_x: Vec<&usize> = scale_x
                .iter()
                .filter(|&&y| g1.1 <= y && y <= g2.1 || g2.1 <= y && y <= g1.1)
                .collect();
            g1.0.abs_diff(g2.0)
                + g1.1.abs_diff(g2.1)
                + (passed_x.len() + passed_y.len()) * (factor - 1)
        })
        .sum::<usize>() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(374_u64, process_part1(input));
    }

    #[test]
    fn test_process_part2_10() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(1030_u64, process_part2_with_factor(input, 10));
    }

    #[test]
    fn test_process_part2_100() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(8410_u64, process_part2_with_factor(input, 100));
    }

    #[test]
    fn test_process_part2_v1() {
        let input = "#.#";
        assert_eq!(11_u64, process_part2_with_factor(input, 10));
    }
}
