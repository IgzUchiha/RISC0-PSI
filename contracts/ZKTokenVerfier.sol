pragma solidity ^0.8.0;

import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol";

contract TokenVerifier {
    IRiscZeroVerifier public immutable verifier;
    uint256 public Token; // Declare Token as a state variable.

    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
    }

    // Set the value of Token after verification.
    function set(uint256 x, bytes calldata seal) public {
        bytes memory journal = abi.encode(x);
        require(verifier.verify(seal, ImageID.IMAGE_ID, sha256(journal)), "Verification failed");
        Token = x;
    }

    // Get the value of Token.
    function get() public view returns (uint256) {
        return Token;
    }

    // Verify a proof with the verifier.
    function verifyProof(bytes calldata proof, bytes calldata publicInputs) external {
        bytes32 publicInputsHash = sha256(publicInputs);
        require(
            verifier.verify(proof, ImageID.IMAGE_ID, publicInputsHash),
            "Proof verification failed"
        );
    }
}
