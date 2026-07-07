# Soroban Airdrop Claim Template 🚀

Welcome to the **Soroban Airdrop Claim Template**! This open-source boilerplate provides everything a Stellar ecosystem project needs to launch a token airdrop using Soroban smart contracts and a modern Next.js frontend.

## 🌟 Features
- **Rust/Soroban Smart Contract**: A highly-optimized smart contract that verifies Merkle proofs to allow users to securely claim their tokens.
- **Next.js Claim UI**: A beautiful, responsive frontend built with Next.js, Tailwind CSS, and TypeScript.
- **Freighter Wallet Integration**: Seamlessly connects with Freighter to sign claim transactions on the Stellar network.
- **Merkle Tree Generator**: Node.js scripts to generate a Merkle Root from a list of eligible public keys and balances.

## 🛠️ Tech Stack
- **Frontend**: Next.js (React), Tailwind CSS, TypeScript
- **Smart Contracts**: Rust, Soroban SDK
- **Wallet**: Freighter API, Stellar SDK

## 🚀 Getting Started

### Prerequisites
- Node.js (v18+)
- Rust & Cargo
- Stellar CLI

### Running the Frontend
```bash
cd frontend
npm install
npm run dev
```

### Compiling the Contracts
```bash
cd contracts
cargo build --target wasm32-unknown-unknown --release
```

## 🤝 Contributing
We love community contributions! This project is participating in the **Stellar Drips Wave**. If you want to contribute and earn rewards, please check out our [CONTRIBUTING.md](./CONTRIBUTING.md) for details on how to get started.

## 📜 License
MIT License
