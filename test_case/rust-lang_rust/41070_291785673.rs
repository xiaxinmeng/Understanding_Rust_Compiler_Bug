
[01:39:48] Testing libstd stage2 (i686-pc-windows-gnu -> i686-pc-windows-gnu)
[01:39:50]    Compiling alloc v0.0.0 (file:///C:/projects/rust/src/liballoc)
[01:39:50]    Compiling core v0.0.0 (file:///C:/projects/rust/src/libcore)
[01:39:55]    Compiling rand v0.0.0 (file:///C:/projects/rust/src/librand)
[01:40:01]    Compiling cmake v0.1.22
[01:40:04]    Compiling collections v0.0.0 (file:///C:/projects/rust/src/libcollections)
[01:40:56]    Compiling rustc_tsan v0.0.0 (file:///C:/projects/rust/src/librustc_tsan)
[01:40:56]    Compiling rustc_asan v0.0.0 (file:///C:/projects/rust/src/librustc_asan)
[01:40:57]    Compiling rustc_msan v0.0.0 (file:///C:/projects/rust/src/librustc_msan)
[01:41:07]    Compiling rustc_lsan v0.0.0 (file:///C:/projects/rust/src/librustc_lsan)
[01:41:08]    Compiling std v0.0.0 (file:///C:/projects/rust/src/libstd)
[01:41:09] error: linking with `gcc` failed: exit code: 1
[01:41:09]   |
[01:41:09]   = note: "gcc" "-Wl,--enable-long-section-names" "-fno-use-linker-plugin" "-Wl,--nxcompat" "-nostdlib" "-Wl,--large-address-aware" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib\\crt2.o" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib\\rsbegin.o" "-L" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\i686-pc-windows-gnu\\release\\deps\\collectionstest-cf7d8872f6b0686a.0.o" "-o" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\i686-pc-windows-gnu\\release\\deps\\collectionstest-cf7d8872f6b0686a.exe" "-Wl,--gc-sections" "-nodefaultlibs" "-L" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\i686-pc-windows-gnu\\release\\deps" "-L" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1-std\\release\\deps" "-L" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "-Wl,-Bstatic" "-Wl,-Bdynamic" "-L" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "-l" "test-51dd1e12bb8fc6c0" "-L" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "-l" "term-9bb4a9959ced7ebc" "-L" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "-l" "getopts-83c65310844a796d" "-L" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib" "-l" "std-4d6881ec6132b951" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib\\libcompiler_builtins-7ac5a34e9b48514f.rlib" "-l" "kernel32" "-l" "advapi32" "-l" "ws2_32" "-l" "userenv" "-l" "shell32" "-l" "gcc_eh" "-lmingwex" "-lmingw32" "-lgcc" "-lmsvcrt" "-luser32" "-lkernel32" "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage1\\lib\\rustlib\\i686-pc-windows-gnu\\lib\\rsend.o"
[01:41:09]   = note: collect2.exe: error: ld returned 5 exit status
[01:41:09]           
[01:41:09] 
[01:41:09] error: aborting due to previous error
[01:41:09] 
[01:41:09] error: Could not compile `collections`.
[01:41:09] Build failed, waiting for other jobs to finish...
[01:43:35] error: build failed
[01:43:35] 
[01:43:35] 
[01:43:35] command did not execute successfully: "C:\\projects\\rust\\build\\i686-pc-windows-gnu\\stage0/bin\\cargo.exe" "test" "-j" "2" "--target" "i686-pc-windows-gnu" "--release" "--locked" "--manifest-path" "C:\\projects\\rust\\src/libstd\\Cargo.toml" "--features" "panic-unwind jemalloc backtrace" "-p" "std:0.0.0" "-p" "alloc:0.0.0" "-p" "rustc_msan:0.0.0" "-p" "panic_abort:0.0.0" "-p" "rustc_asan:0.0.0" "-p" "rustc_tsan:0.0.0" "-p" "core:0.0.0" "-p" "collections:0.0.0" "-p" "rand:0.0.0" "-p" "libc:0.0.0" "-p" "alloc_system:0.0.0" "-p" "compiler_builtins:0.0.0" "-p" "rustc_lsan:0.0.0" "-p" "unwind:0.0.0" "-p" "std_unicode:0.0.0" "--"
[01:43:35] expected success, got: exit code: 101
