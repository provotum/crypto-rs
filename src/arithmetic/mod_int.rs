use ::arithmetic::mod_inverse;
use num::bigint::BigInt;
use num::bigint::RandBigInt;
use num::One;
use num;
use num::pow::Pow;
use num::ToPrimitive;
use num::Zero;
use num::Num;
use rand;
use std::clone::Clone;
use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::cmp::PartialOrd;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::fmt::{Formatter, Result, Display, Debug};
use serde;
use std::result::Result as stdResult;


// TODO
//use std::cmp::Ordering::{self, Less, Greater, Equal};


/// An integer with modular operations.
#[derive(Hash)]
pub struct ModInt {
    /// The value.
    pub value: BigInt,
    /// The modulus.
    pub modulus: BigInt,
}

impl serde::Serialize for ModInt {
    fn serialize<S>(&self, serializer: S) -> stdResult<S::Ok, S::Error> where
        S: serde::Serializer {

        (&self.value, &self.modulus).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ModInt {
    fn deserialize<D>(deserializer: D) -> stdResult<Self, D::Error>
        where D: serde::Deserializer<'de>
    {
        let (value, modulus) = serde::Deserialize::deserialize(deserializer)?;
        Ok(ModInt {value, modulus})
    }
}



impl Clone for ModInt {
    fn clone(&self) -> Self {
        ModInt {
            value: self.value.clone(),
            modulus: self.modulus.clone(),
        }
    }

    fn clone_from(&mut self, source: &Self) {
        self.value = source.value.clone();
        self.modulus = source.modulus.clone();
    }
}

pub trait From {
    /// Create a ModInt with the given value and modulus.
    fn from_value_modulus(value: BigInt, modulus: BigInt) -> ModInt;

    /// Create a ModInt with the given value and a zero modulus.
    fn from_value(value: BigInt) -> ModInt;

    fn from_hex_string(hex_string: String, modulus: BigInt) -> ModInt;
}

impl From for ModInt {
    fn from_value_modulus(value: BigInt, modulus: BigInt) -> ModInt {
        let non_normalized = ModInt {
            value,
            modulus,
        };

        non_normalized.normalize()
    }

    fn from_value(value: BigInt) -> ModInt {
        let non_normalized = ModInt {
            value,
            modulus: BigInt::zero(),
        };

        non_normalized.normalize()
    }

    fn from_hex_string(hex_string: String, modulus: BigInt) -> ModInt {
        let value = BigInt::from_str_radix(&hex_string.as_str(), 16);

        let non_normalized = ModInt {
            value: value.unwrap(),
            modulus,
        };

        non_normalized.normalize()
    }
}


/// Normalize ModInt values.
trait Normalize {
    /// Normalize a ModInt, i.e. reduce it by
    /// applying `value mod modulus`.
    /// Note, that the value is updated but the modulus remains.
    fn normalize(self) -> ModInt;
}


impl Normalize for ModInt {
    fn normalize(mut self) -> ModInt {
        if self.modulus > BigInt::zero() {
            self.value = self.value.rem(self.modulus.clone());
        }

        self
    }
}

impl Eq for ModInt {}

impl PartialEq<ModInt> for ModInt {
    fn eq(&self, other: &ModInt) -> bool {
        // we have to normalize, i.e. reduce the values
        // whenever we have the same modulus.
        // 21 mod 5 === 1 mod 5
        if self.modulus > BigInt::zero() {
            let _val: BigInt = self.value.clone();
            let _mod: BigInt = self.modulus.clone();

            let normalized_val = _val.rem(_mod);

            return normalized_val.eq(&other.value);
        } else {
            return self.value.eq(&other.value);
        }
    }

    fn ne(&self, other: &ModInt) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd<ModInt> for ModInt {
    fn partial_cmp(&self, other: &ModInt) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ModInt {
    fn cmp(&self, other: &Self) -> Ordering {
        let _val: BigInt = self.value.clone();
        let _mod: BigInt = self.modulus.clone();

        let normalized_val = _val.rem(_mod);

        normalized_val.cmp(&other.value)
    }
}

impl Zero for ModInt {
    /// # Zero ModInt
    ///
    /// Returns a ModInt having both the value and
    /// its modulus set to zero.
    fn zero() -> Self {
        ModInt {
            value: BigInt::zero(),
            modulus: BigInt::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        self.value.eq(&BigInt::zero())
    }
}


impl One for ModInt {
    /// # One ModInt
    ///
    /// Returns a ModInt having the value set to one and
    /// its modulus set to zero.
    fn one() -> Self {
        ModInt {
            value: BigInt::one(),
            modulus: BigInt::zero(),
        }
    }

    fn is_one(&self) -> bool where Self: PartialEq {
        // TODO: normalize
        self.value.eq(&BigInt::one())
    }
}

// Negation of ModIntegers
impl Neg for ModInt {
    type Output = ModInt;

    #[inline]
    fn neg(mut self) -> ModInt {
        self = self.normalize();

        let zero = BigInt::zero();

        if self.modulus.eq(&zero) {
            self.value = self.value.neg()
        } else {
            self.value = self.modulus.clone().sub(self.value.clone());
        }

        self.normalize()
    }
}

impl Add<ModInt> for ModInt {
    type Output = ModInt;

    #[inline]
    fn add(mut self, rhs: ModInt) -> ModInt {
        // TODO: apply modulus after add to avoid  overflows
        self.value = self.value.add(rhs.value);

        self.normalize()
    }
}

impl Sub<ModInt> for ModInt {
    type Output = ModInt;

    #[inline]
    fn sub(mut self, rhs: ModInt) -> ModInt {
        let zero = BigInt::zero();

        if self.modulus.eq(&zero) {
            self.value = self.value.sub(rhs.value)
        } else {
            self.value = self.value.add(rhs.neg().value)
        }

        self.normalize()
    }
}

impl Mul<ModInt> for ModInt {
    type Output = ModInt;

    #[inline]
    fn mul(mut self, rhs: ModInt) -> ModInt {
        self.value = self.value.mul(rhs.value);

        self.normalize()
    }
}

impl Div<ModInt> for ModInt {
    type Output = ModInt;

    #[inline]
    fn div(mut self, rhs: ModInt) -> ModInt {
        let zero = BigInt::zero();

        if rhs.value.eq(&BigInt::zero()) {
            panic!("Division by zero is not defined");
        }

        if self.modulus.eq(&zero) {
            self.value = self.value.div(rhs.value)
        } else {
            let inv: Option<BigInt> = mod_inverse::mod_inverse(rhs.value.clone(), self.modulus.clone());

            let inverse: BigInt;
            match inv {
                None => panic!("failed to compute inverse"),
                Some(x) => inverse = x
            }

            self.value = self.value.mul(inverse);
            self.value = self.value.rem(self.modulus.clone());
        }

        self.normalize()
    }
}

impl Rem<ModInt> for ModInt {
    type Output = ModInt;

    #[inline]
    fn rem(mut self, rhs: ModInt) -> ModInt {
        self.value = self.value.rem(rhs.modulus);

        self
    }
}

impl Pow<ModInt> for ModInt {
    type Output = ModInt;

    #[inline]
    fn pow(mut self, rhs: ModInt) -> ModInt {
        let zero = BigInt::zero();

        if self.modulus.eq(&zero) {
            let usize_val: usize;
            let result = rhs.value.to_usize();

            match result {
                Some(x) => usize_val = x,
                None => panic!("Failed to convert BigInt to usize")
            }

            self.value = num::pow(self.value, usize_val)
        } else {
            // Check whether order of the base divides the order of the exponent.
            // Otherwise, the result is not well-defined.
            // TODO: not sure whether this is appropriate...
//            if ! rhs.modulus.clone().rem(self.modulus.clone()).eq(&zero) {
//                panic!("Order of base is not compatible to the order of the exponent. Base modulus: {:?}, exponent modulus: {:?}", self.modulus.clone(), rhs.modulus.clone())
//            }
            let inv: BigInt = self.value.modpow(&rhs.value, &self.modulus);

            self.value = inv;
        }

        self.normalize()
    }
}

impl Display for ModInt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(val: {}, mod: {})", self.value, self.modulus)
    }
}

impl Debug for ModInt {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "(val: {}, mod: {})", self.value, self.modulus)
    }
}


/// # Random ModInt
///
/// Generate random numbers
pub trait RandModInt {
    /// Generate random ModInts with the given upper_bound.
    /// Note, that the returned ModInt has a modulus set equal to the given upper_bound.
    fn gen_modint(upper_bound: ModInt) -> ModInt;
}

impl RandModInt for ModInt {
    fn gen_modint(upper_bound: ModInt) -> ModInt {
        assert!(upper_bound.value > BigInt::zero(), "the upper_bound must be greater than zero");

        let mut rng = rand::thread_rng();
        let rnd_val = rng.gen_bigint_range(&BigInt::zero(), &upper_bound.value);

        ModInt {
            value: rnd_val,
            modulus: upper_bound.value,
        }
    }
}


#[cfg(test)]
mod mod_int_tests {
    use ::arithmetic::mod_int::From;
    use ::arithmetic::mod_int::ModInt;
    use ::arithmetic::mod_int::RandModInt;
    use ::num::bigint::BigInt;
    use ::num::One;
    use ::num::traits::Pow;
    use ::num::Zero;
    use ::std::ops::Neg;

