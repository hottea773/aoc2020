use std::convert::TryInto;
use std::fs;

// FBFBBFFRLR
// 0101100

#[derive(Debug)]
struct Seat {
    raw: String,
    row_number: usize,
    column_number: usize,
}

impl Seat {
    fn seat_id(&self) -> usize {
        self.row_number * 8 + self.column_number
    }

    fn from_string(input_str: String) -> Seat {
        fn bin_to_int(bin_str: &str, ones_char: char, _noughts_char: char) -> usize {
            let reverse_string = bin_str.chars().rev();
            let mut total: usize = 0;
            for (index, item) in reverse_string.enumerate() {
                // println!("Item: {:?}\nIndex: {}", item, index);
                if item == ones_char {
                    // println!("total {:?}", total);
                    total += 2_usize.pow(index.try_into().unwrap());
                }
            }
            total
        }

        Seat {
            raw: input_str.clone(),
            row_number: bin_to_int(&input_str[..7], 'B', 'F'),
            column_number: bin_to_int(&input_str[7..], 'R', 'L'),
        }
    }
}

fn main() {
    let filename = "input.txt";
    let seats: Vec<Seat> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| Seat::from_string(x.to_string()))
        .collect();

    let mut seat_ids: Vec<usize> = seats.iter().map(|seat| seat.seat_id()).collect();
    seat_ids.sort_unstable();

    for (index, seat_id) in seat_ids.iter().enumerate() {
        if seat_ids[index + 1] != seat_id + 1 {
            println!("Missing seat {:?}", seat_id + 1);
        }
    }
    println!("{:?}", seat_ids);

    // let test_row = "FFFBBBFRRR";
    // let test_seat = Seat::from_string(test_row.to_string());
    // println!("Test seat {:#?}\nRow ID: {}", test_seat, test_seat.seat_id());

    // println!("Hello, world! {:#?}", seats);
}
