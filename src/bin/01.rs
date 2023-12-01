use std::fs;

fn main() {
    /*************************
     * Dummy
     *************************/
    let file_path = "inputs/01_dummy.txt";
    let tmp = 
        sum_vector(
            select_digits_to_create_numbers(
                get_digits(file_path)));
    println!("Dummy: The sum of the numbers is {tmp}");

    /*************************
     * First Input
     *************************/
    let file_path = "inputs/01_first.txt";
    let tmp = 
        sum_vector(
            select_digits_to_create_numbers(
                get_digits(file_path)));
    println!("First Input: The sum of the numbers is {tmp}");
}

fn get_digits(file_path: &str) -> Vec<Vec<u64>> {
    /*************************
     * Go through lines and create linewise list of numbers
     *************************/
    // Read file contents
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut vector_of_numerical_lines:Vec<Vec<u64>> = Vec::new();
    for line in contents.lines() {
        let mut digits: Vec<u64> = Vec::new();
        for char in line.chars() {
            if char.is_numeric() {
                digits.push(
                    char.to_string().parse::<u64>().expect("no conversion possible"));
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
    let mut vector_of_numbers:Vec<u64> = Vec::new();
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
    let mut sum:u64 = 0;
    for number in &vector_of_numbers {
        sum += number;
    }

    return sum;
}
