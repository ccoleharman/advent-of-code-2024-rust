fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i64 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    let mut sum: i64 = 0; //sum of all of the lines that are correct
    //go through each line an test
    for line in lines.iter() {
        let line: Vec<&str> = line.split(": ").collect();
        let test: i64 = line[0].parse::<i64>().unwrap();
        let numbers: Vec<&str> = line[1].split_whitespace().collect();
        let numbers: Vec<i64> = numbers.iter().map(|num| num.parse::<i64>().unwrap()).collect();

        for i in 0..2_usize.pow(numbers.len() as u32 - 1) { //use binary to make sure all possible combinations are used
            let mut test_operators: i64 = numbers[0];
            let bin = format!("{:16b}", i);
            let mut bin: Vec<char> = bin.chars().collect();
            bin.reverse();

            for j in 0..numbers.len() - 1 {
                if bin[j] == '1' {
                    test_operators += numbers[j + 1]
                } else {
                    test_operators *= numbers[j + 1]
                }
            }
            if test == test_operators {sum += test; break;}
        }
    }
    return sum;
}