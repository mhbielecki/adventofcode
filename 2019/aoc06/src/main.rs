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
        let parent = objects.entry(s[0]).or_insert(Vec::new());
        parent.push(s[1]);

        // Comment for part 1
        let child = objects.entry(s[1]).or_insert(Vec::new());
        child.push(s[0]);
    }

    //println!("Part 1: {}",part_1("COM", 1, &objects));
    println!("Part 2: {}",part_2(&objects));
}

fn part_1(n: &str, depth: usize, objects: &HashMap<&str, Vec<&str>>) -> usize {
    match objects.get(n) {
        Some(v) => (depth * v.len()) + v.iter().fold(0, |acc, x| acc + part_1(x, depth+1, &objects)),
        _ => 0
    }
}

fn part_2(objects: &HashMap<&str, Vec<&str>>) -> usize {
    let mut distances: HashMap<&str, usize> = HashMap::new();
    let mut queue: Vec<(&str, usize)> = Vec::new();
    queue.push(("YOU", 0));

    while queue.len() > 0 {
        let c = queue.remove(0);
        if !distances.contains_key(c.0) {
            distances.insert(c.0, c.1);
            match objects.get(c.0) {
                Some(v) => {
                    for x in v {
                        queue.push((x, c.1+1));
                    }
                }
                _ => ()
            };
        }
    }

    match distances.get("SAN") {
        Some(v) => *v - 2,
        _ => 0
    }
}
