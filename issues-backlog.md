# Drips Wave Issues Backlog 📋

This document contains a ready-to-copy list of issues for your GitHub repository. When you create your repo, create a new issue for each of these items. Apply the labels suggested to help contributors find them easily.

---

## 🛠️ Smart Contract Issues (Rust / Soroban)

### Issue 1: Implement `claim` function with Merkle Proof Verification
**Labels**: `wave-eligible`, `smart-contract`, `rust`
**Description**:
We need a robust `claim` function in our Soroban smart contract. The function must accept:
1. The claimant's address (Address).
2. The amount of tokens they are eligible for (i128).
3. The Merkle Proof (Vec<BytesN<32>>) that verifies they are part of the Merkle Root stored on the contract.

**Acceptance Criteria:**
- The function correctly hashes the claimant's address and amount.
- It verifies the hash against the stored Merkle Root using the provided proof.
- It transfers the tokens if the proof is valid.
- It records that the address has claimed to prevent double-claiming.

### Issue 2: Create Contract Tests for Airdrop Initialization
**Labels**: `wave-eligible`, `smart-contract`, `testing`
**Description**:
We need to write robust tests using the `soroban-sdk` test framework to ensure the contract initializes correctly.
The initialization should set the Admin Address, Token Address, and the initial Merkle Root.

**Acceptance Criteria:**
- Test successful initialization.
- Test that initializing twice throws an error.
- Test that non-admins cannot change the Merkle Root.

### Issue 3: Implement `update_merkle_root` admin function
**Labels**: `wave-eligible`, `good first issue`, `smart-contract`
**Description**:
Create a function `update_merkle_root(admin: Address, new_root: BytesN<32>)`. It should verify that the caller is the admin before updating the root in the contract's storage.

---

## 💻 Frontend Issues (Next.js / TypeScript)

### Issue 4: Scaffold the Landing Page UI
**Labels**: `wave-eligible`, `frontend`, `ui/ux`
**Description**:
Create a beautiful landing page (`frontend/src/app/page.tsx`) for the Airdrop claim site. 
It should have:
1. A hero section with a title "Claim Your Airdrop".
2. A description of the project.
3. A large CTA button that will eventually link to the Freighter wallet connection.
Please use Tailwind CSS and keep the design sleek and modern (dark mode preferred).

### Issue 5: Integrate Freighter Wallet Connection Hook
**Labels**: `wave-eligible`, `frontend`, `web3`
**Description**:
We need to connect users to the Stellar network using the Freighter wallet.
Create a custom React hook `useFreighter()` in `frontend/src/hooks/` that utilizes `@stellar/freighter-api` to:
- Check if Freighter is installed.
- Request connection and return the user's public key.
- Provide a `disconnect` function.

### Issue 6: Build the 'Check Eligibility' Component
**Labels**: `wave-eligible`, `frontend`, `react`
**Description**:
Create a component `EligibilityChecker.tsx`. 
It should feature an input field where a user can paste their Stellar public key (or use their connected Freighter wallet). 
Upon clicking "Check", it should (for now) display a mock loading state, then show a success message "You are eligible for 1000 TOKENS" or an error "Address not eligible".

### Issue 7: Create the Airdrop Claim Transaction Builder
**Labels**: `wave-eligible`, `frontend`, `soroban`
**Description**:
Using the `stellar-sdk` and `soroban-client`, write a utility function in `frontend/src/utils/soroban.ts` that builds the Soroban transaction to call the `claim` function on our smart contract. It should accept the claimant address, amount, and the Merkle proof array.

### Issue 8: Setup Global State (Zustand or Context API)
**Labels**: `wave-eligible`, `good first issue`, `frontend`
**Description**:
Set up a simple global state to track:
- Is the wallet connected?
- What is the connected address?
- What is the user's eligibility status?
We recommend using `zustand` for this, but standard React Context is also fine.

---

## ⚙️ Backend / Scripting Issues (Node.js)

### Issue 9: Create Merkle Tree Generator Script
**Labels**: `wave-eligible`, `backend`, `scripting`
**Description**:
We need a Node.js script that takes a JSON file of eligible users (e.g., `{"GABC...": 100, "GXYZ...": 500}`) and generates a Merkle Tree. 
You can use `merkletreejs` and `keccak256`. 
The script should output the Merkle Root and a JSON file containing the proofs for every user, which the frontend will consume.

### Issue 10: Create a script to deploy the Airdrop Contract
**Labels**: `wave-eligible`, `backend`, `stellar-cli`
**Description**:
Write a bash or Node script in the `contracts/` directory that uses the `stellar` CLI to build the WASM and deploy it to the Soroban Testnet. It should automatically log the generated Contract ID to a file so the frontend can read it.
