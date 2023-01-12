use std::rc::Rc;

use itertools::Itertools;

pub fn run() {
    println!("day7!");

    for line in include_str!("day7.txt").lines() {
        match line.split_whitespace().collect_vec()[..] {
            ["$", "cd", ".."] => {
                println!("cd up to parent");
            }
            ["$", "cd", dir] => {
                println!("cd into {dir}");
            }
            ["$", "ls"] => {
                println!("list current dir");
            }
            ["dir", dir_name] => {
                println!("file is directory: {dir_name}");
            }
            [size_s, name] => {
                let size = size_s.parse::<usize>().unwrap();
                println!("file {name} with size {size}");
            }
            _ => {
                println!("UNKNOWN INPUT!: {line}");
            }
        }
    }
}

struct File {
    name: String,
    kind: FileKind,
    parent: Option<Rc<File>>,
    children: Option<Vec<File>>,
    size: usize,
}

enum FileKind {
    File,
    Dir,
}
