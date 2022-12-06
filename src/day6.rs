use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> String {
    input.to_string()
}

#[aoc(day6, part1)]
fn part1(datastream: &str) -> usize {
    find_non_consecutive(datastream, 4)
}

#[aoc(day6, part2)]
fn part2(datastream: &str) -> usize {
    find_non_consecutive(datastream, 14)
}

fn find_non_consecutive(datastream: &str, count: usize) -> usize {
    // Loop through the datastream as a moving window of usize characters.
    for i in 0..datastream.len() - count + 1 {
        let window = &datastream[i..i + count];
        // Check that there are no duplicate characters in the window.
        if window.chars().all(|c| window.matches(c).count() == 1) {
            return i + count;
        }
    }

    unreachable!()
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

        get_test_input_part1()
            .iter()
            .enumerate()
            .for_each(|(i, test_case)| {
                let (input, _) = test_case;
                assert_eq!(expected[i], parse_input(input));
            });
    }

    #[test]
    fn part1_example() {
        let test_cases = get_test_input_part1();
        for (input, expected) in test_cases {
            let parsed = part1(&input);
            assert_eq!(expected, part1(&input));
        }
    }

    #[test]
    fn part2_example() {
        let test_cases = get_test_input_part2();
        for (input, expected) in test_cases {
            let parsed = part2(&input);
            assert_eq!(expected, part2(&input));
        }
    }

    fn get_test_input_part1<'a>() -> Vec<(&'a str, usize)> {
        vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5),
            ("nppdvjthqldpwncqszvftbrmjlhg", 6),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11),
        ]
    }

    fn get_test_input_part2<'a>() -> Vec<(&'a str, usize)> {
        vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", 23),
            ("nppdvjthqldpwncqszvftbrmjlhg", 23),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26),
        ]
    }
}
