use rand::prelude::*;
use std::fmt;
use std::fmt::{ Debug, Display };

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
fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 { return 0; }

    let mut copy_a: u64 = a;
    let mut copy_b: u64 = b;
    while copy_a != copy_b {
        if copy_a == copy_b { return copy_a; }
        if copy_a > copy_b {
            copy_a = copy_a % copy_b;
            if copy_a == 0 { return copy_b; };
        }
        else {
            copy_b = copy_b % copy_a;
            if copy_b == 0 { return copy_a; }
        }
    }
    return copy_a;
}
fn are_coprime(a: u64, b: u64) -> bool {
    // Are they both even? If so, they're not coprime
    if ((a | b) & 1) == 0 { return false; }

    return gcd(a, b) == 1;
}
// Find modular inverse b such that ab = 1 (mod m),
// ASSUMING a and m are coprime
fn get_modular_inverse(a: u64, m: u64) -> u64 {
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

    let mut b: i64 = a.try_into().unwrap();
    let mut c: i64 = m.try_into().unwrap();

    let (mut x, mut u, mut v) = (0i64, 1i64,0i64);
    while b != 0 {
        let q = c / b;
        let r = c - b * q;
        println!("{}  = {} * {} + {}", c, b, q, r);
        let m = x-u*q;
        (c,b, x, u) = (b,r, u, m);
    }
    // If X is negative, add m to it -- we can do that since X is the modular inverse
    if x < 0 { x = x + (m as i64); }
    return x.try_into().unwrap();
}

// Compute s^e mod m
fn bigmod(s: u64, mut e: u64, m: u64) -> u64 {
    // We're essentially going to multiple s^n for every (1 << b), accounting
    // for every set bit in e
    let mut final_mod: u64 = 1;
    // The mod of s^(2^n)
    let mut last_mod: u64 = s;

    while e > 0 {
        if (e & 1) == 1 { final_mod = (final_mod * last_mod) % m; };
        last_mod = (last_mod * last_mod) % m;
        e = e >> 1;
    }
    return final_mod % m;
}

// Test one case of the Miller-Rabin for a potential prime p and a base A, given (p - 1)'s mantissa.
// e.g. the number M that satisfies p - 1 = 2^N * M
// Assumes that p is odd
// If any of the following is not true:
//   A^(p - 1) = 1 (mod p)
//   A^((p - 1)/2) = 1 (mod p) OR A^((p - 1)/2) = -1 (mod p)
// Run the last test for {M, M * 2, M * 2^2, M * 2^3, ... prime}
// It's not a prime. Return false if number is not a prime, true if there's a 3/4 chance it is
fn number_passes_miller_rabin(mut mantissa: u64, prime: u64, base: u64) -> bool {
    assert!((prime & 1) == 1);

    let mut power: u64 = bigmod(base, mantissa, prime);
    if power == 1 || power == (prime - 1) { return true; }
    
    while mantissa < prime - 1 {
        power = (power * power) % prime;
        mantissa = mantissa << 1;
        if power == 1 || power == (prime - 1) { return true; }
    }

    return false;
}

