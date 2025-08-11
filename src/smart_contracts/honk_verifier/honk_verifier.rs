// 1. Define the Solidity interface using alloy::sol!
sol! {
    #[sol(rpc)]
    HonkVerifier,
    "out/honk_vk.sol/HonkVerifier.json"
}