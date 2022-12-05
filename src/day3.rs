use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

#[aoc(day3, part1)]
fn part1(contents: &[Vec<char>]) -> usize {
    contents
        .iter()
        .map(|content| {
            // Split the content in two equal parts.
            let (left, right) = content.split_at(content.len() / 2);
            // Filter out all chars which do not exist in both parts.
            let common = left
                .iter()
                .filter(|&c| right.contains(c))
                .collect::<Vec<_>>();
            // Keep only the first common char.
            let common = common.first().unwrap();
            // Convert the char to its ASCII value.
            let common = **common as usize;
            match common.cmp(&96) {
                // If the char is a lowercase letter, subtract 96.
                Ordering::Greater => common - 96,
                // If the char is an uppercase letter, subtract 64.
                _ => common - 38,
            }
        })
        .sum()
}

#[aoc(day3, part2)]
fn part2(contents: &[Vec<char>]) -> usize {
    // Loop over the contents in batches of 3.
    contents
        .chunks(3)
        .map(|content| {
            // Find the chars which are in common.
            let common = content[0]
                .iter()
                .filter(|&c| content[1].contains(c) && content[2].contains(c))
                .collect::<Vec<_>>();
            // Keep only the first common char.
            let common = common.first().unwrap();
            // Convert the char to its ASCII value.
            let common = **common as usize;
            match common.cmp(&96) {
                // If the char is a lowercase letter, subtract 96.
                Ordering::Greater => common - 96,
                // If the char is an uppercase letter, subtract 64.
                _ => common - 38,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected: Vec<Vec<char>> = vec![
            String::from("vJrwpWtwJgWrhcsFMMfFFhFp").chars().collect(),
            String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")
                .chars()
                .collect(),
            String::from("PmmdzqPrVvPwwTWBwg").chars().collect(),
            String::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn")
                .chars()
                .collect(),
            String::from("ttgJtRGJQctTZtZT").chars().collect(),
            String::from("CrZsJsPPZsGzwwsLwLmpwMDw").chars().collect(),
        ];

        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(157, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(70, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "}
    }
}
