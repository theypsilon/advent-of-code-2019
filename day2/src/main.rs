fn main() {
    let mut program = PUZZLE_INPUT.to_vec();
    program[1] = 12;
    program[2] = 2;
    let result = computer(program)[0];
    println!("1. result: {}", result);

    let mut output: i64 = -1;
    'outer: for noun in 0..99 {
        for verb in 0..99 {
            let mut program = PUZZLE_INPUT.to_vec();
            program[1] = noun;
            program[2] = verb;
            let result = computer(program)[0];
            if result == 19690720 {
                output = 100 * noun + verb;
                break 'outer;
            }
        }
    }
    println!("2. output: {}", output);
}

fn computer(mut result: Vec<i64>) -> Vec<i64> {
    let mut index = 0;
    loop {
        match result[index] {
            1 => {
                let first = result[index + 1] as usize;
                let second = result[index + 2] as usize;
                let third = result[index + 3] as usize;
                result[third] = result[first] + result[second];
                index += 4;
            }
            2 => {
                let first = result[index + 1] as usize;
                let second = result[index + 2] as usize;
                let third = result[index + 3] as usize;
                result[third] = result[first] * result[second];
                index += 4;
            }
            99 => break,
            _ => panic!("Something went wrong!"),
        }
    }
    result
}

#[cfg(test)]
mod test {

    use super::*;

    fn stringify(program: Vec<i64>) -> String {
        program
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    #[test]
    fn test_computer() {
        assert_eq!(stringify(computer(vec![1, 0, 0, 0, 99])), "2,0,0,0,99");
        assert_eq!(stringify(computer(vec![2, 3, 0, 3, 99])), "2,3,0,6,99");
        assert_eq!(
            stringify(computer(vec![2, 4, 4, 5, 99, 0])),
            "2,4,4,5,99,9801"
        );
        assert_eq!(
            stringify(computer(vec![1, 1, 1, 4, 99, 5, 6, 0, 99])),
            "30,1,1,4,2,5,6,0,99"
        );
    }
}

const PUZZLE_INPUT: [i64; 149] = [
    1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 5, 23, 2, 23, 6, 27, 1,
    27, 5, 31, 2, 6, 31, 35, 1, 5, 35, 39, 2, 39, 9, 43, 1, 43, 5, 47, 1, 10, 47, 51, 1, 51, 6, 55,
    1, 55, 10, 59, 1, 59, 6, 63, 2, 13, 63, 67, 1, 9, 67, 71, 2, 6, 71, 75, 1, 5, 75, 79, 1, 9, 79,
    83, 2, 6, 83, 87, 1, 5, 87, 91, 2, 6, 91, 95, 2, 95, 9, 99, 1, 99, 6, 103, 1, 103, 13, 107, 2,
    13, 107, 111, 2, 111, 10, 115, 1, 115, 6, 119, 1, 6, 119, 123, 2, 6, 123, 127, 1, 127, 5, 131,
    2, 131, 6, 135, 1, 135, 2, 139, 1, 139, 9, 0, 99, 2, 14, 0, 0,
];
