use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    let filename = "input/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");

    let input: Vec<&str> = contents.split("\n").collect();
    let w1_cords_with_steps = generate_cordinates(input[0].split(",").collect());
    let w2_cords_with_steps = generate_cordinates(input[1].split(",").collect());

    //Part 1
    let mut w1_cords = HashSet::new();
    for k in w1_cords_with_steps.keys() {
        w1_cords.insert(k);
    }

    let mut w2_cords = HashSet::new();
    for k in w2_cords_with_steps.keys() {
        w2_cords.insert(k);
    }

    let crossed_points: HashSet<_> = w1_cords.intersection(&w2_cords).collect();
    let mut manhattans: Vec<i32> = crossed_points.iter().map(|x| manhattan_distance(x.0, x.1)).collect();
    manhattans.sort();
    println!("Part 1: {}", manhattans[0]);

    //Part 2

    let mut min_signal_delay = std::i32::MAX;
    for cp in crossed_points {
        let a = w1_cords_with_steps.get(cp);
        let b = w2_cords_with_steps.get(cp);
        println!("{} {}", cp.0, cp.1);
        match (a, b) {
            (Some(x), Some(y)) => if x+y < min_signal_delay { min_signal_delay = x+y}
            _ => println!("Error")
        }
        println!()
    }

    println!("Part 2: {}", min_signal_delay);
}

fn generate_cordinates(wire_path: Vec<&str>) -> HashMap<(i32, i32), i32> {
    let mut h = HashMap::new();
    let re = Regex::new(r"([UDLR])(\d+)").unwrap();
    let mut current_pos = (0,0);
    let mut total_steps = 0;

    for p in wire_path {
        let caps = re.captures(p).unwrap();
        let dir = caps.get(1).map_or("", |m| m.as_str());
        let steps = caps.get(2).map_or("", |m| m.as_str()).parse::<i32>().unwrap();
        if dir == "U" {
            for n in current_pos.1+1..=current_pos.1+steps {
                total_steps+=1;
                if !h.contains_key(&(current_pos.0, n)) {
                    println!("{} {} {} {}", dir, current_pos.0, n, total_steps);
                    h.insert((current_pos.0, n), total_steps);
                }
            }
            current_pos.1 += steps;
        }
        else if dir == "D" {
            for n in (current_pos.1..=current_pos.1+steps).rev() {
                total_steps+=1;
                if !h.contains_key(&(current_pos.0, n)) {
                    println!("{} {} {} {}", dir, current_pos.0, n, total_steps);
                    h.insert((current_pos.0, n), total_steps);
                }
            }
            current_pos.1 -= steps;

        }
        else if dir == "L" {
            for n in (current_pos.0..=current_pos.0+steps).rev() {
                total_steps+=1; //speilvenna L og D, Ikke generer fra bakover...
                if !h.contains_key(&(n, current_pos.1)) {
                    println!("{} {} {} {}", dir, n, current_pos.1, total_steps);
                    h.insert((n, current_pos.1), total_steps);
                }
                
            }
            current_pos.0 -= steps;

        }
        else {
            for n in current_pos.0+1..=current_pos.0+steps {
                total_steps+=1;
                if !h.contains_key(&(n, current_pos.1)) {
                    println!("{} {} {} {}", dir, n, current_pos.1, total_steps);
                    h.insert((n, current_pos.1), total_steps);
                }
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