static FIRST_PRIMES: [u64; 200] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37,
    41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
    89, 97, 101, 103, 107, 109, 113, 127, 131, 137,
    139, 149, 151, 157, 163, 167, 173, 179, 181, 191,
    193, 197, 199, 211, 223, 227, 229, 233, 239, 241,
    251, 257, 263, 269, 271, 277, 281, 283, 293, 307,
    311, 313, 317, 331, 337, 347, 349, 353, 359, 367,
    373, 379, 383, 389, 397, 401, 409, 419, 421, 431,
    433, 439, 443, 449, 457, 461, 463, 467, 479, 487,
    491, 499, 503, 509, 521, 523, 541, 547, 557, 563,
    569, 571, 577, 587, 593, 599, 601, 607, 613, 617,
    619, 631, 641, 643, 647, 653, 659, 661, 673, 677,
    683, 691, 701, 709, 719, 727, 733, 739, 743, 751,
    757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
    827, 829, 839, 853, 857, 859, 863, 877, 881, 883,
    887, 907, 911, 919, 929, 937, 941, 947, 953, 967,
    971, 977, 983, 991, 997, 1009, 1013, 1019, 1021, 1031,
    1033, 1039, 1049, 1051, 1061, 1063, 1069, 1087, 1091,
    1093, 1097, 1103, 1109, 1117, 1123, 1129, 1151, 1153,
    1163, 1171, 1181, 1187, 1193, 1201, 1213, 1217, 1223 
];
struct NumberHandler {
    rng: ThreadRng
}
impl NumberHandler {
    fn new() -> Self {
        Self { rng: rand::rng() }
    }
    fn get_rng(&mut self) -> &mut ThreadRng {
        return &mut self.rng;
    }
    fn miller_rabin_prime_test(&mut self, num: u64, iterations: u8) -> bool {
        // If it's even and not 2, it's not a prime
        if num < 4 { return num == 2 || num == 3; }
        if (num & 1) == 0 { return false; }
    
        // Find a 2^e * m = num
        let mut e: u8 = 0;
        let mut m: u64 = num - 1;
    
        while (m & 1) == 0 {
            e = e + 1;
            m = m >> 1;
        }
    
        for _iter in 0..iterations {
            let base: u64 = self.get_rng().random_range(2u64..(num - 1));
            if !number_passes_miller_rabin(m, num, base) { return false; }
        }
    
        return true;
    }
    fn get_random_u64(&mut self) -> u64 {
        return self.get_rng().random::<u64>();
    }
    fn get_random_prime(&mut self, iterations: u8) -> u64 {
        loop {
            let candidate: u64 = self.get_rng().random::<u16>() as u64;
            let mut valid: bool = true;
            // Check if it's divisible by the first few hundred prime factors
            // If it is, then it can't itself be prime
            for prime in 0..FIRST_PRIMES.len() {
                // Can't be divisible by numbers greater than it
                if candidate * candidate > FIRST_PRIMES[prime] { break; }
                if candidate % FIRST_PRIMES[prime] == 0 {
                    valid = false;
                    break;
                }
            }
            if valid && self.miller_rabin_prime_test(candidate, iterations) { return candidate; }
        }
    }
    // Get a random prime different from the given number
    fn get_different_random_prime(&mut self, iterations: u8, last_prime: u64) -> u64 {
        let mut prime: u64 = self.get_random_prime(iterations);
        while prime == last_prime { prime = self.get_random_prime(iterations); }
        return prime;
    }

    // Generate a random number min < N < max that is coprime with coprimme
    fn gen_random_coprime_number_in_range(&mut self, min: u64, max: u64, coprime: u64) -> u64 {
        loop {
            let prime: u64 = self.get_rng().random_range(min..max);
            if are_coprime(coprime, prime) { return prime; }
        }
    }
}

type Key = u64;
struct KeyInfo {
    public: Key,
    private: Key,
    shared: Key
}
fn format_keys(keys: &KeyInfo, f: &mut fmt::Formatter) -> fmt::Result {
    writeln!(f, "( public: {}, private: {}, shared: {} )", keys.public, keys.private, keys.shared)
    
}
impl Debug for KeyInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_keys(self, f)
    }
}
impl Display for KeyInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        format_keys(self, f)
    }
}

pub struct Server {
    max_clients: u8,
    handler: NumberHandler
}
impl Server {
    pub fn new(max_clients: u8) -> Self {
        Self{ max_clients, handler: NumberHandler::new() }
    }

    pub fn receive(_request: &Vec<u8>, _response: &mut Vec<u8>) -> bool {
        true
    }

    fn start_rsa(&mut self, iterations: u8) -> KeyInfo {
        let prime_a: u64 = self.handler.get_random_prime(iterations);
        let prime_b: u64 = self.handler.get_different_random_prime(iterations, prime_a);

        let shared: u64 = prime_a * prime_b;
        // Compute Euler's totient funtion for the shared
        let max_range = (prime_a - 1) * (prime_b - 1);
        // Generate random number 1 < N < phi(shared) that is coprime with phi(shared)
        let private = self.handler.gen_random_coprime_number_in_range(1, max_range, max_range);
        let public = get_modular_inverse(private, max_range);

        println!("p = {}, q = {}, n = {}, phi(n) = {}", prime_a, prime_b, shared, max_range);
        println!("(public, private, shared) = ({}, {}, {})", public, private, shared);
        println!("{}", (public * private % max_range));
        println!("{} * {} (mod {}) = 1", private, public, max_range);

        KeyInfo{ private, public, shared }
    }
    fn handle_client(&mut self) -> bool {
        let keys: KeyInfo = self.start_rsa(64);
        println!("{}", keys);
        true
    }
}

fn main() {
    let mut handler: NumberHandler = NumberHandler::new();
    // println!("{}", bigmod(5, 55, 221));
    println!("{}", handler.miller_rabin_prime_test(997, 64));
    println!("{}", get_modular_inverse(37, 50));

    let mut server: Server = Server::new(10);
    server.handle_client();
}