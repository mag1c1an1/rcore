const INODE_DIRECT_COUNT: usize = 28;
const INODE_INDIRECT1_COUNT: usize = super::BLOCK_SZ / 4;
const NAME_LENGTH_LIMIT: usize = 27;

#[repr(C)]
pub struct SuperBlock {
    magic: u32,
    pub totoal_blocks: u32,
    pub inode_bitmap_blocks: u32,
    pub inode_area_blocks: u32,
    pub data_bitmap_blocks: u32,
    pub data_area_blocks: u32,
}

#[repr(C)]
pub struct DiskInode {
    pub size: u32,
    pub direct: [u32; INODE_DIRECT_COUNT],
    pub indirect1: u32,
    pub indirect2: u32,
    type_: DiskInodeType,
}

#[derive(PartialEq)]
pub enum DiskInodeType {
    File,
    Directory,
}

#[repr(C)]
pub struct DirEntry {
    name: [u8; NAME_LENGTH_LIMIT + 1],
    inode_number: u32,
}

pub const DIRENT_SZ: usize = 27 + 1 + 4;




