plain
   Compiling stable_deref_trait v1.0.0
   Compiling byteorder v1.2.7
   Compiling smallbitvec v2.3.0
   Compiling cfg-if v0.1.6
error[E0658]: use of unstable library feature 'core_intrinsics': intrinsics are unlikely to ever be stabilized, instead they should be used through stabilized interfaces in the rest of the standard library
   |
22 | use core::intrinsics::transmute;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
---
warning: `servo_arc` (lib) generated 1 warning
thread 'main' panicked at 'tests failed for https://github.com/servo/servo', src/tools/cargotest/main.rs:101:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Build completed unsuccessfully in 0:23:43
make: *** [check-aux] Error 1
Makefile:44: recipe for target 'check-aux' failed
