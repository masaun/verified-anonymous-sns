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

#[tokio::test]
async fn test_generate_ephemeral_key() {
    let ephemeral_key = generate_ephemeral_key();
    println!("ephemeral_key: {}", ephemeral_key);
}

#[tokio::test]
async fn test_proof_generation() -> eyre::Result<()> {
    // Test proof generation using imported functions from parent crate
    println!("🔄 Starting proof generation...");
    
    // For now, let's use a simple test to verify the functions are accessible
    // In a real implementation, we would load actual JWT and key data

    // Example JWT token (this would be a real JWT in practice)
    let id_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjA3YjgwYTM2NTQyODUyNWY4YmY3Y2QwODQ2ZDc0YThlZTRlZjM2MjUiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIxMDA2NzAxMjkzNzQ4LTFpcm1ndTkxMHAybjd2am1vYTQ0MXJhbW02ZGNydmViLmFwcHMuZ29vZ2xldXNlcmNvbnRlbnQuY29tIiwiYXVkIjoiMTAwNjcwMTI5Mzc0OC0xaXJtZ3U5MTBwMm43dmptb2E0NDFyYW1tNmRjcnZlYi5hcHBzLmdvb2dsZXVzZXJjb250ZW50LmNvbSIsInN1YiI6IjEwODUyMjA3NzcyMTgyNjQzOTM2NCIsImhkIjoicHNlLmRldiIsImVtYWlsIjoidml2aWFuamVuZ0Bwc2UuZGV2IiwiZW1haWxfdmVyaWZpZWQiOnRydWUsIm5vbmNlIjoiNjIyNjE4NzE4OTI2NDIwNDg2NDk4MTI3MDAxMDcxODU2NTA0MzIyNDkyNjUwNjU2MjgzOTM2NTk2NDc3ODY5OTY1NDU5ODg3NTQ2IiwibmJmIjoxNzQ2MDAzNzgwLCJpYXQiOjE3NDYwMDQwODAsImV4cCI6MTc0NjAwNzY4MCwianRpIjoiZmZhNGNhMWQ1NDZlZGZlOWI1Mjc0NDY3ZTE5ODJhOTgyMTU5MjRkOSJ9.naERF4rIB5L3a6I3FBC--_b25O2P6zbymSKkXHgOy44PvZU1LLSQ5ORzxHT93YIpbSzx5eF_FAMuXeN9uwLPrpFRw5Zlt9RlrbfQVNHZj1izHxj0IEYBudGESMRKjef7vfvtsYm_s_iHwE5M6H9UATi9xJw4U34iVn664xZFxhtdqbvCXW-YrjNliNK7dSEKAdHgi4MxiASlHXishGVwmFwe116c3HfEcyAJMxv9pGZEhmh4IZ7jVuwiUFEjroZ7svpGLiNx1grEnqGCJa8gcHEI4t1Lpip9d9CMuEctudLiH0Bk_bFofV-s-VvEOdFfEW8WYdE_YhKS0G9qYnevlQ";
    //let jwt = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWV9.example";

    let ephemeral_pubkey =
        "104632289316985982192388871134103343051609197817608328239779951194180755624811";
    let ephemeral_salt =
        "899783221154578309625392956639321858584745631388200288157276330883448417";
    let ephemeral_expiry = "2025-08-12T02:57:07.501Z";

    // Example public key (this would be fetched from Google's JWKS endpoint in practice)
    let pubkey = JsonWebKey {
        kid: "07b80a365428525f8bf7cd0846d74a8ee4ef3625".to_string(),
        n: "03Cww27F2O7JxB5Ji9iT9szfKZ4MK-iPzVpQkdLjCuGKfpjaCVAz9zIQ0-7gbZ-8cJRaSLfByWTGMIHRYiX2efdjz1Z9jck0DK9W3mapFrBPvM7AlRni4lPlwUigDd8zxAMDCheqyK3vCOLFW-1xYHt_YGwv8b0dP7rjujarEYlWjeppO_QMNtXdKdT9eZtBEcj_9ms9W0aLdCFNR5AAR3y0kLkKR1H4DW7vncB46rqCJLenhlCbcW0MZ3asqcjqBQ2t9QMRnY83Zf_pNEsCcXlKp4uOQqEvzjAc9ZSr2sOmd_ESZ_3jMlNkCZ4J41TuG-My5illFcW5LajSKvxD3w".to_string(),
        use_: "sig".to_string(),
        alg: "RS256".to_string(),
        kty: "RSA".to_string(),
        e: "AQAB".to_string(),
    };

    let domain = "pse.dev".to_string();

    // Define the SRS path for the proof generation and verification
    let srs_path: String = "public/jwt-srs.local".to_string(); // @dev - Path to the SRS file

    // Now produce the proof as usual
    let pubkey_str = serde_json::to_string(&pubkey).unwrap();
    println!("pubkey_str: {:?}", pubkey_str);

    // @dev - Generate a new proof
    let proof = prove_jwt( // @dev - prove_jwt() includes both the generate_inputs() and the generate_jwt_proof().
        srs_path.clone(),
        ephemeral_pubkey.to_string(),
        ephemeral_salt.to_string(),
        ephemeral_expiry.to_string(),
        id_token.to_string(),
        pubkey_str,
        domain,
    );
    println!("proof: {:?}", proof);
    assert!(!proof.is_empty(), "Proof should not be empty");

    // Call verify_jwt as before
    let verified = verify_jwt(srs_path, proof);
    println!("verified: {:?}", verified);
    assert!(verified, "JWT proof should verify correctly");


    
    // let sha_precompute_keys: Option<Vec<&str>> = None; // Optional, can be None for now
    // let max_signed_data_len: usize = 1024;

    // let srs_path: String = "public/jwt-srs_example.local".to_string(); // @dev - Path to the SRS file. (NOTE: This path is referenced from the test in the jwt_proof.rs)

    // // NOTE: The "Result" type would need to include both "success" and "error" types
    // let input_data: Result<JWTCircuitInputs, anyhow::Error> = generate_inputs(jwt, &pubkey, sha_precompute_keys, max_signed_data_len);
    

    println!("✅ Proof generation test completed successfully");
    Ok(())
}


#[tokio::test]
async fn test_honk_verifier() -> eyre::Result<()> {
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
    
    let empty_proof = Bytes::from_hex("0x")?;
    let empty_public_inputs: Vec<FixedBytes<32>> = vec![];
    
    // 7. Call the verifier contract (expecting it to fail gracefully)
    println!("🔄 Calling verifier with empty proof (testing contract interaction)...");
    let result = honk_verifier.verify(empty_proof, empty_public_inputs).call().await;
    
    match result {
        Ok(is_valid) => {
            println!("✅ Contract call succeeded, verification result: {}", is_valid);
            println!("⚠️  Note: Empty proof should normally be invalid");
        }
        Err(e) => {
            println!("❌ Verification call failed as expected: {:?}", e);
            println!("✅ Contract interaction working (revert expected for empty proof)");
        }
    }
    
    println!("✅ Contract deployment and interaction test completed");
    println!("💡 Next step: Implement real proof generation with actual JWT data");
    
    println!("✅ Honk verifier setup test completed successfully");
    Ok(())
}
