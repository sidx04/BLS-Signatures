use anyhow::Context;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use sha2::Sha256;
use std::{
    fs::OpenOptions,
    io::{Read, Write},
    os::unix::fs::OpenOptionsExt,
    path::PathBuf,
};

use ark_bls12_381::{Fr, G2Projective, g2::Config};
use ark_ec::hashing::{
    HashToCurve, HashToCurveError, curve_maps::wb::WBMap,
    map_to_curve_hasher::MapToCurveBasedHasher,
};
use ark_ff::field_hashers::DefaultFieldHasher;

use crate::core::{SecretKey, Signature};

pub fn write_sk(sk: SecretKey, path: &PathBuf) -> anyhow::Result<()> {
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

    let mut buf: Vec<u8> = Vec::new();
    sk.serialize_compressed(&mut buf)?;

    file.write_all(hex::encode(buf).as_bytes())?;
    println!("🔐 Secret key written to: {}", path.display());

    Ok(())
}

pub fn read_sk(path: &PathBuf) -> anyhow::Result<Fr> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(path)
        .with_context(|| format!("Failed to read from {}", path.display()))?;

    let mut buf: Vec<u8> = Vec::new();
    file.read_to_end(&mut buf)?;

    hex::decode(&mut buf)?;
    let sk = Fr::deserialize_compressed(&*buf)?;

    Ok(sk)
}

pub fn hash_to_g2(msg: &[u8]) -> Result<Signature, HashToCurveError> {
    let hasher =
        MapToCurveBasedHasher::<G2Projective, DefaultFieldHasher<Sha256>, WBMap<Config>>::new(
            b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_",
        )?;
    hasher.hash(msg)
}
