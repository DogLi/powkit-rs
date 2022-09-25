use crate::internal::crypto::fnv::fnv1_a;
use crate::internal::progpow::config::FNV_OFFSET_BASIS;
use crate::internal::progpow::kiss99::Kiss99;

pub struct MixRngState {
    size: usize,
    src_counter: usize,
    dst_counter: usize,
    src_sequence: Vec<u32>,
    dst_sequence: Vec<u32>,
    rng: Kiss99,
}

impl MixRngState {
    pub fn next_src(&mut self) -> u32 {
        let val = self.src_sequence[self.src_counter % self.size];
        self.src_counter += 1;
        val
    }

    pub fn next_dst(&mut self) -> u32 {
        let val = self.dst_sequence[self.dst_counter % self.size];
        self.dst_counter += 1;
        val
    }

    pub fn next_rng(&mut self) -> u32 {
        self.rng.kiss()
    }

    pub fn new(seed: u64, size: u32) -> Self {
        let z = fnv1_a(FNV_OFFSET_BASIS, seed as u32);
        let w = fnv1_a(z, (seed >> 32) as u32);
        let jsr = fnv1_a(w, seed as u32);
        let jcong = fnv1_a(jsr, (seed >> 32) as u32);

        let mut rng = Kiss99::new(z, w, jsr, jcong);
        let mut src_seq: Vec<u32> = (0..size).into_iter().collect();
        let mut dst_seq: Vec<u32> = (0..size).into_iter().collect();
        for i in (2..=size).rev() {
            let index = i as usize - 1;

            let dst_ind = (rng.kiss() % i) as usize;
            dst_seq.swap(index, dst_ind);

            let src_ind = (rng.kiss() % i) as usize;
            src_seq.swap(index, src_ind);
        }

        MixRngState {
            size: size as usize,
            src_counter: 0,
            dst_counter: 0,
            src_sequence: src_seq,
            dst_sequence: dst_seq,
            rng,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    struct T {
        number: u64,
        size: u32,
        src: Vec<u32>,
        dst: Vec<u32>,
        kiss99: Kiss99,
    }

    #[test]
    fn test_init_mix_rng_state() {
        let tt = T {
            number: 30000 / 50,
            size: 32,
            src: vec![
                0x1A, 0x1E, 0x01, 0x13, 0x0B, 0x15, 0x0F, 0x12, 0x03, 0x11, 0x1F, 0x10, 0x1C, 0x04,
                0x16, 0x17, 0x02, 0x0D, 0x1D, 0x18, 0x0A, 0x0C, 0x05, 0x14, 0x07, 0x08, 0x0E, 0x1B,
                0x06, 0x19, 0x09, 0x00,
            ],
            dst: vec![
                0x00, 0x04, 0x1B, 0x1A, 0x0D, 0x0F, 0x11, 0x07, 0x0E, 0x08, 0x09, 0x0C, 0x03, 0x0A,
                0x01, 0x0B, 0x06, 0x10, 0x1C, 0x1F, 0x02, 0x13, 0x1E, 0x16, 0x1D, 0x05, 0x18, 0x12,
                0x19, 0x17, 0x15, 0x14,
            ],
            kiss99: Kiss99 {
                z: 0x6535921C,
                w: 0x29345B16,
                jsr: 0xC0DD7F78,
                jcong: 0x1165D7EB,
            },
        };
        let state = MixRngState::new(tt.number, tt.size);
        assert_eq!(state.rng, tt.kiss99);
        assert_eq!(state.src_sequence, tt.src);
        assert_eq!(state.dst_sequence, tt.dst);
    }
}
