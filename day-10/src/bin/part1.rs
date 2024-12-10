fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}
fn part1(input: String) -> i32 {
    let input: Vec<Vec<u32>> = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect();

    fn search(i: usize, j: usize, input: &Vec<Vec<u32>>) -> Vec<Vec<usize>> {
        if input[i][j] == 9 {
            return vec![vec![i, j]];
        }

        let mut ends: Vec<Vec<usize>> = Vec::new();
    
        let num: u32 = input[i][j];
        if i > 0 && input[i - 1][j] == num + 1 {
            ends.append(&mut search(i - 1, j, input));
        }
    
        if j > 0 && input[i][j - 1] == num + 1 {
            ends.append(&mut search(i, j - 1, input));
        }
    
        if i < input.len() - 1 && input[i + 1][j] == num + 1 {
            ends.append(&mut search(i + 1, j, input));
        }
    
        if j < input[0].len() - 1 && input[i][j + 1] == num + 1 {
            ends.append(&mut search(i, j + 1, input));
        }

        let mut unique = Vec::new();
        for val in ends {
            if !unique.contains(&val) {
                unique.push(val);
            }
        }

        return unique;
    }

    let mut ends: Vec<Vec<usize>> = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 0 {
                ends.append(&mut search(i, j, &input));
            }
        }
    }

    return ends.len() as i32;
}
