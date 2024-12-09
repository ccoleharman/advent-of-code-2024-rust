fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i64 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}
fn part1(input: String) -> i64 {
    //develop input into workable structure
    let input: Vec<i32> = input
        .chars()
        .map(|char: char| char.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();

    let mut blocks: Vec<i32> = Vec::new();
    let mut i: i32 = 0;
    for j in 0..input.len() {
        if j % 2 != 0 {
            for _ in 0..input[j] {
                blocks.push(-1);
            }
            i += 1;
        } else {
            for _ in 0..input[j] {
                blocks.push(i);

            }
        }
    }
    
    //get rid of free space
    let mut i = 0;
    while i < blocks.len() {
        if blocks[i] == -1 {
            blocks[i] = blocks.pop().unwrap();
        } else {
            i += 1;
        }
    }

    //compute checksum
    let mut checksum: i64 = 0;
    for i in 0..blocks.len() {
        checksum += i as i64 * blocks[i] as i64;
    }

    return checksum;
}
