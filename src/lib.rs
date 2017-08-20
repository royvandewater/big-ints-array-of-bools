use std::clone::Clone;
use std::fmt;
use std::ops::Add;
use std::ops::Mul;

mod helpers;

#[derive(Debug)]
pub struct BigInt {
    value: Vec<bool>,
}

impl BigInt {
    pub fn new<'a>(value_str: &'a str) -> Result<BigInt, &str> {
        if !helpers::is_number(value_str) {
            return Err("Value contained invalid characters. Only 0-9 allowed");
        }

        let mut big_int = BigInt { value: vec![] };
        let mut count = BigInt { value: vec![] };
        let one = BigInt { value: vec![true] };

        for val in value_str.chars().rev() {
            let mut sub_value = BigInt::char_to_big_int(val);
            let ten = BigInt { value: vec![true, false, true, false] };

            sub_value = sub_value * ten.pow(count.clone());
            big_int = big_int + sub_value;
            count = count.clone() + one.clone();
        }

        Ok(big_int)
    }

    pub fn pow(self, other: BigInt) -> BigInt {
        let mut counter = BigInt::zero();
        let mut product = BigInt::one();

        loop {
            if counter == other {
                return product;
            }
            counter = counter + BigInt::one();
            product = self.clone() * product
        }
    }

    pub fn str<'a>(&self) -> String {
        let mut value = String::from("");

        for val in self.value.iter().rev() {
            if *val {
                value = value + "1"
            } else {
                value = value + "0"
            }
        }

        value
    }

    pub fn to_string(&self) -> String {
        let mut value_str = String::new();

        for val in self.value.clone() {
            if val {
                value_str += "1"
            } else {
                value_str += "0"
            }
        }

        value_str
    }

    // static
    fn char_to_big_int(character: char) -> BigInt {
        BigInt { value: helpers::char_to_bool_vector(character) }
    }

    fn zero() -> BigInt {
        BigInt { value: vec![] }
    }

    fn one() -> BigInt {
        BigInt { value: vec![true] }
    }
}

impl Add<BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> BigInt {
        let mut value: Vec<bool> = Vec::new();
        let mut lhs = self.value.iter().rev();
        let mut rhs = other.value.iter().rev();
        let mut carry = false;

        loop {
            let lopt = lhs.next();
            let ropt = rhs.next();

            if lopt.is_none() && ropt.is_none() {
                if carry {
                    value.push(true);
                }
                value.reverse();
                return BigInt { value };
            }

            let lval = *lopt.unwrap_or(&false);
            let rval = *ropt.unwrap_or(&false);

            let (new_carry, val) = helpers::add_three_bools(lval, rval, carry);
            carry = new_carry;
            value.push(val);
            continue;
        }
    }
}

