use rand::prelude::*;

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
    // By Fermat's Little Theorem, a -1 (mod m) = a^(m - 2) mod m
    return bigmod(a, m - 2, m);
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

// Test one case of the Miller-Rabin for a potential prime p and a base A
// Assumes that p is odd
// If any of the following is not true:
//   A^(p - 1) = 1 (mod p)
//   A^((p - 1)/2) = 1 (mod p) OR A^((p - 1)/2) = -1 (mod p)
fn number_passes_miller_rabin(prime: u64, base: u64) -> bool {
    assert!((prime & 1) == 1);

    let exp: u64 = prime - 1;
    if bigmod(base, exp, prime) != 1 { return false; }

    let halved = bigmod(base, exp >> 1, prime);
    if halved != 1 && halved != (prime - 1) { return false; }

    return true;
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
        if num < 3 { return num == 2; }
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
    // Generate a u8 with the given number of bits
    // fn get_random_num(&mut self, bit_count: u8) -> u8 {
    //     return self.get_rng().random::<u8>()
    // }
    fn get_random_prime(&mut self, iterations: u8) -> u64 {
        loop {
            let candidate: u64 = self.get_rng().random::<u8>() as u64;
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
            println!("testing {} for coprimality with {}", prime, coprime);
            if are_coprime(coprime, prime) { return prime; }
        }
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

    fn start_rsa(&mut self, iterations: u8) -> bool {
        let prime_a: u64 = self.handler.get_random_prime(iterations);
        let prime_b: u64 = self.handler.get_different_random_prime(iterations, prime_a);

        let product: u64 = prime_a * prime_b;
        // Compute Euler's totient funtion for the product
        let max_range = (prime_a - 1) * (prime_b - 1);

        println!("p = {}, q = {}, n = {}, phi(n) = {}", prime_a, prime_b, product, max_range);

        // Generate random number 1 < N < phi(product) that is coprime with phi(product)
        let public = self.handler.gen_random_coprime_number_in_range(1, max_range, max_range);

        let private = get_modular_inverse(public, max_range);

        println!("(public, private, shared) = ({}, {}, {})", public, private, max_range);


        true
    }
}

fn main() {
    let mut handler: NumberHandler = NumberHandler::new();
    println!("{}", bigmod(5, 55, 221));
    println!("{}", handler.miller_rabin_prime_test(993, 64));
    println!("{}", handler.get_random_prime(64));

    let mut server: Server = Server::new(10);
    server.start_rsa(64);

    println!("{}", gcd(127384, 64));
    // println!("{}", are_coprime(5051, 6496));
}