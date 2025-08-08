
// @dev - Noir
use noir::{
    barretenberg::{
        prove::prove_ultra_honk, srs::setup_srs_from_bytecode, utils::get_honk_verification_key,
        verify::verify_ultra_honk,
    },
    witness::from_vec_str_to_witness_map,
};

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



#[tokio::test]
async fn test_generate_ephemeral_key() {
    let ephemeral_key = generate_ephemeral_key();
    println!("ephemeral_key: {}", ephemeral_key);
}

#[tokio::test(flavor = "current_thread")]
async fn test_proof_generation() -> eyre::Result<()> {
    // Test proof generation using imported functions from parent crate
    println!("🔄 Starting proof generation...");

    // Example JWT token (this would be a real JWT in practice)
    let id_token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjA3YjgwYTM2NTQyODUyNWY4YmY3Y2QwODQ2ZDc0YThlZTRlZjM2MjUiLCJ0eXAiOiJKV1QifQ.eyJpc3MiOiJodHRwczovL2FjY291bnRzLmdvb2dsZS5jb20iLCJhenAiOiIxMDA2NzAxMjkzNzQ4LTFpcm1ndTkxMHAybjd2am1vYTQ0MXJhbW02ZGNydmViLmFwcHMuZ29vZ2xldXNlcmNvbnRlbnQuY29tIiwiYXVkIjoiMTAwNjcwMTI5Mzc0OC0xaXJtZ3U5MTBwMm43dmptb2E0NDFyYW1tNmRjcnZlYi5hcHBzLmdvb2dsZXVzZXJjb250ZW50LmNvbSIsInN1YiI6IjEwODUyMjA3NzcyMTgyNjQzOTM2NCIsImhkIjoicHNlLmRldiIsImVtYWlsIjoidml2aWFuamVuZ0Bwc2UuZGV2IiwiZW1haWxfdmVyaWZpZWQiOnRydWUsIm5vbmNlIjoiNjIyNjE4NzE4OTI2NDIwNDg2NDk4MTI3MDAxMDcxODU2NTA0MzIyNDkyNjUwNjU2MjgzOTM2NTk2NDc3ODY5OTY1NDU5ODg3NTQ2IiwibmJmIjoxNzQ2MDAzNzgwLCJpYXQiOjE3NDYwMDQwODAsImV4cCI6MTc0NjAwNzY4MCwianRpIjoiZmZhNGNhMWQ1NDZlZGZlOWI1Mjc0NDY3ZTE5ODJhOTgyMTU5MjRkOSJ9.naERF4rIB5L3a6I3FBC--_b25O2P6zbymSKkXHgOy44PvZU1LLSQ5ORzxHT93YIpbSzx5eF_FAMuXeN9uwLPrpFRw5Zlt9RlrbfQVNHZj1izHxj0IEYBudGESMRKjef7vfvtsYm_s_iHwE5M6H9UATi9xJw4U34iVn664xZFxhtdqbvCXW-YrjNliNK7dSEKAdHgi4MxiASlHXishGVwmFwe116c3HfEcyAJMxv9pGZEhmh4IZ7jVuwiUFEjroZ7svpGLiNx1grEnqGCJa8gcHEI4t1Lpip9d9CMuEctudLiH0Bk_bFofV-s-VvEOdFfEW8WYdE_YhKS0G9qYnevlQ";
    //let jwt = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiYWRtaW4iOnRydWV9.example";

    let ephemeral_pubkey =
        "17302102366996071265028731047581517700208166805377449770193522591062772282670";
    let ephemeral_salt =
        "646645587996092179008704451306999156519169540151959619716525865713892520";
    let ephemeral_expiry = "2025-05-07T09:07:57.379Z";

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
    let srs_path: String = "../public/jwt-srs.local".to_string(); // @dev - Path to the SRS file (relative to contracts dir)

    // Now produce the proof as usual
    let pubkey_str = serde_json::to_string(&pubkey).unwrap();
    //println!("pubkey_str: {:?}", pubkey_str);

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

    println!("✅ Proof generation test completed successfully");
    Ok(())
}