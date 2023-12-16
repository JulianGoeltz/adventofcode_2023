use std::fs;

fn main() {
    // part 1
    let file_path = "inputs/09_dummy.txt";
    let tmp = doit(file_path);
    println!("Sum of values is {tmp}");
    assert_eq!(tmp, 114);

    let file_path = "inputs/09_input.txt";
    let tmp = doit(file_path);
    println!("Sum of values is {tmp}");
    assert_eq!(tmp, 1877825184);

    // part 2
    let file_path = "inputs/09_dummy.txt";
    let tmp = doit2(file_path);
    println!("Sum of values is {tmp}");
    assert_eq!(tmp, 2);

    let file_path = "inputs/09_input.txt";
    let tmp = doit2(file_path);
    println!("Sum of values is {tmp}");
    assert_eq!(tmp, 1108);

}


fn doit2(file_path: &str) -> i32 {
    // get file input
    let list_of_inputs = process_file(file_path);
    // calculate differences till all 0
    let list_of_list_of_differences = differentiate(list_of_inputs);
    // predict
    let list_of_predictions = predict_backwards(list_of_list_of_differences);
    // collect sum
    list_of_predictions
        .iter()
        .map(|list| list
             .first()
             .expect("no items"))
        .sum::<i32>()
}

fn doit(file_path: &str) -> i32 {
    // get file input
    let list_of_inputs = process_file(file_path);
    // calculate differences till all 0
    let list_of_list_of_differences = differentiate(list_of_inputs);
    // predict
    let list_of_predictions = predict(list_of_list_of_differences);
    // collect sum
    list_of_predictions
        .iter()
        .map(|list| list
             .last()
             .expect("no items"))
        .sum::<i32>()
}

fn process_file(file_path: &str) -> Vec<Vec<i32>> {
    let contents = fs::read_to_string(file_path).expect("Can't read file");
    let mut list_of_inputs: Vec<Vec<i32>> = Vec::new();
    for line in contents.lines() {
        // println!("Processing line '{line}'");
        list_of_inputs.push(
            line
            .split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| {
                // println!("{s}");
                s.to_string()
                    .parse::<i32>()
                    .expect("no conversion possible")
            })
            .collect::<Vec<i32>>()
        );
    }

    list_of_inputs
}

fn differentiate(list_of_inputs: Vec<Vec<i32>>) -> Vec<Vec<Vec<i32>>> {
    let mut list_of_list_of_differences: Vec<Vec<Vec<i32>>> = Vec::new();
    for inpts in list_of_inputs.iter() {
        let mut differences: Vec<Vec<i32>> = Vec::new();
        differences.push(inpts.to_vec());
        let mut current_list = differences.last().expect("Should not be empty");
        // println!("{:?}", inpts);
        while current_list.iter().map(|n| n.abs()).sum::<i32>() > 0 {
            // println!("{:?}", current_list);
            let mut tmp_list: Vec<i32> = Vec::new();
            for i in 0..current_list.len() - 1 {
                if i < current_list.len() {
                    tmp_list.push(current_list[i + 1] - current_list[i]);
                }
            }
            differences.push(tmp_list);
            // current_list = &tmp_list;
            current_list = differences.last().expect("Should not be empty");
        }
        list_of_list_of_differences.push(differences);
    }
    /*
    for (i, differences) in list_of_list_of_differences.iter().enumerate() {
        println!("{}, {}: {:?}", i, differences.len(), differences);
    }
    // */
    list_of_list_of_differences
}

fn predict(list_of_list_of_differences: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>> {
    let mut list_of_predictions: Vec<Vec<i32>> = Vec::new();
    for differences in list_of_list_of_differences.iter() {
        let mut last_value:i32 = 0;
        for i in (0..differences.len() - 1).rev() {
            // print!("{}, {}; ", last_value, differences[i].last().expect("No last value"));
            last_value = last_value + differences[i].last().expect("No last value");

        }
        let mut tmp = differences[0].clone();
        tmp.push(last_value);
        list_of_predictions.push(tmp);
        // println!("{last_value}");
    }
    list_of_predictions
}

fn predict_backwards(list_of_list_of_differences: Vec<Vec<Vec<i32>>>) -> Vec<Vec<i32>> {
    let mut list_of_predictions: Vec<Vec<i32>> = Vec::new();
    for differences in list_of_list_of_differences.iter() {
        let mut last_value:i32 = 0;
        for i in (0..differences.len() - 1).rev() {
            // println!("{}, {}; ", last_value, differences[i].first().expect("No first value"));
            last_value = differences[i].first().expect("No first value") - last_value;

        }
        let mut tmp = differences[0].clone();
        tmp.insert(0, last_value);
        list_of_predictions.push(tmp);
        // println!("{last_value}");
    }
    list_of_predictions
}
