pub fn run() {
    println!("day 9!");

    let input = include_str!("day9.txt");

    let mut head_position = (0, 0);

    for line in input.lines() {
        if let Some((direction, moves_s)) = line.split_once(" ") {
            let moves: i32 = moves_s.parse().unwrap();

            let dir = match direction {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => unreachable!(),
            };

            for _ in 0..moves {
                head_position.0 += dir.0;
                head_position.1 += dir.1;

                
            }
        }
    }
}
