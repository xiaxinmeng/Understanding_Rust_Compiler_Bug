plain
[01:26:24] [RUSTC-TIMING] racer test:false 75.276
[01:26:28]    Compiling rls-vfs v0.4.6
[01:26:35] [RUSTC-TIMING] rls_vfs test:false 7.145
[01:27:06] [RUSTC-TIMING] rustfmt_nightly test:false 110.114
[01:27:09] error[E0050]: method `exec` has 4 parameters but the declaration in trait `cargo::core::compiler::Executor::exec` has 5
[01:27:09]    --> tools/rls/src/build/cargo.rs:333:75
[01:27:09]     |
[01:27:09] 333 |     fn exec(&self, mut cargo_cmd: ProcessBuilder, id: &PackageId, target: &Target) -> CargoResult<()> {
[01:27:09]     |                                                                           ^^^^^^^ expected 5 parameters, found 4
[01:27:09]     |
[01:27:09]     = note: `exec` from trait: `fn(&Self, cargo::util::process_builder::ProcessBuilder, &cargo::core::package_id::PackageId, &cargo::core::manifest::Target, cargo::core::compiler::build_config::CompileMode) -> std::result::Result<(), failure::error::Error>`
[01:27:09] error: aborting due to previous error
[01:27:09] 
[01:27:09] For more information about this error, try `rustc --explain E0050`.
[01:27:09] [RUSTC-TIMING] rls test:false 2.470
---
travis_time:end:24b9333a:start=1533056991244739226,finish=1533056991252044284,duration=7305058
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:041cb956
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:397d0890
travis_time:start:397d0890
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:12d635e0
$ dmesg | grep -i kill
