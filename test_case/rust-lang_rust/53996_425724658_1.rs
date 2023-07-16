
+ git clone https://github.com/rust-embedded/cortex-m-rt cortex-m-rt
Cloning into 'cortex-m-rt'...
+ cd cortex-m-rt
+ git reset --hard 62972c8a89ff54b76f9ef0d600c1fcf7a233aabd
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling proc-macro2 v0.4.19
   Compiling unicode-xid v0.1.0
   Compiling rand_core v0.2.1
   Compiling cortex-m-semihosting v0.3.1
   Compiling vcell v0.1.0
   Compiling cortex-m-rt v0.6.4 (file:///home/sekineh/rustme12/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/cortex-m-rt)
   Compiling cortex-m v0.5.7
   Compiling bare-metal v0.2.3
   Compiling r0 v0.2.2
   Compiling aligned v0.2.0
   Compiling panic-halt v0.2.0
   Compiling volatile-register v0.2.0
   Compiling rand v0.5.5
error[E0463]: can't find crate for `proc_macro`
  --> /home/sekineh/.cargo/registry/src/github.com-1ecc6299db9ec823/proc-macro2-0.4.19/src/lib.rs:53:1
   |
53 | extern crate proc_macro;
   | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.
error: Could not compile `proc-macro2`.
warning: build failed, waiting for other jobs to finish...
error: build failed
make: *** [all] Error 1

------------------------------------------

thread '[run-make] run-make/thumb-none-qemu' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3267:9
note: Run with `RUST_BACKTRACE=1` for a backtrace.
