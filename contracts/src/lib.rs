// Library for test_zkjwt_contracts
// This is primarily a Foundry (Solidity) project with Rust integration tests

pub mod utils {
    pub fn init() {
        println!("test_zkjwt_contracts library initialized");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        utils::init();
        assert_eq!(2 + 2, 4);
    }
}
