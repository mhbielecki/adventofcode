use std::fs;

fn main() {
    let filename = "input/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    let lines: Vec<&str> = contents
        .split_whitespace()
        .collect();

    //https://stackoverflow.com/questions/328107/how-can-you-determine-a-point-is-between-two-other-points-on-a-line-segment

    //let asteroid_map: [[]] array of bools
    //let asteroids List of tuples(x,y) which have asteroid in it

    // let angles = angles which have an asteroid

    //let asteroid_map = [bool; contents.len()[]];

    let asteroid_map: Vec<Vec<bool>> = lines.iter().map(|line| line.chars().map(|c| c == '#').collect()).collect();
    let mut asteroids: Vec<(i32, i32)> = Vec::new();

    let mut dx = 0;
    for x in asteroid_map {
        let mut dy = 0;
        for y in x {
            if y {
                asteroids.push((dx, dy));
            }
            dy += 1;
        }
        dx += 1;
    }

    for a in asteroids {
        println!("{:?}", a);
    }
}
