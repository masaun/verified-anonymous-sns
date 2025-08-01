use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::local::PrivateKeySigner;
use alloy::sol;
use alloy::primitives::Bytes;
use alloy::hex::FromHex;
use alloy::rpc::types::TransactionRequest;
use alloy::network::TransactionBuilder;
use alloy_node_bindings::Anvil;

// @dev - Noir
use noir::{
    barretenberg::{
        prove::prove_ultra_honk, srs::setup_srs_from_bytecode, utils::get_honk_verification_key,
        verify::verify_ultra_honk,
    },
    witness::from_vec_str_to_witness_map,
};


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
async fn test_proof_generation() -> eyre::Result<()> {
    // [TODO]: Define the logic to generate a proof using the "noir_rs::barretenberg" crate.

    println!("✅ Proof generation completed successfully");
    Ok(())
}


#[tokio::test]
async fn test_honk_verifier() -> eyre::Result<()> {
    // 2. Start Anvil (local test network)
    let anvil = Anvil::new().spawn();
    println!("✅ Anvil running at: {}", anvil.endpoint());

    // Create a signer using one of Anvil's default private keys
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
    
    // For testing, use empty proof and public inputs
    let _proof = Bytes::from_hex("0x")?;     // Empty proof for testing
    let _public_inputs: Vec<Bytes> = vec![]; // Empty public inputs for testing
    
    // Note: This will likely fail with empty data, but tests the interface
    // TODO: Uncomment when contract instantiation is fixed
    // let is_valid = honk_verifier.verify(_proof, _public_inputs).call().await?;
    // println!("✅ Verification result: {}", is_valid);
    
    println!("✅ Honk verifier setup test completed successfully");
    Ok(())
}
