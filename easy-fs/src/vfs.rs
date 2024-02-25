use alloc::sync::Arc;
use spin::Mutex;

use crate::{block_dev::BlockDevice, efs::EasyFileSystem};
///服务于文件相关系统调用的索引节点层的代码
pub struct Inode {
    block_id: usize,
    block_offset: usize,
    fs: Arc<Mutex<EasyFileSystem>>,
    block_device: Arc<dyn BlockDevice>,
}
