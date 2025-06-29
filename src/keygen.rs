use anyhow::Context;
use ark_bls12_381::{Fr, G1Projective};
use ark_ec::{CurveGroup, PrimeGroup};
use ark_ff::PrimeField;
use hkdf::Hkdf;
use sha2::Sha256;
use std::{fs::OpenOptions, io::Write, os::unix::fs::OpenOptionsExt, path::PathBuf};

use crate::bls::{PublicKey, SecretKey};

pub fn keygen(ikm: &[u8], path: &PathBuf) -> anyhow::Result<()> {
    let salt = b"BLS-SIG-KEYGEN-SALT-";
    let hk = Hkdf::<Sha256>::new(Some(salt), ikm);

    const L: usize = 48;

    let mut okm = [0u8; L];
    hk.expand(&[0x00], &mut okm).expect("HDKF failed!");

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&okm[L - 32..]);

    let sk = Fr::from_be_bytes_mod_order(&mut bytes);

    write_sk(sk, path)
}

pub fn sk_to_pk(sk: SecretKey) -> PublicKey {
    let pk_point = G1Projective::generator() * sk;
    let pk = pk_point.into_affine();

    pk
}

fn write_sk(sk: SecretKey, path: &PathBuf) -> anyhow::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }
    }

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .mode(0o600)
        .open(path)
        .with_context(|| format!("Failed to write to {}", path.display()))?;

    file.write_all(sk.to_string().as_bytes())?;
    println!("🔐 Secret key written to: {}", path.display());

    Ok(())
}
