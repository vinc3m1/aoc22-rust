use std::collections::HashSet;

pub fn run() {
    println!("day 8!");

    let input = include_str!("day8.txt");

    let forest: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|s| s.len() == 1)
                .map(|s| s.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let height = forest.len();
    let width = forest[0].len();

    let mut visible_trees = HashSet::new();

    for row in 0..height {
        // 2 ranges for forward and backwards
        let ranges: Vec<Box<dyn Iterator<Item = usize>>> =
            vec![Box::new(0..width), Box::new((0..width).rev())];
        for range in ranges {
            let mut tallest = -1;
            for col in range {
                let tree = forest[row][col];
                if tree > tallest {
                    visible_trees.insert((row, col));
                    tallest = tree;
                }
            }
        }
    }

    for col in 0..width {
        // 2 ranges for forward and backwards
        let ranges: Vec<Box<dyn Iterator<Item = usize>>> =
            vec![Box::new(0..height), Box::new((0..height).rev())];
        for range in ranges {
            let mut tallest = -1;
            for row in range {
                let tree = forest[row][col];
                if tree > tallest {
                    visible_trees.insert((row, col));
                    tallest = tree;
                }
            }
        }
    }

    println!("visible trees: {}", visible_trees.len());
    assert_eq!(visible_trees.len(), 1690)
}
