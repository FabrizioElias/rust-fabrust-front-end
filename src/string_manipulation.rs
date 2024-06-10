use sha2::{Digest, Sha256};
use flate2::Compression;
use flate2::write::DeflateEncoder;
use std::io::{Read, Write};
use base64::prelude::*;
use rsa::{pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey}, pkcs8::{DecodePrivateKey, DecodePublicKey}, Pkcs1v15Encrypt};
use hmac::{Hmac, Mac};
use xxhash_rust::xxh3::xxh3_128;
use std::error;

pub fn hash_object<T: serde::Serialize + std::hash::Hasher>(object: &T) -> Result<String, Box<dyn error::Error>> {
    let bytes = bincode::serialize(object).expect("Error serializing object");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    //bytes.hash(&mut hasher);
    let hash = hasher.finalize();

    let url_safe_chars: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_!@#$%^&*()+=[]{}|;:,.<>?";
    let mut url_safe_string = String::new();

    for i in (0..hash.len()).step_by(2) {
        let index = ((hash[i] as u16) << 8 | hash[i + 1] as u16) as usize;
        url_safe_string.push(url_safe_chars[index % url_safe_chars.len()] as char);
    }

    Ok(url_safe_string)
}

pub fn hash_string(object: &str) -> Result<String, Box<dyn error::Error>> {
    let bytes = bincode::serialize(object).expect("Error serializing object");
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    //bytes.hash(&mut hasher);
    let hash = hasher.finalize();

    let url_safe_chars: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_!@#$%^&*()+=[]{}|;:,.<>?";
    let mut url_safe_string = String::new();

    for i in (0..hash.len()).step_by(2) {
        let index = ((hash[i] as u16) << 8 | hash[i + 1] as u16) as usize;
        url_safe_string.push(url_safe_chars[index % url_safe_chars.len()] as char);
    }

    Ok(url_safe_string)
}

pub fn hash_object_xxhash<T: serde::Serialize>(object: &T) -> Result<String, Box<dyn error::Error>> {
    let bytes = bincode::serialize(object).expect("Error serializing object");
    let hash = xxh3_128(&bytes).to_be_bytes();
    //hash.to_string()
 
    let url_safe_chars: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_!@#$%^&*()+=[]{}|;:,.<>?";
    let mut url_safe_string = String::new();

    for i in (0..hash.len()).step_by(2) {
        let index = ((hash[i] as u16) << 8 | hash[i + 1] as u16) as usize;
        url_safe_string.push(url_safe_chars[index % url_safe_chars.len()] as char);
    }

    Ok(url_safe_string)
}

pub fn hash_string_xxhash(object: &str) -> Result<String, Box<dyn error::Error>> {
    let bytes = bincode::serialize(object).expect("Error serializing object");
    let hash = xxh3_128(&bytes).to_be_bytes();
    //hash.to_string()
 
    let url_safe_chars: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_!@#$%^&*()+=[]{}|;:,.<>?";
    let mut url_safe_string = String::new();

    for i in (0..hash.len()).step_by(2) {
        let index = ((hash[i] as u16) << 8 | hash[i + 1] as u16) as usize;
        url_safe_string.push(url_safe_chars[index % url_safe_chars.len()] as char);
    }

    Ok(url_safe_string)
}

pub fn compress_object<T: std::fmt::Debug>(object: &T) -> Result<String, Box<dyn error::Error>> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    write!(encoder, "{:?}", object).unwrap();
    let compressed_data = encoder.finish().unwrap();
    Ok(BASE64_STANDARD.encode(compressed_data))
}

pub fn compress_string(object: &str) -> Result<String, Box<dyn error::Error>> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(object.as_bytes())?;
    let compressed_data = encoder.finish().unwrap();
    Ok(BASE64_STANDARD.encode(compressed_data))
}

pub fn compress_object_max<T: serde::Serialize>(object: &T) -> Result<String, Box<dyn error::Error>> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::new(10));
    let bytes = bincode::serialize(object)?;
    encoder.write_all(&bytes)?;
    let compressed_data = encoder.finish().unwrap();
    Ok(BASE64_STANDARD.encode(compressed_data))
}

pub fn compress_object_hex<T: serde::Serialize>(object: &T) -> Result<String, Box<dyn error::Error>> {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    let bytes = bincode::serialize(object)?;
    encoder.write_all(&bytes)?;
    let compressed_data = encoder.finish().unwrap();
    Ok(hex::encode(compressed_data))
}

pub fn decompress_object_hex<T: for<'a> serde::Deserialize<'a>>(hex_string: &str) -> Result<T, Box<dyn error::Error>> {
    let compressed_data = hex::decode(hex_string)?;
    let mut decoder = flate2::read::DeflateDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    let object: T = bincode::deserialize(&decompressed_data)?;

    Ok(object)
}

pub fn decompress_and_decode_object<T:for<'a> serde::Deserialize<'a>>(base64_string: &str) -> Result<T, Box<dyn error::Error>> {
    let compressed_data = BASE64_STANDARD.decode(base64_string)?;
    let mut decoder = flate2::read::DeflateDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    let object: T = bincode::deserialize(&decompressed_data)?;

    Ok(object)
}

pub fn decompress_and_decode_string(base64_string: &str) -> Result<String, Box<dyn error::Error>> {
    let compressed_data = BASE64_STANDARD.decode(base64_string)?;
    let mut decoder = flate2::read::DeflateDecoder::new(&compressed_data[..]);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data)?;

    Ok(String::from_utf8(decompressed_data)?)
}

