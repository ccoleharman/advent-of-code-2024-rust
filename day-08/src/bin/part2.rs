fn main() {
    // Read input from the file
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input");
    let answer: i32 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> i32 {
    // Parse the input into a 2D grid of characters
    let input: Vec<Vec<char>> = input.lines().map(|sub| sub.chars().collect()).collect();
    let mut positions: Vec<[usize; 2]> = Vec::new(); // Use [usize; 2] instead of Vec<usize> for coordinates

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '.' {
                // Skip all non-tower elements
                continue;
            }

            // Add the current position if it's not already in the list
            let current_position = [i, j];
            if !positions.contains(&current_position) {
                positions.push(current_position);
            }

            let freq = input[i][j]; // Frequency of the current element

            for x in 0..input.len() {
                for y in 0..input[x].len() {
                    if input[x][y] == freq && (x != i || y != j) {
                        // Add the current position again if it's not already in the list
                        if !positions.contains(&current_position) {
                            positions.push(current_position);
                        }

                        // Calculate the slope to the next position
                        let slope: [isize; 2] = if i == x {
                            [0, y as isize - j as isize] // Vertical slope
                        } else if (y as isize - j as isize) % (x as isize - i as isize) == 0 {
                            [1, (y as isize - j as isize) / (x as isize - i as isize)] // Integer slope
                        } else {
                            [x as isize - i as isize, y as isize - j as isize] // General slope
                        };

                        let mut position: [isize; 2] = [i as isize, j as isize];
                        while position[0] - slope[0] >= 0
                            && position[1] - slope[1] >= 0
                            && (position[0] - slope[0]) < input.len() as isize
                            && (position[1] - slope[1]) < input[i].len() as isize
                        {
                            position[0] -= slope[0];
                            position[1] -= slope[1];

                            let new_position = [position[0] as usize, position[1] as usize];
                            if !positions.contains(&new_position) {
                                positions.push(new_position);
                            }
                        }

                        let mut position: [isize; 2] = [i as isize, j as isize];
                        while position[0] + slope[0] >= 0
                            && position[1] + slope[1] >= 0
                            && (position[0] + slope[0]) < input.len() as isize
                            && (position[1] + slope[1]) < input[i].len() as isize
                        {
                            position[0] += slope[0];
                            position[1] += slope[1];

                            let new_position = [position[0] as usize, position[1] as usize];
                            if !positions.contains(&new_position) {
                                positions.push(new_position);
                            }
                        }

                    }
                }
            }
        }
    }
    positions.len() as i32
}
