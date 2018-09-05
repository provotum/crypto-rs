use sha2::{Sha512, Digest};

pub struct Serializer {}

impl Serializer {
    pub fn string_to_sha512(string: String) -> String {
        // create a Sha512 object
        let mut hasher = Sha512::default();

        // write input message
        hasher.input(&string.as_bytes());

        let mut hex_string = String::new();
        for byte in hasher.result().iter() {
            hex_string += &format!("{:02x}", byte)
        }

        hex_string
    }
}

#[cfg(test)]
mod serializer_test {

    use ::el_gamal::serializer::Serializer;

    #[test]
    fn test_string_to_sha512_hex() {
        let result = Serializer::string_to_sha512("1234".to_string());

        assert_eq!(
            "d404559f602eab6fd602ac7680dacbfaadd13630335e951f097af3900e9de176b6db28512f2e000b9d04fba5133e8b1c6e8df59db3a8ab9d60be4b97cc9e81db".to_string(),
            result
        );
    }
}