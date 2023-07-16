 rust
#[start]
fn main(_: int, _: *const *const u8) -> int {
    for _ in range(0, 10000u) {
        // allocate some bytes and immediatelly free them
        // box [0u8, ..128];
        // smaller allocations allow valgrind to catch reads of uninitialized memory
        box 0u8;
    }
    0
}
