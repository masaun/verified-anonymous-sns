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
sh test/circuits/zk-jwt/honk_verifier/test_honk_verifier.sh

(Or, cargo test --test test_honk_verifier -- --show-output)
```

<br>

- Run the `test_zk_jwt_proof_verifier.rs`
```bash
cd contracts
sh test/circuits/zk-jwt/test_zk_jwt_proof_verifier.sh

(Or, cargo test --test test_zk_jwt_proof_verifier -- --show-output)
```

<br>

- Run the `test_zk_jwt_proof_manager.rs`
```bash
cd contracts
sh test/circuits/zk-jwt/test_zk_jwt_proof_manager.sh

(Or, cargo test --test test_zk_jwt_proof_manager -- --show-output)
```

<br>

## Proof Geration Test in `Rust`

- Run the `test_proof_generation.rs`
```bash
cd contracts
sh test/circuits/zk-jwt/proof_generator/test_proof_generation.sh

(Or, cargo test --test test_proof_generation -- --show-output)
```