use std::fs;

fn get_number_of_trees(right: usize, down: usize) -> u8 {
    let filename = "input.txt";
    let contents: Vec<String> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let mut number_of_trees = 0;
    for ii in 0..(contents.len() / down) {
        let jj = ii * down;
        if contents[jj].chars().collect::<Vec<char>>()[(right * ii % 31)] == '#' {
            number_of_trees += 1;
        }
    }
    println!(
        "Going right {} and down {}; the Number of trees is: {}",
        right, down, number_of_trees
    );
    number_of_trees
}

fn main() {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let mut total: u64 = 1;

    for slope in slopes {
        let numb: u64 = get_number_of_trees(slope.0, slope.1).into();
        total *= numb;
        println!("New total {:?}", total);
    }
}
