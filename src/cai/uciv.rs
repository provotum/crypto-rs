use std::vec::Vec;
use num::pow::Pow;
use num::Zero;

use std::ops::{Mul, Div, Sub, Add, Neg};

use ::arithmetic::mod_int::From;
use ::arithmetic::mod_int::RandModInt;
use ::arithmetic::mod_int::ModInt;
use ::el_gamal::encryption::PublicKey;
use ::el_gamal::ciphertext::CipherText;
use ::el_gamal::serializer::Serializer;

/// Secret UCIV Information (x1, x2, ..., xn)
#[derive(Clone)]
struct PreImageSet {
    pub pre_images: Vec<ModInt>
}

/// Public UCIV Information (y1, y2, ..., yn)
#[derive(Clone)]
struct ImageSet {
    pub images: Vec<ModInt>
}

impl ImageSet {
    /// Creates an ImageSet `(y1, y2, ..., yn)` by applying the following
    /// arithmetic operation to each element of the given PreImageSet `(x1, x2, ..., xn)`:
    ///
    /// ```
    /// (y1, y2, ..., yn) = ( F(x1), F(x2), ..., F(xn) )
    ///
    /// (y1, y2, ..., yn) = ( generator^x1, generator^x2, ..., generator^xn)
    /// ```
    ///
    /// - generator: The generator of the cyclic group used also during encryption of the vote
    /// - chosen_vote_idx: The index of the vote when mapped to the pre_image_set
    /// - pre_image_set: A number of pre-images equal to the number of voting options available
    ///                  which are specific to a particular voter.
    ///
    pub fn new(generator: ModInt, chosen_vote_idx: usize, pre_image_set: PreImageSet) -> Self {
        let mut vec = vec![];

        // apply g^x as one-way function
        let mut cntr: usize = 0;
        for pre_image in pre_image_set.pre_images.iter() {
            // If modulus is not equal, then we will end up with a different one after the exponentiation
            assert_eq!(pre_image.modulus.clone(), generator.modulus.clone(), "Modulus of pre-image and generator must be equal");

            if cntr == chosen_vote_idx {
                vec.push(ModInt::zero());
            } else {
                vec.push(generator.clone().pow(pre_image.clone()));
            }

            cntr += 1;
        }

        ImageSet {
            images: vec
        }
    }
}

struct CaiProof {
    s1_options: Vec<ModInt>,
    s2_options: Vec<ModInt>,
    c1_options: Vec<ModInt>,
    c2_options: Vec<ModInt>,
    h1_options: Vec<ModInt>,
    h2_options: Vec<ModInt>,

    a_options: Vec<ModInt>,
    b_options: Vec<ModInt>,

    h: ModInt
}

impl CaiProof {
    pub fn new(public_key: PublicKey, cipher_text: CipherText, pre_image_set: PreImageSet, image_set: ImageSet, chosen_vote_idx: usize, voting_options: Vec<ModInt>) -> Self {
        assert_eq!(pre_image_set.pre_images.len(), image_set.images.len(), "The amount of pre-images and images must be equal");
        assert_eq!(pre_image_set.pre_images.len(), voting_options.len(), "The amount of pre-images must be equal to the amount of voting options");
        assert!(chosen_vote_idx < pre_image_set.pre_images.len(), "The chosen vote index must refer to a voting option for which a pre-image exists");

        // initialize vector with the amount of pre_images which are
        // equal to the number of voting options
        let mut s1_options: Vec<ModInt> = vec![ModInt::zero(); pre_image_set.pre_images.len()];
        let mut s2_options: Vec<ModInt> = vec![ModInt::zero(); pre_image_set.pre_images.len()];
        let mut c1_options: Vec<ModInt> = vec![ModInt::zero(); pre_image_set.pre_images.len()];
        let mut c2_options: Vec<ModInt> = vec![ModInt::zero(); pre_image_set.pre_images.len()];
        let mut h1_options: Vec<ModInt> = vec![ModInt::zero(); pre_image_set.pre_images.len()];
        let mut h2_options: Vec<ModInt> = vec![ModInt::zero(); pre_image_set.pre_images.len()];

        let mut a_options: Vec<ModInt> = vec![ModInt::zero(); pre_image_set.pre_images.len()];
        let mut b_options: Vec<ModInt> = vec![ModInt::zero(); pre_image_set.pre_images.len()];

        let j = chosen_vote_idx;

        let c1 = cipher_text.big_g;
        let c2 = cipher_text.big_h;

        let mut string_to_hash = String::new();
        string_to_hash += &c1.to_string();
        string_to_hash += &c2.to_string();

        // TODO: add c3, c4

        for i in 0..pre_image_set.pre_images.len() {
            if i != j {
                // case 1: all not-chosen options

                // generate random values
                let s1_i = ModInt::gen_modint(public_key.q.clone());
                let h1_i = ModInt::gen_modint(public_key.q.clone());
                s1_options[i] = s1_i.clone();
                h1_options[i] = h1_i.clone();

                // the specific values for each voting options
                let c1_i = public_key.g.clone().pow(s1_i.clone()).mul(c1.clone().pow(h1_i.clone().neg()));
                let c2_i = public_key.h.clone().pow(s1_i.clone()).mul((c2.clone().div(public_key.g.clone().pow(voting_options[i].clone()))).pow(h1_i.clone().neg()));
                c1_options[i] = c1_i.clone();
                c2_options[i] = c2_i.clone();

                let a_i = ModInt::gen_modint(public_key.q.clone());
                a_options[i] = a_i.clone();
                let r_i = public_key.g.clone().pow(a_i);

                string_to_hash += &c1_i.to_string();
                string_to_hash += &c2_i.to_string();
                string_to_hash += &r_i.to_string();
            } else {
                // case 2: the chosen option

                let s2_j = ModInt::gen_modint(public_key.q.clone());
                let h2_j = ModInt::gen_modint(public_key.q.clone());
                s2_options[j] = s2_j.clone();
                h2_options[j] = h2_j.clone();

                let b_j = ModInt::gen_modint(public_key.q.clone());
                b_options[j] = b_j.clone();

                let c1_j = public_key.g.clone().pow(b_j.clone());
                let c2_j = public_key.h.clone().pow(b_j.clone());
                c1_options[j] = c1_j.clone();
                c2_options[j] = c2_j.clone();

                let r_j = public_key.g.clone().pow(s2_j).mul(image_set.images[j].clone().pow(h2_j.clone().neg()));

                string_to_hash += &c1_j.to_string();
                string_to_hash += &c2_j.to_string();
                string_to_hash += &r_j.to_string();
            }
        }

        let h_hash = Serializer::string_to_sha512(string_to_hash);
        let h = ModInt::from_hex_string(h_hash, public_key.q.value.clone());

        for i in 0..pre_image_set.pre_images.len() {
            if i != j {
                // case 1: all not-chosen options

                let h2_i = h.clone().sub(h1_options[i].clone());
                h2_options[i] = h2_i.clone();

                let s2_i = a_options[i].clone().add(pre_image_set.pre_images[i].clone().mul(h2_i.clone()));
                s2_options[i] = s2_i;

            } else {
                // case 2: the chosen option

                let h1_j = h.clone().sub(h2_options[j].clone());
                h1_options[j] = h1_j.clone();

                let s1_j = b_options[j].clone().add(cipher_text.random.clone().mul(h1_j.clone()));
                s1_options[j] = s1_j.clone();
            }
        }

        CaiProof {
            s1_options,
            s2_options,
            c1_options,
            c2_options,
            h1_options,
            h2_options,
            a_options,
            b_options,
            h
        }
    }

