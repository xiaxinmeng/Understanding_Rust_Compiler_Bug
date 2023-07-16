
rust@563768380f6b:~/rust/src/liblibc/libc-test$ cargo build --target=x86_64-unknown-linux-uclibc
   Compiling libc v0.1.12
   Compiling libc v0.2.14 (file:///home/rust/rust/src/liblibc)
   Compiling bitflags v0.3.3
   Compiling gcc v0.3.28
   Compiling winapi v0.2.6
   Compiling log v0.3.6
   Compiling winapi-build v0.1.1
   Compiling unicode-xid v0.0.3
   Compiling kernel32-sys v0.2.2
   Compiling rustc-serialize v0.3.19
   Compiling term v0.2.14
   Compiling syntex_syntax v0.19.1
   Compiling ctest v0.1.0 (https://github.com/alexcrichton/ctest#a6becb6d)
   Compiling libc-test v0.1.0 (file:///home/rust/rust/src/liblibc/libc-test)
error: failed to run custom build command for `libc-test v0.1.0 (file:///home/rust/rust/src/liblibc/libc-test)`
process didn't exit successfully: `/home/rust/rust/src/liblibc/libc-test/target/debug/build/libc-test-daf5ba52955de10b/build-script-build` (exit code: 101)
--- stderr
thread 'main' panicked at 'unknown os/family width: x86_64-unknown-linux-uclibc', /home/rust/.cargo/git/checkouts/ctest-2cf90b4dc49563d8/master/src/lib.rs:791
note: Run with `RUST_BACKTRACE=1` for a backtrace.



rust@563768380f6b:~/rust/src/liblibc/libc-test$ RUST_BACKTRACE=1 cargo build --target=x86_64-unknown-linux-uclibc
   Compiling libc-test v0.1.0 (file:///home/rust/rust/src/liblibc/libc-test)
error: failed to run custom build command for `libc-test v0.1.0 (file:///home/rust/rust/src/liblibc/libc-test)`
process didn't exit successfully: `/home/rust/rust/src/liblibc/libc-test/target/debug/build/libc-test-daf5ba52955de10b/build-script-build` (exit code: 101)
--- stderr
thread 'main' panicked at 'unknown os/family width: x86_64-unknown-linux-uclibc', /home/rust/.cargo/git/checkouts/ctest-2cf90b4dc49563d8/master/src/lib.rs:791
stack backtrace:
   1:     0x7f43fdb214cd - std::sys::backtrace::tracing::imp::write::hb62d33c95f81ee87
   2:     0x7f43fdb287f1 - std::panicking::default_hook::_{{closure}}::h984c8a8683e109de
   3:     0x7f43fdb27a99 - std::panicking::default_hook::h99544c6f1463c3a6
   4:     0x7f43fdb280ba - std::panicking::rust_panic_with_hook::h4c9bf4384c12710c
   5:     0x7f43fdb27f72 - std::panicking::begin_panic::h4dfbfe1d1e09435b
   6:     0x7f43fdb27ea0 - std::panicking::begin_panic_fmt::h4e62fd66a38d7059
   7:     0x7f43fd6195df - ctest::default_cfg::he26fe8398e914530
                        at /home/rust/.cargo/git/checkouts/ctest-2cf90b4dc49563d8/master/src/lib.rs:8
   8:     0x7f43fd616be1 - ctest::TestGenerator::_generate_files::h6f1fafd9c082fca5
                        at /home/rust/.cargo/git/checkouts/ctest-2cf90b4dc49563d8/master/src/lib.rs:628
   9:     0x7f43fd616083 - ctest::TestGenerator::generate_files::h75f1b6fca14356cc
                        at /home/rust/.cargo/git/checkouts/ctest-2cf90b4dc49563d8/master/src/lib.rs:598
  10:     0x7f43fd6155f5 - ctest::TestGenerator::_generate::h8a208e6161ffbf61
                        at /home/rust/.cargo/git/checkouts/ctest-2cf90b4dc49563d8/master/src/lib.rs:553
  11:     0x7f43fd5c67de - ctest::TestGenerator::generate::hbf4b58da33afc291
                        at /home/rust/.cargo/git/checkouts/ctest-2cf90b4dc49563d8/master/src/lib.rs:549
  12:     0x7f43fd5ca818 - build_script_build::main::h72505fc50186b65e
                        at /home/rust/rust/src/liblibc/libc-test/build.rs:449
  13:     0x7f43fdb27dc7 - std::panicking::try::call::ha4131bda48c7b34e
  14:     0x7f43fdb2e786 - __rust_maybe_catch_panic
  15:     0x7f43fdb27259 - std::rt::lang_start::hef67681f86ca295c
  16:     0x7f43fd5ccae3 - main
  17:     0x7f43fcfd382f - __libc_start_main
  18:     0x7f43fd5bf248 - _start
  19:                0x0 - <unknown>
