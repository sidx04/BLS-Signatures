use ark_bls12_381::{Fr, G1Affine, G2Affine};

pub type SecretKey = Fr;
pub type PublicKey = G1Affine;
pub type Signature = G2Affine;
