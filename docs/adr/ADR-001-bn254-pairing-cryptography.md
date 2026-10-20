# ADR-001: BN254 Elliptic Curve Pairing Cryptography for Zero-Knowledge Proofs

## Status
Accepted

## Context
Privacy-preserving agentic credentials and zero-knowledge verifiable presentations require bilinear pairing groups over elliptic curves. The BN254 (Alt-bn128) pairing-friendly curve is globally standardized across Ethereum (EIP-196 / EIP-197 precompiles) and modern Web3 privacy protocols.

## Decision
We implement native arithmetic over BN254:
- Scalar and base field elements over $\mathbb{F}_r$ with constant-time modular operations.
- $\mathbb{G}_1$ curve points over $\mathbb{F}_p$ ($y^2 = x^3 + 3$).
- $\mathbb{G}_2$ curve points over $\mathbb{F}_{p^2}$ with Frobenius endomorphisms.
- Bilinear pairing map $e: \mathbb{G}_1 \times \mathbb{G}_2 \to \mathbb{G}_T$ satisfying the bilinear property:
  $$e(aP, bQ) = e(P, Q)^{ab}$$

## Consequences
- Enables sub-millisecond Groth16 verification.
- Direct EVM precompile compatibility for on-chain verification in the RNP Blockchain Observatory.
