use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Opcode {
    Noop,
    Addx,
}

impl Opcode {
    fn cycles(&self) -> usize {
        match self {
            Opcode::Noop => 1,
            Opcode::Addx => 2,
        }
    }

    fn execute(&self, operand: Option<isize>, registers: &mut Registers, clock: &mut usize) {
        match self {
            Opcode::Noop => {}
            Opcode::Addx => {
                registers.x += operand.unwrap();
            }
        }
        *clock += self.cycles()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Registers {
    x: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Instruction {
    opcode: Opcode,
    operand: Option<isize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    registers: Registers,
    program: Vec<Instruction>,
    pc: usize,
    clock: usize,
}

impl Machine {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            registers: Registers { x: 1 },
            program,
            pc: 0,
            clock: 0,
        }
    }

    fn step(&mut self) {
        let instruction = self.program[self.pc];
        instruction
            .opcode
            .execute(instruction.operand, &mut self.registers, &mut self.clock);
        self.pc += 1;
    }

    fn row(&self) -> String {
        let mut row = str::repeat(".", 40);
        for i in self.registers.x - 1..self.registers.x + 2 {
            if (0..40).contains(&i) {
                row.replace_range(i as usize..i as usize + 1, "#");
            }
        }
        row
    }
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let opcode = match parts.next().unwrap() {
                "noop" => Opcode::Noop,
                "addx" => Opcode::Addx,
                _ => panic!("Unknown opcode"),
            };
            let operand = parts.next().map(|s| s.parse().unwrap());
            Instruction { opcode, operand }
        })
        .collect()
}

#[aoc(day10, part1)]
fn part1(motions: &[Instruction]) -> isize {
    let mut signal_strengths: HashMap<usize, isize> = HashMap::new();
    let sample_cycles = [20, 60, 100, 140, 180, 220];
    let mut machine = Machine::new(motions.to_vec());
    let mut xval = machine.registers.x;

    for sample_cycle in sample_cycles.iter() {
        while machine.clock < *sample_cycle {
            xval = machine.registers.x;
            machine.step();
        }
        signal_strengths.insert(*sample_cycle, *sample_cycle as isize * xval);
    }
    signal_strengths.values().sum()
}

#[aoc(day10, part2)]
fn part2(motions: &[Instruction]) -> String {
    let mut machine = Machine::new(motions.to_vec());
    let mut pixels = String::new();

    while machine.pc < machine.program.len() {
        let row = machine.row();
        let current_cycle = machine.clock;

        machine.step();
        let next_cycle = machine.clock;

        for cycle in current_cycle..next_cycle {
            let pos = cycle % 40;
            pixels.push(row.chars().nth(pos).unwrap());
        }
    }

    // Split the long string of pixels into rows of 40 pixels. Not so easy in Rust o.O
    pixels
        .chars()
        .collect::<Vec<char>>()
        .chunks(40)
        .map(|c| c.iter().collect())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = vec![
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(15),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-11),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(6),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-3),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(5),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-8),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(13),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(4),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(5),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(5),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(5),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(5),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-35),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(24),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-19),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(16),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-11),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(21),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-15),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-3),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(9),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-3),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(8),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(5),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-36),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(7),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(2),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(6),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(7),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-13),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(13),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(7),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-33),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(2),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(8),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(2),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(17),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-9),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-3),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(11),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-13),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-19),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(3),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(26),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-30),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(12),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(3),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-9),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(18),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(2),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(9),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(2),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-37),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(3),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(15),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-21),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(22),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-6),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(2),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-10),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(20),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(1),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(2),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(2),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-6),
            },
            Instruction {
                opcode: Opcode::Addx,
                operand: Some(-11),
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
            Instruction {
                opcode: Opcode::Noop,
                operand: None,
            },
        ];

        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(13140, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        let expected = indoc! {"
            ##..##..##..##..##..##..##..##..##..##..
            ###...###...###...###...###...###...###.
            ####....####....####....####....####....
            #####.....#####.....#####.....#####.....
            ######......######......######......####
            #######.......#######.......#######....."};
        assert_eq!(expected, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop
        "}
    }
}
