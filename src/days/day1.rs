pub fn run() {
    println!("day 1!");

    let mut max_calories = 0;

    let mut top3_calories = vec![0; 3];

    let mut calories = 0;

    // loop through all lines
    for line in include_str!("day1.txt").lines() {
        // while lines are not empty, accumulate to current calorie count
        if !line.is_empty() {
            calories += line.parse::<i32>().unwrap();

            // loop and continue to sum calories
            continue;
        }

        // otherwise, if the line is empty, track summed calories and reset count

        // remember max calories
        if calories > max_calories {
            max_calories = calories;
        }

        // if any of the top 3 calories are smaller than the current elf's,
        // find the smallest one and replace it with the current elf's calories
        if let Some(min_top) = top3_calories.iter_mut().filter(|x| **x < calories).min() {
            *min_top = calories;
        }

        while top3_calories.len() > 3 {
            top3_calories.pop();
        }

        // reset calories for next elf
        calories = 0;
    }

    println!("max calories: {max_calories}");

    println!("top calories: {:?}", top3_calories);

    let top3 = top3_calories.iter().sum::<i32>();

    println!("sum top calories: {:?}", top3);

    assert_eq!(max_calories, 68467);
    assert_eq!(top3, 203420);
}
