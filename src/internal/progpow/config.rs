pub const FNV_OFFSET_BASIS: u32 = 0x811c9dc5;

pub struct Config {
    pub period_length: usize,
    pub dag_loads: usize,
    pub cache_bytes: usize,
    pub lane_count: usize,
    pub register_count: usize,
    pub round_count: usize,
    pub round_cache_accesses: usize,
    pub round_math_operations: usize,
}

impl Config {
    pub fn progpow094() -> Self {
        Config {
            period_length: 10,
            dag_loads: 4,
            cache_bytes: 16 * 1024,
            lane_count: 16,
            register_count: 32,
            round_count: 64,
            round_cache_accesses: 11,
            round_math_operations: 18,
        }
    }
}
