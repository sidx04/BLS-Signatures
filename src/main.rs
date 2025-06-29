use std::path::PathBuf;

use anyhow::Context;
#[allow(unused_imports)]
use bls_sig::bls::{Cli, Commands, core_sign, core_verify, key_validate, keygen, sk_to_pk};
use clap::Parser;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    #[allow(unused_variables)]
    match cli.command {
        Commands::Keygen { ikm, out, system } => {
            let path = if let Some(custom) = out {
                PathBuf::from(custom)
            } else if system {
                PathBuf::from("/etc/bls/secret.key")
            } else {
                let home = dirs::home_dir().context("Could not find home directory")?;
                home.join(".bls/secret.key")
            };

            keygen(ikm.unwrap_or_default().as_bytes(), &path)?;
        }

        Commands::Pubkey { sk_file } => {
            todo!()
        }

        Commands::Sign { sk_file, msg } => {
            todo!()
        }

        Commands::Verify { pk, msg, sig } => {
            todo!()
        }
    }

    Ok(())
}
