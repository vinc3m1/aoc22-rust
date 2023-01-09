use itertools::Itertools;

pub fn run() {
    println!("day5!");

    let mut stacks1 = vec![
        String::from("DLJRVGF"),  // 1
        String::from("TPMBVHJS"), // 2
        String::from("VHMFDGPC"), // 3
        String::from("MDPNGQ"),   // 4
        String::from("JLHNF"),    // 5
        String::from("NFVQDGTZ"), // 6
        String::from("FDBL"),     // 7
        String::from("MJBSVDN"),  // 8
        String::from("GLD"),      // 9
    ];
    let mut stacks2 = stacks1.clone();

    for line in include_str!("day5.txt").lines() {
        // Pattern match count, from, to as strings
        if let Some(("move", count_s, "from", from_s, "to", to_s)) = line.split(" ").collect_tuple()
        {
            // parse strings into usize
            let count = count_s.parse::<usize>().unwrap();
            // subtract 1 from from/to to change from 1 index to 0 index
            let from = from_s.parse::<usize>().unwrap() - 1;
            let to = to_s.parse::<usize>().unwrap() - 1;

            // part 1, move each crate one by one
            for _ in 0..count {
                // stack_to.push(stack_from.pop().unwrap());
                let item = stacks1[from].pop().unwrap();
                stacks1[to].push(item);
            }

            // part 2, move all at once
            let split_idx = stacks2[from].len() - count;
            let moving = stacks2[from].split_off(split_idx);
            stacks2[to].push_str(&moving);
        }
    }

    let top_stacks1: String = stacks1.iter().map(|s| s.chars().last().unwrap()).collect();
    println!("top of stacks pt 1: {top_stacks1}");
    assert_eq!(top_stacks1, "QMBMJDFTD");

    let top_stacks2: String = stacks2.iter().map(|s| s.chars().last().unwrap()).collect();
    println!("top of stacks pt 2: {top_stacks2}");
    assert_eq!(top_stacks2, "NBTVTJNFJ");
}
