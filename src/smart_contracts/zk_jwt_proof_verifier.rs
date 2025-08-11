// // @dev - Noir
// use noir::{
//     barretenberg::{
//         prove::prove_ultra_honk, srs::setup_srs_from_bytecode, utils::get_honk_verification_key,
//         verify::verify_ultra_honk,
//     },
//     witness::from_vec_str_to_witness_map,
// };

// @dev - Alloy
use alloy::{
    providers::{Provider, ProviderBuilder},
    signers::local::PrivateKeySigner,
    sol,
    primitives::{Bytes, FixedBytes},
    hex::FromHex,
    rpc::types::TransactionRequest,
    network::TransactionBuilder,
};
use alloy_node_bindings::Anvil;

// @dev - Load the proof_generator module
#[path = "./proof_generator/proof_generator.rs"]
mod proof_generator;
use proof_generator::generate_proof;

// @dev - Imports the following modules for proof/input generation from the parent crate (./src/proof/) directory.
// @dev - "mopro_bindings" would be the parent crate "name", which is defined as the "[lib]" in the Cargo.toml of the parent crate directory. 
use mopro_bindings::{
    generate_ephemeral_key,
    prove_jwt, // @dev - prove_jwt() is available directly from the root
    verify_jwt_proof,
    proof::jwt_proof::{
        generate_inputs,
        verify_jwt, // @dev - verify_jwt() is in the proof::jwt_proof module
        JsonWebKey,
        JWTCircuitInputs
    },
};
use std::collections::HashMap;

// 1. Define the Solidity interface using alloy::sol!
sol! {
    #[sol(rpc)]
    ZkJwtProofVerifier,
    "out/ZkJwtProofVerifier.sol/ZkJwtProofVerifier.json"
}

sol! {
    #[sol(rpc)]  
    HonkVerifier,
    "out/honk_vk.sol/HonkVerifier.json"
}


async fn call_zk_jwt_proof_verifier() -> eyre::Result<()> {
    // [TODO]:
}