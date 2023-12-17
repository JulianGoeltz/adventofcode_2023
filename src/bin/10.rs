use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    // part 1
    //*
    let file_path = "inputs/10_dummy1.txt";
    let tmp = doit(file_path);
    println!("half ringlength is {tmp}");
    assert_eq!(tmp, 4); // */
    //*
    let file_path = "inputs/10_dummy2.txt";
    let tmp = doit(file_path);
    println!("half ringlength is {tmp}");
    assert_eq!(tmp, 4); // */
    let file_path = "inputs/10_dummy3.txt";
    let tmp = doit(file_path);
    println!("half ringlength is {tmp}");
    assert_eq!(tmp, 8); // */
    let file_path = "inputs/10_input.txt";
    let tmp = doit(file_path);
    println!("half ringlength is {tmp}");
    assert_eq!(tmp, 6640); // */
}

fn doit(file_path: &str) -> u64 {
    let contents = fs::read_to_string(file_path).expect("Can't read file");
    // collect data
    let fields: Vec<Vec<char>> = contents
        .lines()
        .map(|str| str.chars().map(|s| s).collect())
        .collect();
    for line in fields.iter() {
        println!("{:?}", line);
    }
    let start_position = locate_start(&fields);

    // for each surrounding start field, collect the ring
    let rings = collect_rings(&fields, start_position);

    let mut max_length: usize = 0;
    for ring in rings.iter() {
        if ring.len() > max_length {
            max_length = ring.len();
        }
    }
    let tmp = (max_length / 2).try_into().unwrap();
    println!("max_length is {} and half is {}", max_length, tmp);
    tmp
}

fn locate_start(fields: &Vec<Vec<char>>) -> (isize, isize) {
    let mut start_x: isize = -1;
    let mut start_y: isize = -1;
    for iy in 0..fields.len() {
        for ix in 0..fields[0].len() {
            if fields[iy][ix] == 'S' {
                println!("{ix} {iy}");
                start_x = ix.try_into().unwrap();
                start_y = iy.try_into().unwrap();
            }
        }
    }
    assert_ne!(start_x, -1);
    assert_ne!(start_y, -1);
    (start_x, start_y)
}

#[derive(Debug, PartialEq, Eq, Hash, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
    Nothing,
}

static GET_DIRECTION: Lazy<HashMap<Direction, (isize, isize)>> = Lazy::new(|| {
    HashMap::from([
        (Direction::North, (0, -1)),
        (Direction::East, (1, 0)),
        (Direction::South, (0, 1)),
        (Direction::West, (-1, 0)),
        (Direction::Nothing, (0, 0)),
    ])
});

static PIPE_I: Lazy<HashMap<Direction, Direction>> = Lazy::new(|| {
    HashMap::from([
        (Direction::North, Direction::North),
        (Direction::South, Direction::South),
    ])
});
static PIPE__: Lazy<HashMap<Direction, Direction>> = Lazy::new(|| {
    HashMap::from([
        (Direction::West, Direction::West),
        (Direction::East, Direction::East),
    ])
});
static PIPE_L: Lazy<HashMap<Direction, Direction>> = Lazy::new(|| {
    HashMap::from([
        (Direction::South, Direction::East),
        (Direction::West, Direction::North),
    ])
});
static PIPE_J: Lazy<HashMap<Direction, Direction>> = Lazy::new(|| {
    HashMap::from([
        (Direction::South, Direction::West),
        (Direction::East, Direction::North),
    ])
});
static PIPE_7: Lazy<HashMap<Direction, Direction>> = Lazy::new(|| {
    HashMap::from([
        (Direction::North, Direction::West),
        (Direction::East, Direction::South),
    ])
});
static PIPE_F: Lazy<HashMap<Direction, Direction>> = Lazy::new(|| {
    HashMap::from([
        (Direction::North, Direction::East),
        (Direction::West, Direction::South),
    ])
});
static PIPE_N: Lazy<HashMap<Direction, Direction>> = Lazy::new(|| HashMap::from([]));

fn collect_rings(
    fields: &Vec<Vec<char>>,
    start_position: (isize, isize),
) -> Vec<Vec<(isize, isize)>> {
    // let start_position: (isize, isize) = (
    //     start_position.0.try_into().unwrap(),
    //     start_position.1.try_into().unwrap()
    //     );
    let mut lists_of_elementpositions: Vec<Vec<(isize, isize)>> = Vec::new();
    println!("start_position {:?}", start_position);
    for dir in Direction::iter() {
        if dir != Direction::Nothing {
            let mut positions = vec![start_position];
            let mut last_direction = &dir;
            println!("{} {}: {}", start_position.0, start_position.1, 
                     fields[
                            usize::try_from(start_position.1).unwrap()
                        ][
                            usize::try_from(start_position.0).unwrap()
                        ],
                     );
            println!(
                "{:?}: {}, {}",
                dir, GET_DIRECTION[&dir].0, GET_DIRECTION[&dir].1
            );
            let mut keep_going = true;
            while keep_going {
                keep_going = false;

                let mut tmp_next_position: (isize, isize) =
                    *positions.last().expect("Should not be empty");
                tmp_next_position.0 += GET_DIRECTION[&last_direction].0;
                tmp_next_position.1 += GET_DIRECTION[&last_direction].1;

                if tmp_next_position.0 >= 0
                    && tmp_next_position.0 < fields[0].len().try_into().unwrap()
                    && tmp_next_position.1 >= 0
                    && tmp_next_position.1 < fields.len().try_into().unwrap()
                {
                    let mut next_position: (usize, usize) = (
                        tmp_next_position.0.try_into().unwrap(),
                        tmp_next_position.1.try_into().unwrap(),
                    );
                    let next_field = fields[next_position.1][next_position.0];
                    if next_field == 'S' {
                        println!("YES, found a ring:");
                        println!("{:?}", positions);
                        lists_of_elementpositions.push(positions);
                        break;
                    }
                    positions.push(tmp_next_position);
                    let next_pipe = &match &next_field {
                        '|' => &PIPE_I,
                        '-' => &PIPE__,
                        'J' => &PIPE_J,
                        'L' => &PIPE_L,
                        'F' => &PIPE_F,
                        '7' => &PIPE_7,
                        _ => &PIPE_N,
                    };
                    let next_direction =
                        next_pipe.get(last_direction).unwrap_or(&Direction::Nothing);
                    println!(
                        "with lastdir {:?} and next field {:?} we get next_direction {:?}",
                        last_direction, next_field, next_direction
                    );
                    if next_direction == &Direction::Nothing {
                        break;
                    }
                    last_direction = &next_direction;
                    keep_going = true;

                    println!(
                        "next field is at {} {} and '{}'",
                        next_position.0, next_position.1, next_field
                    );
                    println!("next_direction {:?}", next_direction)
                }
                // panic!();
            }
        }

        /*
            if !(ix == 0 && iy == 0) &&
                start_position.0 +ix  >= 0 &&
                start_position.1 - iy >= 0 {
                println!("{ix} {iy} {} {}", start_position.0 + ix,start_position.1 + iy);
                let mut keep_going = true;
                while keep_going {

                }
            }
        }
        // */
    }

    lists_of_elementpositions
}
