plain
travis_time:end:01070ef8:start=1547999304493609847,finish=1547999399009341518,duration=94515731671
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:38]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:55] warning: unnecessary `unsafe` block
[00:03:55]    --> src/libcore/num/mod.rs:71:30
[00:03:55]     |
[00:03:55] 33  | / macro_rules! nonzero_integers {
[00:03:55] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:03:55] 36  | |             doc_comment! {
[00:03:55] ...   |
[00:03:55] 71  | |                         Some(unsafe { $Ty(n) })
[00:03:55]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:03:55]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:03:55] ...   |
[00:03:55] 97  | |     }
[00:03:55] 98  | | }
[00:03:55]     | |_- in this expansion of `nonzero_integers!`
[00:03:55] 100 | / nonzero_integers! {
[00:03:55] 101 | |     NonZeroU8(u8);
[00:03:55] 102 | |     NonZeroU16(u16);
[00:03:55] 103 | |     NonZeroU32(u32);
---
[00:03:55] 
[00:03:55] warning: unnecessary `unsafe` block
[00:03:55]    --> src/libcore/num/mod.rs:71:30
[00:03:55]     |
[00:03:55] 33  | / macro_rules! nonzero_integers {
[00:03:55] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:03:55] 36  | |             doc_comment! {
[00:03:55] ...   |
[00:03:55] 71  | |                         Some(unsafe { $Ty(n) })
[00:03:55]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:03:55]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:03:55] ...   |
[00:03:55] 97  | |     }
[00:03:55] 98  | | }
[00:03:55]     | |_- in this expansion of `nonzero_integers!`
[00:03:55] 100 | / nonzero_integers! {
[00:03:55] 101 | |     NonZeroU8(u8);
[00:03:55] 102 | |     NonZeroU16(u16);
[00:03:55] 103 | |     NonZeroU32(u32);
---
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]    --> src/libcore/num/mod.rs:50:17
[00:03:55]     |
[00:03:55] 33  | / macro_rules! nonzero_integers {
[00:03:55] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:03:55] 36  | |             doc_comment! {
[00:03:55] ...   |
[00:03:55] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:03:55]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:03:55]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:03:55] ...   |
[00:03:55] 97  | |     }
[00:03:55] 98  | | }
[00:03:55]     | |_- in this expansion of `nonzero_integers!`
[00:03:55] 100 | / nonzero_integers! {
[00:03:55] 101 | |     NonZeroU8(u8);
[00:03:55] 102 | |     NonZeroU16(u16);
[00:03:55] 103 | |     NonZeroU32(u32);
---
[00:03:55] 
[00:03:55] warning: unused attribute
[00:03:55]    --> src/libcore/num/mod.rs:50:17
[00:03:55]     |
[00:03:55] 33  | / macro_rules! nonzero_integers {
[00:03:55] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:03:55] 36  | |             doc_comment! {
[00:03:55] ...   |
[00:03:55] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:03:55]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:03:55]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:03:55] ...   |
[00:03:55] 97  | |     }
[00:03:55] 98  | | }
[00:03:55]     | |_- in this expansion of `nonzero_integers!`
[00:03:55] 100 | / nonzero_integers! {
[00:03:55] 101 | |     NonZeroU8(u8);
[00:03:55] 102 | |     NonZeroU16(u16);
[00:03:55] 103 | |     NonZeroU32(u32);
---
[00:23:02]    Compiling rustc_interface v0.0.0 (/checkout/src/librustc_interface)
[00:23:03] error[E0433]: failed to resolve: use of undeclared type or module `libc`
[00:23:03]    --> src/librustc_interface/util.rs:400:16
[00:23:03]     |
[00:23:03] 400 |             if libc::dladdr(addr, &mut info) == 0 {
[00:23:03]     |                ^^^^ use of undeclared type or module `libc`
[00:23:04] error: aborting due to previous error
[00:23:04] 
[00:23:04] For more information about this error, try `rustc --explain E0433`.
[00:23:04] error: Could not compile `rustc_interface`.
[00:23:04] error: Could not compile `rustc_interface`.
[00:23:04] warning: build failed, waiting for other jobs to finish...
[00:24:18] error: build failed
[00:24:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:24:18] expected success, got: exit code: 101
[00:24:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:24:18] Build completed unsuccessfully in 0:20:53
[00:24:18] make: *** [all] Error 1
[00:24:18] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04bb1051
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 20 16:14:26 UTC 2019
