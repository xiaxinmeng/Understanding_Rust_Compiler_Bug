
$ ASAN_SYMBOLIZER_PATH=/usr/bin/llvm-symbolizer-12 RUSTFLAGS="-Z sanitizer=address" cargo run --target x86_64-unknown-linux-gnu
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/x86_64-unknown-linux-gnu/debug/sani`
=================================================================
==1516040==ERROR: AddressSanitizer: stack-buffer-overflow on address 0x7fff8edbc530 at pc 0x55aa5aa30c64 bp 0x7fff8edbc470 sp 0x7fff8edbc468
READ of size 4 at 0x7fff8edbc530 thread T0
    #0 0x55aa5aa30c63 in sani::main::h4537ae2e990dccd3 /home/rust/dev/spikes/sani/src/main.rs:3:22
    #1 0x55aa5aa31e2a in core::ops::function::FnOnce::call_once::h44b450c2f5e4ae11 /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/core/src/ops/function.rs:227:5
    #2 0x55aa5aa31ad4 in std::sys_common::backtrace::__rust_begin_short_backtrace::h999109a60c36826c /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/std/src/sys_common/backtrace.rs:123:18
    #3 0x55aa5aa319f3 in std::rt::lang_start::_$u7b$$u7b$closure$u7d$$u7d$::h86afe0d24d76fab5 /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/std/src/rt.rs:146:18
    #4 0x55aa5aa467b0 in core::ops::function::impls::_$LT$impl$u20$core..ops..function..FnOnce$LT$A$GT$$u20$for$u20$$RF$F$GT$::call_once::h6743157f0325d450 /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/core/src/ops/function.rs:259:13
    #5 0x55aa5aa467b0 in std::panicking::try::do_call::hc65378359d322d46 /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/std/src/panicking.rs:403:40
    #6 0x55aa5aa467b0 in std::panicking::try::h52b83ca0140efb28 /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/std/src/panicking.rs:367:19
    #7 0x55aa5aa467b0 in std::panic::catch_unwind::h0ba25f4b0d3448dc /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/std/src/panic.rs:133:14
    #8 0x55aa5aa467b0 in std::rt::lang_start_internal::_$u7b$$u7b$closure$u7d$$u7d$::ha65f28100c5ad390 /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/std/src/rt.rs:128:48
    #9 0x55aa5aa467b0 in std::panicking::try::do_call::h5db5edfaf5b749d9 /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/std/src/panicking.rs:403:40
    #10 0x55aa5aa467b0 in std::panicking::try::h62409771d6cd0419 /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/std/src/panicking.rs:367:19
    #11 0x55aa5aa467b0 in std::panic::catch_unwind::h386261fb8f018fab /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/std/src/panic.rs:133:14
    #12 0x55aa5aa467b0 in std::rt::lang_start_internal::h699f3530566c1833 /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/std/src/rt.rs:128:20
    #13 0x55aa5aa31955 in std::rt::lang_start::hf7817e8d7f09e0f2 /rustc/c1026539bd22e9d070988deaa47b1360cbc76436/library/std/src/rt.rs:145:17
    #14 0x55aa5aa30e7b in main (/home/rust/dev/spikes/sani/target/x86_64-unknown-linux-gnu/debug/sani+0x9be7b)
    #15 0x7faf740310b2 in __libc_start_main /build/glibc-eX1tMB/glibc-2.31/csu/../csu/libc-start.c:308:16
    #16 0x55aa5a9a370d in _start (/home/rust/dev/spikes/sani/target/x86_64-unknown-linux-gnu/debug/sani+0xe70d)

Address 0x7fff8edbc530 is located in stack of thread T0 at offset 176 in frame
    #0 0x55aa5aa3099f in sani::main::h4537ae2e990dccd3 /home/rust/dev/spikes/sani/src/main.rs:1

  This frame has 4 object(s):
    [32, 48) '_15' (line 4)
    [64, 112) '_8' (line 4)
    [144, 148) 'y' (line 3)
    [160, 176) 'xs' (line 2) <== Memory access at offset 176 overflows this variable
HINT: this may be a false positive if your program uses some custom stack unwind mechanism, swapcontext or vfork
      (longjmp and C++ exceptions *are* supported)
SUMMARY: AddressSanitizer: stack-buffer-overflow /home/rust/dev/spikes/sani/src/main.rs:3:22 in sani::main::h4537ae2e990dccd3
Shadow bytes around the buggy address:
  0x100071daf850: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100071daf860: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100071daf870: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100071daf880: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100071daf890: f1 f1 f1 f1 f8 f8 f2 f2 f8 f8 f8 f8 f8 f8 f2 f2
=>0x100071daf8a0: f2 f2 04 f2 00 00[f3]f3 00 00 00 00 00 00 00 00
  0x100071daf8b0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100071daf8c0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100071daf8d0: 00 00 00 00 00 00 00 00 f1 f1 f1 f1 f8 f8 f3 f3
  0x100071daf8e0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
  0x100071daf8f0: 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
Shadow byte legend (one shadow byte represents 8 application bytes):
  Addressable:           00
  Partially addressable: 01 02 03 04 05 06 07
  Heap left redzone:       fa
  Freed heap region:       fd
  Stack left redzone:      f1
  Stack mid redzone:       f2
  Stack right redzone:     f3
  Stack after return:      f5
  Stack use after scope:   f8
  Global redzone:          f9
  Global init order:       f6
  Poisoned by user:        f7
  Container overflow:      fc
  Array cookie:            ac
  Intra object redzone:    bb
  ASan internal:           fe
  Left alloca redzone:     ca
  Right alloca redzone:    cb
==1516040==ABORTING
