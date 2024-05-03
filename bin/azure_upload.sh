#!/bin/bash

# Set the RUST_LOG environment variable
export RUST_LOG=info

# Run your Rust program with the provided arguments
./azure-vault-file-upload "$@"
