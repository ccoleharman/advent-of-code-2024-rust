fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: u32 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> u32 {

    fn check_report(mut report: Vec<i32>, level: usize) -> bool {//checks if the report without the level is a valid report
        report.remove(level);

        //check if not already sorted or reverse sorted
        if !(report.windows(2).all(|pair| pair[0] < pair[1]) || report.windows(2).all(|pair| pair[0] > pair[1])){
            return false;
        }

        //check if the numbers are within the "safety tolerance"
        if report.windows(2).all(|pair| pair[0].abs_diff(pair[1]) <= 3) {
            return true;
        }
        return false;
    }



    let input: Vec<&str> = input.split("\r\n").collect();

    let mut count: u32 = 0;
    for report in input.iter() {
        //get into integer lists
        let list: Vec<&str> = report.split_whitespace().collect();
        let mut report_int: Vec<i32> = Vec::new();
        report_int.extend(list.iter().map(|num| num.parse::<i32>().expect("Invalid number format")));

        //check report
        //check if the report is increasing or decreasing
        
        for i in 0..report_int.len() {
            if check_report(report_int.clone(), i) {
                count += 1;
                break;
            }
        }
    }

    return count;
}