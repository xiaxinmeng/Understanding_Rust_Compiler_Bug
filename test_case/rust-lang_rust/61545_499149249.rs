plain
travis_time:end:01244ea2:start=1559744409378636692,finish=1559744412997065327,duration=3618428635
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:06:31]     Checking syntax_pos v0.0.0 (/checkout/src/libsyntax_pos)
[00:06:32]     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:07:18]     Checking syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:07:43] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[00:07:43]    --> src/librustc/ty/query/plumbing.rs:173:34
[00:07:43]     |
[00:07:43] 173 |                   let result = job.r#await(tcx);
[00:07:43]     | 
[00:07:43]     | 
[00:07:43]    ::: src/librustc/ty/query/job.rs:78:5
[00:07:43]     |
[00:07:43] 78  | /     pub(super) fn r#await<'lcx>(
[00:07:43] 79  | |         &self,
[00:07:43] 80  | |         tcx: TyCtxt<'_, 'tcx, 'lcx>,
[00:07:43] ...   |
[00:07:43] 99  | |         })
[00:07:43] 100 | |     }
[00:07:43]     | |_____- defined here
[00:07:43]     | |_____- defined here
[00:07:43] 
[00:07:43] error[E0308]: mismatched types
[00:07:43]    --> src/librustc/ty/query/plumbing.rs:177:67
[00:07:43]     |
[00:07:43] 177 |                     return TryGetJob::Cycle(Q::handle_cycle_error(tcx, cycle));
[00:07:43]     |                                                                   |
[00:07:43]     |                                                                   |
[00:07:43]     |                                                                   expected struct `ty::context::TyCtxt`, found struct `ty::query::TyCtxtAt`
[00:07:43]     |                                                                   help: consider dereferencing the type: `*tcx`
[00:07:43]     = note: expected type `ty::context::TyCtxt<'_, '_, '_>`
[00:07:43]                found type `ty::query::TyCtxtAt<'a, 'tcx, '_>`
[00:07:43] 
[00:07:47] error: aborting due to 2 previous errors
---
travis_time:end:02409f3c:start=1559744892677729089,finish=1559744892684576331,duration=6847242
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:01b21ecc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:001ad100
travis_time:start:001ad100
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13caeea8
$ dmesg | grep -i kill
