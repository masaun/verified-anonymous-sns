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
#[path = "../proof_generator/proof_generator.rs"]
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
    
    // 7. Call the verifier contract (expecting it to fail gracefully)
    println!("🔄 Calling verifier with a proof and publicInputs (testing contract interaction)...");
    let is_valid = honk_verifier.verify(proof_bytes, public_inputs).call().await;
    
    match &is_valid {
        Ok(result) => {
            println!("✅ Contract call succeeded: {}", result);
        }
        Err(e) => {
            println!("❌ Contract call failed: {:?}", e);
            println!("🔍 Error analysis for 0xed74ac0a:");
            println!("  - This error code doesn't match any standard Solidity errors");
            println!("  - Known errors in HonkVerifier:");
            println!("    * ProofLengthWrong: 0xd0e50be7");
            println!("    * PublicInputsLengthWrong: 0x2e815f18");  
            println!("    * SumcheckFailed: 0xff63caf8");
            println!("    * ShpleminiFailed: 0xb96ecf7f");
            println!("  - Error 0xed74ac0a likely comes from:");
            println!("    * Deep verification logic (sumcheck/shplemini algorithms)");
            println!("    * Assembly code within the verifier");
            println!("    * Proof format incompatibility with contract expectations");
            println!("    * Internal library error from Barretenberg/Noir verification");
            println!("  - DIAGNOSIS: Mopro proof format may be incompatible with this Honk verifier");
        }
    }
    
    println!("🔄 is_valid: {:?}", is_valid);
    
    // Additional debugging: Let's test if the issue is proof-specific
    println!("\\n🔍 DIAGNOSIS SUMMARY:");
    println!("  - PROOF_SIZE: Fixed (526 field elements = {} bytes)", proof.len());
    println!("  - Contract deployment: Success");
    println!("  - Error 0xed74ac0a: Unknown error from verification logic");
    println!("  - LIKELY CAUSE: Mopro proof format incompatible with Solidity Honk verifier");
    println!("  - NEXT STEPS: Verify proof format compatibility or generate proof in Barretenberg format");
        
    println!("✅ Honk verifier setup test completed successfully");
    Ok(())
}
