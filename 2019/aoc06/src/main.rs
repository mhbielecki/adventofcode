use std::fs;
use std::collections::HashMap;

fn main() {
    let filename = "input/input.txt";
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file.");

    let orbits: Vec<&str> = contents.split_whitespace().collect();
    let mut objects: HashMap<&str, Vec<&str>> = HashMap::new();

    for orbit in orbits.iter() {
        let s: Vec<&str> = orbit.split(')').collect();
        let orbiters = objects.entry(s[0]).or_insert(Vec::new());
        orbiters.push(s[1]);
    }

    println!("{}",rec("COM", 1, &objects));
}

fn rec(n: &str, depth: usize, objects: &HashMap<&str, Vec<&str>>) -> usize {
    match objects.get(n) {
        Some(v) => (depth * v.len()) + v.iter().fold(0, |acc, x| acc + rec(x, depth+1, &objects)),
        _ => 0
    }
}