    #[test]
    fn test_equal() {
        let one: ModInt = ModInt::one();
        let one2: ModInt = ModInt::one();

        assert_eq!(true, one == one2);
    }

    #[test]
    fn test_non_equal() {
        let one3: ModInt = ModInt::one();
        let zero: ModInt = ModInt::zero();

        assert_eq!(false, one3 == zero);
    }

    #[test]
    fn test_equal_normalized() {
        let one: ModInt = ModInt::from_value_modulus(
            BigInt::from(21),
            BigInt::from(4),
        );

        let other_one: ModInt = ModInt::from_value_modulus(
            BigInt::from(1),
            BigInt::from(4),
        );

        assert_eq!(true, one == other_one);
    }

    #[test]
    fn test_unequal() {
        let one: ModInt = ModInt::one();
        let zero: ModInt = ModInt::zero();

        assert_eq!(true, one != zero);
    }

    #[test]
    fn test_non_unequal() {
        let one: ModInt = ModInt::one();
        let one2: ModInt = ModInt::one();

        assert_eq!(false, one != one2);
    }

    #[test]
    fn test_zero() {
        let zero: ModInt = ModInt::zero();

        assert_eq!(BigInt::zero(), zero.value);
        assert_eq!(BigInt::zero(), zero.modulus);

        assert_eq!(true, zero.is_zero())
    }

