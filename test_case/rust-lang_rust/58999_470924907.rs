plain
travis_time:end:04769590:start=1552050409805695761,finish=1552050486178871382,duration=76373175621
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
[00:06:26]    Compiling rustc_errors v0.0.0 (/checkout/src/librustc_errors)
[00:06:40]    Compiling synstructure v0.10.1
[00:07:00]    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
[00:07:55]    Compiling syntax_ext v0.0.0 (/checkout/src/libsyntax_ext)
[00:08:37] error[E0501]: cannot borrow `*self` as mutable because previous closure requires unique access
[00:08:37]     --> src/librustc/hir/lowering.rs:3975:21
[00:08:37]      |
[00:08:37] 3975 |                       self.with_new_scopes(|this| {
[00:08:37]      |                       ^    --------------- ------ closure construction occurs here
[00:08:37]      |  _____________________|    first borrow later used by call
[00:08:37]      | |
[00:08:37]      | |
[00:08:37] 3976 | |                         // FIXME(cramertj): allow `async` non-`move` closures with arguments.
[00:08:37] 3977 | |                         if capture_clause == CaptureBy::Ref &&
[00:08:37] 3978 | |                             !decl.inputs.is_empty()
[00:08:37] ...    |
[00:08:37] 4006 | |                         self.returns_impl = returns_impl;
[00:08:37]      | |                         ---- first borrow occurs due to use of `self` in closure
[00:08:37] 4013 | |                         )
[00:08:37] 4014 | |                     })
[00:08:37] 4014 | |                     })
[00:08:37]      | |______________________^ second borrow occurs here
[00:08:37] 
[00:08:37] error[E0500]: closure requires unique access to `self` but it is already borrowed
[00:08:37]     --> src/librustc/hir/lowering.rs:3975:42
[00:08:37]      |
[00:08:37] 3975 |                     self.with_new_scopes(|this| {
[00:08:37]      |                     ---- --------------- ^^^^^^ closure construction occurs here
[00:08:37]      |                     |    first borrow later used by call
[00:08:37]      |                     borrow occurs here
[00:08:37] ...
[00:08:37] ...
[00:08:37] 4006 |                         self.returns_impl = returns_impl;
[00:08:37]      |                         ---- second borrow occurs due to use of `self` in closure
[00:08:37] 
[00:08:37] error[E0501]: cannot borrow `*self` as mutable because previous closure requires unique access
[00:08:37]     --> src/librustc/hir/lowering.rs:4019:21
[00:08:37]      |
[00:08:37] 4019 |                       self.with_new_scopes(|this| {
[00:08:37]      |                       ^    --------------- ------ closure construction occurs here
[00:08:37]      |  _____________________|    first borrow later used by call
[00:08:37]      | |
[00:08:37] 4020 | |                         let mut is_generator = false;
[00:08:37] 4020 | |                         let mut is_generator = false;
[00:08:37] 4021 | |                         let body_id = this.lower_body(Some(decl), |this| {
[00:08:37] 4022 | |                             let e = this.lower_expr(body);
[00:08:37] ...    |
[00:08:37] 4051 | |                         self.returns_impl = returns_impl;
[00:08:37]      | |                         ---- first borrow occurs due to use of `self` in closure
[00:08:37] 4058 | |                         )
[00:08:37] 4059 | |                     })
[00:08:37] 4059 | |                     })
[00:08:37]      | |______________________^ second borrow occurs here
[00:08:37] 
[00:08:37] error[E0500]: closure requires unique access to `self` but it is already borrowed
[00:08:37]     --> src/librustc/hir/lowering.rs:4019:42
[00:08:37]      |
[00:08:37] 4019 |                     self.with_new_scopes(|this| {
[00:08:37]      |                     ---- --------------- ^^^^^^ closure construction occurs here
[00:08:37]      |                     |    first borrow later used by call
[00:08:37]      |                     borrow occurs here
[00:08:37] ...
[00:08:37] ...
[00:08:37] 4051 |                         self.returns_impl = returns_impl;
[00:08:37]      |                         ---- second borrow occurs due to use of `self` in closure
[00:08:52] error: aborting due to 4 previous errors
[00:08:52] 
[00:08:52] Some errors occurred: E0500, E0501.
[00:08:52] For more information about an error, try `rustc --explain E0500`.
---
travis_time:end:05fc6061:start=1552051029738551637,finish=1552051029744319784,duration=5768147
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:005acee5
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:153c9de0
travis_time:start:153c9de0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0009d1c8
$ dmesg | grep -i kill
