.PHONY: all build test clean run-keygen run-pubkey

# The 'all' target now just ensures the project is built.
# You'll run specific commands like 'make run-keygen' or 'make run-pubkey'.
all: build

# Builds the project in release mode for optimized performance.
build:
	cargo build --release

# Target to run the key generation command.
#
# Usage:
#   make run-keygen                      # Generates a random key, prints to console
#   make run-keygen IKM="mysecret"       # Generates key from IKM, prints to console
#   make run-keygen OUT="mykey.sk"       # Generates random key, saves to file
#   make run-keygen IKM="mysecret" OUT="mykey.sk" # Generates key from IKM, saves to file
run-keygen: build
	./target/release/bls-sig keygen \
	$(if $(OUT),--out "$(OUT)",) \
	$(if $(IKM),--ikm "$(IKM)",)

# Target to run the public key derivation command.
#
# Usage:
#   make run-pubkey PATH="mykey.sk"      # Derives public key from 'mykey.sk'
#   (PATH is typically mandatory for pubkey derivation, but made optional here for flexibility)
run-pubkey: build
	./target/release/bls-sig pubkey \
	$(if $(PATH_VAR),--path "$(PATH_VAR)",)


# Target to sign a message using a BLS private key.
#
# Usage:
#   make run-sign MSG="Hello World"                     # Signs "Hello World" (uses default key path if implemented in CLI)
#   make run-sign PATH_VAR="mykey.sk" MSG="Secret Data" # Signs "Secret Data" using 'mykey.sk'
#
# This target requires the 'MSG' variable to be set.
run-sign: build
	@if [ -z "$(MSG)" ]; then \
		echo "Error: MSG variable must be provided for 'run-sign'."; \
		echo "Usage: make run-sign MSG=\"Your message\" [PATH_VAR=\"/path/to/secret.key\"]"; \
		exit 1; \
	fi
	./target/release/bls-sig sign \
	$(if $(PATH_VAR),--path "$(PATH_VAR)",) \
	--msg "$(MSG)"

# Runs all tests in the project.
test:
	cargo test

# Cleans the build artifacts.
clean:
	cargo clean