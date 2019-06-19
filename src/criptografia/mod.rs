use std::fs::File;
use std::io::prelude::*;

use openssl::symm::{encrypt, decrypt, Cipher};

use rand::prelude::*;

use base64::{encode, decode};

pub fn criptografar(s: &str, chave: &[u8; 32]) -> String {
    let cifra = Cipher::aes_256_ecb();

    String::from_utf8_lossy(encode(&encrypt(
        cifra,
        chave,
        None,
        s.as_bytes()).unwrap()).as_bytes()).to_string()
}

pub fn des_criptografar(s: &str, chave: &[u8; 32]) -> String {
    let cifra = Cipher::aes_256_ecb();

    String::from_utf8_lossy(&decrypt(
        cifra,
        chave,
        None,
        &decode(s).unwrap()).unwrap()).to_string()
}

pub fn recuperar_credenciais() -> [u8; 32] {
    let mut chave = [0 as u8; 32];

    let mut kf = File::open("./testes/credenciais/chave.aes").unwrap();
    kf.read_exact(&mut chave).unwrap();

    chave
}

pub fn gerar_credendiais() -> Result<(), std::io::Error> {
    let mut rng = rand::thread_rng();
    let mut chave = [0 as u8; 32];

    rng.fill_bytes(&mut chave);

    let mut kf = File::create("./testes/credenciais/chave.aes")?;
    kf.write_all(&chave)?;

    Ok(())
}
