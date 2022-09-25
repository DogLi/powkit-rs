use crate::internal::crypto::prime::is_prime;
use crate::internal::dag::SeedType;
use sha3::{Digest, Keccak256};
use std::path::PathBuf;

pub const HASH_BYTES: usize = 64;
pub const WORD_BYTES: usize = 4;

pub struct LookupTable {
    pub table: Vec<usize>,
}

impl LookupTable {
    pub fn new(table: Vec<usize>) -> Self {
        Self { table }
    }
}

pub struct DagConfig {
    pub name: String,
    pub revision: u64,
    pub storage_dir: PathBuf,

    /// Bytes in dataset at genesis
    pub dataset_init_bytes: usize,
    /// Dataset growth per epoch
    pub dataset_growth_bytes: usize,
    /// Bytes in cache at genesis
    pub cache_init_bytes: usize,
    /// Cache growth per epoch
    pub cache_growth_bytes: usize,

    // lookup tables
    pub dataset_sizes: LookupTable,
    pub cache_sizes: LookupTable,

    // algorithm variables
    pub mix_bytes: usize,
    pub dataset_parents: usize,
    pub epoch_length: usize,
    // ETC uses a different seed epoch length
    pub seed_epoch_length: usize,

    // cache variables
    pub cache_rounds: usize,
    // Maximum number of caches to keep before eviction (only init, don't modify)
    pub caches_count: usize,
    pub caches_lock_mmap: bool,

    // // L1 variables
    pub l1_enabled: bool,
    pub l1_cache_size: usize,
    pub l1_cache_num_items: usize,
}

impl DagConfig {
    pub fn file_path(&self, epoch: usize, is_l1: bool) -> PathBuf {
        let name = if is_l1 {
            format!("l1-{:?}", epoch)
        } else {
            format!("cache-{:?}", epoch)
        };
        self.storage_dir.join(name)
    }

    pub fn seed_hash(&self, height: usize) -> SeedType {
        let mut seed: [u8; 32] = Default::default();
        if height < self.seed_epoch_length {
            return seed;
        }
        for _ in 0..height / self.seed_epoch_length {
            let mut hasher = Keccak256::default();
            hasher.update(&mut seed);
            seed = hasher.finalize().to_vec().try_into().unwrap();
        }
        seed
    }

    pub fn dataset_size(&self, epoch: usize) -> usize {
        if epoch < self.dataset_sizes.table.len() {
            self.dataset_sizes.table[epoch]
        } else {
            self.calc_dataset_size(epoch)
        }
    }

    pub fn cache_size(&self, epoch: usize) -> usize {
        if epoch < self.cache_sizes.table.len() {
            return self.cache_sizes.table[epoch];
        }
        self.calc_cache_size(epoch)
    }

    fn calc_cache_size(&self, epoch: usize) -> usize {
        let mut size = self.cache_init_bytes + self.cache_growth_bytes * epoch - HASH_BYTES;
        if !is_prime(size / HASH_BYTES) {
            size -= 2 * self.mix_bytes;
        }
        size
    }

    fn calc_dataset_size(&self, epoch: usize) -> usize {
        let mut size = self.dataset_init_bytes + self.dataset_growth_bytes * epoch - self.mix_bytes;
        if !is_prime(size / self.mix_bytes) {
            size -= 2 * self.mix_bytes;
        }
        size
    }

    pub fn calc_epoch(&self, height: usize) -> usize {
        height / self.epoch_length
    }
}
