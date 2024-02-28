use num_bigint::{BigInt, RandBigInt};
use num_traits::One;
use rand::thread_rng;
use tiny_keccak::{Keccak, Hasher};

mod utils;

use utils::Point;
use utils::scalar_multiplication;

// secp256k1 curve parameters
const P: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEFFFFFC2F";
const N: &str = "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141";
const G_X: &str = "79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798";
const G_Y: &str = "483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8";

fn main() {
    let private_key = generate_private_key();
    println!("Private Key: {}", private_key);

    let public_key = derive_public_key(&private_key);
    println!("Public Key: ({}, {})", public_key.0, public_key.1);

    let ethereum_address = convert_to_ethereum_address(&public_key);
    println!("Ethereum Address: {}", ethereum_address);
}

fn generate_private_key() -> BigInt {
    let n = BigInt::parse_bytes(N.as_bytes(), 16).unwrap();
    let mut rng = thread_rng();
    // Correctly generate a random BigInt within the range [1, n-1]
    let private_key = rng.gen_bigint_range(&BigInt::one(), &n);
    private_key
}

fn derive_public_key(private_key: &BigInt) -> (BigInt, BigInt) {
    let modulus = BigInt::parse_bytes(P.as_bytes(), 16).unwrap();
    let a = BigInt::from(0); // secp256k1's 'a' parameter is 0
    let g = Point {
        x: BigInt::parse_bytes(G_X.as_bytes(), 16).unwrap(),
        y: BigInt::parse_bytes(G_Y.as_bytes(), 16).unwrap(),
    };
    let public_key = scalar_multiplication(&g, private_key, &a, &modulus);
    (public_key.x, public_key.y)
}

fn convert_to_ethereum_address(public_key: &(BigInt, BigInt)) -> String {
    let public_key_bytes = [
        &public_key.0.to_bytes_be().1[..],
        &public_key.1.to_bytes_be().1[..],
    ]
    .concat();

    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32]; // Keccak-256 outputs 32 bytes

    hasher.update(&public_key_bytes);
    hasher.finalize(&mut output);

    let address_bytes = &output[12..]; // Skip the first 12 bytes, take the last 20
    format!("0x{}", hex::encode(address_bytes))
}

