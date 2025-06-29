use clap::{Parser, Subcommand, command};

#[derive(Parser, Debug)]
#[command(name = "bls", version = "0.1.0", about = "BLS Signatures CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a new secret key from IKM and write to file
    Keygen {
        #[arg(long, default_value = "", help = "Input Key Material")]
        ikm: Option<String>,

        /// Optional output path for storing secret key
        #[arg(long, help = "Custom output file path")]
        out: Option<String>,

        /// Store key at /etc/bls/secret.key (requires root)
        #[arg(long, help = "Use system-wide key path (/etc/bls/secret.key)")]
        system: bool,
    },

    /// Print public key from secret key file
    Pubkey {
        #[arg(long, help = "Path to secret key file")]
        sk_file: String,
    },

    /// Sign a message using secret key
    Sign {
        #[arg(long)]
        sk_file: String,

        #[arg(long)]
        msg: String,
    },

    /// Verify a signature
    Verify {
        #[arg(long, help = "Public key in hex")]
        pk: String,

        #[arg(long)]
        msg: String,

        #[arg(long, help = "Signature in hex")]
        sig: String,
    },
}
