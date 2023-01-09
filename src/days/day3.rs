pub fn run() {
    println!("day3!");

    let anum = 'a' as i32;
    let znum = 'z' as i32;
    let acapnum = 'A' as i32;
    let zcapnum = 'Z' as i32;
    println!("a: {anum} z: {znum} A: {acapnum} Z: {zcapnum}");

    let mut total_priority1 = 0;
    let mut total_priority2 = 0;

    let mut group_shared = Vec::new();

    for (i, line) in include_str!("day3.txt").lines().enumerate() {
        // part 1, find duplicates between compartments
        let comp1 = &line[..line.len() / 2];
        let comp2 = &line[line.len() / 2..];

        'main: for char1 in comp1.chars() {
            if comp2.contains(char1) {
                total_priority1 += char_priority(char1);
                break 'main;
            }
        }

        // part2 find shared items among 3
        if i % 3 == 0 {
            // first member of group, reset and add all elements of first bag, dedup
            group_shared.clear();
            group_shared.extend(line.chars());
            group_shared.sort_unstable();
            group_shared.dedup();
        } else {
            // filter by shared characters
            group_shared.retain(|c| line.contains(*c))
        }
        if i % 3 == 2 {
            total_priority2 += char_priority(group_shared.pop().unwrap());
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
