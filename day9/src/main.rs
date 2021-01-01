use std::fs;

fn can_get_sum(sum: &usize, slice: &[usize]) -> bool {
    for ii in slice {
        for jj in slice {
            if ii == jj {
                continue;
            }
            if ii + jj == *sum {
                // println!("{:?} + {} = {}", ii, jj, sum);
                return true;
            }
        }
    }
    false
}

fn part_one(filename: String, previous_entries: usize) {
    println!("Part 1 answer for {:?}", filename);
    let contents: Vec<usize> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    if contents.len() <= previous_entries {
        panic!(
            "Contents list is not longer than {} entries, with {} entries: {:#?}",
            previous_entries,
            contents.len(),
            contents
        );
    }

    for (ii, number) in contents.iter().enumerate() {
        if ii <= previous_entries {
            continue;
        }

        if !can_get_sum(number, &contents[(ii - previous_entries - 1)..(ii)]) {
            println!("Cannot get sum for {:?}", number);
            break;
        }
    }
}

fn part_two(filename: String, aim: usize) {
    println!("Part 1 answer for {:?}", filename);
    let contents: Vec<usize> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect();

    for (ii, _number) in contents.iter().enumerate() {
        for jj in 2..(contents.len() - ii) {
            let slice = &contents[ii..(ii + jj)];
            if slice.iter().sum::<usize>() == aim {
                println!("Start: {:?}", ii);
                println!("End: {:?}", ii + jj);
                println!("Smallest: {:?}", slice.iter().min().unwrap());
                println!("Largest: {:?}", slice.iter().max().unwrap());
                println!(
                    "Sum: {:?}",
                    slice.iter().min().unwrap() + slice.iter().max().unwrap()
                );
            }
        }
    }
}

fn main() {
    part_one("example_input1.txt".to_string(), 5);
    part_one("input1.txt".to_string(), 25);
    part_two("example_input1.txt".to_string(), 127);
    part_two("input1.txt".to_string(), 542529149);
}
