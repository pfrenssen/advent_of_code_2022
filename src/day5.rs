use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
struct Storage {
    stacks: Vec<Stack>,
}

#[derive(Debug, Clone, PartialEq)]
struct Stack {
    items: Vec<String>,
}

impl Stack {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn push(&mut self, item: String) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<String> {
        self.items.pop()
    }

    fn reverse(&mut self) {
        self.items.reverse();
    }
}

impl Storage {
    fn new() -> Self {
        Self { stacks: Vec::new() }
    }

    fn push(&mut self, index: usize, item: String) {
        if index >= self.stacks.len() {
            self.stacks.resize(index + 1, Stack::new());
        }
        self.stacks[index].push(item);
    }

    fn pop(&mut self, index: usize) -> Option<String> {
        if index >= self.stacks.len() {
            None
        } else {
            self.stacks[index].pop()
        }
    }

    fn reverse(&mut self) {
        for stack in self.stacks.iter_mut() {
            stack.reverse();
        }
    }

    fn perform(&mut self, operation: &Operation) {
        for _ in 0..operation.quantity {
            let item = self.pop(operation.from).unwrap();
            self.push(operation.to, item);
        }
    }

    fn move_stack(&mut self, operation: &Operation) {
        let mut items = Vec::new();
        for _ in 0..operation.quantity {
            let item = self.pop(operation.from).unwrap();
            items.push(item);
        }
        items.reverse();
        for item in items {
            self.push(operation.to, item);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Operation {
    quantity: usize,
    from: usize,
    to: usize,
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Storage, Vec<Operation>) {
    // Split the input by empty lines.
    let input: Vec<&str> = input.split("\n\n").collect();

    // Parse the storage.
    let mut storage = Storage::new();
    let mut storage_input = input[0].lines().collect::<Vec<&str>>();
    storage_input.pop();

    for line in storage_input {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(i, c)| {
                if c != ' ' {
                    storage.push(i, c.to_string());
                }
            });
    }

    storage.reverse();

    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)").unwrap();
    let operations = input[1]
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            Operation {
                quantity: caps[1].parse().unwrap(),
                from: caps[2].parse::<usize>().unwrap() - 1,
                to: caps[3].parse::<usize>().unwrap() - 1,
            }
        })
        .collect();
    (storage, operations)
}

#[aoc(day5, part1)]
fn part1(procedure: &(Storage, Vec<Operation>)) -> String {
    let (storage, operations) = procedure;
    let mut storage = storage.clone();

    operations.iter().for_each(|op| {
        storage.perform(op);
    });

    storage
        .stacks
        .iter()
        .map(|s| s.items.last().unwrap().clone())
        .collect::<Vec<String>>()
        .join("")
}

#[aoc(day5, part2)]
fn part2(procedure: &(Storage, Vec<Operation>)) -> String {
    let (storage, operations) = procedure;
    let mut storage = storage.clone();

    operations.iter().for_each(|op| {
        storage.move_stack(op);
    });

    storage
        .stacks
        .iter()
        .map(|s| s.items.last().unwrap().clone())
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected: (Storage, Vec<Operation>) = (
            Storage {
                stacks: vec![
                    Stack {
                        items: vec!["Z".to_string(), "N".to_string()],
                    },
                    Stack {
                        items: vec!["M".to_string(), "C".to_string(), "D".to_string()],
                    },
                    Stack {
                        items: vec!["P".to_string()],
                    },
                ],
            },
            vec![
                Operation {
                    quantity: 1,
                    from: 1,
                    to: 0,
                },
                Operation {
                    quantity: 3,
                    from: 0,
                    to: 2,
                },
                Operation {
                    quantity: 2,
                    from: 1,
                    to: 0,
                },
                Operation {
                    quantity: 1,
                    from: 0,
                    to: 1,
                },
            ],
        );

        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!("CMZ".to_string(), part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!("MCD", part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
                [D]    
            [N] [C]    
            [Z] [M] [P]
             1   2   3 

            move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2
        "}
    }
}
