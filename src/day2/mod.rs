use itertools::Itertools;

pub fn day2() {
    println!("day 2!");

    let contents = include_str!("input.txt");

    let mut lines = contents.lines();

    let mut score_p1 = 0;
    let mut score_p2 = 0;
    while let Some(line) = lines.next() {
        if let Some((op_choice, my_choice)) = line.split_whitespace().collect_tuple() {

            // part 1: we think x, y, z mean rock, paper, scissors
            score_p1 += match my_choice {
                "X" => 1 + match op_choice { // rock
                    "A" => 3, // rock - draw
                    "B" => 0, // paper - lose
                    "C" => 6, // scissors - win
                    _ => unreachable!(),
                },
                "Y" => 2 + match op_choice { // paper
                    "A" => 6, // rock - win
                    "B" => 3, // paper - draw
                    "C" => 0, // scissors - lose
                    _ => unreachable!(),
                },
                "Z" => 3 + match op_choice { // scissors
                    "A" => 0, // rock - lose
                    "B" => 6, // paper - win
                    "C" => 3, // scissors - draw
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };


            // part 2: x, y, z, mean lose, draw, win
            let result = my_choice;
            score_p2 += match result {
                "X" => 0 + match op_choice { // lose
                    "A" => 3, // rock - scissors
                    "B" => 1, // paper - rock
                    "C" => 2, // scissors - paper
                    _ => unreachable!(),
                },
                "Y" => 3 + match op_choice { // draw
                    "A" => 1, // rock - rock
                    "B" => 2, // paper - paper
                    "C" => 3, // scissors - scissors
                    _ => unreachable!(),
                },
                "Z" => 6 + match op_choice { // win
                    "A" => 2, // rock - paper
                    "B" => 3, // paper - scissors
                    "C" => 1, // scissors - rock
                    _ => unreachable!(),
                },
                _ => unreachable!(),
            };
        }
    }

    println!("score p1: {score_p1}");
    println!("score p2: {score_p2}");
}
