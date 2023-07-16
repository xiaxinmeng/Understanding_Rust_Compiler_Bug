plain
travis_time:end:05b4342c:start=1548180061259790079,finish=1548180136919798086,duration=75660008007
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:37]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:54] warning: unnecessary `unsafe` block
[00:03:54]    --> src/libcore/num/mod.rs:71:30
[00:03:54]     |
[00:03:54] 33  | / macro_rules! nonzero_integers {
[00:03:54] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:03:54] 36  | |             doc_comment! {
[00:03:54] ...   |
[00:03:54] 71  | |                         Some(unsafe { $Ty(n) })
[00:03:54]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:03:54]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:03:54] ...   |
[00:03:54] 97  | |     }
[00:03:54] 98  | | }
[00:03:54]     | |_- in this expansion of `nonzero_integers!`
[00:03:54] 100 | / nonzero_integers! {
[00:03:54] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:03:54] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:03:54] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
[00:03:54] 
[00:03:54] warning: unnecessary `unsafe` block
[00:03:54]    --> src/libcore/num/mod.rs:71:30
[00:03:54]     |
[00:03:54] 33  | / macro_rules! nonzero_integers {
[00:03:54] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:03:54] 36  | |             doc_comment! {
[00:03:54] ...   |
[00:03:54] 71  | |                         Some(unsafe { $Ty(n) })
[00:03:54]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:03:54]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:03:54] ...   |
[00:03:54] 97  | |     }
[00:03:54] 98  | | }
[00:03:54]     | |_- in this expansion of `nonzero_integers!`
[00:03:54] 100 | / nonzero_integers! {
[00:03:54] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:03:54] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:03:54] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]    --> src/libcore/num/mod.rs:50:17
[00:03:54]     |
[00:03:54] 33  | / macro_rules! nonzero_integers {
[00:03:54] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:03:54] 36  | |             doc_comment! {
[00:03:54] ...   |
[00:03:54] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:03:54]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:03:54]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:03:54] ...   |
[00:03:54] 97  | |     }
[00:03:54] 98  | | }
[00:03:54]     | |_- in this expansion of `nonzero_integers!`
[00:03:54] 100 | / nonzero_integers! {
[00:03:54] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:03:54] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:03:54] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
[00:03:54] 
[00:03:54] warning: unused attribute
[00:03:54]    --> src/libcore/num/mod.rs:50:17
[00:03:54]     |
[00:03:54] 33  | / macro_rules! nonzero_integers {
[00:03:54] 34  | |     ( $( #[$stability: meta] $Ty: ident($Int: ty); )+ ) => {
[00:03:54] 36  | |             doc_comment! {
[00:03:54] ...   |
[00:03:54] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:03:54]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:03:54]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:03:54] ...   |
[00:03:54] 97  | |     }
[00:03:54] 98  | | }
[00:03:54]     | |_- in this expansion of `nonzero_integers!`
[00:03:54] 100 | / nonzero_integers! {
[00:03:54] 101 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU8(u8);
[00:03:54] 102 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU16(u16);
[00:03:54] 103 | |     #[stable(feature = "nonzero", since = "1.28.0")] NonZeroU32(u32);
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:32] 
[01:12:32] running 118 tests
[01:12:57] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:13:01] ......iii.i.....ii
[01:13:01] 
[01:13:01]  finished in 28.927
[01:13:01] travis_fold:end:test_debuginfo

---
[01:35:17]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[01:35:18] error[E0433]: failed to resolve: use of undeclared type or module `HashSet`
[01:35:18]   --> src/librustc_driver/test.rs:98:20
[01:35:18]    |
[01:35:18] 98 |         crate_cfg: HashSet::new(),
[01:35:18]    |                    ^^^^^^^ use of undeclared type or module `HashSet`
[01:35:18] error: unused import: `rustc_metadata::cstore::CStore`
[01:35:18]   --> src/librustc_driver/test.rs:14:5
[01:35:18]    |
[01:35:18] 14 | use rustc_metadata::cstore::CStore;
---
[01:35:18] 
[01:35:18] error[E0308]: mismatched types
[01:35:18]    --> src/librustc_driver/test.rs:104:23
[01:35:18]     |
[01:35:18] 104 |         emitter: Some(emitter),
[01:35:18]     |                       ^^^^^^^ expected trait `std::io::Write`, found trait `errors::emitter::Emitter`
[01:35:18]     = note: expected type `std::boxed::Box<dyn std::io::Write + std::marker::Send>`
[01:35:18]     = note: expected type `std::boxed::Box<dyn std::io::Write + std::marker::Send>`
[01:35:18]                found type `std::boxed::Box<(dyn errors::emitter::Emitter + rustc_data_structures::sync::Send + 'static)>`
[01:35:19] error: aborting due to 5 previous errors
[01:35:19] 
[01:35:19] Some errors occurred: E0308, E0433.
[01:35:19] For more information about an error, try `rustc --explain E0308`.
[01:35:19] For more information about an error, try `rustc --explain E0308`.
[01:35:19] error: Could not compile `rustc_driver`.
[01:35:19] 
[01:35:19] To learn more, run the command again with --verbose.
[01:35:19] 
[01:35:19] 
[01:35:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:35:19] 
[01:35:19] 
[01:35:19] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:19] Build completed unsuccessfully in 0:34:22
[01:35:19] Build completed unsuccessfully in 0:34:22
[01:35:19] Makefile:48: recipe for target 'check' failed
[01:35:19] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:02e40a00
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 22 19:37:44 UTC 2019
---
travis_time:end:0f3784d0:start=1548185866388019179,finish=1548185866441094662,duration=53075483
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0046a2c4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05e85c3d
$ dmesg | grep -i kill
