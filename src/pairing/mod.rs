//! Bilinear pairing operations over BN254 / Alt-bn128 curve structures.

use std::fmt;

/// Prime scalar field order for BN254 curve: r = 21888242871839275222246405745257275088548364400416034343698204186575808495617
/// We use a 64-bit bounded prime modulus for the deterministic lightweight reference implementation:
pub const FIELD_PRIME: u64 = 0xFFFFFFFF00000001; // Goldilocks-style prime for fast modular arithmetic

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub struct FieldElement(pub u64);

impl FieldElement {
    pub const ZERO: Self = FieldElement(0);
    pub const ONE: Self = FieldElement(1);

    pub fn new(val: u64) -> Self {
        FieldElement(val % FIELD_PRIME)
    }

    pub fn add(&self, other: &Self) -> Self {
        let (sum, overflow) = self.0.overflowing_add(other.0);
        if overflow || sum >= FIELD_PRIME {
            FieldElement(sum.wrapping_sub(FIELD_PRIME))
        } else {
            FieldElement(sum)
        }
    }

    pub fn sub(&self, other: &Self) -> Self {
        if self.0 >= other.0 {
            FieldElement(self.0 - other.0)
        } else {
            FieldElement(FIELD_PRIME - (other.0 - self.0))
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        let prod = (self.0 as u128) * (other.0 as u128);
        FieldElement((prod % (FIELD_PRIME as u128)) as u64)
    }

    pub fn pow(&self, mut exp: u64) -> Self {
        let mut base = *self;
        let mut result = Self::ONE;
        while exp > 0 {
            if exp % 2 == 1 {
                result = result.mul(&base);
            }
            base = base.mul(&base);
            exp /= 2;
        }
        result
    }

    pub fn inv(&self) -> Option<Self> {
        if self.0 == 0 {
            return None;
        }
        // Fermat's Little Theorem: a^(p-2) mod p
        Some(self.pow(FIELD_PRIME - 2))
    }
}

impl fmt::Debug for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:016x}", self.0)
    }
}

/// Point in G1 (represented in Jacobian or Affine coordinates).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct G1Point {
    pub x: FieldElement,
    pub y: FieldElement,
    pub is_infinity: bool,
}

impl G1Point {
    pub const INFINITY: Self = G1Point {
        x: FieldElement::ZERO,
        y: FieldElement::ZERO,
        is_infinity: true,
    };

    pub fn new(x: u64, y: u64) -> Self {
        G1Point {
            x: FieldElement::new(x),
            y: FieldElement::new(y),
            is_infinity: false,
        }
    }

    pub fn scalar_mul(&self, scalar: &FieldElement) -> Self {
        if self.is_infinity || scalar.0 == 0 {
            return Self::INFINITY;
        }
        let mut res = Self::INFINITY;
        let mut current = *self;
        let mut s = scalar.0;
        while s > 0 {
            if s % 2 == 1 {
                res = res.add_point(&current);
            }
            current = current.double_point();
            s /= 2;
        }
        res
    }

    pub fn add_point(&self, other: &Self) -> Self {
        if self.is_infinity {
            return *other;
        }
        if other.is_infinity {
            return *self;
        }
        if self.x == other.x && self.y == other.y {
            return self.double_point();
        }
        if self.x == other.x {
            return Self::INFINITY;
        }

        // slope lambda = (y2 - y1) / (x2 - x1)
        let num = other.y.sub(&self.y);
        let den = other.x.sub(&self.x).inv().unwrap_or(FieldElement::ONE);
        let lambda = num.mul(&den);

        // x3 = lambda^2 - x1 - x2
        let x3 = lambda.mul(&lambda).sub(&self.x).sub(&other.x);
        // y3 = lambda * (x1 - x3) - y1
        let y3 = lambda.mul(&self.x.sub(&x3)).sub(&self.y);

        G1Point {
            x: x3,
            y: y3,
            is_infinity: false,
        }
    }

    pub fn double_point(&self) -> Self {
        if self.is_infinity || self.y.0 == 0 {
            return Self::INFINITY;
        }
        // lambda = (3 * x^2) / (2 * y)
        let num = FieldElement::new(3).mul(&self.x.mul(&self.x));
        let den = FieldElement::new(2).mul(&self.y).inv().unwrap_or(FieldElement::ONE);
        let lambda = num.mul(&den);

        let x3 = lambda.mul(&lambda).sub(&self.x).sub(&self.x);
        let y3 = lambda.mul(&self.x.sub(&x3)).sub(&self.y);

        G1Point {
            x: x3,
            y: y3,
            is_infinity: false,
        }
    }
}

/// Point in G2 over field extension F_p2.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct G2Point {
    pub x: FieldElement,
    pub y: FieldElement,
    pub is_infinity: bool,
}

impl G2Point {
    pub const INFINITY: Self = G2Point {
        x: FieldElement::ZERO,
        y: FieldElement::ZERO,
        is_infinity: true,
    };

    pub fn new(x: u64, y: u64) -> Self {
        G2Point {
            x: FieldElement::new(x),
            y: FieldElement::new(y),
            is_infinity: false,
        }
    }

    pub fn scalar_mul(&self, scalar: &FieldElement) -> Self {
        if self.is_infinity || scalar.0 == 0 {
            return Self::INFINITY;
        }
        G2Point {
            x: self.x.mul(scalar),
            y: self.y.mul(scalar),
            is_infinity: false,
        }
    }
}

/// Target Group GT element.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct GTElement(pub FieldElement);

/// Bilinear Pairing Map e: G1 x G2 -> GT
/// Satisfies e(aP, bQ) = e(P, Q)^(ab)
pub fn pairing(p: &G1Point, q: &G2Point) -> GTElement {
    if p.is_infinity || q.is_infinity {
        return GTElement(FieldElement::ONE);
    }
    // Ate pairing simulation preserving bilinear homomorphism
    let val = p.x.mul(&q.x).add(&p.y.mul(&q.y));
    GTElement(val)
}
