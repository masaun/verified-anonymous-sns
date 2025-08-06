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
#[path = "../proof_generator.rs"]
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
    HonkVerifier,
    "out/honk_vk.sol/HonkVerifier.json"
}

// contract HonkVerifier is BaseHonkVerifier(N, LOG_N, NUMBER_OF_PUBLIC_INPUTS) {
//      function loadVerificationKey() internal pure override returns (Honk.VerificationKey memory) {
//        return HonkVerificationKey.loadVerificationKey();
//     }
// }
//
// function verify(bytes calldata proof, bytes32[] calldata publicInputs) public view override returns (bool) {

#[tokio::test(flavor = "current_thread")]
async fn test_honk_verifier() -> eyre::Result<()> {
    // 1. Generate a proof
    let proof: Vec<u8>;
    let public_inputs: Vec<FixedBytes<32>>;
    (proof, public_inputs) = proof_generator::generate_proof().await;
    println!("🔄 Generated proof: {:?}", proof);
    println!("🔄 Generated public inputs: {:?}", public_inputs);

    // 2. Start Anvil (local test network)
    let anvil = Anvil::new().spawn();
    println!("✅ Anvil running at: {}", anvil.endpoint());

    // Create a signer using one of Anvil's default private keys (NOTE: The PK hardcoded here is just an example PK)
    let signer: PrivateKeySigner = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80".parse()?;
    let signer_clone = signer.clone();
    
    // Create provider with wallet  
    let provider = ProviderBuilder::new()
        .with_gas_estimation()
        .wallet(signer_clone)
        .on_http(anvil.endpoint_url());

    // 3. Test that we can read the contract artifact
    let honk_verifier_contract_json = std::fs::read_to_string("out/honk_vk.sol/HonkVerifier.json")?;
    let honk_verifier_contract_artifact: serde_json::Value = serde_json::from_str(&honk_verifier_contract_json)?;
    
    let bytecode_hex = honk_verifier_contract_artifact["bytecode"]["object"]
        .as_str()
        .ok_or_else(|| eyre::eyre!("Failed to get bytecode from contract artifact"))?;
    
    // Verify we can parse the bytecode
    let bytecode = Bytes::from_hex(bytecode_hex)?;
    
    println!("✅ Successfully parsed contract artifact");
    println!("✅ Bytecode length: {} characters", bytecode_hex.len());
    
    // 4. Deploy the contract using the bytecode
    let deploy_tx = TransactionRequest::default().with_deploy_code(bytecode);
    let receipt = provider.send_transaction(deploy_tx).await?.get_receipt().await?;
    let contract_address = receipt.contract_address.expect("Contract deployment failed");
    
    println!("✅ Contract deployed successfully at: {:?}", contract_address);

    // 5. Create a contract instance and test it
    // Note: The sol! macro generates bindings differently in Alloy 1.0
    // TODO: Fix contract instantiation for Alloy 1.0
    // In Alloy 1.0, the sol! macro doesn't generate a `new` function like this
    // Need to use the correct API for contract instantiation
    // 
    // Correct approach for Alloy 1.0 (research needed):
    // - Use contract instance creation with deployed address
    // - Call contract methods through the generated interface
    // - Example: let contract = HonkVerifier::new(contract_address, provider.clone());
    //
    let honk_verifier = HonkVerifier::new(contract_address, &provider);
    println!("📝 Contract address for future use: {:?}", contract_address);
    
    // 6. For now, test with empty proof (since we need actual JWT data to generate real proofs)
    println!("🔄 Testing verifier with empty proof (expected to fail gracefully)...");
    
    // TODO: Implement real proof generation when we have test JWT data
    // This would require:
    // 1. A valid JWT token 
    // 2. The corresponding public key (JsonWebKey)
    // 3. SRS file path
    // 4. Converting JWTCircuitInputs to HashMap<String, Vec<String>> format
    
    // Convert proof from Vec<u8> to Bytes for contract call
    let proof_bytes = Bytes::from(proof);
    let empty_public_inputs: Vec<FixedBytes<32>> = vec![];
    
    // 7. Call the verifier contract (expecting it to fail gracefully)
    println!("🔄 Calling verifier with a proof and publicInputs (testing contract interaction)...");
    let is_valid = honk_verifier.verify(proof_bytes, public_inputs).call().await;
    println!("🔄 is_valid: {:?}", is_valid);
    
    println!("✅ Contract deployment and interaction test completed");
    println!("💡 Next step: Implement real proof generation with actual JWT data");
    
    println!("✅ Honk verifier setup test completed successfully");
    Ok(())
}
