use ark_ec::hashing::HashToCurveError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum KeyValidationError {
    #[error("Point is not on the curve.")]
    NotOnCurve,
    #[error("Point cannot be the identity.")]
    Identity,
    #[error("Point is not in the correct subgroup.")]
    WrongSubgroup,
}

#[derive(Debug, Error)]
pub enum SignatureError {
    #[error("Problem with hashing the message.")]
    HashError(#[from] HashToCurveError),
    #[error("Signature deserialization failed.")]
    InvalidSignature,
    #[error("Signature is not in the correct subgroup.")]
    InvalidSignatureSubgroup,
    #[error("Public key is invalid.")]
    InvalidPublicKey(#[from] KeyValidationError),
    #[error("Pairing mismatch.")]
    InvalidSignaturePairing,
}
