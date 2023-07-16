
[02:12:44] failures:
[02:12:44] 
[02:12:44] ---- /Users/travis/build/rust-lang/rust/src/doc/book/first-edition/src/loops.md - Loops::Loop_labels (line 221) stdout ----
[02:12:44] 	error: linking with `cc` failed: exit code: 254
[02:12:44]   |
[02:12:44]   = note: "cc" "-m64" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out0.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out1.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out2.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out3.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out4.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out5.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out6.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out7.rcgu.o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.rust_out8.rcgu.o" "-o" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out" "/var/folders/bb/n7t3rs157850byt_jfdcq9k80000gn/T/rustdoctest.GRUUWeFHsG46/rust_out.crate.allocator.rcgu.o" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib" "-l" "std-369c1f8e5b16bb0f" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/lib/rustlib/x86_64-apple-darwin/lib/libcompiler_builtins-21e0a7e843257473.rlib" "-l" "System" "-l" "resolv" "-l" "pthread" "-l" "c" "-l" "m"
[02:12:44]   = note: clang: error: unable to execute command: Bus error: 10
[02:12:44]           clang: error: linker command failed due to signal (use -v to see invocation)
[02:12:44]           
[02:12:44] 
[02:12:44] error: aborting due to previous error
[02:12:44] 
[02:12:44] thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:518:8
[02:12:44] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[02:12:44] thread 'rustc' panicked at 'couldn't compile the test', src/librustdoc/test.rs:288:12
[02:12:44] 
[02:12:44] 
[02:12:44] failures:
[02:12:44]     /Users/travis/build/rust-lang/rust/src/doc/book/first-edition/src/loops.md - Loops::Loop_labels (line 221)
[02:12:44] 
[02:12:44] test result: FAILED. 7 passed; 1 failed; 4 ignored; 0 measured; 0 filtered out
