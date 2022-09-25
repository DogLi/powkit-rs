use sha3::{Digest, Keccak256, Keccak512};

pub fn fill_sha256(input: &[u8], a: &mut [u8], from_index: usize) {
    let mut hasher = Keccak256::default();
    hasher.update(input);
    let out = hasher.finalize();
    for i in 0..out.len() {
        a[from_index + i] = out[i];
    }
}

pub fn fill_sha512(input: &[u8], output: &mut [u8], from_index: usize) {
    let mut hasher = Keccak512::default();
    hasher.update(input);
    let out = hasher.finalize();
    for i in 0..out.len() {
        output[from_index + i] = out[i];
    }
}
