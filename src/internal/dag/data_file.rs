use crate::internal::dag::cache::{generate_cache, generate_l1_cache};
use crate::internal::dag::config::DagConfig;
use anyhow::Result;
use memmap::{Mmap, MmapMut};
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::fs::OpenOptions;
use std::sync::Arc;
use std::{fs, io};
use walkdir::{DirEntry, WalkDir};

#[derive(Clone)]
pub struct DataFile {
    pub is_l1: bool,
    pub epoch: usize,
    mmap: Arc<Mmap>,
}

static GENERATOR_LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

impl DataFile {
    pub fn generate(config: &DagConfig, epoch: usize, cache: Option<&[u8]>) -> Result<Self> {
        let _lock = GENERATOR_LOCK.lock();
        if let Ok(cache) = Self::new_from_file(config, epoch, cache.is_some()) {
            return Ok(cache);
        }
        Self::new(config, epoch, cache)
    }

    fn new_from_file(config: &DagConfig, epoch: usize, is_l1: bool) -> Result<Self> {
        let path = config.file_path(epoch, is_l1);
        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(&path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        let cache_size = if is_l1 {
            config.l1_cache_size
        } else {
            config.cache_size(epoch)
        };
        if mmap.len() != cache_size + 1 {
            bail!("invalid cache file");
        }
        // if the last bit is 1, the data is valid
        let finished_flag = mmap[cache_size];
        if finished_flag != 1 {
            bail!("invalid cache file, not finished");
        }
        Ok(Self {
            epoch,
            is_l1,
            mmap: Arc::new(mmap),
        })
    }

    fn new(config: &DagConfig, epoch: usize, cache: Option<&[u8]>) -> Result<Self> {
        let path = config.file_path(epoch, cache.is_some());
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&path)?;
        let is_l1 = cache.is_some();
        let cache_size = if is_l1 {
            config.l1_cache_size
        } else {
            config.cache_size(epoch)
        };
        file.set_len(cache_size as u64 + 1)?;
        let mut mmap = unsafe { MmapMut::map_mut(&file)? };
        if let Some(cache) = cache {
            generate_l1_cache(&mut mmap, cache, config.dataset_parents);
        } else {
            let seed = config.seed_hash(epoch * config.epoch_length + 1);
            generate_cache(&mut mmap, &seed, config.cache_rounds);
        }
        Self::flush(config, epoch, &mut mmap, is_l1)?;
        Ok(Self {
            epoch,
            is_l1,
            mmap: Arc::new(mmap.make_read_only()?),
        })
    }

    fn flush(config: &DagConfig, epoch: usize, mmap: &mut MmapMut, is_l1: bool) -> Result<()> {
        fn is_old_cache_file(entry: &DirEntry, small_epoch: usize, is_l1: bool) -> bool {
            let file_name = entry.file_name().to_str();
            if file_name.is_none() {
                return false;
            }
            let file_name = file_name.unwrap();
            let s: Vec<_> = file_name.rsplitn(1, "-").collect();
            if s.len() != 2 {
                return true;
            }
            let old_epoch: usize = s[1].parse().unwrap_or_default();
            let is_old = old_epoch < small_epoch;
            if is_l1 {
                file_name.starts_with("l1") && is_old
            } else {
                !file_name.starts_with("l1") && is_old
            }
        }
        mmap.flush()?;
        let old_epoch = epoch.checked_sub(3);
        if old_epoch.is_none() {
            return Ok(());
        }
        let old_epoch = old_epoch.unwrap();
        for old_cache_file in WalkDir::new(&config.storage_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|p| is_old_cache_file(p, old_epoch, is_l1))
            .map(|p| p.path().to_path_buf())
        {
            fs::remove_file(old_cache_file).unwrap_or_else(|error| match error.kind() {
                io::ErrorKind::NotFound => (),
                _ => warn!("Error removing stale DAG cache: {:?}", error),
            })
        }
        Ok(())
    }

    pub fn data(&self) -> &[u8] {
        let cache_len = self.mmap.len();
        let dag: &[u8] = &self.mmap[0..cache_len - 1];
        dag
    }

    // pub fn compute_light(&self, hash: H256, nonce: H64) -> (H256, H256) {
    //     let cache_len = self.cache.len();
    //     let dag = &self.cache[0..cache_len - 1];
    //     crate::hashimoto_light(hash, nonce, self.full_size, dag)
    // }
}
