
error[E0658]: use of unstable library feature 'llvm_asm': LLVM-style inline assembly will never be stabilized, prefer using asm! instead
   --> util_linux/volume_id/sysv.rs:253:17
    |
253 |                 llvm_asm!("bswap $0" : "=r" (fresh4) : "0"
    |                 ^^^^^^^^
    |
    = note: see issue #70173 <https://github.com/rust-lang/rust/issues/70173> for more information
    = help: add `#![feature(llvm_asm)]` to the crate attributes to enable
