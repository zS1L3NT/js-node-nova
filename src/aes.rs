pub fn validate(key: &String) -> bool {
    let encrypted_key = option_env!("AES__ENCRYPTED_KEY").unwrap();
    match decrypt(&encrypted_key.into(), key) {
        Ok(data) => &data == key,
        Err(_) => false,
    }
}

pub fn encrypt(data: &String, key: &str) -> aes_gcm::aead::Result<String> {
    let key = key.repeat(32);
    let key = key[0..32].as_bytes();
    let cipher = <aes_gcm::Aes256Gcm as aes_gcm::KeyInit>::new_from_slice(key).unwrap();
    let nonce = option_env!("AES__NONCE").unwrap();
    let nonce = aes_gcm::Nonce::from_slice(nonce.as_bytes());

    let mut buffer = data.as_bytes().to_vec();
    aes_gcm::AeadInPlace::encrypt_in_place(&cipher, nonce, &[], &mut buffer)?;
    Ok(base64::encode(buffer))
}

pub fn decrypt(data: &String, key: &str) -> aes_gcm::aead::Result<String> {
    let key = key.repeat(32);
    let key = key[0..32].as_bytes();
    let cipher = <aes_gcm::Aes256Gcm as aes_gcm::KeyInit>::new_from_slice(key).unwrap();
    let nonce = option_env!("AES__NONCE").unwrap();
    let nonce = aes_gcm::Nonce::from_slice(nonce.as_bytes());

    let mut buffer = base64::decode(data).unwrap();
    aes_gcm::AeadInPlace::decrypt_in_place(&cipher, nonce, &[], &mut buffer)?;
    Ok(std::str::from_utf8(&buffer).unwrap().to_string())
}
