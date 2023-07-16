plain
travis_time:end:03cdab5c:start=1561404259753616608,finish=1561404262139765483,duration=2386148875
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:01:49] 
[01:01:49] running 9 tests
[01:01:49] iiiiiiiii
[01:01:49] 
[01:01:49]  finished in 0.151
[01:01:49] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:02:05] 
[01:02:05] running 122 tests
[01:02:31] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:02:36] .i.i......iii.i.....ii
[01:02:36] 
[01:02:36]  finished in 30.648
[01:02:36] travis_fold:end:test_debuginfo

---
[01:23:13]    Compiling syntax v0.0.0 (/checkout/src/libsyntax)
[01:23:22] error[E0308]: mismatched types
[01:23:22]     --> src/libsyntax/mut_visit.rs:1275:9
[01:23:22]      |
[01:23:22] 1274 |                         krate: &ast::Crate) -> io::Result<()> {
[01:23:22]      |                                                -------------- expected `std::result::Result<(), std::io::Error>` because of return type
[01:23:22] 1275 |         s.print_mod(&krate.module, &krate.attrs)
[01:23:22]      |
[01:23:22]      = note: expected type `std::result::Result<(), std::io::Error>`
[01:23:22]                 found type `()`
[01:23:22] 
[01:23:22] 
[01:23:22] error[E0308]: mismatched types
[01:23:22]     --> src/libsyntax/mut_visit.rs:1315:39
[01:23:22]      |
[01:23:22] 1315 |                 pprust::to_string(|s| fake_print_crate(s, &krate)),
[01:23:22]      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found enum `std::result::Result`
[01:23:22]      = note: expected type `()`
[01:23:22]                 found type `std::result::Result<(), std::io::Error>`
[01:23:22] 
[01:23:22] error[E0308]: mismatched types
[01:23:22] error[E0308]: mismatched types
[01:23:22]     --> src/libsyntax/mut_visit.rs:1331:39
[01:23:22]      |
[01:23:22] 1331 |                 pprust::to_string(|s| fake_print_crate(s, &krate)),
[01:23:22]      |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected (), found enum `std::result::Result`
[01:23:22]      = note: expected type `()`
[01:23:22]                 found type `std::result::Result<(), std::io::Error>`
[01:23:22] 
[01:23:25] error: aborting due to 3 previous errors
[01:23:25] error: aborting due to 3 previous errors
[01:23:25] 
[01:23:25] For more information about this error, try `rustc --explain E0308`.
[01:23:25] error: Could not compile `syntax`.
[01:23:25] 
[01:23:25] To learn more, run the command again with --verbose.
[01:23:25] 
[01:23:25] 
[01:23:25] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:23:25] 
[01:23:25] 
[01:23:25] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:23:25] Build completed unsuccessfully in 1:19:05
---
travis_time:end:03325cfa:start=1561409279616669704,finish=1561409279622546288,duration=5876584
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01942b08
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
