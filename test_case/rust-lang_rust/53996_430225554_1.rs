
+ cp -a /home/sekineh/rustme12/src/test/run-make/thumb-none-qemu/example .
+ pushd example
+ /home/sekineh/rustme12/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run --target thumbv6m-none-eabi
+ grep 'x = 42'
    Updating registry `https://github.com/rust-lang/crates.io-index`
   Compiling cortex-m v0.5.7
   Compiling cortex-m-rt v0.5.4
   Compiling cortex-m-semihosting v0.3.1
   Compiling vcell v0.1.0
   Compiling bare-metal v0.2.3
   Compiling r0 v0.2.2
   Compiling aligned v0.2.0
   Compiling panic-halt v0.2.0
   Compiling volatile-register v0.2.0
   Compiling example v0.1.0 (file:///home/sekineh/rustme12/build/x86_64-unknown-linux-gnu/test/run-make/thumb-none-qemu/thumb-none-qemu/example)
error: linker `rust-lld` not found
  |
  = note: No such file or directory (os error 2)

