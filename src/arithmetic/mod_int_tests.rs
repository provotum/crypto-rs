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
#[should_panic]
fn test_invalid_div() {
    let one: ModInt = ModInt::one();
    let zero: ModInt = ModInt::zero();

    one / zero;
}

#[test]
#[should_panic(expected = "failed to compute inverse")]
fn test_invalid_div_modulus() {
    let one: ModInt = ModInt::from_value_modulus(
        BigInt::one(),
        BigInt::from(5)
    );
    let zero: ModInt = ModInt::from_value_modulus(
        BigInt::zero(),
        BigInt::from(5)
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
#[should_panic(expected="the upper_bound must be greater than zero")]
fn test_random_failing() {
    ModInt::gen_modint(ModInt::zero());
}