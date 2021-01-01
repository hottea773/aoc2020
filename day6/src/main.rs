use std::collections::HashSet;
use std::fs;

// Part 1
// fn main() {
//     let filename = "input.txt";
//     let seats: Vec<HashSet<char>> = fs::read_to_string(filename)
//         .expect("Something went wrong reading the file")
//         .replace("\r\n", "\n")
//         .split("\n\n")
//         .map(|x| {
//             let mystr = x.to_string().replace("\n", "");
//             let mut empty_hash: HashSet<char> = HashSet::new();
//             for mychar in mystr.chars() {
//               empty_hash.insert(mychar);
//             }
//             empty_hash
//         })
//         .collect();
//     println!("Hello, world! {:#?}", seats);

//     let total: usize = seats.iter().map(|x| x.len()).sum();
//     println!("{:?}", total);
// }

fn main() {
    let filename = "input.txt";
    let seats: Vec<HashSet<char>> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|x| {
            let mystrvec: Vec<String> = x.to_string().split("\n").map(|y| y.to_string()).collect();
            let mut empty_hash: HashSet<char> = HashSet::new();

            for letter in "abcdefghijklmnopqrstuvwxyz".chars() {
                if mystrvec.iter().filter(|y| y.contains(letter)).count() == mystrvec.len() {
                    println!("match {}  in  {:?}", letter, mystrvec);
                    empty_hash.insert(letter);
                }
            }
            empty_hash
        })
        .collect();
    // println!("Hello, world! {:#?}", seats);

    let total: usize = seats.iter().map(|x| x.len()).sum();
    println!("{:?}", total);
}

