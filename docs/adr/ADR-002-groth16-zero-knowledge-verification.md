# ADR-002: Groth16 zk-SNARK Verification and R1CS Constraint Circuit

## Status
Accepted

## Context
When an autonomous agent proves its credentials to an institutional gateway in a scientific data space, revealing raw identity attributes violates privacy principles and exposes institutional IP.

## Decision
We implement the **Groth16 zk-SNARK** verification algorithm combined with a **Rank-1 Constraint System (R1CS)** solver:
- Proof format: $(A \in \mathbb{G}_1, B \in \mathbb{G}_2, C \in \mathbb{G}_1)$ (constant 128 bytes size regardless of circuit complexity).
- Constant-time verification requiring only 3 pairing evaluations:
  $$e(A, B) = e(\alpha, \beta) \cdot e(K, \gamma) \cdot e(C, \delta)$$
- R1CS circuit enforcing threshold satisfiability: an agent proves that their `trust_score \ge min_threshold` and `preimage == commitment` without disclosing the score or identity.

## Consequences
- 100% zero-knowledge proof generation and verification.
- Verification latency $< 2$ milliseconds on standard hardware.
