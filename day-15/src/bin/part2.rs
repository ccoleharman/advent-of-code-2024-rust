fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}
fn part2(input: String) -> i32 {
    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut map: Vec<Vec<char>> = input[0].lines().map(|line| line.chars().collect()).collect();
    let moves: Vec<char> = input[1].lines().collect::<Vec<_>>().join("").chars().collect();

    fn search_up(i: usize, j: usize, map: &Vec<Vec<char>>) -> (Vec<(usize, usize)>, bool) {
        let mut path = vec![(j, i)];
        let mut found = true;
        if map[i - 1][j] == '.' {
            return (path, found);
        }
        if map[i][j] == '[' {
            if map[i - 1][j] == '[' {
                let search = search_up(i - 1, j, map);
                path.extend(search.0);
                found = found && search.1;
            } else if map[i - 1][j] == ']' {
                let search = search_up(i - 1, j, map);
                path.extend(search.0);
                found = found && search.1;
                let search = search_up(i - 1, j - 1, map);
                path.extend(search.0);
                found = found && search.1;
            } else {
                found = false;
            }
        } else if map[i][j] == ']' {
            if map[i - 1][j] == ']' {
                let search = search_up(i - 1, j, map);
                path.extend(search.0);
                found = found && search.1;
            } else if map[i - 1][j] == '[' {
                let search = search_up(i - 1, j, map);
                path.extend(search.0);
                found = found && search.1;
                let search = search_up(i - 1, j + 1, map);
                path.extend(search.0);
                found = found && search.1;
            } else {
                found = false;
            }
        } else {
            if map[i - 1][j] == ']' {
                let search = search_up(i - 1, j, map);
                path.extend(search.0);
                found = found && search.1;
                let search = search_up(i - 1, j - 1, map);
                path.extend(search.0);
                found = found && search.1;
            } else if map[i - 1][j] == '[' {
                let search = search_up(i - 1, j, map);
                path.extend(search.0);
                found = found && search.1;
                let search = search_up(i - 1, j + 1, map);
                path.extend(search.0);
                found = found && search.1;
            } else {
                found = false;
            }
        }

        let mut unique_path = Vec::new();
        for coordinates in path.iter() {
            if !unique_path.contains(coordinates) {
                unique_path.push(*coordinates);
            }
        }
        return (unique_path, found);
    }

    fn search_down(i: usize, j: usize, map: &Vec<Vec<char>>) -> (Vec<(usize, usize)>, bool) {
        let mut path = vec![(j, i)];
        let mut found = true;
        if map[i + 1][j] == '.' {
            return (path, found);
        }
        if map[i][j] == '[' {
            if map[i + 1][j] == '[' {
                let search = search_down(i + 1, j, map);
                path.extend(search.0);
                found = found && search.1;
            } else if map[i + 1][j] == ']' {
                let search = search_down(i + 1, j, map);
                path.extend(search.0);
                found = found && search.1;
                let search = search_down(i + 1, j - 1, map);
                path.extend(search.0);
                found = found && search.1;
            } else {
                found = false;
            }
        } else if map[i][j] == ']' {
            if map[i + 1][j] == ']' {
                let search = search_down(i + 1, j, map);
                path.extend(search.0);
                found = found && search.1;
            } else if map[i + 1][j] == '[' {
                let search = search_down(i + 1, j, map);
                path.extend(search.0);
                found = found && search.1;
                let search = search_down(i + 1, j + 1, map);
                path.extend(search.0);
                found = found && search.1;
            } else {
                found = false;
            }
        } else {
            if map[i + 1][j] == ']' {
                let search = search_down(i + 1, j, map);
                path.extend(search.0);
                found = found && search.1;
                let search = search_down(i + 1, j - 1, map);
                path.extend(search.0);
                found = found && search.1;
            } else if map[i + 1][j] == '[' {
                let search = search_down(i + 1, j, map);
                path.extend(search.0);
                found = found && search.1;
                let search = search_down(i + 1, j + 1, map);
                path.extend(search.0);
                found = found && search.1;
            } else {
                found = false;
            }
        }

        let mut unique_path = Vec::new();
        for coordinates in path.iter() {
            if !unique_path.contains(coordinates) {
                unique_path.push(*coordinates);
            }
        }
        return (unique_path, found);
    }

    let mut rob_pos = (0, 0); //x, y position of robot
    for i in 0..map.len() {
        let mut j = 0;
        while j < map[i].len() {
            let char = map[i][j];
            if char == '@' {
                map[i].insert(j + 1, '.');
                rob_pos = (j, i);
            } else if char == 'O' {
                map[i][j] = '[';
                map[i].insert(j + 1, ']');
            } else {
                map[i].insert(j + 1, char);
            }
            j += 2;
        }
    }


    for m in moves.iter() {
        let (mut x, y) = rob_pos;
        if *m == '<' {
            x -= 1;
            while map[y][x] == ']' || map[y][x] == '[' {
                x -= 1;
            }
            if map[y][x] == '.' {
                while map[y][x] != '@' {
                    if map[y][x] == '[' {
                        map[y][x] = ']'
                    } else if map[y][x] == ']' {
                        map[y][x] = '['
                    } else {
                        map[y][x] = '['
                    }
                    x += 1;
                }
                map[rob_pos.1][rob_pos.0] = '.';
                rob_pos.0 -= 1;
                map[rob_pos.1][rob_pos.0] = '@';
            }
        } else if *m == '>' {
            x += 1;
            while map[y][x] == ']' || map[y][x] == '[' {
                x += 1;
            }
            if map[y][x] == '.' {
                while map[y][x] != '@' {
                    if map[y][x] == '[' {
                        map[y][x] = ']'
                    } else if map[y][x] == ']' {
                        map[y][x] = '['
                    } else {
                        map[y][x] = ']'
                    }
                    x -= 1;
                }
                map[rob_pos.1][rob_pos.0] = '.';
                rob_pos.0 += 1;
                map[rob_pos.1][rob_pos.0] = '@';
            }
        } else if *m == '^' {
            let mut search = search_up(y, x, &map);
            if search.1 {
                search.0.sort_by_key(|&(_, y)| y);
                for (x, y) in search.0.iter() {
                    map[y - 1][*x] = map[*y][*x];
                    if !search.0.contains(&(*x, y + 1)) {
                        map[*y][*x] = '.';
                    }
                }
                map[rob_pos.1][rob_pos.0] = '.';
                rob_pos.1 -= 1;
            }
        } else if *m == 'v' {
            let mut search = search_down(y, x, &map);
            if search.1 {
                search.0.sort_by_key(|&(_, y)| y);
                search.0.reverse();
                for (x, y) in search.0.iter() {
                    map[y + 1][*x] = map[*y][*x];
                    if !search.0.contains(&(*x, y - 1)) {
                        map[*y][*x] = '.';
                    }
                }
                map[rob_pos.1][rob_pos.0] = '.';
                rob_pos.1 += 1;
            }
        }
    }

    let mut gps_sum = 0;

    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '[' {
                gps_sum += i * 100 + j;
            }
        }
    }
    return gps_sum as i32;
}
