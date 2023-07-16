
$ ./x.py test --stage 0 --no-doc src/libcore
Updating only changed submodules
Submodules updated in 0.26 seconds
    Finished dev [unoptimized] target(s) in 0.30s
Building stage0 std artifacts (x86_64-pc-windows-gnu -> x86_64-pc-windows-gnu)
   Compiling libc v0.2.54
   Compiling backtrace-sys v0.1.27
   Compiling unwind v0.0.0 (C:\Users\Allen\Documents\GitHub\rust\src\libunwind)
   Compiling panic_abort v0.0.0 (C:\Users\Allen\Documents\GitHub\rust\src\libpanic_abort)
   Compiling backtrace v0.3.29
   Compiling panic_unwind v0.0.0 (C:\Users\Allen\Documents\GitHub\rust\src\libpanic_unwind)
   Compiling std v0.0.0 (C:\Users\Allen\Documents\GitHub\rust\src\libstd)
    Finished release [optimized] target(s) in 17.49s
Copying stage0 std from stage0 (x86_64-pc-windows-gnu -> x86_64-pc-windows-gnu / x86_64-pc-windows-gnu)
Building stage0 test artifacts (x86_64-pc-windows-gnu -> x86_64-pc-windows-gnu)
   Compiling unicode-width v0.1.5
   Compiling term v0.0.0 (C:\Users\Allen\Documents\GitHub\rust\src\libterm)
   Compiling proc_macro v0.0.0 (C:\Users\Allen\Documents\GitHub\rust\src\libproc_macro)
   Compiling getopts v0.2.19
   Compiling test v0.0.0 (C:\Users\Allen\Documents\GitHub\rust\src\libtest)
    Finished release [optimized] target(s) in 20.71s
Copying stage0 test from stage0 (x86_64-pc-windows-gnu -> x86_64-pc-windows-gnu / x86_64-pc-windows-gnu)
Testing core stage0 (x86_64-pc-windows-gnu -> x86_64-pc-windows-gnu)
   Compiling rand v0.6.1
error[E0460]: found possibly newer version of crate `std` which `rand_core` depends on
  --> C:\Users\Allen\.cargo\registry\src\github.com-1ecc6299db9ec823\rand-0.6.1\src\lib.rs:75:1
   |
75 | extern crate rand_core;
   | ^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: perhaps that crate needs to be recompiled?
   = note: the following crate versions were found:
           crate `std`: \\?\C:\Users\Allen\Documents\GitHub\rust\build\x86_64-pc-windows-gnu\stage0-sysroot\lib\rustlib\x86_64-pc-windows-gnu\lib\libstd-62f505b9307d737b.rlib
           crate `std`: \\?\C:\Users\Allen\Documents\GitHub\rust\build\x86_64-pc-windows-gnu\stage0-std\x86_64-pc-windows-gnu\release\deps\libstd-62f505b9307d737b.rlib
           crate `std`: \\?\C:\Users\Allen\Documents\GitHub\rust\build\x86_64-pc-windows-gnu\stage0-sysroot\lib\rustlib\x86_64-pc-windows-gnu\lib\std-62f505b9307d737b.dll
           crate `std`: \\?\C:\Users\Allen\Documents\GitHub\rust\build\x86_64-pc-windows-gnu\stage0-std\x86_64-pc-windows-gnu\release\deps\std-62f505b9307d737b.dll
           crate `rand_core`: \\?\C:\Users\Allen\Documents\GitHub\rust\build\x86_64-pc-windows-gnu\stage0-std\x86_64-pc-windows-gnu\release\deps\librand_core-6c85245e34681cba.rlib
