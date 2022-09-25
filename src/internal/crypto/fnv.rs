use byteorder::{ByteOrder, LittleEndian};

pub const FNV_PRIME: u32 = 0x01000193;

pub fn fnv1(v1: u32, v2: u32) -> u32 {
    let v1 = v1 as u64;
    let v2 = v2 as u64;
    ((v1 * FNV_PRIME as u64) ^ v2) as _
}

pub fn fnv1_u64(u: u64, v: u64) -> u64 {
    (u * FNV_PRIME as u64) ^ v
}

pub fn fnv1_a(u: u32, v: u32) -> u32 {
    ((u ^ v) as u64 * FNV_PRIME as u64) as _
}

pub fn fnv_hash(mix: &mut [u32], data: &[u32]) {
    for i in 0..mix.len() {
        mix[i] = fnv1(mix[i], data[i])
    }
}
pub fn fnv64(a: [u8; 64], b: [u8; 64]) -> [u8; 64] {
    let mut r = [0u8; 64];
    for i in 0..(64 / 4) {
        let j = i * 4;

        LittleEndian::write_u32(
            &mut r[j..],
            fnv1(
                LittleEndian::read_u32(&a[j..]),
                LittleEndian::read_u32(&b[j..]),
            ),
        );
    }
    r
}

pub fn fnv128(a: [u8; 128], b: [u8; 128]) -> [u8; 128] {
    let mut r = [0u8; 128];
    for i in 0..(128 / 4) {
        let j = i * 4;

        LittleEndian::write_u32(
            &mut r[j..],
            fnv1(
                LittleEndian::read_u32(&a[j..]),
                LittleEndian::read_u32(&b[j..]),
            ),
        );
    }
    r
}
