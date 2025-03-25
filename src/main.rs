use std::env;
use rand::prelude::*;

mod primes;
mod hash;
use hash::{ sha256 };
mod keygen;
use keygen::{ Key, RSAKeyInfo, NumberHandler, bigmod };

mod socket;
use crate::socket::WinSock;

trait Default {
    const DEFAULT: Self;
}
impl Default for RSAKeyInfo {
    const DEFAULT: RSAKeyInfo = RSAKeyInfo { public: Key::ZERO, private: Key::ZERO, shared: Key::ZERO };
}

/* Keys take a while to generate, so the server will store a list of keys from which to choose randomly.
    It will keep generating new keys in the background and add the key info to the list of possible keys.
    We also can't store an unlimited number of keys -- they're way too big. So, after a while, we'll overwrite
    the oldest keys. */
struct KeysContainer<KeyInfo: Default, const N: usize> {
    keys: [KeyInfo; N],
    current_size: usize,
    insert_index: usize
}
impl<KeyInfo: Default + Copy, const N: usize> KeysContainer<KeyInfo, { N }> {
    fn new() -> Self {
        Self{
            keys: [KeyInfo::DEFAULT; N],
            current_size: 0,
            insert_index: 0 }
    }
    fn insert_key(&mut self, key: &KeyInfo) {
        self.keys[self.insert_index] = *key;
        self.insert_index = self.insert_index + 1;
        // Start overwriting if we're at max capacity
        if self.current_size < N - 1 {
            self.current_size = self.current_size + 1;
            self.insert_index = self.insert_index % N;
        }
    }
    fn get_random(&self, rng: &mut ThreadRng) -> &KeyInfo {
        // Make sure the list isn't empty. If it is, we can't sample from it
        assert!(self.current_size > 0);
        &self.keys[rng.random_range(0usize..self.current_size)]
    }
}

pub struct Server<const MAX_KEYS: usize = 10> {
    handler: NumberHandler,
    rsa_keys: KeysContainer<RSAKeyInfo, { MAX_KEYS }>
}
impl<const MAX_KEYS: usize> Server<{ MAX_KEYS }> {
    // Key size in bytes
    pub fn new(key_byte_size: usize) -> Self {
        // A key can be at max one byte less than half of the max capacity
        // Otherwise, overflow errors will occur
        assert!(key_byte_size < (Key::BYTES >> 1).try_into().unwrap());
        Self{ rsa_keys: KeysContainer::<RSAKeyInfo, { MAX_KEYS }>::new(), handler: NumberHandler::new(key_byte_size) }
    }

    pub fn receive(_request: &Vec<u8>, _response: &mut Vec<u8>) -> bool {
        true
    }

    fn get_rsa_keys(&mut self, iterations: u8) -> RSAKeyInfo {
        let prime_a: Key = self.handler.get_random_prime(iterations);
        let prime_b: Key = self.handler.get_different_random_prime(iterations, prime_a);

        let shared: Key = prime_a * prime_b;
        // Compute Euler's totient funtion for the shared
        let max_range = (prime_a - Key::ONE) * (prime_b - Key::ONE);
        // Generate random number 1 < N < phi(shared) that is coprime with phi(shared)
        let private = self.handler.gen_random_coprime_number_in_range(Key::ONE, max_range, max_range);
        let public = keygen::get_modular_inverse(private, max_range);

        RSAKeyInfo{ private, public, shared }
    }
    // fn hash_dhke(&self, dhke: DHKEKeyInfo) -> [u64; 4] {}
    // fn get_dhke_keys(&mut self, iterations: u8) -> DHKEKeyInfo {
    //     let shared_base: Key = self.handler.get_random_prime(iterations);
    //     let shared_mod: Key = self.handler.gen_random_coprime(shared_base);
    // }
    fn handle_client(&mut self) -> bool {
        let keys: RSAKeyInfo = self.get_rsa_keys(64);
        println!("{}", keys);

        let payload = Key::NINE;
        let encrypted = bigmod(payload, keys.private, keys.shared);
        let decrypted = bigmod(encrypted, keys.public, keys.shared);
        println!("{}", decrypted);

        true
    }
}

fn main() {
    // let sha = hash::sha256("quisieara");
    // println!("{:x}{:x}{:x}{:x}", sha[0], sha[1], sha[2], sha[3]);
    // let mut server: Server = Server::new(100);
    // server.handle_client();
    // server.start_debug_rsa();

    let args: Vec<String> = env::args().collect();

    socket::initialize_sockets();

    if args.len() == 1 || args[1] == "server" {
        let server = socket::create_server_socket();
        socket::server_listen(server);

        socket::close_socket(server);
        socket::clean_up();
    }
    else if args[1] == "queue" {
        if args.len() == 2 {
            panic!("Queue expects a socket index argument");
        }
        let client = socket::create_client_socket();
        socket::queue_server(
            client,
            WinSock::SOCKET(args[2].parse::<usize>().unwrap())
        );
    }
}