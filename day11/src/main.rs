use std::collections::{HashMap, VecDeque};

fn main() {
    let (mut panels, borders) = run_robot(PUZZLE_INPUT.to_vec());
    println!("1. panels: {}", panels.len());
    println!("2. code:");
    let (min_x, min_y, max_x, max_y) = borders;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = (x, y);
            if !panels.contains_key(&pos) {
                panels.insert(pos, Panel::new(Color::Black));
            }
            print!(
                "{}",
                match panels[&pos].color {
                    Color::Black => ' ',
                    Color::White => '*',
                }
            );
        }
        println!();
    }
}

fn run_robot(instructions: Vec<i64>) -> (HashMap<(i64, i64), Panel>, (i64, i64, i64, i64)) {
    let mut computer = Computer::new(instructions);
    let mut robot = Robot {
        dir: Direction::Up,
        pos: (0, 0),
    };
    let mut panels: HashMap<(i64, i64), Panel> = HashMap::new();
    panels.insert((0, 0), Panel::new(Color::White));
    let mut min_x = 10000000;
    let mut min_y = 10000000;
    let mut max_x = -10000000;
    let mut max_y = -10000000;
    loop {
        let input = if !panels.contains_key(&robot.pos) {
            0
        } else {
            match panels[&robot.pos].color {
                Color::Black => 0,
                Color::White => 1,
            }
        };
        computer.add_input(input);
        if let Some((out1, out2)) = run_twice(&mut computer) {
            if !panels.contains_key(&robot.pos) {
                panels.insert(robot.pos, Panel::new(Color::Black));
            }
            let panel = panels.get_mut(&robot.pos).unwrap();
            panel.color = match out1 {
                0 => Color::Black,
                1 => Color::White,
                _ => panic!("Unexpected out1 {}", out1),
            };
            match out2 {
                0 => robot.turn_left(),
                1 => robot.turn_right(),
                _ => panic!("Unexpected out2 {}", out2),
            }
        } else {
            break;
        }
        robot.advance();
        if robot.pos.0 > max_x {
            max_x = robot.pos.0;
        }
        if robot.pos.1 > max_y {
            max_y = robot.pos.1;
        }
        if robot.pos.0 < min_x {
            min_x = robot.pos.0;
        }
        if robot.pos.1 < min_y {
            min_y = robot.pos.1;
        }
    }
    (panels, (min_x, min_y, max_x, max_y))
}

fn run_twice(computer: &mut Computer) -> Option<(i64, i64)> {
    if let ComputerExecution::Yield(output_1) = computer.next_output() {
        if let ComputerExecution::Yield(output_2) = computer.next_output() {
            return Some((output_1, output_2));
        }
    }
    None
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Robot {
    dir: Direction,
    pos: (i64, i64),
}

impl Robot {
    fn turn_left(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        };
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Left => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
        };
    }

    fn advance(&mut self) {
        match self.dir {
            Direction::Up => self.pos.1 -= 1,
            Direction::Left => self.pos.0 -= 1,
            Direction::Down => self.pos.1 += 1,
            Direction::Right => self.pos.0 += 1,
        };
    }
}

enum Color {
    Black,
    White,
}

struct Panel {
    color: Color,
}

impl Panel {
    pub fn new(color: Color) -> Self {
        Panel { color }
    }
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

    pub fn run(&mut self) -> Vec<i64> {
        let mut outputs = vec![];
        while let ComputerExecution::Yield(output) = self.next_output() {
            outputs.push(output);
        }
        outputs
    }

    pub fn next_output(&mut self) -> ComputerExecution {
        loop {
            let op = self.opcode();
            match op.de {
                1 => {
                    let first = self.get_first(op);
                    let second = self.get_second(op);
                    self.set_third(op, first + second);
                    self.ptr += 4;
                }
                2 => {
                    let first = self.get_first(op);
                    let second = self.get_second(op);
                    self.set_third(op, first * second);
                    self.ptr += 4;
                }
                3 => {
                    if let Some(input) = self.input.pop_front() {
                        self.set_first(op, input);
                    } else {
                        panic!("Missing input!");
                    }
                    self.ptr += 2;
                }
                4 => {
                    let first = self.get_first(op);
                    self.ptr += 2;
                    return ComputerExecution::Yield(first);
                }
                5 => {
                    let first = self.get_first(op);
                    let second = self.get_second(op);
                    if first != 0 {
                        if second < 0 {
                            panic!("Second can't be 0 here: {}", second);
                        }
                        self.ptr = second;
                    } else {
                        self.ptr += 3;
                    }
                }
                6 => {
                    let first = self.get_first(op);
                    let second = self.get_second(op);
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
                    let first = self.get_first(op);
                    let second = self.get_second(op);
                    self.set_third(op, if first < second { 1 } else { 0 });
                    self.ptr += 4;
                }
                8 => {
                    let first = self.get_first(op);
                    let second = self.get_second(op);
                    self.set_third(op, if first == second { 1 } else { 0 });
                    self.ptr += 4;
                }
                9 => {
                    let first = self.get_first(op);
                    self.relative_base += first;
                    self.ptr += 2;
                }
                99 => return ComputerExecution::Halt,
                _ => panic!("Opcode not implemented: {}", op.de),
            }
        }
    }

