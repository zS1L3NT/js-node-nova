use {
    aes_gcm::{
        aead::{AeadInPlace, KeyInit, Result},
        Aes256Gcm, Nonce,
    },
    std::{env::var, str},
};

pub fn validate(key: &String) -> bool {
    let encrypted_key = var("AES__ENCRYPTED_KEY").unwrap();
    match decrypt(&encrypted_key, key) {
        Ok(data) => &data == key,
        Err(_) => false,
    }
}

pub fn encrypt(data: &String, key: &str) -> Result<String> {
    let key = key.repeat(32);
    let key = key[0..32].as_bytes();
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce = var("AES__NONCE").unwrap();
    let nonce = Nonce::from_slice(nonce.as_bytes());

    let mut buffer = data.as_bytes().to_vec();
    cipher.encrypt_in_place(nonce, &[], &mut buffer)?;
    Ok(base64::encode(buffer))
}

pub fn decrypt(data: &String, key: &str) -> Result<String> {
    let key = key.repeat(32);
    let key = key[0..32].as_bytes();
    let cipher = Aes256Gcm::new_from_slice(key).unwrap();
    let nonce = var("AES__NONCE").unwrap();
    let nonce = Nonce::from_slice(nonce.as_bytes());

    let mut buffer = base64::decode(data).unwrap();
    cipher.decrypt_in_place(nonce, &[], &mut buffer)?;
    Ok(str::from_utf8(&buffer).unwrap().to_string())
}
