fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: u32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> u32 {
    let input: Vec<&str> = input.split("\r\n").collect();

    let mut count: u32 = 0;
    for report in input.iter() {
        //get into integer lists
        let list: Vec<&str> = report.split_whitespace().collect();
        let mut report_int: Vec<u32> = Vec::new();
        report_int.extend(list.iter().map(|num| num.parse::<u32>().expect("Invalid number format")));
        
        //check if not already sorted or reverse sorted
        if !(report_int.windows(2).all(|pair| pair[0] < pair[1]) || report_int.windows(2).all(|pair| pair[0] > pair[1])){
            continue;
        }

        //check if the numbers are within the "safety tolerance"
        if report_int.windows(2).all(|pair| pair[0].abs_diff(pair[1]) <= 3) {
            count += 1;
        }
    }

    return count;
}