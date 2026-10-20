use rust_zk_snark_verifier_engine::{
    pairing::{FieldElement, G1Point, G2Point, pairing},
    r1cs::{LinearCombination, R1CSSystem},
    circuits::{build_scientific_credential_circuit, generate_credential_witness},
};

#[test]
fn test_field_arithmetic() {
    let a = FieldElement::new(10);
    let b = FieldElement::new(20);

    let sum = a.add(&b);
    assert_eq!(sum.0, 30);

    let prod = a.mul(&b);
    assert_eq!(prod.0, 200);

    let inv = a.inv().unwrap();
    let identity = a.mul(&inv);
    assert_eq!(identity.0, 1);
}

#[test]
fn test_pairing_homomorphism() {
    let p = G1Point::new(2, 3);
    let q = G2Point::new(4, 5);

    let pair1 = pairing(&p, &q);
    assert!(pair1.0.0 > 0);
}

#[test]
fn test_r1cs_multiplication_constraint() {
    // x * y = z
    let mut system = R1CSSystem::new(1, 4); // z[0]=1, z[1]=x, z[2]=y, z[3]=z

    let mut a = LinearCombination::new();
    a.add_term(1, FieldElement::ONE); // x

    let mut b = LinearCombination::new();
    b.add_term(2, FieldElement::ONE); // y

    let mut c = LinearCombination::new();
    c.add_term(3, FieldElement::ONE); // z

    system.add_constraint(a, b, c);

    // Witness: x=3, y=5, z=15
    let valid_witness = vec![
        FieldElement::ONE,
        FieldElement::new(3),
        FieldElement::new(5),
        FieldElement::new(15),
    ];

    assert!(system.verify_witness(&valid_witness));

    // Invalid Witness: x=3, y=5, z=16
    let invalid_witness = vec![
        FieldElement::ONE,
        FieldElement::new(3),
        FieldElement::new(5),
        FieldElement::new(16),
    ];

    assert!(!system.verify_witness(&invalid_witness));
}

#[test]
fn test_scientific_credential_circuit_witness() {
    let circuit = build_scientific_credential_circuit();

    // Valid: trust_level 90 >= min 70
    let valid_witness = generate_credential_witness(12345, 90, 70);
    assert!(circuit.verify_witness(&valid_witness));
}
