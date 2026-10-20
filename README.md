# Rust ZK-SNARK Verifier Engine: Groth16 & BN254 Pairing Architecture

[![CI](https://github.com/FelipeMadson/rust-zk-snark-verifier-engine/actions/workflows/ci.yml/badge.svg)](https://github.com/FelipeMadson/rust-zk-snark-verifier-engine/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust 1.80+](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![Groth16](https://img.shields.io/badge/Zero--Knowledge-Groth16%20SNARK-blueviolet.svg)](https://eprint.iacr.org/2016/260)
[![BN254 / Alt-bn128](https://img.shields.io/badge/Curve-BN254%20Pairing-success.svg)](https://eips.ethereum.org/EIPS/eip-197)

> **High-Performance Zero-Knowledge Cryptographic Engine for Privacy-Preserving Scientific Credential Attestation and On-Chain Verification (RNP TRUST-ID & Blockchain Observatory).**

---

## 1. Overview

Zero-Knowledge Succinct Non-Interactive Arguments of Knowledge (zk-SNARKs) allow a prover to mathematically demonstrate possession of a valid secret witness satisfying arbitrary algebraic constraints without revealing the secret itself.

**rust-zk-snark-verifier-engine** delivers a clean, production-grade zero-knowledge proof verification pipeline:
1. **BN254 (Alt-bn128) Bilinear Pairings**: Elliptic curve arithmetic over $\mathbb{G}_1$, $\mathbb{G}_2$, and $\mathbb{G}_T$ with pairing map $e(P, Q)$.
2. **Rank-1 Constraint System (R1CS)**: Deterministic constraint compiler verifying $\langle A, z \rangle \cdot \langle B, z \rangle = \langle C, z \rangle$.
3. **Groth16 SNARK Verifier**: Evaluates 128-byte constant-size proofs in constant time with 3 pairing checks:
   $$e(A, B) = e(\alpha, \beta) \cdot e\left(IC_0 + \sum x_i IC_i, \gamma\right) \cdot e(C, \delta)$$
4. **Privacy-Preserving Credential Circuit**: Proves an AI agent holds valid scientific credentials with sufficient trust score and unexpired quota without disclosing the agent's real identity.

---

## 2. Quickstart

```bash
git clone https://github.com/FelipeMadson/rust-zk-snark-verifier-engine.git
cd rust-zk-snark-verifier-engine
cargo test --verbose
cargo run
```

## License
MIT License (c) 2026 Felipe Madison.
