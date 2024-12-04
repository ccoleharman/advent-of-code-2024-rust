fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> i32 {
    let input: Vec<&str> = input.split("\r\n").collect();
    let input: Vec<Vec<&str>> = input.iter().map(|line| line.split("").collect()).collect();

    let mut count = 0;

    for i in 1..input.len() - 1 {
        for j in 1..input[i].len() - 1 {
            //check if we found a center A
            if input[i][j] != "A" {continue;} 
            //check if M's are up
            if input[i - 1][j - 1] == "M" && input[i - 1][j + 1] == "M" && input[i + 1][j - 1] == "S" && input[i + 1][j + 1] == "S" {count += 1;}
            //check if M's are right
            if input[i - 1][j + 1] == "M" && input[i + 1][j + 1] == "M" && input[i - 1][j - 1] == "S" && input[i + 1][j - 1] == "S" {count += 1;}
            //check if M's are down
            if input[i + 1][j - 1] == "M" && input[i + 1][j + 1] == "M" && input[i - 1][j - 1] == "S" && input[i - 1][j + 1] == "S" {count += 1;}
            //check if M's are left
            if input[i - 1][j - 1] == "M" && input[i + 1][j - 1] == "M" && input[i - 1][j + 1] == "S" && input[i + 1][j + 1] == "S" {count += 1;}
        }
    }
    return count;
}