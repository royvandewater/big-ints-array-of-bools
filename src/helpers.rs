pub fn char_to_bool_vector(character: char) -> Vec<bool> {
    match character {
        '0' => vec![false],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_number_when_it_is_returns_true() {
        assert!(is_number("123"))
    }

    #[test]
    fn test_is_number_when_it_is_not_returns_false() {
        assert!(!is_number("abc"))
    }
}
