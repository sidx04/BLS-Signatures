use ark_bls12_381::{G2Projective, g2::Config};
use ark_ec::{
    AffineRepr, CurveGroup,
    hashing::{
        HashToCurve, HashToCurveError, curve_maps::wb::WBMap,
        map_to_curve_hasher::MapToCurveBasedHasher,
    },
};
use ark_ff::{PrimeField, field_hashers::DefaultFieldHasher};
use sha2::Sha256;

use crate::bls::{SecretKey, Signature, SignatureError};

pub fn core_sign(msg: &[u8], sk: &SecretKey) -> Result<Signature, SignatureError> {
    let q = hash_to_g2(msg)?;
    let r = q.mul_bigint(sk.into_bigint());
    let sig = r.into_affine();

    Ok(sig)
}

pub(crate) fn hash_to_g2(msg: &[u8]) -> Result<Signature, HashToCurveError> {
    let hasher =
        MapToCurveBasedHasher::<G2Projective, DefaultFieldHasher<Sha256>, WBMap<Config>>::new(
            b"BLS_SIG_BLS12381G2_XMD:SHA-256_SSWU_RO_",
        )?;
    hasher.hash(msg)
}
