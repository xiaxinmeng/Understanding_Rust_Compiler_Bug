plain
2019-07-20T15:06:47.1888553Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T15:06:47.2063382Z ##[command]git config gc.auto 0
2019-07-20T15:06:48.1514372Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T15:06:48.1536906Z ##[command]git config --get-all http.proxy
2019-07-20T15:06:48.1539674Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62823/merge:refs/remotes/pull/62823/merge
---
2019-07-20T15:07:21.3133434Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T15:07:21.3133473Z 
2019-07-20T15:07:21.3133639Z   git checkout -b <new-branch-name>
2019-07-20T15:07:21.3133685Z 
2019-07-20T15:07:21.3133722Z HEAD is now at 24821c584 Merge a6eeedbd0810deec0fcf565cc04e890b8f69218b into f69b07144a151f46aaee1b6230ba4160e9394562
2019-07-20T15:07:21.3273142Z ##[section]Finishing: Checkout
2019-07-20T15:07:21.3280749Z ##[section]Starting: Decide whether to run this job
2019-07-20T15:07:21.3284179Z Task         : Bash
2019-07-20T15:07:21.3284226Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-20T15:07:21.3284275Z Version      : 3.151.2
2019-07-20T15:07:21.3284337Z Author       : Microsoft Corporation
2019-07-20T15:07:21.3284337Z Author       : Microsoft Corporation
2019-07-20T15:07:21.3284387Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-20T15:07:21.3284440Z ==============================================================================
2019-07-20T15:07:21.4593890Z Generating script.
2019-07-20T15:07:21.4628579Z ========================== Starting Command Output ===========================
2019-07-20T15:07:21.4646735Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d6d0802c-bf2a-4072-ae64-2ca76c2080de.sh
2019-07-20T15:07:21.5972234Z Executing the job since submodules are updated
2019-07-20T15:07:21.6067808Z ##[section]Finishing: Decide whether to run this job
2019-07-20T15:07:21.6077877Z ==============================================================================
2019-07-20T15:07:21.6077926Z Task         : Bash
2019-07-20T15:07:21.6077965Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-20T15:07:21.6078041Z Version      : 3.151.2
---
2019-07-20T17:14:00.8347189Z test [compile-fail] compile-fail/cast_fn_ptr5.rs ... ok
2019-07-20T17:14:00.9662016Z test [compile-fail] compile-fail/cast_int_to_fn_ptr.rs ... ok
2019-07-20T17:14:00.9737592Z test [compile-fail] compile-fail/copy_nonoverlapping.rs ... ok
2019-07-20T17:14:01.1064125Z 
2019-07-20T17:14:01.1066994Z error: error pattern ' invalid use of NULL pointer' not found!
2019-07-20T17:14:01.1067236Z status: exit code: 1
2019-07-20T17:14:01.1070346Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/copy_null.rs" "-L" "/tmp/compiletestOYaUHs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestOYaUHs/copy_null.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestOYaUHs/copy_null.stage-id.aux" "-A" "unused"
2019-07-20T17:14:01.1077752Z ------------------------------------------
2019-07-20T17:14:01.1079047Z 
2019-07-20T17:14:01.1079424Z ------------------------------------------
2019-07-20T17:14:01.1079505Z stderr:
2019-07-20T17:14:01.1079505Z stderr:
2019-07-20T17:14:01.1079734Z ------------------------------------------
2019-07-20T17:14:01.1080185Z error[E0080]: Miri evaluation error: the evaluated program panicked at 'attempt to copy from unaligned or null pointer', src/libcore/intrinsics.rs:1504:5
2019-07-20T17:14:01.1080523Z    |
2019-07-20T17:14:01.1080523Z    |
2019-07-20T17:14:01.1080760Z 12 |         $crate::panicking::panic(&($msg, file!(), line!(), __rust_unstable_column!()))
2019-07-20T17:14:01.1084545Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Miri evaluation error: the evaluated program panicked at 'attempt to copy from unaligned or null pointer', src/libcore/intrinsics.rs:1504:5
2019-07-20T17:14:01.1084899Z    |
2019-07-20T17:14:01.1084964Z    = note: inside call to `std::ptr::copy::<u16>` at /checkout/src/libcore/ptr/mod.rs:2285:9
2019-07-20T17:14:01.1085319Z note: inside call to `std::ptr::<impl *mut u16>::copy_from` at tests/compile-fail/copy_null.rs:7:14
2019-07-20T17:14:01.1085573Z    |
2019-07-20T17:14:01.1085573Z    |
2019-07-20T17:14:01.1085611Z 7  |     unsafe { ptr.copy_from(std::ptr::null(), 0); }
2019-07-20T17:14:01.1085649Z    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-20T17:14:01.1085716Z    = note: inside call to `main` at /checkout/src/libstd/rt.rs:64:34
2019-07-20T17:14:01.1085758Z    = note: inside call to closure at /checkout/src/libstd/rt.rs:49:73
2019-07-20T17:14:01.1085801Z    = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:77:5
2019-07-20T17:14:01.1086213Z    = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1:5947 ~ std[e915]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:49:13
2019-07-20T17:14:01.1086439Z    = note: inside call to closure at /checkout/src/libstd/panicking.rs:296:40
2019-07-20T17:14:01.1086815Z    = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1:5946 ~ std[e915]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:292:5
2019-07-20T17:14:01.1087164Z    = note: inside call to `std::panicking::try::<i32, [closure@DefId(1:5946 ~ std[e915]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:394:9
2019-07-20T17:14:01.1087530Z    = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1:5946 ~ std[e915]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:48:25
2019-07-20T17:14:01.1087635Z    = note: inside call to `std::rt::lang_start::<()>`
2019-07-20T17:14:01.1087677Z 
2019-07-20T17:14:01.1087711Z error: aborting due to previous error
2019-07-20T17:14:01.1087734Z 
---
2019-07-20T17:14:01.1089287Z thread '[compile-fail] compile-fail/copy_null.rs' panicked at 'explicit panic', /cargo/registry/src/github.com-1ecc6299db9ec823/compiletest_rs-0.3.22/src/runtest.rs:2632:9
2019-07-20T17:14:01.1089373Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-20T17:14:01.1089697Z test [compile-fail] compile-fail/copy_null.rs ... FAILED
2019-07-20T17:14:01.1093486Z 
2019-07-20T17:14:01.1095053Z error: error pattern ' tried to access memory with alignment 1, but alignment 2 is required' not found!
2019-07-20T17:14:01.1095383Z status: exit code: 1
2019-07-20T17:14:01.1101090Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/miri" "tests/compile-fail/copy_unaligned.rs" "-L" "/tmp/compiletestOYaUHs" "--target=x86_64-unknown-linux-gnu" "-C" "prefer-dynamic" "-o" "/tmp/compiletestOYaUHs/copy_unaligned.stage-id" "-Dwarnings" "-Dunused" "--edition" "2018" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "-L" "/tmp/compiletestOYaUHs/copy_unaligned.stage-id.aux" "-A" "unused"
2019-07-20T17:14:01.1104874Z ------------------------------------------
2019-07-20T17:14:01.1104976Z 
2019-07-20T17:14:01.1105992Z ------------------------------------------
2019-07-20T17:14:01.1106543Z stderr:
2019-07-20T17:14:01.1106543Z stderr:
2019-07-20T17:14:01.1106766Z ------------------------------------------
2019-07-20T17:14:01.1107032Z error[E0080]: Miri evaluation error: the evaluated program panicked at 'attempt to copy to unaligned or null pointer', src/libcore/intrinsics.rs:1505:5
2019-07-20T17:14:01.1107441Z    |
2019-07-20T17:14:01.1107441Z    |
2019-07-20T17:14:01.1107651Z 12 |         $crate::panicking::panic(&($msg, file!(), line!(), __rust_unstable_column!()))
2019-07-20T17:14:01.1111550Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ Miri evaluation error: the evaluated program panicked at 'attempt to copy to unaligned or null pointer', src/libcore/intrinsics.rs:1505:5
2019-07-20T17:14:01.1111915Z    |
2019-07-20T17:14:01.1112558Z    = note: inside call to `std::ptr::copy::<u16>` at /checkout/src/libcore/ptr/mod.rs:2285:9
2019-07-20T17:14:01.1115387Z note: inside call to `std::ptr::<impl *mut u16>::copy_from` at tests/compile-fail/copy_unaligned.rs:7:14
2019-07-20T17:14:01.1116692Z    |
2019-07-20T17:14:01.1116692Z    |
2019-07-20T17:14:01.1116745Z 7  |     unsafe { ptr.copy_from(&data[5], 0); }
2019-07-20T17:14:01.1116782Z    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-20T17:14:01.1116842Z    = note: inside call to `main` at /checkout/src/libstd/rt.rs:64:34
2019-07-20T17:14:01.1116882Z    = note: inside call to closure at /checkout/src/libstd/rt.rs:49:73
2019-07-20T17:14:01.1116924Z    = note: inside call to closure at /checkout/src/libstd/sys_common/backtrace.rs:77:5
2019-07-20T17:14:01.1117668Z    = note: inside call to `std::sys_common::backtrace::__rust_begin_short_backtrace::<[closure@DefId(1:5947 ~ std[e915]::rt[0]::lang_start_internal[0]::{{closure}}[0]::{{closure}}[0]) 0:&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:49:13
2019-07-20T17:14:01.1119106Z    = note: inside call to closure at /checkout/src/libstd/panicking.rs:296:40
2019-07-20T17:14:01.1120779Z    = note: inside call to `std::panicking::try::do_call::<[closure@DefId(1:5946 ~ std[e915]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/panicking.rs:292:5
2019-07-20T17:14:01.1122374Z    = note: inside call to `std::panicking::try::<i32, [closure@DefId(1:5946 ~ std[e915]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe]>` at /checkout/src/libstd/panic.rs:394:9
2019-07-20T17:14:01.1123029Z    = note: inside call to `std::panic::catch_unwind::<[closure@DefId(1:5946 ~ std[e915]::rt[0]::lang_start_internal[0]::{{closure}}[0]) 0:&&dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe], i32>` at /checkout/src/libstd/rt.rs:48:25
2019-07-20T17:14:01.1123321Z    = note: inside call to `std::rt::lang_start::<()>`
2019-07-20T17:14:01.1123366Z 
2019-07-20T17:14:01.1123402Z error: aborting due to previous error
2019-07-20T17:14:01.1123425Z 
---
2019-07-20T17:14:12.3541998Z Verifying status of clippy-driver...
2019-07-20T17:14:12.3551171Z Verifying status of miri...
2019-07-20T17:14:12.3563718Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-07-20T17:14:12.3575134Z 
2019-07-20T17:14:12.3575631Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-07-20T17:14:12.3575665Z 
2019-07-20T17:14:12.3575903Z If you do intend to update 'miri', please check the error messages above and
2019-07-20T17:14:12.3575948Z commit another update.
2019-07-20T17:14:12.3575971Z 
2019-07-20T17:14:12.3576175Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-07-20T17:14:12.3576411Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-07-20T17:14:12.3576453Z proper steps.
2019-07-20T17:14:13.1586284Z ##[error]Bash exited with code '3'.
2019-07-20T17:14:13.1620443Z ##[section]Starting: Checkout
2019-07-20T17:14:13.1622365Z ==============================================================================
2019-07-20T17:14:13.1622592Z Task         : Get sources
2019-07-20T17:14:13.1622648Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
