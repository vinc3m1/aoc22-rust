use itertools::Itertools;

pub fn day2() {
    println!("day 2!");

    let contents = include_str!("input.txt");

    let mut lines = contents.lines();

    let mut score = 0;
    while let Some(line) = lines.next() {
        if let Some((op_choice, my_choice)) = line.split_whitespace().collect_tuple() {

            score += match my_choice {
                "X" => 1 + match op_choice { // rock
                    "A" => 3, // rock - draw
                    "B" => 0, // paper - lose
                    "C" => 6, // scissors - win
                    _ => panic!(),
                },
                "Y" => 2 + match op_choice { // paper
                    "A" => 6, // rock - win
                    "B" => 3, // paper - draw
                    "C" => 0, // scissors - lose
                    _ => panic!(),
                },
                "Z" => 3 + match op_choice { // scissors
                    "A" => 0, // rock - lose
                    "B" => 6, // paper - win
                    "C" => 3, // scissors - draw
                    _ => panic!(),
                },
                _ => panic!(),
            };
        }
    }

    println!("score: {score}")
}
