use ::arithmetic::mod_int::ModInt;

/// # ElGamal CipherText.
#[derive(Eq, PartialEq, Clone, Debug, Hash, Serialize, Deserialize)]
pub struct CipherText {
    pub big_g: ModInt,
    pub big_h: ModInt,
    pub random: ModInt
}