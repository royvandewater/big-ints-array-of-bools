use std::clone::Clone;
use std::ops::Add;
use std::ops::Mul;
use std::slice::Iter;

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

        let mut big_int = BigInt { value: vec![false] };
        // let mut count = 0;

        for val in value_str.chars() {
            let sub_value = BigInt::char_to_big_int(val);
            // let ten = BigInt { value: vec![true, false, true, false] };
            // if count > 0 {
            //     sub_value = sub_value * ten
            // }
            big_int = big_int + sub_value;
            // count += 1;
        }

        Ok(big_int)
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
}

impl Add<BigInt> for BigInt {
    type Output = BigInt;

    fn add(self, other: BigInt) -> BigInt {
        let mut value: Vec<bool> = Vec::new();

        println!("add: {:?} + {:?}", self.value, other.value);

        let mut lvalue = self.value.clone();
        lvalue.reverse();
        let mut lhs = lvalue.iter();

        let mut rvalue = other.value.clone();
        rvalue.reverse();
        let mut rhs = rvalue.iter();

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
            //
            // if !(lval || rval || carry) {
            //     value.push(false);
            //     carry = false;
            //     continue;
            // }
            //
            // if (lval && !rval && !carry) || (!lval && !rval && carry) || (!lval && rval && !carry) {
            //     value.push(true);
            //     carry = false;
            //     continue;
            // }
            //
            // if (lval && rval && !carry) || (lval && !rval && carry) || (!lval && rval && carry) {
            //     value.push(false);
            //     carry = true;
            //     continue;
            // }
            //
            // // The only combination left is (lval && rval && carry)
            // value.push(true);
            // carry = true;
        }
    }
}

impl Clone for BigInt {
    fn clone(&self) -> BigInt {
        BigInt { value: self.value.clone() }
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, other: BigInt) -> BigInt {
        let rhs = other.clone();
        let mut product = BigInt { value: vec![] };

        for val in self.value {
            if val == true {
                product = product + rhs.clone()
            }
            // rhs << 1
        }

        return BigInt { value: vec![false] };
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

        assert_eq!(vec![false], zero.value);
    }

    #[test]
    fn it_stores_its_value_as_bool_vec_with_2() {
        let two = BigInt::new("2").unwrap();

        assert_eq!(vec![true, false], two.value);
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

    // Add

    #[test]
    fn zero_plus_zero_is_zero() {
        let zero1 = BigInt::new("0").unwrap();
        let zero2 = BigInt::new("0").unwrap();
        let zero3 = BigInt::new("0").unwrap();

        assert_eq!(zero3, zero1 + zero2);
    }

    #[test]
    fn zero_plus_one_is_one() {
        let zero = BigInt::new("0").unwrap();
        let one1 = BigInt::new("1").unwrap();
        let one2 = BigInt::new("1").unwrap();

        assert_eq!(one1, zero + one2);
    }

    #[test]
    fn one_plus_zero_is_one() {
        let zero = BigInt::new("0").unwrap();
        let one1 = BigInt::new("1").unwrap();
        let one2 = BigInt::new("1").unwrap();

        assert_eq!(one1, one2 + zero);
    }

    #[test]
    fn one_plus_one_is_two() {
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

    // #[test]
    // fn test_1_times_1_is_1() {
    //     let one = BigInt::new("1").unwrap();
    //     assert_eq!(one, one.clone() * one.clone())
    // }
}
