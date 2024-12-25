//Part2 is personalized to individual input
fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: u64 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}

fn part2(input: String) -> u64 {
    let input: Vec<&str> = input.split("\r\n\r\n").collect();

    let mut program: Vec<u32> = input[1]
        .lines()
        .collect::<Vec<_>>()[0]
        .split(": ")
        .collect::<Vec<&str>>()[1]
        .split(",")
        .map(|num| num.parse().unwrap())
        .collect();
    
    program.reverse();
    let mut a: u64 = 0;
    let mut tracker = true;
    let mut j = 0;
    while j < program.len() {
        println!("{a}");
        if tracker {a *= 8;}
        for i in 0..8 {
            let (mut b, c);
            b = (a + i) % 8;
            b ^= 2;
            c = (a + i) >> b;
            b ^= c;
            b ^= 3;
            if (b % 8) == program[j] as u64 && tracker{
                a += i;
                break;
            } else if (b % 8) == program[j] as u64 {
                tracker = true;
            }
            if i == 7 {
                a /= 8;
                tracker = false;
                j -= 2;
            }
        }
        j += 1;
    }

    return a;
}
