use std::collections::{HashSet, VecDeque};

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut patterns: Vec<String> = input[0].split(", ").map(|s| s.to_string()).collect();
    patterns.sort_by(|a, b| (a.len()).cmp(&b.len()).reverse());
    let tests: Vec<&str> = input[1].lines().collect();

    let mut towels: VecDeque<(usize, &str)> = VecDeque::new();
    for (i, test) in tests.iter().enumerate() {
        towels.push_back((i, test));
    }

    let mut count: HashSet<usize> = HashSet::new();
    while !towels.is_empty() {
        let (i, test) = towels.pop_front().unwrap();
        if count.contains(&i) {
            continue;
        }
        if test.len() == 0 {
            count.insert(i);
            continue;
        }
        for pattern in patterns.iter() {
            if test.starts_with(pattern) {
                let r = test.get(pattern.len()..).unwrap();
                towels.push_front((i, r));
            }
        }
    }

    return count.len() as i32;
}