
use std::convert::TryInto;
use std::fs;

fn main() {
    let filename = "input.txt";

    println!("In file {}", filename);

    let contents: Vec<String> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file").lines().map(|x| x.to_string()).collect();

    let mut number_valid_one = 0;
    let mut number_valid_two = 0;

    for line in contents.iter() {
        let my_5_item_vec = line.split(&[' ', '-', ':'][..]).collect::<Vec<&str>>();

        let (lowest_number, highest_number, character, password): (u8, u8, &str, &str) = (my_5_item_vec[0].parse().unwrap(), my_5_item_vec[1].parse().unwrap(), my_5_item_vec[2], my_5_item_vec[4]);

        // println!("{:?} {} {} {} {} {}", my_5_item_vec, lowest_number, highest_number, character, password, password.matches(character).count());


        // Problem one
        let number_matches_one: u8 = password.matches(character).count().try_into().unwrap();
        if (lowest_number <= number_matches_one) && (number_matches_one <= highest_number) {
          number_valid_one += 1;
        }

        // Problem two

        let char_vec: Vec<char> = password.chars().collect();
        let character_char_vec: Vec<char> = character.chars().collect();
        let low_entry: usize = (lowest_number - 1).try_into().unwrap();
        let high_entry: usize = (highest_number - 1).try_into().unwrap();
          // println!("{:?} {} {} {} {} {}", my_5_item_vec, lowest_number, highest_number, character, password, password.matches(character).count());
        if (char_vec[low_entry] == character_char_vec[0]) ^ (char_vec[high_entry] == character_char_vec[0]) {
          println!("{:?} {} {} {} {} {}", my_5_item_vec, lowest_number, highest_number, character, password, password.matches(character).count());
          number_valid_two += 1;
        }

    }

    println!("Number of matches in version 1 {}", number_valid_one);
    println!("Number of matches in version 2 {}", number_valid_two);
}
