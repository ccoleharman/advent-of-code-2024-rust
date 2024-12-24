use std::collections::{BinaryHeap, HashSet, HashMap, VecDeque};
use std::cmp::Reverse;

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> i32 {
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
    let mut low: HashMap<(usize, usize, i32, i32), i32> = HashMap::new(); //low positions
    low.insert((start_pos.1, start_pos.0, 0, 1), 0);
    let mut back: HashMap<(usize, usize, i32, i32), HashSet<(usize, usize, i32, i32)>> = HashMap::new();
    let mut best = std::i32::MAX;
    let mut end_states: HashSet<(usize, usize, i32, i32)> = HashSet::new();

    while heap.len() > 0 {
        let Reverse((cost, r, c, dr, dc)) = heap.pop().unwrap();
        if low.contains_key(&(r, c, dr, dc)) && low[&(r, c, dr, dc)] < cost {
            continue;
        }
        if input[r][c] == 'E' {
            if cost > best {
                break;
            }
            best = cost;
            end_states.insert((r, c, dr, dc));
        }
        for new in [(cost + 1, (r as i32 + dr) as usize, (c as i32 + dc) as usize, dr, dc), (cost + 1000, r, c, -dc, dr), (cost + 1000, r, c, dc, -dr)].iter() {
            let low_1 = low.get(&(new.1, new.2, new.3, new.4)).unwrap_or(&std::i32::MAX);
            if input[new.1][new.2] == '#' || new.0 > *low_1 {
                continue;
            }
            if new.0 < *low_1 {
                back.insert((new.1, new.2, new.3, new.4), HashSet::new());
                low.insert((new.1, new.2, new.3, new.4), new.0);
            }
            back.get_mut(&(new.1, new.2, new.3, new.4)).unwrap().insert((r, c, dr, dc));
            heap.push(Reverse(*new));
        }
    }
    
    let mut deq: VecDeque<(usize, usize, i32, i32)> = VecDeque::from(end_states.into_iter().collect::<Vec<_>>());
    let mut visited: HashSet<(usize, usize, i32, i32)> = HashSet::new();

    while deq.len() > 0 {
        let key = deq.pop_front().unwrap();
        for last in back.get(&key).unwrap_or(&HashSet::new()).iter() {
            if visited.contains(last) {
                continue;
            }
            visited.insert(*last);
            deq.push_back(*last);
        }
    }

    let mut pos: HashSet<(usize, usize)> = HashSet::new();
    for (i, j, _, _) in visited.iter() {
        pos.insert((*i, *j));
    }
    
    return pos.len() as i32 + 1;
}
