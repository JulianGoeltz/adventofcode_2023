use regex::Regex;
use std::fs;

fn main() {
    let file_path = "inputs/03_dummy.txt";
    println!("On file {file_path}");
    let tmp = do_the_thing(file_path);
    println!("Total number is {tmp}");
    assert_eq!(tmp, 4361);

    let file_path = "inputs/03_input.txt";
    println!("On file {file_path}");
    let tmp = do_the_thing(file_path);
    println!("Total number is {tmp}");
    assert_eq!(tmp, 559667);

    println!("======================== new rule: ratios");
    let file_path = "inputs/03_dummy.txt";
    println!("On file {file_path}");
    let tmp = do_the_other_thing(file_path);
    println!("Total number is {tmp}");
    assert_eq!(tmp, 467835);

    let file_path = "inputs/03_input.txt";
    println!("On file {file_path}");
    let tmp = do_the_other_thing(file_path);
    println!("Total number is {tmp}");
    assert_eq!(tmp, 86841457);

    println!();
}

fn get_shifted_boundaries(m_start: usize, m_end: usize, line_len: usize) -> (usize, usize) {
    let start: usize = match m_start {
        0 => 0,
        _ => m_start - 1,
    };
    let mut end: usize = m_end;
    if end >= line_len {
        end -= 1
    };
    (start, end)
}

fn do_the_other_thing(file_path: &str) -> u64 {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let vector_of_lines: Vec<String> = contents.lines().map(String::from).collect();
    let regex_number: Regex = Regex::new(r"[0-9]+").unwrap();

    let mut vector_of_ratios: Vec<u64> = Vec::new();

    for (i_line, line) in vector_of_lines.iter().enumerate() {
        for (m_idx, _m_str) in line.match_indices('*') {
            // println!("{}", line);
            // println!("{} {}", m_idx, m_str);
            let mut vector_of_numbers: Vec<u64> = Vec::new();
            // search above
            if i_line > 0 {
                // println!("prvs line {}", vector_of_lines[i_line - 1]);
                for m in regex_number.find_iter(&vector_of_lines[i_line - 1]) {
                    let (start, end) = get_shifted_boundaries(m.start(), m.end(), line.len());
                    // println!("index gear {}, start&end {} {}", m_idx, start, end);
                    if start <= m_idx && end >= m_idx {
                        vector_of_numbers
                            .push(m.as_str().parse::<u64>().expect("No conversion possible"));
                        // println!("found number {}", &vector_of_numbers.last().unwrap());
                    }
                }
            }
            // search below
            if i_line < vector_of_lines.len() - 1 {
                // println!("next line {}", vector_of_lines[i_line + 1]);
                for m in regex_number.find_iter(&vector_of_lines[i_line + 1]) {
                    let (start, end) = get_shifted_boundaries(m.start(), m.end(), line.len());
                    // println!("index gear {}, start&end {} {}", m_idx, start, end);
                    if start <= m_idx && end >= m_idx {
                        vector_of_numbers
                            .push(m.as_str().parse::<u64>().expect("No conversion possible"));
                        // println!("found number {}", &vector_of_numbers.last().unwrap());
                    }
                }
            }
            // search in same line
            {
                for m in regex_number.find_iter(line) {
                    let (start, end) = get_shifted_boundaries(m.start(), m.end(), line.len());
                    // println!("index gear {}, start&end {} {}", m_idx, start, end);
                    if start == m_idx || end == m_idx {
                        vector_of_numbers
                            .push(m.as_str().parse::<u64>().expect("No conversion possible"));
                        // println!("found number {}", &vector_of_numbers.last().unwrap());
                    }
                }
            }

            // see if it is exactly two numbers
            if vector_of_numbers.len() == 2 {
                vector_of_ratios.push(vector_of_numbers.iter().product::<u64>());
                // println!("exactly two numbers! added {}", &vector_of_ratios.last().unwrap());
            }
        }
    }

    vector_of_ratios.iter().sum::<u64>()
}

fn do_the_thing(file_path: &str) -> u64 {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let vector_of_lines: Vec<String> = contents.lines().map(String::from).collect();
    let regex_number: Regex = Regex::new(r"[0-9]+").unwrap();

    let mut vector_of_numbers: Vec<u64> = Vec::new();

    for (i_line, line) in vector_of_lines.iter().enumerate() {
        // println!("{line}");
        for m in regex_number.find_iter(line) {
            // println!("  {} {} {} {}", m.as_str(), m.start(), m.end(), m.len());
            let mut has_symbol_adjacent = false;
            // define extended sizes for diagonal check
            let (start, end) = get_shifted_boundaries(m.start(), m.end(), line.len());

            // check the adjacent chars
            if is_special_char(line.chars().nth(start).expect("index of start is wrong"))
                || is_special_char(line.chars().nth(end).expect("index of end is wrong"))
            {
                has_symbol_adjacent = true;
            }
            // check previous line if not the first line
            if i_line > 0 {
                // println!("previous line {}", vector_of_lines[i_line - 1]);
                if vector_of_lines[i_line - 1][start..end + 1].contains(is_special_char) {
                    has_symbol_adjacent = true;
                }
            }
            // check next line if not the last line
            if i_line < vector_of_lines.len() - 1 {
                // println!("previous line {}", vector_of_lines[i_line + 1]);
                if vector_of_lines[i_line + 1][start..end + 1].contains(is_special_char) {
                    has_symbol_adjacent = true;
                }
            }

            // add number to vector
            if has_symbol_adjacent {
                vector_of_numbers.push(m.as_str().parse::<u64>().expect("No conversion possible"));
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

    vector_of_numbers.iter().sum::<u64>()
}

fn is_special_char(c: char) -> bool {
    !char::is_numeric(c) && c != '.'
}
