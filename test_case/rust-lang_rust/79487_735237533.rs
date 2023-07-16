rust
mod sha2 {
    extern "C" {
        pub(super) fn GFp_sha512_block_data_order();
    }
}
