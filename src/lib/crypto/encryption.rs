use std::{env, process};

use aes_gcm::{aead::Aead, Aes256Gcm, Key, NewAead, Nonce};

pub fn encrypt(text: &str, nonce: &str) -> Vec<u8> {
    let text_bytes = text.as_bytes();

    let enc_key = env::var("ENCRYPTION_KEY").unwrap_or_else(|err| {
        log::error!("{err}");
        process::exit(1)
    });
    let enc_key_bytes = enc_key.as_bytes();

    let key = Key::from_slice(enc_key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce.as_bytes());

    match cipher.encrypt(nonce, text_bytes) {
        Ok(cipher_bytes) => cipher_bytes,
        Err(err) => {
            log::error!("{err}");
            panic!("{err}")
        }
    }
}

pub fn decrypt(cipher_bytes: &[u8], nonce: &str) -> String {
    let enc_key = env::var("ENCRYPTION_KEY").unwrap_or_else(|err| {
        log::error!("{err}");
        process::exit(1)
    });
    let enc_key_bytes = enc_key.as_bytes();

    let key = Key::from_slice(enc_key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce.as_bytes());

    match cipher.decrypt(nonce, cipher_bytes) {
        Ok(ciphertext) => String::from_utf8_lossy(&ciphertext).to_string(),
        Err(err) => {
            log::error!("{err}");
            panic!("{err}")
        }
    }
}
