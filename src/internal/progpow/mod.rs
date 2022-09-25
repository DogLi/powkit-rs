use crate::internal::crypto::keccak::keccak_f800;
use crate::internal::progpow;
use crate::internal::progpow::config::Config;
use byteorder::{ByteOrder, LittleEndian};

pub mod algorithm;
pub mod client;
pub mod config;
pub mod convutil;
pub mod kiss99;
pub mod math;
pub mod mix_rng;

pub const RAVEN_COIN_KAWPOW: [u32; 15] = [
    0x00000072, //R
    0x00000041, //A
    0x00000056, //V
    0x00000045, //E
    0x0000004E, //N
    0x00000043, //C
    0x0000004F, //O
    0x00000049, //I
    0x0000004E, //N
    0x0000004B, //K
    0x00000041, //A
    0x00000057, //W
    0x00000050, //P
    0x0000004F, //O
    0x00000057, //W
];

pub fn initialize(hash: &[u8], nonce: u64) -> ([u32; 25], u64) {
    let mut seed: [u32; 25] = [0; 25];
    for i in 0..8 {
        seed[i] = LittleEndian::read_u32(&hash[i * 4..]);
    }
    seed[8] = nonce as u32;
    seed[9] = (nonce >> 32) as u32;
    for i in 10..25 {
        seed[i] = RAVEN_COIN_KAWPOW[i - 10]
    }
    keccak_f800(&mut seed);
    let seed_head = seed[0] as u64 + ((seed[1] as u64) << 32);
    (seed, seed_head)
}

pub fn finalize(seed: [u32; 25], mix_hash: &[u8]) -> Vec<u8> {
    let mut state: [u32; 25] = [0; 25];
    for i in 0..8 {
        state[i] = seed[i];
        state[i + 8] = LittleEndian::read_u32(&mix_hash[i * 4..i * 4 + 4]);
    }
    for i in 16..25 {
        state[i] = RAVEN_COIN_KAWPOW[i - 16]
    }
    keccak_f800(&mut state);
    convutil::u32array_to_bytes(&state[0..8])
}

pub fn kawpow<F: Fn(usize) -> Vec<u32>>(
    hash: &[u8],
    height: u64,
    nonce: u64,
    dataset_size: usize,
    lookup: F,
    l1: &[u8],
) -> (Vec<u8>, Vec<u8>) {
    let cfg = Config {
        period_length: 3,
        dag_loads: 4,
        cache_bytes: 16 * 1024,
        lane_count: 16,
        register_count: 32,
        round_count: 64,
        round_cache_accesses: 11,
        round_math_operations: 18,
    };
    let (seed, seed_head) = initialize(hash, nonce);
    let mix_hash = progpow::algorithm::hash(&cfg, height, seed_head, dataset_size, lookup, l1);
    let digest = progpow::finalize(seed, &mix_hash);
    (mix_hash, digest)
}
