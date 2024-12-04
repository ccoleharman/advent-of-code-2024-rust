fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    let input: Vec<&str> = input.split("\r\n").collect();
    let input: Vec<Vec<&str>> = input.iter().map(|line| line.split("").collect()).collect();

    let mut count = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            //check if we found a starting X
            if input[i][j] != "X" {continue;} 
            //check up
            if i >= 3 && input[i - 1][j] == "M" && input[i - 2][j] == "A" && input[i - 3][j] == "S" {count += 1;}
            //check up right
            if i >= 3 && j <= input[i].len() - 4 && input[i - 1][j + 1] == "M" && input[i - 2][j + 2] == "A" && input[i - 3][j + 3] == "S" {count += 1;} 
            //check right
            if j <= input[i].len() - 4 && &input[i][j + 1..=j + 3].join("") == "MAS" {count += 1;} 
            //check down right
            if i <= input.len() - 4 && j <= input[i].len() - 3 && input[i + 1][j + 1] == "M" && input[i + 2][j + 2] == "A" && input[i + 3][j + 3] == "S" {count += 1;}
            //check down
            if i <= input.len() - 4 && input[i + 1][j] == "M" && input[i + 2][j] == "A" && input[i + 3][j] == "S" {count += 1;}
            //check down left
            if i <= input.len() - 4 && j >= 3 && input[i + 1][j - 1] == "M" && input[i + 2][j - 2] == "A" && input[i + 3][j - 3] == "S" {count += 1;} 
            //check left
            if j >= 3 && &input[i][j - 3..=j - 1].join("") == "SAM" {count += 1;} 
            //check up left
            if i >= 3 && j >= 3 && input[i - 1][j - 1] == "M" && input[i - 2][j - 2] == "A" && input[i - 3][j - 3] == "S" {count += 1;} 
         }

    }
    return count;
}