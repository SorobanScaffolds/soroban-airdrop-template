# Soroban Airdrop Claim Template 🚀

[![CI](https://github.com/SorobanScaffolds/soroban-airdrop-template/actions/workflows/ci.yml/badge.svg)](https://github.com/SorobanScaffolds/soroban-airdrop-template/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Soroban](https://img.shields.io/badge/Soroban-v22-blue.svg)](https://soroban.stellar.org/)

A production-ready, open-source boilerplate for conducting token airdrops on the Stellar network using Soroban smart contracts, off-chain Merkle tree generators, and a modern Next.js claim portal.

---

## 🌟 Key Features

- **Soroban Merkle Contract (`contracts/src/lib.rs`)**:
  - Cryptographic Merkle proof verification for gas-efficient, arbitrary-sized recipient sets.
  - Sorted-pair leaf hashing ensuring determinism without ordering exploits.
  - Robust double-claim prevention using persistent instance storage (`DataKey::UserClaim`).
  - Standard SEP-41 token transfers via `token::Client`.
  - Secure admin access controls for root updates (`update_root`) and unallocated token recovery (`withdraw_remaining`).
- **Off-Chain Cryptographic Parity (`scripts/merkle.js`)**:
  - Byte-for-byte canonical leaf encoding:
    $$\text{Leaf} = \text{SHA-256}\left(\texttt{ScVal::Address(claimant).to\_xdr()} \mathbin{\Vert} \texttt{amount.to\_be\_bytes(16)}\right)$$
  - CLI scripts to parse recipient CSV/JSON, build trees, and export verified JSON proofs.
- **Interactive Next.js Claim UI (`frontend/`)**:
  - Live Freighter wallet integration with automatic network detection (Testnet & Mainnet).
  - Eligibility and proof lookup against generated airdrop data.
  - Safe transaction signing and duplicate submission guards.
- **Automated CI/CD (`.github/workflows/ci.yml`)**:
  - Runs cargo unit tests (8 passing tests), builds WASM bytecode, verifies cryptographic off-chain parity, and executes frontend production builds on every commit.

---

## 🏗️ Architecture

```
┌─────────────────────────────────┐       ┌──────────────────────────────────┐
│      Off-Chain Pipeline         │       │        On-Chain Contract         │
│  (scripts/generate_merkle.js)   │       │      (contracts/src/lib.rs)      │
├─────────────────────────────────┤       ├──────────────────────────────────┤
│ 1. recipients.json              │       │                                  │
│    [{ address, amount }, ...]   │       │                                  │
│                 │               │       │                                  │
│ 2. Canonical Leaf Encoding      │       │                                  │
│    SHA-256(Address || Amount)   │       │                                  │
│                 │               │       │                                  │
│ 3. Sorted-Pair Merkle Tree      │───────┼─> init(admin, token, root)       │
│    Computes 32-byte Merkle Root │       │                                  │
│                 │               │       │                                  │
│ 4. Proof Export for UI          │       │                                  │
│    merkle_tree_output.json      │       │                                  │
└────────────────┬────────────────┘       └─────────────────┬────────────────┘
                 │                                          │
                 ▼                                          ▼
       ┌──────────────────┐                       ┌──────────────────┐
       │   Claimant UI    │───[ Freighter Sign ]─>│  claim(claimant, │
       │   (frontend/)    │                       │  amount, proof)  │
       └──────────────────┘                       └──────────────────┘
```

---

## 🛠️ Smart Contract Specification

### Contract Functions

| Function | Parameters | Description |
| :--- | :--- | :--- |
| `init` | `admin: Address, token: Address, merkle_root: BytesN<32>` | Initializes the airdrop contract once with the admin, distribution token, and Merkle root. |
| `claim` | `claimant: Address, amount: i128, proof: Vec<BytesN<32>>` | Verifies claimant signature, validates Merkle proof, checks for double-claiming, and transfers tokens. |
| `update_root` | `new_root: BytesN<32>` | Allows admin to update the Merkle root for subsequent distribution rounds. |
| `withdraw_remaining`| `to: Address` | Allows admin to withdraw remaining unallocated tokens after the airdrop concludes. |
| `is_claimed` | `user: Address` | Returns `bool` indicating if the user has already claimed their tokens. |
| `compute_leaf` | `claimant: Address, amount: i128` | Helper returning the canonical 32-byte leaf hash for a given claimant and amount. |
| `verify_proof` | `proof: Vec<BytesN<32>>, root: BytesN<32>, leaf: BytesN<32>` | Helper returning `bool` for proof verification. |
| `get_merkle_root` | *none* | Returns the currently active 32-byte Merkle root. |

---

## 🧪 Verification & Testing

### 1. Rust Unit Tests
The contract test suite tests all edge cases and boundary conditions:
```bash
cargo test --manifest-path contracts/Cargo.toml
```
**Test Matrix (8/8 Passing)**:
- `test_initialization`: Verifies one-time initialization guard.
- `test_update_root`: Verifies admin authorization for updating Merkle roots.
- `test_successful_claim_and_double_claim_prevention`: Verifies valid claim, token transfer, and verifies subsequent claim attempt is rejected.
- `test_claim_invalid_proof`: Ensures forged proofs revert.
- `test_claim_wrong_amount`: Ensures tampered amounts revert.
- `test_claim_zero_amount`: Rejects non-positive claim amounts.
- `test_admin_withdraw_remaining`: Verifies remaining funds recovery.
- `test_offchain_consistency`: Confirms exact cryptographic leaf hash match with off-chain JavaScript tooling.

### 2. WASM Compilation
```bash
cargo build --manifest-path contracts/Cargo.toml --target wasm32-unknown-unknown --release
```
Builds `contracts/target/wasm32-unknown-unknown/release/soroban_airdrop_contract.wasm` (19.3 KB).

### 3. Merkle Generator Verification
```bash
node scripts/generate_merkle_tree.js data/sample_recipients.json data/merkle_tree_output.json
node scripts/verify_merkle_tree.js data/merkle_tree_output.json
```

---

## 🌐 Live Stellar Testnet Evidence

Live end-to-end verification was conducted on Stellar Testnet using dedicated accounts:
- **Verified Timestamp**: `2026-10-13T15:02:27Z`
- **Network**: Stellar Testnet (`https://horizon-testnet.stellar.org`)
- **Ledger Sequence**: `5176032`
- **Admin Account**: [`GBVZFSSQOGIL6VZ27QQJABGIYSK4B45HZIQOS5BDBKZJOSQPE5VIEJVT`](https://stellar.expert/explorer/testnet/account/GBVZFSSQOGIL6VZ27QQJABGIYSK4B45HZIQOS5BDBKZJOSQPE5VIEJVT)
- **Claimant 1 (Eligible)**: [`GDDQA3JUT3LUF3ZJ36VOBH6VMT62YLOUVWQDNHF4R6H5ZCXAQNOURCIH`](https://stellar.expert/explorer/testnet/account/GDDQA3JUT3LUF3ZJ36VOBH6VMT62YLOUVWQDNHF4R6H5ZCXAQNOURCIH)
- **Claimant 2 (Unauthorized)**: [`GD6KCVYJREOW3XE4D64SWK6JE25CJYFMSIJZ25QFWN5OMRCGRFEOAYSP`](https://stellar.expert/explorer/testnet/account/GD6KCVYJREOW3XE4D64SWK6JE25CJYFMSIJZ25QFWN5OMRCGRFEOAYSP)
- **Merkle Root**: `0x66e3513bcac762c8ccf4d648aff21fbbde32783d3bfefc85f2b665e294d402b2`
- **Live Transaction Hash**: [`f3dd58082684ae48c825e656b133a741d2a7c6ffed8becf66e7f879cf24c39ab`](https://stellar.expert/explorer/testnet/tx/f3dd58082684ae48c825e656b133a741d2a7c6ffed8becf66e7f879cf24c39ab)
- **Observed Results**:
  - Claimant 1 received +50 XLM distribution on-chain.
  - Rejection Case 1: Tampered claim amount correctly rejected by Merkle verification.
  - Rejection Case 2: Unauthorized claimant attempting to use another user's proof correctly rejected.
  - Rejection Case 3: Double-claim attempt correctly blocked by duplicate claim guard.

---

## 💻 Running the Frontend

```bash
cd frontend
npm install
npm run dev
```
Open [http://localhost:3000](http://localhost:3000) to view the claim portal.

To build for production:
```bash
npm run build
```

---

## 🤝 Contributing

We welcome community contributions! Please review [CONTRIBUTING.md](./CONTRIBUTING.md) for contribution guidelines.

### Contributor Backlog
Check our [Issues](https://github.com/SorobanScaffolds/soroban-airdrop-template/issues) for open tasks, including:
- Merkle tree batching optimization for large recipient lists (>100k addresses).
- On-chain event indexing service for claim tracking.
- CSV upload and validation modal in the admin UI.

---

## 📜 License

This project is licensed under the [MIT License](./LICENSE).
