extern crate openssl;
extern crate rand;

use openssl::rsa::{Padding, Rsa};

pub const PUB_KEY: &str = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCKFctVrhfF3m2Kes0FBL/JFeO
cmNg9eJz8k/hQy1kadD+XFUpluRqa//Uxp2s9W2qE0EoUCu59ugcf/p7lGuL99Uo
SGmQEynkBvZct+/M40L0E0rZ4BVgzLOJmIbXMp0J4PnPcb6VLZvxazGcmSfjauC7
F3yWYqUbZd/HCBtawwIDAQAB
-----END PUBLIC KEY-----";

/// Lower level implementation for encrypting a string with a public key
pub fn crypt(input: &String) -> Vec<u8> {
    let pkey = match Rsa::public_key_from_pem(PUB_KEY.as_bytes()) {
        Ok(v) => v,
        Err(e) => panic!("Error making pkey: {}", e),
    };
    let mut ciphertext = vec![0; pkey.size() as usize];
    match pkey.public_encrypt(input.as_bytes(), &mut ciphertext, Padding::PKCS1) {
        Ok(_) => return ciphertext,
        Err(e) => panic!("Error ciphering rsa: {}", e),
    };
}
