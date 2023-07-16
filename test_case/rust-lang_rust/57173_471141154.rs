plain
travis_time:end:3a396554:start=1552100451593451715,finish=1552100454241567859,duration=2648116144
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:18:05]    Compiling rustc_allocator v0.0.0 (/checkout/src/librustc_allocator)
[00:19:57]    Compiling rustc_lint v0.0.0 (/checkout/src/librustc_lint)
[00:19:58]    Compiling rustc_traits v0.0.0 (/checkout/src/librustc_traits)
[00:20:01]    Compiling rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
[00:20:03] error[E0559]: variant `resolve_imports::ImportDirectiveSubclass<'_>::ExternCrate` has no field named `no_link`
[00:20:03]    --> src/librustc_resolve/build_reduced_graph.rs:388:25
[00:20:03]     |
[00:20:03] 388 |                         no_link: attr::contains_name(&item.attrs, "no_link"),
[00:20:03]     |                         ^^^^^^^ `resolve_imports::ImportDirectiveSubclass<'_>::ExternCrate` does not have this field
[00:20:03] error: aborting due to previous error
[00:20:03] 
[00:20:03] For more information about this error, try `rustc --explain E0559`.
[00:20:03] error: Could not compile `rustc_resolve`.
---
173732 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
156416 ./src/llvm-project/clang
156208 ./obj/build/bootstrap/debug/incremental
141344 ./obj/build/bootstrap/debug/incremental/bootstrap-5f86g9tk67ex
141340 ./obj/build/bootstrap/debug/incremental/bootstrap-5f86g9tk67ex/s-fa6965y5hy-1vaz4b3-1f021mdjmj9bb
116360 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
116356 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
113708 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
108532 ./src/llvm-project/lldb
---
travis_time:end:05109340:start=1552101749571327395,finish=1552101749576565686,duration=5238291
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f41ff9c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06e928ab
travis_time:start:06e928ab
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:057eac7f
$ dmesg | grep -i kill
