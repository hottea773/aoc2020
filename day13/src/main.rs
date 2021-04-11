use num_format::{Locale, ToFormattedString};
use std::fs;

#[derive(Clone, Debug)]
struct Notes {
    departure_time: u32,
    buses_in_service: Vec<(usize, u32)>,
}

fn get_notes(filename: &str) -> Notes {
    let lines: Vec<String> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let buses_in_service: Vec<(usize, u32)> = lines
        .get(1)
        .unwrap()
        .split(',')
        .enumerate()
        .filter(|(_, y)| y != &"x")
        .map(|(i, x)| (i, x.parse().unwrap()))
        .collect();

    Notes {
        departure_time: lines.get(0).unwrap().parse().unwrap(),
        buses_in_service,
    }
}

fn part_one(filename: &str) {
    println!("\nPart One");
    println!("Reading file {:?}", filename);

    let notes = get_notes(filename);

    println!("Got notes: {:?}", notes);

    let mut smallest_remainder: u32 = std::u32::MAX;
    let mut correct_bus: u32 = std::u32::MAX;

    for bus in notes.buses_in_service.iter().map(|(_i, x)| x) {
        let remainder = bus - (notes.departure_time % bus);

        if remainder < smallest_remainder {
            smallest_remainder = remainder;
            correct_bus = *bus;
        }
    }

    println!(
        "Catch bus {} with a wait of {} minutes which multiply to {}",
        correct_bus,
        smallest_remainder,
        correct_bus * smallest_remainder
    );
}

fn part_two(filename: &str) {
    println!("\nPart Two");
    println!("Reading file {:?}", filename);

    let notes = get_notes(filename);

    println!("Got notes: {:?}", notes);

    let largest_number = notes
        .buses_in_service
        .iter()
        .max_by_key(|(_, x)| x)
        .unwrap();

    println!("Found largest number: {:?}", largest_number);

    for ii in 0..std::u64::MAX {
        let simple_ii = (ii * largest_number.1 as u64) - largest_number.0 as u64;

        if ii.is_power_of_two() {
            println!(
                "At ii of {} and simple_ii of {} at time {:?}",
                ii.to_formatted_string(&Locale::en),
                simple_ii.to_formatted_string(&Locale::en),
                chrono::Utc::now()
            )
        }

        let mut correct_ii = true;
        for bus in notes.buses_in_service.iter() {
            if (simple_ii + bus.0 as u64) % bus.1 as u64 != 0_u64 {
                correct_ii = false;
                break;
            };
        }

        if correct_ii {
            println!("Found correct time which is {}", simple_ii);
            break;
        }
    }
}

fn main() {
    part_one("example_input.txt");
    part_one("input1.txt");
    part_two("example_input.txt");
    part_two("input1.txt");
}
