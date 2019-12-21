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
const JT: i32 = 5;
const JF: i32 = 6;
const LT: i32 = 7;
const EQ: i32 = 8;
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

    fn jump_true(&mut self) {
        let first_param = self.get_operand(1);
        if first_param != 0 {
            self.instruction_pointer = self.get_operand(2) as usize;
        } else {
            self.step(3);
        }
    }

    fn jump_false(&mut self) {
        let first_param = self.get_operand(1);
        if first_param == 0 {
            self.instruction_pointer = self.get_operand(2) as usize;
        } else {
            self.step(3);
        }
    }

    fn less_than(&mut self) {
        let operands = self.get_operands();
        let store_pos = self.memory[self.instruction_pointer + 3] as usize;
        if operands.0 < operands.1 {
            self.memory[store_pos] = 1;
        } else {
            self.memory[store_pos] = 0;
        }
        self.step(4);
    }

    fn equal(&mut self) {
        let operands = self.get_operands();
        let store_pos = self.memory[self.instruction_pointer + 3] as usize;
        if operands.0 == operands.1 {
            self.memory[store_pos] = 1;
        }  else {
            self.memory[store_pos] = 0;
        }
        self.step(4);
    }

    fn get_parameter(&self, param_number: usize) -> i32 {
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

    fn get_operand(&self, position: usize) -> i32 {
        if self.get_parameter(position) == 0 {
             self.read_positional_value(position)}
        else {
            self.read_immediate_value(position)
        }
    }

    fn get_operands(&self) -> (i32, i32) {
        let val_a = self.get_operand(1);
        let val_b = self.get_operand(2);
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
                JT => self.jump_true(),
                JF => self.jump_false(),
                LT => self.less_than(),
                EQ => self.equal(),
                HALT => break,
                _ => {
                    panic!("Unknown op-code: {}", self.memory[self.instruction_pointer]);
                }
            }
        }
    }
}
