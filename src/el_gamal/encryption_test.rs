use ::arithmetic::mod_int::From;
use ::arithmetic::mod_int::ModInt;
use ::el_gamal::encryption::{decrypt, encrypt, PrivateKey, PublicKey};
use ::num::BigInt;
use ::num::One;
use ::num::Zero;

#[test]
fn encrypt_decrypt() {
    let message: ModInt = ModInt::one();

    let priv_key: PrivateKey = PrivateKey {
        p: ModInt::from_value_modulus(BigInt::from(5), BigInt::zero()),
        q: ModInt::from_value_modulus(BigInt::from(2), BigInt::zero()),
        g: ModInt::from_value_modulus(BigInt::from(2), BigInt::zero()),
        x: ModInt::from_value_modulus(BigInt::from(5), BigInt::zero())
    };

    //h := (g^x) mod p
    //2 := 2^5 mod 5
    let pub_key: PublicKey = PublicKey {
        p: ModInt::from_value_modulus(BigInt::from(5), BigInt::zero()),
        q: ModInt::from_value_modulus(BigInt::from(2), BigInt::zero()),
        h: ModInt::from_value_modulus(BigInt::from(32), BigInt::from(5)),
        g: ModInt::from_value_modulus(BigInt::from(2), BigInt::from(5))
    };

    let c = encrypt(&pub_key, message);

    let result_message = decrypt(priv_key, c);

    assert_eq!(ModInt::one().value, result_message.value);
}