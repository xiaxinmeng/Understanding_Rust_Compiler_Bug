 rust
extern "C" {
    fn snappy_uncompressed_length(compressed: *const u8,
                                  compressed_length: size_t,
                                  result: Option<&mut size_t>) -> c_int;
}

let compressed: Vec<u8> = get_compressed_data();
let mut uncompressed_length = 0;
snappy_uncompressed_length(compressed.as_ptr(),
                           compressed.len(),
                           Some(&mut uncompressed_length));
