use rand::prelude::*;

struct U4096 {
    // Little endian system... e.g., if you have 1, then 0b1 will be stored in the first element
    bytes: [u8; 512]
}
impl U4096 {
    pub fn add(&mut self, num: &U4096) {
        let carry: u8 = 0;
        for byte in 0..512 {
            let added: u16 = self.bytes[byte] as u16 + num.bytes[byte] as u16 + carry;
            self.bytes[byte] = (added & 255u8) as u8;
            carry = 0u8;
            if added > 255u8 {
                carry = 1u8;
            }
        }
    }
}

fn number_bit_set(num: u64, bit: u8) -> bool {
    return (num & (1 << bit as u64)) != 0;
}
// Compute s^e mod m
fn bigmod(s: u64, e: u64, m: u64) -> u64 {
    // We're essentially going to multiple s^n for every (1 << b), accounting
    // for every set bit in e
    let mut final_mod: u64 = 1;

    // Position of highest set bit
    let final_bit: u8 = (e as f64 + 1.0).log2().ceil() as u8;
    // The mod of s^(2^n)
    let mut last_mod: u64 = s;

    for bit in 0u8..final_bit {
        if number_bit_set(e, bit) { final_mod *= last_mod };
        last_mod = (last_mod * last_mod) % m;
    }
    println!("got past {}^{}%{}", s, e, m);
    return final_mod % m;
}

// Test one case of the Miller-Rabin for a potential prime p and a base A
// Assumes that p is odd
// If any of the following is not true:
//   A^(p - 1) = 1 (mod p)
//   A^((p - 1)/2) = 1 (mod p) OR A^((p - 1)/2) = -1 (mod p)
fn number_passes_miller_rabin(prime: u64, base: u64) -> bool {
    assert!((prime & 1) == 1);
    println!("trying {} with base {}", prime, base);

    let exp: u64 = prime - 1;
    if bigmod(base, exp, prime) != 1 { return false; }

    let halved = bigmod(base, exp >> 1, prime);
    if halved != 1 && halved != (prime - 1) { return false; }

    return true;
}

static first_primes: [u64; 200] = [
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
        if num == 2 { return true; }
        if (num & 1) == 0 { return false; }
    
        // // Find a 2^e * m = num
        // let mut e: u8 = 0;
        // let mut m: u8 = 0;
        // let mut even: u64 = num - 1;
    
        // while (even & 1) == 0 {
        //     e = e + 1;
        //     even = even >> 1;
        // }
        // m = (even & 0b11111111) as u8;
        // println!("2^{}*{}={}", e, m, num);
    
        for _iter in 0..iterations {
            let mut base: u64 = self.get_rng().random_range(2u64..num);
            // Make sure it's odd -- an even number is obviously not a prime
            if base != 2 && (base & 1) == 0 { base = base | 1; }
            if !number_passes_miller_rabin(num, base) { return false; }
        }
    
        return true;
    }
    fn get_random_u64(&mut self) -> u64 {
        return self.get_rng().random::<u64>();
    }
    fn get_random_prime(&mut self, iterations: u8) -> u64 {
        while (true) {
            let candidate: u64 = self.get_rng().random::<u16>() as u64;
            let mut valid: bool = true;
            for prime in 0..first_primes.len() {
                if candidate % first_primes[prime] == 0 {
                    valid = false;
                    break;
                }
            }
            if valid && self.miller_rabin_prime_test(candidate, iterations) { return candidate; }
        }
        return 1;
    }
}

pub struct Server {
    max_clients: u8
}
impl Server {
    pub fn receive(_request: &Vec<u8>, _response: &mut Vec<u8>) -> bool {
        true
    }
}

fn main() {
    let mut handler: NumberHandler = NumberHandler::new();
    println!("{}", bigmod(5, 55, 221));
    println!("{}", bigmod(3, 7, 11));
    println!("{}", handler.miller_rabin_prime_test(993, 64));
    println!("{}", handler.get_random_prime(64));
}