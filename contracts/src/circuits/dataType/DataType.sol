pragma solidity >=0.8.21;

library DataType {

    struct PublicInput {
        //string jwtPubkeyModulusLimbs;
        string domain;
        bytes32 nullifierHash;
        string createdAt;     // @dev - ISO String format (i.e. "2025-07-16T07:20:30.000Z")
        //uint256 createdAt;  // @dev - block.timestamp
    }

}