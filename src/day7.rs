use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
enum NodeType {
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    name: String,
    size: usize,
    node_type: NodeType,
    children: Vec<Node>,
}

impl Node {
    fn new(name: String, size: usize, node_type: NodeType) -> Self {
        Self {
            name,
            size,
            node_type,
            children: Vec::new(),
        }
    }

    fn add_child(&mut self, child: &Node) {
        self.children.push(child.clone());
    }

    fn total_size(&self) -> usize {
        self.size + self.children.iter().map(|c| c.total_size()).sum::<usize>()
    }

    fn find_by_path_mut(&mut self, path: &[String]) -> Option<&mut Node> {
        if path.len() == 1 {
            Some(self)
        } else {
            let child = self
                .children
                .iter_mut()
                .find(|c| c.name == path[1] && c.node_type == NodeType::Directory);
            if let Some(child) = child {
                child.find_by_path_mut(&path[1..])
            } else {
                panic!("Child not found");
            }
        }
    }

    fn get_children_recursive(&self) -> Vec<Node> {
        // Return a flat list of all children.
        let mut children: Vec<Node> = vec![];

        for child in self.children.iter() {
            children.push(Node::new(
                child.name.clone(),
                child.total_size(),
                child.node_type.clone(),
            ));
            children.extend(child.get_children_recursive());
        }

        children
    }
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_string()).collect()
}

#[aoc(day7, part1)]
fn part1(output: &[String]) -> usize {
    let filesystem = get_filesystem(output);

    // Sum up the total size of all directories in the filesystem that have a size lower than 100kb.
    filesystem
        .get_children_recursive()
        .iter()
        .filter(|c| c.node_type == NodeType::Directory && c.size <= 100000)
        .map(|c| c.size)
        .sum()
}

#[aoc(day7, part2)]
fn part2(output: &[String]) -> usize {
    let filesystem = get_filesystem(output);
    let min_free_size = filesystem.total_size() - 40000000;

    // Filter out all files and directories that have a size lower than the minimum free size.
    let mut candidates = filesystem.get_children_recursive();

    candidates.retain(|c| c.node_type == NodeType::Directory && c.size >= min_free_size);

    // Sort the candidates by size and return the size of the smallest one.
    candidates.sort_by(|a, b| a.size.cmp(&b.size));
    candidates.first().unwrap().size
}

fn get_filesystem(output: &[String]) -> Node {
    let mut filesystem = Node {
        name: "/".to_string(),
        size: 0,
        node_type: NodeType::Directory,
        children: Vec::new(),
    };

    let mut path = vec!["/".to_string()];

    for line in output {
        match line {
            // Skip navigating to the root directory.
            l if l.eq("$ cd /") => {}
            // Skip listing the current directory.
            l if l.eq("$ ls") => {}
            // Navigate to the parent directory.
            l if l.eq("$ cd ..") => {
                path.pop();
            }
            // Navigate to a subdirectory.
            l if l.starts_with("$ cd ") => {
                let dir = l[5..].to_string();
                path.push(dir);
            }
            // Add a directory to the filesystem.
            l if l.starts_with("dir ") => {
                let name = l[4..].to_string();
                let node = Node::new(name, 0, NodeType::Directory);
                let parent = filesystem.find_by_path_mut(&path).unwrap();
                parent.add_child(&node);
            }
            // Add a file if the line starts with the filesize.
            l if l.starts_with(char::is_numeric) => {
                let re = Regex::new(r"(\d+) (.+)").unwrap();
                let caps = re.captures(l).unwrap();
                let size = caps[1].parse::<usize>().unwrap();
                let name = caps[2].to_string();
                let node = Node::new(name, size, NodeType::File);
                let parent = filesystem.find_by_path_mut(&path).unwrap();
                parent.add_child(&node);
            }
            _ => {}
        }
    }

    filesystem
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_parse_input() {
        let expected = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];

        assert_eq!(expected, parse_input(get_test_input()));
    }

    #[test]
    fn part1_example() {
        let input = parse_input(get_test_input());
        assert_eq!(95437, part1(&input));
    }

    #[test]
    fn part2_example() {
        let input = parse_input(get_test_input());
        assert_eq!(24933642, part2(&input));
    }

    fn get_test_input<'a>() -> &'a str {
        indoc! {"
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        "}
    }
}