    pub fn get_first(&mut self, op: Opcode) -> i64 {
        *self.access(op.c, 1)
    }
    pub fn get_second(&mut self, op: Opcode) -> i64 {
        *self.access(op.b, 2)
    }
    pub fn set_third(&mut self, op: Opcode, value: i64) {
        *self.access(op.a, 3) = value;
    }
    pub fn set_first(&mut self, op: Opcode, value: i64) {
        *self.access(op.c, 1) = value;
    }

    pub fn access(&mut self, mode: u16, offset: i64) -> &mut i64 {
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
            _ => panic!("Mode not implemented: {}", mode),
        }
    }

    pub fn instruction(&mut self, position: i64) -> &mut i64 {
        if position >= self.instructions.len() as i64 {
            if !self.memory.contains_key(&position) {
                self.memory.insert(position, 0);
            }
            self.memory.get_mut(&position).expect("Not initialized.")
        } else {
            &mut self.instructions[position as usize]
        }
    }

    fn opcode(&mut self) -> Opcode {
        to_opcode(*self.instruction(self.ptr))
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Opcode {
    a: u16,
    b: u16,
    c: u16,
    de: u16,
}

fn to_opcode(mut n: i64) -> Opcode {
    let de: u16 = (n % 100) as u16;
    n /= 100;
    let c: u16 = (n % 10) as u16;
    n /= 10;
    let b: u16 = (n % 10) as u16;
    n /= 10;
    let a: u16 = (n % 10) as u16;
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
    struct ComputerResult {
        computer: Computer,
        outputs: Vec<i64>,
    }
    fn computer_one(instructions: Instructions) -> ComputerResult {
        computer_n(instructions, 1)
    }

    fn computer_n(instructions: Instructions, input: i64) -> ComputerResult {
        let mut computer = Computer::new(instructions).with_input(input);
        let outputs = computer.run();
        ComputerResult { computer, outputs }
    }
    fn get_value(instructions: Instructions, mode: u16, value: i64) -> i64 {
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
        computer_one_1: stringify(computer_one(vec![1, 0, 0, 0, 99]).computer.instructions) => "2,0,0,0,99";
        computer_one_2: stringify(computer_one(vec![2, 3, 0, 3, 99]).computer.instructions) => "2,3,0,6,99";
        computer_one_3: stringify(computer_one(vec![2, 4, 4, 5, 99, 0]).computer.instructions) => "2,4,4,5,99,9801";
        computer_one_4: stringify(computer_one(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).computer.instructions) => "30,1,1,4,2,5,6,0,99";

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

        relative_mode_example_1: stringify(computer_n(vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99], 42).computer.instructions) => "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        relative_mode_example_2: count_digits(computer_n(vec![1102,34915192,34915192,7,4,7,99,0], 42).outputs[0]) => 16;
        relative_mode_example_3: computer_n(vec![104,1125899906842624,99], 42).outputs[0] => 1125899906842624;

        op_203_base_1000_input_42_eq_42: computer_n(vec![109, 1000, 203, 5, 4, 1005, 99], 42).outputs[0] => 42;
    }
}

