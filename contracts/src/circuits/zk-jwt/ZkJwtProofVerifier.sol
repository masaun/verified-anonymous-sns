pragma solidity >=0.8.21;

import { HonkVerifier } from "./honk-verifier/honk_vk.sol";

/**
 * @notice - The ZkJwtProofVerifier contract is used to verify a ZK-JWT proof, which enable to verify whether a work email domain is affiliated with a company.
 */
contract ZkJwtProofVerifier {
    HonkVerifier public verifier;

    constructor(HonkVerifier _verifier) {
        verifier = _verifier;
    }

    function verifyZkJwtProof(bytes calldata proof, bytes32[] calldata publicInputs) public view returns (bool) {
        bool proofResult = verifier.verify(proof, publicInputs);
        require(proofResult, "Proof is not valid");
        return proofResult;
    }
}
