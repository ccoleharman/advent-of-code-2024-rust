fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input, 101, 103);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String, width: i32, height: i32) -> i32 {
    let input: Vec<String> = input.lines().map(|s| s.to_string()).collect::<Vec<_>>();
    let mut robots: Vec<Vec<i32>> = Vec::new();
    for group in input {
        let mut i = 0;
        let mut robot: Vec<i32> = Vec::new();
        let mut num: String = String::new();

        while i < group.len() {
            if group.chars().nth(i).unwrap().is_digit(10) || group.chars().nth(i).unwrap() == '-' {
                num.push(group.chars().nth(i).unwrap());
            } else {
                if num.len() != 0 {
                    if num.chars().nth(0).unwrap() == '-' {
                        num = num.trim_start_matches('-').to_string();
                        robot.push(num.parse::<i32>().unwrap() * -1);
                    } else {robot.push(num.parse::<i32>().unwrap())}
                    num = String::new();
                }
            }

            i += 1;
        }
        if num.chars().nth(0).unwrap() == '-' {
            num = num.trim_start_matches('-').to_string();
            robot.push(num.parse::<i32>().unwrap() * -1);
        } else {robot.push(num.parse::<i32>().unwrap())}

        robots.push(robot.clone());
    }

    let mut quad: Vec<i32> = vec![0; 4];

    for robot in robots {
        let mut x = (robot[0] + (robot[2] * 100)) % width;
        let mut y = (robot[1] + (robot[3] * 100)) % height;
        if x < 0 {x = width + x}
        if y < 0 {y = height + y}

        if x < (width as i32 - 1) / 2 {
            if y < (height as i32 - 1) / 2 {
                quad[0] += 1;
            } else if y > (height as i32 - 1) / 2 {
                quad[2] += 1;
            }
        } else if x > (width as i32 - 1) / 2 {
            if y < (height as i32 - 1) / 2 {
                quad[1] += 1;
            } else if y > (height as i32 - 1) / 2 {
                quad[3] += 1;
            }
        }
    }

    return quad[0] * quad[1] * quad[2] * quad[3];
}