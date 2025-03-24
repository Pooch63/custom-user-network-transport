use rand::prelude::*;
#[cfg(feature = "std")]
use std::fmt;
#[cfg(feature = "std")]
use std::fmt::{ Debug, Display };

mod primes;
mod keygen;
use keygen::{ Key, RSAKeyInfo, NumberHandler, bigmod };

}
}

struct NumberHandler {
    key_byte_size: usize,
    rng: ThreadRng
}
        }
    }
    fn get_random(&self, rng: &mut ThreadRng) -> &KeyInfo {
        // Make sure the list isn't empty. If it is, we can't sample from it
        assert!(self.current_size > 0);
        &self.keys[rng.random_range(0usize..self.current_size)]
    }
}

pub struct Server {
    handler: NumberHandler
}
impl Server {
    // Key size in bytes
    pub fn new(key_byte_size: usize) -> Self {
        // A key can be at max one byte less than half of the max capacity
        // Otherwise, overflow errors will occur
        assert!(key_byte_size < (Key::BYTES >> 1).try_into().unwrap());
        Self{ handler: NumberHandler::new(key_byte_size) }
    }

    pub fn receive(_request: &Vec<u8>, _response: &mut Vec<u8>) -> bool {
        true
    }

    fn start_rsa(&mut self, iterations: u8) -> KeyInfo {
        let prime_a: Key = self.handler.get_random_prime(iterations);
        let prime_b: Key = self.handler.get_different_random_prime(iterations, prime_a);

        let shared: Key = prime_a * prime_b;
        // Compute Euler's totient funtion for the shared
        let max_range = (prime_a - Key::ONE) * (prime_b - Key::ONE);
        // Generate random number 1 < N < phi(shared) that is coprime with phi(shared)
        let private = self.handler.gen_random_coprime_number_in_range(Key::ONE, max_range, max_range);
        let public = get_modular_inverse(private, max_range);

        KeyInfo{ private, public, shared }
    }
    fn handle_client(&mut self) -> bool {
        let keys: KeyInfo = self.start_rsa(64);
        println!("{}", keys);

        let payload = Key::NINE;
        let encrypted = bigmod(payload, keys.private, keys.shared);
        let decrypted = bigmod(encrypted, keys.public, keys.shared);
        println!("{}", decrypted);

        true
    }
}

fn main() {
    let mut server: Server = Server::new(100);
    server.handle_client();
    // server.start_debug_rsa();
}