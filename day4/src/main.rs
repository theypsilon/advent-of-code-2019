fn main() {
    let range_start = 353_096;
    let range_end = 843_212;
    let passwords = (range_start..=range_end).filter(|n| rule(*n)).count();
    println!("2. passwords: {}", passwords);
}

fn rule(n: i64) -> bool {
    let mut min = 0;
    let mut last_digit = -1;
    let mut last_digit_counter = 0;
    let mut last_digit_condition = false;
    for digit in decompose_10(n).into_iter() {
        if digit > min {
            min = digit;
        }
        if min > digit {
            return false;
        }
        if digit == last_digit {
            last_digit_counter += 1;
        } else {
            if last_digit_counter == 1 {
                last_digit_condition = true;
            }
            last_digit_counter = 0;
        }
        last_digit = digit;
    }
    last_digit_condition || last_digit_counter == 1
}

fn decompose_10(mut n: i64) -> Vec<i64> {
    let mut result = vec![];
    while n > 0 {
        result.push(n % 10);
        n /= 10;
    }
    result.reverse();
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decompose_10_1() {
        assert_eq!(decompose_10(12), vec![1, 2]);
    }

    #[test]
    fn test_decompose_10_2() {
        assert_eq!(decompose_10(21), vec![2, 1]);
    }

    #[test]
    fn test_decompose_10_3() {
        assert_eq!(decompose_10(20), vec![2, 0]);
    }

    #[test]
    fn test_rule_00() {
        assert_eq!(rule(1), false);
    }

    #[test]
    fn test_rule_01() {
        assert_eq!(rule(112), true);
    }

    #[test]
    fn test_rule_1() {
        assert_eq!(rule(111111), false);
    }

    #[test]
    fn test_rule_2() {
        assert_eq!(rule(223450), false);
    }

    #[test]
    fn test_rule_3() {
        assert_eq!(rule(123789), false);
    }

    #[test]
    fn test_rule_4() {
        assert_eq!(rule(112233), true);
    }

    #[test]
    fn test_rule_5() {
        assert_eq!(rule(123444), false);
    }

    #[test]
    fn test_rule_5bis() {
        assert_eq!(rule(111), false);
    }

    #[test]
    fn test_rule_6() {
        assert_eq!(rule(111122), true);
    }
}
