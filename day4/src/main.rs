fn main() {
    let range_start = 353_096;
    let range_end = 843_212;
    println!(
        "1. passwords: {}",
        (range_start..=range_end).filter(|n| rules_1(*n)).count()
    );
    println!(
        "2. passwords: {}",
        (range_start..=range_end).filter(|n| rules_2(*n)).count()
    );
}

fn rules_1(n: i64) -> bool {
    let mut last_digit = -1;
    let mut last_digit_condition = false;
    do_if_increasing_number(n, |digit| {
        if digit == last_digit {
            last_digit_condition = true;
        }
        last_digit = digit;
    }) && last_digit_condition
}

fn rules_2(n: i64) -> bool {
    let mut last_digit = -1;
    let mut last_digit_counter = 0;
    let mut last_digit_condition = false;
    do_if_increasing_number(n, |digit| {
        if digit == last_digit {
            last_digit_counter += 1;
        } else {
            if last_digit_counter == 1 {
                last_digit_condition = true;
            }
            last_digit_counter = 0;
        }
        last_digit = digit;
    }) && (last_digit_condition || last_digit_counter == 1)
}

fn do_if_increasing_number(n: i64, mut action: impl FnMut(i64)) -> bool {
    let mut min = 0;
    for digit in decompose_10(n).into_iter() {
        if digit > min {
            min = digit;
        }
        if min > digit {
            return false;
        }
        action(digit);
    }
    true
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
    fn test_rules1_00() {
        assert_eq!(rules_1(1), false);
    }

    #[test]
    fn test_rules1_01() {
        assert_eq!(rules_1(112), true);
    }

    #[test]
    fn test_rules1_1() {
        assert_eq!(rules_1(111111), true);
    }

    #[test]
    fn test_rules1_2() {
        assert_eq!(rules_1(223450), false);
    }

    #[test]
    fn test_rules1_3() {
        assert_eq!(rules_1(123789), false);
    }

    #[test]
    fn test_rules1_4() {
        assert_eq!(rules_1(112233), true);
    }
    #[test]
    fn test_rules2_1() {
        assert_eq!(rules_2(111111), false);
    }

    #[test]
    fn test_rule_5() {
        assert_eq!(rules_2(123444), false);
    }

    #[test]
    fn test_rule_5bis() {
        assert_eq!(rules_2(111), false);
    }

    #[test]
    fn test_rule_6() {
        assert_eq!(rules_2(111122), true);
    }
}
