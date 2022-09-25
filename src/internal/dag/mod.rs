use anyhow::Result;
use parking_lot::RwLock;
use std::collections::HashMap;

use crate::internal::dag::cache::Cache;
use crate::internal::dag::config::DagConfig;

pub mod cache;
pub mod config;
pub(crate) mod data_file;

pub type SeedType = [u8; 32];

pub struct Dag {
    pub config: DagConfig,
    /// epoch -> Cache
    caches: RwLock<HashMap<usize, Cache>>,
}

impl Dag {
    pub fn new(config: DagConfig) -> Self {
        Self {
            config,
            caches: Default::default(),
        }
    }

    pub fn get_cache(&self, epoch: usize) -> Result<Cache> {
        let cache = {
            let caches = self.caches.read();
            caches.get(&epoch).cloned()
        };
        if let Some(cache) = cache {
            return Ok(cache);
        }
        let new_cache = Cache::generate(epoch, &self.config)?;
        let mut caches = self.caches.write();
        caches.insert(epoch, new_cache.clone());
        Ok(new_cache)
    }
}
