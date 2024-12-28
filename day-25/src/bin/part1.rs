use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut blocks: Vec<Vec<Vec<char>>> = Vec::new();
    for block in input {
        let mut lines: Vec<Vec<char>> = Vec::new();
        for line in block.lines() {
            lines.push(line.chars().collect());
        }
        blocks.push(lines);
    }

    let mut hashtags: Vec<HashSet<(usize, usize)>> = Vec::new();
    for block in blocks {
        let mut hashset: HashSet<(usize, usize)> = HashSet::new();
        for (i, line) in block.iter().enumerate() {
            for (j, c) in line.iter().enumerate() {
                if *c == '#' {
                    hashset.insert((i, j));
                }
            }
        }
        hashtags.push(hashset);
    }
    
    let mut count = 0;
    for i in hashtags.iter() {
        for j in hashtags.iter() {
            let ideal = i.len() + j.len();
            let new: HashSet<&(usize, usize)> = i.union(j).collect();
            if i != j && ideal == new.len() {
                count += 1;
            }
        }
    }

    return count / 2;
}