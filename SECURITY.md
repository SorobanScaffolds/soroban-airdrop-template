# Security Policy

## Reporting Security Issues

We take the security of our Soroban smart contracts and templates seriously. If you discover a potential security vulnerability, please report it responsibly.

### Responsible Disclosure

- **Do NOT** open a public GitHub issue for security vulnerabilities.
- Please email the maintainers directly at `security@sorobanscaffolds.org` (or contact via GitHub Security Advisories).
- Provide a detailed description of the vulnerability, steps to reproduce, and any proof-of-concept code.
- We will acknowledge receipt of your report within 48 hours and work with you to coordinate a responsible disclosure and patch.

### Scope

- Soroban smart contract logic (`contracts/src/lib.rs`): Merkle tree verification, double-claim guards, token balance transfers, admin permissions.
- Off-chain cryptographic generators (`scripts/merkle.js`): Leaf hashing parity, proof generation correctness.
- Web frontend (`frontend/`): Wallet signature verification, XDR sanitization.
