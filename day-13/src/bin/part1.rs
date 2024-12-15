fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    let input: Vec<String> = input.split("\r\n\r\n").map(|s| s.to_string()).collect::<Vec<_>>();
    let mut sets: Vec<Vec<i32>> = Vec::new();
    for group in input {
        let mut i = 0;
        let mut set: Vec<i32> = Vec::new();
        let mut num: String = String::new();

        while i < group.len() {
            if !group.chars().nth(i).unwrap().is_digit(10) {
                if num.len() != 0 {
                    set.push(num.parse::<i32>().unwrap());
                    num.clear();
                }
                i += 1;
                continue;
            }

            loop {
                if i < group.len() && group.chars().nth(i).unwrap().is_digit(10) {
                    num.push_str(&group.chars().nth(i).unwrap().to_string());
                    i += 1;
                } else {
                    i += 1;
                    break;
                }
            }
        }
        set.push(num.parse::<i32>().unwrap());
        sets.push(set);
    }

    let mut coins = 0;

    for set in sets.iter() {
        for a in 0..=100 {
            for b in 0..=100 {
                if a as i32 * set[0] + b as i32 * set[2] == set[4] && a as i32 * set[1] + b as i32 * set[3] == set[5] {
                    coins += a * 3 + b;
                }
            }
        }
    }

    return coins;
}
