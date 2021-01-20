use std::fs;

enum Occupation {
    Occupied,
    Empty,
    Floor,
    NonExistant,
}

fn is_occupied(adjacent_ii: usize, adjacent_jj: usize, collection: &[Vec<char>]) -> Occupation {
    if let Some(inner_vec) = collection.get(adjacent_ii) {
        match inner_vec.get(adjacent_jj) {
            Some('#') => Occupation::Occupied,
            Some('L') => Occupation::Empty,
            Some('.') => Occupation::Floor,
            None => Occupation::NonExistant,
            _ => unimplemented!(),
        }
    } else {
        Occupation::NonExistant
    }
}

#[derive(Copy, Clone)]
enum MaxDistance {
    One,
    Infinity,
}

fn get_occupied(
    ii: isize,
    jj: isize,
    adjacent_ii: &isize,
    adjacent_jj: &isize,
    collection: &[Vec<char>],
) -> usize {
    if adjacent_ii == &ii && adjacent_jj == &jj {
        // println!("    adjacent_ii {:?} adjacent_jj {} ==", adjacent_ii, adjacent_jj);
        return 0;
    }

    if adjacent_ii < &0 || adjacent_jj < &0 {
        // println!("    adjacent_ii {:?} adjacent_jj {} < 0", adjacent_ii, adjacent_jj);
        return 0;
    }

    match is_occupied(*adjacent_ii as usize, *adjacent_jj as usize, collection) {
        Occupation::Occupied => 1,
        _ => 0,
    }
}

fn occupied_adjacent(
    ii: isize,
    jj: isize,
    collection: &[Vec<char>],
    max_distance: MaxDistance,
) -> usize {
    let mut occupied_adjacent: usize = 0;

    match max_distance {
        MaxDistance::One => {
            for adjacent_ii in [ii - 1, ii, ii + 1].iter() {
                for adjacent_jj in [jj - 1, jj, jj + 1].iter() {
                    occupied_adjacent += get_occupied(ii, jj, adjacent_ii, adjacent_jj, collection);
                }
            }
        }
        MaxDistance::Infinity => {
            for (ii_increment, jj_increment) in vec![
                (0, 1),
                (1, 0),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, 0),
                (0, -1),
                (-1, -1),
            ]
            .iter()
            {
                let mut new_ii = ii as isize;
                let mut new_jj = jj as isize;

                loop {
                    new_ii += ii_increment;
                    new_jj += jj_increment;

                    if new_jj < 0 || new_ii < 0 {
                        break;
                    }

                    match is_occupied(new_ii as usize, new_jj as usize, collection) {
                        Occupation::Occupied => {
                            occupied_adjacent += 1;
                            break;
                        }
                        Occupation::NonExistant | Occupation::Empty => break,
                        Occupation::Floor => {
                            continue;
                        }
                    }
                }
            }
        }
    }

    occupied_adjacent
}

fn update_collection(
    initial_collection: Vec<Vec<char>>,
    max_distance: MaxDistance,
    max_company: usize,
) -> Vec<Vec<char>> {
    let mut updated_collection = initial_collection.clone();

    for ii in 0..initial_collection.len() {
        for jj in 0..initial_collection[0].len() {
            let initial_value = initial_collection[ii][jj];

            // println!(
            //     "initial_value: {} ii: {}  jj: {}",
            //     initial_value, ii, jj
            // );

            let occupied_adjacent =
                occupied_adjacent(ii as isize, jj as isize, &initial_collection, max_distance);

            // println!(
            //     "occupied_adjacent: {}",
            //     occupied_adjacent
            // );

            if initial_value == 'L' && occupied_adjacent == 0 {
                updated_collection[ii][jj] = '#';
            };

            if initial_value == '#' && occupied_adjacent >= max_company {
                updated_collection[ii][jj] = 'L';
            };
        }
    }

    updated_collection
}

fn total_occupied(collection: &[Vec<char>]) -> usize {
    collection
        .iter()
        .map(|x| x.iter().filter(|y| y == &&'#').count())
        .sum()
}

fn print_collection(collection: &[Vec<char>]) {
    for vector in collection.iter() {
        println!("{:?}", vector.iter().collect::<String>());
    }
}

fn generic_part(filename: &str, max_distance: MaxDistance, max_company: usize) {
    println!("\nReading file {:?}", filename);

    let collection: Vec<Vec<char>> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    println!("\nInitial collection\n");
    print_collection(&collection);

    let mut previous_collection = collection;

    loop {
        let next_collection =
            update_collection(previous_collection.clone(), max_distance, max_company);

        // println!("Next collection: ");
        // print_collection(&next_collection);

        if next_collection == previous_collection {
            break;
        }

        previous_collection = next_collection;
    }

    println!("\nFinal collection\n");
    print_collection(&previous_collection);

    println!(
        "\nTotal occupied {:?}",
        total_occupied(&previous_collection)
    );
}

fn part_one(filename: &str) {
    println!("Part One");
    generic_part(filename, MaxDistance::One, 4);
}

fn part_two(filename: &str) {
    println!("Part Two");
    generic_part(filename, MaxDistance::Infinity, 5);
}

fn main() {
    part_one("example_input1.txt");
    part_one("input1.txt");
    part_two("example_input1.txt");
    part_two("input1.txt");
}
