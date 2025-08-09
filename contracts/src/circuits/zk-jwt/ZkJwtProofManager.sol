pragma solidity >=0.8.21;

import { ZkJwtProofVerifier } from "./ZkJwtProofVerifier.sol";
import { DataType } from "../dataType/DataType.sol";

/**
 * @notice - This contract is used to manage the position and salary proof (1024-bit DKIM signature) with its publicInputs.
 */
contract ZkJwtProofManager {
    using DataType for DataType.PublicInput;

    ZkJwtProofVerifier public zkJwtProofVerifier;

    // @dev - Storages
    mapping(bytes32 nullifierHash => DataType.PublicInput) public publicInputsOfZkJwtProofs;  // nullifierHash -> PublicInput    
    mapping(bytes32 nullifierHash => bool isNullified) public nullifiers;
    DataType.PublicInput[] public publicInputsOfAllProofs;  // The publicInputs of all ZK-JWT proofs to show the list of all proofs related data on FE (front-end).

    constructor(
        ZkJwtProofVerifier _zkJwtProofVerifier
    ) {
        zkJwtProofVerifier = _zkJwtProofVerifier;
    }

    /**
     * @notice - Record the publicInputs of a given ZK-JWT proof on-chain.
     */
    function recordPublicInputsOfZkJwtProof(
        bytes calldata proof, 
        bytes32[] calldata publicInputs,
        DataType.PublicInput memory separatedPublicInputs // @dev - To avoid the "Stack too deep" error, a given publicInputs would be stored in the form of the struct data (= DataType.PublicInput)
    ) public returns (bool) {
        // @dev - Verify a ZK-JWT proof
        bool result = zkJwtProofVerifier.verifyZkJwtProof(proof, publicInputs);
        require(result, "A given ZK-JWT proof is not valid");

        // @dev - Record a publicInput of a given ZK-JWT proof
        DataType.PublicInput memory publicInput;
        //publicInput.jwtPubkeyModulusLimbs = separatedPublicInputs.jwtPubkeyModulusLimbs;
        publicInput.domain = separatedPublicInputs.domain;
        publicInput.nullifierHash = separatedPublicInputs.nullifierHash;
        publicInput.createdAt = separatedPublicInputs.createdAt;
        //publicInput.createdAt = block.timestamp;

        // @dev - Store the publicInput of a given ZK-JWT proof
        publicInputsOfZkJwtProofs[publicInput.nullifierHash] = publicInput;

        // @dev - Checking whether a given nullifierHash is already used or not for preventing from double spending of a given proof.
        require(nullifiers[publicInput.nullifierHash] == false, "A given nullifierHash is already used, which means a given proof is already used");

        // @dev - Store the nullifierHash to prevent double submission of the same email
        nullifiers[publicInput.nullifierHash] = true;

        // @dev - Store the publicInputs into the list of all proofs to be displayed on the UI (front-end).
        publicInputsOfAllProofs.push(publicInput);
    }

    /**
     * @notice - Retrieve the publicInputs of a given proof from on-chain.
     * @dev - When a proof is stored with publicInput into the this smart contract via the recordPositionAndSalaryProof(), the given proof is verfied by the validation. 
     *        Hence, the publicInput is guaranteed to be valid and a proof does not need to be specified in this function.
     */
    //function getPublicInputsOfPositionAndSalaryProof(bytes32 nullifierHash) public view returns (bytes32[] memory _publicInput) {
    function getPublicInputsOfZkJwtProof(bytes32 nullifierHash) public view returns (DataType.PublicInput memory _publicInput) {
        require(nullifiers[nullifierHash] == true, "A given nullifierHash is invalid"); // Double spending (of proof) prevention
        return publicInputsOfZkJwtProofs[nullifierHash];
    }

    /**
     * @notice - Retrieve the publicInputs of all proofs from on-chain to be displayed on the UI (front-end).
     */
    //function getPublicInputsOfAllProofs() public view returns (bytes32[] memory _publicInputsOfAllProofs) {
    function getPublicInputsOfAllProofs() public view returns (DataType.PublicInput[] memory) {
        return publicInputsOfAllProofs;
    }
}