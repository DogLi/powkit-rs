use crate::internal::crypto::fnv::fnv1_a;
use crate::internal::progpow::config::{Config, FNV_OFFSET_BASIS};
use crate::internal::progpow::convutil::u32array_to_bytes;
use crate::internal::progpow::kiss99::Kiss99;
use crate::internal::progpow::math::{random_math, random_merge};
use crate::internal::progpow::mix_rng::MixRngState;
use byteorder::{ByteOrder, LittleEndian};

pub fn init_mix(seed: u64, num_lanes: usize, num_regs: usize) -> Vec<Vec<u32>> {
    let z = fnv1_a(FNV_OFFSET_BASIS, seed as u32);
    let w = fnv1_a(z, (seed >> 32) as u32);

    let mut mix: Vec<Vec<u32>> = vec![vec![0; num_regs]; num_lanes];
    for lane in 0..num_lanes {
        let jsr = fnv1_a(w, lane as u32);
        let jcong = fnv1_a(jsr, lane as u32);
        let mut rng = Kiss99::new(z, w, jsr, jcong);
        for reg in 0..num_regs {
            mix[lane][reg] = rng.next();
        }
    }
    mix
}

pub fn round<F: Fn(usize) -> Vec<u32>>(
    cfg: &Config,
    seed: u64,
    r: usize,
    mix: &mut [Vec<u32>],
    dataset_size: usize,
    lookup: &F,
    l1: &[u8],
) {
    let mut state = MixRngState::new(seed, cfg.register_count as u32);
    let num_items = (dataset_size / (2 * 128)) as u32;
    let item_index = mix[r % (cfg.lane_count)][0] % num_items;
    let item = lookup(item_index as usize);
    let num_words_per_lane = item.len() / cfg.lane_count;
    let max_operations = usize::max(cfg.round_cache_accesses, cfg.round_math_operations);
    for i in 0..max_operations {
        if i < cfg.round_cache_accesses {
            let src = state.next_src();
            let dst = state.next_dst() as usize;
            let sel = state.next_rng();
            for l in 0..cfg.lane_count {
                let offset = mix[l][src as usize] as usize % (cfg.cache_bytes / 4);
                let u32_l1 = LittleEndian::read_u32(&l1[offset * 4..]);
                mix[l][dst] = random_merge(mix[l][dst], u32_l1, sel);
            }
        }

        if i < cfg.round_math_operations {
            let src_rand =
                state.next_rng() % ((cfg.register_count as u32) * (cfg.register_count - 1) as u32);
            let src1 = (src_rand % cfg.register_count as u32) as usize;
            let mut src2 = (src_rand / cfg.register_count as u32) as usize;
            if src2 >= src1 {
                src2 += 1;
            }

            let sel1 = state.next_rng();
            let dst = state.next_dst() as usize;
            let sel2 = state.next_rng();
            for l in 0..cfg.lane_count {
                let data = random_math(mix[l][src1], mix[l][src2], sel1);
                mix[l][dst] = random_merge(mix[l][dst], data, sel2);
            }
        }
    }

    // DAG access pattern
    let mut dsts: Vec<u32> = vec![0; num_words_per_lane];
    let mut sels: Vec<u32> = vec![0; num_words_per_lane];
    for i in 0..num_words_per_lane {
        dsts[i] = if i == 0 { 0 } else { state.next_dst() };
        sels[i] = state.next_rng();
    }

    for l in 0..cfg.lane_count {
        let offset =
            ((l as u32) ^ r as u32) % (cfg.lane_count as u32) * (num_words_per_lane as u32);
        for i in 0..num_words_per_lane {
            let index = offset as usize + i;
            let word = item[index];
            let r = dsts[i] as usize;
            mix[l][r] = random_merge(mix[l][r], word, sels[i]);
        }
    }
}

pub fn hash<F: Fn(usize) -> Vec<u32>>(
    cfg: &Config,
    height: u64,
    seed: u64,
    dataset_size: usize,
    lookup: F,
    l1: &[u8],
) -> Vec<u8> {
    let mut mix = init_mix(seed, cfg.lane_count, cfg.register_count as usize);
    let number = height / cfg.period_length as u64;
    for i in 0..cfg.round_count {
        round(cfg, number, i, &mut mix, dataset_size, &lookup, l1);
    }
    let mut lane_hash = vec![0; cfg.lane_count];
    for l in 0..lane_hash.len() {
        lane_hash[l] = FNV_OFFSET_BASIS;
        for i in 0..cfg.register_count {
            lane_hash[l] = fnv1_a(lane_hash[l], mix[l][i]);
        }
    }
    let num_words = 8;
    let mut mix_hash: Vec<u32> = vec![FNV_OFFSET_BASIS; num_words];
    for l in 0..cfg.lane_count {
        mix_hash[l % num_words] = fnv1_a(mix_hash[l % num_words], lane_hash[l]);
    }
    u32array_to_bytes(&mix_hash)
}
