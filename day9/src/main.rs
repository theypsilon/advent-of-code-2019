use std::collections::{HashMap, VecDeque};

fn main() {
    let outputs = Computer::new(PUZZLE_INPUT.to_vec())
        .with_input(1)
        .run()
        .outputs;
    println!("1. outputs: {:?}", outputs);

    let outputs = Computer::new(PUZZLE_INPUT.to_vec())
        .with_input(2)
        .run()
        .outputs;
    println!("2. outputs: {:?}", outputs);
}

type Instructions = Vec<i64>;

struct Computer {
    instructions: Instructions,
    input: VecDeque<i64>,
    memory: HashMap<i64, i64>,
    ptr: i64,
    relative_base: i64,
}

enum ComputerExecution {
    Yield(i64),
    Halt,
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
            memory: HashMap::new(),
            input: VecDeque::new(),
            ptr: 0,
            relative_base: 0,
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
        while let ComputerExecution::Yield(output) = self.next_output() {
            outputs.push(output);
        }
        ComputerResult {
            #[cfg(test)]
            instructions: self.instructions,
            outputs,
        }
    }

    pub fn next_output(&mut self) -> ComputerExecution {
        loop {
            let op = self.opcode();
            match op.de {
                1 => {
                    let first = *self.access(op.c, 1);
                    let second = *self.access(op.b, 2);
                    *self.access(op.a, 3) = first + second;
                    self.ptr += 4;
                }
                2 => {
                    let first = *self.access(op.c, 1);
                    let second = *self.access(op.b, 2);
                    *self.access(op.a, 3) = first * second;
                    self.ptr += 4;
                }
                3 => {
                    if let Some(input) = self.input.pop_front() {
                        *self.access(op.c, 1) = input;
                    } else {
                        panic!("Missing input!");
                    }
                    self.ptr += 2;
                }
                4 => {
                    let first = *self.access(op.c, 1);
                    self.ptr += 2;
                    return ComputerExecution::Yield(first);
                }
                5 => {
                    let first = *self.access(op.c, 1);
                    let second = *self.access(op.b, 2);
                    if first != 0 {
                        if second < 0 {
                            panic!("Second cant be 0 here: {}", second);
                        }
                        self.ptr = second;
                    } else {
                        self.ptr += 3;
                    }
                }
                6 => {
                    let first = *self.access(op.c, 1);
                    let second = *self.access(op.b, 2);
                    if first == 0 {
                        if second < 0 {
                            panic!("Second can't be less than 0 here: {}", second);
                        }
                        self.ptr = second;
                    } else {
                        self.ptr += 3;
                    }
                }
                7 => {
                    let first = *self.access(op.c, 1);
                    let second = *self.access(op.b, 2);
                    *self.access(op.a, 3) = if first < second { 1 } else { 0 };
                    self.ptr += 4;
                }
                8 => {
                    let first = *self.access(op.c, 1);
                    let second = *self.access(op.b, 2);
                    *self.access(op.a, 3) = if first == second { 1 } else { 0 };
                    self.ptr += 4;
                }
                9 => {
                    let first = *self.access(op.c, 1);
                    self.relative_base += first;
                    self.ptr += 2;
                }
                99 => return ComputerExecution::Halt,
                _ => panic!("Something went wrong!"),
            }
        }
    }

    pub fn access(&mut self, mode: i64, offset: i64) -> &mut i64 {
        match mode {
            0 => {
                let ptr = *self.instruction(self.ptr + offset);
                self.instruction(ptr)
            }
            1 => self.instruction(self.ptr + offset),
            2 => {
                let ptr = *self.instruction(self.ptr + offset);
                self.instruction(ptr + self.relative_base)
            }
            _ => panic!("Mode not implemented"),
        }
    }

    pub fn instruction(&mut self, position: i64) -> &mut i64 {
        if position > self.instructions.len() as i64 {
            if !self.memory.contains_key(&position) {
                self.memory.insert(position, 0);
            }
            self.memory
                .get_mut(&position)
                .expect("Memory should be initialized.")
        } else {
            &mut self.instructions[position as usize]
        }
    }

    fn opcode(&mut self) -> Opcode {
        to_opcode(*self.instruction(self.ptr))
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
        *Computer::new(instructions)
            .with_input(0)
            .access(mode, value)
    }

    fn stringify(program: Instructions) -> String {
        program
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    fn count_digits(mut n: i64) -> i64 {
        let mut counter = 0;
        while n > 0 {
            n /= 10;
            counter += 1;
        }
        counter
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

        op_203_base_0_input_42_eq_42: computer_n(vec![203, 1, 4, 1, 99], 42).outputs[0] => 42;
        op_203_base_0_eq_op_3: computer_n(vec![203, 1, 4, 1, 99], 42).outputs[0] => computer_n(vec![3, 1, 4, 1, 99], 42).outputs[0];

        relative_mode_example_1: stringify(computer_n(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], 42).instructions) => "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        relative_mode_example_2: count_digits(computer_n(vec![1102,34915192,34915192,7,4,7,99,0], 42).outputs[0]) => 16;
        relative_mode_example_3: computer_n(vec![104,1125899906842624,99], 42).outputs[0] => 1125899906842624;

        op_203_base_1000_input_42_eq_42: computer_n(vec![109, 1000, 203, 5, 4, 1005, 99], 42).outputs[0] => 42;
    }
}

