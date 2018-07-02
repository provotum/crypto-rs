use ::arithmetic::mod_inverse;
use ::num::bigint::BigInt;
use ::num::bigint::RandBigInt;
use ::num::One;
use ::num::pow;
use ::num::pow::Pow;
use ::num::ToPrimitive;
use ::num::Zero;
use ::rand;
use ::std::cmp::Ordering;
use ::std::cmp::PartialEq;
use ::std::cmp::PartialOrd;
use ::std::ops::{Add, Div, Mul, Neg, Rem, Sub};

// TODO
//use std::cmp::Ordering::{self, Less, Greater, Equal};


/// An integer with modular operations.
pub struct ModInt {
    /// The value.
    pub value: BigInt,
    /// The modulus.
    pub modulus: BigInt,
}

pub trait From {
    /// Create a ModInt with the given value and modulus.
    fn from_value_modulus(value: BigInt, modulus: BigInt) -> ModInt;

    /// Create a ModInt with the given value and a zero modulus.
    fn from_value(value: BigInt) -> ModInt;
}

impl From for ModInt {
    fn from_value_modulus(value: BigInt, modulus: BigInt) -> ModInt {
        ModInt {
            value,
            modulus,
        }
    }

    fn from_value(value: BigInt) -> ModInt {
        ModInt {
            value,
            modulus: BigInt::zero(),
        }
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

            return normalized_val.eq(&other.value) && self.modulus.eq(&other.modulus);
        } else {
            return self.value.eq(&other.value) && self.modulus.eq(&other.modulus);
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

            self.value = pow(self.value, usize_val)
        } else {
            let inv: BigInt = rhs.value.modpow(&rhs.value, &self.modulus);

            self.value = inv;
        }

        self.normalize()
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
