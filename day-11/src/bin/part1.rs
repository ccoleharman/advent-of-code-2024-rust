fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    let mut stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    for _ in 0..25 { //blink
        let mut i = 0;
        while i < stones.len() { //go to each stone
            let stone = stones[i].clone();
            if stone == "0" { //if stone is 0
                stones[i] = "1".to_string();
                i += 1;
            } else if stone.len() % 2 == 0 { //split stone into 2
                stones[i] = stone.get(0..stone.len() / 2).unwrap().to_string();
                stones.insert(i + 1, stone.get(stone.len() / 2..stone.len()).unwrap().parse::<i64>().unwrap().to_string());
                i += 2;
            } else { //multiply by 2024
                stones[i] = (stone.parse::<i64>().unwrap() * 2024).to_string();
                i += 1;
            }
        }
    }

    return stones.len() as i32;
}