#![no_std]
extern crate alloc;
mod block_cache;
mod block_dev;
mod vfs;

pub const BLOCK_SZ: usize = 512;