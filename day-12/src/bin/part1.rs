use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part1(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}
fn part1(input: String) -> i32 {
    fn search(i: usize, j: usize, input: Vec<Vec<char>>, tracked: &mut HashMap<Vec<usize>, bool>) -> (i32, i32) {
        //dir is the direction that it is coming from 0 - left 1 - up 2 - right 3 - down
        let mut area = 0;
        let mut perimeter = 0;

        if j > 0 && input[i][j - 1] == input[i][j] && !tracked.get(&vec![i, j - 1]).expect("Nothing") {
            tracked.insert(vec![i, j - 1], true);
            let search = search(i, j - 1, input.clone(), tracked);
            area += search.0 + 1;
            perimeter += search.1;
        } else if j == 0 || input[i][j - 1] != input[i][j] {
            perimeter += 1;
        }

        if i > 0 && input[i - 1][j] == input[i][j] && !tracked.get(&vec![i - 1, j]).expect("Nothing") {
            tracked.insert(vec![i - 1, j], true);
            let search = search(i - 1, j, input.clone(), tracked);
            area += search.0 + 1;
            perimeter += search.1;
        } else if i == 0 || input[i - 1][j] != input[i][j] {
            perimeter += 1;
        }

        if j < input[i].len() - 1 && input[i][j + 1] == input[i][j] && !tracked.get(&vec![i, j + 1]).expect("Nothing") {
            tracked.insert(vec![i, j + 1], true);
            let search = search(i, j + 1, input.clone(), tracked);
            area += search.0 + 1;
            perimeter += search.1;
        } else if j == input[i].len() - 1 || input[i][j + 1] != input[i][j] {
            perimeter += 1;
        }

        if i < input.len() - 1 && input[i + 1][j] == input[i][j] && !tracked.get(&vec![i + 1, j]).expect("Nothing") {
            tracked.insert(vec![i + 1, j], true);
            let search = search(i + 1, j, input.clone(), tracked);
            area += search.0 + 1;
            perimeter += search.1;
        } else if i == input.len() - 1 || input[i + 1][j] != input[i][j] {
            perimeter += 1;
        }

        return (area, perimeter);
    }

    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut tracked: HashMap<Vec<usize>, bool> = HashMap::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            tracked.insert(vec![i, j], false);
        }
    }


    let mut price: i32 = 0;

    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if !tracked.get(&vec![i, j]).expect("Nothing") {
                tracked.insert(vec![i, j], true);
                let search = search(i, j, input.clone(), &mut tracked);
                price += (search.0 + 1) * search.1;
            }
        }
    }

    return price;
}