console
     Running `target/x86_64-unknown-linux-gnu/debug/use-after-move`
Wrapper("A")
=================================================================
==184053==ERROR: AddressSanitizer: stack-use-after-scope on address 0x7ffe619d05c0 at pc 0x563fd541589f bp 0x7ffe619ce700 sp 0x7ffe619ce6f8
READ of size 8 at 0x7ffe619d05c0 thread T0
    #0 0x563fd541589e in <&str as core::fmt::Debug>::fmt library/core/src/fmt/mod.rs:2033:71
    ...
    #13 0x563fd4f65f9d in use_after_move::main src/main.rs:9:5
    ...

Address 0x7ffe619d05c0 is located in stack of thread T0 at offset 384 in frame
    #0 0x563fd4f65a0f in use_after_move::main src/main.rs:4

  This frame has 9 object(s):
    [32, 40) '_34' (line 9)
    [64, 72) '_32' (line 9)
    [96, 112) '_31' (line 9)
    [128, 176) '_24' (line 9)
    [208, 216) '_16' (line 7)
    [240, 248) '_14' (line 7)
    [272, 288) '_13' (line 7)
    [304, 352) '_6' (line 7)
    [384, 400) 'w' (line 5) <== Memory access at offset 384 is inside this variable
HINT: this may be a false positive if your program uses some custom stack unwind mechanism, swapcontext or vfork
      (longjmp and C++ exceptions *are* supported)
SUMMARY: AddressSanitizer: stack-use-after-scope library/core/src/fmt/mod.rs:2033:71 in <&str as core::fmt::Debug>::fmt
