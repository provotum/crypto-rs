use ::arithmetic::mod_int::From;
use ::arithmetic::mod_int::ModInt;
use ::arithmetic::mod_int::RandModInt;
use ::el_gamal::ciphertext::CipherText;
use ::num::traits::Pow;
use ::num::Zero;
use ::num::One;

pub struct PublicKey {
    pub p: ModInt,
    pub q: ModInt,
    pub h: ModInt,
    pub g: ModInt,
}

pub struct PrivateKey {
    pub p: ModInt,
    pub q: ModInt,
    pub g: ModInt,
    pub x: ModInt,
}


pub fn encrypt(public_key: &PublicKey, message: ModInt) -> CipherText {
    //let random_secret: ModInt = ModInt::gen_modint(public_key.g.clone());
    let random_secret: ModInt = ModInt::one();

    // TODO: there must be a better way instead of cloning

    let g = public_key.g.clone();
    let h = public_key.h.clone();

    let big_g = g.clone().pow(random_secret.clone());
    let big_h1= h.clone().pow(random_secret.clone());
    let big_h2 = g.clone().pow(message.clone());

    let big_h = big_h1 * big_h2;

    let random = random_secret.clone();

    CipherText {
        big_g: g.clone().pow(random_secret.clone()),
        big_h: h.clone().pow(random_secret.clone()) * g.clone().pow(message.clone()),
        random: random_secret.clone(),
    }
}

pub fn decrypt(private_key: PrivateKey, cipher_text: CipherText) -> ModInt {
    // cipherText.getH().divide(cipherText.getG().pow(privateKey.getX()));

    let h: &ModInt = &cipher_text.big_h;
    let g: &ModInt = &cipher_text.big_g;
    let x: &ModInt = &private_key.x;

    let g_to_m: ModInt = h.clone() / (g.clone().pow(x.clone()));

    let mut i: ModInt = ModInt::zero();
    loop {
        let target: ModInt = ModInt::from_value_modulus(private_key.g.value.clone(), g_to_m.modulus.clone()).pow(i.clone());

        if target.eq(&g_to_m) {
            return i;
        }

        i = i + ModInt::one();
    }
}
