use anyhow::Context;
use ark_bls12_381::{G1Affine, G2Affine};
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use bls_sig_cli::core::{core_sign, core_verify, keygen, sk_to_pk};
use bls_sig_cli::interface::{Cli, Commands};
use clap::Parser;
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Keygen { ikm, out } => {
            let path = handle_path(&out)?;
            keygen(ikm.unwrap_or_default().as_bytes(), &path)?;
        }

        Commands::Pubkey { path } => {
            let p = handle_path(&path)?;

            let pk = sk_to_pk(&p)?;
            let mut buf: Vec<u8> = Vec::new();
            pk.serialize_compressed(&mut buf)?;
            println!("{}", hex::encode(buf));
        }

        Commands::Sign { path, msg } => {
            let path = handle_path(&path)?;
            let msg = handle_msg(&msg).with_context(|| "Failed to interpret message.")?;
            let sig = core_sign(&msg, &path)?;

            let mut buf: Vec<u8> = Vec::new();
            sig.serialize_compressed(&mut buf)?;
            println!("{}", hex::encode(buf));
        }

        Commands::Verify { pk, msg, sig } => {
            let pk_buf: Vec<u8> = hex::decode(pk)?;
            let sig_buf: Vec<u8> = hex::decode(sig)?;
            let msg = handle_msg(&msg).with_context(|| "Failed to interpret message.")?;

            let pk = G1Affine::deserialize_compressed(&*pk_buf)?;
            let sig = G2Affine::deserialize_compressed(&*sig_buf)?;

            core_verify(&pk, &msg, &sig)?;
        }
    }

    Ok(())
}

fn handle_path(path: &Option<String>) -> Result<PathBuf, anyhow::Error> {
    let path_buf = if let Some(custom) = path {
        PathBuf::from(custom)
    // } else if system {
    //     PathBuf::from("/etc/bls/secret.key")
    } else {
        let home = dirs::home_dir().context("Could not find home directory")?;
        home.join(".bls/secret.key")
    };
    Ok(path_buf)
}

fn handle_msg(input: &str) -> anyhow::Result<Vec<u8>> {
    let path = Path::new(input);

    if path.exists() && path.is_file() {
        if let Some(ext) = path.extension() {
            if ext == "json" {
                let json_str = std::fs::read_to_string(path)?;
                if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&json_str) {
                    println!("📝 Signing JSON file: {}", path.display());
                    return Ok(serde_json::to_vec(&json_val)?);
                }
            }
        }

        let content = std::fs::read(path)?;
        println!("📝 Signing binary file: {}", path.display());
        return Ok(content);
    }

    if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(input) {
        println!("📝 Signing JSON string: {}", input);
        return Ok(serde_json::to_vec(&json_val)?);
    }

    println!("📝 Signing raw string: \"{}\"", input);
    Ok(input.as_bytes().to_vec())
}
