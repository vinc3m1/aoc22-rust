use std::cmp::max;
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

    // PART 1

    let mut visible_trees = HashSet::new();

    for row in 0..height {
        // 2 ranges for forward and backwards
        let ranges: [Box<dyn Iterator<Item = usize>>; 2] =
            [Box::new(0..width), Box::new((0..width).rev())];
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
        let ranges: [Box<dyn Iterator<Item = usize>>; 2] =
            [Box::new(0..height), Box::new((0..height).rev())];
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
    assert_eq!(visible_trees.len(), 1690);

    // PART 2

    let mut max_score = 0;

    // ignore outer columns because their scores are all 0
    for row in 1..height - 1 {
        for col in 1..width - 1 {
            let tree = forest[row][col];

            // go north
            let mut north_score = 0;
            for test_row in (0..row).rev() {
                north_score += 1;
                if forest[test_row][col] >= tree {
                    break;
                }
            }

            // go south
            let mut south_score = 0;
            for test_row in row + 1..height {
                south_score += 1;
                if forest[test_row][col] >= tree {
                    break;
                }
            }

            // go west
            let mut west_score = 0;
            for test_col in (0..col).rev() {
                west_score += 1;
                if forest[row][test_col] >= tree {
                    break;
                }
            }

            // go east
            let mut east_score = 0;
            for test_col in col + 1..width {
                east_score += 1;
                if forest[row][test_col] >= tree {
                    break;
                }
            }

            max_score = max(
                north_score * south_score * west_score * east_score,
                max_score,
            );
        }
    }

    println!("max scenic score: {}", max_score);
    assert_eq!(max_score, 535680);
}
