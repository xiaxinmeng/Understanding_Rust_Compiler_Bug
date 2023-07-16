bash
 RUST_BACKTRACE=1 cargo +nightly -Z build-std bolero fuzz --release nodes::behaviors::databases::database::tests::insert_movement_edge_different_source_and_target -t 100sec -s address

running 1 test
INFO: Seed: 3654582759
INFO: Loaded 1 modules   (804422 inline 8-bit counters): 804422 [0x55fbc6857338, 0x55fbc691b97e), 
INFO: Loaded 1 PC tables (804422 PCs): 804422 [0x55fbc691b980,0x55fbc7561de0), 
INFO:     1658 files found in /home/cfkaran2/Documents/repositories/bit_network/bit_network_library/src/nodes/behaviors/databases/__fuzz__/nodes__behaviors__databases__database__tests__insert_movement_edge_different_source_and_target/corpus
INFO: -max_len is not provided; libFuzzer will not generate inputs larger than 8192 bytes
INFO: seed corpus: files: 1658 min: 1b max: 8192b total: 560700b rss: 91Mb
F
failures:

failures:
    nodes::behaviors::databases::database::tests::insert_movement_edge_different_source_and_target

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 468 filtered out

==21293== ERROR: libFuzzer: fuzz target exited
    #0 0x55fbc145d391  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x15c8391)
    #1 0x55fbc4bd6646  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x4d41646)
    #2 0x55fbc4bb676b  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x4d2176b)
    #3 0x7f65bcb4c0f0  (/lib/x86_64-linux-gnu/libc.so.6+0x430f0)
    #4 0x7f65bcb4c1e9  (/lib/x86_64-linux-gnu/libc.so.6+0x431e9)
    #5 0x55fbc57460b6  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x58b10b6)
    #6 0x55fbc573477e  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x589f77e)
    #7 0x55fbc3e36f70  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x3fa1f70)
    #8 0x55fbc3e38488  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x3fa3488)
    #9 0x55fbc2b8d07f  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x2cf807f)
    #10 0x55fbc24fd842  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x2668842)
    #11 0x55fbc573bf50  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x58a6f50)
    #12 0x55fbc24fd7b8  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x26687b8)
    #13 0x7f65bcb2ab96  (/lib/x86_64-linux-gnu/libc.so.6+0x21b96)
    #14 0x55fbc13da099  (/home/cfkaran2/Documents/repositories/bit_network/target/fuzz/build_859dae3fae0b1004/x86_64-unknown-linux-gnu/release/deps/bit_network_library-4f9b998d20e451bb+0x1545099)

SUMMARY: libFuzzer: fuzz target exited
MS: 0 ; base unit: 0000000000000000000000000000000000000000
artifact_prefix='/home/cfkaran2/Documents/repositories/bit_network/bit_network_library/src/nodes/behaviors/databases/__fuzz__/nodes__behaviors__databases__database__tests__insert_movement_edge_different_source_and_target/crashes/'; Test unit written to /home/cfkaran2/Documents/repositories/bit_network/bit_network_library/src/nodes/behaviors/databases/__fuzz__/nodes__behaviors__databases__database__tests__insert_movement_edge_different_source_and_target/crashes/crash-8ef6b619b4577a97c20fc0b06f52c8287f21c4b1
error: process exited with status 77
