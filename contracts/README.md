# Smart Contracts


## Smart Contract Test in `Rust` using `Alloy.rs`

- Run the all smart contract tests
```bash
cd contracts
cargo test
```

<br>

- Run the `test_honk_verifier.rs`
```bash
cd contracts
sh test/circuits/zk_jwt/honk_verifier/test_honk_verifier.sh

(Or, cargo test --test test_honk_verifier -- --show-output)
```

<br>

- Run the `test_proof_generation.rs`
```bash
cd contracts
sh test/circuits/zk_jwt/test_proof_generation.sh

(Or, cargo test --test test_proof_generation -- --show-output)
```