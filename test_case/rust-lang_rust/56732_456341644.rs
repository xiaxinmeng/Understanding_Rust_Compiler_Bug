plain
travis_time:end:0c9433f8:start=1548145540050404650,finish=1548145542226430952,duration=2176026302
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:20]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:04:36] warning: unnecessary `unsafe` block
[00:04:36]    --> src/libcore/num/mod.rs:71:30
[00:04:36]     |
[00:04:36] 33  | / macro_rules! nonzero_integers {
[00:04:36] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:36] 36  | |             doc_comment! {
[00:04:36] ...   |
[00:04:36] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:36]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:36]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:36] ...   |
[00:04:36] 97  | |     }
[00:04:36] 98  | | }
[00:04:36]     | |_- in this expansion of `nonzero_integers!`
[00:04:36] 100 | / nonzero_integers! {
[00:04:36] 101 | |     NonZeroU8(u8);
[00:04:36] 102 | |     NonZeroU16(u16);
[00:04:36] 103 | |     NonZeroU32(u32);
---
[00:04:36] 
[00:04:36] warning: unnecessary `unsafe` block
[00:04:36]    --> src/libcore/num/mod.rs:71:30
[00:04:36]     |
[00:04:36] 33  | / macro_rules! nonzero_integers {
[00:04:36] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:36] 36  | |             doc_comment! {
[00:04:36] ...   |
[00:04:36] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:36]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:36]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:36] ...   |
[00:04:36] 97  | |     }
[00:04:36] 98  | | }
[00:04:36]     | |_- in this expansion of `nonzero_integers!`
[00:04:36] 100 | / nonzero_integers! {
[00:04:36] 101 | |     NonZeroU8(u8);
[00:04:36] 102 | |     NonZeroU16(u16);
[00:04:36] 103 | |     NonZeroU32(u32);
---
[00:04:36] 
[00:04:37] warning: unused attribute
[00:04:37]    --> src/libcore/num/mod.rs:50:17
[00:04:37]     |
[00:04:37] 33  | / macro_rules! nonzero_integers {
[00:04:37] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:37] 36  | |             doc_comment! {
[00:04:37] ...   |
[00:04:37] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:37]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:37]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:37] ...   |
[00:04:37] 97  | |     }
[00:04:37] 98  | | }
[00:04:37]     | |_- in this expansion of `nonzero_integers!`
[00:04:37] 100 | / nonzero_integers! {
[00:04:37] 101 | |     NonZeroU8(u8);
[00:04:37] 102 | |     NonZeroU16(u16);
[00:04:37] 103 | |     NonZeroU32(u32);
---
[00:04:37] 
[00:04:37] warning: unused attribute
[00:04:37]    --> src/libcore/num/mod.rs:50:17
[00:04:37]     |
[00:04:37] 33  | / macro_rules! nonzero_integers {
[00:04:37] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:37] 36  | |             doc_comment! {
[00:04:37] ...   |
[00:04:37] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:37]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:37]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:37] ...   |
[00:04:37] 97  | |     }
[00:04:37] 98  | | }
[00:04:37]     | |_- in this expansion of `nonzero_integers!`
[00:04:37] 100 | / nonzero_integers! {
[00:04:37] 101 | |     NonZeroU8(u8);
[00:04:37] 102 | |     NonZeroU16(u16);
[00:04:37] 103 | |     NonZeroU32(u32);
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:16:22] 
[01:16:22] running 118 tests
[01:16:47] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:16:51] ......iii.i.....ii
[01:16:51] 
[01:16:51]  finished in 28.803
[01:16:51] travis_fold:end:test_debuginfo

---
[01:38:47]    Compiling rustc v0.0.0 (/checkout/src/librustc)
[01:39:09] error[E0308]: mismatched types
[01:39:09]     --> src/librustc/session/config.rs:2589:50
[01:39:09]      |
[01:39:09] 2589 |             let cfg = build_configuration(&sess, cfg);
[01:39:09]      |                                                  ^^^ expected struct `syntax_pos::symbol::Symbol`, found struct `std::string::String`
[01:39:09]      |
[01:39:09]      = note: expected type `std::collections::HashSet<(syntax_pos::symbol::Symbol, std::option::Option<syntax_pos::symbol::Symbol>), std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[01:39:09]                 found type `std::collections::HashSet<(std::string::String, std::option::Option<std::string::String>), std::collections::hash_map::RandomState>`
[01:39:09] error[E0308]: mismatched types
[01:39:09]     --> src/librustc/session/config.rs:2607:50
[01:39:09]      |
[01:39:09]      |
[01:39:09] 2607 |             let cfg = build_configuration(&sess, cfg);
[01:39:09]      |                                                  ^^^ expected struct `syntax_pos::symbol::Symbol`, found struct `std::string::String`
[01:39:09]      |
[01:39:09]      = note: expected type `std::collections::HashSet<(syntax_pos::symbol::Symbol, std::option::Option<syntax_pos::symbol::Symbol>), std::hash::BuildHasherDefault<rustc_data_structures::fx::FxHasher>>`
[01:39:09]                 found type `std::collections::HashSet<(std::string::String, std::option::Option<std::string::String>), std::collections::hash_map::RandomState>`
[01:39:18] error: aborting due to 2 previous errors
[01:39:18] 
[01:39:18] For more information about this error, try `rustc --explain E0308`.
[01:39:18] error: Could not compile `rustc`.
[01:39:18] error: Could not compile `rustc`.
[01:39:18] 
[01:39:18] To learn more, run the command again with --verbose.
[01:39:18] 
[01:39:18] 
[01:39:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc" "--" "--quiet"
[01:39:18] 
[01:39:18] 
[01:39:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:18] Build completed unsuccessfully in 0:34:59
[01:39:18] Build completed unsuccessfully in 0:34:59
[01:39:18] make: *** [check] Error 1
[01:39:18] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:04117731
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan 22 10:05:12 UTC 2019
---
travis_time:end:0b07b428:start=1548151513550488422,finish=1548151513556065393,duration=5576971
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:19152eb8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
