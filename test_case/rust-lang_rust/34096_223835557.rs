
run doc-book-lang-items [x86_64-unknown-linux-gnu]

running 1 test
test _0 ... FAILED

failures:

---- _0 stdout ----
    <anon>:36:9: 36:13 warning: unused variable: `argc`, #[warn(unused_variables)] on by default
<anon>:36 fn main(argc: isize, argv: *const *const u8) -> isize {
                  ^~~~
<anon>:36:22: 36:26 warning: unused variable: `argv`, #[warn(unused_variables)] on by default
<anon>:36 fn main(argc: isize, argv: *const *const u8) -> isize {
                               ^~~~
<anon>:37:9: 37:10 warning: unused variable: `x`, #[warn(unused_variables)] on by default
<anon>:37     let x = box 1;
                  ^
error: linking with `cc` failed: exit code: 1
note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/tmp/rustdoctest.goiITNucvd13/rust_out.0.o" "-o" "/tmp/rustdoctest.goiITNucvd13/rust_out" "-Wl,--gc-sections" "-pie" "-nodefaultlibs" "-L" "/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/liblibc-fe3cdf61.rlib" "/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcore-fe3cdf61.rlib" "-l" "c" "-l" "m" "-l" "rt" "-l" "util" "-l" "compiler-rt"
note: /tmp/rustdoctest.goiITNucvd13/rust_out.0.o: In function `rust_out::main::hec68da2489a63b6a':
rust_out.0.rs:(.text._ZN8rust_out4main17hec68da2489a63b6aE+0x6a): undefined reference to `_Unwind_Resume'
collect2: error: ld returned 1 exit status

error: aborting due to previous error
thread '_0' panicked at 'Box<Any>', src/libsyntax/errors/mod.rs:622
note: Run with `RUST_BACKTRACE=1` for a backtrace.
thread '_0' panicked at 'couldn't compile the test', src/librustdoc/test.rs:281


failures:
    _0

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

/build/mk/tests.mk:865: recipe for target 'tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-doc-book-lang-items.ok' failed
make: *** [tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-doc-book-lang-items.ok] Error 101
