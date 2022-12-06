use anyhow::{bail, Result};
use std::path::Path;

pub fn load_rsa_key(path: impl AsRef<Path>) -> Result<rustls::PrivateKey> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let datas = rustls_pemfile::rsa_private_keys(&mut reader)?;
    for data in datas.into_iter() {
        return Ok(rustls::PrivateKey(data));
    }
    bail!("未找到秘钥")
}

pub fn load_pkcs8_key(path: impl AsRef<Path>) -> Result<rustls::PrivateKey> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let datas = rustls_pemfile::pkcs8_private_keys(&mut reader)?;
    for data in datas.into_iter() {
        return Ok(rustls::PrivateKey(data));
    }
    bail!("未找到秘钥")
}
