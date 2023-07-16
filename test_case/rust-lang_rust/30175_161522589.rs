
CFG_LLVM_LINKAGE_FILE=/home/builder/build/openbsd-master/build/x86_64-unknown-openbsd/rt/llvmdeps.rs LD_LIBRARY_PATH=/home/builder/build/openbsd-master/build/x86_64-unknown-openbsd/stage0/lib:$LD_LIBRARY_PATH   x86_64-unknown-openbsd/stage0/bin/rustc --cfg stage0 -Z print-link-args  -O --cfg rtopt -C prefer-dynamic -C no-stack-check --target=x86_64-unknown-openbsd  -W warnings -L "x86_64-unknown-openbsd/rt" -L native="/opt/llvm-cde1ed3196ba9b39bcf028e06e08a8722113a5cb/lib"     -L "/usr/local/lib/gcc/x86_64-unknown-openbsd5.8/4.9.3/../../../" --out-dir x86_64-unknown-openbsd/stage0/lib/rustlib/x86_64-unknown-openbsd/lib -C extra-filename=-71b07a99 ../src/libstd/lib.rs
../src/libstd/sys/unix/fs.rs:220:13: 220:34 error: failed to resolve. Use of undeclared type or module `slice` [E0433]
../src/libstd/sys/unix/fs.rs:220             slice::from_raw_parts(self.entry.d_name.as_ptr() as *const u8,
                                             ^~~~~~~~~~~~~~~~~~~~~
../src/libstd/sys/unix/fs.rs:220:13: 220:34 help: run `rustc --explain E0433` to see a detailed explanation
../src/libstd/sys/unix/fs.rs:220:13: 220:34 error: unresolved name `slice::from_raw_parts` [E0425]
../src/libstd/sys/unix/fs.rs:220             slice::from_raw_parts(self.entry.d_name.as_ptr() as *const u8,
                                             ^~~~~~~~~~~~~~~~~~~~~
../src/libstd/sys/unix/fs.rs:220:13: 220:34 help: run `rustc --explain E0425` to see a detailed explanation
../src/libstd/sys/unix/os.rs:207:24: 207:44 error: unresolved name `libc::KERN_PROC_ARGS` [E0425]
../src/libstd/sys/unix/os.rs:207                        libc::KERN_PROC_ARGS,
                                                        ^~~~~~~~~~~~~~~~~~~~
../src/libstd/sys/unix/os.rs:207:24: 207:44 help: run `rustc --explain E0425` to see a detailed explanation
../src/libstd/sys/unix/os.rs:209:24: 209:44 error: unresolved name `libc::KERN_PROC_ARGV` [E0425]
../src/libstd/sys/unix/os.rs:209                        libc::KERN_PROC_ARGV];
                                                        ^~~~~~~~~~~~~~~~~~~~
