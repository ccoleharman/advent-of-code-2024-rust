use std::collections::{BinaryHeap, HashSet};
use std::cmp::Reverse;

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    let input: Vec<Vec<usize>> = input.lines().map(|line| line.split(",").map(|s| s.parse::<usize>().unwrap()).collect()).collect();
    let bytes: Vec<Vec<usize>> = input[0..1024].to_vec();
    let mut map: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];
    for byte in bytes {
        map[byte[1]][byte[0]] = '#';
    }

    let mut heap: BinaryHeap<Reverse<(i32, usize, usize, i32, i32)>> = BinaryHeap::new();
    heap.push(Reverse((0, 0, 0, 0, 1))); //push the start position to the heap
    let mut visited: HashSet<(usize, usize, i32, i32)> = HashSet::new(); //visited positions
    visited.insert((0, 0, 0, 1));

    while heap.len() > 0 {
        let Reverse((steps, r, c, dr, dc)) = heap.pop().unwrap();
        visited.insert((r, c, dr, dc));
        if (r, c) == (map.len() - 1, map[0].len() - 1) {
            return steps;
        }
        for new in [(steps + 1, (r as i32 + dr) as usize, (c as i32 + dc) as usize, dr, dc), (steps, r, c, -dc, dr), (steps, r, c, dc, -dr)].iter() {
            if !(new.1 < map.len() && new.2 < map[0].len() && map[new.1][new.2] != '#') {
                continue;
            }
            if !visited.contains(&(new.1, new.2, new.3, new.4)) {
                heap.push(Reverse(*new));
            }
        }
    }
    
    -1 //no path found
}