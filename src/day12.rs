use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::{bfs, Matrix};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    x: usize,
    y: usize,
}

// Implement the From trait for (usize, usize).
impl<T> From<T> for Coordinate
where
    T: Into<(usize, usize)>,
{
    fn from(key: T) -> Self {
        let (y, x) = key.into();
        Coordinate { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct HeightMap {
    map: Matrix<u8>,
    start: Coordinate,
    dest: Coordinate,
}

#[aoc_generator(day12, part1)]
fn parse_input_part1(input: &str) -> HeightMap {
    let mut matrix: Matrix<u8> = Matrix::from_rows(input.lines().map(str::bytes)).unwrap();

    // Find the keys for the start and end points.
    let start = matrix.keys().find(|k| matrix[*k] == b'S').unwrap();
    let dest = matrix.keys().find(|k| matrix[*k] == b'E').unwrap();

    // We start at height 'a' and end at height 'z'. Overwrite the heights for the start and end
    // points with the appropriate values.
    matrix[start] = b'a';
    matrix[dest] = b'z';

    HeightMap {
        map: matrix,
        start: start.into(),
        dest: dest.into(),
    }
}

#[aoc_generator(day12, part2)]
fn parse_input_part2(input: &str) -> HeightMap {
    parse_input_part1(input)
}

#[aoc(day12, part1)]
fn part1(height_map: &HeightMap) -> usize {
    let start = (height_map.start.y, height_map.start.x);
    let dest = (height_map.dest.y, height_map.dest.x);
    let matrix = &height_map.map;
    bfs(
        &start,
        // Find all neighbours that are at most 1 meter higher than the current point.
        |&p| {
            matrix.neighbours(p, false).filter(move |&q| {
                let height_diff = (matrix[q] as i32) - (matrix[p] as i32);
                height_diff <= 1
            })
        },
        |&p| p == dest,
    )
    .unwrap()
    .len()
        - 1
}

#[aoc(day12, part2)]
fn part2(height_map: &HeightMap) -> usize {
    // Walk from the top down to the nearest point at elevation 'a'.
    let start = (height_map.dest.y, height_map.dest.x);
    let matrix = &height_map.map;
    bfs(
        &start,
        // Find all neighbours that are at most 1 meter lower than the current point.
        |&p| {
            matrix.neighbours(p, false).filter(move |&q| {
                let height_diff = (matrix[q] as i32) - (matrix[p] as i32);
                height_diff >= -1
            })
        },
        |&p| matrix[p] == b'a',
    )
    .unwrap()
    .len()
        - 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input_part1() {
        let expected = HeightMap {
            map: Matrix::<u8>::from_rows(vec![
                vec![b'a', b'a', b'b', b'q', b'p', b'o', b'n', b'm'],
                vec![b'a', b'b', b'c', b'r', b'y', b'x', b'x', b'l'],
                vec![b'a', b'c', b'c', b's', b'z', b'z', b'x', b'k'],
                vec![b'a', b'c', b'c', b't', b'u', b'v', b'w', b'j'],
                vec![b'a', b'b', b'd', b'e', b'f', b'g', b'h', b'i'],
            ])
            .unwrap(),
            start: Coordinate { x: 0, y: 0 },
            dest: Coordinate { x: 5, y: 2 },
        };

        assert_eq!(expected, parse_input_part1(get_test_input_part1()));
    }

    #[test]
    fn test_parse_input_part2() {
        test_parse_input_part1();
    }

    #[test]
    fn part1_example() {
        let input = parse_input_part1(get_test_input_part1());
        assert_eq!(31, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input_part2(get_test_input_part2());
        assert_eq!(29, part2(&input));
    }

    fn get_test_input_part1<'a>() -> &'a str {
        indoc! {"
            Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi
        "}
    }

    fn get_test_input_part2<'a>() -> &'a str {
        get_test_input_part1()
    }
}
