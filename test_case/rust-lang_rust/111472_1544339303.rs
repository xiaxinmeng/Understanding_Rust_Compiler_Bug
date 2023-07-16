plain
    Finished release [optimized] target(s) in 28.42s
     Running unittests src/main.rs (obj/build/x86_64-unknown-linux-gnu/stage1-tools/x86_64-unknown-linux-gnu/release/deps/compiletest-53d0e36955df0a46)

running 38 tests
..FF.F........................F.FFFFFF
failures:

---- header::tests::matches_abi stdout ----
---- header::tests::matches_abi stdout ----
thread 'header::tests::matches_abi' panicked at 'aarch64-apple-ios-macabi macabi', src/tools/compiletest/src/header/tests.rs:396:9
error: test failed, to rerun pass `--bin compiletest`

---- header::tests::asm_support stdout ----
thread 'header::tests::asm_support' panicked at 'assertion failed: `(left == right)`
thread 'header::tests::asm_support' panicked at 'assertion failed: `(left == right)`
  left: `true`,
 right: `false`', src/tools/compiletest/src/header/tests.rs:296:9

---- header::tests::only_target stdout ----
thread 'header::tests::only_target' panicked at 'assertion failed: check_ignore(&config, \"// only-linux\")', src/tools/compiletest/src/header/tests.rs:207:5
---- header::tests::families stdout ----
---- header::tests::families stdout ----
thread 'header::tests::families' panicked at 'assertion failed: config.matches_family(family)', src/tools/compiletest/src/header/tests.rs:481:9
---- header::tests::matches_env stdout ----
---- header::tests::matches_env stdout ----
thread 'header::tests::matches_env' panicked at 'x86_64-fortanix-unknown-sgx sgx', src/tools/compiletest/src/header/tests.rs:381:9
---- header::tests::matches_os stdout ----
---- header::tests::matches_os stdout ----
thread 'header::tests::matches_os' panicked at 'x86_64-fortanix-unknown-sgx unknown', src/tools/compiletest/src/header/tests.rs:366:9
---- header::tests::ignore_arch stdout ----
---- header::tests::ignore_arch stdout ----
thread 'header::tests::ignore_arch' panicked at 'i686-unknown-linux-gnu x86', src/tools/compiletest/src/header/tests.rs:350:9
---- header::tests::is_big_endian stdout ----
thread 'header::tests::is_big_endian' panicked at 'assertion failed: `(left == right)`
  left: `false`,
  left: `false`,
 right: `true`: bpfeb-unknown-none true', src/tools/compiletest/src/header/tests.rs:413:9
---- header::tests::pointer_width stdout ----
---- header::tests::pointer_width stdout ----
thread 'header::tests::pointer_width' panicked at 'assertion failed: `(left == right)`
  left: `64`,
 right: `32`: i686-unknown-linux-gnu 32', src/tools/compiletest/src/header/tests.rs:429:9
---- header::tests::wasm_special stdout ----
thread 'header::tests::wasm_special' panicked at 'assertion failed: `(left == right)`
  left: `false`,
  left: `false`,
 right: `true`: wasm32-unknown-unknown wasm32', src/tools/compiletest/src/header/tests.rs:461:9

failures:
    header::tests::matches_abi
    header::tests::asm_support
