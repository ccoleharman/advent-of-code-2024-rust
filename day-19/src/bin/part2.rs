use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i64 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> i64 {
    let input: Vec<&str> = input.split("\r\n\r\n").collect();
    let mut patterns: Vec<String> = input[0].split(", ").map(|s| s.to_string()).collect();
    patterns.sort_by(|a, b| (a.len()).cmp(&b.len()).reverse());
    let tests: Vec<&str> = input[1].lines().collect();

    fn find(test: &str, patterns: &Vec<String>, cache: &mut HashMap<String, i64>) -> i64 {
        let mut count: i64 = 0;
        if test.len() == 0 {
            return 1;
        }
        if cache.contains_key(test) {
            return *cache.get(test).unwrap();
        }
        for pattern in patterns.iter() {
            if test.starts_with(pattern) {
                count += find(&test[pattern.len()..], patterns, cache);
            }
        }
        cache.insert(test.to_string(), count);
        return count;
    }
    let mut count: i64 = 0;
    let mut cache: HashMap<String, i64> = HashMap::new();
    for test in tests.iter() {
        count += find(test, &patterns, &mut cache);
    }

    return count;
}