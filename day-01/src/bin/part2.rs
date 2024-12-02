use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> i32 {
    let re = Regex::new(r"\s+|\r\n").unwrap(); //regex
    let input: Vec<&str> = re.split(&input).collect();

    //break input into two vectors
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut is_left: bool = true;
    for num in input.iter() {
        let int_num: i32 = num.parse().expect("{num} is not a number");
        if is_left {
            left.push(int_num);
            is_left = false;
        } else {
            right.push(int_num);
            is_left = true;
        }
    }
    left.sort();
    right.sort();

    let mut score: i32 = 0;

    for num_left in left.iter() {
        for num_right in right.iter() {
            if num_left == num_right {
                score += num_left;
            }
        }
    }
    return score;
}