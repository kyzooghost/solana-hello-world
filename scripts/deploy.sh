#!/bin/bash

rm -rf target/deploy/solana_hello_world-keypair.json
cargo build-sbf
solana program deploy ./target/deploy/solana_hello_world.so