    #[test]
    fn test_one() {
        let one: ModInt = ModInt::one();

        assert_eq!(BigInt::one(), one.value);
        assert_eq!(BigInt::zero(), one.modulus);

        assert_eq!(true, one.is_one())
    }

    #[test]
    fn test_negation_zero_modulus() {
        let one: ModInt = ModInt::one();
        let neg_one: ModInt = one.neg();

        assert_eq!(BigInt::one().neg(), neg_one.value);
        assert_eq!(BigInt::zero(), neg_one.modulus);
    }

    #[test]
    fn test_negation_non_zero_modulus() {
        let zero: ModInt = ModInt {
            value: BigInt::zero(),
            modulus: BigInt::from(11),
        };

        // 0 mod 11 = 0
        // (0 mod 11)^-1 = 11 mod 11 = 0
        let neg_zero: ModInt = zero.neg();
        assert_eq!(BigInt::from(0), neg_zero.value);
        assert_eq!(BigInt::from(11), neg_zero.modulus);


        let one: ModInt = ModInt {
            value: BigInt::from(23),
            modulus: BigInt::from(11),
        };

        // 23 mod 11 = 1
        // (23 mod 11)^-1 = 10
        let neg_one: ModInt = one.neg();
        assert_eq!(BigInt::from(10), neg_one.value);
        assert_eq!(BigInt::from(11), neg_one.modulus);

        let two: ModInt = ModInt {
            value: BigInt::from(2),
            modulus: BigInt::from(11),
        };

        // 2 mod 11 = 2
        // (2 mod 11)^-1 = 9
        let neg_two = two.neg();
        assert_eq!(BigInt::from(9), neg_two.value);
        assert_eq!(BigInt::from(11), neg_two.modulus);
    }

    #[test]
    fn test_add() {
        let one: ModInt = ModInt::one();
        let one2: ModInt = ModInt::one();

        let two = one + one2;
        assert_eq!(BigInt::from(2), two.value);
        assert_eq!(BigInt::zero(), two.modulus);

        let zero: ModInt = ModInt::zero();
        let zero2: ModInt = ModInt::zero();

        let zero_result = zero + zero2;
        assert_eq!(BigInt::zero(), zero_result.value);
        assert_eq!(BigInt::zero(), zero_result.modulus);

        // test overflow of mod round
        let nine: ModInt = ModInt {
            value: BigInt::from(9),
            modulus: BigInt::from(11),
        };
        let three: ModInt = ModInt {
            value: BigInt::from(3),
            modulus: BigInt::from(11),
        };

        let twelve_one = nine + three;
        assert_eq!(BigInt::from(1), twelve_one.value);
        assert_eq!(BigInt::from(11), twelve_one.modulus);
    }

    #[test]
    fn test_sub() {
        let two: ModInt = ModInt::from_value_modulus(BigInt::from(2), BigInt::zero());
        let one: ModInt = ModInt::one();

        let one2 = two - one;
        assert_eq!(BigInt::from(1), one2.value);
        assert_eq!(BigInt::zero(), one2.modulus);

        let one3: ModInt = ModInt::one();
        let one4: ModInt = ModInt::one();

        let zero: ModInt = one3 - one4;
        assert_eq!(BigInt::zero(), zero.value);
        assert_eq!(BigInt::zero(), zero.modulus);
    }

