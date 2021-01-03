use petgraph::dot::{Config, Dot};
use petgraph::prelude::NodeIndex;
use petgraph::Graph;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::time::Instant;

fn get_sorted_collection(filename: String) -> Vec<usize> {
    let mut collection: Vec<usize> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    collection.sort_unstable();
    collection
}

fn get_collection_set(filename: String) -> HashSet<usize> {
    let collection: HashSet<usize> = fs::read_to_string(filename)
        .expect("Something went wrong reading the file")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<HashSet<usize>>();
    collection
}

fn part_one(filename: String) {
    println!("Part 1 answer for {:?}", filename);

    let sorted_collection = get_sorted_collection(filename);

    let (one_volt_differences, three_volt_differences, _) = sorted_collection.iter().fold(
        (0, 1, 0),
        |(one_volt_differences, three_volt_differences, previous_value), x| match x - previous_value
        {
            1 => (one_volt_differences + 1, three_volt_differences, *x),
            3 => (one_volt_differences, three_volt_differences + 1, *x),
            _ => unimplemented!(),
        },
    );
    println!("One volt differences: {:?}", one_volt_differences);
    println!("Three volt differences: {:?}", three_volt_differences);
    println!(
        "3v x 1v: {:?}",
        three_volt_differences * one_volt_differences
    );
}

fn create_collection_part_two(filename: String) -> (HashSet<usize>, usize) {
    let mut now = Instant::now();

    println!("\nPart 2 answer for {:?}", filename);
    let mut collection: HashSet<usize> = get_collection_set(filename);
    collection.insert(0);

    println!("Created HashSet in {:#?}", now.elapsed());
    now = Instant::now();

    let max_power = collection.iter().max().unwrap() + 3;
    collection.insert(max_power);

    println!("Got largest value in {:#?}", now.elapsed());
    (collection, max_power)
}

fn part_two(filename: String) {
    let mut part_two_time = Instant::now();
    let (collection, max_power) = create_collection_part_two(filename);
    let mut now = Instant::now();

    let mut graph = Graph::<usize, usize>::new();

    let mut node_indecies: HashMap<usize, NodeIndex> = HashMap::new();

    for power_output in collection.iter() {
        let node_index = graph.add_node(*power_output);
        node_indecies.insert(*power_output, node_index);
    }

    println!("Added nodes in {:#?}", now.elapsed());
    now = Instant::now();

    for power_output in collection.iter() {
        let allowed_increments = vec![power_output + 1, power_output + 2, power_output + 3];
        for increment in allowed_increments.iter() {
            if collection.contains(&(increment)) {
                graph.extend_with_edges(&[(
                    node_indecies.get(power_output).unwrap().to_owned(),
                    node_indecies.get(increment).unwrap().to_owned(),
                )])
            }
        }
    }

    println!("Added edges in {:#?}", now.elapsed());
    now = Instant::now();

    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    // println!("{:?}", graph);

    let all_paths: Vec<Vec<NodeIndex>> =
        petgraph::algo::all_simple_paths::<Vec<NodeIndex>, &petgraph::Graph<usize, usize>>(
            &graph,
            node_indecies.get(&0).unwrap().to_owned(),
            node_indecies.get(&max_power).unwrap().to_owned(),
            0,
            None,
        )
        .collect();

    for path in all_paths.iter() {
        let understood_path: Vec<usize> = path
            .iter()
            .map(|x| graph.node_weight(*x).unwrap().to_owned())
            .collect();
        // println!("A path {:?}", understood_path);
    }

    println!("Total paths {:?}", all_paths.iter().count());

    println!("Found paths in {:#?}", now.elapsed());
    println!("Total time for this file {:#?}", part_two_time.elapsed());
}

fn part_two_attempt_two(filename: String) {
    println!("\nPart 2 attempt 2 answer for {:?}", filename);
    let mut sorted_collection: Vec<usize> = get_sorted_collection(filename);

    sorted_collection.insert(0, 0);
    sorted_collection.reverse();

    let mut paths_to_end: HashMap<usize, usize> = HashMap::new();
    paths_to_end.insert(sorted_collection.first().unwrap() + 3, 1);

    for number in sorted_collection.iter() {
        let total_paths_to_end = paths_to_end.get(&(number + 1)).unwrap_or(&0)
            + paths_to_end.get(&(number + 2)).unwrap_or(&0)
            + paths_to_end.get(&(number + 3)).unwrap_or(&0);
        paths_to_end.insert(*number, total_paths_to_end);
    }

    println!("{:?}", paths_to_end);
    println!("Total paths {:?}", paths_to_end.get(&0).unwrap());
}

fn main() {
    part_one("example_input1.txt".to_string());
    part_one("input1.txt".to_string());
    // part_two("simple_example_input.txt".to_string());
    // part_two("example_input1.txt".to_string());
    part_two_attempt_two("simple_example_input.txt".to_string());
    part_two_attempt_two("example_input1.txt".to_string());
    part_two_attempt_two("input1.txt".to_string());
    // part_two("input1.txt".to_string());
    // part_two_with_csr("simple_example_input.txt".to_string());
    // part_two_with_csr("example_input1.txt".to_string());
}
