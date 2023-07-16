plain
    100% |████████████████████████████████| 51kB 8.7MB/s 
Collecting botocore==1.10.59 (from awscli)
/usr/local/lib/python2.7/dist-packages/pip/_vendor/requests/packages/urllib3/util/ssl_.py:122: InsecurePlatformWarning: A true SSLContext object is not available. This prevents urllib3 from configuring SSL appropriately and may cause certain SSL connections to fail. You can upgrade to a newer version of Python to solve this. For more information, see https://urllib3.readthedocs.io/en/latest/security.html#insecureplatformwarning.
  InsecurePlatformWarning
  Downloading https://files.pythonhosted.org/packages/bf/77/26dab42032978b5f547869836523701279fc207d4affcad74b4b6d65f13f/botocore-1.10.59-py2.py3-none-any.whl (4.4MB)
    0% |                                | 10kB 48.1MB/s eta 0:00:01
    0% |▏                               | 20kB 41.5MB/s eta 0:00:01
    0% |▎                               | 30kB 44.5MB/s eta 0:00:01
    0% |▎                               | 40kB 10.2MB/s eta 0:00:01
---

[00:07:03]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:07:03]     Checking cc v1.0.17
[00:07:04]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:07:09] error[E0277]: `llvm::ffi::::ObjectFile` cannot be sent between threads safely
[00:07:09]   --> librustc_codegen_llvm/metadata.rs:68:16
[00:07:09]    |
[00:07:09] 68 |             Ok(rustc_erase_owner!(buf))
[00:07:09]    |                ^^^^^^^^^^^^^^^^^^^^^^^ `llvm::ffi::::ObjectFile` cannot be sent between threads safely
[00:07:09]    |
[00:07:09]    = help: the trait `std::marker::Send` is not implemented for `llvm::ffi::::ObjectFile`
[00:07:09]    = note: required because of the requirements on the impl of `std::marker::Send` for `&'static mut llvm::ffi::::ObjectFile`
[00:07:09]    = note: required because it appears within the type `llvm::ObjectFile`
[00:07:09]    = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<llvm::ObjectFile>`
[00:07:09]    = note: required because it appears within the type `std::boxed::Box<llvm::ObjectFile>`
[00:07:09]    = note: required because of the requirements on the impl of `std::marker::Send` for `rustc_data_structures::owning_ref::OwningRef<std::boxed::Box<llvm::ObjectFile>, [u8]>`
[00:07:09]    = note: required by `rustc_data_structures::sync::assert_send_val`
[00:07:09] 
[00:07:09] 
[00:07:09] error[E0599]: no method named `erase_send_sync_owner` found for type `rustc_data_structures::owning_ref::OwningRef<std::boxed::Box<llvm::ObjectFile>, [u8]>` in the current scope
[00:07:09]   --> librustc_codegen_llvm/metadata.rs:68:16
[00:07:09]    |
[00:07:09] 68 |             Ok(rustc_erase_owner!(buf))
[00:07:09]    |
[00:07:09]    = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
[00:07:09] 
[00:07:10] error: aborting due to 2 previous errors
[00:07:10] error: aborting due to 2 previous errors
[00:07:10] 
[00:07:10] Some errors occurred: E0277, E0599.
[00:07:10] For more information about an error, try `rustc --explain E0277`.
[00:07:10] error: Could not compile `rustc_codegen_llvm`.
[00:07:10] 
[00:07:10] Caused by:
[00:07:10]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_codegen_llvm librustc_codegen_llvm/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C opt-level=2 --cfg feature="jemalloc" --cfg feature="rustc_target" -C metadata=f54d758f11fac2cc -C extra-filename=-f54d758f11fac2cc --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-5099f2622d0ac9cb.rmeta --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-d4f20c4579f8a859.rmeta --extern env_logger=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libenv_logger-c89742f3f7220fb5.rmeta --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-fd60372f83685884.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-5fa0c601664c62aa.rmeta --extern libc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblibc-35106da195965c50.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-93b7c86a2fe6817d.rmeta --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-fc23dd3af341abe7.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-0f0d136f5ae5f501.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-749b6d2d402b71c0.rmeta --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-8ed452114d1779df.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-4a7b93a63f774f75.rmeta --extern rustc_codegen_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_codegen_utils-992d75f8bd75ada4.rmeta --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-a47fb3a40e998a7f.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-ae25928553554db9.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-27ab3a66299fe546.rmeta --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-a5e6a35ea3f8fee3.rmeta --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-4e71b682cc2a774c.rmeta --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-a39e6982ccbdb66b.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-03e7d42fc96ee656.rmeta --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f17568cb588a2773.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7392599013e26fe0.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-5e8d06e392ccc6bf.rmeta --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-1238cf2992abc0a0.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-571491b5b700df39/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-fcd4d8eb5dc11f1a/out` (exit code: 101)
[00:07:10] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--manifest-path" "/checkout/src/librustc_codegen_llvm/Cargo.toml" "--features" " jemalloc" "--message-format" "json"
[00:07:10] expected success, got: exit code: 101
[00:07:10] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:07:10] travis_fold:start:stage0-rustc_codegen_llvm
travis_time:start:stage0-rustc_codegen_llvm
travis_fold:end:stage0-rustc_codegen_llvm

---
travis_time:end:023d3a08:start=1531851369410474537,finish=1531851369418141725,duration=7667188
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:029ca810
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:09c21da8
travis_time:start:09c21da8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:20002617
$ dmesg | grep -i kill
