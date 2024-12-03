fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}
fn part1(input: String) -> i32 {
    // Split the input string into a vector of substrings
    let input_parts: Vec<&str> = input.split("").collect();

    let mut sum: i32 = 0; // Initialize the sum variable
    let mut index: usize = 0; // Initialize the index variable

    // Loop through the input parts
    while index < input_parts.len() - 4 {
        // Check if the current part is a "mul(" substring
        if &input_parts[index..index + 4].join("") != "mul(" {
            index += 1; // Move to the next index if not a "mul(" substring
            continue;
        }

        index += 4; // Move to the inside of the parentheses

        // Find the index of the closing parenthesis
        let mut closing_parenthesis_index: usize = 0;
        for j in index..index + 8 {
            if j > input_parts.len() {
                break;
            }
            if &input_parts[j] == &")" {
                closing_parenthesis_index = j;
                break;
            }
        }

        // Skip the current iteration if there is no closing parenthesis
        if closing_parenthesis_index == 0 {
            index += 1;
            continue;
        }

        // Check if the numbers inside the parentheses are valid
        if input_parts[index..closing_parenthesis_index]
            .join("")
            .split(",")
            .map(|s| s.trim())
            .all(|c| c.chars().all(|d| d.is_digit(10)))
        {
            // Parse the numbers and calculate the sum
            let numbers: Vec<i32> = input_parts[index..closing_parenthesis_index]
                .join("")
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();

            // Skip the current iteration if there are not exactly two numbers
            if numbers.len() != 2 {
                continue;
            }

            sum += numbers[0] * numbers[1];
        }

        // Move to the next index
        index = closing_parenthesis_index + 1;
    }

    // Return the final sum
    return sum;
}