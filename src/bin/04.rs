use std::fs;

fn main() {
    let file_path = "inputs/04_dummy.txt";
    let tmp = collect_points(file_path);
    println!("Sum of points is {tmp}");
    assert_eq!(tmp, 13);

    let file_path = "inputs/04_input.txt";
    let tmp = collect_points(file_path);
    println!("Sum of points is {tmp}");
    assert_eq!(tmp, 21959);

    println!("============== Counting cards");
    let file_path = "inputs/04_dummy.txt";
    let tmp = count_cards(file_path);
    println!("total number of cards is {tmp}");
    assert_eq!(tmp, 30);

    let file_path = "inputs/04_input.txt";
    let tmp = count_cards(file_path);
    println!("total number of cards is {tmp}");
    assert_eq!(tmp, 5132675);
}

fn count_cards(file_path: &str) -> u64 {
    let contents = fs::read_to_string(file_path).expect("Can't read file");
    let mut number_of_cards: u64 = 0;
    let mut multiplier: Vec<u64> = Vec::new();
    for (_i_card, line) in contents.lines().enumerate() {
        let mut counting_wins: Vec<bool> = Vec::new();
        let game_stats = line.split(':').collect::<Vec<&str>>()[1];
        let tmp = game_stats.split('|').collect::<Vec<&str>>();
        let string_winning = tmp[0];
        let string_having = tmp[1];
        let numbers_winning = string_winning
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.to_string()
                    .parse::<u64>()
                    .expect("no conversion possible")
            });
        let numbers_having: Vec<u64> = string_having
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.to_string()
                    .parse::<u64>()
                    .expect("no conversion possible")
            })
            .collect();
        for nw in numbers_winning {
            if numbers_having.contains(&nw) {
                counting_wins.push(true);
            }
        }
        let wins: u64 = counting_wins.len().try_into().unwrap();
        let copies: u64 = (1 + multiplier.len()).try_into().unwrap();
        number_of_cards += copies;
        // println!("before cleaning");
        // println!("{}", multiplier.iter().map(|x| {x.to_string()}).collect::<Vec<String>>().join(" "));
        multiplier = multiplier
            .iter()
            .filter(|x| x >= &&2)
            .map(|x| x - 1)
            .collect();
        // println!("after cleaning");
        // println!("{}", multiplier.iter().map(|x| {x.to_string()}).collect::<Vec<String>>().join(" "));
        if wins > 0 {
            for _ in 0..copies {
                multiplier.push(wins)
            }
        }

        // println!("card {} has {copies} copies and {wins} wins; current number_of_cards {number_of_cards}", _i_card + 1);
    }

    number_of_cards
}

fn collect_points(file_path: &str) -> u64 {
    let contents = fs::read_to_string(file_path).expect("Can't read file");
    let mut sum: u64 = 0;
    for line in contents.lines() {
        let mut counting_wins: Vec<bool> = Vec::new();
        // println!("line is {line}");
        let game_stats = line.split(':').collect::<Vec<&str>>()[1];
        // println!("game_stats is {game_stats}");
        let tmp = game_stats.split('|').collect::<Vec<&str>>();
        let string_winning = tmp[0];
        let string_having = tmp[1];
        // println!("winning is '{string_winning}', having is '{string_having}'");
        let numbers_winning = string_winning
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.to_string()
                    .parse::<u64>()
                    .expect("no conversion possible")
            });
        let numbers_having: Vec<u64> = string_having
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| {
                s.to_string()
                    .parse::<u64>()
                    .expect("no conversion possible")
            })
            .collect();
        for nw in numbers_winning {
            // println!("{nw}");
            if numbers_having.contains(&nw) {
                counting_wins.push(true);
            }
        }
        let factor: u64 = 2;
        if !counting_wins.is_empty() {
            // println!("with {} wins, we add to the sum: ", counting_wins.len());
            let summand: u64 = factor.pow((counting_wins.len() - 1).try_into().unwrap());
            // println!("{}", summand);
            sum += summand;
        }
    }
    sum
}
