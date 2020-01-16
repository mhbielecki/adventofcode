use std::io;

const ADD: i32 = 1;
const MUL: i32 = 2;
const INPUT: i32 = 3;
const OUTPUT: i32 = 4;
const JT: i32 = 5;
const JF: i32 = 6;
const LT: i32 = 7;
const EQ: i32 = 8;
const ADJUST_REL_BASE: i32 = 9;
const HALT: i32 = 99;

pub struct IntCodeInterpreter {
    instruction_pointer: usize,
    memory: Vec<i64>,
    custom_input: Vec<i64>,
    output: Vec<i64>,
    relative_base: usize
}

impl IntCodeInterpreter {
    pub fn new(input_program: Vec<i64>) -> IntCodeInterpreter {
        let mut mem = input_program.clone();
        mem.append(&mut vec![0; 100000000]);
        IntCodeInterpreter {
            instruction_pointer: 0,
            memory: mem,
            custom_input: Vec::new(),
            output: Vec::new(),
            relative_base: 0
        }
    }

    fn add(&mut self) {
        let operands = self.get_operands();
        self.store(operands.0 + operands.1, 3);
        self.step(4);
    }

    fn mul(&mut self) {
        let operands = self.get_operands();
        self.store(operands.0 * operands.1, 3);
        self.step(4);
    }

    fn input(&mut self) {
        let mut n: i64;
        if self.custom_input.len() > 0 {
            n = self.custom_input.remove(0);
        } else {
            println!("input:");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            n = input.trim().parse().unwrap();
        }
        self.store(n, 1);
        self.step(2);
    }

    fn output(&mut self) {
        let param = self.get_parameter(1);

        if param == 0 {
            let v = self.read_positional_value(1);
            self.output.push(v);
            println!("{}", v);
        } else if param == 1 {
            let v = self.read_immediate_value(1);
            self.output.push(v);
            println!("{}", v);
        } else {
            let v = self.read_relative_value(1);
            self.output.push(v);
            println!("{}", v);
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
        if operands.0 < operands.1 {
            self.store(1, 3);
        } else {
            self.store(0, 3);
        }
        self.step(4);
    }

    fn equal(&mut self) {
        let operands = self.get_operands();
        if operands.0 == operands.1 {
            self.store(1, 3);
        }  else {
            self.store(0, 3);
        }
        self.step(4);
    }

    fn get_operand(&self, position: usize) -> i64 {
        match self.get_parameter(position) {
            0 => self.read_positional_value(position),
            1 => self.read_immediate_value(position),
            2 => self.read_relative_value(position),
            _ => panic!("Error: invalid position in get_operand")
        }
    }

    fn get_operands(&self) -> (i64, i64) {
        let val_a = self.get_operand(1);
        let val_b = self.get_operand(2);
        (val_a, val_b)
    }

    fn step(&mut self, steps: usize) {
        self.instruction_pointer = self.instruction_pointer + steps;
    }

    fn store(&mut self, value: i64, param_number: usize) {
        let param = self.get_parameter(param_number);
        let mut idx = 0;
        if param == 0 {
            idx = self.memory[self.instruction_pointer + param_number]

        } else if param == 2 {
            idx = self.read_immediate_value(param_number) + (self.relative_base as i64);
        } 
        else {
            panic!("Error: invalid parameter in input")
        }
        self.memory[idx as usize] = value;
    }

    fn get_parameter(&self, param_number: usize) -> i64 {
        (self.memory[self.instruction_pointer] / ((10_i64.pow((param_number+1) as u32))) % 10 as i64)
    }
     
    fn read_positional_value(&self, offset: usize) -> i64 {
        self.memory[self.memory[self.instruction_pointer + offset] as usize]
    }

    fn read_immediate_value(&self, offset: usize) -> i64 {
        self.memory[self.instruction_pointer + offset]
    }

    fn read_relative_value(&self, offset: usize) -> i64 {
        let f = self.read_immediate_value(offset) as i64;
        let index = (self.relative_base as i64) + f;
        self.memory[index as usize]
    }

    fn adjust_relative_base(&mut self) {
        let first_param = self.get_operand(1);
        self.relative_base = (self.relative_base as i64 + first_param) as usize;
        self.step(2);
    }

    pub fn add_custom_input(&mut self, input: i64) {
        self.custom_input.push(input);
    }

    pub fn get_last_output(&mut self) -> i64 {
        match self.output.pop() {
            Some(v) => v,
            None => -1
        }
    }

    pub fn run(&mut self) {
        loop {
            let opcode = self.memory[self.instruction_pointer] % 100;
            match opcode as i32 {
                ADD => self.add(),
                MUL => self.mul(),
                INPUT => self.input(),
                OUTPUT => self.output(),
                JT => self.jump_true(),
                JF => self.jump_false(),
                LT => self.less_than(),
                EQ => self.equal(),
                ADJUST_REL_BASE => self.adjust_relative_base(),
                HALT => break,
                _ => {
                    panic!("Unknown op-code: {}", self.memory[self.instruction_pointer]);
                }
            }
        }
    }
}