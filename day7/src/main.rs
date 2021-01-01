use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Colour {
    shade: String,
    base_colour: String,
}

#[derive(Debug, PartialEq, Clone)]
struct BagType {
    colour: Colour,
    contents: Option<Vec<BagQuantity>>,
    is_fully_resolved: bool,
    can_contain_gold: Option<bool>,
}

#[derive(Debug, PartialEq, Clone)]
struct BagQuantity {
    bag: BagType,
    quantity: u32,
}

impl BagType {
    fn from_string(input_string: String) -> BagType {
        let (colour, unparsed_contents) = input_string
            .split("bags contain")
            .map(|x| x.trim())
            .collect_tuple()
            .unwrap();
        let (shade, base_colour) = colour.split_whitespace().collect_tuple().unwrap();
        let contents: Vec<BagQuantity> =
            unparsed_contents
                .trim_end_matches(".")
                .split(", ")
                .fold(Vec::new(), |mut acc, x| {
                    let (unparsed_quantity, shade, base_colour) =
                        x.split_whitespace().next_tuple().unwrap();

                    if unparsed_quantity == "no" {
                        return acc;
                    };

                    let quantity: u32 = unparsed_quantity.parse().unwrap();

                    let bag = BagType {
                        colour: Colour {
                            shade: shade.to_string(),
                            base_colour: base_colour.to_string(),
                        },
                        contents: None,
                        is_fully_resolved: false,
                        can_contain_gold: None,
                    };
                    acc.push(BagQuantity { bag, quantity });
                    acc
                });

        BagType {
            colour: Colour {
                shade: shade.to_string(),
                base_colour: base_colour.to_string(),
            },
            contents: Some(contents),
            is_fully_resolved: false,
            can_contain_gold: None,
        }
    }
}

// fn resolve_bag_rules(bag_rules: Vec<BagType>) -> Vec<BagType> {
//     fn resolve_bag(bag: BagType, bag_rules: Vec<BagType>) -> Vec<BagType> {
//         if bag.is_fully_resolved == true {
//             return;
//         }

//         let mut thing = bag.contents.unwrap();
//         // let &mut thing: std::vec::Vec<BagQuantity> = bag.contents.as_ref().unwrap();

//         for mut content_bag in thing.iter_mut() {
//             println!("content_bag {:?}", content_bag);
//             let corresponding_bag = bag_rules.iter().find(|x| x.colour == content_bag.bag.colour).unwrap();
//             resolve_bag(corresponding_bag, bag_rules);
//             content_bag.bag = corresponding_bag.clone();
//         }
//     }
//     for bag in bag_rules {
//         resolve_bag(bag, bag_rules)
//     }
//     bag_rules
// }

fn bags_can_contain_gold(bag_rules: Vec<BagType>) -> Vec<BagType> {
    let mut updated_list: Vec<BagType> = Vec::new();

    let mut ii = 0;

    let mut gold_containing_colours: HashSet<Colour> = HashSet::new();
    let mut not_gold_containing_colours: HashSet<Colour> = HashSet::new();

    for bag in &bag_rules {
        ii += 1;
        let mut bag_clone = bag.clone();

        if gold_containing_colours.contains(&bag.colour) {
            bag_clone.can_contain_gold = Some(true)
        } else if not_gold_containing_colours.contains(&bag.colour) {
            bag_clone.can_contain_gold = Some(false)
        } else {
            let (can_contain_gold, some_gold_containing_colours, some_not_gold_containing_colours) =
                can_contain_gold(bag.to_owned(), bag_rules.clone());
            gold_containing_colours.extend(some_gold_containing_colours);
            not_gold_containing_colours.extend(some_not_gold_containing_colours);

            bag_clone.can_contain_gold = Some(can_contain_gold);
        }

        println!(
            "found an {}th {} {} bag which contains gold: {:?}",
            ii, bag_clone.colour.shade, bag_clone.colour.base_colour, bag_clone.can_contain_gold
        );
        updated_list.push(bag_clone);
    }
    updated_list
}

