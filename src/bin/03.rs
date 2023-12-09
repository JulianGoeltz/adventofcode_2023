use std::fs;
use regex::Regex;

fn main() {
    let file_path = "inputs/03_dummy.txt";
    println!("On file {file_path}");
    do_the_thing(file_path);

    let file_path = "inputs/03_input.txt";
    println!("On file {file_path}");
    do_the_thing(file_path);

}

fn do_the_thing(file_path: &str) {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!();

    // println!("number of chars is {}", contents.len());
    // println!("number of lines is {}", contents.lines().len());

    //
    // let mut vector_of_lines : Vec<Vec<char>> = vec![
    //     0; len];
    let vector_of_lines : Vec<String> = contents
        .lines()
        .map(String::from)
        .collect();

    let regex_number : Regex = Regex::new(r"[0-9]+").unwrap();

    let mut vector_of_numbers : Vec<u64> = Vec::new();

    for (i_line, line) in vector_of_lines.iter().enumerate() {
        // println!("{line}");
        for m in regex_number.find_iter(line) {
            // println!("  {} {} {} {}", m.as_str(), m.start(), m.end(), m.len());
            let mut has_symbol_adjacent = false;
            // define extended sizes for diagonal check
            let start : usize = match m.start() {
                0 => 0,
                _ => m.start() - 1,
            };
            let mut end : usize = m.end();
            if end >= line.len() {end -= 1};

            // check the adjacent chars
            if
                is_special_char(line
                                .chars()
                                .nth(start)
                                .expect("index of start is wrong")
                                )
                ||
                is_special_char(line
                                .chars()
                                .nth(end)
                                .expect("index of end is wrong")
                                )
            {
                    has_symbol_adjacent = true;
            }
            // check previous line if not the first line
            if i_line > 0 {
                 // println!("previous line {}", vector_of_lines[i_line - 1]);
                 if vector_of_lines[i_line - 1][start..end + 1]
                     .contains(is_special_char)
                 {
                     has_symbol_adjacent = true;
                 }
            }
            // check next line if not the last line
            if i_line < vector_of_lines.len() - 1 {
                 // println!("previous line {}", vector_of_lines[i_line + 1]);
                 if vector_of_lines[i_line + 1][start..end + 1]
                     .contains(is_special_char)
                 {
                     has_symbol_adjacent = true;
                 }
            }

            // add number to vector
            if has_symbol_adjacent {
                vector_of_numbers.push(
                    m.as_str().parse::<u64>().expect("No conversion possible")
                    );
            }

            // println!(
            //     "   {} {} {}",
            //     line[start..end + 1].to_string(),
            //     line.chars().nth(start).expect("index of start wrong"),
            //     line.chars().nth(end).expect("index of end wrong"),
            // );

            // println!("{}", line[start..end].to_string());
            // println!("{start}, {}, {end}", line.len());
            // let start : usize = m.start() - 1 else { return 0; };
        }
    }

    println!("Total number is {}",
             vector_of_numbers.iter().sum::<u64>()
    );
}

fn is_special_char(c: char) -> bool {
    !char::is_numeric(c) && c != '.'
}
