fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}
fn part1(input: String) -> i32 {
    let input: Vec<&str> = input.split("\r\n").collect();
    let mut input: Vec<Vec<&str>> = input.iter().map(|x| x.split("").filter(|&c| !c.is_empty()).collect()).collect();

    let next = |pos: (i32, i32), input: &mut Vec<Vec<&str>>| -> (i32, i32) {
        match input[pos.0 as usize][pos.1 as usize] {
            "^" => (pos.0 - 1, pos.1),
            "v" => (pos.0 + 1, pos.1),
            "<" => (pos.0, pos.1 - 1),
            ">" => (pos.0, pos.1 + 1),
            _ => panic!("Invalid direction: {:?}", pos),
        }
    };


    //find the position of the start
    let mut position: (i32, i32) = (0, 0);
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == "^" {
                position = (i as i32, j as i32);
            }
        }
    }

    //start moving until out of bounds
    let mut next_pos: (i32, i32) = next(position, &mut input);
    while next_pos.0 >= 0 && next_pos.0 < input.len() as i32 && next_pos.1 >= 0 && next_pos.1 < input[1].len() as i32{
        if input[next_pos.0 as usize][next_pos.1 as usize] == "#" {
            input[position.0 as usize][position.1 as usize] = match input[position.0 as usize][position.1 as usize] {
                "^" => ">",
                "v" => "<",
                "<" => "^",
                ">" => "v",
                _ => panic!("Invalid direction: {:?}", position),
            };
            next_pos = next(position, &mut input);
        }
        else {
            input[next_pos.0 as usize][next_pos.1 as usize] = input[position.0 as usize][position.1 as usize];
            input[position.0 as usize][position.1 as usize] = "X";
            position = next_pos;
            next_pos = next(position, &mut input);
        }
    }
    input[position.0 as usize][position.1 as usize] = "X";

    //final output
    let mut count = 0; //returns the number of index positions the guard has been
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == "X" {
                count += 1;
            }
        }
    }
    return count;
}
