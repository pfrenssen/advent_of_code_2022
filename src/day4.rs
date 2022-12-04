use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Assignment {
    start: u8,
    end: u8,
}

impl Assignment {
    fn contains(&self, value: u8) -> bool {
        self.start <= value && value <= self.end
    }

    fn covers(&self, other: &Assignment) -> bool {
        self.contains(other.start) && self.contains(other.end)
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.contains(other.start) || self.contains(other.end)
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<(Assignment, Assignment)> {
    let re = Regex::new(r"^(\d+)-(\d+),(\d+)-(\d+)$").unwrap();
    input
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            (
                Assignment {
                    start: caps[1].parse().unwrap(),
                    end: caps[2].parse().unwrap(),
                },
                Assignment {
                    start: caps[3].parse().unwrap(),
                    end: caps[4].parse().unwrap(),
                },
            )
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(assignment_pairs: &[(Assignment, Assignment)]) -> usize {
    assignment_pairs
        .iter()
        .filter(|(a, b)| a.covers(b) || b.covers(a))
        .count()
}

#[aoc(day4, part2)]
fn part2(assignment_pairs: &[(Assignment, Assignment)]) -> usize {
    assignment_pairs
        .iter()
        .filter(|(a, b)| a.overlaps(b) || b.overlaps(a))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let expected: Vec<(Assignment, Assignment)> = vec![
            (
                Assignment { start: 2, end: 4 },
                Assignment { start: 6, end: 8 },
            ),
            (
                Assignment { start: 2, end: 3 },
                Assignment { start: 4, end: 5 },
            ),
            (
                Assignment { start: 5, end: 7 },
                Assignment { start: 7, end: 9 },
            ),
            (
                Assignment { start: 2, end: 8 },
                Assignment { start: 3, end: 7 },
            ),
            (
                Assignment { start: 6, end: 6 },
                Assignment { start: 4, end: 6 },
            ),
            (
                Assignment { start: 2, end: 6 },
                Assignment { start: 4, end: 8 },
            ),
        ];

        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(2, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(4, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8
        "}
    }
}
