use std::collections::VecDeque;

fn main() {
    let max_thruster = permutations(vec![0, 1, 2, 3, 4])
        .into_iter()
        .map(|phase| amplifiers_part1(PUZZLE_INPUT.to_vec(), phase))
        .max();
    println!("1. max_thruster: {:?}", max_thruster);

    let max_thruster = permutations(vec![5, 6, 7, 8, 9])
        .into_iter()
        .map(|phase| amplifiers_part2(PUZZLE_INPUT.to_vec(), phase))
        .max();
    println!("2. max_thruster: {:?}", max_thruster);
}

fn amplifiers_part1(instructions: Instructions, phase: PhaseSetting) -> i64 {
    phase.into_iter().fold(0, |output, digit| {
        Computer::new(instructions.clone())
            .with_input(digit)
            .with_input(output)
            .run()
            .outputs[0]
    })
}

fn amplifiers_part2(instructions: Instructions, phase: PhaseSetting) -> i64 {
    const LAST_COMPUTER: usize = 4;
    let mut computers: Vec<Computer> = (0..=LAST_COMPUTER)
        .map(|i| Computer::new(instructions.clone()).with_input(phase[i]))
        .collect();

    let mut result = -1;
    let mut output = 0;
    'feedback: loop {
        for (i, computer) in computers.iter_mut().enumerate() {
            computer.add_input(output);
            match computer.yield_output() {
                ComputerState::Paused(value) => output = value,
                ComputerState::Halted => {
                    if i == LAST_COMPUTER {
                        break 'feedback;
                    }
                }
            }
        }
        result = output;
    }
    result
}

fn permutations(list: Vec<i64>) -> Vec<Vec<i64>> {
    match list.len() {
        0 => vec![],
        1 => vec![list],
        _ => {
            let mut result = vec![];
            for i in 0..list.len() {
                let current = list[i];

                let mut others = list[..i].to_vec();
                others.extend(list[(i + 1)..].iter());

                for sub_list in permutations(others) {
                    let mut current_list = vec![current];
                    current_list.extend(sub_list);
                    result.push(current_list);
                }
            }
            result
        }
    }
}

type PhaseSetting = Vec<i64>;
type Instructions = Vec<i64>;

struct Computer {
    instructions: Instructions,
    input: VecDeque<i64>,
    index: usize,
    finalized: bool,
}

enum ComputerState {
    Paused(i64),
    Halted,
}

struct ComputerResult {
    #[cfg(test)]
    instructions: Instructions,
    outputs: Vec<i64>,
}

impl Computer {
    pub fn new(instructions: Instructions) -> Self {
        Computer {
            instructions,
            input: VecDeque::new(),
            index: 0,
            finalized: false,
        }
    }

    pub fn with_input(mut self, input: i64) -> Self {
        self.add_input(input);
        self
    }

    pub fn add_input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    pub fn run(mut self) -> ComputerResult {
        let mut outputs = vec![];
        while let ComputerState::Paused(value) = self.yield_output() {
            outputs.push(value);
        }
        ComputerResult {
            #[cfg(test)]
            instructions: self.instructions,
            outputs,
        }
    }

    pub fn yield_output(&mut self) -> ComputerState {
        loop {
            if self.finalized {
                return ComputerState::Halted;
            }
            let op = self.opcode();
            match op.de {
                1 => {
                    let first = self.get(op.c, 1);
                    let second = self.get(op.b, 2);
                    self.set(op.a, 3, first + second);
                    self.index += 4;
                }
                2 => {
                    let first = self.get(op.c, 1);
                    let second = self.get(op.b, 2);
                    self.set(op.a, 3, first * second);
                    self.index += 4;
                }
                3 => {
                    if let Some(input) = self.input.pop_front() {
                        self.set(op.c, 1, input);
                    } else {
                        panic!("Missing input!");
                    }
                    self.index += 2;
                }
                4 => {
                    let first = self.get(op.c, 1);
                    self.index += 2;
                    return ComputerState::Paused(first);
                }
                5 => {
                    let first = self.get(op.c, 1);
                    let second = self.get(op.b, 2);
                    if first != 0 {
                        if second < 0 {
                            panic!("Second cant be 0 here: {}", second);
                        }
                        self.index = second as usize;
                    } else {
                        self.index += 3;
                    }
                }
                6 => {
                    let first = self.get(op.c, 1);
                    let second = self.get(op.b, 2);
                    if first == 0 {
                        if second < 0 {
                            panic!("Second can't be less than 0 here: {}", second);
                        }
                        self.index = second as usize;
                    } else {
                        self.index += 3;
                    }
                }
                7 => {
                    let first = self.get(op.c, 1);
                    let second = self.get(op.b, 2);
                    self.set(op.a, 3, if first < second { 1 } else { 0 });
                    self.index += 4;
                }
                8 => {
                    let first = self.get(op.c, 1);
                    let second = self.get(op.b, 2);
                    self.set(op.a, 3, if first == second { 1 } else { 0 });
                    self.index += 4;
                }
                99 => self.finalized = true,
                _ => panic!("Something went wrong!"),
            }
        }
    }

