use crate::internal::dag::cache::generate_dataset_item_unit;
use crate::internal::dag::config::{DagConfig, LookupTable};
use crate::internal::dag::Dag;
use crate::internal::progpow::kawpow;
use crate::kawpow::lookup::{CACHE_SIZES, DATASET_SIZES};
use anyhow::Result;
use std::path::PathBuf;

pub struct Client {
    dag: Dag,
}

impl Client {
    pub fn new(cfg: DagConfig) -> Self {
        let dag = Dag::new(cfg);
        Self { dag }
    }

    pub fn new_raven_coin(storage_dir: PathBuf) -> Self {
        let cfg = DagConfig {
            name: "RVN".into(),
            revision: 23,
            storage_dir,

            dataset_init_bytes: 1 << 30,
            dataset_growth_bytes: 1 << 23,
            cache_init_bytes: 1 << 24,
            cache_growth_bytes: 1 << 17,

            cache_sizes: LookupTable::new(CACHE_SIZES.to_vec()),
            dataset_sizes: LookupTable::new(DATASET_SIZES.to_vec()),

            mix_bytes: 128,
            dataset_parents: 512,
            epoch_length: 7500,
            seed_epoch_length: 7500,

            cache_rounds: 3,
            caches_count: 3,
            caches_lock_mmap: false,

            l1_enabled: true,
            l1_cache_size: 4096 * 4,
            l1_cache_num_items: 4096,
        };
        Self::new(cfg)
    }

    // return mix, degest
    pub fn compute(&self, hash: &[u8; 32], height: u64, nonce: u64) -> Result<(Vec<u8>, Vec<u8>)> {
        let epoch = self.dag.config.calc_epoch(height as usize);
        let cache = self.dag.get_cache(epoch)?;
        let look_up = |index| {
            generate_dataset_item_unit(cache.cache(), index, 4, self.dag.config.dataset_parents)
        };
        let dataset_size = self.dag.config.dataset_size(epoch);

        let r = kawpow(
            hash,
            height as u64,
            nonce,
            dataset_size,
            look_up,
            cache.l1().unwrap(),
        );
        Ok(r)
    }
}
