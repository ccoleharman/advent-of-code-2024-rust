fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i64 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> i64 {
    let input: Vec<String> = input.split("\r\n\r\n").map(|s| s.to_string()).collect::<Vec<_>>();
    let mut sets: Vec<Vec<i64>> = Vec::new();
    for group in input {
        let mut i = 0;
        let mut set: Vec<i64> = Vec::new();
        let mut num: String = String::new();

        while i < group.len() {
            if !group.chars().nth(i).unwrap().is_digit(10) {
                if num.len() != 0 {
                    set.push(num.parse::<i64>().unwrap());
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
        set.push(num.parse::<i64>().unwrap());

        set[4] += 10000000000000;
        set[5] += 10000000000000;
        sets.push(set);
    }

    let mut coins: i64 = 0;

    for set in sets.iter() {
        let x: f64 = (set[4] as f64 * set[3] as f64 - set[5] as f64 * set[2] as f64) / (set[0] as f64 * set[3] as f64 - set[1] as f64 * set[2] as f64);
        let y: f64 = (set[4] as f64 - set[0] as f64 * x) / set[2] as f64;
        if x % 1.0 == 0.0 && y % 1.0 == 0.0 && x > 0.0 && y > 0.0{
            coins += (x * 3.0 + y) as i64;
        }
    }

    return coins;
}
