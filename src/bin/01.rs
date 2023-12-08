use std::fs;

fn main() {
    /*************************
     * Dummy
     *************************/
    let file_path = "inputs/01_dummy.txt";
    let tmp = sum_vector(select_digits_to_create_numbers(get_digits(file_path)));
    println!("Dummy: The sum of the numbers is {tmp}");

    /*************************
     * First Input
     *************************/
    let file_path = "inputs/01_first.txt";
    let tmp = sum_vector(select_digits_to_create_numbers(get_digits(file_path)));
    println!("First Input: The sum of the numbers is {tmp}");
    /*************************
     * Second dummy
     *************************/
    let file_path = "inputs/01_dummy2.txt";
    let tmp = sum_vector(select_digits_to_create_numbers(
        get_digits_also_alphabetical(file_path),
    ));
    println!("second dummy: The sum of the numbers is {tmp}");
    /*************************
     * Second part with old input
     *************************/
    let file_path = "inputs/01_first.txt";
    let tmp = sum_vector(select_digits_to_create_numbers(
        get_digits_also_alphabetical(file_path),
    ));
    println!("Input with other rules: The sum of the numbers is {tmp}");
}

fn get_digits_also_alphabetical(file_path: &str) -> Vec<Vec<u64>> {
    /*************************
     * Go through lines and create linewise list of numbers
     *************************/
    // Read file contents
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let vec_replace: Vec<String> = [
        // "0zero",
        "1one", "2two", "3three", "4four", "5five", "6six", "7seven", "8eight", "9nine",
    ]
    .iter()
    .map(|&s| s.into())
    .collect();

    let mut vector_of_numerical_lines: Vec<Vec<u64>> = Vec::new();
    for line in contents.lines() {
        let mut digits: Vec<u64> = Vec::new();
        let mut lines: Vec<String> = Vec::new();
        lines.push(line.to_string());
        let mut keepgoing = true;
        while keepgoing {
            keepgoing = false;
            let mut index_find = 1000;
            let mut index_number = 0;
            let mut i = 0;

            for replace in vec_replace.iter() {
                let tmp = &replace.clone();
                let _pattern = &tmp[1..].to_string();
                match lines[lines.len() - 1].find(_pattern) {
                    Some(index) => {
                        if index < index_find {
                            index_find = index;
                            index_number = i;
                            keepgoing = true;
                        }
                    }
                    None => (),
                }
                i += 1;
            }
            if keepgoing {
                // println!("Now replacing {} at place {index_find}", index_number + 1);
                let tmp = vec_replace[index_number].clone();
                let _replace = &tmp[0..1].to_string();
                let mut line_new = lines[lines.len() - 1].clone();
                line_new.replace_range(index_find..index_find + 1, _replace);
                lines.push(line_new);
            }
        }

        let line_new = lines[lines.len() - 1].clone();
        // println!("line {line} is now {line_new}");
        for char in line_new.chars() {
            if char.is_numeric() {
                digits.push(
                    char.to_string()
                        .parse::<u64>()
                        .expect("no conversion possible"),
                );
            }
        }
        vector_of_numerical_lines.push(digits);
    }
    return vector_of_numerical_lines;
}

fn get_digits(file_path: &str) -> Vec<Vec<u64>> {
    /*************************
     * Go through lines and create linewise list of numbers
     *************************/
    // Read file contents
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut vector_of_numerical_lines: Vec<Vec<u64>> = Vec::new();
    for line in contents.lines() {
        let mut digits: Vec<u64> = Vec::new();
        for char in line.chars() {
            if char.is_numeric() {
                digits.push(
                    char.to_string()
                        .parse::<u64>()
                        .expect("no conversion possible"),
                );
            }
        }
        vector_of_numerical_lines.push(digits);
    }
    return vector_of_numerical_lines;
}

fn select_digits_to_create_numbers(vector_of_numerical_lines: Vec<Vec<u64>>) -> Vec<u64> {
    /*************************
     * Select first and last per line
     *************************/
    let mut vector_of_numbers: Vec<u64> = Vec::new();
    for line_vec in &vector_of_numerical_lines {
        let tmp = 10 * line_vec[0] + line_vec[line_vec.len() - 1];
        vector_of_numbers.push(tmp);
    }
    return vector_of_numbers;
}

fn sum_vector(vector_of_numbers: Vec<u64>) -> u64 {
    /*************************
     * Sum up
     *************************/
    let mut sum: u64 = 0;
    for number in &vector_of_numbers {
        sum += number;
    }

    return sum;
}