fn can_contain_gold(
    bag: BagType,
    bag_rules: Vec<BagType>,
) -> (bool, HashSet<Colour>, HashSet<Colour>) {
    let gold_colour = Colour {
        shade: "shiny".to_string(),
        base_colour: "gold".to_string(),
    };

    let mut gold_containing_colours: HashSet<Colour> = HashSet::new();
    let mut not_gold_containing_colours: HashSet<Colour> = HashSet::new();

    if bag.colour == (gold_colour)
        || bag.contents.is_none()
        || bag.contents.clone().unwrap_or_default().is_empty()
    {
        return (false, gold_containing_colours, not_gold_containing_colours);
    }

    let mut contain_gold = false;
    for content_bag in bag.contents.unwrap() {
        if content_bag.quantity == 0 {
            continue;
        };

        if content_bag.bag.can_contain_gold.unwrap_or(false) {
            contain_gold = true;
            break;
        }

        if content_bag.bag.colour == gold_colour {
            contain_gold = true;
            break;
        }

        let corresponding_bag = bag_rules
            .clone()
            .iter()
            .find(|x| x.colour == content_bag.bag.colour)
            .unwrap()
            .to_owned();

        let (
            corresponding_bag_can_contain_gold,
            corresponding_bag_gold_containing_colours,
            corresponding_bag_not_gold_containing_colours,
        ) = can_contain_gold(corresponding_bag, bag_rules.clone());

        gold_containing_colours.extend(corresponding_bag_gold_containing_colours);
        not_gold_containing_colours.extend(corresponding_bag_not_gold_containing_colours);

        if corresponding_bag_can_contain_gold {
            contain_gold = true;
            break;
        }
    }
    if contain_gold {
        gold_containing_colours.insert(bag.colour)
    } else {
        not_gold_containing_colours.insert(bag.colour)
    };

    (
        contain_gold,
        gold_containing_colours,
        not_gold_containing_colours,
    )
}

fn part_one(filename: String) {
    println!("Answer for {:?}", filename);
    let contents: Vec<String> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let bag_rules: Vec<BagType> = contents
        .iter()
        .map(|line| BagType::from_string(line.to_string()))
        .fold(Vec::new(), |mut acc, x| {
            acc.push(x);
            acc
        });

    let bags_can_contain_gold = bags_can_contain_gold(bag_rules);
    //println!("Unresolved Bag Rules: {:#?}", bag_rules);
    // println!("bags_can_contain_gold: {:#?}", bags_can_contain_gold);

    let total_bags_can_contain_gold: u32 = bags_can_contain_gold
        .iter()
        .map(|x| {
            if x.can_contain_gold.unwrap_or(false) {
                1
            } else {
                0
            }
        })
        .sum();
    println!(
        "total_bags_can_contain_gold {:?}",
        total_bags_can_contain_gold
    );
}

fn bags_contained_by(colour: Colour, bag_rules: Vec<BagType>) -> u32 {
    let mut bags_contained_by_int: u32 = 0;

    let bag = bag_rules
        .iter()
        .find(|x| x.colour == colour)
        .unwrap()
        .to_owned();

    if bag.contents.is_none() || bag.contents.clone().unwrap_or_default().is_empty() {
        return 0;
    }

    // #[derive(Debug, PartialEq, Clone)]
    // struct BagType {
    //     colour: Colour,
    //     contents: Option<Vec<BagQuantity>>,
    //     is_fully_resolved: bool,
    //     can_contain_gold: Option<bool>,
    // }

    // #[derive(Debug, PartialEq, Clone)]
    // struct BagQuantity {
    //     bag: BagType,
    //     quantity: u32,
    // }

    for content_bag in bag.contents.unwrap() {
        bags_contained_by_int += content_bag.quantity
            * (1 + bags_contained_by(content_bag.bag.colour, bag_rules.clone()))
    }

    bags_contained_by_int
}

fn part_two(filename: String) {
    println!("Answer for {:?}", filename);
    let contents: Vec<String> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.to_string())
        .collect();

    let bag_rules: Vec<BagType> = contents
        .iter()
        .map(|line| BagType::from_string(line.to_string()))
        .fold(Vec::new(), |mut acc, x| {
            acc.push(x);
            acc
        });

    let total_bags_gold_contains: u32 = bags_contained_by(
        Colour {
            shade: "shiny".to_string(),
            base_colour: "gold".to_string(),
        },
        bag_rules,
    );

    println!("total_bags_can_contain_gold {:?}", total_bags_gold_contains);
}

fn main() {
    let filename = "test_inputp1.txt".to_string();
    part_one(filename);
    // let filename2 = "inputp1.txt".to_string();
    // part_one(filename2);

    let filename = "test_inputp1.txt".to_string();
    part_two(filename);
    let filename = "inputp1.txt".to_string();
    part_two(filename);
}
