const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("ZKTokenVerifier", function () {
  let Token;
  let ZKTokenVerifier;
  let token;
  let zkTokenVerifier;
  let owner;
  let addr1;
  let bonsaiMock; // Mock Bonsai contract

  beforeEach(async function () {
    [owner, addr1] = await ethers.getSigners();

    // Deploy Token contract
    Token = await ethers.getContractFactory("Token");
    token = await Token.deploy("Test Token", "TTK", 1000000);
    await token.deployed();

    // Deploy Mock Bonsai contract (replace with your actual Bonsai implementation)
    const BonsaiMock = await ethers.getContractFactory("BonsaiMock"); // Create a mock contract for testing
    bonsaiMock = await BonsaiMock.deploy();
    await bonsaiMock.deployed();

    // Deploy ZKTokenVerifier contract
    ZKTokenVerifier = await ethers.getContractFactory("ZKTokenVerifier");
    zkTokenVerifier = await ZKTokenVerifier.deploy(bonsaiMock.address, token.address);
    await zkTokenVerifier.deployed();
  });

  describe("requestComputation", function () {
    it("should mint tokens if proof is verified", async function () {
      const input = ethers.utils.formatBytes32String("valid proof");

      // Simulate a valid proof from Bonsai
      await expect(zkTokenVerifier.requestComputation(input))
        .to.emit(zkTokenVerifier, "ProofVerified")
        .withArgs(owner.address, ethers.utils.keccak256(input));

      const balance = await token.balanceOf(owner.address);
      expect(balance).to.equal(100 * (10**18)); // 100 tokens were minted
    });

    it("should revert if proof verification fails", async function () {
      const invalidInput = ethers.utils.formatBytes32String("invalid proof");

      // Simulate proof failure in Bonsai
      await expect(zkTokenVerifier.requestComputation(invalidInput))
        .to.be.revertedWith("Proof verification failed");
    });
  });
});
