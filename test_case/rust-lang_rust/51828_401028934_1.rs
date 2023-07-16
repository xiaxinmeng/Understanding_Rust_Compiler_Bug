rust
#[thread_local]
static mut ZERO_ARRAY: [u32; 250_000] = [0; 250_000];
// adds 1 MB to the compiled output unless placed into __DATA,__thread_bss
