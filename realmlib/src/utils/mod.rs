pub mod rc4;
pub mod rsa;

extern crate base64;
extern crate rand;

/// Wrapper for encoding text to b64 text
pub fn encode_base64(input: String) -> String {
    base64::encode(&input)
}

/// Wrapper for encoding Vec<u8> to b64 text
pub fn encode_base64_bytes(input: Vec<u8>) -> String {
    base64::encode(&input)
}

/// Wrapper for decoding b64 text to text
pub fn decode_base64(input: String) -> String {
    String::from_utf8_lossy(base64::decode(&input).unwrap().as_ref()).to_string()
}

/// Wrapper for decoding b64 text to Vec<u8>
pub fn decode_base64_bytes(input: String) -> Vec<u8> {
    base64::decode(&input).unwrap()
}

/// High level wrapper for rsa encrypting strings
pub fn encrypt_string(input: &String) -> String {
    encode_base64_bytes(rsa::crypt(input))
}

/// Encrypts the string with RSA pubkey, but doesn't b64 encode the result
pub fn encrypt_string_debug(input: &String) -> String {
    String::from_utf8(rsa::crypt(input)).expect("Bad decode")
}

/// Generates a random i32 that is greater than 0
pub fn rand_i32() -> i32 {
    let x = rand::random::<i32>();
    if x > 0 {
        return x;
    } else {
        return rand_i32();
    }
}
