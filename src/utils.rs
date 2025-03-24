use crate::curves::Point;
use ark_ed25519::{Fr as ED25519Fr, Fq as ED25519Fq};
use ark_ff::{BigInteger, PrimeField};
use ark_std::UniformRand;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::SeedableRng;

// Generate a random field element
pub fn random_field_element(seed: u64) -> BigUint {
    let mut rng = StdRng::seed_from_u64(seed);
    let f = ED25519Fq::rand(&mut rng);
    BigUint::from_bytes_le(&f.into_bigint().to_bytes_le())
}

// Generate a random scalar (element in Fr)
pub fn random_scalar(seed: u64) -> BigUint {
    let mut rng = StdRng::seed_from_u64(seed);
    let f = ED25519Fr::rand(&mut rng);
    BigUint::from_bytes_le(&f.into_bigint().to_bytes_le())
}

// Split 128-bit integers into 64-bit limbs
pub fn split_128(n: &BigUint) -> Vec<u64> {
    let bytes = n.to_bytes_le();
    let mut result = vec![0u64, 0u64];
    
    // Extract low 128 bits (16 bytes)
    let mut low_bytes = [0u8; 16];
    for (i, &byte) in bytes.iter().take(16).enumerate() {
        low_bytes[i] = byte;
    }
    
    // Convert to u64 values
    result[0] = u64::from_le_bytes([
        low_bytes[0], low_bytes[1], low_bytes[2], low_bytes[3],
        low_bytes[4], low_bytes[5], low_bytes[6], low_bytes[7],
    ]);
    
    result[1] = u64::from_le_bytes([
        low_bytes[8], low_bytes[9], low_bytes[10], low_bytes[11],
        low_bytes[12], low_bytes[13], low_bytes[14], low_bytes[15],
    ]);
    
    result
}

// Serialize a point for Cairo code
pub fn serialize_point(point: &Point) -> String {
    format!("G1Point {{ x: {}u.into(), y: {}u.into() }}", point.x, point.y)
} 