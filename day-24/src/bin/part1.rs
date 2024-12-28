use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: u64 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> u64 {
    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    let xy: Vec<&str> = input[0].lines().collect();
    let mut map: HashMap<&str, bool> = HashMap::new();
    for line in xy.iter() {
        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let (k, v) = (parts[0], parts[1]);
            match v.parse().unwrap() {
                0 => {
                    map.insert(k, false);
                }
                1 => {
                    map.insert(k, true);
                }
                _ => {
                    panic!("{}", v);
                }
            }
        }
    }

    let mut check: Vec<Vec<&str>> = input[1].lines().map(|l| l.split(" ").collect()).collect();
    let mut i = 0;
    while !map.contains_key(check[i][4]) {
        if !map.contains_key(check[i][0]) || !map.contains_key(check[i][2]) {
            i += 1;
            i %= check.len();
            continue;
        }
        let (a, b) = (map[check[i][0]], map[check[i][2]]);
        if check[i][1] == "AND" {
            map.insert(check[i][4], a && b);
        } else if check[i][1] == "OR" {
            map.insert(check[i][4], a || b);
        } else if check[i][1] == "XOR" {
            map.insert(check[i][4], a ^ b);
        }
        check.remove(i);
        if check.len() == 0 {
            break;
        }
        i %= check.len();
    }
    
    let mut count = 0;
    for (k, v) in map.iter() {
        if k.starts_with("z") && *v {
            count += 2_u64.pow(k.get(1..).unwrap().parse::<u32>().unwrap());
        }
    }
    return count;
}