use std::fs;

fn main() {
    /*************************
     * Dummy
     *************************/
    let file_path = "inputs/01_dummy.txt";
    let tmp = countup(file_path);
    println!("Dummy: The sum of the numbers is {tmp}");

    /*************************
     * First Input
     *************************/
    let file_path = "inputs/01_first.txt";
    let tmp = countup(file_path);
    println!("First Input: The sum of the numbers is {tmp}");
}

fn countup(file_path: &str) -> u64 {
    println!("Read file {}", file_path);

    // Read file contents
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    /*************************
     * Go through lines and create linewise list of numbers
     *************************/
    let mut vector_of_numerical_lines = Vec::new();
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

    /*************************
     * Select first and last per line
     *************************/
    let mut vector_of_numbers:Vec<u64> = Vec::new();
    let mut i = 0;
    for line_vec in &vector_of_numerical_lines {
        if line_vec.len() == 0 {
            println!("Error in line {i}");
            // lol, all this hassle because I had an addditional \n in the input, i.e. empty last
            // line
            let mut j = 0;
            for line in contents.lines() {
                if j == i {
                    println!("Line: {}", line);
                }
                j += 1;
            }
        }
        let tmp = 10 * line_vec[0] + line_vec[line_vec.len() - 1];
        vector_of_numbers.push(tmp);
        i += 1;
    }

    /*************************
     * Sum up
     *************************/
    let mut sum:u64 = 0;
    for number in &vector_of_numbers {
        sum += number;
    }

    return sum;
}
