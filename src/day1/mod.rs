use std::include_str;

pub fn day1() {
    println!("day1!");

    let contents = include_str!("input.txt");

    let mut lines = contents.lines();

    let mut line = lines.next();

    let mut max_calories = 0;

    while !(line.is_none()) {
        let mut calories = 0;

        while !(line.unwrap_or_default().is_empty()) {
            calories = calories + line.unwrap().parse::<i32>().unwrap();
            line = lines.next();
        }

        if calories > max_calories {
            max_calories = calories
        }

        line = lines.next();
    }

    println!("max calories: {max_calories}");
}
