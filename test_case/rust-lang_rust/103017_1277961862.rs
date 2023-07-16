rust
// library/std/src/sys/sgx/thread_local_key.rs
#[inline(always)]
pub fn destroy_on_drop() -> bool {
    false
}
