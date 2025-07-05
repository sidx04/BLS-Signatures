# BLS Signatures CLI (`bls-sig-cli`)

A proof-of-concept, command-line tool for BLS signature operations, providing functionalities for key generation, public key derivation, message signing, and signature verification.

---

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
  - [From Crates.io (Recommended)](#from-cratesio-recommended)
  - [From Source](#from-source)
- [Usage](#usage)
  - [Global Options](#global-options)
  - [Commands](#commands)
    - [keygen](#keygen)
    - [pubkey](#pubkey)
    - [sign](#sign)
    - [verify](#verify)
- [Examples](#examples)
- [Contributing](#contributing)
- [License](#license)
- [Authors](#authors)

---

## Overview

**`bls-sig-cli`** is a versatile command-line interface designed to facilitate various operations related to BLS (Boneh-Lynn-Shacham) signatures. It leverages robust cryptographic libraries to provide a secure and easy-to-use tool for generating keys, signing messages, and verifying signatures.

---

## Features

- **Key Generation**: Generate BLS secret keys from Input Key Material (IKM) or randomly.
- **Public Key Derivation**: Derive the corresponding public key from a given secret key file.
- **Message Signing**: Sign messages using a BLS secret key.
- **Signature Verification**: Verify BLS signatures against a public key and a message.
- **Flexible Output**: Output keys and signatures to standard output or specified files.

---

## Installation

### From Crates.io (Recommended)

To install `bls-sig-cli` directly from [Crates.io](https://crates.io/crates/bls-sig-cli), ensure you have **Rust** and **Cargo** installed, then run:

```sh
cargo install bls-sig-cli
```

### From Source

If you prefer to build from source, follow these steps:

```sh
git clone https://github.com/sidx04/bls-sig-cli.git
cd bls-sig-cli
cargo build --release
```

The executable will be located at `target/release/bls-sig-cli`. You can then move it to a directory in your `PATH`:

```sh
cp target/release/bls-sig-cli ~/.local/bin/ # or any other specified directory
```

---

## Usage

The `bls-sig-cli` CLI provides several subcommands, each corresponding to a specific BLS signature operation.

### Global Options

```text
BLS Signatures CLI

Usage: bls-sig-cli <COMMAND>
```

### Commands

#### `keygen`

Generate a new BLS secret key.

```text
Usage: bls-sig-cli keygen [OPTIONS]
```

Options:

- `-i`, `--ikm <IKM>`: Optional. Input Key Material used to derive the BLS private key.
- `-o`, `--out <OUT>`: Optional. Path to store the generated secret key.
- `-h`, `--help`: Print help.

#### `pubkey`

Derive and print the public key from a secret key file.

```text
Usage: bls-sig-cli pubkey [OPTIONS]
```

Options:

- `-p`, `--path <PATH>`: Optional. Path to the secret key file.
- `-h`, `--help`: Print help.

#### `sign`

Sign a message using a BLS secret key. A message can be any file or any arbitrary string of characters.

```text
Usage: bls-sig-cli sign [OPTIONS] --msg <MSG>
```

Options:

- `-p`, `--path <PATH>`: Optional. Path to the secret key file.
- `-m`, `--msg <MSG>`: Mandatory. Message to be signed.
- `-h`, `--help`: Print help.

#### `verify`

Verify a BLS signature.

```text
Usage: bls-sig-cli verify --pk <PK> --msg <MSG> --sig <SIG>
```

Options:

- `-p`, `--pk <PK>`: Mandatory. Public key (hex).
- `-m`, `--msg <MSG>`: Mandatory. Message used during signing.
- `-s`, `--sig <SIG>`: Mandatory. Signature (hex).
- `-h`, `--help`: Print help.

---

## Examples

Generate a random secret key and save to a file:

```sh
bls-sig-cli keygen --out my_secret.key
```

Generate a secret key from IKM:

```sh
bls-sig-cli keygen --ikm "MY_SUPER_SECRET_STRING"
```

Derive a public key:

```sh
bls-sig-cli pubkey --path my_secret.key
```

Sign a message:

```sh
bls-sig-cli sign --path my_secret.key --msg "Hello, world!"
```

Verify a signature:

```sh
bls-sig-cli verify --pk PK_HEX --msg "MSG_STRING" --sig SIG_HEX
```

> Replace `PK_HEX`, `MSG_STRING`, and `SIG_HEX` with actual values.

---

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request on the [GitHub repository](https://github.com/sidx04/bls-sig-cli).

---

## License

This project is licensed under:

- [MIT License](http://opensource.org/licenses/MIT)

---

## Authors

- **sidx04** – [siddpal04@gmail.com](mailto:siddpal04@gmail.com)
