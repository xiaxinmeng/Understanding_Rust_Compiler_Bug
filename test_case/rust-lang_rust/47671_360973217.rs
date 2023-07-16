
[01:38:24]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[01:41:32] error: linking with `cc` failed: exit code: 1
[01:41:32]   |
<snip>
[01:41:32]   = note: /rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/../../../../x86_64-unknown-linux-gnu/bin/ld: skipping incompatible /rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/../../../../lib64/libgcc_s.so when searching for -lgcc_s
[01:41:32]           /rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/../../../../x86_64-unknown-linux-gnu/bin/ld: i386:x86-64 architecture of input file `/tmp/rustc.CNu40ABKRiEL/librustc_llvm-9ea7cfc2f58bb7a4.rlib(compatibility.o)' is incompatible with i386 output
[01:41:32]           /rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/../../../../x86_64-unknown-linux-gnu/bin/ld: i386:x86-64 architecture of input file `/tmp/rustc.CNu40ABKRiEL/librustc_llvm-9ea7cfc2f58bb7a4.rlib(compatibility-debug_list.o)' is incompatible with i386 output
...
<snip>
...
[01:41:32]           /rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/../../../../x86_64-unknown-linux-gnu/bin/ld: i386:x86-64 architecture of input file `/tmp/rustc.CNu40ABKRiEL/librustc_llvm-9ea7cfc2f58bb7a4.rlib(thread.o)' is incompatible with i386 output
[01:41:32]           /rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/../../../../x86_64-unknown-linux-gnu/bin/ld: i386:x86-64 architecture of input file `/tmp/rustc.CNu40ABKRiEL/librustc_llvm-9ea7cfc2f58bb7a4.rlib(regex.o)' is incompatible with i386 output
[01:41:32]           collect2: error: ld returned 1 exit status
[01:41:32]           
[01:41:32] 
[01:41:32] error: aborting due to previous error
[01:41:32] 
[01:41:33] error: Could not compile `rustc_trans`.
