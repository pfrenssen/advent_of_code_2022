use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<Vec<usize>> {
    let mut inventories = vec![];
    let mut inventory = vec![];

    for line in input.lines() {
        if line.is_empty() {
            inventories.push(inventory);
            inventory = vec![];
        } else {
            inventory.push(line.parse::<usize>().unwrap());
        }
    }
    inventories.push(inventory);
    inventories
}

#[aoc(day1, part1)]
fn part1(inventories: &[Vec<usize>]) -> usize {
    // Sum up all inventories and return the largest one.
    inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .max()
        .unwrap()
}

#[aoc(day1, part2)]
fn part2(inventories: &[Vec<usize>]) -> usize {
    let mut inventories = inventories
        .iter()
        .map(|inventory| inventory.iter().sum())
        .collect::<Vec<usize>>();

    inventories.sort();
    inventories.reverse();

    inventories.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];

        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(24000, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(45000, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            1000
            2000
            3000

            4000

            5000
            6000

            7000
            8000
            9000

            10000
        "}
    }
}
