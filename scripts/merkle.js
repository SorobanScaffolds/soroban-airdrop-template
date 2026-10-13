const crypto = require('crypto');
const { Address, xdr } = require('@stellar/stellar-sdk');

/**
 * Computes the canonical Merkle leaf for an airdrop recipient.
 * Matches Soroban smart contract `AirdropContract::compute_leaf`:
 * Leaf = SHA-256( ScVal::Address(claimant).to_xdr() || amount.to_be_bytes(16) )
 */
function computeLeaf(addressStr, amountStroops) {
  const addr = Address.fromString(addressStr);
  const scVal = xdr.ScVal.scvAddress(addr.toScAddress());
  const addrXdr = scVal.toXDR();

  const amtBig = BigInt(amountStroops);
  const amtBuf = Buffer.alloc(16);
  amtBuf.writeBigInt64BE(amtBig >> 64n, 0);
  amtBuf.writeBigInt64BE(amtBig & 0xFFFFFFFFFFFFFFFFn, 8);

  const payload = Buffer.concat([addrXdr, amtBuf]);
  return crypto.createHash('sha256').update(payload).digest();
}

/**
 * Sorted pair hashing matching Soroban contract verify_merkle_proof:
 * If a <= b, hash(a || b), else hash(b || a).
 */
function hashPair(a, b) {
  const combined = Buffer.compare(a, b) <= 0
    ? Buffer.concat([a, b])
    : Buffer.concat([b, a]);
  return crypto.createHash('sha256').update(combined).digest();
}

class MerkleTree {
  constructor(leaves) {
    this.leaves = leaves;
    this.layers = [leaves];
    this.build();
  }

  build() {
    let current = this.layers[0];
    while (current.length > 1) {
      const next = [];
      for (let i = 0; i < current.length; i += 2) {
        if (i + 1 < current.length) {
          next.push(hashPair(current[i], current[i + 1]));
        } else {
          // Odd element carries over
          next.push(current[i]);
        }
      }
      this.layers.push(next);
      current = next;
    }
  }

  getRoot() {
    return this.layers[this.layers.length - 1][0];
  }

  getProof(index) {
    const proof = [];
    let idx = index;
    for (let layerIdx = 0; layerIdx < this.layers.length - 1; layerIdx++) {
      const layer = this.layers[layerIdx];
      const isRight = idx % 2 === 1;
      const siblingIdx = isRight ? idx - 1 : idx + 1;
      if (siblingIdx < layer.length) {
        proof.push(layer[siblingIdx]);
      }
      idx = Math.floor(idx / 2);
    }
    return proof;
  }

  static verify(leaf, proof, root) {
    let current = leaf;
    for (const p of proof) {
      current = hashPair(current, p);
    }
    return Buffer.compare(current, root) === 0;
  }
}

module.exports = { computeLeaf, hashPair, MerkleTree };
