//! Groth16 Zero-Knowledge SNARK Proof Verification Engine.

use crate::pairing::{pairing, FieldElement, G1Point, G2Point, GTElement};

/// Groth16 zk-SNARK Proof tuple (A, B, C).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Groth16Proof {
    pub a: G1Point,
    pub b: G2Point,
    pub c: G1Point,
}

/// Groth16 Verifying Key generated during the trusted setup.
#[derive(Clone, Debug)]
pub struct VerifyingKey {
    pub alpha_g1: G1Point,
    pub beta_g2: G2Point,
    pub gamma_g2: G2Point,
    pub delta_g2: G2Point,
    /// IC elements for public inputs [IC_0, IC_1, ..., IC_ell]
    pub ic: Vec<G1Point>,
}

/// Verifies a Groth16 zero-knowledge proof against public inputs.
///
/// Verifier Equation:
/// e(A, B) = e(alpha, beta) * e(IC_0 + sum(x_i * IC_i), gamma) * e(C, delta)
pub fn verify_groth16_proof(
    vk: &VerifyingKey,
    proof: &Groth16Proof,
    public_inputs: &[FieldElement],
) -> Result<bool, &'static str> {
    if public_inputs.len() + 1 != vk.ic.len() {
        return Err("mismatched public inputs count against verifying key IC elements");
    }

    // 1. Compute LHS: e(A, B)
    let lhs = pairing(&proof.a, &proof.b);

    // 2. Compute Public Input linear combination: K = IC_0 + sum_{i=1}^ell (x_i * IC_i)
    let mut k_acc = vk.ic[0];
    for (i, x_i) in public_inputs.iter().enumerate() {
        let scaled = vk.ic[i + 1].scalar_mul(x_i);
        k_acc = k_acc.add_point(&scaled);
    }

    // 3. Compute RHS: e(alpha, beta) * e(K, gamma) * e(C, delta)
    let term1 = pairing(&vk.alpha_g1, &vk.beta_g2);
    let term2 = pairing(&k_acc, &vk.gamma_g2);
    let term3 = pairing(&proof.c, &vk.delta_g2);

    let rhs = GTElement(term1.0.mul(&term2.0).mul(&term3.0));

    // Equality check over target group GT
    Ok(lhs == rhs)
}
