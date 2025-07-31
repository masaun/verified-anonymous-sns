use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::{LocalWallet, Signer};
use alloy::sol;
use alloy::contract::ContractInstance;
use alloy::types::{Address, U256};
use std::sync::Arc;

// 1. Define the Solidity interface using alloy::sol!
sol! {
    contract HonkVerifier {
        function verify(bytes calldata proof, bytes32[] calldata publicInputs) external view returns (bool);
    }
}

// contract HonkVerifier is BaseHonkVerifier(N, LOG_N, NUMBER_OF_PUBLIC_INPUTS) {
//      function loadVerificationKey() internal pure override returns (Honk.VerificationKey memory) {
//        return HonkVerificationKey.loadVerificationKey();
//     }
// }
//
// function verify(bytes calldata proof, bytes32[] calldata publicInputs) public view override returns (bool) {



#[tokio::test]
async fn test_honk_verifier() -> eyre::Result<()> {
    // // 2. Start Anvil with a known mnemonic or default
    // let anvil = alloy::rpc::testutils::spawn_anvil().await?;
    // let url = anvil.endpoint();
    // let provider = ProviderBuilder::new().on_http(&url)?;

    // // 3. Use the default private key
    // let wallet: LocalWallet = anvil.keys()[0].clone().into();
    // let chain_id = provider.get_chain_id().await?;
    // let wallet = wallet.with_chain_id(chain_id);
    // let client = Arc::new(provider.with_signer(wallet.clone()));

    // // 4. Deploy the contract bytecode (you must have compiled first)
    // let bytecode = std::fs::read("out/SimpleStorage.sol/SimpleStorage.bin")?;
    // let abi_path = "out/SimpleStorage.sol/SimpleStorage.abi.json";

    // let factory = alloy::contract::ContractFactory::new_from_json_file(abi_path, bytecode.into(), client.clone())?;
    // let contract = factory.deploy(())?.send().await?;

    // // 5. Call `set` and verify `get`
    // let instance = ContractInstance::<SimpleStorage>::new(contract.address(), client.clone());

    // let tx = instance.set(U256::from(42)).send().await?.await_receipt().await?;
    // assert!(tx.is_some());

    // let value = instance.get().call().await?;
    // assert_eq!(value, U256::from(42));

    // println!("✅ Storage value is: {}", value);
    Ok(())
}
