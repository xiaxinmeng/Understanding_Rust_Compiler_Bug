plain
travis_time:end:22139daf:start=1552950955234833528,finish=1552951029670501309,duration=74435667781
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:08:28]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:48] error[E0061]: this function takes 2 parameters but 1 parameter was supplied
[00:08:48]    --> src/librustc/infer/error_reporting/need_type_info.rs:168:25
[00:08:48]     |
[00:08:48] 69  | /     pub fn extract_type_name(
[00:08:48] 70  | |         &self,
[00:08:48] 71  | |         ty: &'a Ty<'tcx>,
[00:08:48] 72  | |         highlight: Option<ty::print::RegionHighlightMode>,
[00:08:48] 88  | |         s
[00:08:48] 89  | |     }
[00:08:48]     | |_____- defined here
[00:08:48] ...
[00:08:48] ...
[00:08:48] 168 |           let name = self.extract_type_name(&ty);
[00:08:48] 
[00:09:02] error: aborting due to previous error
[00:09:02] 
[00:09:02] For more information about this error, try `rustc --explain E0061`.
---
travis_time:end:14c3a0e1:start=1552951582334241514,finish=1552951582339595020,duration=5353506
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d1e3e28
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1daa473c
travis_time:start:1daa473c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file 
