fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    //initialize rules and updates to make them readable
    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    let rules: Vec<&str> = input[0].split("\r\n").collect();
    let rules: Vec<Vec<&str>> = rules.iter().map(|rule| rule.split("|").collect()).collect();
    let updates: Vec<&str> = input[1].split("\r\n").collect();
    let updates: Vec<Vec<&str>> = updates.iter().map(|update| update.split(",").collect()).collect();

    //initialize vector holding the correct updates
    let mut correct_updates: Vec<&Vec<&str>> = Vec::new();

    //loop through each update and check the rules to make sure that they are correct
    for update in updates.iter() {
        let mut correct = true;
        for i in 0..update.len() {
            for rule in rules.iter() {
                if rule[0] == update[i] {
                    let pos = update.iter().position(|&x| x == rule[1]);
                    if pos != None && pos < Some(i) {
                        correct = false;
                        break;
                    }
                }
                if !correct {break;}
            }
        }
        if correct {correct_updates.push(update);}
    }

    //find middle number of correct updates and add to sum
    let mut sum: i32 = 0;
    for update in correct_updates.iter() {
        let mid: usize = (update.len() / 2) as i32 as usize;
        sum += update[mid].parse::<i32>().unwrap();
    }

    return sum;
}
