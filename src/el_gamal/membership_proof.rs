use ::arithmetic::mod_int::ModInt;
use ::arithmetic::mod_int::RandModInt;
use ::el_gamal::ciphertext::CipherText;
use ::el_gamal::encryption::{PublicKey};
use arithmetic::mod_int::From;
use num::bigint::BigInt;
use num::{Zero};
use num::traits::Pow;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::vec::Vec;
use std::ops::Add;
use ::el_gamal::serializer::Serializer;

#[derive(Clone, Debug)]
pub struct MembershipProof {
    s_responses: Vec<ModInt>,
    c_responses: Vec<ModInt>,
    y_responses: Vec<ModInt>,
    z_responses: Vec<ModInt>,

    p: ModInt,
    q: ModInt,
}

impl MembershipProof {
    pub fn new(public_key: PublicKey, plain_text: ModInt, cipher_text: CipherText, domains: Vec<ModInt>) -> MembershipProof {
        let mut y_response: Vec<ModInt> = vec![];
        let mut z_response: Vec<ModInt> = vec![];
        let mut s_response: Vec<ModInt> = vec![];
        let mut c_response: Vec<ModInt> = vec![];

        let g = ModInt {
            value: public_key.g.value.clone(),
            modulus: public_key.p.value.clone(),
        };

        let h = ModInt {
            value: public_key.h.value.clone(),
            modulus: public_key.p.value.clone(),
        };

        let t = ModInt::gen_modint(public_key.q.clone());

        let mut string_to_hash = String::new();
        string_to_hash += &g.to_string();
        string_to_hash += &h.to_string();
        string_to_hash += &cipher_text.big_g.to_string();
        string_to_hash += &cipher_text.big_h.to_string();

        let mut message_idx = 0;
        for i in 0..domains.len() {
            let mut y: ModInt;
            let mut z: ModInt;

            let domain_val: ModInt = (*domains.get(i).unwrap()).clone();

            if domain_val.eq(&plain_text) {
                // we need to add fake values
                s_response.push(ModInt::zero());
                c_response.push(ModInt::zero());

                y = g.clone().pow(t.clone());
                z = h.clone().pow(t.clone());

                message_idx = i;
            } else {
                // add fake commitments as well as the corresponding response
                // for a value which is not the plaintext message
                let s = ModInt::gen_modint(public_key.q.clone());
                let c = ModInt::gen_modint(public_key.q.clone());

                s_response.push(s.clone());
                c_response.push(c.clone());

                let neg_c = c.neg();
                let g_pow = g.clone().pow(domain_val.clone());

                y = g.clone().pow(s.clone()).mul(cipher_text.big_g.clone().pow(neg_c.clone()));
                z = h.clone().pow(s.clone()).mul(cipher_text.big_h.clone().div(g_pow).pow(neg_c.clone()));
            }

            y_response.push(y.clone());
            z_response.push(z.clone());

            string_to_hash += &y.to_string();
            string_to_hash += &z.to_string();
        }

        let c_hash = Serializer::string_to_sha512(string_to_hash);
        let mut c_0 = ModInt::from_hex_string(c_hash, public_key.q.value.clone());

        for fake_c in c_response.clone() {
            c_0 = c_0.sub(fake_c);
        }

        s_response[message_idx] = c_0.clone().mul(cipher_text.random.clone()).add(t.clone());
        c_response[message_idx] = c_0;

        MembershipProof {
            s_responses: s_response,
            c_responses: c_response,
            y_responses: y_response,
            z_responses: z_response,
            p: public_key.p,
            q: public_key.q,
        }
    }

