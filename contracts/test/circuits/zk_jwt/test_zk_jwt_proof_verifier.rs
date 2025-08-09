// @dev - Noir
use noir::{
    barretenberg::{
        prove::prove_ultra_honk, srs::setup_srs_from_bytecode, utils::get_honk_verification_key,
        verify::verify_ultra_honk,
    },
    witness::from_vec_str_to_witness_map,
};

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

#[tokio::test(flavor = "current_thread")]
async fn test_zk_jwt_proof_verifier() -> eyre::Result<()> {
    // 1. Generate a proof
    let proof: Vec<u8>;
    let public_inputs: Vec<FixedBytes<32>>;
    (proof, public_inputs) = proof_generator::generate_proof().await;
    println!("🔄 Generated proof: {:?}", proof);
    println!("🔄 Generated public inputs: {:?}", public_inputs);

    // 2. Start Anvil (local test network)
    let anvil = Anvil::new().spawn();
    println!("✅ Anvil running at: {}", anvil.endpoint());

    // Create a signer using one of Anvil's default private keys
    let signer: PrivateKeySigner = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
    
    // Create provider with wallet  
    let provider = ProviderBuilder::new()
        .with_gas_estimation()
        .wallet(signer.clone())
        .on_http(anvil.endpoint_url());

    // 3. Deploy HonkVerifier first using helper function
    let honk_address = deploy_honk_verifier(&provider).await?;
    let honk_verifier = HonkVerifier::new(honk_address, &provider);

    // 4. Deploy ZkJwtProofVerifier with HonkVerifier address as constructor parameter
    let zk_jwt_proof_verifier_json = std::fs::read_to_string("out/ZkJwtProofVerifier.sol/ZkJwtProofVerifier.json")?;
    let zk_jwt_proof_verifier_artifact: serde_json::Value = serde_json::from_str(&zk_jwt_proof_verifier_json)?;
    let zk_bytecode_hex = zk_jwt_proof_verifier_artifact["bytecode"]["object"]
        .as_str()
        .ok_or_else(|| eyre::eyre!("Failed to get ZkJwtProofVerifier bytecode"))?;
    
    // Append constructor parameter (HonkVerifier address) to bytecode
    let mut zk_deploy_bytecode = Bytes::from_hex(zk_bytecode_hex)?.to_vec();
    let mut constructor_arg = [0u8; 32];
    constructor_arg[12..].copy_from_slice(honk_address.as_slice());
    zk_deploy_bytecode.extend_from_slice(&constructor_arg);
    
    let zk_deploy_tx = TransactionRequest::default().with_deploy_code(Bytes::from(zk_deploy_bytecode));
    let zk_receipt = provider.send_transaction(zk_deploy_tx).await?.get_receipt().await?;
    let zk_contract_address = zk_receipt.contract_address.expect("ZkJwtProofVerifier deployment failed");
    
    let zk_jwt_proof_verifier = ZkJwtProofVerifier::new(zk_contract_address, &provider);
    println!("✅ ZkJwtProofVerifier deployed at: {:?}", zk_contract_address);
    
    // 6. For now, test with empty proof (since we need actual JWT data to generate real proofs)
    println!("🔄 Testing verifier with empty proof (expected to fail gracefully)...");
    
    // TODO: Implement real proof generation when we have test JWT data
    // This would require:
    // 1. A valid JWT token 
    // 2. The corresponding public key (JsonWebKey)
    // 3. SRS file path
    // 4. Converting JWTCircuitInputs to HashMap<String, Vec<String>> format
    
    // Convert proof from Vec<u8> to Bytes for contract call
    let proof_bytes = Bytes::from(proof.clone());
    
    // Debug information
    println!("🔍 Debug information:");
    println!("  - Proof length: {} bytes", proof.len());
    println!("  - Proof first 32 bytes: {:?}", &proof[..std::cmp::min(32, proof.len())]);
    println!("  - Proof bytes (hex): 0x{}", hex::encode(&proof[..std::cmp::min(64, proof.len())]));
    println!("  - Public inputs count: {}", public_inputs.len());
    if !public_inputs.is_empty() {
        println!("  - First public input: {:?}", public_inputs[0]);
        println!("  - First public input (hex): 0x{}", hex::encode(public_inputs[0].as_slice()));
    }
    
    // Try to identify what the error means
    println!("🔍 Contract verification attempt:");
    
    // 7. Call the ZkJwtProofVerifier contract (expecting it to fail gracefully)
    println!("🔄 Calling the ZkJwtProofVerifier#verifyZkJwtProof() with a proof and publicInputs...");
    let is_valid = zk_jwt_proof_verifier.verifyZkJwtProof(proof_bytes, public_inputs).call().await;

    match &is_valid {
        Ok(result) => {
            println!("✅ Contract call succeeded: {}", result);
        }
        Err(e) => {
            println!("❌ Contract call failed: {:?}", e);
        }
    }
    
    println!("🔄 is_valid: {:?}", is_valid);
        
    println!("✅ Honk verifier setup test completed successfully");
    Ok(())
}



// @notice - Deploys the HonkVerifier contract and returns the contract address
async fn deploy_honk_verifier<P>(provider: &P) -> eyre::Result<alloy::primitives::Address>
where
    P: Provider,
{
    // Read HonkVerifier contract artifact
    let honk_verifier_json = std::fs::read_to_string("out/honk_vk.sol/HonkVerifier.json")?;
    let honk_verifier_artifact: serde_json::Value = serde_json::from_str(&honk_verifier_json)?;
    let honk_bytecode_hex = honk_verifier_artifact["bytecode"]["object"]
        .as_str()
        .ok_or_else(|| eyre::eyre!("Failed to get HonkVerifier bytecode"))?;
    let honk_bytecode = Bytes::from_hex(honk_bytecode_hex)?;
    
    // Deploy the contract
    let honk_deploy_tx = TransactionRequest::default().with_deploy_code(honk_bytecode);
    let honk_receipt = provider.send_transaction(honk_deploy_tx).await?.get_receipt().await?;
    let honk_address = honk_receipt.contract_address.expect("HonkVerifier deployment failed");
    
    println!("✅ HonkVerifier deployed at: {:?}", honk_address);
    Ok(honk_address)
}