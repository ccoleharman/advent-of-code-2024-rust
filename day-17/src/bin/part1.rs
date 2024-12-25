fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: String = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part1(input: String) -> String {
    let input: Vec<&str> = input.split("\r\n\r\n").collect();

    let registers: Vec<&str> = input[0].lines().collect();
    let mut reg_a: u64 = registers[0].split(": ").collect::<Vec<&str>>()[1].parse().unwrap();
    let mut reg_b: u64 = registers[1].split(": ").collect::<Vec<&str>>()[1].parse().unwrap();
    let mut reg_c: u64 = registers[2].split(": ").collect::<Vec<&str>>()[1].parse().unwrap();

    let program: Vec<u8> = input[1]
        .lines()
        .collect::<Vec<_>>()[0]
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    
    let mut out: Vec<String> = Vec::new();

    let mut i = 0;
    while i < program.len() {
        let opcode: u8 = program[i];
        let literal: u8 = program[i + 1];
        let combo: u64 = match literal {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => panic!("Invalid operand value")
        };

        if opcode == 0 {
            reg_a /= 2_u64.pow(combo as u32);
        } else if opcode == 1 {
            reg_b ^= literal as u64;
        } else if opcode == 2 {
            reg_b = combo % 8;
        } else if opcode == 3 && reg_a != 0 {
            i = literal as usize;
        } else if opcode == 4 {
            reg_b ^= reg_c;
        } else if opcode == 5 {
            out.push((combo % 8).to_string());
        } else if opcode == 6 {
            reg_b = reg_a / 2_u64.pow(combo as u32);
        } else if opcode == 7 {
            reg_c = reg_a / 2_u64.pow(combo as u32);
        } 
        
        i += 2;
        if opcode == 3 && reg_a != 0 {
            i -= 2;
        }
    }

    return out.join(",");
}
