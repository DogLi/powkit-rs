use crate::internal::crypto::fnv::{fnv1, fnv64};
use crate::internal::crypto::hasher::fill_sha512;
use crate::internal::dag::config::{DagConfig, HASH_BYTES, WORD_BYTES};
use crate::internal::dag::data_file::DataFile;
use crate::internal::dag::SeedType;
use anyhow::Result;
use byteorder::{ByteOrder, LittleEndian};
use std::ops::BitXor;

/// Make an Ethash cache using the given seed.
pub fn generate_cache(cache: &mut [u8], seed: &SeedType, cache_rounds: usize) {
    let cache_len = cache.len() - 1;
    assert_eq!(cache_len % HASH_BYTES, 0);
    let n = cache_len / HASH_BYTES;

    fill_sha512(seed, cache, 0);

    for i in 1..n {
        let (last, next) = cache.split_at_mut(i * 64);
        fill_sha512(&last[(last.len() - 64)..], next, 0);
    }

    for _ in 0..cache_rounds {
        for i in 0..n {
            let v = (LittleEndian::read_u32(&cache[(i * 64)..]) as usize) % n;

            let mut r = [0u8; 64];
            for j in 0..64 {
                let a = cache[((n + i - 1) % n) * 64 + j];
                let b = cache[v * 64 + j];
                r[j] = a.bitxor(b);
            }
            fill_sha512(&r, cache, i * 64);
        }
    }
    cache[cache_len] = 1;
}

/// Calculate the dataset item.
pub fn generate_dataset_item(cache: &[u8], i: usize, dataset_parents: usize) -> [u8; HASH_BYTES] {
    let n = cache.len() / HASH_BYTES;
    let r = HASH_BYTES / WORD_BYTES;
    let mut mix = [0u8; HASH_BYTES];
    for j in 0..64 {
        mix[j] = cache[(i % n) * 64 + j];
    }
    let mix_first32 = LittleEndian::read_u32(mix.as_ref()).bitxor(i as u32);
    LittleEndian::write_u32(mix.as_mut(), mix_first32);
    {
        fill_sha512(&mix.clone(), &mut mix, 0);
    }
    for j in 0..dataset_parents {
        let cache_index = fnv1(
            (i.bitxor(j) & (u32::MAX as usize)) as u32,
            LittleEndian::read_u32(&mix[(j % r * 4)..]),
        ) as usize;
        let mut item = [0u8; 64];
        let cache_index = cache_index % n;
        for i in 0..64 {
            item[i] = cache[cache_index * 64 + i];
        }
        mix = fnv64(mix, item);
    }
    let mut z = [0u8; HASH_BYTES];
    fill_sha512(&mix, &mut z, 0);
    z
}

// generateDatasetItem combines data from 256 pseudorandomly selected cache nodes,
// and hashes that to compute a single dataset node.
// fn generate_dataset_item(cache: &[u32], index: usize, dataset_parents: u32) -> Vec<u8> {
//     let rows = cache.len() / HASH_BYTES;
//     // Initialize the mix
//     let mut mix: Vec<u8> = Vec::with_capacity(HASH_BYTES);
//     let index_0 = (index % rows) * HASH_WORDS;
//     LittleEndian::write_u32(&mut mix, cache[index_0]^index);
//     // 1 - 16
//     for i in 1..HASH_WORDS {
//         let cache_index = (index % rows) * HASH_BYTES + i;
//         LittleEndian::write_u32(&mut mix[i*4..], cache[cache_index]);
//     }
//     fill_sha512(&mix.clone(), &mut mix, 0);
//
//     // Vonvert the mix to uint32 to avoid constant bit shifting
//     let mut int_mix: Vec<u32> = Vec::with_capacity(HASH_WORDS);
//     for i in 0..HASH_WORDS {
//         int_mix[i] = LittleEndian::read_u32(&mix[i*4..]);
//     }
//
//     // fnv it with a lot of random cache nodes based on index
//     for i in 0..dataset_parents {
//         let parent = fnv1(index as u32^i, int_mix[i%16]) % rows;
//         fnv_hash(&mut int_mix, &cache[parent as usize * HASH_WORDS..]);
//     }
//
//     // Flatten the uint32 mix into a binary one and return
//     u32array_to_bytes(&int_mix, &mut mix);
//     fill_sha512(&mix, &mut mix, 0);
//     mix
// }

pub fn generate_dataset_item_unit(
    cache: &[u8],
    index: usize,
    size: usize,
    dataset_parents: usize,
) -> Vec<u32> {
    let hash_words = HASH_BYTES / WORD_BYTES;
    let mut data = vec![0; hash_words * size];
    for n in 0..size {
        let item = generate_dataset_item(cache, index * size + n, dataset_parents);
        for i in 0..hash_words {
            data[n * hash_words + i] = LittleEndian::read_u32(&item[i * 4..]);
        }
    }
    data
}

pub fn generate_l1_cache(l1: &mut [u8], cache: &[u8], dataset_parents: usize) {
    let size = l1.len() - 1;
    let rows = size / HASH_BYTES;
    for i in 0..rows {
        let item = generate_dataset_item(&cache, i, dataset_parents);
        l1[i * HASH_BYTES..].copy_from_slice(&item);
    }
    l1[size] = 1;
}

#[derive(Clone)]
pub struct Cache {
    cache: DataFile,
    l1: Option<DataFile>,
}

impl Cache {
    pub fn cache(&self) -> &[u8] {
        self.cache.data()
    }

    pub fn l1(&self) -> Option<&[u8]> {
        if let Some(l1) = &self.l1 {
            Some(l1.data())
        } else {
            None
        }
    }

    pub fn generate(epoch: usize, config: &DagConfig) -> Result<Self> {
        let cache_data_file = DataFile::generate(config, epoch, None)?;
        let l1_data_file = if config.l1_enabled {
            let data_file = DataFile::generate(config, epoch, Some(cache_data_file.data()))?;
            Some(data_file)
        } else {
            None
        };
        Ok(Self {
            cache: cache_data_file,
            l1: l1_data_file,
        })
    }
}
