use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i64 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> i64 {
    let input: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();

    let mut combos: HashSet<Vec<i64>> = HashSet::new();
    for i in -18..=18 {
        for j in -18..=18 {
            for k in -18..=18 {
                for l in -18..=18 {
                    combos.insert(vec![i, j, k, l]);
                }
            }
        }
    }

    let mut ds: Vec<Vec<i64>> = Vec::new();
    for secret in input.iter() {
        let mut d: Vec<i64> = Vec::new();
        let mut s = *secret;
        let mut last_dig = s % 10;
        d.push(last_dig);
        for _ in 0..2000 {
            s ^= s * 64;
            s %= 16777216;
            s ^= s >> 5;
            s %= 16777216;
            s ^= s * 2048;
            s %= 16777216;
            last_dig = s % 10;
            d.push(last_dig);
        }
        ds.push(d);
    }
    let mut diff = Vec::new();
    for d in ds.iter() {
        let mut differences = vec![20];
        for i in 1..d.len() {
            differences.push(d[i] - d[i - 1]);
        }
        diff.push(differences);
    }

    let mut max: HashMap<Vec<i64>, i64> = HashMap::new();
    for j in 0..diff.len() {
        let mut visited: HashSet<Vec<i64>, _> = HashSet::new();
        for i in 1..diff[j].len() - 4 {
            if combos.contains(&diff[j][i..i + 4].to_vec()) && !visited.contains(&diff[j][i..i + 4].to_vec()) {
                visited.insert(diff[j][i..i + 4].to_vec());
                if max.contains_key(&diff[j][i..i + 4].to_vec()) {
                    let count = max.get_mut(&diff[j][i..i + 4].to_vec()).unwrap();
                    *count += ds[j][i + 3];
                } else {
                    max.insert(diff[j][i..i + 4].to_vec(), ds[j][i + 3]);
                }
            }
        }
    }
    return max.values().max().unwrap().clone();
}