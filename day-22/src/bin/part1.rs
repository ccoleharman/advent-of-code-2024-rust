fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i64 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i64 {
    let input: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();

    let mut total = 0;
    for mut secret in input {
        for _ in 0..2000 {
            secret ^= secret * 64;
            secret %= 16777216;
            secret ^= secret >> 5;
            secret %= 16777216;
            secret ^= secret * 2048;
            secret %= 16777216;
        }
        total += secret;
    }

    return total;
}