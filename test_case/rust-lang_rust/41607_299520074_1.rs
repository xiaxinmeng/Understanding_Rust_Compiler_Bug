
/c/try$ cargo clean
/c/try$ cargo +nightly-2017-04-07-gnu test
   Compiling try v0.1.0 (file:///C:/try)
    Finished dev [unoptimized + debuginfo] target(s) in 1.44 secs
     Running target\debug\deps\try-ebd40fb5c9184741.exe

running 1 test
test tests::t ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured

   Doc-tests try

running 1 test
test src\lib.rs -  (line 3) ... FAILED

failures:

---- src\lib.rs -  (line 3) stdout ----
	error: linking with `gcc` failed: exit code: 1
  |
  = note: "gcc" "-Wl,--enable-long-section-names" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-nostdlib" "-m64" "C:\\Users\\Trevor\\.rustup\\toolchains\\nightly-2017-04-07-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\crt2.o" "C:\\Users\\Trevor\\.rustup\\toolchains\\nightly-2017-04-07-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsbegin.o" "-L" "C:\\Users\\Trevor\\.rustup\\toolchains\\nightly-2017-04-07-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "C:\\Users\\Trevor\\AppData\\Local\\Temp\\rustdoctest.Ll2ouv1FREn9\\rust_out.0.o" "-o" "C:\\Users\\Trevor\\AppData\\Local\\Temp\\rustdoctest.Ll2ouv1FREn9\\rust_out.exe" "-Wl,--gc-sections" "-nodefaultlibs" "-L" "C:\\try\\target\\debug\\deps" "-L" "C:\\try\\target\\debug\\build\\try-6cfbfcbf6205bd80\\out" "-L" "C:\\Users\\Trevor\\.rustup\\toolchains\\nightly-2017-04-07-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-Wl,-Bstatic" "C:\\try\\target\\debug\\deps\\libtry-cca920ec7eb23b3b.rlib" "-L" "C:\\Users\\Trevor\\.rustup\\toolchains\\nightly-2017-04-07-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib" "-Wl,-Bdynamic" "-l" "std-e2451a256f66ac19" "-Wl,-Bstatic" "C:\\Users\\Trevor\\.rustup\\toolchains\\nightly-2017-04-07-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\libcompiler_builtins-91b619d34dd1f5aa.rlib" "-Wl,-Bdynamic" "-l" "advapi32" "-l" "ws2_32" "-l" "userenv" "-l" "shell32" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-luser32" "-lkernel32" "C:\\Users\\Trevor\\.rustup\\toolchains\\nightly-2017-04-07-x86_64-pc-windows-gnu\\lib\\rustlib\\x86_64-pc-windows-gnu\\lib\\rsend.o"
  = note: C:\try\target\debug\deps\libtry-cca920ec7eb23b3b.rlib(trytls.o):trytls.c:(.text+0x13): undefined reference to `__emutls_get_address'
          

error: aborting due to previous error

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:453
note: Run with `RUST_BACKTRACE=1` for a backtrace.
thread 'rustc' panicked at 'couldn't compile the test', src\librustdoc\test.rs:270


failures:
    src\lib.rs -  (line 3)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured

error: test failed, to rerun pass '--doc'
