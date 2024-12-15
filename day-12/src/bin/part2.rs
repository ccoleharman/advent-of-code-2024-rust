use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src//input.txt").expect("Error reading input"); //read in input
    let answer: i32 = part2(input);
    println!("#############################\n\nAnswer: {answer}\n\n#############################");
}
fn part2(input: String) -> i32 {
    fn search(i: usize, j: usize, input: Vec<Vec<char>>, tracked: &mut HashMap<Vec<usize>, bool>) -> (i32, Vec<Vec<(usize, usize, usize)>>) {
        //dir is the direction that it is coming from 0 - left 1 - up 2 - right 3 - down
        let mut area = 0;
        let mut sides: Vec<Vec<(usize, usize, usize)>> = vec![vec![], vec![]];

        if j > 0 && input[i][j - 1] == input[i][j] && !tracked.get(&vec![i, j - 1]).expect("Nothing") {
            tracked.insert(vec![i, j - 1], true);
            let search = search(i, j - 1, input.clone(), tracked);
            area += search.0 + 1;

            for x in 0..=1 {
                for side in search.1[x].iter() {
                    if !sides[x].contains(&side) {
                        sides[x].push(side.clone());
                    }
                }
            }

        } else if j == 0 || input[i][j - 1] != input[i][j] {
            let mut x = i as i32 - 1;
            while x >= 0 && input[x as usize][j] == input[i][j] && (j == 0 || input[x as usize][j - 1] != input[i][j]) {
                x -= 1;
            }
            let min = (x + 1) as usize;
            let mut x = i + 1;
            while x <= input.len() - 1 && input[x][j] == input[i][j] && (j == 0 || input[x][j - 1]!= input[i][j]) {
                x += 1;
            }
            let max = x - 1;
            let side = (j, min, max);
            if !sides[1].contains(&side) {
                sides[1].push(side.clone());
            }
        }

        if i > 0 && input[i - 1][j] == input[i][j] && !tracked.get(&vec![i - 1, j]).expect("Nothing") {
            tracked.insert(vec![i - 1, j], true);
            let search = search(i - 1, j, input.clone(), tracked);
            area += search.0 + 1;

            for x in 0..=1 {
                for side in search.1[x].iter() {
                    if !sides[x].contains(&side) {
                        sides[x].push(side.clone());
                    }
                }
            }

        } else if i == 0 || input[i - 1][j] != input[i][j] {
            let mut x = j as i32 - 1;
            while x >= 0 && input[i][x as usize] == input[i][j] && (i == 0 || input[i - 1][x as usize] != input[i][j]) {
                x -= 1;
            }
            let min = (x + 1) as usize;
            let mut x = j + 1;
            while x <= input[i].len() - 1 && input[i][x] == input[i][j] && (i == 0 || input[i - 1][x]!= input[i][j]) {
                x += 1;
            }
            let max = x - 1;
            let side = (i, min, max);
            if !sides[0].contains(&side) {
                sides[0].push(side.clone());
            }
        }

        if j < input[i].len() - 1 && input[i][j + 1] == input[i][j] && !tracked.get(&vec![i, j + 1]).expect("Nothing") {
            tracked.insert(vec![i, j + 1], true);
            let search = search(i, j + 1, input.clone(), tracked);
            area += search.0 + 1;

            for x in 0..=1 {
                for side in search.1[x].iter() {
                    if !sides[x].contains(&side) {
                        sides[x].push(side.clone());
                    }
                }
            }

        } else if j == input[i].len() - 1 {
            let mut x = i as i32 - 1;
            while x >= 0 && input[x as usize][j] == input[i][j] {
                x -= 1;
            }
            let min = (x + 1) as usize;
            let mut x = i + 1;
            while x <= input.len() - 1 && input[x][j] == input[i][j] {
                x += 1;
            }
            let max = x - 1;
            let side = (j + 1, min, max);
            if !sides[1].contains(&side) {
                sides[1].push(side.clone());
            }
        } else if input[i][j + 1] != input[i][j] {
            let mut x = i as i32 - 1;
            while x >= 0 && input[x as usize][j] == input[i][j] && input[x as usize][j + 1] != input[i][j] {
                x -= 1;
            }
            let min = (x + 1) as usize;
            let mut x = i + 1;
            while x <= input.len() - 1 && input[x][j] == input[i][j] && input[x][j + 1]!= input[i][j] {
                x += 1;
            }
            let max = x - 1;
            let side = (j + 1, min, max);
            if !sides[1].contains(&side) {
                sides[1].push(side.clone());
            }
        }

        if i < input.len() - 1 && input[i + 1][j] == input[i][j] && !tracked.get(&vec![i + 1, j]).expect("Nothing") {
            tracked.insert(vec![i + 1, j], true);
            let search = search(i + 1, j, input.clone(), tracked);
            area += search.0 + 1;

            for x in 0..=1 {
                for side in search.1[x].iter() {
                    if !sides[x].contains(&side) {
                        sides[x].push(side.clone());
                    }
                }
            }

        } else if i == input.len() - 1 {
            let mut x = j as i32 - 1;
            while x >= 0 && input[i][x as usize] == input[i][j] {
                x -= 1;
            }
            let min = (x + 1) as usize;
            let mut x = j + 1;
            while x <= input[i].len() - 1 && input[i][x] == input[i][j] {
                x += 1;
            }
            let max = x - 1;
            let side = (i + 1, min, max);
            if !sides[0].contains(&side) {
                sides[0].push(side.clone());
            }
        } else if input[i + 1][j] != input[i][j] {
            let mut x = j as i32 - 1;
            while x >= 0 && input[i][x as usize] == input[i][j] && input[i + 1][x as usize] != input[i][j] {
                x -= 1;
            }
            let min = (x + 1) as usize;
            let mut x = j + 1;
            while x <= input[i].len() - 1 && input[i][x] == input[i][j] && input[i + 1][x]!= input[i][j] {
                x += 1;
            }
            let max = x - 1;
            let side = (i + 1, min, max);
            if !sides[0].contains(&side) {
                sides[0].push(side.clone());
            }
        }

        return (area, sides);
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
                price += (search.0 + 1) * (search.1[0].len() + search.1[1].len()) as i32;
            }
        }
    }

    return price;
}