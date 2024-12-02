pragma solidity ^0.8.0;

// import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {ImageID} from "./ImageID.sol"; // auto-generated contract after running `cargo build`.
import {Token} from "./Token.sol"; 
import {TokenVerifier} from "./ZKTokenVerfier.sol"; 
contract TokenVerifier {
    IRiscZeroVerifier public immutable verifier;

   constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
        tokens = x > 0;
    }

       function set(uint256 x, bytes calldata seal) public {
        // Construct the expected journal data. Verify will fail if journal does not match.
        bytes memory journal = abi.encode(x);
        verifier.verify(seal, imageId, sha256(journal));
        tokens = x;
    }

     function get() public view returns (uint256) {
        return tokens;
    }
    // The proof parameter should include the actual proof, 
    // and you may need to pass the public inputs as well
    function verifyProof(bytes calldata proof, bytes calldata publicInputs) external {
        require(verifier.verify(proof, publicInputs), "Proof verification failed");
    }
}
