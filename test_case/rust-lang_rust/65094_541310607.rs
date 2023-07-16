plain
2019-10-12T10:15:41.8139757Z    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2019-10-12T10:15:42.1095001Z [RUSTC-TIMING] panic_unwind test:false 0.291
2019-10-12T10:15:42.6190404Z [RUSTC-TIMING] rustc_demangle test:false 2.044
2019-10-12T10:15:43.0700753Z [RUSTC-TIMING] hashbrown test:false 0.957
2019-10-12T10:15:44.2402550Z error[E0412]: cannot find type `statx_timestamp` in crate `libc`
2019-10-12T10:15:44.2405215Z   --> src/libstd/sys/unix/fs.rs:56:22
2019-10-12T10:15:44.2410789Z    |
2019-10-12T10:15:44.2411157Z 56 |     stx_btime: libc::statx_timestamp,
2019-10-12T10:15:44.2411572Z    |                      ^^^^^^^^^^^^^^^ not found in `libc`
2019-10-12T10:15:44.2411673Z 
2019-10-12T10:15:44.2505029Z error[E0412]: cannot find type `statx` in crate `libc`
2019-10-12T10:15:44.2505508Z   --> src/libstd/sys/unix/fs.rs:79:34
2019-10-12T10:15:44.2506635Z    |
2019-10-12T10:15:44.2507037Z 79 |             statxbuf: *mut libc::statx
2019-10-12T10:15:44.2507491Z    |                                  ^^^^^ help: a struct with a similar name exists: `stat`
2019-10-12T10:15:44.2507586Z 
2019-10-12T10:15:44.2637573Z error[E0425]: cannot find value `SYS_statx` in this scope
2019-10-12T10:15:44.2637986Z    --> src/libstd/sys/unix/weak.rs:95:17
2019-10-12T10:15:44.2638646Z 87  | / macro_rules! syscall {
2019-10-12T10:15:44.2638646Z 87  | / macro_rules! syscall {
2019-10-12T10:15:44.2639603Z 88  | |     (fn $name:ident($($arg_name:ident: $t:ty),*) -> $ret:ty) => (
2019-10-12T10:15:44.2640065Z 89  | |         unsafe fn $name($($arg_name:$t),*) -> $ret {
2019-10-12T10:15:44.2640744Z 90  | |             // This looks like a hack, but concat_idents only accepts idents
2019-10-12T10:15:44.2641097Z ...   |
2019-10-12T10:15:44.2641492Z 95  | |                 concat_idents!(SYS_, $name),
2019-10-12T10:15:44.2642292Z     | |                 |
2019-10-12T10:15:44.2642725Z     | |                 help: a constant with a similar name exists: `SYS_stat`
2019-10-12T10:15:44.2643122Z     | |                 in this macro invocation
2019-10-12T10:15:44.2643564Z ...   |
2019-10-12T10:15:44.2643564Z ...   |
2019-10-12T10:15:44.2643878Z 99  | |     )
2019-10-12T10:15:44.2644213Z 100 | | }
2019-10-12T10:15:44.2644570Z     | |_- in this expansion of `syscall!`
2019-10-12T10:15:44.2645162Z    ::: <::core::macros::builtin::concat_idents macros>:1:1
2019-10-12T10:15:44.2645549Z     |
2019-10-12T10:15:44.2645549Z     |
2019-10-12T10:15:44.2646356Z 1   |   ($ ($ e : ident), +) => ({ }) ; ($ ($ e : ident,) +) => ({ })
2019-10-12T10:15:44.2646871Z     |   ------------------------------------------------------------- in this expansion of `concat_idents!`
2019-10-12T10:15:44.2647507Z    ::: src/libstd/sys/unix/fs.rs:73:5
2019-10-12T10:15:44.2647778Z     |
2019-10-12T10:15:44.2648137Z 73  | /     syscall! {
2019-10-12T10:15:44.2648583Z 74  | |         fn statx(
2019-10-12T10:15:44.2648583Z 74  | |         fn statx(
2019-10-12T10:15:44.2648965Z 75  | |             fd: c_int,
2019-10-12T10:15:44.2649622Z 76  | |             pathname: *const libc::c_char,
2019-10-12T10:15:44.2650252Z 80  | |         ) -> c_int
2019-10-12T10:15:44.2650596Z 81  | |     }
2019-10-12T10:15:44.2651080Z     | |_____- in this macro invocation
2019-10-12T10:15:44.2651170Z 
2019-10-12T10:15:44.2651170Z 
2019-10-12T10:15:44.2732123Z error[E0412]: cannot find type `statx` in crate `libc`
2019-10-12T10:15:44.2733059Z   --> src/libstd/sys/unix/fs.rs:87:24
2019-10-12T10:15:44.2733395Z    |
2019-10-12T10:15:44.2733739Z 87 |     let mut buf: libc::statx = mem::zeroed();
2019-10-12T10:15:44.2734181Z    |                        ^^^^^ help: a struct with a similar name exists: `stat`
2019-10-12T10:15:44.2734279Z 
2019-10-12T10:15:44.2826902Z error[E0425]: cannot find value `STATX_BTIME` in crate `libc`
2019-10-12T10:15:44.2827332Z    --> src/libstd/sys/unix/fs.rs:256:49
2019-10-12T10:15:44.2827657Z     |
2019-10-12T10:15:44.2828039Z 256 |                 return if (ext.stx_mask & libc::STATX_BTIME) != 0 {
2019-10-12T10:15:44.2828844Z     |                                                 ^^^^^^^^^^^ not found in `libc`
2019-10-12T10:15:44.2828964Z 
2019-10-12T10:15:44.2919198Z error[E0425]: cannot find value `AT_STATX_SYNC_AS_STAT` in crate `libc`
2019-10-12T10:15:44.2919654Z    --> src/libstd/sys/unix/fs.rs:420:51
2019-10-12T10:15:44.2919930Z     |
2019-10-12T10:15:44.2920547Z 420 |                 libc::AT_SYMLINK_NOFOLLOW | libc::AT_STATX_SYNC_AS_STAT,
2019-10-12T10:15:44.2920999Z     |                                                   ^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
2019-10-12T10:15:44.2925500Z 
2019-10-12T10:15:44.3010242Z error[E0425]: cannot find value `STATX_ALL` in crate `libc`
2019-10-12T10:15:44.3010673Z    --> src/libstd/sys/unix/fs.rs:421:23
2019-10-12T10:15:44.3010966Z     |
2019-10-12T10:15:44.3011290Z 421 |                 libc::STATX_ALL,
2019-10-12T10:15:44.3011708Z     |                       ^^^^^^^^^ not found in `libc`
2019-10-12T10:15:44.3018603Z 
2019-10-12T10:15:44.3099929Z error[E0425]: cannot find value `AT_STATX_SYNC_AS_STAT` in crate `libc`
2019-10-12T10:15:44.3100346Z    --> src/libstd/sys/unix/fs.rs:644:45
2019-10-12T10:15:44.3100740Z     |
2019-10-12T10:15:44.3101124Z 644 |                 libc::AT_EMPTY_PATH | libc::AT_STATX_SYNC_AS_STAT,
2019-10-12T10:15:44.3101627Z     |                                             ^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
2019-10-12T10:15:44.3107566Z 
2019-10-12T10:15:44.3198784Z error[E0425]: cannot find value `STATX_ALL` in crate `libc`
2019-10-12T10:15:44.3199430Z    --> src/libstd/sys/unix/fs.rs:645:23
2019-10-12T10:15:44.3199702Z     |
2019-10-12T10:15:44.3200009Z 645 |                 libc::STATX_ALL,
2019-10-12T10:15:44.3200543Z     |                       ^^^^^^^^^ not found in `libc`
2019-10-12T10:15:44.3206843Z 
2019-10-12T10:15:44.3290737Z error[E0425]: cannot find value `AT_STATX_SYNC_AS_STAT` in crate `libc`
2019-10-12T10:15:44.3291501Z    --> src/libstd/sys/unix/fs.rs:938:19
2019-10-12T10:15:44.3292034Z     |
2019-10-12T10:15:44.3292424Z 938 |             libc::AT_STATX_SYNC_AS_STAT,
2019-10-12T10:15:44.3292820Z     |                   ^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
2019-10-12T10:15:44.3297953Z 
2019-10-12T10:15:44.3388709Z error[E0425]: cannot find value `STATX_ALL` in crate `libc`
2019-10-12T10:15:44.3389159Z    --> src/libstd/sys/unix/fs.rs:939:19
2019-10-12T10:15:44.3389452Z     |
2019-10-12T10:15:44.3389779Z 939 |             libc::STATX_ALL,
2019-10-12T10:15:44.3390187Z     |                   ^^^^^^^^^ not found in `libc`
2019-10-12T10:15:44.9077796Z 
2019-10-12T10:15:44.9078781Z error[E0425]: cannot find value `AT_STATX_SYNC_AS_STAT` in crate `libc`
2019-10-12T10:15:44.9079188Z    --> src/libstd/sys/unix/fs.rs:960:47
2019-10-12T10:15:44.9082872Z     |
2019-10-12T10:15:44.9083293Z 960 |             libc::AT_SYMLINK_NOFOLLOW | libc::AT_STATX_SYNC_AS_STAT,
2019-10-12T10:15:44.9083778Z     |                                               ^^^^^^^^^^^^^^^^^^^^^ not found in `libc`
2019-10-12T10:15:44.9083898Z 
2019-10-12T10:15:44.9084210Z error[E0425]: cannot find value `STATX_ALL` in crate `libc`
2019-10-12T10:15:44.9084546Z    --> src/libstd/sys/unix/fs.rs:961:19
2019-10-12T10:15:44.9084841Z     |
2019-10-12T10:15:44.9085181Z 961 |             libc::STATX_ALL,
2019-10-12T10:15:44.9085592Z     |                   ^^^^^^^^^ not found in `libc`
2019-10-12T10:15:47.1237244Z error: aborting due to 13 previous errors
2019-10-12T10:15:47.1237975Z 
2019-10-12T10:15:47.1238533Z Some errors have detailed explanations: E0412, E0425.
2019-10-12T10:15:47.1239043Z For more information about an error, try `rustc --explain E0412`.
---
2019-10-12T10:15:47.2103764Z == clock drift check ==
2019-10-12T10:15:47.2123885Z   local time: Sat Oct 12 10:15:47 UTC 2019
2019-10-12T10:15:47.3016761Z   network time: Sat, 12 Oct 2019 10:15:47 GMT
2019-10-12T10:15:47.3019886Z == end clock drift check ==
2019-10-12T10:15:48.9369900Z ##[error]Bash exited with code '1'.
2019-10-12T10:15:48.9423275Z ##[section]Starting: Upload CPU usage statistics
2019-10-12T10:15:48.9438825Z ==============================================================================
2019-10-12T10:15:48.9438933Z Task         : Bash
2019-10-12T10:15:48.9439023Z Description  : Run a Bash script on macOS, Linux, or Windows
