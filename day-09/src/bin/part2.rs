fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i64 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}
fn part2(input: String) -> i64 {
    //develop input into workable structure
    let input: Vec<i32> = input
        .chars()
        .map(|char: char| char.to_digit(10).unwrap() as i32)
        .collect::<Vec<i32>>();

    let mut blocks: Vec<i32> = Vec::new();
    let mut files: Vec<Vec<i32>> = Vec::new();
    let mut i: i32 = 0;
    for j in 0..input.len() {
        let mut file: Vec<i32> = Vec::new();
        if j % 2 != 0 {
            for _ in 0..input[j] {
                blocks.push(-1);
            }
            i += 1;
        } else {
            for _ in 0..input[j] {
                blocks.push(i);
                file.push(i);
            }
            files.push(file);
        }
    }
    
    files.reverse();
    
    //move files to free space
    while files.len() > 0 {
        let file = files.remove(0);
        
        let mut num = -1;
        let mut pos = 0;
        for i in 0..blocks.len() - file.len() {
            if blocks[i..i + file.len()].iter().all(|&x| x == -1) {
                for j in i..i + file.len() {
                    pos = j + 1;
                    blocks[j] = file[0];
                }
                num = file[0];
                break;
            } else if blocks[i] == file[0] {
                break;
            }
        }

        if num != -1 {
            for i in pos..blocks.len() {
                if blocks[i] == num {
                    blocks[i] = -1;
                }
            }
        }
    }

    //compute checksum
    let mut checksum: i64 = 0;
    for i in 0..blocks.len() {
        if blocks[i] != -1 {
            checksum += i as i64 * blocks[i] as i64;
        }
    }

    return checksum;
}
