use std::fs;

fn main() {
    let filename = "input/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");

    let fuel_for_modules: Vec<i32> = contents
        .split_whitespace()
        .map(|x| calc_fuel(x.parse::<i32>().unwrap())).collect();

    let part_1: i32 = fuel_for_modules.iter().sum();

    println!("Part 1 - Total fuel for all the modules: {}", part_1);

    let part_2: i32 = fuel_for_modules.iter()
        .map(|x| x + additional_fuel(*x))
        .sum();

    println!("Part 2 - Total fuel for all the modules and fuel: {}", part_2);
}

fn calc_fuel(module: i32) -> i32 {
    return (module / 3) - 2;
}

fn additional_fuel(fuel: i32) -> i32 {
    let f = calc_fuel(fuel);
    if f <= 0 {
        return 0;
    }
    else {
        return f + additional_fuel(f);
    }
}