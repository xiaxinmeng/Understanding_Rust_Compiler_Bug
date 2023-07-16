
   Compiling libc v0.2.68 (/home/amanieu/code/rust-libc)
error: failed to run custom build command for `libc v0.2.68 (/home/amanieu/code/rust-libc)`

Caused by:
  process didn't exit successfully: `/home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
--- stdout
cargo:rustc-cfg=freebsd11
cargo:rustc-cfg=libc_priv_mod_use
cargo:rustc-cfg=libc_union
cargo:rustc-cfg=libc_const_size_of
cargo:rustc-cfg=libc_align
cargo:rustc-cfg=libc_core_cvoid
cargo:rustc-cfg=libc_packedN

--- stderr
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x73693724)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x646e6148)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x34616639)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x6124544c)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous run count: corrupt object tag (0x636f6c6c)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x735f646c)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0xf7ebb268)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000001)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x65727573)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
profiling: /home/amanieu/code/rust-libc/target/debug/build/libc-5d0f6fa6d686682d/build_script_build-5d0f6fa6d686682d.gcda: cannot merge previous GCDA file: corrupt arc tag (0x00000000)
