use std::collections::BinaryHeap;
use std::fs;

use crate::measured;
use crate::Error;

pub const INPUT_PATH: &str = "inputs/day07.txt";

type InputType = Vec<(String, Vec<String>)>;

#[derive(Debug, Clone, PartialEq)]
struct File<'a> {
    name: &'a str,
    size: usize,
}

impl<'a> File<'a> {
    pub fn new(name: &'a str, size: usize) -> Self {
        Self { name, size }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Directory<'a> {
    name: &'a str,
    files: Vec<usize>,
    parent: Option<usize>,
}

impl<'a> Directory<'a> {
    pub fn new(name: &'a str, parent: Option<usize>) -> Self {
        Self {
            name,
            files: Vec::new(),
            parent,
        }
    }

    pub fn add_file(&mut self, file: usize) {
        self.files.push(file);
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Node<'a> {
    File(File<'a>),
    Directory(Directory<'a>),
}

#[allow(unused)]
impl<'a> Node<'a> {
    pub fn new_file(name: &'a str, size: usize) -> Self {
        Self::File(File::new(name, size))
    }

    pub fn new_dir(name: &'a str, parent: Option<usize>) -> Self {
        Self::Directory(Directory::new(name, parent))
    }

    pub fn name(&self) -> &'a str {
        match self {
            Node::File(file) => file.name,
            Node::Directory(dir) => dir.name,
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(..))
    }

    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Directory(..))
    }

    pub fn as_file(&self) -> Option<&File<'a>> {
        match self {
            Self::File(file) => Some(file),
            Self::Directory(_) => None,
        }
    }

    pub fn as_dir(&self) -> Option<&Directory<'a>> {
        match self {
            Self::File(_) => None,
            Self::Directory(dir) => Some(dir),
        }
    }

    pub fn as_file_mut(&mut self) -> Option<&mut File<'a>> {
        match self {
            Self::File(file) => Some(file),
            Self::Directory(_) => None,
        }
    }

    pub fn as_dir_mut(&mut self) -> Option<&mut Directory<'a>> {
        match self {
            Self::File(_) => None,
            Self::Directory(dir) => Some(dir),
        }
    }
}

fn construct_tree<'a>(input: &'a InputType) -> Vec<Node<'a>> {
    let mut nodes = vec![Node::new_dir("/", None)];
    let mut cwd_idx = 0;

    for (cmd, output) in input {
        let mut iter = cmd.split(' ');

        let cmd = iter.next().expect("invalid input");
        let arg = iter.next();

        match (cmd, arg) {
            ("cd", Some("/")) => {
                cwd_idx = 0;
            }
            ("cd", Some("..")) => {
                let cwd = nodes[cwd_idx].as_dir().expect("cwd is not a dir");
                cwd_idx = cwd.parent.expect("missing parent for `cd ..`");
            }
            ("cd", Some(name)) => {
                let cwd = nodes[cwd_idx].as_dir().expect("cwd is not a dir");
                cwd_idx = cwd
                    .files
                    .iter()
                    .copied()
                    .find(|it| nodes[*it].name() == name)
                    .expect("dir not found");
            }
            ("ls", None) => {
                for line in output {
                    let (size, name) = line.split_once(' ').expect("invalid input");
                    match size {
                        "dir" => {
                            let dir = Node::new_dir(name, Some(cwd_idx));
                            let idx = nodes.len();
                            nodes.push(dir);

                            let cwd = nodes[cwd_idx].as_dir_mut().expect("cwd is not a dir");
                            cwd.add_file(idx);
                        }
                        size => {
                            let size = size.parse().expect("invalid input");
                            let dir = Node::new_file(name, size);
                            let idx = nodes.len();
                            nodes.push(dir);

                            let cwd = nodes[cwd_idx].as_dir_mut().expect("cwd is not a dir");
                            cwd.add_file(idx);
                        }
                    }
                }
            }
            _ => {
                panic!("invalid command format");
            }
        }
    }

    nodes
}

#[allow(unused)]
fn print_tree(nodes: &[Node<'_>], idx: usize, level: usize) {
    let indent = " ".repeat(level);

    match &nodes[idx] {
        Node::File(file) => {
            let name = file.name;
            let size = file.size;
            println!("{indent}- {name} (file, size = {size})");
        }
        Node::Directory(dir) => {
            let name = dir.name;
            println!("{indent}- {name} (dir)");
            for file in dir.files.iter().copied() {
                print_tree(nodes, file, level + 1);
            }
        }
    }
}

fn compute_size(nodes: &[Node<'_>], idx: usize) -> usize {
    match &nodes[idx] {
        Node::File(file) => file.size,
        Node::Directory(dir) => dir.files.iter().map(|idx| compute_size(nodes, *idx)).sum(),
    }
}

fn part1(input: &InputType) -> Result<(), Error> {
    let nodes = construct_tree(input);

    let answer: usize = (0..nodes.len())
        .filter_map(|idx| nodes[idx].is_dir().then(|| compute_size(&nodes, idx)))
        .filter(|size| *size <= 100_000)
        .sum();

    println!("part1: {answer}");
    Ok(())
}

fn part2(input: &InputType) -> Result<(), Error> {
    let nodes = construct_tree(input);

    let target = compute_size(&nodes, 0) - 40_000_000;

    let mut heap = (0..nodes.len())
        .filter_map(|idx| nodes[idx].is_dir().then(|| compute_size(&nodes, idx)))
        .map(core::cmp::Reverse)
        .collect::<BinaryHeap<_>>();

    let answer = loop {
        let size = heap.pop().expect("answer not found");
        if size.0 >= target {
            break size.0;
        }
    };

    println!("part2: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Error> {
    let input = fs::read_to_string(INPUT_PATH)?;

    let input = {
        let mut value = Vec::new();
        let mut current = None;

        for line in input.lines() {
            if line.starts_with("$") {
                let cmd = line[2..].to_string();
                let output = Vec::new();

                if let Some(output) = current.replace((cmd, output)) {
                    value.push(output);
                }
            } else {
                if let Some((_, current)) = current.as_mut() {
                    current.push(line.to_string());
                }
            }
        }

        if let Some(cmd) = current.take() {
            value.push(cmd);
        }

        value
    };

    measured!(part1(&input))?;
    measured!(part2(&input))?;

    Ok(())
}
