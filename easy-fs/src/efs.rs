use alloc::sync::Arc;
use spin::Mutex;

use crate::{bitmap::Bitmap, block_dev::BlockDevice};
pub struct EasyFileSystem {
    pub block_device: Arc<dyn BlockDevice>,
    pub innode_bitmap: Bitmap,
    pub data_bitmap: Bitmap,
    innode_area_start_block: u32,
    data_area_start_block: u32,
}
type DataBlock = [u8; super::BLOCK_SZ];

impl EasyFileSystem {
    pub fn create() -> Arc<Mutex<Self>> {
        todo!()
    }
}
