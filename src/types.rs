use cainome_cairo_serde_derive::CairoSerde;
use serde::Serialize;

/// Represents a u384 integer from Cairo, with 4 limbs
#[derive(Debug, Clone, PartialEq, CairoSerde, Serialize)]
pub struct U384 {
    pub limb0: u64,
    pub limb1: u64,
    pub limb2: u64,
    pub limb3: u64,
}

impl U384 {
    pub fn is_zero(&self) -> bool {
        self.limb0 == 0 && self.limb1 == 0 && self.limb2 == 0 && self.limb3 == 0
    }
}

/// Represents a u256 integer from Cairo
#[derive(Debug, Clone, PartialEq, CairoSerde, Serialize)]
pub struct U256 {
    pub low: u128,
    pub high: u128,
}

/// Represents a point on an elliptic curve (G1)
#[derive(Debug, Clone, PartialEq, CairoSerde, Serialize)]
pub struct G1Point {
    pub x: U384,
    pub y: U384,
}

/// Data for deriving a point from its x-coordinate
#[derive(Debug, Clone, PartialEq, CairoSerde, Serialize)]
pub struct DerivePointFromXHint {
    pub y: U384,
}

/// Data for multi-scalar multiplication hints
#[derive(Debug, Clone, PartialEq, CairoSerde, Serialize)]
pub struct MSMHint {
    pub result: G1Point,
}

/// An ECDSA signature with associated public key and message hash
#[derive(Debug, Clone, PartialEq, CairoSerde, Serialize)]
pub struct ECDSASignature {
    pub rx: U384,
    pub s: U256,
    pub v: bool,
    pub px: U384,
    pub py: U384,
    pub z: U256,
}

/// An ECDSA signature bundled with computation hints required for Cairo verification
#[derive(Debug, Clone, PartialEq, CairoSerde, Serialize)]
pub struct ECDSASignatureWithHint {
    pub signature: ECDSASignature,
    pub msm_hint: MSMHint,
    pub msm_derive_hint: DerivePointFromXHint,
}

#[cfg(test)]
mod tests {
    use super::*;
    use cainome_cairo_serde::Serde;

    #[test]
    fn test_ecdsa_signature_with_hint_serialization() {
        // Create a sample signature with hint
        let signature = ECDSASignature {
            rx: U384 { limb0: 1, limb1: 2, limb2: 3, limb3: 4 },
            s: U256 { low: 5, high: 6 },
            v: true,
            px: U384 { limb0: 7, limb1: 8, limb2: 9, limb3: 10 },
            py: U384 { limb0: 11, limb1: 12, limb2: 13, limb3: 14 },
            z: U256 { low: 15, high: 16 },
        };

        let msm_hint = MSMHint {
            result: G1Point {
                x: U384 { limb0: 17, limb1: 18, limb2: 19, limb3: 20 },
                y: U384 { limb0: 21, limb1: 22, limb2: 23, limb3: 24 },
            }
        };

        let msm_derive_hint = DerivePointFromXHint {
            y: U384 { limb0: 25, limb1: 26, limb2: 27, limb3: 28 },
        };

        let sig_with_hint = ECDSASignatureWithHint {
            signature,
            msm_hint,
            msm_derive_hint,
        };

        // Serialize the struct to felt252 array
        let mut serialized = Vec::new();
        sig_with_hint.serialize(&mut serialized);

        // The vector should not be empty
        assert!(!serialized.is_empty());

        // Deserialize back
        let deserialized = ECDSASignatureWithHint::deserialize(&mut serialized.as_slice());
        
        // Check that we got back the same data
        assert!(deserialized.is_some());
        let deserialized_sig = deserialized.unwrap();
        assert_eq!(sig_with_hint, deserialized_sig);
    }
}
