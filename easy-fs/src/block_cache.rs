use alloc::{collections::VecDeque, sync::Arc};
use lazy_static::*;
use spin::Mutex;

use crate::{block_dev::BlockDevice, BLOCK_SZ};

pub struct BlockCache {
    cache: [u8; BLOCK_SZ],
    block_id: usize,
    block_device: Arc<dyn BlockDevice>,
    modified: bool,
}

impl BlockCache {
    pub fn new() -> Self {
        todo!()
    }

    pub fn addr_of_offset() -> usize {
        todo!()
    }

    pub fn get_ref<T>(&self) -> &T {
        todo!()
    }

    pub fn get_mut<T>(&self) -> &mut T {
        todo!()
    }

    pub fn read(&self) {
        todo!()
    }

    pub fn modify(&self) {
        todo!()
    }

    pub fn sync(&mut self) {}
}

impl Drop for BlockCache {
    fn drop(&mut self) {
        todo!()
    }
}

pub struct BlockCacheManager {
    queue: VecDeque<(usize, Arc<Mutex<BlockCache>>)>,
}

impl BlockCacheManager {
    pub fn new() -> Self {
        todo!()
    }

    pub fn get_block_cache() {}
}

lazy_static! {
    pub static ref BLOCK_CACHE_MANAGER: Mutex<BlockCacheManager> =
        Mutex::new(BlockCacheManager::new());
}

pub fn get_block_cache() {

}

/// Sync all block cache to block device
/// write_to_disk
pub fn block_cache_sync_all() {}