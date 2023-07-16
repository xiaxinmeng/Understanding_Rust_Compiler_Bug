
          ld.lld: error: undefined symbol: _Unwind_Resume
          >>> referenced by string.rs:579 (src/liballoc/string.rs:579)
          >>>               alloc-6bcd594557d0587e.alloc.6cu7tpd8-cgu.0.rcgu.o:(alloc::string::String::from_utf8_lossy::h9a7aba1aef1d966f) in archive /home/harald/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-6bcd594557d0587e.rlib
          
          ld.lld: error: undefined symbol: rust_eh_personality
          >>> referenced by alloc.6cu7tpd8-cgu.0
          >>>               alloc-6bcd594557d0587e.alloc.6cu7tpd8-cgu.0.rcgu.o:(DW.ref.rust_eh_personality) in archive /home/harald/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-musl/lib/liballoc-6bcd594557d0587e.rlib
