rust
% RUST_BACKTRACE=1 mach build
   Compiling script v0.0.1 (file:///home/simon/servo1/components/script)
warning: variable does not need to be mutable
   --> /home/simon/servo1/target/debug/build/script-9370d82a1d46a1f2/out/Bindings/MutationObserverBinding.rs:483:17
    |
483 |             let mut argc = 2;
    |                 ^^^^^^^^
    |
    = note: #[warn(unused_mut)] on by default

error: internal compiler error: unexpected panic
