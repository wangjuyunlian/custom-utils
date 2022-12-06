use anyhow::{Context, Result};
use rustls::Certificate;
use std::path::Path;

pub fn init_root_certs_by_native() -> Result<rustls::RootCertStore> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in rustls_native_certs::load_native_certs().context("could not load platform certs")? {
        roots.add(&rustls::Certificate(cert.0))?;
    }
    Ok(roots)
}
pub fn init_root_certs_by_path(path: impl AsRef<Path>) -> Result<rustls::RootCertStore> {
    let mut roots = rustls::RootCertStore::empty();
    for cert in load_pem_certs_by_path(path).context("could not load certs")? {
        roots.add(&cert)?;
    }
    Ok(roots)
}

pub fn load_pem_certs_by_path(path: impl AsRef<Path>) -> Result<Vec<Certificate>> {
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let datas = rustls_pemfile::certs(&mut reader)?;
    let mut certs = Vec::with_capacity(datas.len());
    for data in datas.into_iter() {
        certs.push(rustls::Certificate(data));
    }
    Ok(certs)
}
