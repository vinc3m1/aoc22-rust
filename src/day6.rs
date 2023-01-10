use std::collections::HashSet;

pub fn run() {
    let input = include_str!("day6.txt");

    let mut buffer = vec![' '; 4];
    let mut set: HashSet<char> = HashSet::new();

    for (i, c) in input.chars().enumerate() {
        {
            buffer[i % 4] = c;
        }

        set.clear();
        set.extend(buffer.iter());
        if i > 3 && set.len() == 4 {
            println!("set: {:?}", set);
            println!("packet start: {:?}", i + 1); // add 1 because we count 1 indexed

            assert_eq!(i + 1, 1953);
            return;
        }
    }
}