pub fn encode_utf8_to_ascii(input: &str) -> Result<String, Box<dyn error::Error>> {
    let mut encoded_string = String::new();
    
    for c in input.chars() {
        if c.is_ascii() {
            encoded_string.push(c);
        } else {
            let ascii_representation = c.escape_unicode().to_string();
            encoded_string.push_str(&ascii_representation);
        }
    }
    
    Ok(encoded_string)
}

pub fn decode_ascii_to_utf8(input: &str) -> Result<String, Box<dyn error::Error>> {
    let mut decoded_string = String::new();
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(escaped) = chars.next() {
                if escaped == 'u' {
                    let mut unicode = String::new();
                    for _ in 0..4 {
                        if let Some(digit) = chars.next() {
                            unicode.push(digit);
                        } else {
                            break;
                        }
                    }
                    if let Ok(codepoint) = u32::from_str_radix(&unicode, 16) {
                        if let Some(decoded) = std::char::from_u32(codepoint) {
                            decoded_string.push(decoded);
                        }
                    }
                } else {
                    decoded_string.push(escaped);
                }
            }
        } else {
            decoded_string.push(c);
        }
    }

    Ok(decoded_string)
}

pub fn convert_to_title_case(input: &str) -> Result<String, Box<dyn error::Error>> {
    let mut title_case = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_alphabetic() {
            if capitalize_next {
                title_case.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                title_case.push(c.to_ascii_lowercase());
            }
        } else {
            title_case.push(c);
            capitalize_next = true;
        }
    }

    Ok(title_case)
}

pub fn convert_to_sentence_case(input: &str, preserve_acronyms: bool) -> Result<String, Box<dyn error::Error>> {
    let mut sentence_case = String::new();
    let mut capitalize_next = true;

    for c in input.chars() {
        if c.is_alphabetic() {
            if capitalize_next {
                sentence_case.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                sentence_case.push(c.to_ascii_lowercase());
            }
        } else if c.is_whitespace() {
            sentence_case.push(' ');
            capitalize_next = true;
        } else if preserve_acronyms && c.is_ascii_uppercase() {
            sentence_case.push(c);
            capitalize_next = false;
        }
    }

    Ok(sentence_case)
}

pub fn beautify_json(json_string: &str) -> Result<String, Box<dyn error::Error>> {
    let json_value = serde_json::from_str::<serde_json::Value>(json_string)?;
    let beautified_json = serde_json::to_string_pretty(&json_value)?;
    Ok(beautified_json)
}

pub fn beautify_xml(xml_string: &str) -> Result<String, Box<dyn error::Error>> {
    let document = roxmltree::Document::parse(xml_string)?;
    let beautified_xml = document.input_text();
    Ok(beautified_xml.to_string())
}

pub fn beautify_sql(sql_string: &str) -> Result<String, Box<dyn error::Error>> {
    let beautified_sql = sqlformat::format(sql_string, &sqlformat::QueryParams::None,  sqlformat::FormatOptions { indent: sqlformat::Indent::Spaces(4), uppercase: true, lines_between_queries: 1 });

    Ok(beautified_sql)
}

pub fn encrypt_rsa_pkcs1(public_key: &str, plaintext: &str) -> Result<String, Box<dyn error::Error>> {
    let mut rng = rand::thread_rng();
    let rsa_key = rsa::RsaPublicKey::from_pkcs1_pem(public_key)?;
    let ciphertext = rsa_key.encrypt(&mut rng, Pkcs1v15Encrypt, plaintext.as_bytes())?;
    let base64_ciphertext = BASE64_STANDARD.encode(ciphertext);
    Ok(base64_ciphertext)
}

pub fn decrypt_rsa_pkcs1(private_key: &str, ciphertext: &str) -> Result<String, Box<dyn error::Error>> {
    let ciphertext_bytes = BASE64_STANDARD.decode(ciphertext)?;
    let rsa_key = rsa::RsaPrivateKey::from_pkcs1_pem(private_key)?;
    let plaintext = rsa_key.decrypt(Pkcs1v15Encrypt, &ciphertext_bytes)?;
    let plaintext_string = String::from_utf8_lossy(&plaintext).to_string();
    Ok(plaintext_string)
}

pub fn encrypt_rsa_pkcs8(public_key: &str, plaintext: &str) -> Result<String, Box<dyn error::Error>> {
    let mut rng = rand::thread_rng();
    let rsa_key = rsa::RsaPublicKey::from_public_key_pem(public_key)?;
    let ciphertext = rsa_key.encrypt(&mut rng, Pkcs1v15Encrypt, plaintext.as_bytes())?;
    let base64_ciphertext = BASE64_STANDARD.encode(ciphertext);
    Ok(base64_ciphertext)
}

pub fn decrypt_rsa_pkcs8(private_key: &str, ciphertext: &str) -> Result<String, Box<dyn error::Error>> {
    let ciphertext_bytes = BASE64_STANDARD.decode(ciphertext)?;
    let rsa_key = rsa::RsaPrivateKey::from_pkcs8_pem(private_key)?;
    let plaintext = rsa_key.decrypt(Pkcs1v15Encrypt, &ciphertext_bytes)?;
    let plaintext_string = String::from_utf8_lossy(&plaintext).to_string();
    Ok(plaintext_string)
}

pub fn sign_hmac(key: &[u8], message: &str) -> Result<String, Box<dyn error::Error>> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key)?;
    mac.update(message.as_bytes());

    let result = mac.finalize();
    let signature = result.into_bytes();

    Ok(hex::encode(signature))
}