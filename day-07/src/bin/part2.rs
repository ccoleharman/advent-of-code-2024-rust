fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i64 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> i64 {
    fn decimal_to_ternary(mut n: u32) -> String {
        if n == 0 {
            return "0".to_string();
        }
        
        let mut ternary = String::new();
        while n > 0 {
            let remainder = n % 3;
            ternary.push(std::char::from_digit(remainder, 10).unwrap());
            n /= 3;
        }
        // Reverse the string because the digits are collected in reverse order
        ternary.chars().rev().collect()
    }


    let lines: Vec<&str> = input.lines().collect();

    let mut sum: i64 = 0; //sum of all of the lines that are correct
    //go through each line an test
    for line in lines.iter() {
        let line: Vec<&str> = line.split(": ").collect();
        let test: i64 = line[0].parse::<i64>().unwrap();
        let numbers: Vec<&str> = line[1].split_whitespace().collect();

        for i in 0..3_usize.pow(numbers.len() as u32 - 1) { //use binary to make sure all possible combinations are used
            let mut test_operators: i64 = numbers[0].parse().unwrap();
            let tern = decimal_to_ternary(i as u32);
            let mut tern: Vec<char> = tern.chars().collect();
            tern.reverse();

            for _j in tern.len()..16 { //make sure 16 bits are represented
                tern.push('0');
            }

            for j in 0..numbers.len() - 1 {
                if tern[j] == '1' {
                    test_operators += numbers[j + 1].parse::<i64>().unwrap()
                } else if tern[j] == '2'{
                    test_operators *= numbers[j + 1].parse::<i64>().unwrap()
                } else {
                    test_operators = (test_operators.to_string() + numbers[j + 1]).parse::<i64>().unwrap()
                }
            }
            if test == test_operators {sum += test; break;}
        }
    }
    return sum;
}