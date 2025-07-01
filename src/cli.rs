use clap::{Parser, Subcommand, command};

#[derive(Parser, Debug)]
#[command(name = "bls", version = "0.1.0", about = "BLS Signatures CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Available commands for the BLS signature CLI.
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Generate a new BLS secret key.
    ///
    /// This command allows you to derive a new secret key from Input Key Material (IKM)
    /// or generate a random one if no IKM is provided.
    /// The generated key can be printed to standard output or saved to a specified file.
    Keygen {
        /// Optional: Input Key Material (IKM) used to derive the BLS private key.
        /// This should be a high-entropy, secret string.
        /// If not provided, a cryptographically secure random private key will be generated.
        #[arg(short = 'i', long, help = "Input Key Material")]
        ikm: Option<String>,

        /// Optional: Specifies the file path where the generated secret key should be stored.
        /// If this argument is omitted, the secret key will be printed to standard output.
        #[arg(short = 'o', long, help = "Custom output file path")]
        out: Option<String>,
        // /// Store key at /etc/bls/secret.key (requires root)
        // #[arg(long, help = "Use system-wide key path (/etc/bls/secret.key)")]
        // system: bool, // This argument is commented out in your provided code.
    },

    /// Derive and print the public key associated with a given BLS secret key file.
    ///
    /// This command reads a secret key from a specified file and computes its
    /// corresponding public key, which is then printed to standard output.
    Pubkey {
        /// Optional: Path to the BLS secret key file.
        /// If omitted, the application might look for a default key file
        /// (e.g., in a standard configuration directory), or it may result in an error
        /// if no key can be found.
        #[arg(short = 'p', long, help = "Path to secret key file")]
        path: Option<String>,
    },

    /// Sign a message using a BLS secret key.
    ///
    /// This command takes a message and a secret key (from a file) to produce
    /// a BLS signature. The signature will be printed to standard output.
    Sign {
        /// Optional: Path to the BLS secret key file to be used for signing.
        /// If omitted, the application might look for a default key file
        /// (e.g., in a standard configuration directory), or it may result in an error
        /// if no key can be found.
        #[arg(short, long, help = "Path to secret key file")]
        path: Option<String>,

        /// Mandatory: The message data to be signed.
        /// This can be a raw string provided directly on the command line,
        /// or the content of a file (if your application's logic supports reading from file).
        #[arg(
            short,
            long,
            required = true,
            help = "Message to be signed. Accepts a generic file or a raw string"
        )]
        msg: String,
    },

    /// Verify a BLS signature against a public key and a message.
    ///
    /// This command takes a public key, a message, and a signature (all in hexadecimal format)
    /// and verifies if the signature is valid for the given message and public key.
    /// The verification result (success or failure) will be printed.
    Verify {
        /// Mandatory: The BLS public key, provided in hexadecimal string format.
        #[arg(short, long, required = true, help = "Public key in hex")]
        pk: String,

        /// Mandatory: The original message that was signed.
        /// This can be a raw string provided directly on the command line,
        /// or the content of a file (if your application's logic supports reading from file).
        /// It must exactly match the message used during signing.
        #[arg(
            short,
            long,
            required = true,
            help = "Message to be signed. Accepts a generic file or a raw string"
        )]
        msg: String,

        /// Mandatory: The BLS signature, provided in hexadecimal string format.
        #[arg(short, long, required = true, help = "Signature in hex")]
        sig: String,
    },
}
