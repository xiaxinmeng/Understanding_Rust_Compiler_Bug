rust
#![feature(cfg_target_thread_local)]
fn main() {
    dbg!(cfg!(target_thread_local));
    dbg!(cfg!(not(all(target_family = "wasm", not(target_feature = "atomics")))));
}
