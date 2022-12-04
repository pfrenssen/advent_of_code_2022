use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Copy, Clone, Debug, PartialEq)]
enum Unit {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Unit {
    fn beats(&self, other: &Unit) -> bool {
        match self {
            Unit::Rock => other == &Unit::Scissors,
            Unit::Paper => other == &Unit::Rock,
            Unit::Scissors => other == &Unit::Paper,
        }
    }

    fn from_char(c: char) -> Unit {
        match c {
            'A' | 'X' => Unit::Rock,
            'B' | 'Y' => Unit::Paper,
            _ => Unit::Scissors,
        }
    }

    fn outcome(&self, other: &Unit) -> Outcome {
        if self.beats(other) {
            Outcome::Win
        } else if other.beats(self) {
            Outcome::Loss
        } else {
            Outcome::Draw
        }
    }
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl Outcome {
    fn from_char(c: char) -> Outcome {
        match c {
            'Z' => Outcome::Win,
            'Y' => Outcome::Draw,
            _ => Outcome::Loss,
        }
    }

    fn matching_unit(&self, other: &Unit) -> Unit {
        match self {
            Outcome::Win => match other {
                Unit::Rock => Unit::Paper,
                Unit::Paper => Unit::Scissors,
                Unit::Scissors => Unit::Rock,
            },
            Outcome::Draw => *other,
            Outcome::Loss => match other {
                Unit::Rock => Unit::Scissors,
                Unit::Paper => Unit::Rock,
                Unit::Scissors => Unit::Paper,
            },
        }
    }
}

#[aoc_generator(day2, part1)]
fn parse_input_part1(input: &str) -> Vec<(Unit, Unit)> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let a = Unit::from_char(chars.next().unwrap());
            chars.next().unwrap();
            let b = Unit::from_char(chars.next().unwrap());
            (a, b)
        })
        .collect()
}

#[aoc_generator(day2, part2)]
fn parse_input_part2(input: &str) -> Vec<(Unit, Outcome)> {
    input
        .lines()
        .map(|line| {
            let mut chars = line.chars();
            let a = Unit::from_char(chars.next().unwrap());
            chars.next().unwrap();
            let b = Outcome::from_char(chars.next().unwrap());
            (a, b)
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(games: &[(Unit, Unit)]) -> usize {
    let mut score = 0;
    for (a, b) in games {
        score += b.outcome(a) as usize;
        score += *b as usize;
    }
    score
}

#[aoc(day2, part2)]
fn part2(games: &[(Unit, Outcome)]) -> usize {
    let mut score = 0;
    for (a, b) in games {
        let matching_unit = b.matching_unit(a);
        score += matching_unit.outcome(a) as usize;
        score += matching_unit as usize;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let expected = vec![
            (Unit::Rock, Unit::Paper),
            (Unit::Paper, Unit::Rock),
            (Unit::Scissors, Unit::Scissors),
        ];

        assert_eq!(expected, parse_input_part1(get_test_input()));
    }

    #[test]
    fn test_parse_input_part2() {
        let expected = vec![
            (Unit::Rock, Outcome::Draw),
            (Unit::Paper, Outcome::Loss),
            (Unit::Scissors, Outcome::Win),
        ];

        assert_eq!(expected, parse_input_part2(get_test_input()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input());
        assert_eq!(15, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input());
        assert_eq!(12, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            A Y
            B X
            C Z
        "}
    }
}
