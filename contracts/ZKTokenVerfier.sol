// pragma solidity ^0.8.0;

// import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
// import {ImageID} from "./ImageID.sol";

// contract TokenVerifier {
//     IRiscZeroVerifier public immutable verifier;
//     uint256 public Token; // Declare Token as a state variable.

//     constructor(IRiscZeroVerifier _verifier) {
//         verifier = _verifier;
//     }

//     // Set the value of Token after verification.
//     function set(uint256 x, bytes calldata seal) public {
//         bytes memory journal = abi.encode(x);
//         require(verifier.verify(seal, ImageID.IMAGE_ID, sha256(journal)), "Verification failed");
//         Token = x;
//     }

//     // Get the value of Token.
//     function get() public view returns (uint256) {
//         return Token;
//     }

//     // Verify a proof with the verifier.
//     function verifyProof(bytes calldata proof, bytes calldata publicInputs) external {
//         bytes32 publicInputsHash = sha256(publicInputs);
//         require(
//             verifier.verify(proof, ImageID.IMAGE_ID, publicInputsHash),
//             "Proof verification failed"
//         );
//     }

//     // Verify transfer details and proof.
//     function verifyTransfer(
//         bytes calldata proof,
//         address sender,
//         address recipient,
//         uint256 amount
//     ) public {
//         // Encode and hash the inputs
//         bytes memory journal = abi.encode(sender, recipient, amount);
//         bytes32 journalHash = sha256(journal);

//         // Verify the proof
//         require(
//             verifier.verify(proof, ImageID.IMAGE_ID, journalHash),
//             "Verification failed"
//         );

//         // Logic after verification (e.g., updating state or transferring tokens)
//         // Example: Emit an event or integrate with a token transfer mechanism.
//     }
// }

// SPDX-License-Identifier: MIT
pragma solidity ^0.8.28;

import {Script} from "forge-std/Script.sol";
import {console2} from "forge-std/console2.sol";
import {RiscZeroCheats} from "risc0/test/RiscZeroCheats.sol";
import {IRiscZeroVerifier} from "risc0/IRiscZeroVerifier.sol";
import {Steel} from "risc0/steel/Steel.sol";

contract PrivateTokenVerifier {
    IRiscZeroVerifier public verifier;
    bytes32 public constant YOUR_IMAGE_ID = 0x0000000000000000000000000000000000000000000000000000000000000000;
 // Replace with actual image ID

    constructor(IRiscZeroVerifier _verifier) {
        verifier = _verifier;
    }

    struct Journal {
        Steel.Commitment commitment;
        address PublicTokenVerifier; // Removed `public` and `immutable`
    }

    function validate(
        bytes calldata journalData,
        bytes calldata seal,
        bytes32 _imageId
    ) external {
        Journal memory journal = abi.decode(journalData, (Journal));
        require(Steel.validateCommitment(journal.commitment), "Invalid commitment");
        verifier.verify(seal, _imageId, sha256(journalData));
    }

   function verifyTransfer(
    bytes calldata proof,
    bytes32 senderHash,
    bytes32 recipientHash,
    uint256 amount
    ) public {
    // Encode and hash the inputs
    bytes memory journal = abi.encode(senderHash, recipientHash, amount);
    bytes32 journalHash = sha256(journal);

    // Call the verifier's verify function
    verifier.verify(proof, YOUR_IMAGE_ID, journalHash);

    // Post-verification logic (e.g., updating state)
    }

}