    pub fn verify(&self, public_key: PublicKey, cipher_text: CipherText, image_set: ImageSet, voting_options: Vec<ModInt>) -> bool {
        let c1 = cipher_text.big_g;
        let c2 = cipher_text.big_h;

        let mut string_to_hash = String::new();
        string_to_hash += &c1.to_string();
        string_to_hash += &c2.to_string();
        // TODO: c3, c4

        // reconstruct h
        for i in 0..self.s1_options.len() {
            let c1_i = public_key.g.clone().pow(self.s1_options[i].clone()).mul(c1.clone().pow(self.h1_options[i].clone().neg()));
            let c2_i = public_key.h.clone().pow(self.s1_options[i].clone()).mul((c2.clone().div(public_key.g.clone().pow(voting_options[i].clone()))).pow(self.h1_options[i].clone().neg()));
            let r_i = public_key.g.clone().pow(self.s2_options[i].clone()).mul(image_set.images[i].clone().pow(self.h2_options[i].clone().neg()));

            string_to_hash += &c1_i.to_string();
            string_to_hash += &c2_i.to_string();
            string_to_hash += &r_i.to_string();
        }

        let h_hash = Serializer::string_to_sha512(string_to_hash);
        let h = ModInt::from_hex_string(h_hash, public_key.q.value.clone());



        self.h == h
    }
}



#[cfg(test)]
mod membership_proof_test {

    use ::el_gamal::encryption::PublicKey;
    use ::el_gamal::encryption::{encrypt};
    use ::arithmetic::mod_int::ModInt;
    use arithmetic::mod_int::From;
    use ::num::bigint::BigInt;
    use ::num::Zero;
    use ::num::One;
    use std::vec::Vec;
    use std::clone::Clone;
    use ::cai::uciv::{CaiProof, ImageSet, PreImageSet};

    #[test]
    pub fn test_one_or_proof() {
        //h := (g^x) mod p
        //2 := 2^5 mod 5
        let pub_key: PublicKey = PublicKey {
            p: ModInt::from_value_modulus(BigInt::from(5), BigInt::zero()),
            q: ModInt::from_value_modulus(BigInt::from(2), BigInt::zero()),
            h: ModInt::from_value_modulus(BigInt::from(32), BigInt::from(5)),
            g: ModInt::from_value_modulus(BigInt::from(2), BigInt::from(5))
        };

        let mut voting_options = Vec::new();
        voting_options.push(ModInt::zero());
        voting_options.push(ModInt::one());

        let message: ModInt = ModInt {
            value: BigInt::one(),
            modulus: BigInt::from(5) // must be equal to the value p of the public key
        };
        let cipher_text = encrypt(&pub_key, message.clone());
        let chosen_vote_idx = 1;

        let pre_image_set = PreImageSet {
            pre_images: vec![
                ModInt::from_value_modulus(BigInt::from(1), BigInt::from(5)),
                ModInt::from_value_modulus(BigInt::from(0), BigInt::from(5))
            ]
        };

        let image_set = ImageSet::new(pub_key.g.clone(), chosen_vote_idx, pre_image_set.clone());

        let proof = CaiProof::new(
            pub_key.clone(),
            cipher_text.clone(),
            pre_image_set.clone(),
            image_set.clone(),
            chosen_vote_idx,
            voting_options.clone()
        );

        let is_proven = proof.verify(
            pub_key.clone(),
            cipher_text.clone(),
            image_set.clone(),
            voting_options.clone()
        );

        assert!(is_proven);
    }
}