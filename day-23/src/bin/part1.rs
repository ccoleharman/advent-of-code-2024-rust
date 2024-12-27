use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i32 {
    let input: Vec<Vec<&str>> = input.lines().map(|l| l.split("-").collect()).collect();
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for pair in input {
        let key = pair[0];
        let value = pair[1];
        graph.entry(key).or_insert(HashSet::new()).insert(value);
        graph.entry(value).or_insert(HashSet::new()).insert(key);
    }

    let mut count = 0;
    let mut visited_t: HashSet<&str> = HashSet::new();
    for (s, h) in graph.iter() {
        if s.starts_with("t") {
            for i in h.iter() {
                if visited_t.contains(i) {
                    continue;
                }
                for j in h.iter() {
                    if visited_t.contains(j) {
                        continue;
                    }
                    if graph[*i].contains(j) {
                        count += 1;
                    }
                }
            }
            visited_t.insert(s);
        }
    }

    return count / 2;
}