    pub fn get(&self, mode: i64, offset: i64) -> i64 {
        match mode {
            0 => self.instructions[self.instructions[self.index + offset as usize] as usize],
            1 => self.instructions[self.index + offset as usize],
            _ => panic!("Mode not implemented"),
        }
    }

    pub fn set(&mut self, mode: i64, offset: i64, value: i64) {
        match mode {
            0 => {
                let pointer = self.instructions[self.index + offset as usize] as usize;
                self.instructions[pointer] = value
            }
            1 => self.instructions[self.index + offset as usize] = value,
            _ => panic!("Mode not implemented"),
        }
    }

    fn opcode(&self) -> Opcode {
        to_opcode(self.instructions[self.index])
    }
}

#[derive(Debug, PartialEq)]
struct Opcode {
    a: i64,
    b: i64,
    c: i64,
    de: i64,
}

fn to_opcode(mut n: i64) -> Opcode {
    let de = n % 100;
    n /= 100;
    let c = n % 10;
    n /= 10;
    let b = n % 10;
    n /= 10;
    let a = n % 10;
    Opcode { a, b, c, de }
}

#[cfg(test)]
mod test {

    use super::*;

    macro_rules! eq_tests {
        ( $( $name:ident: $input:expr => $expected:expr;)* ) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!($input, $expected);
                }
            )*
        };
    }
    fn computer_one(instructions: Instructions) -> ComputerResult {
        computer_n(instructions, 1)
    }

    fn computer_n(instructions: Instructions, input: i64) -> ComputerResult {
        Computer::new(instructions).with_input(input).run()
    }
    fn get_value(instructions: Instructions, mode: i64, value: i64) -> i64 {
        Computer::new(instructions).with_input(0).get(mode, value)
    }

    fn stringify(program: Instructions) -> String {
        program
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    eq_tests! {
        computer_one_1: stringify(computer_one(vec![1, 0, 0, 0, 99]).instructions) => "2,0,0,0,99";
        computer_one_2: stringify(computer_one(vec![2, 3, 0, 3, 99]).instructions) => "2,3,0,6,99";
        computer_one_3: stringify(computer_one(vec![2, 4, 4, 5, 99, 0]).instructions) => "2,4,4,5,99,9801";
        computer_one_4: stringify(computer_one(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).instructions) => "30,1,1,4,2,5,6,0,99";

        opcodes_1: to_opcode(1002) => Opcode {a: 0, b: 1, c: 0, de: 2};

        get_value_01: get_value(vec![0], 0, 0) => 0;
        get_value_02: get_value(vec![1, 2], 0, 0) => 2;
        get_value_03: get_value(vec![1, 0], 0, 1) => 1;
        get_value_04c: get_value(vec![1, 0, 0, 0, 99], 0, 1) => 1;
        get_value_04b: get_value(vec![1, 0, 0, 0, 99], 0, 2) => 1;

        get_value_11: get_value(vec![1], 1, 0) => 1;
        get_value_12: get_value(vec![1, 2], 1, 1) => 2;
        get_value_13: get_value(vec![1, 2, 3], 1, 2) => 3;

        computer_n_01: computer_n(vec![3,9,8,9,10,9,4,9,99,-1,8], 8).outputs[0] => 1;
        computer_n_02: computer_n(vec![3,9,8,9,10,9,4,9,99,-1,8], 7).outputs[0] => 0;

        amplifiers_part1_example_1: amplifiers_part1(vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0], vec![4,3,2,1,0]) => 43210;
        amplifiers_part1_example_2: amplifiers_part1(vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
            101,5,23,23,1,24,23,23,4,23,99,0,0], vec![0, 1, 2, 3, 4]) => 54321;
        amplifiers_part1_example_3: amplifiers_part1(vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0], vec![1,0,4,3,2]) => 65210;

        test_permutations: permutations(vec![0, 1, 2]) => vec![vec![0, 1, 2], vec![0, 2, 1], vec![1, 0, 2], vec![1, 2, 0], vec![2, 0, 1], vec![2, 1, 0]];

        amplifiers_part2_example_4: amplifiers_part2(vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
            27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5], vec![9,8,7,6,5]) => 139629729;
        amplifiers_part2_example_5: amplifiers_part2(vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10], vec![9,7,8,5,6]) => 18216;
    }
}

const PUZZLE_INPUT: [i64; 523] = [
    3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 46, 63, 76, 97, 118, 199, 280, 361, 442, 99999, 3, 9,
    102, 4, 9, 9, 101, 2, 9, 9, 1002, 9, 5, 9, 101, 4, 9, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 101, 5,
    9, 9, 102, 3, 9, 9, 101, 3, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 102, 3, 9, 9, 4, 9, 99, 3, 9,
    1002, 9, 5, 9, 101, 4, 9, 9, 1002, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 5, 9, 101,
    3, 9, 9, 1002, 9, 5, 9, 1001, 9, 5, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9,
    4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9,
    1001, 9, 1, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9,
    4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
    1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9,
    4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9,
    102, 2, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9,
    4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102,
    2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9,
    99, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101,
    1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9,
    3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 101,
    1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9,
    3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2,
    9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99,
];
