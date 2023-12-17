use std::fs;

fn main() {
    // part 1
    let file_path = "inputs/11_dummy.txt";
    let tmp = doit(file_path);
    assert_eq!(tmp, 374);
    // */
    let file_path = "inputs/11_input.txt";
    let tmp = doit(file_path);
    assert_eq!(tmp, 9550717);
    // */
}

fn doit(file_path: &str) -> u64 {
    // read file
    let contents = fs::read_to_string(file_path).expect("Can't read file");
    let observation: Vec<Vec<u64>> = contents
        .lines()
        .map(|str| {
            str.chars()
                .map(|c| match c {
                    '#' => 1,
                    _ => 0,
                })
                .collect()
        })
        .collect();
    // print(&observation);

    // do gravity expansion
    /*
    let expanded1 = gravity_expansion(observation);
    print(&expanded1);
    let expanded1T = transpose(expanded1);
    print(&expanded1T);
    let expanded2T = gravity_expansion(expanded1T);
    print(&expanded2T);
    let expanded = transpose(expanded2T);
    // */
    let expanded = transpose(gravity_expansion(transpose(gravity_expansion(observation))));
    // print(&expanded);

    // identify galaxies
    let pairs = identity_pairs(expanded);
    /*
    println!("{:?}", pairs);
    for (i1, i2) in vec![(4, 8), (0, 6), (2, 5), (7, 8)] {
        println!(
            "{:?}: {:?}, {:?}: {}",
            (i1 + 1, i2 + 1),
            pairs[i1],
            pairs[i2],
            distance(pairs[i1], pairs[i2]),
        );
    }
    // */

    // iterate through pairs and collect distances
    sum_distances(&pairs)
}

fn sum_distances(pairs: &[(usize, usize)]) -> u64 {
    let mut sum: u64 = 0;
    for (i, gal1) in pairs.iter().enumerate() {
        for (j, gal2) in pairs.iter().enumerate() {
            if j > i {
                // don't count twice
                if i != j {
                    /*
                    println!("Distance between {} and {} is {}",
                             i + 1,
                             j + 1,
                             distance(*gal1, *gal2));
                    // */
                    sum += distance(*gal1, *gal2);
                }
            }
        }
    }
    sum
}

fn distance(gal1: (usize, usize), gal2: (usize, usize)) -> u64 {
    ((<usize as TryInto<i32>>::try_into(gal1.0).unwrap()
        - <usize as TryInto<i32>>::try_into(gal2.0).unwrap())
    .abs()
        + (<usize as TryInto<i32>>::try_into(gal1.1).unwrap()
            - <usize as TryInto<i32>>::try_into(gal2.1).unwrap())
        .abs())
    .try_into()
    .unwrap()
}

fn identity_pairs(observation: Vec<Vec<u64>>) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for i in 0..observation.len() {
        for j in 0..observation[0].len() {
            if observation[i][j] == 1 {
                pairs.push((i, j));
            }
        }
    }
    pairs
}

fn transpose(observation: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut transposed: Vec<Vec<u64>> = Vec::new();
    for jj in 0..observation[0].len() {
        let mut tmp: Vec<u64> = Vec::new();
        for ii in 0..observation.len() {
            tmp.push(observation[ii][jj]);
        }
        transposed.push(tmp);
    }
    transposed
}

fn gravity_expansion(observation: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut expanded: Vec<Vec<u64>> = Vec::new();
    for line in observation.iter() {
        expanded.push(line.to_vec());
        if line.iter().sum::<u64>() == 0 {
            expanded.push(line.to_vec());
        }
    }
    expanded
}

fn print(v: &Vec<Vec<u64>>) {
    println!("{}x{}", v.len(), v[0].len());
    for line in v.iter() {
        println!("{:?}", line);
    }
    println!();
}
