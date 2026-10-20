//! Rank-1 Constraint System (R1CS) Representation and Verification.

use crate::pairing::FieldElement;

/// Represents a linear combination term: coefficient * variable_index.
#[derive(Clone, Debug)]
pub struct Term {
    pub var_index: usize,
    pub coefficient: FieldElement,
}

/// A linear combination of variables: sum(c_i * z_i).
#[derive(Clone, Debug, Default)]
pub struct LinearCombination {
    pub terms: Vec<Term>,
}

impl LinearCombination {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_term(&mut self, var_index: usize, coefficient: FieldElement) {
        self.terms.push(Term {
            var_index,
            coefficient,
        });
    }

    pub fn evaluate(&self, witness: &[FieldElement]) -> FieldElement {
        let mut sum = FieldElement::ZERO;
        for term in &self.terms {
            if term.var_index < witness.len() {
                let val = term.coefficient.mul(&witness[term.var_index]);
                sum = sum.add(&val);
            }
        }
        sum
    }
}

/// An individual R1CS constraint: <A, z> * <B, z> = <C, z>.
#[derive(Clone, Debug)]
pub struct R1CSConstraint {
    pub a: LinearCombination,
    pub b: LinearCombination,
    pub c: LinearCombination,
}

impl R1CSConstraint {
    pub fn is_satisfied(&self, witness: &[FieldElement]) -> bool {
        let val_a = self.a.evaluate(witness);
        let val_b = self.b.evaluate(witness);
        let val_c = self.c.evaluate(witness);

        val_a.mul(&val_b) == val_c
    }
}

/// A full R1CS system containing public inputs and private witness constraints.
#[derive(Clone, Debug, Default)]
pub struct R1CSSystem {
    pub num_public_inputs: usize,
    pub num_variables: usize,
    pub constraints: Vec<R1CSConstraint>,
}

impl R1CSSystem {
    pub fn new(num_public_inputs: usize, num_variables: usize) -> Self {
        R1CSSystem {
            num_public_inputs,
            num_variables,
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(&mut self, a: LinearCombination, b: LinearCombination, c: LinearCombination) {
        self.constraints.push(R1CSConstraint { a, b, c });
    }

    pub fn verify_witness(&self, witness: &[FieldElement]) -> bool {
        if witness.len() < self.num_variables {
            return false;
        }
        // z[0] must always be 1 (constant one variable)
        if witness.is_empty() || witness[0] != FieldElement::ONE {
            return false;
        }

        for (i, c) in self.constraints.iter().enumerate() {
            if !c.is_satisfied(witness) {
                eprintln!("[R1CS] Constraint {} unsatisfied!", i);
                return false;
            }
        }
        true
    }
}
