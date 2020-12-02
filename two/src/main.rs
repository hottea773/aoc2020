
use std::fs;
use itertools::Itertools;

fn main() {
    let filename = "input.txt";

    println!("In file {}", filename);

    let contents: Vec<String> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").lines().map(|x| x.to_string()).collect();

    let mut number_valid = 0;

    for line in contents.iter() {
        let my_tuple: (&str, &str, &str) = line.split_whitespace().collect();
        println!("{:?}", my_tuple);
    }
}
