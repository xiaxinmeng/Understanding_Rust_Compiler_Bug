
 LD_LIBRARY_PATH=/home/william/development/rust/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-dylib-link.stage2-x86_64-unknown-linux-gnu /home/william/development/rust/build/x86_64-unknown-linux-gnu/test/run-make/sanitizer-dylib-link.stage2-x86_64-unknown-linux-gnu/program
=================================================================
==14626==ERROR: AddressSanitizer: stack-buffer-overflow on address 0x7ffd83a1c970 at pc 0x7f32a1ca64d0 bp 0x7ffd83a1c930 sp 0x7ffd83a1c928
READ of size 4 at 0x7ffd83a1c970 thread T0
    #0 0x7f32a1ca64cf in overflow /home/william/development/rust/src/test/run-make/sanitizer-dylib-link/library.rs:14
    #1 0x7f32a23187e8 in program::main::h06fb8338f7689bca /home/william/development/rust/src/test/run-make/sanitizer-dylib-link/program.rs:16
    #2 0x7f32a1ce034e in core::ops::function::FnOnce::call_once::h77fe44d6fc2de6f5 /home/william/development/rust/src/libcore/ops/function.rs:143

...
