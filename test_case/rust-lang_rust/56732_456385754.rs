plain
travis_time:end:0015dcaa:start=1548155298043218202,finish=1548155300191523412,duration=2148305210
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:49]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:04:05] warning: unnecessary `unsafe` block
[00:04:05]    --> src/libcore/num/mod.rs:71:30
[00:04:05]     |
[00:04:05] 33  | / macro_rules! nonzero_integers {
[00:04:05] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:04:05] 36  | |             doc_comment! {
[00:04:05] ...   |
[00:04:05] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:05]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:05]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:05] ...   |
[00:04:05] 97  | |     }
[00:04:05] 98  | | }
[00:04:05]     | |_- in this expansion of `nonzero_integers!`
[00:04:05] 100 | / nonzero_integers! {
[00:04:05] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:05] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:05] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
[00:04:05] 
[00:04:05] warning: unnecessary `unsafe` block
[00:04:05]    --> src/libcore/num/mod.rs:71:30
[00:04:05]     |
[00:04:05] 33  | / macro_rules! nonzero_integers {
[00:04:05] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:04:05] 36  | |             doc_comment! {
[00:04:05] ...   |
[00:04:05] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:05]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:05]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:05] ...   |
[00:04:05] 97  | |     }
[00:04:05] 98  | | }
[00:04:05]     | |_- in this expansion of `nonzero_integers!`
[00:04:05] 100 | / nonzero_integers! {
[00:04:05] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:05] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:05] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]    --> src/libcore/num/mod.rs:50:17
[00:04:05]     |
[00:04:05] 33  | / macro_rules! nonzero_integers {
[00:04:05] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:04:05] 36  | |             doc_comment! {
[00:04:05] ...   |
[00:04:05] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:05]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:05]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:05] ...   |
[00:04:05] 97  | |     }
[00:04:05] 98  | | }
[00:04:05]     | |_- in this expansion of `nonzero_integers!`
[00:04:05] 100 | / nonzero_integers! {
[00:04:05] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:05] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:05] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
[00:04:05] 
[00:04:05] warning: unused attribute
[00:04:05]    --> src/libcore/num/mod.rs:50:17
[00:04:05]     |
[00:04:05] 33  | / macro_rules! nonzero_integers {
[00:04:05] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:04:05] 36  | |             doc_comment! {
[00:04:05] ...   |
[00:04:05] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:05]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:05]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:05] ...   |
[00:04:05] 97  | |     }
[00:04:05] 98  | | }
[00:04:05]     | |_- in this expansion of `nonzero_integers!`
[00:04:05] 100 | / nonzero_integers! {
[00:04:05] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:04:05] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:04:05] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:10:23] 
[01:10:23] running 118 tests
[01:10:47] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:10:51] ......iii.i.....ii
[01:10:51] 
[01:10:51]  finished in 28.492
[01:10:51] travis_fold:end:test_debuginfo

---
[01:33:19] travis_fold:start:test_stage1-rustc_driver
travis_time:start:test_stage1-rustc_driver
Testing rustc_driver stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:33:20]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[01:33:20] error[E0432]: unresolved import `driver`
[01:33:20]  --> src/librustc_driver/test.rs:3:5
[01:33:20] 3 | use driver;
[01:33:20]   |     ^^^^^^ no `driver` in the root
[01:33:20] 
[01:33:20] error[E0425]: cannot find function `get_codegen_backend` in the crate root
[01:33:20] error[E0425]: cannot find function `get_codegen_backend` in the crate root
[01:33:20]    --> src/librustc_driver/test.rs:115:32
[01:33:20]     |
[01:33:20] 115 |     let cstore = CStore::new(::get_codegen_backend(&sess).metadata_loader());
[01:33:20]     |                                ^^^^^^^^^^^^^^^^^^^ not found in the crate root
[01:33:20]     |
[01:33:20] 3   | use rustc_interface::util::get_codegen_backend;
[01:33:20]     |
[01:33:20] 
[01:33:20] 
[01:33:21] error[E0599]: no function or associated item named `create_and_enter` found for type `rustc::ty::TyCtxt<'_, '_, '_>` in the current scope
[01:33:21]    --> src/librustc_driver/test.rs:152:13
[01:33:21] 152 |     TyCtxt::create_and_enter(
[01:33:21]     |     --------^^^^^^^^^^^^^^^^
[01:33:21]     |     |
[01:33:21]     |     function or associated item not found in `rustc::ty::TyCtxt<'_, '_, '_>`
---
[01:33:21] 
[01:33:21] To learn more, run the command again with --verbose.
[01:33:21] 
[01:33:21] 
[01:33:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:33:21] 
[01:33:21] 
[01:33:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:33:21] Build completed unsuccessfully in 0:34:36
[01:33:21] Build completed unsuccessfully in 0:34:36
[01:33:21] make: *** [check] Error 1
[01:33:21] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:045777d8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 22 12:41:51 UTC 2019
---
travis_time:end:05102f2b:start=1548160912801045886,finish=1548160912805487662,duration=4441776
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:027f5132
$ ln -s . checkout && for CORE in obj/cores/core.*; do E
