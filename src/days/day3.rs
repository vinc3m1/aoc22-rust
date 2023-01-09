pub fn run() {
    println!("day3!");

    let anum = 'a' as i32;
    let znum = 'z' as i32;
    let acapnum = 'A' as i32;
    let zcapnum = 'Z' as i32;
    println!("a: {anum} z: {znum} A: {acapnum} Z: {zcapnum}");

    let mut total_priority1 = 0;
    let mut total_priority2 = 0;

    let mut group = vec![""; 3];

    for (i, rucksack) in include_str!("day3.txt").lines().enumerate() {
        // part 1, find duplicates between compartments
        let (comp1, comp2) = rucksack.split_at(rucksack.len()/2);

        'main: for char1 in comp1.chars() {
            if comp2.contains(char1) {
                total_priority1 += char_priority(char1);
                break 'main;
            }
        }

        group[i % 3] = rucksack;

        if i % 3 == 2 {
            for c in group[0].chars() {
                if group[1].contains(c) && group[2].contains(c) {
                    total_priority2 += char_priority(c);
                    break;
                }
            }
        }
    }
    println!("total priority (part1): {total_priority1}");
    println!("total priority (part2): {total_priority2}");

    assert_eq!(total_priority1, 7824);
    assert_eq!(total_priority2, 2798);
}

fn char_priority(char: char) -> i32 {
    if char.is_lowercase() {
        // a-z is 97-122
        return char as i32 - 96;
    } else {
        // A-Z is 65-90
        return char as i32 - 38;
    }
}
