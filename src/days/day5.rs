use itertools::Itertools;

pub fn run() {
    println!("day5!");

    let mut stacks = vec![
        "DLJRVGF",  // 1
        "TPMBVHJS", // 2
        "VHMFDGPC", // 3
        "MDPNGQ",   // 4
        "JLHNF",    // 5
        "NFVQDGTZ", // 6
        "FDBL",     // 7
        "MJBSVDN",  // 8
        "GLD",      // 9
    ]
    .iter()
    .map(|s| s.chars().collect_vec())
    .collect_vec();

    for line in include_str!("day5.txt").lines() {
        if let Some(("move", count_s, "from", from_s, "to", to_s)) = line.split(" ").collect_tuple()
        {
            let count = count_s.parse::<i32>().unwrap();

            // subtract 1 to change from 1 index to 0 index
            let from = from_s.parse::<usize>().unwrap() - 1;
            let to = to_s.parse::<usize>().unwrap() - 1;

            for _ in 0..count {
                // stack_to.push(stack_from.pop().unwrap());
                let item = stacks[from].pop().unwrap();
                stacks[to].push(item);
            }
        }
    }

    let top_stacks: String = stacks.iter().map(|s| s.last().unwrap()).collect();

    println!("top of stacks: {top_stacks}");
}
