fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}
fn part2(input: String) -> i32 {
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

    let start_input: Vec<Vec<&str>> = input.clone();
    let start_pos: (i32, i32) = position;



    let mut count = 0; //returns the number of places I can put the object
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            //reset the input
            input = start_input.clone();
            position = start_pos;

            if input[i][j] != "#" && (i as i32, j as i32) != position {
                let mut iter_counter = 0;
                let max_iterations = 4000; //4000 was chosen as the maximum because in my input there were less than 1000 '#' characters
                //only use 4000 if there are less than 1000 '#' characters in your input, otherwise multiply the amount by 4 and round up
                let mut breakout = false;

                //add object
                input[i][j] = "#";

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

                        iter_counter += 1;
                        if iter_counter >= max_iterations {
                            breakout = true;
                            break;
                        }
                    }
                    else {
                        input[next_pos.0 as usize][next_pos.1 as usize] = input[position.0 as usize][position.1 as usize];
                        input[position.0 as usize][position.1 as usize] = ".";
                        position = next_pos;
                        next_pos = next(position, &mut input);
                    }
                }
                if breakout {
                    count += 1;
                }
            }
        }
    }
    return count;
}
