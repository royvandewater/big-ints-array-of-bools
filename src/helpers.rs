pub fn add_three_bools(lhs: bool, rhs: bool, carry: bool) -> (bool, bool) {
    if lhs && rhs && carry {
        return (true, true);
    }

    if (lhs && rhs) || (lhs && carry) || (rhs && carry) {
        return (true, false);
    }

    if lhs || rhs || carry {
        return (false, true);
    }

    (false, false)
}

pub fn char_to_bool_vector(character: char) -> Vec<bool> {
    match character {
        '0' => vec![],
        '1' => vec![true],
        '2' => vec![true, false],
        '3' => vec![true, true],
        '4' => vec![true, false, false],
        '5' => vec![true, false, true],
        '6' => vec![true, true, false],
        '7' => vec![true, true, true],
        '8' => vec![true, false, false, false],
        '9' => vec![true, false, false, true],
        _ => vec![],
    }
}

pub fn is_number(value: &str) -> bool {
    for val in value.chars() {
        if !val.is_digit(10) {
            return false;
        }
    }
    true
}

pub fn sub_three_bools(lhs: bool, rhs: bool, carry: bool) -> (bool, bool) {
    let new_carry = (!lhs && (rhs || carry)) || (lhs && rhs && carry);
    let val = (lhs && !(rhs || carry)) || (lhs && rhs && carry) || (!lhs && rhs && !carry) ||
        (!lhs && !rhs && carry);

    (new_carry, val)
}

#[cfg(test)]
mod tests {
    use super::*;

    // add_three_bools
    #[test]
    fn test_add_three_bools_fff() {
        assert_eq!((false, false), add_three_bools(false, false, false))
    }

    #[test]
    fn test_add_three_bools_fft() {
        assert_eq!((false, true), add_three_bools(false, false, true))
    }

    #[test]
    fn test_add_three_bools_ftf() {
        assert_eq!((false, true), add_three_bools(false, true, false))
    }

    #[test]
    fn test_add_three_bools_tff() {
        assert_eq!((false, true), add_three_bools(true, false, false))
    }

    #[test]
    fn test_add_three_bools_ftt() {
        assert_eq!((true, false), add_three_bools(false, true, true))
    }

    #[test]
    fn test_add_three_bools_tft() {
        assert_eq!((true, false), add_three_bools(true, false, true))
    }

    #[test]
    fn test_add_three_bools_ttf() {
        assert_eq!((true, false), add_three_bools(true, true, false))
    }

    #[test]
    fn test_add_three_bools_ttt() {
        assert_eq!((true, true), add_three_bools(true, true, true))
    }

    // is_number
    #[test]
    fn test_is_number_when_it_is_returns_true() {
        assert!(is_number("123"))
    }

    #[test]
    fn test_is_number_when_it_is_not_returns_false() {
        assert!(!is_number("abc"))
    }

    // sub_three_bools
    #[test]
    fn test_sub_three_bools_fff() {
        assert_eq!((false, false), sub_three_bools(false, false, false))
    }

    #[test]
    fn test_sub_three_bools_fft() {
        assert_eq!((true, true), sub_three_bools(false, false, true))
    }

    #[test]
    fn test_sub_three_bools_ftf() {
        assert_eq!((true, true), sub_three_bools(false, true, false))
    }

    #[test]
    fn test_sub_three_bools_ftt() {
        assert_eq!((true, true), sub_three_bools(false, true, false))
    }

    #[test]
    fn test_sub_three_bools_tff() {
        assert_eq!((false, true), sub_three_bools(true, false, false))
    }

    #[test]
    fn test_sub_three_bools_tft() {
        assert_eq!((false, false), sub_three_bools(true, false, true))
    }

    #[test]
    fn test_sub_three_bools_ttf() {
        assert_eq!((false, false), sub_three_bools(true, true, false))
    }

    #[test]
    fn test_sub_three_bools_ttt() {
        assert_eq!((true, true), sub_three_bools(true, true, true))
    }
}
