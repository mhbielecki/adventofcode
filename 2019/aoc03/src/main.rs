use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let filename = "input/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");

    let input: Vec<&str> = contents.split("\n").collect();

    let wire_1_path: Vec<&str> = input[0].split(",").collect();
    let wire_2_path: Vec<&str> = input[1].split(",").collect();

    let w1_cords = generate_cordinates(wire_1_path);
    let w2_cords = generate_cordinates(wire_2_path);

    let crossed_points: HashSet<_> = w1_cords.intersection(&w2_cords).collect();
    let mut manhattans: Vec<i32> = crossed_points.iter().map(|x| manhattan_distance(x.0, x.1)).collect();
    manhattans.sort();

    println!("{}", manhattans[0]);
}

fn generate_cordinates(wire_path: Vec<&str>) -> HashSet<(i32, i32)>	{
    // https://doc.rust-lang.org/std/collections/struct.HashSet.html
    // Use hashset with coords, intersect to find common, use manhattan distance to find closest
    let mut h = HashSet::new();
    let re = Regex::new(r"([UDLR])(\d+)").unwrap();

    for p in wire_path {
        let caps = re.captures(p).unwrap();
        let dir = caps.get(1).map_or("", |m| m.as_str());
        let end_pos = caps.get(2).map_or("", |m| m.as_str());
        println!("{} {}", dir, end_pos);
    }

    return h;
}

fn manhattan_distance(x: i32, y: i32) -> i32 {
    return (x-0).abs() + (y-0).abs();
}