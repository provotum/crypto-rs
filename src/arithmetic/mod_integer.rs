use ::num::bigint::BigInt;
use ::num::bigint::RandBigInt;
use ::num::pow::Pow;
use ::num::pow;
use ::rand;
use ::num::Zero; // TODO: use also for ModInteger
use ::num::One;
use ::num::Signed;
use ::num::ToPrimitive;
use ::std::cmp::PartialEq;
use ::arithmetic::mod_inverse;

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
// TODO
//use std::cmp::Ordering::{self, Less, Greater, Equal};

pub fn random(upper_bound: BigInt) -> BigInt {
    let mut rng = rand::thread_rng();

    // no semicolon -> return value
    rng.gen_bigint_range(&BigInt::zero(), &upper_bound)
}

pub struct ModInteger {
    pub value: BigInt,
    pub modulus: BigInt
}

pub trait From {
    fn from_value_modulus(value: BigInt, modulus: BigInt) -> ModInteger;
    fn from_value(value: BigInt) -> ModInteger;
}

impl From for ModInteger {
    fn from_value_modulus(value: BigInt, modulus: BigInt) -> ModInteger {
        ModInteger {
            value,
            modulus
        }
    }

    fn from_value(value: BigInt) -> ModInteger {
        ModInteger {
            value,
            modulus: BigInt::zero()
        }
    }
}

pub trait Finalize {
    fn finalize(self) -> ModInteger;
}


impl Finalize for ModInteger {
    fn finalize(mut self) -> ModInteger {
        if self.modulus > BigInt::zero() {
            self.value = self.value.rem(self.modulus.clone());
        }

        self
    }
}

impl PartialEq<ModInteger> for ModInteger {
    fn eq(&self, other: &ModInteger) -> bool {
        // TODO: find a way to finalize
        self.value.eq(&other.value) && self.modulus.eq(&other.modulus)
    }

    fn ne(&self, other: &ModInteger) -> bool {
        ! self.eq(other)
    }
}

impl Zero for ModInteger {

    /// # Zero ModInteger
    ///
    /// Returns a ModInteger having both the value and
    /// its modulus set to zero.
    fn zero() -> Self {
        ModInteger {
            value: BigInt::zero(),
            modulus: BigInt::zero()
        }
    }

    fn is_zero(&self) -> bool {
        self.value.eq(&BigInt::zero())
    }
}


impl One for ModInteger {

    /// # One ModInteger
    ///
    /// Returns a ModInteger having the value set to one and
    /// its modulus set to zero.
    fn one() -> Self {
        ModInteger {
            value: BigInt::one(),
            modulus: BigInt::zero()
        }
    }

    fn is_one(&self) -> bool where Self: PartialEq {
        // TODO: finalize
        self.value.eq(&BigInt::one())
    }
}

// Negation of ModIntegers
impl Neg for ModInteger {
    type Output = ModInteger;

    #[inline]
    fn neg(mut self) -> ModInteger {
        self = self.finalize();

        let zero = BigInt::zero();

        if self.modulus.eq(&zero) {
            self.value = self.value.neg()
        } else {
            self.value = self.modulus.clone().sub(self.value.clone());
        }

        self.finalize()
    }
}

impl Add<ModInteger> for ModInteger {
    type Output = ModInteger;

    #[inline]
    fn add(mut self, rhs: ModInteger) -> ModInteger {
        // TODO: apply modulus after add to avoid  overflows
        self.value = self.value.add(rhs.value);

        self.finalize()
    }
}

impl Sub<ModInteger> for ModInteger {
    type Output = ModInteger;

    #[inline]
    fn sub(mut self, rhs: ModInteger) -> ModInteger {
        let zero = BigInt::zero();

        if self.modulus.eq(&zero) {
            self.value = self.value.sub(rhs.value)
        } else {
            self.value = self.value.add(rhs.neg().value)
        }

        self.finalize()
    }
}

impl Mul<ModInteger> for ModInteger {
    type Output = ModInteger;

    #[inline]
    fn mul(mut self, rhs: ModInteger) -> ModInteger {
        self.value = self.value.mul(rhs.value);

        self.finalize()
    }
}

impl Div<ModInteger> for ModInteger {
    type Output = ModInteger;

    #[inline]
    fn div(mut self, rhs: ModInteger) -> ModInteger {
        let zero = BigInt::zero();

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

        self.finalize()
    }
}

impl Rem<ModInteger> for ModInteger {
    type Output = ModInteger;

    #[inline]
    fn rem(mut self, rhs: ModInteger) -> ModInteger {
        self.value = self.value.rem(rhs.modulus);

        self
    }
}

impl Pow<ModInteger> for ModInteger {
    type Output = ModInteger;

    #[inline]
    fn pow(mut self, rhs: ModInteger) -> ModInteger {
        let zero = BigInt::zero();

        if self.modulus.eq(&zero) {
            let usize_val: usize;
            let result = rhs.value.to_usize();

            match result {
                Some(x) => usize_val = x,
                None => panic!("Failed to convert BigInt to usize")
            }

            self.value = pow(self.value, usize_val)
        } else {
            let inv: BigInt = rhs.value.modpow(&rhs.value, &self.modulus);

            self.value = inv;
        }

        self.finalize()
    }
}