const PUZZLE_INPUT: [i64; 973] = [
    1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1102, 3, 1, 1000, 109, 988,
    209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008, 1000, 2, 63,
    1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104, 0, 99, 4, 17,
    104, 0, 99, 0, 0, 1102, 33, 1, 1011, 1102, 1, 26, 1010, 1101, 0, 594, 1029, 1101, 0, 20, 1018,
    1102, 38, 1, 1000, 1102, 35, 1, 1001, 1101, 800, 0, 1023, 1101, 0, 599, 1028, 1101, 0, 34,
    1013, 1101, 0, 737, 1026, 1102, 21, 1, 1005, 1102, 1, 0, 1020, 1102, 1, 195, 1024, 1101, 31, 0,
    1016, 1101, 0, 1, 1021, 1102, 22, 1, 1004, 1102, 1, 32, 1014, 1102, 37, 1, 1019, 1102, 36, 1,
    1002, 1101, 23, 0, 1003, 1102, 190, 1, 1025, 1101, 28, 0, 1009, 1101, 807, 0, 1022, 1102, 30,
    1, 1015, 1101, 0, 27, 1017, 1102, 1, 25, 1012, 1102, 1, 39, 1008, 1101, 0, 29, 1007, 1101, 734,
    0, 1027, 1101, 0, 24, 1006, 109, 28, 2105, 1, -4, 4, 187, 1105, 1, 199, 1001, 64, 1, 64, 1002,
    64, 2, 64, 109, -19, 1208, -9, 37, 63, 1005, 63, 219, 1001, 64, 1, 64, 1106, 0, 221, 4, 205,
    1002, 64, 2, 64, 109, 20, 1206, -8, 233, 1106, 0, 239, 4, 227, 1001, 64, 1, 64, 1002, 64, 2,
    64, 109, -29, 2101, 0, 4, 63, 1008, 63, 21, 63, 1005, 63, 259, 1106, 0, 265, 4, 245, 1001, 64,
    1, 64, 1002, 64, 2, 64, 109, -2, 2107, 37, 4, 63, 1005, 63, 285, 1001, 64, 1, 64, 1106, 0, 287,
    4, 271, 1002, 64, 2, 64, 109, 14, 1206, 8, 301, 4, 293, 1105, 1, 305, 1001, 64, 1, 64, 1002,
    64, 2, 64, 109, 11, 21101, 40, 0, -6, 1008, 1017, 40, 63, 1005, 63, 331, 4, 311, 1001, 64, 1,
    64, 1105, 1, 331, 1002, 64, 2, 64, 109, -21, 1208, 1, 23, 63, 1005, 63, 353, 4, 337, 1001, 64,
    1, 64, 1106, 0, 353, 1002, 64, 2, 64, 109, 26, 1205, -7, 371, 4, 359, 1001, 64, 1, 64, 1106, 0,
    371, 1002, 64, 2, 64, 109, -15, 21102, 41, 1, 2, 1008, 1015, 40, 63, 1005, 63, 395, 1001, 64,
    1, 64, 1106, 0, 397, 4, 377, 1002, 64, 2, 64, 109, -3, 2108, 22, -6, 63, 1005, 63, 415, 4, 403,
    1105, 1, 419, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -6, 1201, -4, 0, 63, 1008, 63, 35, 63,
    1005, 63, 439, 1106, 0, 445, 4, 425, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 14, 21102, 42, 1,
    -4, 1008, 1014, 42, 63, 1005, 63, 467, 4, 451, 1105, 1, 471, 1001, 64, 1, 64, 1002, 64, 2, 64,
    109, -23, 1201, 10, 0, 63, 1008, 63, 21, 63, 1005, 63, 497, 4, 477, 1001, 64, 1, 64, 1105, 1,
    497, 1002, 64, 2, 64, 109, 16, 21101, 43, 0, 2, 1008, 1013, 42, 63, 1005, 63, 521, 1001, 64, 1,
    64, 1105, 1, 523, 4, 503, 1002, 64, 2, 64, 109, 3, 21107, 44, 45, 1, 1005, 1015, 541, 4, 529,
    1105, 1, 545, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -2, 1205, 8, 561, 1001, 64, 1, 64, 1106,
    0, 563, 4, 551, 1002, 64, 2, 64, 109, -7, 1207, 2, 28, 63, 1005, 63, 579, 1106, 0, 585, 4, 569,
    1001, 64, 1, 64, 1002, 64, 2, 64, 109, 24, 2106, 0, -1, 4, 591, 1106, 0, 603, 1001, 64, 1, 64,
    1002, 64, 2, 64, 109, -4, 21108, 45, 45, -9, 1005, 1016, 625, 4, 609, 1001, 64, 1, 64, 1105, 1,
    625, 1002, 64, 2, 64, 109, -24, 2101, 0, 0, 63, 1008, 63, 35, 63, 1005, 63, 651, 4, 631, 1001,
    64, 1, 64, 1106, 0, 651, 1002, 64, 2, 64, 109, 10, 1202, -7, 1, 63, 1008, 63, 24, 63, 1005, 63,
    675, 1001, 64, 1, 64, 1105, 1, 677, 4, 657, 1002, 64, 2, 64, 109, -2, 2102, 1, -1, 63, 1008,
    63, 41, 63, 1005, 63, 697, 1105, 1, 703, 4, 683, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -2,
    21108, 46, 45, 3, 1005, 1010, 723, 1001, 64, 1, 64, 1105, 1, 725, 4, 709, 1002, 64, 2, 64, 109,
    28, 2106, 0, -8, 1106, 0, 743, 4, 731, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -37, 2102, 1, 3,
    63, 1008, 63, 35, 63, 1005, 63, 769, 4, 749, 1001, 64, 1, 64, 1105, 1, 769, 1002, 64, 2, 64,
    109, 26, 21107, 47, 46, -8, 1005, 1016, 789, 1001, 64, 1, 64, 1106, 0, 791, 4, 775, 1002, 64,
    2, 64, 109, 7, 2105, 1, -8, 1001, 64, 1, 64, 1106, 0, 809, 4, 797, 1002, 64, 2, 64, 109, -37,
    1202, 7, 1, 63, 1008, 63, 35, 63, 1005, 63, 831, 4, 815, 1105, 1, 835, 1001, 64, 1, 64, 1002,
    64, 2, 64, 109, 18, 1207, -5, 30, 63, 1005, 63, 853, 4, 841, 1106, 0, 857, 1001, 64, 1, 64,
    1002, 64, 2, 64, 109, -7, 2108, 37, -5, 63, 1005, 63, 873, 1105, 1, 879, 4, 863, 1001, 64, 1,
    64, 1002, 64, 2, 64, 109, -7, 2107, 23, 8, 63, 1005, 63, 897, 4, 885, 1106, 0, 901, 1001, 64,
    1, 64, 4, 64, 99, 21101, 27, 0, 1, 21102, 1, 915, 0, 1106, 0, 922, 21201, 1, 12374, 1, 204, 1,
    99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21101, 942, 0, 0, 1105, 1, 922,
    22102, 1, 1, -1, 21201, -2, -3, 1, 21102, 957, 1, 0, 1105, 1, 922, 22201, 1, -1, -2, 1106, 0,
    968, 21201, -2, 0, -2, 109, -3, 2106, 0, 0,
];