    #[test]
    fn test_mul() {
        let one: ModInt = ModInt::one();
        let one2: ModInt = ModInt::one();

        let one_mul: ModInt = one * one2;
        assert_eq!(BigInt::one(), one_mul.value);
        assert_eq!(BigInt::zero(), one_mul.modulus);

        let two: ModInt = ModInt::from_value_modulus(
            BigInt::from(2),
            BigInt::from(4),
        );
        let three: ModInt = ModInt::from_value_modulus(
            BigInt::from(3),
            BigInt::from(4),
        );

        // 2 * 3 mod 4 = 2
        let two = two * three;
        assert_eq!(BigInt::from(2), two.value);
        assert_eq!(BigInt::from(4), two.modulus);
    }

    #[test]
    fn test_div() {
        let one: ModInt = ModInt::from_value_modulus(
            BigInt::from(23),
            BigInt::from(11),
        );

        let two: ModInt = ModInt::from_value_modulus(
            BigInt::from(2),
            BigInt::from(0),
        );

        let div = one / two;
        assert_eq!(BigInt::from(6), div.value);
        assert_eq!(BigInt::from(11), div.modulus);


        let one2: ModInt = ModInt::from_value_modulus(
            BigInt::from(23),
            BigInt::from(11),
        );
        let two2: ModInt = ModInt::from_value_modulus(
            BigInt::from(2),
            BigInt::from(0),
        );

        let zero: ModInt = one2 - ModInt::one();
        let zero_res: ModInt = zero / two2;
        assert_eq!(BigInt::from(0), zero_res.value);
        assert_eq!(BigInt::from(11), zero_res.modulus);
    }

    #[test]
    #[should_panic(expected = "Division by zero is not defined")]
    fn test_invalid_div() {
        let one: ModInt = ModInt::one();
        let zero: ModInt = ModInt::zero();

        one / zero;
    }

    #[test]
    #[should_panic(expected = "Division by zero is not defined")]
    fn test_invalid_div_modulus() {
        let one: ModInt = ModInt::from_value_modulus(
            BigInt::one(),
            BigInt::from(5),
        );
        let zero: ModInt = ModInt::from_value_modulus(
            BigInt::zero(),
            BigInt::from(5),
        );

        one / zero;
    }

    #[test]
    fn test_rem() {
        let one: ModInt = ModInt::from_value_modulus(
            BigInt::from(21),
            BigInt::from(4),
        );

        let four: ModInt = ModInt::from_value_modulus(
            BigInt::from(4),
            BigInt::from(4),
        );

        let result = one % four;
        assert_eq!(BigInt::from(1), result.value);
        assert_eq!(BigInt::from(4), result.modulus);
    }

    #[test]
    fn test_negative_rem() {
        let neg_one: ModInt = ModInt::from_value_modulus(
            BigInt::from(-21),
            BigInt::from(4),
        );

        let four: ModInt = ModInt::from_value_modulus(
            BigInt::from(4),
            BigInt::from(4),
        );

        let result = neg_one % four;
        assert_eq!(BigInt::from(-1), result.value);
        assert_eq!(BigInt::from(4), result.modulus);
    }

    #[test]
    fn test_pow_zero_modulus() {
        let two: ModInt = ModInt::from_value_modulus(
            BigInt::from(2),
            BigInt::from(0),
        );

        let four: ModInt = ModInt::from_value_modulus(
            BigInt::from(4),
            BigInt::from(0),
        );

        let result = two.pow(four);
        assert_eq!(BigInt::from(16), result.value);
        assert_eq!(BigInt::from(0), result.modulus);
    }

    #[test]
    fn test_pow() {
        let two: ModInt = ModInt::from_value_modulus(
            BigInt::from(2),
            BigInt::from(20),
        );

        let four: ModInt = ModInt::from_value_modulus(
            BigInt::from(4),
            BigInt::from(20),
        );

        let result = two.pow(four);
        assert_eq!(BigInt::from(16), result.value);
        assert_eq!(BigInt::from(20), result.modulus);
    }

    #[test]
    fn test_pow_with_modulus() {
        let two: ModInt = ModInt::from_value_modulus(
            BigInt::from(2),
            BigInt::from(5),
        );

        let four: ModInt = ModInt::from_value_modulus(
            BigInt::from(4),
            BigInt::from(5),
        );

        let result = two.pow(four);
        assert_eq!(BigInt::from(1), result.value);
        assert_eq!(BigInt::from(5), result.modulus);
    }

    #[test]
    fn test_random() {
        let rnd: ModInt = ModInt::gen_modint(ModInt::one());

        assert!(rnd.value < BigInt::one());
        assert_eq!(BigInt::one(), rnd.modulus);
    }

    #[test]
    #[should_panic(expected = "the upper_bound must be greater than zero")]
    fn test_random_failing() {
        ModInt::gen_modint(ModInt::zero());
    }
}