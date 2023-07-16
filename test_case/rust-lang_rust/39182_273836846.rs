rust
#[repr(C)]
#[derive(Clone,Debug)]
pub struct MetaData {
    key_offset: u64,
    key_len: u32,
    data_offset: u64,
    data_len: u32,
    key0: u64,
    key1: u64,
    unix_time: u64,
    hits: u64,
    ns_time: u32,
    timeout: u32
}
assert_eq!(72, size_of::<MetaData>());