impl Clone for BigInt {
    fn clone(&self) -> BigInt {
        BigInt { value: self.value.clone() }
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "{}", self.str())
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, other: BigInt) -> BigInt {
        let mut rhs = other.clone();
        let mut product = BigInt { value: vec![] };

        for val in self.value.iter().rev() {
            if *val {
                product = product + rhs.clone()
            }
            rhs.value.push(false)
        }

        return product;
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &BigInt) -> bool {
        let mut lhs = self.value.iter();
        let mut rhs = other.value.iter();

        loop {
            let lval = lhs.next();
            let rval = rhs.next();

            if lval.is_none() && rval.is_none() {
                return true;
            }
            if lval.is_none() || rval.is_none() {
                return false;
            }

            if lval != rval {
                return false;
            }
        }
    }

    fn ne(&self, other: &BigInt) -> bool {
        self.to_string() != other.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_instantiates() {
        let one = BigInt::new("1");
        assert!(one.is_ok());
    }

    #[test]
    fn it_returns_an_error_if_not_a_digit() {
        let result = BigInt::new("a");
        assert!(result.is_err());
    }

    #[test]
    fn it_stores_its_value_as_bool_vec_with_0() {
        let zero = BigInt::new("0").unwrap();
        let zero_vec: Vec<bool> = vec![];

        assert_eq!(zero_vec, zero.value);
    }

    #[test]
    fn it_stores_its_value_as_bool_vec_with_2() {
        let two = BigInt::new("2").unwrap();

        assert_eq!(vec![true, false], two.value);
    }

    #[test]
    fn it_stores_its_value_as_bool_vec_with_10() {
        let ten = BigInt::new("10").unwrap();

        assert_eq!(vec![true, false, true, false], ten.value);
    }

    #[test]
    fn it_stores_its_value_as_bool_vec_with_64() {
        let ten = BigInt::new("64").unwrap();

        assert_eq!(
            vec![true, false, false, false, false, false, false],
            ten.value
        );
    }

    #[test]
    fn it_stores_its_value_as_bool_vec_with_128() {
        let ten = BigInt::new("128").unwrap();

        assert_eq!(
            vec![true, false, false, false, false, false, false, false],
            ten.value
        );
    }

    #[test]
    fn it_stores_a_big_value() {
        // (2^64) + 1
        let big = BigInt::new("9223372036854775808");
        assert!(big.is_ok());
    }

    #[test]
    fn it_stores_a_bigger_value() {
        // (2^64) + 1
        let big = BigInt::new("9223372036854775808").unwrap();
        let small = BigInt::new("100").unwrap();
        let bigger = big.pow(small);
    }

    #[test]
    fn it_is_not_the_same_as_another() {
        let zero = BigInt::new("0").unwrap();
        let one = BigInt::new("1").unwrap();
        assert_ne!(zero, one);
    }

    #[test]
    fn it_is_the_same_as_itself() {
        let one1 = BigInt::new("1").unwrap();
        let one2 = BigInt::new("1").unwrap();

        assert_eq!(one1, one2);
    }

    #[test]
    fn it_is_does_not_think_zero_is_2() {
        let zero = BigInt::new("0").unwrap();
        let two = BigInt::new("2").unwrap();

        assert_ne!(zero, two);
    }

    // str()
    #[test]
    fn test_str_0() {
        let zero = BigInt::new("0").unwrap();

        assert_eq!("0", zero.str())
    }

    #[test]
    fn test_str_1() {
        let one = BigInt::new("1").unwrap();

        assert_eq!("1", one.str())
    }

    #[test]
    fn test_str_2() {
        let two = BigInt::new("2").unwrap();

        assert_eq!("2", two.str())
    }

    // Add

    #[test]
    fn test_zero_plus_zero_is_zero() {
        let zero1 = BigInt::new("0").unwrap();
        let zero2 = BigInt::new("0").unwrap();
        let zero3 = BigInt::new("0").unwrap();

        assert_eq!(zero3, zero1 + zero2);
    }

    #[test]
    fn test_zero_plus_one_is_one() {
        let zero = BigInt::new("0").unwrap();
        let one1 = BigInt::new("1").unwrap();
        let one2 = BigInt::new("1").unwrap();

        assert_eq!(one1, zero + one2);
    }

    #[test]
    fn test_one_plus_zero_is_one() {
        let zero = BigInt::new("0").unwrap();
        let one1 = BigInt::new("1").unwrap();
        let one2 = BigInt::new("1").unwrap();

        assert_eq!(one1, one2 + zero);
    }

    #[test]
    fn test_one_plus_one_is_two() {
        let one1 = BigInt::new("1").unwrap();
        let one2 = BigInt::new("1").unwrap();
        let two = BigInt::new("2").unwrap();

        assert_eq!(two, one1 + one2);
    }

    #[test]
    fn test_1_plus_2_is_3() {
        let one = BigInt::new("1").unwrap();
        let two = BigInt::new("2").unwrap();
        let three = BigInt::new("3").unwrap();

        assert_eq!(three, one + two);
    }

    #[test]
    fn test_1_plus_2_is_not_1() {
        let one = BigInt::new("1").unwrap();
        let two = BigInt::new("2").unwrap();

        assert_ne!(one.clone(), one + two);
    }

    // Multiply
    #[test]
    fn test_0_times_0_is_0() {
        let zero = BigInt::new("0").unwrap();
        assert_eq!(zero, zero.clone() * zero.clone())
    }

    #[test]
    fn test_1_times_1_is_1() {
        let one = BigInt::new("1").unwrap();
        assert_eq!(one, one.clone() * one.clone())
    }

    #[test]
    fn test_1_times_2_is_2() {
        let one = BigInt::new("1").unwrap();
        let two = BigInt::new("2").unwrap();
        assert_eq!(two, one.clone() * two.clone())
    }

    #[test]
    fn test_2_times_2_is_4() {
        let two = BigInt::new("2").unwrap();
        let four = BigInt::new("4").unwrap();
        assert_eq!(four, two.clone() * two.clone())
    }

    // Pow
    #[test]
    fn test_pow_0_pow_0_is_1() {
        let zero1 = BigInt::new("0").unwrap();
        let zero2 = BigInt::new("0").unwrap();
        let one = BigInt::new("1").unwrap();
        assert_eq!(one, zero1.pow(zero2))
    }

    // Pow
    #[test]
    fn test_pow_2_pow_1_is_2() {
        let one = BigInt::new("1").unwrap();
        let two1 = BigInt::new("2").unwrap();
        let two2 = BigInt::new("2").unwrap();
        assert_eq!(two1, two2.pow(one))
    }

    // Pow
    #[test]
    fn test_pow_2_pow_2_is_4() {
        let two1 = BigInt::new("2").unwrap();
        let two2 = BigInt::new("2").unwrap();
        let four = BigInt::new("4").unwrap();
        assert_eq!(four, two1.pow(two2))
    }
}
