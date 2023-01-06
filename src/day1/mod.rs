use std::include_str;

pub fn day1() {
    println!("day1!");

    let contents = include_str!("input.txt");

    let mut lines = contents.lines();

    let mut line = lines.next();

    let mut max_calories = 0;

    let mut all_calories = Vec::new();

    // loop until end
    while !(line.is_none()) {
        let mut calories = 0;

        // while lines are not empty, accumulate to current calorie count
        while !(line.unwrap_or_default().is_empty()) {
            calories = calories + line.unwrap().parse::<i32>().unwrap();
            line = lines.next();
        }

        // keep track of max calories
        if calories > max_calories {
            max_calories = calories;
        }

        all_calories.push(calories);

        // reset current calorie count after hitting empty line
        line = lines.next();
    }

    println!("max calories: {max_calories}");

    all_calories.sort_unstable();
    all_calories.reverse();
    let top3_calories = &all_calories[..3];

    println!("top calories: {:?}", top3_calories);

    println!("sum top calories: {:?}", top3_calories.iter().sum::<i32>());
}
