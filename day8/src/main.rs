use itertools::Itertools;
use std::fmt;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Acc {
    increment: i32,
}

#[derive(Debug, Clone)]
struct Jmp {
    increment: i32,
}

#[derive(Debug, Clone)]
struct NoOp {
    increment: i32,
}

#[derive(Debug, Clone)]
enum Instruction {
    Acc(Acc),
    Jmp(Jmp),
    NoOp(NoOp),
}

#[derive(Debug, Clone)]
struct InstructionParsingError {
    instruction_string: String,
    reason_string: String,
}

impl fmt::Display for InstructionParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to generate instruction from {}; {}",
            self.instruction_string, self.reason_string
        )
    }
}

impl FromStr for Instruction {
    type Err = InstructionParsingError;

    fn from_str(input_string: &str) -> Result<Self, Self::Err> {
        let (action_str, increment_str) = input_string
            .split_whitespace()
            .collect_tuple()
            .ok_or_else(|| InstructionParsingError {
                instruction_string: input_string.to_string(),
                reason_string: "Failed to split into instruction action and increment".to_string(),
            })?;

        let increment_result = increment_str.parse::<i32>();
        // .unwrap_or_else(|x| InstructionParsingError {
        //     instruction_string: input_string.to_string(),
        //     reason_string: "Failed to split into instruction action and increment".to_string(),
        // })?;
        let increment = match increment_result {
            Ok(increment) => increment,
            Err(_) => {
                return Err(InstructionParsingError {
                    instruction_string: input_string.to_string(),
                    reason_string: "Failed to split into instruction action and increment"
                        .to_string(),
                })
            }
        };

        match action_str {
            "acc" => Ok(Instruction::Acc(Acc { increment })),
            "jmp" => Ok(Instruction::Jmp(Jmp { increment })),
            "nop" => Ok(Instruction::NoOp(NoOp { increment })),
            _ => {
                return Err(InstructionParsingError {
                    instruction_string: input_string.to_string(),
                    reason_string: "Unrecognized instruction.".to_string(),
                })
            }
        }
    }
}



fn get_final_accumulator(instructions: Vec<Instruction>) -> i32 {
  let mut locations_visited: Vec<i32> = Vec::new();
  let mut acc: i32 = 0;

  let mut current_location: i32 = 0;

  loop {

    // println!("{:?} {}", current_location, instructions.len());

    if locations_visited.contains(&current_location) {
      // println!("Returning to previous visited location {:?}", current_location);
      // println!("Got Acc {:?}", acc);
      return acc;
    }


    if current_location as usize == instructions.len() {
      println!("Hit location 1 beyond end of string {:?}", current_location);
      println!("Got Acc {:?}", acc);
      return acc;
    }

    locations_visited.push(current_location);

    match &instructions[current_location as usize] {
      Instruction::Acc(local_acc) => {
        acc += local_acc.increment;
        current_location += 1;
      }
      Instruction::Jmp(jmp) => {
        current_location = current_location + jmp.increment;
      }
      Instruction::NoOp(_) => {
        current_location += 1;
      }
    }
  }

}

fn part_one(filename: String) {
    println!("Answer for {:?}", filename);
    let contents: Vec<Instruction> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| Instruction::from_str(x))
        .collect::<Result<Vec<Instruction>, InstructionParsingError>>()
        .unwrap();

    // println!("{:#?}", contents);
    for (ii, _) in contents.iter().enumerate() {
      match &contents[ii] {
        Instruction::Acc(_) => continue,
        Instruction::Jmp(jmp) => {
          let mut contents_clone = contents.clone();
          contents_clone[ii] = Instruction::NoOp(NoOp { increment: jmp.increment });
          get_final_accumulator(contents_clone);
        }
        Instruction::NoOp(nop) => {
          let mut contents_clone = contents.clone();
          contents_clone[ii] = Instruction::Jmp(Jmp { increment: nop.increment });
          get_final_accumulator(contents_clone);
        }
      }
    }
}

fn main() {
    part_one("example_input.txt".to_string());
    part_one("input1.txt".to_string());
}
