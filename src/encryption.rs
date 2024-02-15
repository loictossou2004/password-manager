use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray
};
use hex::encode;

pub fn add_space(mystr: &str) -> String {
    let mut result = String::from(mystr);
    
    for _ in 0..(16 - mystr.len()) {
        result.push(' ');
    }
    result
}

pub fn encrypt_text(key_str: &str, input_text: &str) -> String {
    let key: [u8; 16] = key_str.as_bytes().try_into().expect("Key must be 16 bytes");

    let mut block = GenericArray::clone_from_slice(input_text.as_bytes());

    let cipher = Aes128::new(GenericArray::from_slice(&key));


    cipher.encrypt_block(&mut block);

    encode(&block)
}

pub fn decrypt_text(key_str: &str, encrypted_text: &str) -> String {
    let key: [u8; 16] = key_str.as_bytes().try_into().expect("Key must be 16 bytes");

    let encrypted_bytes = hex::decode(encrypted_text).expect("Invalid hex string");

    let mut block = GenericArray::clone_from_slice(&encrypted_bytes);

    let cipher = Aes128::new(GenericArray::from_slice(&key));

    cipher.decrypt_block(&mut block);

    let decrypted_text = String::from_utf8_lossy(&block);

    decrypted_text.to_string()
}