    pub fn verify(&self, public_key: PublicKey, cipher_text: CipherText, domain: Vec<ModInt>) -> bool {
        if domain.len() < self.c_responses.len() || domain.len() < self.s_responses.len() {
            // The domain of the message is bigger than specified.
            // Therefore, the proof that the message is within the given domain is invalid.
            panic!("Domain has not the same length as the values of the proof.")
        }

        let g = ModInt {
            value: public_key.g.value.clone(),
            modulus: public_key.p.value.clone(),
        };

        let h = ModInt {
            value: public_key.h.value.clone(),
            modulus: public_key.p.value.clone(),
        };

        let mut c_choices = ModInt {
            value: BigInt::zero(),
            modulus: public_key.q.value.clone()
        };

        let mut string_to_hash = String::new();
        string_to_hash += &g.to_string();
        string_to_hash += &h.to_string();
        string_to_hash += &cipher_text.big_g.to_string();
        string_to_hash += &cipher_text.big_h.to_string();

        for i in 0..self.c_responses.len() {
            let domain_val = domain.get(i).unwrap();
            let g_pow = g.clone().pow(domain_val.clone());

            let s: ModInt = (*self.s_responses.get(i).unwrap()).clone();
            let c: ModInt = (*self.c_responses.get(i).unwrap()).clone();
            let neg_c = c.clone().neg();

            c_choices = c_choices.add(c.clone());

            let y = g.clone().pow(s.clone()).mul(cipher_text.big_g.clone().pow(neg_c.clone()));
            let z = h.clone().pow(s.clone()).mul(cipher_text.big_h.clone().div(g_pow).pow(neg_c.clone()));

            string_to_hash += &y.to_string();
            string_to_hash += &z.to_string();
        }

        let c_hash: String = Serializer::string_to_sha512(string_to_hash);
        let new_c = ModInt::from_hex_string(c_hash, self.q.value.clone());

        return c_choices.eq(&new_c);
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
    use ::el_gamal::membership_proof::MembershipProof;
    use std::vec::Vec;
    use std::clone::Clone;

    #[test]
    pub fn test_one_or_proof() {
        let message: ModInt = ModInt {
            value: BigInt::one(),
            modulus: BigInt::from(5) // must be equal to the value p of the public key
        };

        //h := (g^x) mod p
        //2 := 2^5 mod 5
        let pub_key: PublicKey = PublicKey {
            p: ModInt::from_value_modulus(BigInt::from(5), BigInt::zero()),
            q: ModInt::from_value_modulus(BigInt::from(2), BigInt::zero()),
            h: ModInt::from_value_modulus(BigInt::from(32), BigInt::from(5)),
            g: ModInt::from_value_modulus(BigInt::from(2), BigInt::from(5))
        };

        let cipher_text = encrypt(&pub_key, message.clone());

        let mut domains = Vec::new();
        domains.push(ModInt::zero());
        domains.push(ModInt::one());


        let proof = MembershipProof::new(
            pub_key.clone(),
            message,
            cipher_text.clone(),
            domains.clone()
        );

        let is_proven = proof.verify(pub_key.clone(), cipher_text.clone(), domains.clone());

        assert!(is_proven);
    }

    #[test]
    pub fn test_zero_or_proof() {
        let message: ModInt = ModInt {
            value: BigInt::zero(),
            modulus: BigInt::from(5) // must be equal to the value p of the public key
        };

        //h := (g^x) mod p
        //2 := 2^5 mod 5
        let pub_key: PublicKey = PublicKey {
            p: ModInt::from_value_modulus(BigInt::from(5), BigInt::zero()),
            q: ModInt::from_value_modulus(BigInt::from(2), BigInt::zero()),
            h: ModInt::from_value_modulus(BigInt::from(32), BigInt::from(5)),
            g: ModInt::from_value_modulus(BigInt::from(2), BigInt::from(5))
        };

        let cipher_text = encrypt(&pub_key, message.clone());

        let mut domains = Vec::new();
        domains.push(ModInt::zero());
        domains.push(ModInt::one());


        let proof = MembershipProof::new(
            pub_key.clone(),
            message, // <- other message than encrypted
            cipher_text.clone(),
            domains.clone()
        );

        let is_proven = proof.verify(pub_key.clone(), cipher_text.clone(), domains.clone());

        assert!(is_proven);
    }
}
