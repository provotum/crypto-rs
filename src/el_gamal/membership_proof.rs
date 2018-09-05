use ::arithmetic::mod_int::ModInt;
use ::arithmetic::mod_int::RandModInt;
use ::el_gamal::ciphertext::CipherText;
use ::el_gamal::encryption::{PrivateKey, PublicKey};
use arithmetic::mod_int::From;
use ::num::bigint::BigInt;
use num::{One, Zero};
use num::traits::Pow;
use rustc_serialize::hex::ToHex;
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

        //let t = ModInt::gen_modint(public_key.q.clone());
        let t = ModInt {
            value: BigInt::from(0),
            modulus: BigInt::from(2)
        };

        println!("t: {:?}", t.clone());


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
            println!("Domainval: {:?}", domain_val);

            if domain_val.eq(&plain_text) {
                println!("Domain value is equal");
                // we need to add fake values
                s_response.push(ModInt::zero());
                c_response.push(ModInt::zero());

                println!("s: {:?}", ModInt::zero());
                println!("c: {:?}", ModInt::zero());

                y = g.clone().pow(t.clone());
                z = h.clone().pow(t.clone());

                println!("y: {:?}", y.clone());
                println!("z: {:?}", z.clone());

                message_idx = i;

                println!("message_idx: {:?}", i);
            } else {
                // add fake commitments as well as the corresponding response
                // for a value which is not the plaintext message
                //let s = ModInt::gen_modint(public_key.q.clone());
                let s = ModInt {
                    value: BigInt::from(1),
                    modulus: BigInt::from(2)
                };
                //let c = ModInt::gen_modint(public_key.q.clone());
                let c = ModInt {
                    value: BigInt::from(0),
                    modulus: BigInt::from(2)
                };

                s_response.push(s.clone());
                c_response.push(c.clone());

                println!("s: {:?}", s.clone());
                println!("c: {:?}", c.clone());


                let neg_c = c.neg();
                println!("neg_c: {:?}", neg_c);
                let g_pow = g.clone().pow(domain_val.clone());
                println!("g_pow: {:?}", g_pow);

                y = g.clone().pow(s.clone()).mul(cipher_text.big_g.clone().pow(neg_c.clone()));
                z = h.clone().pow(s.clone()).mul(cipher_text.big_h.clone().div(g_pow).pow(neg_c.clone()));

                println!("y: {:?}", y.clone());
                println!("z: {:?}", z.clone());
            }

            y_response.push(y.clone());
            z_response.push(z.clone());

            string_to_hash += &y.to_string();
            string_to_hash += &z.to_string();
        }

        println!("Plain: {:?}", string_to_hash.clone());
        let c_hash = Serializer::string_to_sha512(string_to_hash);
        println!("c_hash: {:?}", c_hash);

        let mut c_0 = ModInt::from_hex_string(c_hash, public_key.q.value.clone());
        println!("c_0 from hex string: {:?}", c_0.clone());

        for fake_c in c_response.clone() {
            c_0 = c_0.sub(fake_c);
        }

        println!("c_0 after subtracting: {:?}", c_0.clone());

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
        println!("Verifying----------------------------------------");
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
            println!("domain_val {:?}", domain_val.clone());
            let g_pow = g.clone().pow(domain_val.clone());
            println!("g_pow: {:?}", g_pow.clone());

            let s: ModInt = (*self.s_responses.get(i).unwrap()).clone();
            let c: ModInt = (*self.c_responses.get(i).unwrap()).clone();
            let neg_c = c.clone().neg();

            println!("s: {:?}", s.clone());
            println!("c: {:?}", c.clone());
            println!("neg_c: {:?}", neg_c.clone());

            c_choices = c_choices.add(c.clone());

            let y = g.clone().pow(s.clone()).mul(cipher_text.big_g.clone().pow(neg_c.clone()));
            let z = h.clone().pow(s.clone()).mul(cipher_text.big_h.clone().div(g_pow).pow(neg_c.clone()));

            println!("y: {:?}", y.clone());
            println!("z: {:?}", z.clone());

            string_to_hash += &y.to_string();
            string_to_hash += &z.to_string();
        }

        println!("Restored Plain: {:?}", string_to_hash.clone());
        let c_hash: String = Serializer::string_to_sha512(string_to_hash);
        println!("Restored c_hash: {:?}", c_hash);
        let new_c = ModInt::from_hex_string(c_hash, self.q.value.clone());
        println!("expected c: {:?}, actual c: {:?}", c_choices.clone(), new_c.clone());

        return c_choices.eq(&new_c);
    }
}

#[cfg(test)]
mod membership_proof_test {

    use ::el_gamal::encryption::PrivateKey;
    use ::el_gamal::encryption::PublicKey;
    use ::el_gamal::encryption::{encrypt, decrypt};
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

//        println!("priv: {:?}", priv_key);
//        println!("pub: {:?}", pub_key);
//        println!("cipher: {:?}", cipher_text);
//        println!("domains: {:?}", domains);
//        println!("proof: {:?}", proof);
//        println!("is_proven: {:?}", is_proven);

        assert!(is_proven);
    }
}
