use std::ops::AddAssign;

struct Tree<T>
where
    T: PartialEq,
{
    nodes: Vec<Node<T>>,
    current_index: usize,
}

impl<T> Tree<T>
where
    T: Copy + PartialEq + AddAssign,
{
    fn new(nodes: Vec<Node<T>>) -> Self {
        Self {
            nodes,
            current_index: 0,
        }
    }

    fn insert(&mut self, node: Node<T>) {
        self.nodes.push(node);
    }

    /// Gets the parent node at a given index.
    fn parent(&self, index: usize) -> Option<usize> {
        self.nodes[index].parent
    }

    /// Gets the child node at a given index with a target_label.
    fn child(&self, target_label: &str) -> Option<usize> {
        self.nodes[self.current_index]
            .children
            .iter()
            .find(|child_index| self.nodes[**child_index].label == target_label)
            .copied()
    }

    fn add_file(&mut self, index: usize, file_size: T) {
        // Add the file size to the current node.
        self.nodes[index].size += file_size;

        // Recursively add the file size to the parent nodes.
        if let Some(index) = self.nodes[index].parent {
            self.add_file(index, file_size);
        }
    }

    fn go_to_root_directory(&mut self) {
        self.current_index = 0;
    }

    fn go_to_parent_directory(&mut self) {
        match self.parent(self.current_index) {
            Some(parent_index) => {
                self.current_index = parent_index;
            }
            None => unreachable!("Invalid parent"),
        };
    }

    fn go_to_child_directory(&mut self, directory: &str) {
        match self.child(directory) {
            Some(child_index) => {
                self.current_index = child_index;
            }
            None => unreachable!("Invalid child"),
        };
    }

    fn go_to_directory(&mut self, directory: &str) {
        match directory {
            "/" => self.go_to_root_directory(),
            ".." => self.go_to_parent_directory(),
            child_dir => self.go_to_child_directory(child_dir),
        }
    }
}

struct Node<T>
where
    T: PartialEq,
{
    label: String,
    size: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(label: &str, size: T) -> Self {
        Self {
            label: label.to_string(),
            size,
            parent: None,
            children: vec![],
        }
    }
}

type Argument<'a> = &'a str;
type FileSize = u32;

enum Line<'a> {
    ChangeDirectory(Argument<'a>),
    Directory(Argument<'a>),
    File(FileSize),
}

fn parse_line(line: &'_ str) -> Option<Line<'_>> {
    match line.split_once(' ') {
        Some(("$", command)) => match command.split_once(' ') {
            Some(("cd", dir)) => Some(Line::ChangeDirectory(dir)),
            // Ignore the `ls` command.
            _ => None,
        },

        Some(("dir", name)) => Some(Line::Directory(name)),

        Some((size, _)) => Some(Line::File(
            size.parse::<u32>().expect("Failed to parse file size"),
        )),

        _ => unreachable!("Invalid line"),
    }
}

fn parse_input(input: &str) -> Tree<u32> {
    let mut tree: Tree<u32> = Tree::new(vec![Node::new("/", 0)]);

    for line in input.lines() {
        match parse_line(line) {
            Some(Line::ChangeDirectory(directory)) => tree.go_to_directory(directory),
            Some(Line::Directory(dir_name)) => {
                let new_node_index = tree.nodes.len();
                let mut node = Node::new(dir_name, 0);
                node.parent = Some(tree.current_index);
                tree.insert(node);
                tree.nodes[tree.current_index].children.push(new_node_index);
            }
            Some(Line::File(size)) => {
                tree.add_file(tree.current_index, size);
            }
            None => {}
        }
    }

    tree
}

fn part1(tree: &Tree<u32>) -> u32 {
    let max_directory_size = 100_000;

    tree.nodes
        .iter()
        // Find all directories with a total size of at most 100'000
        .filter(|node| node.size <= max_directory_size)
        // Sum their total size
        .fold(0, |a, b| a + b.size)
}

fn part2(tree: &Tree<u32>) -> u32 {
    let file_system_size = 70_000_000;
    let required_unused_space = 30_000_000;
    let root_size = tree.nodes[0].size;

    let deficit = required_unused_space - (file_system_size - root_size);

    let mut possible_nodes = tree
        .nodes
        .iter()
        .filter(|node| node.size >= deficit)
        .collect::<Vec<_>>();

    possible_nodes.sort_by(|a, b| b.size.cmp(&a.size));

    let node = possible_nodes.pop().expect("No node found");

    node.size
}

pub fn solve() {
    let input = include_str!("../../input/day07.txt");
    let file_system = parse_input(input);

    println!("Day 1 Part 1: {:?}", part1(&file_system));
    println!("Day 1 Part 2: {:?}", part2(&file_system));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "$ cd /
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
7214296 k";

    #[test]
    fn day07_part1() {
        let file_system = parse_input(INPUT);
        assert_eq!(part1(&file_system), 95437);
    }

    #[test]
    fn day07_part2() {
        let file_system = parse_input(INPUT);
        assert_eq!(part2(&file_system), 24933642);
    }
}

