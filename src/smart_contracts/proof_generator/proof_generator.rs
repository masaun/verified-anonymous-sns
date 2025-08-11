
// // @dev - Noir
// use noir::{
//     barretenberg::{
//         prove::prove_ultra_honk, srs::setup_srs_from_bytecode, utils::get_honk_verification_key,
//         verify::verify_ultra_honk,
//     },
//     witness::from_vec_str_to_witness_map,
// };

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