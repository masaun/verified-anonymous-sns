use std::hash::poseidon2::Poseidon2;

/** 
 * @notice - Calculate a nullifier
 */
pub fn generate_nullifier(inputs: [Field; _]) -> Field {
    let nullifier_hash = Poseidon2::hash(
        inputs,
        inputs.len() as u32
    );
    nullifier_hash
}