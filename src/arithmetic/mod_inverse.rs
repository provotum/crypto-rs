use ::num::bigint::BigInt;
use ::num::Zero;
use ::num::One;

///
/// # Modular Inverse
///
/// Calculates the modular inverse `a^-1 mod m`
///
/// ## Credits
/// Inspired by [simon-andrews/rust-modinverse](https://github.com/simon-andrews/rust-modinverse)
///
pub fn mod_inverse(a: BigInt, m: BigInt) -> Option<BigInt> {
    let (g, x, _) = extended_gcd(a.clone(), m.clone());
    if g != BigInt::one() {
        return None;
    } else {
        // actually use the modulus instead of the remainder
        // operator "%" which behaves differently for negative values
        // -> https://stackoverflow.com/questions/31210357/is-there-a-modulus-not-remainder-function-operation
        let modulus: BigInt = (x % m.clone()) + m;

        return Some(modulus);
    }
}

fn extended_gcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    assert!(a < b);
    if a == BigInt::zero() {
        return (b, BigInt::zero(), BigInt::one());
    } else {
        let (g, x, y) = extended_gcd(b.clone() % a.clone(), a.clone());
        return (g, y - (b / a) * x.clone(), x.clone());
    }
}
