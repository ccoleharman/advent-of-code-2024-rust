use grid::*;

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

    let mut grid: Grid<char> = Grid::new(height as usize, width as usize);
    let mut robot_pos: Vec<(usize, usize)> = Vec::new();
    for robot in robots.iter() {
        robot_pos.push((robot[0] as usize, robot[1] as usize));
        grid[(robot[1] as usize, robot[0] as usize)] = '#';
    }
    for row in 0..grid.rows() {
        for col in 0..grid.cols() {
            if grid[(row, col)] != '#' {
                grid[(row, col)] = '.';
            }
        }
    }

    let mut seconds = 0;
    loop {
        seconds += 1;
        for i in 0..robots.len() {
            let (x, y) = robot_pos[i];
            grid[(y, x)] = '.';
            let mut x = x as i32;
            let mut y = y as i32;
            x = (x + robots[i][2]) % width;
            y = (y + robots[i][3]) % height;
            if x < 0 {x = width + x}
            if y < 0 {y = height + y}

            grid[(y as usize, x as usize)] = '#';
            robot_pos[i] = (x as usize, y as usize);
        }

        for row in 0..grid.rows() {
            let mut consecutive = 0;
            let mut i = 0;
            while i < grid.cols() && consecutive <= 25 {
                if grid[(row, i)] == '#' {
                    consecutive += 1;
                } else {consecutive = 0;}
                i += 1;
            }
            if consecutive > 25 {
                return seconds;
            }
        }
    }
}