use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i64 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

/*Part 2 significantly differs from part 1. Part 1 was small enough to run a while 
loop nested in a for loop. Part 2 is much larger, so I recursively went to each stone
and used a cache to not repeat the function more than needed.*/

fn part2(input: String) -> i64 {
    let stones: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();

    let cache: HashMap<(String, i32), i64> = HashMap::new();

    fn count(stone: String, blinks: i32, cache: &mut HashMap<(String, i32), i64>) -> i64 {
        if let Some(&result) = cache.get(&(stone.clone(), blinks)) {
            return result;
        }

        let result = if blinks == 0 {
            1
        } else if stone == "0" {
            count("1".to_string(), blinks - 1, cache)
        } else if stone.len() % 2 == 0 {
            let mid = stone.len() / 2;
            count(stone.get(0..mid).unwrap().parse::<i64>().unwrap().to_string(), blinks - 1, cache) + count(stone.get(mid..stone.len()).unwrap().parse::<i64>().unwrap().to_string(), blinks - 1, cache)
        } else {
            count((stone.parse::<i64>().unwrap() * 2024).to_string(), blinks - 1, cache)
        };

        cache.insert((stone.clone(), blinks), result);
        result
    }

    let count: i64 = stones.par_iter().map(|stone| {
        let mut cache_clone = cache.clone();
        count(stone.clone(), 75, &mut cache_clone)
    }).sum();
    return count;
}

