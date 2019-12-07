fn main() {
    let result = Computer::new(PUZZLE_INPUT.to_vec(), 1).run();
    println!("1. outputs: {:?}", result.1);

    let result = Computer::new(PUZZLE_INPUT.to_vec(), 5).run();
    println!("2. outputs: {:?}", result.1);
}

struct Computer {
    instructions: Vec<i64>,
    input: i64,
    index: usize,
}

impl Computer {
    pub fn new(instructions: Vec<i64>, input: i64) -> Self {
        Computer {
            instructions,
            input,
            index: 0,
        }
    }

    pub fn run(mut self) -> (Vec<i64>, Vec<i64>) {
        let mut output = vec![];
        loop {
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
                    self.set(op.c, 1, self.input);
                    self.index += 2;
                }
                4 => {
                    let first = self.get(op.c, 1);
                    output.push(first);
                    self.index += 2;
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
                99 => break,
                _ => panic!("Something went wrong!"),
            }
        }
        (self.instructions, output)
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
    fn computer_one(instructions: Vec<i64>) -> (Vec<i64>, Vec<i64>) {
        computer_n(instructions, 1)
    }

    fn computer_n(instructions: Vec<i64>, input: i64) -> (Vec<i64>, Vec<i64>) {
        Computer::new(instructions, input).run()
    }
    fn get_value(instructions: Vec<i64>, mode: i64, value: i64) -> i64 {
        Computer::new(instructions, 0).get(mode, value)
    }

    fn stringify(program: Vec<i64>) -> String {
        program
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    eq_tests! {
        test_computer_one_1: stringify(computer_one(vec![1, 0, 0, 0, 99]).0) => "2,0,0,0,99";
        test_computer_one_2: stringify(computer_one(vec![2, 3, 0, 3, 99]).0) => "2,3,0,6,99";
        test_computer_one_3: stringify(computer_one(vec![2, 4, 4, 5, 99, 0]).0) => "2,4,4,5,99,9801";
        test_computer_one_4: stringify(computer_one(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).0) => "30,1,1,4,2,5,6,0,99";

        test_opcodes_1: to_opcode(1002) => Opcode {a: 0, b: 1, c: 0, de: 2};

        test_get_value_01: get_value(vec![0], 0, 0) => 0;
        test_get_value_02: get_value(vec![1, 2], 0, 0) => 2;
        test_get_value_03: get_value(vec![1, 0], 0, 1) => 1;
        test_get_value_04c: get_value(vec![1, 0, 0, 0, 99], 0, 1) => 1;
        test_get_value_04b: get_value(vec![1, 0, 0, 0, 99], 0, 2) => 1;

        test_get_value_11: get_value(vec![1], 1, 0) => 1;
        test_get_value_12: get_value(vec![1, 2], 1, 1) => 2;
        test_get_value_13: get_value(vec![1, 2, 3], 1, 2) => 3;

        test_computer_n_01: computer_n(vec![3,9,8,9,10,9,4,9,99,-1,8], 8).1[0] => 1;
        test_computer_n_02: computer_n(vec![3,9,8,9,10,9,4,9,99,-1,8], 7).1[0] => 0;
    }
}

const PUZZLE_INPUT: [i64; 678] = [
    3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1101, 86, 8, 225, 1101, 82, 69, 225, 101, 36,
    65, 224, 1001, 224, -106, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 5, 224, 1, 223, 224, 223,
    102, 52, 148, 224, 101, -1144, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 1, 224, 224, 1, 224,
    223, 223, 1102, 70, 45, 225, 1002, 143, 48, 224, 1001, 224, -1344, 224, 4, 224, 102, 8, 223,
    223, 101, 7, 224, 224, 1, 223, 224, 223, 1101, 69, 75, 225, 1001, 18, 85, 224, 1001, 224, -154,
    224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 224, 223, 223, 1101, 15, 59, 225, 1102, 67,
    42, 224, 101, -2814, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 3, 224, 224, 1, 223, 224, 223,
    1101, 28, 63, 225, 1101, 45, 22, 225, 1101, 90, 16, 225, 2, 152, 92, 224, 1001, 224, -1200,
    224, 4, 224, 102, 8, 223, 223, 101, 7, 224, 224, 1, 223, 224, 223, 1101, 45, 28, 224, 1001,
    224, -73, 224, 4, 224, 1002, 223, 8, 223, 101, 7, 224, 224, 1, 224, 223, 223, 1, 14, 118, 224,
    101, -67, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 2, 224, 1, 223, 224, 223, 4, 223, 99,
    0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999,
    1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999,
    1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225,
    1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225,
    1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 7, 677, 677, 224, 102, 2, 223, 223, 1005, 224, 329,
    1001, 223, 1, 223, 1008, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 344, 1001, 223, 1, 223,
    1107, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 359, 1001, 223, 1, 223, 107, 677, 677, 224,
    102, 2, 223, 223, 1005, 224, 374, 101, 1, 223, 223, 1108, 677, 226, 224, 102, 2, 223, 223,
    1005, 224, 389, 1001, 223, 1, 223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 404, 101,
    1, 223, 223, 1008, 677, 226, 224, 102, 2, 223, 223, 1005, 224, 419, 101, 1, 223, 223, 1108,
    226, 677, 224, 102, 2, 223, 223, 1006, 224, 434, 1001, 223, 1, 223, 8, 677, 226, 224, 1002,
    223, 2, 223, 1005, 224, 449, 101, 1, 223, 223, 1008, 677, 677, 224, 1002, 223, 2, 223, 1006,
    224, 464, 1001, 223, 1, 223, 1108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 479, 1001, 223,
    1, 223, 1007, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 494, 1001, 223, 1, 223, 1007, 226,
    226, 224, 102, 2, 223, 223, 1005, 224, 509, 101, 1, 223, 223, 107, 677, 226, 224, 1002, 223, 2,
    223, 1006, 224, 524, 1001, 223, 1, 223, 108, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 539,
    101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 554, 1001, 223, 1, 223, 1107,
    226, 677, 224, 102, 2, 223, 223, 1005, 224, 569, 101, 1, 223, 223, 108, 677, 226, 224, 1002,
    223, 2, 223, 1006, 224, 584, 101, 1, 223, 223, 108, 226, 226, 224, 102, 2, 223, 223, 1006, 224,
    599, 1001, 223, 1, 223, 1107, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 614, 1001, 223, 1,
    223, 8, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 629, 1001, 223, 1, 223, 107, 226, 226, 224,
    102, 2, 223, 223, 1005, 224, 644, 101, 1, 223, 223, 8, 226, 226, 224, 102, 2, 223, 223, 1006,
    224, 659, 101, 1, 223, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 674, 101, 1, 223,
    223, 4, 223, 99, 226,
];
