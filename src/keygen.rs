use anyhow::{Context, Ok};
use ark_bls12_381::{Fr, G1Projective};
use ark_ec::{CurveGroup, PrimeGroup};
use ark_ff::PrimeField;
use hkdf::Hkdf;
use sha2::Sha256;
use std::path::PathBuf;

use crate::{
    core::PublicKey,
    util::{read_sk, write_sk},
};

pub fn keygen(ikm: &[u8], path: &PathBuf) -> anyhow::Result<()> {
    let salt = b"BLS-SIG-KEYGEN-SALT-";
    let hk = Hkdf::<Sha256>::new(Some(salt), ikm);

    const L: usize = 48;

    let mut okm = [0u8; L];
    hk.expand(&[0x00], &mut okm).expect("HDKF failed!");

    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&okm[L - 32..]);

    let sk = Fr::from_be_bytes_mod_order(&bytes);

    write_sk(sk, path)
}

pub fn sk_to_pk(path: &PathBuf) -> anyhow::Result<PublicKey> {
    let sk = read_sk(path).with_context(|| "Failed to get secret key from file.")?;
    let pk_point = G1Projective::generator() * sk;
    let pk = pk_point.into_affine();
    Ok(pk)
}
