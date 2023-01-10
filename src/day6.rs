use std::collections::HashSet;

pub fn run() {
    println!("day6!");

    let input = include_str!("day6.txt");

    let mut buf1 = vec![' '; 4];
    let mut set1: HashSet<char> = HashSet::new();

    let mut buf2 = vec![' '; 14];
    let mut set2: HashSet<char> = HashSet::new();

    let mut packet_start = 0;
    let mut message_start = 0;
    println!("packet start {:?}", packet_start);
    println!("message start {:?}", message_start);

    for (i, c) in input.chars().enumerate() {
        if packet_start == 0 {
            {
                buf1[i % 4] = c;
            }
            set1.clear();
            set1.extend(buf1.iter());
            if i > 3 && set1.len() == 4 {
                packet_start = i + 1;
            }
        }
        if message_start == 0 {
            {
                buf2[i % 14] = c;
            }
            set2.clear();
            set2.extend(buf2.iter());
            if i > 13 && set2.len() == 14 {
                message_start = i + 1;
            }
        }
    }
    println!("packet start: {packet_start}"); // add 1 because we count 1 indexed
    assert_eq!(packet_start, 1953);
    println!("message start: {message_start}"); // add 1 because we count 1 indexed
    assert_eq!(message_start, 2301);
    return;
}
