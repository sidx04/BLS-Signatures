use anyhow::Context;
use bls_sig::core::{core_sign, core_verify, keygen, sk_to_pk};
use bls_sig::interface::{Cli, Commands};
use clap::Parser;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    #[allow(unused_variables)]
    match cli.command {
        Commands::Keygen { ikm, out } => {
            let path = handle_path(&out)?;
            keygen(ikm.unwrap_or_default().as_bytes(), &path)?;
        }

        Commands::Pubkey { path } => {
            let p = handle_path(&path)?;

            let res = sk_to_pk(&p)?;
            println!("{}", res);
        }

        Commands::Sign { path, msg } => {
            let path = handle_path(&path)?;
            let sig = core_sign(msg.as_bytes(), &path)?;
            println!("{}", sig);
        }

        Commands::Verify { pk, msg, sig } => {}
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
