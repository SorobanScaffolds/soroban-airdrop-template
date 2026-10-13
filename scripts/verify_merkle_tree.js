#!/usr/bin/env node

/**
 * Verify a Merkle proof for an address and amount.
 * Usage: node scripts/verify_merkle_tree.js [path/to/merkle_tree_output.json] [address]
 */

const fs = require('fs');
const path = require('path');
const { computeLeaf, MerkleTree } = require('./merkle');

const defaultTree = path.join(__dirname, '../data/merkle_tree_output.json');
const treeFile = process.argv[2] || defaultTree;
const targetAddress = process.argv[3];

if (!fs.existsSync(treeFile)) {
  console.error(`Error: Tree file ${treeFile} not found.`);
  process.exit(1);
}

const data = JSON.parse(fs.readFileSync(treeFile, 'utf-8'));
const rootBuffer = Buffer.from(data.merkleRoot, 'hex');

console.log(`Verifying Merkle Tree from ${treeFile}`);
console.log(`Merkle Root: ${data.merkleRoot}`);

if (targetAddress) {
  const claim = data.claims[targetAddress];
  if (!claim) {
    console.error(`Address ${targetAddress} not found in airdrop claims list.`);
    process.exit(1);
  }
  const leaf = computeLeaf(targetAddress, claim.amount);
  const proofBuffers = claim.proof.map((p) => Buffer.from(p, 'hex'));
  const isValid = MerkleTree.verify(leaf, proofBuffers, rootBuffer);
  console.log(`Verification for ${targetAddress}: ${isValid ? 'VALID' : 'INVALID'}`);
  process.exit(isValid ? 0 : 1);
}

// Otherwise verify all claims in file
let validCount = 0;
let invalidCount = 0;

for (const [address, claim] of Object.entries(data.claims)) {
  const leaf = computeLeaf(address, claim.amount);
  const proofBuffers = claim.proof.map((p) => Buffer.from(p, 'hex'));
  const isValid = MerkleTree.verify(leaf, proofBuffers, rootBuffer);
  if (isValid) {
    validCount++;
  } else {
    invalidCount++;
    console.error(`Claim for ${address} FAILED verification!`);
  }
}

console.log(`Verification complete: ${validCount} valid, ${invalidCount} invalid.`);
if (invalidCount > 0) process.exit(1);
