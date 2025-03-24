use ark_ed25519::{EdwardsProjective as ED25519Point, Fr as ED25519Fr, Fq as ED25519Fq};
use ark_ec::{CurveGroup, PrimeGroup};
use ark_ff::{BigInteger, PrimeField};
use ark_std::UniformRand;
use num_bigint::BigUint;
use rand::rngs::StdRng;
use rand::SeedableRng;

// Simple struct to represent an ED25519 point
#[derive(Debug, Clone)]
pub struct Point {
    pub x: BigUint,
    pub y: BigUint,
}

impl Point {
    pub fn infinity() -> Self {
        Self {
            x: BigUint::from(0u32),
            y: BigUint::from(0u32),
        }
    }
    
    pub fn gen_random_point(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        
        let g = ED25519Point::rand(&mut rng).into_affine();
        let x = BigUint::from_bytes_le(&g.x.into_bigint().to_bytes_le());
        let y = BigUint::from_bytes_le(&g.y.into_bigint().to_bytes_le());
        
        Self { x, y }
    }
    
    pub fn get_generator() -> Self {
        let g = ED25519Point::generator().into_affine();
        let x = BigUint::from_bytes_le(&g.x.into_bigint().to_bytes_le());
        let y = BigUint::from_bytes_le(&g.y.into_bigint().to_bytes_le());
        
        Self { x, y }
    }
    
    pub fn scalar_mul(scalar: u64) -> Self {
        let g_base = ED25519Point::generator();
        let scalar_field = ED25519Fr::from(scalar);
        let result = g_base * scalar_field;
        
        let g_affine = result.into_affine();
        let x = BigUint::from_bytes_le(&g_affine.x.into_bigint().to_bytes_le());
        let y = BigUint::from_bytes_le(&g_affine.y.into_bigint().to_bytes_le());
        
        Self { x, y }
    }
}

// Constants for ED25519
pub struct ED25519Constants {
    pub curve_id: usize,
    pub field_modulus: BigUint,
    pub curve_order: BigUint,
}

impl ED25519Constants {
    pub fn new() -> Self {
        let field_modulus = ED25519Fq::MODULUS;
        let curve_order = ED25519Fr::MODULUS;
        
        Self {
            curve_id: 3, // Value used in the Cairo code for ED25519
            field_modulus: BigUint::from_bytes_le(&field_modulus.to_bytes_le()),
            curve_order: BigUint::from_bytes_le(&curve_order.to_bytes_le()),
        }
    }
} 