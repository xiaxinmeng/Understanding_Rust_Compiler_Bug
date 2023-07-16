plain
[01:11:59] [RUSTC-TIMING] rustc_rayon test:false 5.890
[01:11:59]    Compiling git2 v0.7.5
[01:11:59] [RUSTC-TIMING] failure test:false 1.994
[01:11:59]    Compiling rls-analysis v0.16.10
[01:12:05] [RUSTC-TIMING] im_rc test:false 5.705
[01:12:18] [RUSTC-TIMING] ignore test:false 31.913
[01:12:18]    Compiling crates-io v0.21.0 (/checkout/src/tools/cargo/src/crates-io)
[01:12:23] [RUSTC-TIMING] rustc_data_structures test:false 6.319
[01:12:34]    Compiling opener v0.3.0
---
[01:19:20] 608 | |         Ok(())
[01:19:20] 609 | |     }
[01:19:20]     | |_____^ expected struct `cargo::core::package_id::PackageId`, found reference
[01:19:20]     |
[01:19:20]     = note: expected type `fn(&build::cargo::RlsExecutor, cargo::util::process_builder::ProcessBuilder, cargo::core::package_id::PackageId, &cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode) -> std::result::Result<(), failure::error::Error>`
[01:19:20]                found type `fn(&build::cargo::RlsExecutor, cargo::util::process_builder::ProcessBuilder, &cargo::core::package_id::PackageId, &cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode) -> std::result::Result<(), failure::error::Error>`
[01:19:21] error: aborting due to previous error
[01:19:21] 
[01:19:21] For more information about this error, try `rustc --explain E0053`.
[01:19:21] [RUSTC-TIMING] rls test:false 2.591
---
travis_time:end:0389edd0:start=1543818023074685868,finish=1543818023080925294,duration=6239426
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:16643dba
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16e1b50a
travis_time:start:16e1b50a
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b873ea4
$ dmesg | grep -i kill
