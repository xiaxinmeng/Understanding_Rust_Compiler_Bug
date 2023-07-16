plain
travis_time:end:00ebbeb8:start=1548061327791666416,finish=1548061330354665378,duration=2562998962
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:12]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:28] warning: unnecessary `unsafe` block
[00:04:28]    --> src/libcore/num/mod.rs:71:30
[00:04:28]     |
[00:04:28] 33  | / macro_rules! nonzero_integers {
[00:04:28] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:28] 36  | |             doc_comment! {
[00:04:28] ...   |
[00:04:28] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:28]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:28]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:28] ...   |
[00:04:28] 97  | |     }
[00:04:28] 98  | | }
[00:04:28]     | |_- in this expansion of `nonzero_integers!`
[00:04:28] 100 | / nonzero_integers! {
[00:04:28] 101 | |     NonZeroU8(u8);
[00:04:28] 102 | |     NonZeroU16(u16);
[00:04:28] 103 | |     NonZeroU32(u32);
---
[00:04:28] 
[00:04:28] warning: unnecessary `unsafe` block
[00:04:28]    --> src/libcore/num/mod.rs:71:30
[00:04:28]     |
[00:04:28] 33  | / macro_rules! nonzero_integers {
[00:04:28] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:28] 36  | |             doc_comment! {
[00:04:28] ...   |
[00:04:28] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:28]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:28]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:28] ...   |
[00:04:28] 97  | |     }
[00:04:28] 98  | | }
[00:04:28]     | |_- in this expansion of `nonzero_integers!`
[00:04:28] 100 | / nonzero_integers! {
[00:04:28] 101 | |     NonZeroU8(u8);
[00:04:28] 102 | |     NonZeroU16(u16);
[00:04:28] 103 | |     NonZeroU32(u32);
---
[00:04:28] 
[00:04:28] warning: unused attribute
[00:04:28]    --> src/libcore/num/mod.rs:50:17
[00:04:28]     |
[00:04:28] 33  | / macro_rules! nonzero_integers {
[00:04:28] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:28] 36  | |             doc_comment! {
[00:04:28] ...   |
[00:04:28] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:28]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:28]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:28] ...   |
[00:04:28] 97  | |     }
[00:04:28] 98  | | }
[00:04:28]     | |_- in this expansion of `nonzero_integers!`
[00:04:28] 100 | / nonzero_integers! {
[00:04:28] 101 | |     NonZeroU8(u8);
[00:04:28] 102 | |     NonZeroU16(u16);
[00:04:28] 103 | |     NonZeroU32(u32);
---
[00:04:28] 
[00:04:28] warning: unused attribute
[00:04:28]    --> src/libcore/num/mod.rs:50:17
[00:04:28]     |
[00:04:28] 33  | / macro_rules! nonzero_integers {
[00:04:28] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:28] 36  | |             doc_comment! {
[00:04:28] ...   |
[00:04:28] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:28]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:28]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:28] ...   |
[00:04:28] 97  | |     }
[00:04:28] 98  | | }
[00:04:28]     | |_- in this expansion of `nonzero_integers!`
[00:04:28] 100 | / nonzero_integers! {
[00:04:28] 101 | |     NonZeroU8(u8);
[00:04:28] 102 | |     NonZeroU16(u16);
[00:04:28] 103 | |     NonZeroU32(u32);
---
[00:54:06]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:54:08] error[E0107]: wrong number of lifetime arguments: expected 1, found 3
[00:54:08]   --> src/librustdoc/passes/check_code_block_syntax.rs:22:28
[00:54:08]    |
[00:54:08] 22 |     cx: &'a DocContext<'a, 'tcx, 'rcx>,
[00:54:08]    |                            ^^^^  ^^^^ unexpected lifetime argument
[00:54:08]    |                            unexpected lifetime argument
[00:54:08] 
[00:54:08] error: aborting due to previous error
[00:54:08] 
---
[00:54:08] 
[00:54:08] 
[00:54:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:54:08] Build completed unsuccessfully in 0:50:09
[00:54:08] Makefile:18: recipe for target 'all' failed
[00:54:08] make: *** [all] Error 1
1743320 ./obj
1743280 ./obj/build
1131084 ./src
1078164 ./obj/build/x86_64-unknown-linux-gnu
---
176340 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc
163004 ./obj/build/bootstrap/debug/incremental
153280 ./src/tools/clang
146884 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405
146880 ./obj/build/bootstrap/debug/incremental/bootstrap-1o7ipylf5x405/s-f8qpmp28c7-1dbnqyb-3v9gdvsuxn2fo
139724 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
137360 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps
115328 ./src/llvm/test/CodeGen
110548 ./obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib
---
travis_fold:end:after_failure.1
travis_fold:start:after_failure.2
travis_time:start:1552a991
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access '/home/travis/Library/Lo: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05cdbb50
$ dmesg | grep -i kill
