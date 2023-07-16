
Testing alloc stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling getrandom v0.1.14
   Compiling ppv-lite86 v0.2.8
   Compiling rand_core v0.5.1
   Compiling rand_chacha v0.2.2
   Compiling rand_xorshift v0.2.0
   Compiling rand v0.7.3
   Compiling alloc v0.0.0 (/home/joshua/src/rust/rust/library/alloc)
    Finished release [optimized + debuginfo] target(s) in 1m 15s
     Running unittests (build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/alloc-cd5eecf891e8cfac)

running 1 test
F
failures:

---- collections::btree::node::tests::test_debug_assert stdout ----
thread 'collections::btree::node::tests::test_debug_assert' panicked at 'assertion failed: false', library/alloc/src/collections/btree/node/tests.rs:106:5
