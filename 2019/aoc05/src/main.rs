use std::fs;
use std::io;

fn main() {
    let filename = "input/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    let int_code_program: Vec<i32> = contents
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut interpreter = IntCodeInterpreter::new(int_code_program);
    interpreter.run();
}

const ADD: i32 = 1;
const MUL: i32 = 2;
const INPUT: i32 = 3;
const OUTPUT: i32 = 4;
const HALT: i32 = 99;

struct IntCodeInterpreter {
    instruction_pointer: usize,
    memory: Vec<i32>,
}

impl IntCodeInterpreter {
    pub fn new(input_program: Vec<i32>) -> IntCodeInterpreter {
        IntCodeInterpreter {
            instruction_pointer: 0,
            memory: input_program,
        }
    }

    fn add(&mut self) {
        let operands = self.get_operands();
        let store_pos = self.memory[self.instruction_pointer + 3] as usize;
        self.memory[store_pos] = operands.0 + operands.1;
        self.step(4);
    }

    fn mul(&mut self) {
        let operands = self.get_operands();
        let store_pos = self.memory[self.instruction_pointer + 3] as usize;
        self.memory[store_pos] = operands.0 * operands.1;
        self.step(4);
    }

    fn input(&mut self) {
        println!("input:");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();
        let store_pos = self.memory[self.instruction_pointer + 1] as usize;
        self.memory[store_pos] = n;
        self.step(2);
    }

    fn output(&mut self) {
        let param = self.get_parameter(1);
        if param == 0 {
            println!("{}", self.read_positional_value(1));
        } else {
            println!("{}", self.read_immediate_value(1));
        }
        self.step(2);
    }

    fn get_parameter(&self, param_number: i32) -> i32 {
        (self.memory[self.instruction_pointer] / (10_i32.pow((param_number+1) as u32))) % 10
    }
     
    fn read_positional_value(&self, offset: usize) -> i32 {
        self.memory[self.memory[self.instruction_pointer + offset] as usize]
    }

    fn read_immediate_value(&self, offset: usize) -> i32 {
        self.memory[self.instruction_pointer + offset]
    }

    fn step(&mut self, steps: usize) {
        self.instruction_pointer = self.instruction_pointer + steps;
    }

    fn get_operands(&self) -> (i32, i32) {
        let param_1 = self.get_parameter(1);
        let param_2 = self.get_parameter(2);
        let val_a = if param_1 == 0 { self.read_positional_value(1)} else {self.read_immediate_value(1)};
        let val_b = if param_2 == 0 { self.read_positional_value(2)} else {self.read_immediate_value(2)};
        (val_a, val_b)
    }

    fn run(&mut self) {
        loop {
            let opcode = self.memory[self.instruction_pointer] % 100;

            match opcode {
                ADD => self.add(),
                MUL => self.mul(),
                INPUT => self.input(),
                OUTPUT => self.output(),
                HALT => break,
                _ => {
                    eprintln!("Unknown op-code: {}", self.memory[self.instruction_pointer]);
                    break;
                }
            }
        }
    }
}
