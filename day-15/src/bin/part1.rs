fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut map: Vec<Vec<char>> = input[0].lines().map(|line| line.chars().collect()).collect();
    let moves: Vec<char> = input[1].lines().collect::<Vec<_>>().join("").chars().collect();

    let mut rob_pos = (0, 0); //x, y position of robot
    'find_robot: for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '@' {
                rob_pos = (j, i);
                break 'find_robot;
            }
        }
    }

    for m in moves.iter() {
        let (mut x, mut y) = rob_pos;
        if *m == '<' {
            x -= 1;
            while map[y][x] == 'O' {
                x -= 1;
            }
            if map[y][x] == '.' {
                map[y][x] = 'O';
                map[rob_pos.1][rob_pos.0] = '.';
                rob_pos.0 -= 1;
                map[rob_pos.1][rob_pos.0] = '@';
            }
        } else if *m == '^' {
            y -= 1;
            while map[y][x] == 'O' {
                y -= 1;
            }
            if map[y][x] == '.' {
                map[y][x] = 'O';
                map[rob_pos.1][rob_pos.0] = '.';
                rob_pos.1 -= 1;
                map[rob_pos.1][rob_pos.0] = '@';
            }
        } else if *m == '>' {
            x += 1;
            while map[y][x] == 'O' {
                x += 1;
            }
            if map[y][x] == '.' {
                map[y][x] = 'O';
                map[rob_pos.1][rob_pos.0] = '.';
                rob_pos.0 += 1;
                map[rob_pos.1][rob_pos.0] = '@';
            }
        } else if *m == 'v' {
            y += 1;
            while map[y][x] == 'O' {
                y += 1;
            }
            if map[y][x] == '.' {
                map[y][x] = 'O';
                map[rob_pos.1][rob_pos.0] = '.';
                rob_pos.1 += 1;
                map[rob_pos.1][rob_pos.0] = '@';
            }
        }
    }

    let mut gps_sum = 0;
    
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'O' {
                gps_sum += i * 100 + j;
            }
        }
    }

    return gps_sum as i32;
}