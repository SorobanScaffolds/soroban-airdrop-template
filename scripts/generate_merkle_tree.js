#!/usr/bin/env node

/**
 * Generate Merkle Tree from airdrop recipients list.
 * Usage: node scripts/generate_merkle_tree.js [path/to/recipients.json] [output/path.json]
 */

const fs = require('fs');
const path = require('path');
const { computeLeaf, MerkleTree } = require('./merkle');

const defaultInput = path.join(__dirname, '../data/sample_recipients.json');
const defaultOutput = path.join(__dirname, '../data/merkle_tree_output.json');

const inputFile = process.argv[2] || defaultInput;
const outputFile = process.argv[3] || defaultOutput;

if (!fs.existsSync(inputFile)) {
  console.error(`Error: Input file ${inputFile} not found.`);
  process.exit(1);
}

const recipients = JSON.parse(fs.readFileSync(inputFile, 'utf-8'));
console.log(`Loaded ${recipients.length} recipients from ${inputFile}`);

const leaves = recipients.map((r, i) => {
  const leaf = computeLeaf(r.address, r.amount);
  return {
    index: i,
    address: r.address,
    amount: r.amount,
    leafBuffer: leaf,
    leafHex: leaf.toString('hex'),
  };
});

const tree = new MerkleTree(leaves.map((l) => l.leafBuffer));
const root = tree.getRoot();
const rootHex = root.toString('hex');

console.log(`\n========================================`);
console.log(`Merkle Root: ${rootHex}`);
console.log(`========================================\n`);

const outputData = {
  merkleRoot: rootHex,
  merkleRootBytes32: Array.from(root),
  totalRecipients: recipients.length,
  generatedAt: new Date().toISOString(),
  claims: {},
};

leaves.forEach((l) => {
  const proofBuffers = tree.getProof(l.index);
  const proofHex = proofBuffers.map((p) => p.toString('hex'));
  const proofBytes32 = proofBuffers.map((p) => Array.from(p));

  // Verify proof locally
  const isValid = MerkleTree.verify(l.leafBuffer, proofBuffers, root);
  if (!isValid) {
    throw new Error(`Self-verification failed for recipient ${l.address}`);
  }

  outputData.claims[l.address] = {
    amount: l.amount,
    leaf: l.leafHex,
    proof: proofHex,
    proofBytes32: proofBytes32,
  };
});

fs.mkdirSync(path.dirname(outputFile), { recursive: true });
fs.writeFileSync(outputFile, JSON.stringify(outputData, null, 2), 'utf-8');

console.log(`Successfully generated proofs and wrote output to: ${outputFile}`);
