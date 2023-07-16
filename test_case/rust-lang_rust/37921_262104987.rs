rust
let mut ptr: *const u8;
let end_block; // A whole number of blocks from ptr
let real_end;
while ptr != end_block {
    ptr = ptr.offset(block_size);
}
while ptr != real_end {
    ptr = ptr.offset(1);
}
