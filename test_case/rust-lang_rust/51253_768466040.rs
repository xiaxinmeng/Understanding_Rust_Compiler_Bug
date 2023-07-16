
thread 'main' panicked at 'attempt to add with overflow', src/main.rs:5:13
stack backtrace:
   0: rust_begin_unwind
(...)
  19:     0x55f9928ac3c1 - std::rt::lang_start::{{closure}}::hcc222df3ba29fe0f
                               at /home/toto/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:66:18
  20:     0x55f9928c1dc7 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::h57e2a071d427b24c
                               at /rustc/e1884a8e3c3e813aada8254edfa120e85bf5ffca/library/core/src/ops/function.rs:259:13
  21:     0x55f9928c1dc7 - std::panicking::try::do_call::h81cbbe0c3b30a28e
                               at /rustc/e1884a8e3c3e813aada8254edfa120e85bf5ffca/library/std/src/panicking.rs:381:40
  22:     0x55f9928c1dc7 - std::panicking::try::hbeeb95b4e1f0a876
                               at /rustc/e1884a8e3c3e813aada8254edfa120e85bf5ffca/library/std/src/panicking.rs:345:19
  23:     0x55f9928c1dc7 - std::panic::catch_unwind::h59c48ccb40a0bf20
                               at /rustc/e1884a8e3c3e813aada8254edfa120e85bf5ffca/library/std/src/panic.rs:396:14
  24:     0x55f9928c1dc7 - std::rt::lang_start_internal::ha53ab63f88fee728
                               at /rustc/e1884a8e3c3e813aada8254edfa120e85bf5ffca/library/std/src/rt.rs:51:25
  25:     0x55f9928ac397 - std::rt::lang_start::h6ffeed854908e5f1
                               at /home/toto/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:65:5
  26:     0x55f9928ac34a - main
  27:     0x7f0e96185152 - __libc_start_main
  28:     0x55f9928ac08e - _start
  29:                0x0 - <unknown>
