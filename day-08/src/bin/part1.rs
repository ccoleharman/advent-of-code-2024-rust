fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    let input: Vec<Vec<char>> = input.lines().map(|sub| sub.chars().collect()).collect();
    let mut positions: Vec<Vec<usize>> = Vec::new(); //initialize answer vector

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '.' { //allows to skip all non tower elements
                continue;
            }
            let freq = input[i][j]; //catches frequency

            for x in 0..input.len() {
                for y in 0..input[x].len() {
                    if input[x][y] == freq 
                    && (x != i || y != j) 
                    && 2 * i as i32 - x as i32 >= 0 
                    && 2 * j as i32 - y as i32 >= 0 
                    && !positions.contains(&vec![2 * i - x, 2 * j - y]) 
                    && (2 * i as i32 - x as i32) < (input.len() as i32)  
                    && (2 * j as i32 - y as i32) < (input[i].len() as i32)
                    {
                        //if statement ensures that all conditions are met before adding the antinode to the positions vector
                        positions.push(vec![(2 * i) - x, (2 * j) - y]);
                    }
                }
            }
        }
    }
    return positions.len() as i32;
}