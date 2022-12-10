use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
struct Motion {
    direction: Directions,
    distance: usize,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl Coordinate {
    fn step(&mut self, direction: &Directions) {
        match direction {
            Directions::Up => self.y -= 1,
            Directions::Down => self.y += 1,
            Directions::Left => self.x -= 1,
            Directions::Right => self.x += 1,
        }
    }

    fn follow(&mut self, target: &Coordinate) {
        let xdelta = target.x - self.x;
        let ydelta = target.y - self.y;

        // Only follow if the target is more than 1 step away.
        if xdelta.abs() < 2 && ydelta.abs() < 2 {
            return;
        }

        // Move diagonally if possible.
        if xdelta.abs() > 0 && ydelta.abs() > 0 {
            if xdelta > 0 {
                self.step(&Directions::Right);
            } else {
                self.step(&Directions::Left);
            }

            if ydelta > 0 {
                self.step(&Directions::Down);
            } else {
                self.step(&Directions::Up);
            }
        } else {
            // Move either horizontally or vertically.
            if xdelta.abs() > 1 {
                if xdelta > 0 {
                    self.step(&Directions::Right);
                } else {
                    self.step(&Directions::Left);
                }
            } else if ydelta > 0 {
                self.step(&Directions::Down);
            } else {
                self.step(&Directions::Up);
            }
        }
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Motion> {
    input
        .lines()
        .map(|line| {
            let (direction, distance) = line.split_at(1);
            let direction = match direction {
                "U" => Directions::Up,
                "D" => Directions::Down,
                "L" => Directions::Left,
                "R" => Directions::Right,
                _ => panic!("Invalid direction"),
            };
            let distance = distance[1..].parse().unwrap();
            Motion {
                direction,
                distance,
            }
        })
        .collect()
}

#[aoc(day9, part1)]
fn part1(motions: &[Motion]) -> usize {
    let mut hcoord = Coordinate { x: 0, y: 0 };
    let mut tcoord = Coordinate { x: 0, y: 0 };

    let mut visited = HashSet::new();

    // Add the starting point to the set of visited coordinates.
    visited.insert(tcoord);

    for motion in motions {
        for _ in 0..motion.distance {
            hcoord.step(&motion.direction);
            tcoord.follow(&hcoord);
            visited.insert(tcoord);
        }
    }

    visited.len()
}

#[aoc(day9, part2)]
fn part2(motions: &[Motion]) -> usize {
    let mut coords: [Coordinate; 10] = [Coordinate { x: 0, y: 0 }; 10];

    let mut visited = HashSet::new();

    // Add the starting point to the set of visited coordinates.
    visited.insert(coords[9]);

    for motion in motions {
        for _ in 0..motion.distance {
            coords[0].step(&motion.direction);
            for i in 1..10 {
                let previous_coord = coords[i - 1];
                coords[i].follow(&previous_coord);
            }
            visited.insert(coords[9]);
        }
    }

    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = vec![
            Motion {
                direction: Directions::Right,
                distance: 4,
            },
            Motion {
                direction: Directions::Up,
                distance: 4,
            },
            Motion {
                direction: Directions::Left,
                distance: 3,
            },
            Motion {
                direction: Directions::Down,
                distance: 1,
            },
            Motion {
                direction: Directions::Right,
                distance: 4,
            },
            Motion {
                direction: Directions::Down,
                distance: 1,
            },
            Motion {
                direction: Directions::Left,
                distance: 5,
            },
            Motion {
                direction: Directions::Right,
                distance: 2,
            },
        ];

        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(13, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(1, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2
        "}
    }
}
