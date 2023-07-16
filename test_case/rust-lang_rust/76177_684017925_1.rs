bash
$ RUST_BACKTRACE=1 cargo +nightly -Z build-std bolero fuzz --release nodes::behaviors::databases::database::tests::insert_movement_edge_different_source_and_target -t 100sec -s memory
    Finished release [optimized + debuginfo] target(s) in 5m 08s
    Finished release [optimized + debuginfo] target(s) in 0.08s
     Running target/fuzz/build_75c399cd177824ee/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb
Uninitialized bytes in __interceptor_memchr at offset 0 inside [0x701000000000, 4)
==15912==WARNING: MemorySanitizer: use-of-uninitialized-value
    #0 0x55fee4ce1aae  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_75c399cd177824ee/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x4f18aae)
    #1 0x55fee4d04cf0  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_75c399cd177824ee/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x4f3bcf0)
    #2 0x55fee1f52113  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_75c399cd177824ee/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x2189113)
    #3 0x55fee24d5ea5  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_75c399cd177824ee/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x270cea5)
    #4 0x7f4d6dbeab96  (/lib/x86_64-linux-gnu/libc.so.6+0x21b96)
    #5 0x55fee10f5739  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_75c399cd177824ee/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x132c739)

SUMMARY: MemorySanitizer: use-of-uninitialized-value (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_75c399cd177824ee/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x4f18aae) 
Exiting
error: test failed, to rerun pass '--lib'
error: process exited with status 77
