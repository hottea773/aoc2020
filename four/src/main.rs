use regex::Regex;
use std::convert::TryFrom;
use std::fs;

#[derive(Debug)]
struct Height {
    unit: String,
    value: u8,
}

impl TryFrom<String> for Height {
    type Error = &'static str;

    fn try_from(item: String) -> Result<Self, Self::Error> {
        let len = item.len();

        if len < 4 {
            return Err("Whoops, that's not a height");
        }

        let maybe_unit = &item[(len - 2)..];

        let maybe_value = if let Ok(maybe_value) = item[..(len - 2)].parse::<u8>() {
            maybe_value
        } else {
            return Err("Whoops, that's not a height");
        };

        if ((maybe_unit == "in") && (maybe_value <= 76) && (maybe_value >= 59))
            || (maybe_unit == "cm" && maybe_value >= 150 && maybe_value <= 193)
        {
            Ok(Height {
                unit: maybe_unit.to_string(),
                value: maybe_value,
            })
        } else {
            // What do I put here if the unit is neither cm of in ?
            Err("Whoops, that's not a height")
        }
    }
}

#[derive(Debug)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<Height>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u16>,
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
        // && self.cid.is_some()
    }
}

impl From<String> for Passport {
    fn from(item: String) -> Self {
        let passport_strings: Vec<&str> = item.split(' ').collect::<Vec<&str>>();

        let mut building_passport = Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        };

        for entry in passport_strings {
            // Skip invalid entries
            if entry.trim().is_empty() {
                continue;
            }

            let key = entry.split(':').collect::<Vec<&str>>()[0];
            let value = entry.split(':').collect::<Vec<&str>>()[1];
            // println!("key: {:?} value: {} entry {}", key, value, entry);
            match key {
                "byr" => {
                    let maybe_byr = value.parse().unwrap();
                    if 1920 <= maybe_byr && maybe_byr <= 2002 && value.len() == 4 {
                        building_passport.byr = Some(maybe_byr);
                    }
                }
                "iyr" => {
                    let maybe_iyr = value.parse().unwrap();
                    if 2010 <= maybe_iyr && maybe_iyr <= 2020 && value.len() == 4 {
                        building_passport.iyr = Some(maybe_iyr);
                    }
                }
                "eyr" => {
                    let maybe_eyr = value.parse().unwrap();
                    if 2020 <= maybe_eyr && maybe_eyr <= 2030 && value.len() == 4 {
                        building_passport.eyr = Some(maybe_eyr);
                    }
                }
                "hgt" => {
                    let maybe_height = Height::try_from(value.to_string());
                    match maybe_height {
                        Ok(height) => building_passport.hgt = Some(height),
                        Err(_) => building_passport.hgt = None,
                    }
                }
                "hcl" => {
                    let re = Regex::new(r"#[0-9a-f]{6}").unwrap();
                    let maybe_hcl = value;
                    if re.is_match(maybe_hcl) {
                        building_passport.hcl = Some(maybe_hcl.to_string());
                    }
                }
                "ecl" => {
                    let maybe_ecl = value;
                    let ecls = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    if ecls.contains(&maybe_ecl) {
                        building_passport.ecl = Some(maybe_ecl.to_string());
                    }
                }
                "pid" => {
                    if value.len() == 9 {
                        building_passport.pid = Some(value.parse().unwrap());
                    }
                }
                "cid" => {
                    building_passport.cid = Some(value.parse().unwrap());
                }
                _ => {
                    println!(
                        " unimplemented key: {:?} value: {} entry {}",
                        key, value, entry
                    );
                    unimplemented!()
                }
            }
        }
        building_passport
    }
}

fn get_passport_list(filename: &str) {
    // println!("In file {}", filename);

    let passports: Vec<Passport> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .replace("\r\n", "\n")
        .split("\n\n")
        .map(|x| Passport::from(x.to_string().replace("\n", " ")))
        .collect();
    // println!("{:#?}", passports);

    let total_valid: u32 = passports
        .iter()
        .map(|x| if x.is_valid() { 1 } else { 0 })
        .sum();
    println!("total valid: {:?}", total_valid);
}
fn main() {
    let _inputs = get_passport_list("input.txt");
    let _test_inputs = get_passport_list("test_input.txt");
    let _test_inputs = get_passport_list("valid_input.txt");
}
