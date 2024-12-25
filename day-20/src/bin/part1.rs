fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    let map = input.lines().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut start_pos: (usize, usize) = (0, 0);
    'find_start: for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 'S' {
                start_pos = (i, j);
                break 'find_start;
            }
        }
    }

    let (mut r, mut c) = start_pos;
    let mut steps = vec![vec![-1; map[0].len()]; map.len()];
    steps[r][c] = 0;

    while map[r][c] != 'E' {
        for (nr, nc) in [((r as i32 - 1) as usize, c), (r + 1, c), (r, (c as i32 - 1) as usize), (r, c + 1)].iter() {
            if *nr < map.len() && *nc < map[0].len() && map[*nr][*nc] != '#' && steps[*nr][*nc] == -1 {
                steps[*nr][*nc] = steps[r][c] + 1;
                r = *nr;
                c = *nc;
            }
        }
    }

    let mut count= 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == '#' {
                continue;
            }
            for (nr, nc) in [(r + 2, c), ((r as i32 - 2) as usize, c), (r, c + 2), (r, (c as i32 - 2) as usize), (r + 1, c + 1), ((r as i32 - 1) as usize, c + 1), ((r as i32 - 1) as usize, (c as i32 - 1) as usize), (r + 1, (c as i32 - 1) as usize)].iter() {
                if *nr < map.len() && *nc < map[0].len() && map[*nr][*nc] != '#' {
                    if steps[*nr][*nc] - steps[r][c] >= 102 {
                        count += 1;
                    }
                }
            }
        }
    }

    return count;
}