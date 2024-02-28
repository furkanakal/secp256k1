use num_bigint::BigInt;
use num_traits::{One, Zero};

#[derive(Clone, Debug)]
pub struct Point {
    pub x: BigInt,
    pub y: BigInt,
}

pub fn add_points(p1: &Point, p2: &Point, a: &BigInt, modulus: &BigInt) -> Point {
    if p1.x == p2.x && p1.y == p2.y {
        return double_point(p1, a, modulus);
    }
    
    let p2_x_minus_p1_x = (&p2.x - &p1.x) % modulus;
    let p2_y_minus_p1_y = (&p2.y - &p1.y) % modulus;

    let inv = mod_inverse(&p2_x_minus_p1_x, modulus).unwrap_or_default();

    let m = (p2_y_minus_p1_y * inv) % modulus;
    let x3 = (m.pow(2) - &p1.x - &p2.x) % modulus;
    let y3 = (m * (&p1.x - &x3) - &p1.y) % modulus;

    Point {
        x: (x3 + modulus) % modulus,
        y: (y3 + modulus) % modulus,
    }
}

pub fn double_point(point: &Point, a: &BigInt, modulus: &BigInt) -> Point {
    let two = BigInt::from(2);
    let three = BigInt::from(3);

    let inv = mod_inverse(&(&point.y * &two), modulus).unwrap_or_default();

    let m = ((&point.x.modpow(&two, modulus) * &three + a) * &inv) % modulus;

    let x3 = (m.pow(2) - &point.x * 2) % modulus;
    let y3 = (m * (&point.x - &x3) - &point.y) % modulus;

    Point {
        x: (x3 + modulus) % modulus,
        y: (y3 + modulus) % modulus,
    }
}

pub fn scalar_multiplication(point: &Point, scalar: &BigInt, a: &BigInt, modulus: &BigInt) -> Point {
    let mut result = Point { x: BigInt::zero(), y: BigInt::zero() }; // Assume this represents the point at infinity
    let mut temp_point = point.clone();
    let mut current_scalar = scalar.clone();

    while current_scalar > BigInt::zero() {
        if &current_scalar & BigInt::one() != BigInt::zero() {
            result = add_points(&result, &temp_point, a, modulus);
        }
        temp_point = double_point(&temp_point, a, modulus);
        current_scalar >>= 1;
    }

    result
}

pub fn mod_inverse(value: &BigInt, modulus: &BigInt) -> Option<BigInt> {
    let (mut old_r, mut r) = (value.clone(), modulus.clone());
    let (mut old_s, mut s) = (BigInt::one(), BigInt::zero());

    while r != BigInt::zero() {
        let quotient = &old_r / &r;
        old_r = old_r - &quotient * &r;
        std::mem::swap(&mut old_r, &mut r);

        old_s = old_s - quotient * &s;
        std::mem::swap(&mut old_s, &mut s);
    }

    if old_r > BigInt::one() {
        return None;
    }

    while old_s < BigInt::zero() {
        old_s = old_s + modulus;
    }

    Some(old_s)
}