const PUZZLE_INPUT: [i64; 656] = [
    3,
    8,
    1005,
    8,
    334,
    1106,
    0,
    11,
    0,
    0,
    0,
    104,
    1,
    104,
    0,
    3,
    8,
    102,
    -1,
    8,
    10,
    101,
    1,
    10,
    10,
    4,
    10,
    108,
    0,
    8,
    10,
    4,
    10,
    1002,
    8,
    1,
    28,
    2,
    1108,
    5,
    10,
    3,
    8,
    102,
    -1,
    8,
    10,
    101,
    1,
    10,
    10,
    4,
    10,
    1008,
    8,
    1,
    10,
    4,
    10,
    1001,
    8,
    0,
    55,
    1,
    102,
    18,
    10,
    1,
    2,
    5,
    10,
    3,
    8,
    1002,
    8,
    -1,
    10,
    1001,
    10,
    1,
    10,
    4,
    10,
    108,
    1,
    8,
    10,
    4,
    10,
    1001,
    8,
    0,
    84,
    1,
    106,
    11,
    10,
    2,
    1008,
    6,
    10,
    1,
    4,
    4,
    10,
    1006,
    0,
    55,
    3,
    8,
    1002,
    8,
    -1,
    10,
    1001,
    10,
    1,
    10,
    4,
    10,
    108,
    0,
    8,
    10,
    4,
    10,
    102,
    1,
    8,
    121,
    1,
    107,
    9,
    10,
    3,
    8,
    102,
    -1,
    8,
    10,
    101,
    1,
    10,
    10,
    4,
    10,
    108,
    1,
    8,
    10,
    4,
    10,
    101,
    0,
    8,
    147,
    2,
    1002,
    4,
    10,
    2,
    104,
    18,
    10,
    1,
    107,
    16,
    10,
    1,
    108,
    8,
    10,
    3,
    8,
    102,
    -1,
    8,
    10,
    101,
    1,
    10,
    10,
    4,
    10,
    108,
    0,
    8,
    10,
    4,
    10,
    102,
    1,
    8,
    185,
    3,
    8,
    1002,
    8,
    -1,
    10,
    1001,
    10,
    1,
    10,
    4,
    10,
    1008,
    8,
    0,
    10,
    4,
    10,
    101,
    0,
    8,
    208,
    2,
    1009,
    16,
    10,
    1006,
    0,
    7,
    1006,
    0,
    18,
    1,
    1105,
    8,
    10,
    3,
    8,
    1002,
    8,
    -1,
    10,
    101,
    1,
    10,
    10,
    4,
    10,
    108,
    1,
    8,
    10,
    4,
    10,
    101,
    0,
    8,
    243,
    2,
    1105,
    20,
    10,
    2,
    106,
    10,
    10,
    1006,
    0,
    67,
    3,
    8,
    1002,
    8,
    -1,
    10,
    101,
    1,
    10,
    10,
    4,
    10,
    108,
    0,
    8,
    10,
    4,
    10,
    1001,
    8,
    0,
    276,
    2,
    1103,
    5,
    10,
    2,
    1104,
    7,
    10,
    1006,
    0,
    35,
    2,
    1105,
    3,
    10,
    3,
    8,
    1002,
    8,
    -1,
    10,
    101,
    1,
    10,
    10,
    4,
    10,
    1008,
    8,
    1,
    10,
    4,
    10,
    1002,
    8,
    1,
    314,
    101,
    1,
    9,
    9,
    1007,
    9,
    1097,
    10,
    1005,
    10,
    15,
    99,
    109,
    656,
    104,
    0,
    104,
    1,
    21102,
    936995824532,
    1,
    1,
    21101,
    0,
    351,
    0,
    1105,
    1,
    455,
    21102,
    1,
    387508445964,
    1,
    21102,
    362,
    1,
    0,
    1106,
    0,
    455,
    3,
    10,
    104,
    0,
    104,
    1,
    3,
    10,
    104,
    0,
    104,
    0,
    3,
    10,
    104,
    0,
    104,
    1,
    3,
    10,
    104,
    0,
    104,
    1,
    3,
    10,
    104,
    0,
    104,
    0,
    3,
    10,
    104,
    0,
    104,
    1,
    21102,
    1,
    235244973059,
    1,
    21101,
    409,
    0,
    0,
    1106,
    0,
    455,
    21102,
    179410541659,
    1,
    1,
    21101,
    0,
    420,
    0,
    1105,
    1,
    455,
    3,
    10,
    104,
    0,
    104,
    0,
    3,
    10,
    104,
    0,
    104,
    0,
    21101,
    868402070292,
    0,
    1,
    21102,
    1,
    443,
    0,
    1106,
    0,
    455,
    21102,
    1,
    709584749324,
    1,
    21102,
    454,
    1,
    0,
    1106,
    0,
    455,
    99,
    109,
    2,
    22102,
    1,
    -1,
    1,
    21101,
    40,
    0,
    2,
    21102,
    486,
    1,
    3,
    21101,
    0,
    476,
    0,
    1106,
    0,
    519,
    109,
    -2,
    2105,
    1,
    0,
    0,
    1,
    0,
    0,
    1,
    109,
    2,
    3,
    10,
    204,
    -1,
    1001,
    481,
    482,
    497,
    4,
    0,
    1001,
    481,
    1,
    481,
    108,
    4,
    481,
    10,
    1006,
    10,
    513,
    1101,
    0,
    0,
    481,
    109,
    -2,
    2106,
    0,
    0,
    0,
    109,
    4,
    2102,
    1,
    -1,
    518,
    1207,
    -3,
    0,
    10,
    1006,
    10,
    536,
    21102,
    0,
    1,
    -3,
    21202,
    -3,
    1,
    1,
    22102,
    1,
    -2,
    2,
    21102,
    1,
    1,
    3,
    21102,
    555,
    1,
    0,
    1106,
    0,
    560,
    109,
    -4,
    2106,
    0,
    0,
    109,
    5,
    1207,
    -3,
    1,
    10,
    1006,
    10,
    583,
    2207,
    -4,
    -2,
    10,
    1006,
    10,
    583,
    21201,
    -4,
    0,
    -4,
    1106,
    0,
    651,
    21201,
    -4,
    0,
    1,
    21201,
    -3,
    -1,
    2,
    21202,
    -2,
    2,
    3,
    21102,
    602,
    1,
    0,
    1106,
    0,
    560,
    22102,
    1,
    1,
    -4,
    21101,
    0,
    1,
    -1,
    2207,
    -4,
    -2,
    10,
    1006,
    10,
    621,
    21102,
    0,
    1,
    -1,
    22202,
    -2,
    -1,
    -2,
    2107,
    0,
    -3,
    10,
    1006,
    10,
    643,
    21201,
    -1,
    0,
    1,
    21102,
    643,
    1,
    0,
    106,
    0,
    518,
    21202,
    -2,
    -1,
    -2,
    22201,
    -4,
    -2,
    -4,
    109,
    -5,
    2106,
    0,
    0,
];
