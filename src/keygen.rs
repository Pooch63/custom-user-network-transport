use rand::prelude::*;
#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
use std::fmt::{ Debug, Display };

use crate::primes;

// The key is signed because we need intermediate negative values
// during multiplicative modular inverse calculations
pub type Key = bnum::types::I4096;

// fn serialize_key(key: &Key) -> [u8; Key::BYTES_USIZE] {
//     *key.to_le_bytes()
// }

/*
    Tested a GCD function like this:
    
fn gcd(mut a: u64, mut b: u64) -> u64 {
    loop {
        if b == 0 { return a; }

        let temp = a;
        a = b;
        b = temp % b;
    }
}

    That is actually tested to be slower than the following implementation:
*/
fn gcd(a: Key, b: Key) -> Key {
    if a == Key::ZERO || b == Key::ZERO { return Key::ZERO; }

    let mut copy_a: Key = a;
    let mut copy_b: Key = b;
    while copy_a != copy_b {
        if copy_a == copy_b { return copy_a; }
        if copy_a > copy_b {
            copy_a = copy_a % copy_b;
            if copy_a == Key::ZERO { return copy_b; };
        }
        else {
            copy_b = copy_b % copy_a;
            if copy_b == Key::ZERO { return copy_a; }
        }
    }
    return copy_a;
}
fn are_coprime(a: Key, b: Key) -> bool {
    // Are they both even? If so, they're not coprime
    if ((a | b) & Key::ONE) == Key::ZERO { return false; }

    return gcd(a, b) == Key::ONE;
}
// Find modular inverse i such that ai = 1 (mod b),
// ASSUMING n and b are coprime
pub fn get_modular_inverse(mut a: Key, mut b: Key) -> Key {
    // Using Extended Euclidean algorithm

    // Express r0 = x0 * a + m0
    // let mut r0: u64 = m;
    // let mut a0: u64 = a;
    // while a0 > 1 {
    //     // r0 = x0 * a0 + m0
    //     let x0: u64 = r0 / a0;
    //     let m0: u64 = r0 - x0 * a0;
    //     println!("{}  = {} * {} + {}", r0, x0, a0, m0);
    //     r0 = a0;
    //     a0 = m0;
    // }
    // 1

    let original_b: Key = b;

    let (mut x, mut u) = (Key::ZERO, Key::ONE);
    // Recursively find such that x0 = m0 * a1 + m1
    while a != Key::ZERO {
        let q = b / a;
        let r = b - a * q;
        println!("{}  = {} * {} + {}", b, a, q, r);
        let m = x - u * q;
        (b, a, x, u) = (a, r, u, m);
    }
    // If X is negative, add the original value of b to it --
    // we can do that since X is the modular inverse mod b
    if x < Key::ZERO { x = x + original_b; }
    return x;
}

// Compute s^e mod m
pub fn bigmod(s: Key, mut e: Key, m: Key) -> Key {
    // We're essentially going to multiple s^n for every (1 << b), accounting
    // for every set bit in e
    let mut final_mod: Key = Key::ONE;
    // The mod of s^(2^n)
    let mut last_mod: Key = s;

    while e > Key::ZERO {
        if (e & Key::ONE) == Key::ONE { final_mod = (final_mod * last_mod) % m; };
        last_mod = (last_mod * last_mod) % m;
        e = e >> Key::ONE;
    }
    return final_mod;
}

// Test one case of the Miller-Rabin for a potential prime p and a base A, given (p - 1)'s mantissa.
// e.g. the number M that satisfies p - 1 = 2^N * M
// Assumes that p is odd
// If any of the following is not true:
//   A^(p - 1) = 1 (mod p)
//   A^((p - 1)/2) = 1 (mod p) OR A^((p - 1)/2) = -1 (mod p)
// Run the last test for {M, M * 2, M * 2^2, M * 2^3, ... prime}
// It's not a prime. Return false if number is not a prime, true if there's a 3/4 chance it is
fn number_passes_miller_rabin(mut mantissa: Key, prime: Key, base: Key) -> bool {
    // Make sure it's odd
    assert!((prime & Key::ONE) == Key::ONE);

    let mut power: Key = bigmod(base, mantissa, prime);
    if power == Key::ONE || power == (prime - Key::ONE) { return true; }
    
    while mantissa < (prime - Key::ONE) {
        power = (power * power) % prime;
        mantissa = mantissa << Key::ONE;
        if power == Key::ONE || power == (prime - Key::ONE) { return true; }
    }

    return false;
}

