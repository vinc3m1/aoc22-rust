use itertools::Itertools;

pub fn run() {
    println!("day4!");

    let mut contained = 0;
    for line in include_str!("day4.txt").lines() {
        let pair: AssignmentPair = line.into();

        if pair.0.contains(&pair.1) || pair.1.contains(&pair.0) {
            contained += 1;
        }
    }

    println!("full contained: {contained}");
    assert_eq!(contained, 576);
}

struct AssignmentPair(Assignment, Assignment);

impl From<&str> for AssignmentPair {
    fn from(line: &str) -> Self {
        let (a_start, a_end, b_start, b_end) = line
            .split(&['-', ','][..])
            .map(|s| s.parse::<i32>().unwrap())
            .collect_tuple()
            .unwrap();
        return AssignmentPair(Assignment(a_start, a_end), Assignment(b_start, b_end));
    }
}

struct Assignment(i32, i32);

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}
