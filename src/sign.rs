use std::path::PathBuf;

use anyhow::Context;
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::PrimeField;

use crate::{
    core::Signature,
    util::{hash_to_g2, read_sk},
};

pub fn core_sign(msg: &[u8], path: &PathBuf) -> anyhow::Result<Signature> {
    let sk = read_sk(path).with_context(|| "Failed to get secret key from file.")?;
    let q = hash_to_g2(msg)?;
    let r = q.mul_bigint(sk.into_bigint());
    let sig = r.into_affine();

    Ok(sig)
}
