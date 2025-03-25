/* Program to implement SHA-256 Hash. Quoted comments come from specification at https://helix.stormhub.org/papers/SHA-256.pdf */ 

// "The first 32 bits of the fractional parts of the cube roots of the first 64 prime numbers"
static K: [u32; 64] = [
    0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
    0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
    0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
    0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
    0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
    0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
    0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
    0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2
];

// A circular rotate right, so eg rotate_right(0b00111000, 4) = 0b00000011
fn rotate_right(word: u32, shift: u8) -> u32 {
    assert!(shift < 32);
    // (word >> (32 - shift)) | (word << shift)
    (word >> shift) | ((word & ((1 << shift) - 1)) << (32 - shift))
}
// Choose function: if a bit is 0 in x, we concatenate the corresponding
// bit in z. If it's 1, we use the bit in y
fn ch(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (!x & z)
}
fn maj(x: u32, y: u32, z: u32) -> u32 {
    (x & y) ^ (x & z) ^ (y & z)
}
fn sigma_0(x: u32) -> u32 {
    rotate_right(x, 2) ^ rotate_right(x, 13) ^ rotate_right(x, 22)
}
fn sigma_1(x: u32) -> u32 {
    rotate_right(x, 6) ^ rotate_right(x, 11) ^ rotate_right(x, 25)
}
fn sigmoid_0(x: u32) -> u32 {
    rotate_right(x, 7) ^ rotate_right(x, 18) ^ (x >> 3)
}
fn sigmoid_1(x: u32) -> u32 {
    rotate_right(x, 17) ^ rotate_right(x, 19) ^ (x >> 10)
}
fn u64_to_u32(a: u64) -> u32 {
    (a & ((1u64 << 31) - 1)) as u32
}
// Compute the 64 32-bit blocks of the input, starting at the given byte index
fn get_blocks(input: &Vec<u8>, start_ind: usize) -> [u32; 64] {
    let mut blocks: [u32; 64] = [0; 64];
    for i in 0usize..16usize {
        let start: usize = start_ind + i * 4;
        blocks[i] = u32::from_le_bytes([input[start], input[start + 1], input[start + 2], input[start + 3]]);
    }
    // Now get the remaining 48
    for i in 16usize..64usize {
        println!("sig0({}) = {}, sig1({}) = {}", blocks[i - 15], sigmoid_0(blocks[i - 15]), blocks[i - 2], sigmoid_1(blocks[i - 2]));
        blocks[i] =
            u64_to_u32(u64::from(sigmoid_1(blocks[i - 2])) + u64::from(blocks[i - 7]) +
                u64::from(sigmoid_0(blocks[i - 15])) + u64::from(blocks[i - 16]))
    }
    blocks
}
// Compute the 8 new hash values for the next 64 32-bit blocks
fn get_new_hashes(old_hashes: &mut [u32; 8], blocks: [u32; 64]) {
    let mut a = old_hashes[0];
    let mut b = old_hashes[1];
    let mut c = old_hashes[2];
    let mut d = old_hashes[3];
    let mut e = old_hashes[4];
    let mut f = old_hashes[5];
    let mut g = old_hashes[6];
    let mut h = old_hashes[7];

    for i in 0usize..64usize {
        let t1: u64 = u64::from(h) + u64::from(sigma_1(e)) + u64::from(ch(e, f, g)) + u64::from(K[i]) + u64::from(blocks[i]);
        let t2: u64 = u64::from(sigma_0(a)) + u64::from(maj(a, b, c));
        h = g;
        g = f;
        f = e;
        e = u64_to_u32(u64::from(d) + t1);
        d = c;
        c = b;
        b = a;
        a = u64_to_u32(t1 + t2);
    }
    old_hashes[0] = u64_to_u32(u64::from(old_hashes[0]) + u64::from(a));
    old_hashes[1] = u64_to_u32(u64::from(old_hashes[1]) + u64::from(b));
    old_hashes[2] = u64_to_u32(u64::from(old_hashes[2]) + u64::from(c));
    old_hashes[3] = u64_to_u32(u64::from(old_hashes[3]) + u64::from(d));
    old_hashes[4] = u64_to_u32(u64::from(old_hashes[4]) + u64::from(e));
    old_hashes[5] = u64_to_u32(u64::from(old_hashes[5]) + u64::from(f));
    old_hashes[6] = u64_to_u32(u64::from(old_hashes[6]) + u64::from(g));
    old_hashes[7] = u64_to_u32(u64::from(old_hashes[7]) + u64::from(h));
}
#[inline]
fn u32s_to_u64(high: u32, low: u32) -> u64 {
    (u64::from(high) << 32) | u64::from(low)
}
// Hash the input.
fn hash_input(input: &mut Vec<u8>) -> [u64; 4] {
    let initial_length: u64 = input.len() as u64;
    // First, add a one
    // Then, add zeroes until the input length = 448 (mod 512)
    let last_block_size: usize = ((input.len() << 3)) & 511;
    let bits: usize = 448 + if last_block_size > 448 { 512 } else { 0 } - last_block_size;
    println!("{:#?} needs {} bits", input, bits);
    input.push(0b10000000);
    // We can assume that the input had a number of bits divisble by 8,
    // so don't worry about individual byte padding
    for _ in 0..((bits - 8) >> 3) {
        input.push(0);
    }
    for byte in initial_length.to_le_bytes() {
        input.push(byte);
    }
    println!("{:?} {:?}", input.len(), input);

    // Initialize 8 hash values to "the first 32 bits of the fractional part of the square roots of the
    // first 8 prime numbers"
    let mut hashes: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0x3c6ef372,
        0x3c6ef372, 0x3c6ef372, 0x3c6ef372, 0x3c6ef372
    ];
    let mut byte_ind = 0;
    while byte_ind < input.len() {
        let blocks: [u32; 64] = get_blocks(input, byte_ind);
    println!("Blocks: {:?}", blocks);
        get_new_hashes(&mut hashes, blocks);
        byte_ind = byte_ind + 512;
        println!("{:?}", hashes);
    }

    println!("{:b}", hashes[0]);
    [
        u32s_to_u64(hashes[0], hashes[1]),
        u32s_to_u64(hashes[2], hashes[3]),
        u32s_to_u64(hashes[4], hashes[5]),
        u32s_to_u64(hashes[6], hashes[7])
    ]
}

pub fn sha256(input: &str) -> [u64; 4] {
    assert_eq!(rotate_right(0xffeeddcc, 8), 0xccffeedd);
    // assert_eq!(ch(0b1010001_00001110_01010010_01111111, 0b10011011_00000101_01101000_10001100, 0b00011111_10000011_11011001_10101011), 0b00011111_10000101_11001001_10001100);
    hash_input(&mut input.as_bytes().to_vec())
}