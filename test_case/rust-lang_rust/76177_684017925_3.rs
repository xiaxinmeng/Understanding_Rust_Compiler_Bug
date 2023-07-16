bash
RUST_BACKTRACE=1 cargo +nightly -Z build-std bolero fuzz --release nodes::behaviors::databases::database::tests::insert_movement_edge_different_source_and_target -t 100sec -s leak
running 1 test
INFO: Seed: 3607781050
INFO: Loaded 1 modules   (802931 inline 8-bit counters): 802931 [0x562a2cec1288, 0x562a2cf852fb), 
INFO: Loaded 1 PC tables (802931 PCs): 802931 [0x562a2cf85300,0x562a2dbc5a30), 
INFO:     1658 files found in /home/cfkaran2/Documents/repositories/bit_network/bit_network_library/src/nodes/behaviors/databases/__fuzz__/nodes__behaviors__databases__database__tests__insert_movement_edge_different_source_and_target/corpus
INFO: -max_len is not provided; libFuzzer will not generate inputs larger than 8192 bytes
INFO: seed corpus: files: 1658 min: 1b max: 8192b total: 560700b rss: 66Mb
F
failures:

failures:
    nodes::behaviors::databases::database::tests::insert_movement_edge_different_source_and_target

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 468 filtered out

==26446== ERROR: libFuzzer: fuzz target exited
    #0 0x562a2a7d2601  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x1334601)
    #1 0x562a2be089a6  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x296a9a6)
    #2 0x562a2bde8acb  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x294aacb)
    #3 0x7f71bf1500f0  (/lib/x86_64-linux-gnu/libc.so.6+0x430f0)
    #4 0x7f71bf1501e9  (/lib/x86_64-linux-gnu/libc.so.6+0x431e9)
    #5 0x562a2c321556  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x2e83556)
    #6 0x562a2c30fc1e  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x2e71c1e)
    #7 0x562a2b7a30e0  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x23050e0)
    #8 0x562a2b7a45f8  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x23065f8)
    #9 0x562a2b0b5c5f  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x1c17c5f)
    #10 0x562a2ade5bc2  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x1947bc2)
    #11 0x562a2c3173f0  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x2e793f0)
    #12 0x562a2ade5b99  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x1947b99)
    #13 0x7f71bf12eb96  (/lib/x86_64-linux-gnu/libc.so.6+0x21b96)
    #14 0x562a2a7d2279  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_67f8130972a5c98c/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x1334279)

SUMMARY: libFuzzer: fuzz target exited
MS: 0 ; base unit: 0000000000000000000000000000000000000000
artifact_prefix='/home/cfkaran2/Documents/repositories/bit_network/bit_network_library/src/nodes/behaviors/databases/__fuzz__/nodes__behaviors__databases__database__tests__insert_movement_edge_different_source_and_target/crashes/'; Test unit written to /home/cfkaran2/Documents/repositories/bit_network/bit_network_library/src/nodes/behaviors/databases/__fuzz__/nodes__behaviors__databases__database__tests__insert_movement_edge_different_source_and_target/crashes/crash-8ef6b619b4577a97c20fc0b06f52c8287f21c4b1
error: process exited with status 77