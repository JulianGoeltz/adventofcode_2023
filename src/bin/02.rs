use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "inputs/02_dummy.txt";
    let vector_games = create_game_vec(file_path);

    println!();
    println!();
    let tmp = check_condition(
        vector_games.clone(),
        HashMap::from([(0, 12), (1, 13), (2, 14)]),
    );
    println!();
    println!();
    println!("The indices of the valid games sum to {tmp}");

    let tmp = check_condition2(vector_games);
    println!("The sum of the power of the minimums is {tmp}");

    //*
    let file_path = "inputs/02_first.txt";
    let vector_games = create_game_vec(file_path);

    println!();
    println!();
    let tmp = check_condition(
        vector_games.clone(),
        HashMap::from([(0, 12), (1, 13), (2, 14)]),
    );
    println!();
    println!();
    println!("The indices of the valid games sum to {tmp}");
    let tmp = check_condition2(vector_games);
    println!("The sum of the power of the minimums is {tmp}");
    // */
}

fn check_condition2(vector_games: HashMap<i32, Vec<Vec<u64>>>) -> u64 {
    let mut sum: u64 = 0;
    for (_gameindex, subsets) in vector_games.iter() {
        let mut minimum: Vec<u64> = vec![0, 0, 0];
        for subset in subsets {
            for (num_sub, num_min) in subset.iter().zip(minimum.iter_mut()) {
                if *num_sub > *num_min {
                    *num_min = *num_sub;
                }
            }
        }
        // let tmp = minimum.iter().product::<u64>();
        // println!("game {_gameindex} has minima r{}, g{}, b{}; and power {tmp}",
        //          minimum[0], minimum[1], minimum[2]);
        sum += minimum.iter().product::<u64>();
    }
    sum
}

fn check_condition(
    vector_games: HashMap<i32, Vec<Vec<u64>>>,
    comparison: HashMap<usize, u64>,
) -> i32 {
    let mut sum: i32 = 0;
    for (gameindex, subsets) in &vector_games {
        let mut correct = true;
        for subset in subsets {
            for (idx, num) in comparison.iter() {
                if subset[*idx] > *num {
                    correct = false;
                }
            }
        }
        if correct {
            sum += gameindex;
            println!("Game {gameindex} is valid");
        }
    }
    sum
}

fn create_game_vec(file_path: &str) -> HashMap<i32, Vec<Vec<u64>>> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // let mut vector_games:Vec<Vec<Vec<u64>>> = Vec::new();
    let mut vector_games = HashMap::new();
    for line in contents.lines() {
        // println!("{}", line);
        let mut line_rest = line;
        let mut gameindex: i32 = -1;
        // find game index
        if let Some(index) = line_rest.find(':') {
            gameindex = line_rest[5..index]
                .parse::<i32>()
                .expect("No conversion possible");
            line_rest = &line_rest[index + 2..];
            // println!("found at {index} {gameindex};; {line_rest}");
        }
        // find subsets

        vector_games.insert(gameindex, identify_subsets(line_rest));
    }

    vector_games
}

fn parse_subset(line: &str) -> Vec<u64> {
    // println!("trying to parse '{line}'");
    let mut subset: Vec<u64> = vec![0, 0, 0];

    let colours: HashMap<&str, usize> = HashMap::from([("red", 0), ("green", 1), ("blue", 2)]);

    for (c_name, c_id) in &colours {
        for substring in line.split(", ") {
            if let Some(index) = substring.find(c_name) {
                // let tmp: Vec<&str> = line[0..index - 1].matches(char::is_numeric).collect();
                let tmp = &substring[0..index - 1];
                // println!("{tmp}");
                subset[*c_id] = tmp.parse::<u64>().expect("No conversion possible");
                // let tmp = subset[*c_id];
                // println!("colour {c_name} found at {index} with value {tmp}");
            }
        }
    }
    subset
}

fn identify_subsets(line: &str) -> Vec<Vec<u64>> {
    let mut vector_subsets: Vec<Vec<u64>> = Vec::new();
    let mut keepgoing = true;
    let mut line_rest = line.to_string();
    while keepgoing {
        keepgoing = false;
        if let Some(index) = line_rest.find(';') {
            vector_subsets.push(parse_subset(&line_rest[0..index]));
            line_rest = line_rest[index + 2..].to_string();
            // println!("found at {index} {gameindex};; {line_rest}");
            keepgoing = true;
        }
    }
    // also parse the last subset
    vector_subsets.push(parse_subset(&line_rest));
    vector_subsets
}
