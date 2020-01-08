use std::fs;
use intcodecomputer::IntCodeInterpreter;

fn main() {
    let filename = "input/input.txt";
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file.");

    let int_code_program: Vec<i64> = contents
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let p = permutations(5).collect::<Vec<_>>();
    for i in p {
        let mut amp_1 = IntCodeInterpreter::new(int_code_program.clone());
        let mut amp_2 = IntCodeInterpreter::new(int_code_program.clone());
        let mut amp_3 = IntCodeInterpreter::new(int_code_program.clone());
        let mut amp_4 = IntCodeInterpreter::new(int_code_program.clone());
        let mut amp_5 = IntCodeInterpreter::new(int_code_program.clone());
        amp_1.add_custom_input(i[0] as i64);
        amp_1.add_custom_input(0);
        amp_1.run();

        amp_2.add_custom_input(i[1] as i64);
        amp_2.add_custom_input(amp_1.get_last_output());
        amp_2.run();

        amp_3.add_custom_input(i[2] as i64);
        amp_3.add_custom_input(amp_2.get_last_output());
        amp_3.run();

        amp_4.add_custom_input(i[3] as i64);
        amp_4.add_custom_input(amp_3.get_last_output());
        amp_4.run();

        amp_5.add_custom_input(i[4] as i64);
        amp_5.add_custom_input(amp_4.get_last_output());
        amp_5.run();
        println!("{}", amp_5.get_last_output());
    }
}

/* Generating permutations copied from https://rosettacode.org/wiki/Permutations#Rust */

pub fn permutations(size: usize) -> Permutations {
    Permutations { idxs: (0..size).collect(), swaps: vec![0; size], i: 0 }
}
 
pub struct Permutations {
    idxs: Vec<usize>,
    swaps: Vec<usize>,
    i: usize,
}
 
impl Iterator for Permutations {
    type Item = Vec<usize>;
 
    fn next(&mut self) -> Option<Self::Item> {
        if self.i > 0 {
            loop {
                if self.i >= self.swaps.len() { return None; }
                if self.swaps[self.i] < self.i { break; }
                self.swaps[self.i] = 0;
                self.i += 1;
            }
            self.idxs.swap(self.i, (self.i & 1) * self.swaps[self.i]);
            self.swaps[self.i] += 1;
        }
        self.i = 1;
        Some(self.idxs.clone())
    }
}
