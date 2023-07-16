plain
travis_time:end:0a878030:start=1558825991989590043,finish=1558826081954928493,duration=89965338450
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:07:31]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:38]    Compiling syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:07:42]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:08:51]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:09:23] error[E0599]: no variant named `AssociatedConst` found for type `hir::def::DefKind` in the current scope
[00:09:23]    --> src/librustc/ty/print/pretty.rs:826:33
[00:09:23]     |
[00:09:23] 826 |                 | Some(DefKind::AssociatedConst) => p!(print_value_path(did, substs)),
[00:09:23]     |                                 |
[00:09:23]     |                                 variant not found in `hir::def::DefKind`
[00:09:23]     |                                 help: did you mean: `AssocConst`
[00:09:23]     | 
[00:09:23]     | 
[00:09:23]    ::: src/librustc/hir/def.rs:48:1
[00:09:23]     |
[00:09:23] 48  | pub enum DefKind {
[00:09:23]     | ---------------- variant `AssociatedConst` not found here
[00:09:27] error: aborting due to previous error
[00:09:27] 
[00:09:27] For more information about this error, try `rustc --explain E0599`.
[00:09:28] error: Could not compile `rustc`.
---
travis_time:end:033f75de:start=1558826660768890565,finish=1558826660774246810,duration=5356245
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02f85b3a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:056e79f7
travis_time:start:056e79f7
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0eb169c7
$ dmesg | grep -i kill
