use std::fs;

fn main() {
    // part 1
    let file_path = "inputs/11_dummy.txt";
    let _tmp = doit_old(file_path, 2);
    let tmp = doit(file_path, 2);
    assert_eq!(tmp, 374);
    //*
    let file_path = "inputs/11_input.txt";
    let tmp = doit(file_path, 2);
    assert_eq!(tmp, 9550717);

    // part 2
    let file_path = "inputs/11_dummy.txt";
    let tmp = doit(file_path, 10);
    assert_eq!(tmp, 1030);
    let file_path = "inputs/11_dummy.txt";
    let tmp = doit(file_path, 100);
    assert_eq!(tmp, 8410);
    let file_path = "inputs/11_input.txt";
    let tmp = doit(file_path, 1000000);
    assert_eq!(tmp, 648458253817);
    // */
}

fn doit(file_path: &str, expansion_factor: u64) -> u64 {
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
    // identify galaxies
    let mut pairs = identity_pairs(&observation);

    // do gravity expansion
    let expansions_rows = identify_expansions(&observation);
    let expansions_cols = identify_expansions(&transpose(observation));
    // println!("{:?}", expansions_rows);
    // println!("{:?}", expansions_cols);
    let expansion_factor: usize = expansion_factor.try_into().unwrap();
    for pair in pairs.iter_mut() {
        pair.0 += expansions_rows.iter().filter(|i| i < &&pair.0).count() * (expansion_factor - 1);
        pair.1 += expansions_cols.iter().filter(|i| i < &&pair.1).count() * (expansion_factor - 1);
    }

    /*
    println!("{:?}", pairs);
    for (i1, i2, d) in vec![(4, 8, 9), (0, 6, 15), (2, 5, 17), (7, 8, 5)] {
        println!(
            "{:?}: {:?}, {:?}: {}",
            (i1 + 1, i2 + 1),
            pairs[i1],
            pairs[i2],
            distance(pairs[i1], pairs[i2]),
        );
        assert_eq!(distance(pairs[i1], pairs[i2]), d);
    }
    // */

    // iterate through pairs and collect distances
    let tmp = sum_distances(&pairs);
    println!("Calculated {}", tmp);
    tmp
}

fn identify_expansions(observation: &Vec<Vec<u64>>) -> Vec<usize> {
    let mut expansions: Vec<usize> = Vec::new();
    for (i, line) in observation.iter().enumerate() {
        if line.iter().sum::<u64>() == 0 {
            expansions.push(i);
        }
    }
    expansions
}

fn doit_old(file_path: &str, expansion_factor: u64) -> u64 {
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
    print!("from size {}x{} ", observation.len(), observation[0].len());
    let tmp = gravity_expansion(observation, expansion_factor);
    print!("via size {}x{} ", tmp.len(), tmp[0].len());
    let expanded = transpose(gravity_expansion(transpose(tmp), expansion_factor));
    println!("to size {}x{}", expanded.len(), expanded[0].len());
    // print(&expanded);

    // identify galaxies
    let pairs = identity_pairs(&expanded);
    /*
    println!("{:?}", pairs);
    for (i1, i2, d) in vec![(4, 8, 9), (0, 6, 15), (2, 5, 17), (7, 8, 5)] {
        println!(
            "{:?}: {:?}, {:?}: {}",
            (i1 + 1, i2 + 1),
            pairs[i1],
            pairs[i2],
            distance(pairs[i1], pairs[i2]),
        );
        assert_eq!(distance(pairs[i1], pairs[i2]), d);
    }
    // */

    // iterate through pairs and collect distances
    let tmp = sum_distances(&pairs);
    println!("Calculated {}", tmp);
    tmp
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

fn identity_pairs(observation: &Vec<Vec<u64>>) -> Vec<(usize, usize)> {
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

fn gravity_expansion(observation: Vec<Vec<u64>>, expansion_factor: u64) -> Vec<Vec<u64>> {
    let mut expanded: Vec<Vec<u64>> = Vec::new();
    for line in observation.iter() {
        expanded.push(line.to_vec());
        if line.iter().sum::<u64>() == 0 {
            for _ in 0..expansion_factor - 1 {
                expanded.push(line.to_vec());
            }
        }
    }
    expanded
}
