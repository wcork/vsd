use anyhow::Result;
use openssl::symm::{decrypt, Cipher};

enum HlsEncryptionMethod {
    Aes128,
    SampleAes,
    None,
}
pub struct HlsDecrypt {
    key: Vec<u8>,
    iv: Option<Vec<u8>>,
    method: HlsEncryptionMethod,
}

impl HlsDecrypt {
    pub fn from_key(key: m3u8_rs::Key, key_content: Vec<u8>) -> Self {
        let iv = key.iv.map(|encryption_iv| encryption_iv.as_bytes().to_vec());

        match key.method.as_str() {
            "NONE" => Self {
                key: vec![],
                iv,
                method: HlsEncryptionMethod::None,
            },
            "AES-128" => Self {
                key: key_content,
                iv,
                method: HlsEncryptionMethod::Aes128,
            },
            "SAMPLE-AES" => Self {
                key: key_content,
                iv,
                method: HlsEncryptionMethod::SampleAes,
            },
            _ => {
                panic!("Unsupported key method {}", key.method);
            }
        }
    }

    pub fn decrypt(&self, buf: &[u8]) -> Result<Vec<u8>> {
        match self.method {
            HlsEncryptionMethod::None => Ok(buf.to_vec()),
            HlsEncryptionMethod::Aes128 => {
                if let Some(encryption_iv) = self.iv.clone() {
                    Ok(decrypt(
                        Cipher::aes_128_cbc(),
                        &self.key,
                        Some(&encryption_iv),
                        buf,
                    )?)
                } else {
                    Ok(decrypt(Cipher::aes_128_cbc(), &self.key, None, buf)?)
                }
            }
            HlsEncryptionMethod::SampleAes => {
                let mut new_buf = vec![];

                for byte in buf {
                    let mut data = if let Some(encryption_iv) = self.iv.clone() {
                        decrypt(
                            Cipher::aes_128_cbc(),
                            &self.key,
                            Some(&encryption_iv),
                            &[byte.to_owned()],
                        )
                    } else {
                        decrypt(Cipher::aes_128_cbc(), &self.key, None, &[byte.to_owned()])
                    };

                    if let Ok(bytes) = &mut data {
                        new_buf.append(bytes);
                    } else {
                        new_buf.push(byte.to_owned());
                    }
                }

                Ok(new_buf)
            }
        }
    }
}
