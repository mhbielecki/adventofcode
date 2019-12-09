use std::fs;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    let filename = "input/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");

    let input: Vec<&str> = contents.split("\n").collect();
    let w1_cords = generate_cordinates(input[0].split(",").collect());
    let w2_cords = generate_cordinates(input[1].split(",").collect());

    let crossed_points: HashSet<_> = w1_cords.intersection(&w2_cords).collect();
    let mut manhattans: Vec<i32> = crossed_points.iter().map(|x| manhattan_distance(x.0, x.1)).collect();
    manhattans.sort();

    println!("Part 1: {}", manhattans[0]);
}

fn generate_cordinates(wire_path: Vec<&str>) -> HashSet<(i32, i32)>	{
    let mut h = HashSet::new();
    let re = Regex::new(r"([UDLR])(\d+)").unwrap();
    let mut current_pos = (0,0);

    for p in wire_path {
        let caps = re.captures(p).unwrap();
        let dir = caps.get(1).map_or("", |m| m.as_str());
        let steps = caps.get(2).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
        if dir == "U" {
            for n in current_pos.1..=current_pos.1+steps {
                h.insert((current_pos.0, n));
            }
            current_pos.1 += steps;
        }
        else if dir == "D" {
            for n in current_pos.1-steps..=current_pos.1 {
                h.insert((current_pos.0, n));
            }
            current_pos.1 -= steps;

        }
        else if dir == "L" {
            for n in current_pos.0-steps..=current_pos.0 {
                h.insert((n, current_pos.1));
            }
            current_pos.0 -= steps;

        }
        else {
            for n in current_pos.0..=current_pos.0+steps {
                h.insert((n, current_pos.1));
            }
            current_pos.0 += steps;
        }
    }

    h.remove(&(0,0));
    return h;
}

fn manhattan_distance(x: i32, y: i32) -> i32 {
    return (x-0).abs() + (y-0).abs();
}