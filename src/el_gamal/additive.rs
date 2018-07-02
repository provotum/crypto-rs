use ::el_gamal::ciphertext::CipherText;

/// # Homomorphic Operation
///
/// Operate in a homomorphic way on the given cipher text.
pub trait Operate {
    fn operate(self, cipher_text: CipherText) -> CipherText;
}

impl Operate for CipherText {

    fn operate(self, cipher_text: CipherText) -> CipherText {
        CipherText {
            big_g: self.big_g * cipher_text.big_g,
            big_h: self.big_h * cipher_text.big_h,
            random: self.random + cipher_text.random
        }
    }
}
