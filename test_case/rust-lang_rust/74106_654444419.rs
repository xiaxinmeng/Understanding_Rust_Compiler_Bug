
error[E0277]: cannot subtract `core::sync::atomic::AtomicUsize` from `usize`
   --> src/libstd/sys/unix/thread.rs:387:24
    |
387 |         Some(stackaddr - PAGE_SIZE..stackaddr)
    |                        ^ no implementation for `usize - core::sync::atomic::AtomicUsize`
    |
    = help: the trait `core::ops::Sub<core::sync::atomic::AtomicUsize>` is not implemented for `usize`
