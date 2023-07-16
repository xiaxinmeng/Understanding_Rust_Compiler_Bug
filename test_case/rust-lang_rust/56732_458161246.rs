plain
travis_time:end:0b303dd2:start=1548686474417952464,finish=1548686477401099376,duration=2983146912
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:07:15]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:39] error[E0308]: mismatched types
[00:07:39]     --> src/librustc/session/mod.rs:1046:32
[00:07:39]      |
[00:07:39] 1046 |             EmitterWriter::new(dst, Some(source_map.clone()), false, false)
[00:07:39]      |                                ^^^ expected trait `std::io::Write + std::marker::Send`, found trait `std::io::Write + rustc_data_structures::sync::Send`
[00:07:39]      |
[00:07:39]      = note: expected type `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>`
[00:07:39]                 found type `std::boxed::Box<dyn std::io::Write + rustc_data_structures::sync::Send>`
[00:07:39] error[E0308]: mismatched types
[00:07:39]     --> src/librustc/session/mod.rs:1058:17
[00:07:39]      |
[00:07:39] 1058 |                 dst,
[00:07:39] 1058 |                 dst,
[00:07:39]      |                 ^^^ expected trait `std::io::Write + std::marker::Send`, found trait `std::io::Write + rustc_data_structures::sync::Send`
[00:07:39]      |
[00:07:39]      = note: expected type `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>`
[00:07:39]                 found type `std::boxed::Box<dyn std::io::Write + rustc_data_structures::sync::Send>`
[00:07:39] error[E0308]: mismatched types
[00:07:39]     --> src/librustc/session/mod.rs:1068:41
[00:07:39]      |
[00:07:39]      |
[00:07:39] 1068 |             Box::new(EmitterWriter::new(dst, Some(source_map.clone()), true, false))
[00:07:39]      |                                         ^^^ expected trait `std::io::Write + std::marker::Send`, found trait `std::io::Write + rustc_data_structures::sync::Send`
[00:07:39]      |
[00:07:39]      = note: expected type `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>`
[00:07:39]                 found type `std::boxed::Box<dyn std::io::Write + rustc_data_structures::sync::Send>`
[00:07:39] error[E0308]: mismatched types
[00:07:39]     --> src/librustc/session/mod.rs:1109:65
[00:07:39]      |
[00:07:39]      |
[00:07:39] 1109 |             default_emitter(&sopts, registry, &source_map, Some(write))
[00:07:39]      |                                                                 ^^^^^ expected trait `std::io::Write + rustc_data_structures::sync::Send`, found trait `std::io::Write + std::marker::Send`
[00:07:39]      |
[00:07:39]      = note: expected type `std::boxed::Box<dyn std::io::Write + rustc_data_structures::sync::Send>`
[00:07:39]                 found type `std::boxed::Box<(dyn std::io::Write + std::marker::Send + 'static)>`
[00:07:39] error[E0308]: match arms have incompatible types
[00:07:39]     --> src/librustc/session/mod.rs:1106:19
[00:07:39]      |
[00:07:39]      |
[00:07:39] 1106 |       let emitter = match diagnostics_output {
[00:07:39]      |  ___________________^
[00:07:39] 1107 | |         DiagnosticOutput::Default => default_emitter(&sopts, registry, &source_map, None),
[00:07:39] 1108 | |         DiagnosticOutput::Raw(write) => {
[00:07:39] 1109 | |             default_emitter(&sopts, registry, &source_map, Some(write))
[00:07:39] 1110 | |         }
[00:07:39] 1111 | |         DiagnosticOutput::Emitter(emitter) => emitter,
[00:07:39] 1112 | |     };
[00:07:39] 1112 | |     };
[00:07:39]      | |_____^ expected trait `errors::emitter::Emitter + rustc_data_structures::sync::Send`, found trait `errors::emitter::Emitter + std::marker::Send`
[00:07:39]      |
[00:07:39]      = note: expected type `std::boxed::Box<dyn errors::emitter::Emitter + rustc_data_structures::sync::Send>`
[00:07:39]                 found type `std::boxed::Box<(dyn errors::emitter::Emitter + std::marker::Send + 'static)>`
[00:07:47] error: aborting due to 5 previous errors
[00:07:47] 
[00:07:47] For more information about this error, try `rustc --explain E0308`.
[00:07:47] error: Could not compile `rustc`.
[00:07:47] error: Could not compile `rustc`.
[00:07:47] 
[00:07:47] To learn more, run the command again with --verbose.
[00:07:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:47] expected success, got: exit code: 101
[00:07:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:47] Build completed unsuccessfully in 0:03:49
[00:07:47] make: *** [all] Error 1
[00:07:47] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:001067f9
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Jan 28 14:49:14 UTC 2019
---
travis_time:end:00dae170:start=1548686955485962928,finish=1548686955490550223,duration=4587295
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:105dcff9
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:178d826c
travis_time:start:178d826c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0156c8ac
$ dmesg | grep -i kill
