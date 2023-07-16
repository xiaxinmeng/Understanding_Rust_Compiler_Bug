 bash
$ time RUST_BACKTRACE= /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage2/bin/rustc -VV
error: Option 'version' given more than once.
Segmentation fault (core dumped)

real    0m15.874s
user    0m13.973s
sys 0m1.537s

$ time LD_LIBRARY_PATH=/home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage2/lib coredump_to_backtrace   /home/zazdxscf/build/1nonpkgs/rust/rust/x86_64-unknown-linux-gnu/stage2/bin/rustc ./core all >~/logs/rustc2_crash_backtrace_full.log 2>&1 

real    0m26.339s
user    0m24.480s
sys 0m1.377s
