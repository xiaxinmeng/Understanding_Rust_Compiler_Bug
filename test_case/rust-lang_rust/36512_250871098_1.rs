
fenrir@DGS-Z87-HD3:~/projects/rustc-builtins$ xargo build --target 3ds --features c
   Compiling gcc v0.3.35
   Compiling rustc-cfg v0.1.2
   Compiling rustc_builtins v0.1.0 (file:///home/fenrir/projects/rustc-builtins)
warning: compiler-rt/compiler-rt-cdylib/compiler-rt/lib/builtins/arm/comparesf2.S: Assembler messages:
warning: compiler-rt/compiler-rt-cdylib/compiler-rt/lib/builtins/arm/comparesf2.S:60: Warning: IT blocks containing 32-bit Thumb instructions are deprecated in ARMv8
warning: compiler-rt/compiler-rt-cdylib/compiler-rt/lib/builtins/arm/switch16.S: Assembler messages:
warning: compiler-rt/compiler-rt-cdylib/compiler-rt/lib/builtins/arm/switch16.S:39: Warning: IT blocks containing 32-bit Thumb instructions are deprecated in ARMv8
warning: compiler-rt/compiler-rt-cdylib/compiler-rt/lib/builtins/arm/switch32.S: Assembler messages:
warning: compiler-rt/compiler-rt-cdylib/compiler-rt/lib/builtins/arm/switch32.S:39: Warning: IT blocks containing 32-bit Thumb instructions are deprecated in ARMv8
warning: compiler-rt/compiler-rt-cdylib/compiler-rt/lib/builtins/arm/switch8.S: Assembler messages:
warning: compiler-rt/compiler-rt-cdylib/compiler-rt/lib/builtins/arm/switch8.S:37: Warning: IT blocks containing 32-bit Thumb instructions are deprecated in ARMv8
warning: compiler-rt/compiler-rt-cdylib/compiler-rt/lib/builtins/arm/switchu8.S: Assembler messages:
warning: compiler-rt/compiler-rt-cdylib/compiler-rt/lib/builtins/arm/switchu8.S:37: Warning: IT blocks containing 32-bit Thumb instructions are deprecated in ARMv8
warning: ar: `u' modifier ignored since `D' is the default (see `U')
warning: unnecessary `unsafe` block, #[warn(unused_unsafe)] on by default
  --> src/int/sdiv.rs:44:21
   |
44 |             let r = unsafe { $div(a, b) };
   |                     ^^^^^^^^^^^^^^^^^^^^^
...
66 | divmod!(__divmoddi4, __divdi3: i64);
   | ------------------------------------ in this macro invocation

error[E0463]: can't find crate for `libc`
  --> src/bin/intrinsics.rs:16:1
   |
16 | extern crate libc;
   | ^^^^^^^^^^^^^^^^^^ can't find crate

error: aborting due to previous error

error: Could not compile `rustc_builtins`.

To learn more, run the command again with --verbose.
error: `cargo` process didn't exit successfully
fenrir@DGS-Z87-HD3:~/projects/rustc-builtins$ 
