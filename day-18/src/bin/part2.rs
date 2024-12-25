use std::collections::{VecDeque, HashSet};

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> String {
    let input: Vec<&str> = input.lines().collect();
    let mut map: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];

    let mut path = true;
    let mut i = 0;
    while path {
        path = false;
        let byte: Vec<usize> = input[i].split(",").map(|b| b.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        map[byte[1]][byte[0]] = '#';
        let mut deq:VecDeque<(usize, usize)> =VecDeque::new();
        deq.push_back((0, 0)); //push to the end position of the deq
        let mut visited: HashSet<(usize, usize)> = HashSet::new(); //visited positions
        visited.insert((0, 0));

        while deq.len() > 0 {
            let (r, c) = deq.pop_front().unwrap();
            if (r, c) == (map.len() - 1, map[0].len() - 1) {
                path = true;
                break;
            }
            for (r, c) in [((r as i32 - 1) as usize, c), (r + 1, c), (r, (c as i32 - 1) as usize), (r, c + 1)].iter() {
                if *r < map.len() && *c < map[0].len() && map[*r][*c] == '.' && !visited.contains(&(*r, *c)) {
                    deq.push_back((*r, *c));
                    visited.insert((*r, *c));
                }
            }
        }
        i += 1;
    }
    return input[i - 1].to_string();
}