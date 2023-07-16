
error: linking with `rust-lld` failed: exit code: 1
  |
  = note: rust-lld: error: undefined symbol: rust_oom
          >>> referenced by alloc.rs:224 (liballoc/alloc.rs:224)
          >>>               alloc-c69b28e77453f7a5.alloc.4j3hju2s-cgu.11.rcgu.o:(.text._ZN5alloc5alloc18handle_alloc_error17hcb467c6fec4cebfdE+0x0) in archive /home/philipp/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/thumbv7em-none-eabihf/lib/liballoc-c69b28e77453f7a5.rlib

          
          rust-lld: error: undefined symbol: rust_begin_unwind
          >>> referenced by panicking.rs:77 (libcore/panicking.rs:77)
          >>>               core-59aaa68b41c4baf4.core.82zizo12-cgu.9.rcgu.o:(core::panicking::panic_fmt::h380215f9f52a35ae) in archive /home/philipp/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/thumbv7em-none-eabihf/lib/libcore-59aaa68b41c4baf4.rlib
