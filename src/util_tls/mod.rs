#![allow(dead_code)]
use anyhow::{bail, Context, Result};
use rustls::Certificate;
use std::path::Path;

pub fn tls_load_native_root_certs() -> Result<rustls::RootCertStore> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().context("could not load platform certs")? {
        roots.add(&rustls::Certificate(cert.0)).unwrap();
    }
    Ok(roots)
}

pub fn tls_load_pem_certs(path: impl AsRef<Path>) -> Result<Vec<Certificate>> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let datas = rustls_pemfile::certs(&mut reader)?;
    let mut certs = Vec::with_capacity(datas.len());
    for data in datas.into_iter() {
        certs.push(rustls::Certificate(data));
    }
    Ok(certs)
}
pub fn tls_load_pem_rsa_key(path: impl AsRef<Path>) -> Result<rustls::PrivateKey> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let datas = rustls_pemfile::rsa_private_keys(&mut reader)?;
    for data in datas.into_iter() {
        return Ok(rustls::PrivateKey(data));
    }
    bail!("未找到秘钥")
}
