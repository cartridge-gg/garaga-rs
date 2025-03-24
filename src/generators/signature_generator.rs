use crate::curves::{Point, ED25519Constants};
use crate::utils::random_scalar;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::SeedableRng;

// ECDSA signature for ED25519
pub struct ECDSASignature {
    r: BigUint,
    s: BigUint,
    v: u8,
    px: BigUint,
    py: BigUint,
    z: BigUint,  // message hash
}

impl ECDSASignature {
    pub fn sample(seed: u64) -> Self {
        let mut _rng = StdRng::seed_from_u64(seed);
        
        // Generate public key
        let public_key = Point::gen_random_point(seed + 1000);
        
        // Generate random values for the signature
        // In a real implementation, these would be properly computed
        let r = random_scalar(seed + 2000);
        let s = random_scalar(seed + 3000);
        let z = random_scalar(seed + 4000);  // message hash
        
        // Recovery parameter v is 0 or 1
        let v = (seed % 2) as u8;
        
        ECDSASignature {
            r,
            s,
            v,
            px: public_key.x,
            py: public_key.y,
            z,
        }
    }
    
    pub fn serialize_with_hints(&self) -> String {
        // Simple serialization for the Cairo test
        format!(
            "{{ r: {}u.into(), s: {}u.into(), v: {}, px: {}u.into(), py: {}u.into(), z: {}u.into() }}",
            self.r, self.s, self.v, self.px, self.py, self.z
        )
    }
}

pub fn generate_ecdsa_test(seed: u64) -> String {
    // Get curve constants
    let constants = ED25519Constants::new();
    
    // Generate an ECDSA signature
    let ecdsa_sig = ECDSASignature::sample(seed);
    
    let mut code = String::new();
    
    // Add module header with imports
    code.push_str("#[cfg(test)]\nmod ecdsa_tests {\n");
    code.push_str("    use garaga::ecdsa::*;\n");
    code.push_str("    use garaga::ec_ops::G1PointImpl;\n\n");
    
    // Test function header
    code.push_str("    #[test]\n    fn test_ecdsa_ED25519() {\n");
    
    // Serialize the signature
    code.push_str(&format!("        let mut ecdsa_sig_with_hints_serialized = array!{}.span();\n", 
                         ecdsa_sig.serialize_with_hints()));
    
    // Deserialize and verify
    code.push_str("        let ecdsa_with_hints = Serde::<ECDSASignatureWithHint>::deserialize(ref ecdsa_sig_with_hints_serialized).expect('FailToDeserialize');\n");
    code.push_str(&format!("        let is_valid = is_valid_ecdsa_signature(ecdsa_with_hints, {});\n", constants.curve_id));
    code.push_str("        assert!(is_valid);\n");
    
    // Close test function and module
    code.push_str("    }\n}\n");
    
    code
} 