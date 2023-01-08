mod days;

fn main() {
    println!("Hello, world!");

    let days = [days::day1::run, days::day2::run, days::day3::run];

    for day in days {
        println!("---------------------------------");
        day();
    }
}
