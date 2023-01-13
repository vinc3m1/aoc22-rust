use itertools::Itertools;

pub fn run() {
    println!("day 7!");

    let mut tree = ArenaTree::new();
    let mut current_dir: usize = 0;

    for line in include_str!("day7.txt").lines().skip(1) {
        match line.split_whitespace().collect_vec()[..] {
            ["$", "cd", ".."] => {
                // cd into parent dir
                current_dir = tree.parent(current_dir).unwrap();
            }
            ["$", "cd", child] => {
                // cd into child dir
                current_dir = tree.child_by_name(current_dir, child).unwrap();
            }
            ["$", "ls"] => {
                // do nothing
            }
            ["dir", dir_name] => {
                // add child directory
                tree.new_dir(current_dir, dir_name);
            }
            [size_s, name] => {
                // add child file of size
                let size = size_s.parse::<usize>().unwrap();
                tree.new_file(current_dir, name, size);
            }
            _ => {
                unreachable!("unknown input");
            }
        }
    }

    let at_most_100k: usize = tree
        .dir_sizes()
        .iter()
        .filter(|&&size| size <= 100000)
        .sum();

    println!("part 1 -- sum of dirs at most 100000: {:?}", at_most_100k);
    assert_eq!(at_most_100k, 1315285);

    let total_disk_space: usize = 70000000;
    let need_unused_space: usize = 30000000;
    let needed_size = need_unused_space - (total_disk_space - tree.root_size());

    let mut dir_sizes = tree.dir_sizes();
    dir_sizes.sort_unstable();
    let dir_to_delete_size = dir_sizes
        .iter()
        .find(|&&size| size > needed_size)
        .copied()
        .unwrap();

    println!("part 2 -- size of dir to delete: {:?}", dir_to_delete_size);
    assert_eq!(dir_to_delete_size, 9847279);
}

struct ArenaTree {
    arena: Vec<File>,
}

impl ArenaTree {
    fn new() -> Self {
        let root = File {
            name: String::from("/"),
            size: 0,
            parent: None,
            children: Some(Vec::new()),
        };

        Self { arena: vec![root] }
    }

    fn parent(&self, idx: usize) -> Option<usize> {
        return self.arena[idx].parent;
    }

    fn child_by_name(&self, parent: usize, name: &str) -> Option<usize> {
        return self.arena[parent]
            .children
            .as_ref()?
            .iter()
            .find(|i| self.arena[**i].name == name)
            .copied();
    }

    fn new_dir(&mut self, parent: usize, name: &str) {
        let new_idx = self.arena.len();

        let file = File {
            name: String::from(name),
            size: 0,
            parent: Some(parent),
            children: Some(Vec::new()),
        };

        self.arena.push(file);

        self.arena[parent].children.as_mut().unwrap().push(new_idx);
    }

    fn new_file(&mut self, parent: usize, name: &str, size: usize) {
        let new_idx = self.arena.len();

        let file = File {
            name: String::from(name),
            size,
            parent: Some(parent),
            children: None,
        };

        self.arena.push(file);

        let mut parent = self.arena.get_mut(parent).unwrap();
        parent.children.as_mut().unwrap().push(new_idx);
        parent.size += size;

        // also loop up to root to update sizes for all parent dirs
        while let Some(next_parent) = parent.parent {
            parent = self.arena.get_mut(next_parent).unwrap();
            parent.size += size;
        }
    }

    fn dir_sizes(&self) -> Vec<usize> {
        return self
            .arena
            .iter()
            .filter(|f| f.children.is_some())
            .map(|f| f.size)
            .collect();
    }

    fn root_size(&self) -> usize {
        self.arena[0].size
    }
}

struct File {
    name: String,
    size: usize,
    parent: Option<usize>,
    children: Option<Vec<usize>>,
}
