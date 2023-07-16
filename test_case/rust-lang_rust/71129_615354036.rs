
error: this operation will panic at runtime
   --> src/libstd/sys/unix/thread.rs:322:25
    |
322 |         let remainder = (stackaddr as usize) % PAGE_SIZE;
    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ attempt to calculate the remainder with a divisor of zero
    |
    = note: `#[deny(unconditional_panic)]` on by default
