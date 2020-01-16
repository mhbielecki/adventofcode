use std::fs;
use intcodecomputer::IntCodeInterpreter;

fn main() {
    let filename = "input/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    let int_code_program: Vec<i64> = contents
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut interpreter = IntCodeInterpreter::new(int_code_program);
    interpreter.run();
}