plain
travis_time:end:09d3523f:start=1549547678291096058,finish=1549547764844086432,duration=86552990374
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=mingw-check
---
[00:04:40]     Checking arena v0.0.0 (/checkout/src/libarena)
[00:04:41]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:04:42]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:04:57]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:05:19] error[E0277]: `std::rc::Rc<[syntax::ast::Symbol]>` cannot be sent between threads safely
[00:05:19]    --> src/librustc/ty/query/job.rs:501:5
[00:05:19]     |
[00:05:19] 501 |     thread::spawn(move || {
[00:05:19]     |     ^^^^^^^^^^^^^ `std::rc::Rc<[syntax::ast::Symbol]>` cannot be sent between threads safely
[00:05:19]     |
[00:05:19]     = help: within `syntax_pos::hygiene::MarkData`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<[syntax::ast::Symbol]>`
[00:05:19]     = note: required because it appears within the type `std::option::Option<std::rc::Rc<[syntax::ast::Symbol]>>`
[00:05:19]     = note: required because it appears within the type `syntax_pos::hygiene::ExpnInfo`
[00:05:19]     = note: required because it appears within the type `std::option::Option<syntax_pos::hygiene::ExpnInfo>`
[00:05:19]     = note: required because it appears within the type `syntax_pos::hygiene::MarkData`
[00:05:19]     = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<syntax_pos::hygiene::MarkData>`
[00:05:19]     = note: required because it appears within the type `alloc::raw_vec::RawVec<syntax_pos::hygiene::MarkData>`
[00:05:19]     = note: required because it appears within the type `std::vec::Vec<syntax_pos::hygiene::MarkData>`
[00:05:19]     = note: required because it appears within the type `syntax_pos::hygiene::HygieneData`
[00:05:19]     = note: required because of the requirements on the impl of `std::marker::Sync` for `lock_api::mutex::Mutex<parking_lot::raw_mutex::RawMutex, syntax_pos::hygiene::HygieneData>`
[00:05:19]     = note: required because it appears within the type `rustc_data_structures::sync::Lock<syntax_pos::hygiene::HygieneData>`
[00:05:19]     = note: required because it appears within the type `syntax_pos::Globals`
[00:05:19]     = note: required because of the requirements on the impl of `std::marker::Send` for `&syntax_pos::Globals`
[00:05:19]     = note: required because it appears within the type `[closure@src/librustc/ty/query/job.rs:501:19: 511:6 gcx_ptr:&rustc_data_structures::sync::Lock<usize>, syntax_pos_globals:&syntax_pos::Globals, registry:std::sync::Arc<rustc_rayon_core::registry::Registry>]`
[00:05:19]     = note: required by `std::thread::spawn`
[00:05:22] error: aborting due to previous error
[00:05:22] 
[00:05:22] For more information about this error, try `rustc --explain E0277`.
[00:05:22] error: Could not compile `rustc`.
[00:05:22] error: Could not compile `rustc`.
[00:05:22] 
[00:05:22] To learn more, run the command again with --verbose.
[00:05:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
[00:05:22] Build completed unsuccessfully in 0:03:18
travis_time:end:3be53496:start=1549547774229008589,finish=1549548097362783728,duration=323133775139
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
---
travis_time:end:13a9cbfe:start=1549548098108541543,finish=1549548098114486036,duration=5944493
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2563a090
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1778376c
travis_time:start:1778376c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1614d80c
$ dmesg | grep -i kill
