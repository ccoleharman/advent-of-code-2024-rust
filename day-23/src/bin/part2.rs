use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: String = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> String {
    let input: Vec<Vec<&str>> = input.lines().map(|l| l.split("-").collect()).collect();
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();
    for pair in input {
        let key = pair[0];
        let value = pair[1];
        graph.entry(key).or_insert(HashSet::new()).insert(value);
        graph.entry(value).or_insert(HashSet::new()).insert(key);
    }

    fn search(node: &str, set: HashSet<&str>, graph: &HashMap<&str, HashSet<&str>>, rooms: &mut HashSet<Vec<String>>) {
        let mut check: Vec<String> = set.iter().map(|&s| s.to_string()).collect();
        check.sort();
        if !rooms.insert(check) {
            return;
        }
        for neighbor in graph.get(node).unwrap() {
            if set.contains(neighbor) || !set.iter().all(|x| graph[x].contains(neighbor)) {
                continue;
            }
            let mut new_set = set.clone();
            new_set.insert(neighbor);
            search(neighbor, new_set, graph, rooms);
        }
    }

    let mut rooms: HashSet<Vec<String>> = HashSet::new();
    for node in graph.keys() {
        let mut set = HashSet::new();
        set.insert(*node);
        search(node, set, &graph, &mut rooms);
    }

    let mut max = Vec::new();
    for room in rooms.iter() {
        if room.len() > max.len() {
            max = room.iter().map(|s| s.as_str()).collect();
        }
    }
    max.sort();
    return max.join(",");
}