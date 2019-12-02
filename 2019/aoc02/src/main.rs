use std::fs;

fn main() {

    let filename = "input/input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file.");

    let int_code_program: Vec<usize> = contents
        .split(",")
        .map(|x| x.parse::<usize>().unwrap()).collect();

    let part_1 = execute_program(int_code_program.clone(), 12, 2);
    println!("Part 1: {}", part_1);

    // Part 2
    for n in 0..100 {
        for v in 0..100 {
            let res = execute_program(int_code_program.clone(), n, v);
            if res == 19690720 {
                println!("Part 2: {}", 100 * n + v);
            }
        }
    }
}

fn execute_program(mut int_code_program: Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut idx = 0;
    int_code_program[1] = noun;
    int_code_program[2] = verb;
    loop {
        if int_code_program[idx] == 99 {
            break;
        }
        let val_a = int_code_program[int_code_program[idx+1]];
        let val_b = int_code_program[int_code_program[idx+2]];
        let store_pos = int_code_program[idx+3];

        if int_code_program[idx] == 1 {
             int_code_program[store_pos] = val_a + val_b;
        }
        else if int_code_program[idx] == 2 {
             int_code_program[store_pos] = val_a * val_b;
        }
        else {
            eprintln!("Unknown op-code: {}", int_code_program[idx]);
            break;
        }
        idx = idx + 4;
    }
    int_code_program[0]
}