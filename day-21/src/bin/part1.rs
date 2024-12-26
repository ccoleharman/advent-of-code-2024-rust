use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use std::cmp::min;

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    fn length(s: Vec<char>, depth: i32, dir_len: &HashMap<(char, char), usize>, seqs: &HashMap<(char, char), Vec<Vec<char>>>, cache: &mut HashMap<(Vec<char>, i32), i32>) -> i32 {
        if cache.contains_key(&(s.clone(), depth)) {
            return *cache.get(&(s.clone(), depth)).unwrap();
        }
        let mut len = 0;
        if depth == 1 {
            let mut a_line = vec!['A'];
            a_line.extend(s.clone());
            for (j, i) in s.iter().zip(&a_line) {
                len += dir_len.get(&(*i, *j)).unwrap();
            }
        } else {
            let mut a_line = vec!['A'];
            a_line.extend(s.clone());
            for (j, i) in s.iter().zip(&a_line) {
                let mut m = std::i32::MAX;
                for sub in seqs.get(&(*i, *j)).unwrap().iter() {
                    m = min(m, length(sub.clone(), depth - 1, dir_len, seqs, cache));
                }
                len += m as usize;
            }
        }
        cache.insert((s.clone(), depth), len as i32);
        return len as i32;
    }

    let input: Vec<&str> = input.lines().collect();

    let num_pad: Vec<Vec<char>> = vec![
    vec!['7', '8', '9'],
    vec!['4', '5', '6'],
    vec!['1', '2', '3'],
    vec!['#', '0', 'A']
    ];
    let mut num_pos: HashMap<char, (usize, usize)> = HashMap::new();
    for r in 0..num_pad.len() {
        for c in 0..num_pad[0].len() {
            if num_pad[r][c] != '#' {num_pos.insert(num_pad[r][c], (r, c));}
        }
    }
    let mut num_seq: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    for i in num_pos.keys() {
        for j in num_pos.keys() {
            if i == j {
                num_seq.insert((*i, *j), vec![vec!['A']]);
                continue;
            }

            let mut seq: Vec<Vec<char>> = Vec::new();
            let mut deq: VecDeque<((usize, usize), Vec<char>)> = VecDeque::new();
            deq.push_front((*num_pos.get(i).unwrap(), vec![]));
            
            'big: while deq.len() > 0 {
                let ((r, c), m) = deq.pop_front().unwrap();
                for ((nr, nc), nm) in [(((r as i32 - 1) as usize, c), '^'), ((r, (c as i32 - 1) as usize), '<'), ((r + 1, c), 'v'), ((r, c + 1), '>')].iter() {
                    if !(*nr < num_pad.len() && *nc < num_pad[0].len() && num_pad[*nr][*nc] != '#') {
                        continue;
                    }
                    let mut new_m = m.clone();
                    if num_pad[*nr][*nc] == *j {
                        if !seq.is_empty() && m.len() + 2 > seq[0].len() {
                            break 'big;
                        }
                        new_m.extend(vec![*nm, 'A']);
                        seq.push(new_m);
                    } else {
                        new_m.push(*nm);
                        deq.push_back(((*nr, *nc), new_m));
                    }
                }
            }
            num_seq.insert((*i, *j), seq);
        }
    }
    let dir_pad = vec![
        vec!['#', '^', 'A'],
        vec!['<', 'v', '>'],

    ];
    let mut dir_pos: HashMap<char, (usize, usize)> = HashMap::new();
    for r in 0..dir_pad.len() {
        for c in 0..dir_pad[0].len() {
            if dir_pad[r][c] != '#' {dir_pos.insert(dir_pad[r][c], (r, c));}
        }
    }
    let mut dir_seq: HashMap<(char, char), Vec<Vec<char>>> = HashMap::new();
    for i in dir_pos.keys() {
        for j in dir_pos.keys() {
            if i == j {
                dir_seq.insert((*i, *j), vec![vec!['A']]);
                continue;
            }

            let mut seq: Vec<Vec<char>> = Vec::new();
            let mut deq: VecDeque<((usize, usize), Vec<char>)> = VecDeque::new();
            deq.push_front((*dir_pos.get(i).unwrap(), vec![]));
            
            'big: while deq.len() > 0 {
                let ((r, c), m) = deq.pop_front().unwrap();
                for ((nr, nc), nm) in [(((r as i32 - 1) as usize, c), '^'), ((r, (c as i32 - 1) as usize), '<'), ((r + 1, c), 'v'), ((r, c + 1), '>')].iter() {
                    if !(*nr < dir_pad.len() && *nc < dir_pad[0].len() && dir_pad[*nr][*nc] != '#') {
                        continue;
                    }
                    let mut new_m = m.clone();
                    if dir_pad[*nr][*nc] == *j {
                        if !seq.is_empty() && m.len() + 2 > seq[0].len() {
                            break 'big;
                        }
                        new_m.extend(vec![*nm, 'A']);
                        seq.push(new_m);
                    } else {
                        new_m.push(*nm);
                        deq.push_back(((*nr, *nc), new_m));
                    }
                }
            }
            dir_seq.insert((*i, *j), seq);
        }
    }
    let mut dir_len: HashMap<(char, char), usize> = HashMap::new();
    for (k, v) in dir_seq.iter() {
        dir_len.insert(*k, v[0].len());
    }

    let mut cache: HashMap<(Vec<char>, i32), i32> = HashMap::new();
    let mut complexity = 0;
    for line in input.iter() {
        let mut poss: Vec<Vec<Vec<char>>> = Vec::new();
        let line_chars: Vec<char> = line.chars().collect::<Vec<char>>();
        let mut a_line = vec!['A'];
        a_line.extend(line_chars.clone());
        for (j, i) in line_chars.iter().zip(&a_line) {
            poss.push(num_seq.get(&(*i, *j)).unwrap().clone());
        }
        let poss: Vec<Vec<&char>> = poss.iter().map(|v| v.iter()).multi_cartesian_product().map(|v| v.into_iter().flatten().collect()).collect::<Vec<Vec<&char>>>();
        let mut m = std::i32::MAX;
        for p in poss.iter() {
            m = min(m, length(p.iter().copied().map(|c| *c).collect(), 2, &dir_len, &dir_seq, &mut cache));
        }
        complexity += m * (line.get(0..line.len() - 1).unwrap().parse::<i32>().unwrap());
    }

    return complexity;
}