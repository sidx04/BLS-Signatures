use crate::{
    core::{KeyValidationError, PublicKey, Signature, SignatureError},
    util::hash_to_g2,
};

use ark_bls12_381::{Bls12_381, G1Affine};
use ark_ec::{AffineRepr, pairing::Pairing};

pub fn key_validate(pk: &PublicKey) -> Result<String, KeyValidationError> {
    if !pk.is_on_curve() {
        return Err(KeyValidationError::NotOnCurve);
    }

    if pk.is_zero() {
        return Err(KeyValidationError::Identity);
    }

    if !pk.is_in_correct_subgroup_assuming_on_curve() {
        return Err(KeyValidationError::WrongSubgroup);
    }

    Ok(String::from("Public Key is valid."))
}

pub fn core_verify(pk: &PublicKey, msg: &[u8], sig: &Signature) -> Result<(), SignatureError> {
    if !sig.is_on_curve() {
        return Err(SignatureError::InvalidSignature);
    }
    if !sig.is_in_correct_subgroup_assuming_on_curve() {
        return Err(SignatureError::InvalidSignatureSubgroup);
    }
    if let Err(e) = key_validate(pk) {
        return Err(SignatureError::InvalidPublicKey(e));
    }
    let q = hash_to_g2(msg)?;

    let c1 = Bls12_381::pairing(pk, q);
    let c2 = Bls12_381::pairing(G1Affine::generator(), sig);

    if c1 == c2 {
        println!("Verification succesful!");
        Ok(())
    } else {
        Err(SignatureError::InvalidSignature)
    }
}
