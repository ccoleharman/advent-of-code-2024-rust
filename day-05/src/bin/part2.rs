fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); // Read in input
    let answer: i32 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}
//code is not optimized
fn part2(input: String) -> i32 {
    // Initialize rules and updates to make them readable
    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    let rules: Vec<&str> = input[0].split("\r\n").collect();
    let rules: Vec<Vec<&str>> = rules.iter().map(|rule| rule.split("|").collect()).collect();
    let updates: Vec<&str> = input[1].split("\r\n").collect();
    let updates: Vec<Vec<&str>> = updates.iter().map(|update| update.split(",").collect()).collect();

    // Initialize vector holding the incorrect updates
    let mut incorrect_updates: Vec<Vec<&str>> = Vec::new();

    // Loop through each update and check the rules to make sure that they are correct
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
            }
            if !correct { break; }
        }
        if !correct { incorrect_updates.push(update.to_vec()); }
    }

    // Fix incorrect updates and add middle number to sum
    let mut sum: i32 = 0;
    for update in incorrect_updates.iter_mut() {
        let mut i: usize = 0;
        while i < update.len() {
            for rule in rules.iter() {
                if rule[0] == update[i] {
                    let pos = update.iter().position(|&x| x == rule[1]);
                    if pos != None && pos < Some(i) {
                        update.remove(pos.unwrap());
                        update.insert(i, rule[1]); 
                        i = 0;
                    }
                }
            }
            i += 1;
        }
        let mid: usize = (update.len() / 2) as usize;
        sum += update[mid].parse::<i32>().unwrap();
    }

    return sum;
}
