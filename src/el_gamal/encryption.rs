use ::arithmetic::mod_int::From;
use ::arithmetic::mod_int::ModInt;
use ::arithmetic::mod_int::RandModInt;
use ::el_gamal::ciphertext::CipherText;
use num::traits::Pow;
use num::Zero;
use num::One;
use std::fs::File;
use std::io::Read;
use serde_json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    pub p: ModInt,
    pub q: ModInt,
    pub h: ModInt,
    pub g: ModInt,
}

impl PublicKey {
    /// Create a PublicKey based its string representation at a specific path.
    ///
    /// - `public_key_file_name`: The file name of the public key.
    ///                           Must reside in the same directory as the binary is launched.
    ///
    pub fn new(public_key_file_name: &str) -> Self {
        // Read the input file to string.
        let mut file = File::open("./".to_owned() + public_key_file_name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let public_key: PublicKey = match serde_json::from_str(&contents) {
            Ok(public_key_data) => {
                public_key_data
            },
            Err(e) => {
                panic!("Failed to transform file {:?} into PublicKey: {:?}", file, e);
            }
        };

        public_key
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrivateKey {
    pub p: ModInt,
    pub q: ModInt,
    pub g: ModInt,
    pub x: ModInt,
}

impl PrivateKey {
    /// Create a PrivateKey based its string representation at a specific path.
    ///
    /// - `private_key_file_name`: The file name of the private key.
    ///                            Must reside in the same directory as the binary is launched.
    ///
    pub fn new(private_key_file_name: &str) -> Self {
        // Read the input file to string.
        let mut file = File::open("./".to_owned() + private_key_file_name).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let private_key: PrivateKey = match serde_json::from_str(&contents) {
            Ok(private_key_data) => {
                private_key_data
            },
            Err(e) => {
                panic!("Failed to transform file {:?} into PrivatKey: {:?}", file, e);
            }
        };

        private_key
    }
}

pub fn encrypt(public_key: &PublicKey, message: ModInt) -> CipherText {
    let random: ModInt = ModInt::gen_modint(public_key.q.clone());

    let g = public_key.g.clone();
    let h = public_key.h.clone();

    let big_g = g.clone().pow(random.clone());
    let big_h1= h.clone().pow(random.clone());
    let big_h2 = g.clone().pow(message.clone());

    let big_h = big_h1 * big_h2;

    CipherText {
        big_g,
        big_h,
        random,
    }
}

pub fn decrypt(private_key: PrivateKey, cipher_text: CipherText) -> ModInt {

    let h: &ModInt = &cipher_text.big_h;
    let g: &ModInt = &cipher_text.big_g;
    let x: &ModInt = &private_key.x;

    let g_to_m: ModInt = h.clone() / (g.clone().pow(x.clone()));

    let mut i: ModInt = ModInt::zero();
    // find cleartext value so that it matches target
    loop {
        let target: ModInt = ModInt::from_value_modulus(private_key.g.value.clone(), g_to_m.modulus.clone()).pow(i.clone());

        if target.eq(&g_to_m) {
            return i;
        }

        i = i + ModInt::one();
    }
}


#[cfg(test)]
mod encryption_test {

    use ::el_gamal::encryption::PrivateKey;
    use ::el_gamal::encryption::PublicKey;
    use ::el_gamal::encryption::{encrypt, decrypt};
    use ::arithmetic::mod_int::ModInt;
    use arithmetic::mod_int::From;
    use ::num::bigint::BigInt;
    use ::num::Zero;
    use ::num::One;

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
}