pub struct NumberHandler {
    key_byte_size: usize,
    rng: ThreadRng
}
impl NumberHandler {
    pub fn new(key_byte_size: usize) -> Self {
        Self { key_byte_size, rng: rand::rng() }
    }
    pub fn get_rng(&mut self) -> &mut ThreadRng {
        return &mut self.rng;
    }
    
    fn get_random_u8(&mut self) -> u8 {
        self.get_rng().random::<u8>()
    }
    // Ensures that the number is at least 1 << bits
    fn get_random_n_byte_key(&mut self, byte_count: usize) -> Key {
        let mut bytes: [u8; Key::BYTES as usize] = [0; Key::BYTES as usize];
        for ind in 0usize..byte_count {
            let mut byte: u8 = self.get_random_u8();
            if ind == byte_count - 1 { byte = byte | 0b10000000; }
            bytes[ind] = byte;
        }
        match Key::from_le_slice(&bytes) {
            Some(key) => key,
            None      => Key::ZERO
        }
    }
    #[inline]
    fn get_random_key(&mut self, ensure_odd: bool) -> Key {
        self.get_random_n_byte_key(self.key_byte_size) | (if ensure_odd { Key::ONE } else { Key::ZERO }) 
    }
    fn get_random_key_range(&mut self, range: std::ops::Range<Key>, ensure_odd: bool) -> Key {
        self.get_random_key(ensure_odd) % (range.end - range.start - Key::ONE) + range.start
    }
    // An even number will correctly fail the test, but it's a good idea to just
    // avoid passing in an even number anyway
    fn miller_rabin_prime_test(&mut self, num: Key, iterations: u8) -> bool {
        // If it's even and not 2, it's not a prime
        if num < Key::FOUR { return num == Key::TWO || num == Key::THREE; }
    
        // Find a 2^e * m = num
        let mut m: Key = num - Key::ONE;
    
        while (m & Key::ONE) == Key::ZERO {
            m = m >> Key::ONE;
        }
    
        for _iter in 0..iterations {
            let base: Key = self.get_random_key_range(Key::TWO..(num - Key::ONE), false);
            if !number_passes_miller_rabin(m, num, base) { return false; }
        }
    
        return true;
    }
    
    pub fn get_random_prime(&mut self, iterations: u8) -> Key {
        
        loop {
            // Make sure key is odd
            let candidate: Key = self.get_random_key(true);

            let mut valid: bool = true;
            // Check if it's divisible by the first few hundred prime factors
            // If it is, then it can't itself be prime
            for prime in 0..primes::FIRST_PRIMES.len() {
                let key: Key = Key::from(prime);
                // Can't be divisible by numbers greater than it
                if candidate * candidate > key { break; }
                if candidate % key == Key::ZERO {
                    valid = false;
                    break;
                }
            }
            if valid && self.miller_rabin_prime_test(candidate, iterations) { return candidate; }
        }
    }
    // Get a random prime different from the given number
    pub fn get_different_random_prime(&mut self, iterations: u8, last_prime: Key) -> Key {
        let mut prime: Key = self.get_random_prime(iterations);
        while prime == last_prime { prime = self.get_random_prime(iterations); }
        return prime;
    }

    // Generate a random number coprime to the given key
    pub fn gen_random_coprime(&mut self, coprime: Key) -> Key {
        loop {
            let prime: Key = self.get_random_key(false);
            if are_coprime(coprime, prime) { return prime; }
        }
    }
    // Generate a random number min < N < max that is coprime with coprimme
    pub fn gen_random_coprime_number_in_range(&mut self, min: Key, max: Key, coprime: Key) -> Key {
        loop {
            let prime: Key = self.get_random_key_range(min..max, false);
            if are_coprime(coprime, prime) { return prime; }
        }
    }
}

#[derive(Clone, Copy)]
pub struct RSAKeyInfo {
    pub public: Key,
    pub private: Key,
    pub shared: Key
}
fn format_keys(keys: &RSAKeyInfo, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "( public: {}, private: {}, shared: {} )", keys.public, keys.private, keys.shared)
}
#[cfg(feature = "std")]
impl Debug for RSAKeyInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_keys(self, f)
    }
}
#[cfg(feature = "std")]
impl Display for RSAKeyInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_keys(self, f)
    }
}