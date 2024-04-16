https://solana.com/developers/guides/getstarted/local-rust-hello-world

# Build and deploy Solana bytecode
```
rm -rf target/deploy/solana_hello_world-keypair.json
cargo build-sbf
solana program deploy ./target/deploy/solana_hello_world.so
```


# Setup local blockchain
`solana config set --url localhost`
`npx kill-port 9900`
`solana-test-validator`