use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> String {
    input.to_string()
}

#[aoc(day6, part1)]
fn part1(datastream: &str) -> usize {
    0
}

#[aoc(day6, part2)]
fn part2(datastream: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = vec![
            "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(),
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_string(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(),
        ];

        get_test_input()
            .iter()
            .enumerate()
            .for_each(|(i, test_case)| {
                let (input, _) = test_case;
                assert_eq!(expected[i], parse_input(input));
            });
    }

    #[test]
    fn part1_example() {
        let test_cases = get_test_input();
        for (input, expected) in test_cases {
            let parsed = part1(&input);
            assert_eq!(expected, part1(&input));
        }
    }

    #[test]
    fn part2_example() {
        // let input = parse_input(get_test_input());
        // assert_eq!("MCD", part2(&input));
    }

    fn get_test_input<'a>() -> Vec<(&'a str, usize)> {
        vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ]
    }
}
