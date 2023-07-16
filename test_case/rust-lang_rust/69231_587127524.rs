rust
Breakpoint 3, rust_begin_unwind (_info=0x2000feb8)
    at /home/mabez/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-halt-0.2.0/src/lib.rs:32
32          loop {
(gdb) bt
#0  rust_begin_unwind (_info=0x2000feb8)
    at /home/mabez/.cargo/registry/src/github.com-1ecc6299db9ec823/panic-halt-0.2.0/src/lib.rs:32
#1  0x080008de in core::panicking::panic_fmt (fmt=..., 
    location=0x8003a40 <.Lanon.b0bb6e8da484f667ddcf6a6acf4928ab.9>)
    at /home/mabez/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/panicking.rs:85
#2  0x08002972 in core::slice::slice_index_len_fail (index=1024, len=1)
    at /home/mabez/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/slice/mod.rs:2674
#3  0x08000800 in <core::ops::range::Range<usize> as core::slice::SliceIndex<[T]>>::index (self=..., slice=...)
    at /home/mabez/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/slice/mod.rs:2838
#4  0x08000830 in <core::ops::range::RangeTo<usize> as core::slice::SliceIndex<[T]>>::index (self=..., slice=...)
    at /home/mabez/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/slice/mod.rs:2880
#5  0x08000516 in core::slice::<impl core::ops::index::Index<I> for [T]>::index (
    self=..., index=...)
    at /home/mabez/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/src/libcore/slice/mod.rs:2656
#6  0x0800043c in backtrace::__cortex_m_rt_main () at src/main.rs:19
