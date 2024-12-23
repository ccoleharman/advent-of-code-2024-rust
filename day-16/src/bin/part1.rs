use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect();

    let mut start_pos = (0, 0); //x, y position of start
    'find_start: for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == 'S' {
                start_pos = (j, i);
                break 'find_start;
            }
        }
    }

    let mut heap: BinaryHeap<Reverse<(i32, usize, usize, i32, i32)>> = BinaryHeap::new();
    heap.push(Reverse((0, start_pos.1, start_pos.0, 0, 1))); //push the start position to the heap
    let mut visited: HashSet<(usize, usize, i32, i32)> = HashSet::new(); //visited positions
    visited.insert((start_pos.1, start_pos.0, 0, 1));

    while heap.len() > 0 {
        let Reverse((cost, r, c, dr, dc)) = heap.pop().unwrap();
        visited.insert((r, c, dr, dc));
        if input[r][c] == 'E' {
            return cost;
        }

        for new in [(cost + 1, (r as i32 + dr) as usize, (c as i32 + dc) as usize, dr, dc), (cost + 1000, r, c, -dc, dr), (cost + 1000, r, c, dc, -dr)].iter() {
            if input[new.1][new.2] != '#' && !visited.contains(&(new.1, new.2, new.3, new.4)) {
                heap.push(Reverse(*new));
            }
        }
    }
    
    -1 //no path found
}
