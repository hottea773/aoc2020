use std::fs;

#[derive(Copy, Clone, Debug)]
enum Instruction {
    North(usize),
    South(usize),
    East(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl From<String> for Instruction {
    fn from(deg_str: String) -> Self {
        let action: char = deg_str.chars().nth(0).expect("Expected initial char");
        let size: usize = deg_str
            .chars()
            .skip(1)
            .collect::<String>()
            .parse()
            .expect("Expected size as int");

        match action {
            'N' => Instruction::North(size),
            'S' => Instruction::South(size),
            'E' => Instruction::East(size),
            'W' => Instruction::West(size),
            'L' => Instruction::Left(size),
            'R' => Instruction::Right(size),
            'F' => Instruction::Forward(size),
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Position {
    northing: isize,
    easting: isize,
    bearing: Option<isize>,
}

impl Position {
    fn update_ship_position(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(size) => self.northing += size as isize,
            Instruction::South(size) => self.northing -= size as isize,
            Instruction::East(size) => self.easting += size as isize,
            Instruction::West(size) => self.easting -= size as isize,
            Instruction::Left(size) => {
                self.bearing = Some((self.bearing.unwrap() - size as isize).rem_euclid(360))
            }
            Instruction::Right(size) => {
                self.bearing = Some((self.bearing.unwrap() + size as isize).rem_euclid(360))
            }
            Instruction::Forward(size) => {
                if self.bearing == Some(0) {
                    self.northing += size as isize
                } else if self.bearing == Some(90) {
                    self.easting += size as isize
                } else if self.bearing == Some(180) {
                    self.northing -= size as isize
                } else if self.bearing == Some(270) {
                    self.easting -= size as isize
                } else {
                    println!("Bearing {:?} Instruction {:?}", self.bearing, instruction);
                    unreachable!();
                }
            }
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Positions {
    ship: Position,
    waypoint: Position,
}

fn rotate(waypoint: Position, angle: isize) -> Position {
    let rads: f64 = 57.295779513;

    let cos = ((angle as f64 / rads).cos()) as isize;
    let sin = ((angle as f64 / rads).sin()) as isize;

    let old_northing = waypoint.northing;
    let old_easting = waypoint.easting;
    Position {
        northing: old_northing * cos - old_easting * sin,
        easting: old_northing * sin + old_easting * cos,
        bearing: None,
    }
}

impl Positions {
    fn update_positions(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::North(size) => self.waypoint.northing += size as isize,
            Instruction::South(size) => self.waypoint.northing -= size as isize,
            Instruction::East(size) => self.waypoint.easting += size as isize,
            Instruction::West(size) => self.waypoint.easting -= size as isize,
            Instruction::Left(size) => {
                self.waypoint = rotate(self.waypoint, -(size as isize));
            }
            Instruction::Right(size) => {
                self.waypoint = rotate(self.waypoint, size as isize);
            }
            Instruction::Forward(size) => {
                self.ship.northing += self.waypoint.northing * size as isize;
                self.ship.easting += self.waypoint.easting * size as isize;
            }
        }
    }
}

fn get_instructions(filename: &str) -> Vec<Instruction> {
    fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.to_string().into())
        .collect::<Vec<Instruction>>()
}

fn part_one(filename: &str) {
    println!("\nPart One");
    println!("Reading file {:?}", filename);

    let instructions: Vec<Instruction> = get_instructions(filename);

    // println!("Got input instructions: {:?}", instructions);

    let mut position: Position = Position {
        northing: 0,
        easting: 0,
        bearing: Some(90),
    };

    for instruction in instructions {
        position.update_ship_position(instruction);
    }

    // println!("Final Position: {:?}", position);
    println!(
        "Manhatten distance: {:?}",
        position.northing.abs() + position.easting.abs()
    );
}

fn part_two(filename: &str) {
    println!("\nPart Two");
    println!("Reading file {:?}", filename);

    let instructions: Vec<Instruction> = get_instructions(filename);

    // println!("Got input instructions: {:?}", instructions);
    let mut position: Positions = Positions {
        ship: Position {
            northing: 0,
            easting: 0,
            bearing: Some(90),
        },
        waypoint: Position {
            northing: 1,
            easting: 10,
            bearing: None,
        },
    };

    for instruction in instructions {
        position.update_positions(instruction);
    }

    println!("Final Position: {:?}", position);
    println!(
        "Manhatten distance: {:?}",
        position.ship.northing.abs() + position.ship.easting.abs()
    );
}

fn main() {
    part_one("example_input1.txt");
    part_one("input1.txt");
    part_two("example_input1.txt");
    part_two("input1.txt");
}
