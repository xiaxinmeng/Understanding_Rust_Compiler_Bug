plain
travis_time:end:0fb9ee44:start=1547971373675295725,finish=1547971374697735046,duration=1022439321
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:02]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:04:16] warning: unnecessary `unsafe` block
[00:04:16]    --> src/libcore/num/mod.rs:71:30
[00:04:16]     |
[00:04:16] 33  | / macro_rules! nonzero_integers {
[00:04:16] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:16] 36  | |             doc_comment! {
[00:04:16] ...   |
[00:04:16] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:16]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:16]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:16] ...   |
[00:04:16] 97  | |     }
[00:04:16] 98  | | }
[00:04:16]     | |_- in this expansion of `nonzero_integers!`
[00:04:16] 100 | / nonzero_integers! {
[00:04:16] 101 | |     NonZeroU8(u8);
[00:04:16] 102 | |     NonZeroU16(u16);
[00:04:16] 103 | |     NonZeroU32(u32);
---
[00:04:16] 
[00:04:16] warning: unnecessary `unsafe` block
[00:04:16]    --> src/libcore/num/mod.rs:71:30
[00:04:16]     |
[00:04:16] 33  | / macro_rules! nonzero_integers {
[00:04:16] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:16] 36  | |             doc_comment! {
[00:04:16] ...   |
[00:04:16] 71  | |                         Some(unsafe { $Ty(n) })
[00:04:16]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:16]     | |                              ^^^^^^ unnecessary `unsafe` block
[00:04:16] ...   |
[00:04:16] 97  | |     }
[00:04:16] 98  | | }
[00:04:16]     | |_- in this expansion of `nonzero_integers!`
[00:04:16] 100 | / nonzero_integers! {
[00:04:16] 101 | |     NonZeroU8(u8);
[00:04:16] 102 | |     NonZeroU16(u16);
[00:04:16] 103 | |     NonZeroU32(u32);
---
[00:04:16] 
[00:04:17] warning: unused attribute
[00:04:17]    --> src/libcore/num/mod.rs:50:17
[00:04:17]     |
[00:04:17] 33  | / macro_rules! nonzero_integers {
[00:04:17] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:17] 36  | |             doc_comment! {
[00:04:17] ...   |
[00:04:17] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:17]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:17]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:17] ...   |
[00:04:17] 97  | |     }
[00:04:17] 98  | | }
[00:04:17]     | |_- in this expansion of `nonzero_integers!`
[00:04:17] 100 | / nonzero_integers! {
[00:04:17] 101 | |     NonZeroU8(u8);
[00:04:17] 102 | |     NonZeroU16(u16);
[00:04:17] 103 | |     NonZeroU32(u32);
---
[00:04:17] 
[00:04:17] warning: unused attribute
[00:04:17]    --> src/libcore/num/mod.rs:50:17
[00:04:17]     |
[00:04:17] 33  | / macro_rules! nonzero_integers {
[00:04:17] 34  | |     ( $( $Ty: ident($Int: ty); )+ ) => {
[00:04:17] 36  | |             doc_comment! {
[00:04:17] ...   |
[00:04:17] 50  | |                 #[rustc_layout_scalar_valid_range_start(1)]
[00:04:17]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:17]     | |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:04:17] ...   |
[00:04:17] 97  | |     }
[00:04:17] 98  | | }
[00:04:17]     | |_- in this expansion of `nonzero_integers!`
[00:04:17] 100 | / nonzero_integers! {
[00:04:17] 101 | |     NonZeroU8(u8);
[00:04:17] 102 | |     NonZeroU16(u16);
[00:04:17] 103 | |     NonZeroU32(u32);
---
[00:05:37]    Compiling parking_lot v0.6.4
[00:05:39]    Compiling rustc-rayon v0.1.1
[00:05:42]    Compiling rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
[00:05:42]    Compiling tempfile v3.0.5
[00:05:42] error: trait objects without an explicit `dyn` are deprecated
[00:05:42]  --> src/librustc_data_structures/box_region.rs:6:17
[00:05:42]   |
[00:05:42] 6 |     Access(*mut FnMut()),
[00:05:42]   |                 ^^^^^^^ help: use `dyn`: `dyn FnMut()`
[00:05:42]   |
[00:05:42]   = note: requested on the command line with `-D bare-trait-objects`
[00:05:42] 
[00:05:42] error: trait objects without an explicit `dyn` are deprecated
[00:05:42]  --> src/librustc_data_structures/box_region.rs:6:17
[00:05:42]   |
[00:05:42] 6 |     Access(*mut FnMut()),
[00:05:42]   |                 ^^^^^^^ help: use `dyn`: `dyn FnMut()`
[00:05:44] error: aborting due to 2 previous errors
[00:05:44] 
[00:05:44] error: Could not compile `rustc_data_structures`.
[00:05:44] 
[00:05:44] 
[00:05:44] To learn more, run the command again with --verbose.
[00:05:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:05:44] expected success, got: exit code: 101
[00:05:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:44] Build completed unsuccessfully in 0:01:54
[00:05:44] make: *** [all] Error 1
[00:05:44] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:03071d74
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Jan 20 08:08:49 UTC 2019
---
travis_time:end:047ff716:start=1547971730459250604,finish=1547971730463448052,duration=4197448
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12a202be
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:23c61630
travis_time:start:23c61630
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:003e6980
$ dmesg | grep -i kill
