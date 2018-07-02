use ::num::bigint::BigInt;
use ::num::Zero;
use ::num::One;

pub fn egcd(a: BigInt, b: BigInt) -> (BigInt, BigInt, BigInt) {
    assert!(a < b);
    if a == BigInt::zero() {
        return (b, BigInt::zero(), BigInt::one());
    } else {
        let (g, x, y) = egcd(b.clone() % a.clone(), a.clone());
        return (g, y - (b / a) * x.clone(), x.clone());
    }
}

pub fn modinverse(a: BigInt, m: BigInt) -> Option<BigInt> {
    let (g, x, _) = egcd(a.clone(), m.clone());
    if g != BigInt::one() {
        return None;
    } else {
        let modulus: BigInt = (x % m.clone()) + m;

        return Some(modulus);
        //return Some(x % m);
    }
}