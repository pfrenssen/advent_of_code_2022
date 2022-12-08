use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::Range;

#[derive(Debug, Clone, Copy)]
enum Directions {
    North,
    South,
    East,
    West,
}

impl Directions {
    const VALUES: [Self; 4] = [
        Directions::North,
        Directions::South,
        Directions::East,
        Directions::West,
    ];

    fn ranges(
        &self,
        xpos: usize,
        ypos: usize,
        xsize: usize,
        ysize: usize,
    ) -> (Range<usize>, Range<usize>) {
        match self {
            Directions::North => (xpos..xpos + 1, 0..ypos),
            Directions::East => (xpos + 1..xsize, ypos..ypos + 1),
            Directions::South => (xpos..xpos + 1, ypos + 1..ysize),
            Directions::West => (0..xpos, ypos..ypos + 1),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Heightmap {
    data: Vec<Vec<u8>>,
}

impl Heightmap {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[y][x]
    }

    fn is_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == self.data[0].len() - 1 || y == self.data.len() - 1
    }

    fn xsize(&self) -> usize {
        self.data[0].len()
    }

    fn ysize(&self) -> usize {
        self.data.len()
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        // Trees on the edge of the forest are always visible.
        if self.is_edge(x, y) {
            return true;
        }

        // The tree is visible if all the squares between it and the edge have a lower height.
        let height = self.get(x, y);

        for dir in Directions::VALUES.iter() {
            let (x_range, y_range) = dir.ranges(x, y, self.xsize(), self.ysize());
            let mut visible = true;

            'outer: for x2 in x_range {
                for y2 in y_range.clone() {
                    if self.get(x2, y2) >= height {
                        visible = false;
                        break 'outer;
                    }
                }
            }

            if visible {
                return true;
            }
        }

        false
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        // Trees on the edge always score 0.
        if self.is_edge(x, y) {
            return 0;
        }

        Directions::VALUES
            .iter()
            .map(|dir| {
                let (x_range, y_range) = dir.ranges(x, y, self.xsize(), self.ysize());
                let mut distance = 0;
                match dir {
                    Directions::North => {
                        for y2 in y_range.rev() {
                            distance += 1;
                            if self.get(x, y2) >= self.get(x, y) {
                                break;
                            }
                        }
                        distance
                    }
                    Directions::East => {
                        for x2 in x_range {
                            distance += 1;
                            if self.get(x2, y) >= self.get(x, y) {
                                break;
                            }
                        }
                        distance
                    }
                    Directions::South => {
                        for y2 in y_range {
                            distance += 1;
                            if self.get(x, y2) >= self.get(x, y) {
                                break;
                            }
                        }
                        distance
                    }
                    Directions::West => {
                        for x2 in x_range.rev() {
                            distance += 1;
                            if self.get(x2, y) >= self.get(x, y) {
                                break;
                            }
                        }
                        distance
                    }
                }
            })
            .product()
    }
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Heightmap {
    Heightmap {
        data: input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as u8).collect())
            .collect(),
    }
}

#[aoc(day8, part1)]
fn part1(heightmap: &Heightmap) -> usize {
    // Count the number of visible trees.
    let mut count = 0;

    for x in 0..heightmap.xsize() {
        for y in 0..heightmap.ysize() {
            if heightmap.is_visible(x, y) {
                count += 1;
            }
        }
    }

    count
}

#[aoc(day8, part2)]
fn part2(heightmap: &Heightmap) -> usize {
    // Calculate the scenic score for each tree.
    let mut highest_score = 0;

    for x in 0..heightmap.xsize() {
        for y in 0..heightmap.ysize() {
            let score = heightmap.scenic_score(x, y);
            if score > highest_score {
                highest_score = score;
            }
        }
    }

    highest_score
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = Heightmap {
            data: vec![
                vec![3, 0, 3, 7, 3],
                vec![2, 5, 5, 1, 2],
                vec![6, 5, 3, 3, 2],
                vec![3, 3, 5, 4, 9],
                vec![3, 5, 3, 9, 0],
            ],
        };

        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(21, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(8, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            30373
            25512
            65332
            33549
            35390
        "}
    }
}
