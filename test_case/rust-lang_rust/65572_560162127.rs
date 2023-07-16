plain
2019-12-01T20:23:20.6698701Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T20:23:20.6908222Z ##[command]git config gc.auto 0
2019-12-01T20:23:20.6991114Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T20:23:20.7049841Z ##[command]git config --get-all http.proxy
2019-12-01T20:23:20.7195252Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65572/merge:refs/remotes/pull/65572/merge
---
2019-12-01T21:22:13.5658576Z .................................................................................................... 1300/9316
2019-12-01T21:22:20.2349803Z .................................................................................................... 1400/9316
2019-12-01T21:22:26.0020741Z ................................................................................F................... 1500/9316
2019-12-01T21:22:32.0637389Z .................................................................................................... 1600/9316
2019-12-01T21:22:36.5281711Z ....................................F.F..FFFF......F................................................ 1700/9316
2019-12-01T21:22:56.4097913Z .................................................................................................... 1900/9316
2019-12-01T21:22:56.4097913Z .................................................................................................... 1900/9316
2019-12-01T21:23:09.9692122Z .........................iiiii...................................................................... 2000/9316
2019-12-01T21:23:19.9026247Z .................................................................................................... 2200/9316
2019-12-01T21:23:22.4037287Z .................................................................................................... 2300/9316
2019-12-01T21:23:26.7888222Z .................................................................................................... 2400/9316
2019-12-01T21:23:48.0713061Z .................................................................................................... 2500/9316
---
2019-12-01T21:26:25.4581273Z ...........................i...............i........................................................ 4800/9316
2019-12-01T21:26:35.7726986Z .................................................................................................... 4900/9316
2019-12-01T21:26:41.9109452Z .................................................................................................... 5000/9316
2019-12-01T21:26:49.9628103Z .................................................................................................... 5100/9316
2019-12-01T21:26:57.4492892Z .................................ii.ii...........i.................................................. 5200/9316
2019-12-01T21:27:06.7500496Z .................................................................................................... 5400/9316
2019-12-01T21:27:16.6829769Z .................................................................................................... 5500/9316
2019-12-01T21:27:23.8919672Z ...............i.................................................................................... 5600/9316
2019-12-01T21:27:29.8708590Z .................................................................................................... 5700/9316
2019-12-01T21:27:29.8708590Z .................................................................................................... 5700/9316
2019-12-01T21:27:40.9793826Z .................................................................................................... 5800/9316
2019-12-01T21:27:52.8850050Z .ii...i...ii..........i............................................................................. 5900/9316
2019-12-01T21:28:10.8241023Z .................................................................................................... 6100/9316
2019-12-01T21:28:14.5281435Z .................................................................................................... 6200/9316
2019-12-01T21:28:14.5281435Z .................................................................................................... 6200/9316
2019-12-01T21:28:28.2355591Z ........................i..ii....................................................................... 6300/9316
2019-12-01T21:28:48.0248536Z ...............................................................................................i.... 6500/9316
2019-12-01T21:28:50.1175631Z .................................................................................................... 6600/9316
2019-12-01T21:28:52.2411724Z ......................................................................................i............. 6700/9316
2019-12-01T21:28:54.8370519Z .................................................................................................... 6800/9316
---
2019-12-01T21:33:41.4040310Z 10 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4042214Z -   --> $DIR/const-extern-fn-min-const-fn.rs:5:41
2019-12-01T21:33:41.4043806Z +   --> $DIR/const-extern-fn-min-const-fn.rs:5:1
2019-12-01T21:33:41.4044968Z 12    |
2019-12-01T21:33:41.4046888Z 13 LL | const unsafe extern "C" fn closure() -> fn() { || {} }
2019-12-01T21:33:41.4049763Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T21:33:41.4050913Z 15    |
2019-12-01T21:33:41.4050913Z 15    |
2019-12-01T21:33:41.4051567Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4055886Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4057172Z 
2019-12-01T21:33:41.4057398Z The actual stderr differed from the expected stderr.
2019-12-01T21:33:41.4058111Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn/const-extern-fn-min-const-fn.stderr
2019-12-01T21:33:41.4058111Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn/const-extern-fn-min-const-fn.stderr
2019-12-01T21:33:41.4059509Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T21:33:41.4061078Z To only update this specific test, also pass `--test-args consts/const-extern-fn/const-extern-fn-min-const-fn.rs`
2019-12-01T21:33:41.4061474Z error: 1 errors occurred comparing output.
2019-12-01T21:33:41.4061595Z status: exit code: 1
2019-12-01T21:33:41.4061595Z status: exit code: 1
2019-12-01T21:33:41.4062833Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn/auxiliary" "-A" "unused"
2019-12-01T21:33:41.4063600Z ------------------------------------------
2019-12-01T21:33:41.4063745Z 
2019-12-01T21:33:41.4064066Z ------------------------------------------
2019-12-01T21:33:41.4064212Z stderr:
2019-12-01T21:33:41.4064212Z stderr:
2019-12-01T21:33:41.4064490Z ------------------------------------------
2019-12-01T21:33:41.4064666Z error[E0723]: unsizing casts are not allowed in const fn
2019-12-01T21:33:41.4065654Z   --> /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs:3:48
2019-12-01T21:33:41.4065944Z    |
2019-12-01T21:33:41.4066323Z LL | const extern fn unsize(x: &[u8; 3]) -> &[u8] { x }
2019-12-01T21:33:41.4066667Z    |
2019-12-01T21:33:41.4066667Z    |
2019-12-01T21:33:41.4067113Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4067333Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4067631Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4068037Z   --> /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs:5:1
2019-12-01T21:33:41.4068236Z    |
2019-12-01T21:33:41.4068236Z    |
2019-12-01T21:33:41.4068593Z LL | const unsafe extern "C" fn closure() -> fn() { || {} }
2019-12-01T21:33:41.4069070Z    |
2019-12-01T21:33:41.4069070Z    |
2019-12-01T21:33:41.4069595Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4069778Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4069885Z 
2019-12-01T21:33:41.4070001Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-12-01T21:33:41.4070504Z    |
2019-12-01T21:33:41.4070504Z    |
2019-12-01T21:33:41.4070630Z LL | const unsafe extern fn use_float() { 1.0 + 1.0; }
2019-12-01T21:33:41.4070883Z    |
2019-12-01T21:33:41.4070883Z    |
2019-12-01T21:33:41.4071250Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4071434Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4071671Z error[E0723]: casting pointers to ints is unstable in const fn
2019-12-01T21:33:41.4072023Z   --> /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs:9:48
2019-12-01T21:33:41.4072174Z    |
2019-12-01T21:33:41.4072174Z    |
2019-12-01T21:33:41.4072309Z LL | const extern "C" fn ptr_cast(val: *const u8) { val as usize; }
2019-12-01T21:33:41.4072541Z    |
2019-12-01T21:33:41.4072541Z    |
2019-12-01T21:33:41.4072905Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4073079Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4073830Z error: aborting due to 4 previous errors
2019-12-01T21:33:41.4073941Z 
2019-12-01T21:33:41.4074310Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T21:33:41.4074472Z 
---
2019-12-01T21:33:41.4076500Z 1 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4076880Z -   --> $DIR/allow_const_fn_ptr.rs:4:16
2019-12-01T21:33:41.4077372Z +   --> $DIR/allow_const_fn_ptr.rs:4:25
2019-12-01T21:33:41.4077556Z 3    |
2019-12-01T21:33:41.4077695Z 4 LL | const fn error(_: fn()) {}
2019-12-01T21:33:41.4078212Z +    |                         ^
2019-12-01T21:33:41.4078450Z 6    |
2019-12-01T21:33:41.4078450Z 6    |
2019-12-01T21:33:41.4079355Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4079728Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4080192Z 
2019-12-01T21:33:41.4080330Z The actual stderr differed from the expected stderr.
2019-12-01T21:33:41.4080924Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/allow_const_fn_ptr.stderr
2019-12-01T21:33:41.4080924Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/allow_const_fn_ptr.stderr
2019-12-01T21:33:41.4081609Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T21:33:41.4082179Z To only update this specific test, also pass `--test-args consts/min_const_fn/allow_const_fn_ptr.rs`
2019-12-01T21:33:41.4082812Z error: 1 errors occurred comparing output.
2019-12-01T21:33:41.4083253Z status: exit code: 1
2019-12-01T21:33:41.4083253Z status: exit code: 1
2019-12-01T21:33:41.4085000Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/auxiliary" "-A" "unused"
2019-12-01T21:33:41.4086477Z ------------------------------------------
2019-12-01T21:33:41.4086723Z 
2019-12-01T21:33:41.4087101Z ------------------------------------------
2019-12-01T21:33:41.4087302Z stderr:
2019-12-01T21:33:41.4087302Z stderr:
2019-12-01T21:33:41.4087837Z ------------------------------------------
2019-12-01T21:33:41.4088058Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4088654Z   --> /checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr.rs:4:25
2019-12-01T21:33:41.4089009Z    |
2019-12-01T21:33:41.4089306Z LL | const fn error(_: fn()) {} //~ ERROR function pointers in const fn are unstable
2019-12-01T21:33:41.4089735Z    |
2019-12-01T21:33:41.4089735Z    |
2019-12-01T21:33:41.4090140Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4090312Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4090560Z error: aborting due to previous error
2019-12-01T21:33:41.4090660Z 
2019-12-01T21:33:41.4090983Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T21:33:41.4091121Z 
---
2019-12-01T21:33:41.4092628Z 10 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4092916Z -   --> $DIR/cast_errors.rs:5:23
2019-12-01T21:33:41.4093254Z +   --> $DIR/cast_errors.rs:5:1
2019-12-01T21:33:41.4093404Z 12    |
2019-12-01T21:33:41.4093717Z 13 LL | const fn closure() -> fn() { || {} }
2019-12-01T21:33:41.4119456Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T21:33:41.4119523Z 15    |
2019-12-01T21:33:41.4119523Z 15    |
2019-12-01T21:33:41.4120082Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4120159Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4120236Z 18 
2019-12-01T21:33:41.4120443Z 19 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4120834Z -   --> $DIR/cast_errors.rs:8:5
2019-12-01T21:33:41.4121447Z +   --> $DIR/cast_errors.rs:7:21
2019-12-01T21:33:41.4121447Z +   --> $DIR/cast_errors.rs:7:21
2019-12-01T21:33:41.4121494Z 21    |
2019-12-01T21:33:41.4121915Z - LL |     (|| {}) as fn();
2019-12-01T21:33:41.4122123Z + LL | const fn closure2() {
2019-12-01T21:33:41.4122160Z +    |                     ^
2019-12-01T21:33:41.4122209Z 24    |
2019-12-01T21:33:41.4122209Z 24    |
2019-12-01T21:33:41.4122460Z 25    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4122526Z 26    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4122596Z 27 
2019-12-01T21:33:41.4122634Z 28 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4122843Z -   --> $DIR/cast_errors.rs:11:28
2019-12-01T21:33:41.4123018Z +   --> $DIR/cast_errors.rs:11:1
2019-12-01T21:33:41.4123018Z +   --> $DIR/cast_errors.rs:11:1
2019-12-01T21:33:41.4123056Z 30    |
2019-12-01T21:33:41.4123257Z 31 LL | const fn reify(f: fn()) -> unsafe fn() { f }
2019-12-01T21:33:41.4123498Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T21:33:41.4123534Z 33    |
2019-12-01T21:33:41.4123534Z 33    |
2019-12-01T21:33:41.4123796Z 34    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4123845Z 35    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4123921Z 36 
2019-12-01T21:33:41.4123959Z 37 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4124142Z -   --> $DIR/cast_errors.rs:13:21
2019-12-01T21:33:41.4124344Z +   --> $DIR/cast_errors.rs:13:19
2019-12-01T21:33:41.4124344Z +   --> $DIR/cast_errors.rs:13:19
2019-12-01T21:33:41.4124383Z 39    |
2019-12-01T21:33:41.4124420Z 40 LL | const fn reify2() { main as unsafe fn(); }
2019-12-01T21:33:41.4124660Z +    |                   ^
2019-12-01T21:33:41.4124694Z 42    |
2019-12-01T21:33:41.4124694Z 42    |
2019-12-01T21:33:41.4125558Z 43    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4125634Z 44    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4125694Z 
2019-12-01T21:33:41.4125756Z The actual stderr differed from the expected stderr.
2019-12-01T21:33:41.4126104Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cast_errors/cast_errors.stderr
2019-12-01T21:33:41.4126104Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cast_errors/cast_errors.stderr
2019-12-01T21:33:41.4126352Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T21:33:41.4126642Z To only update this specific test, also pass `--test-args consts/min_const_fn/cast_errors.rs`
2019-12-01T21:33:41.4126730Z error: 1 errors occurred comparing output.
2019-12-01T21:33:41.4126776Z status: exit code: 1
2019-12-01T21:33:41.4126776Z status: exit code: 1
2019-12-01T21:33:41.4127594Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/cast_errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cast_errors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cast_errors/auxiliary" "-A" "unused"
2019-12-01T21:33:41.4127922Z ------------------------------------------
2019-12-01T21:33:41.4127957Z 
2019-12-01T21:33:41.4128171Z ------------------------------------------
2019-12-01T21:33:41.4128354Z stderr:
2019-12-01T21:33:41.4128354Z stderr:
2019-12-01T21:33:41.4128591Z ------------------------------------------
2019-12-01T21:33:41.4128642Z error[E0723]: unsizing casts are not allowed in const fn
2019-12-01T21:33:41.4129253Z   --> /checkout/src/test/ui/consts/min_const_fn/cast_errors.rs:3:41
2019-12-01T21:33:41.4129301Z    |
2019-12-01T21:33:41.4129868Z LL | const fn unsize(x: &[u8; 3]) -> &[u8] { x }
2019-12-01T21:33:41.4130023Z    |
2019-12-01T21:33:41.4130023Z    |
2019-12-01T21:33:41.4130296Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4130360Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4130424Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4130652Z   --> /checkout/src/test/ui/consts/min_const_fn/cast_errors.rs:5:1
2019-12-01T21:33:41.4130694Z    |
2019-12-01T21:33:41.4130694Z    |
2019-12-01T21:33:41.4130875Z LL | const fn closure() -> fn() { || {} }
2019-12-01T21:33:41.4130976Z    |
2019-12-01T21:33:41.4130976Z    |
2019-12-01T21:33:41.4131218Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4131282Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4131347Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4131567Z   --> /checkout/src/test/ui/consts/min_const_fn/cast_errors.rs:7:21
2019-12-01T21:33:41.4131624Z    |
2019-12-01T21:33:41.4131660Z LL | const fn closure2() {
2019-12-01T21:33:41.4131660Z LL | const fn closure2() {
2019-12-01T21:33:41.4131696Z    |                     ^
2019-12-01T21:33:41.4131747Z    |
2019-12-01T21:33:41.4131983Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4132032Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4132111Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4132332Z   --> /checkout/src/test/ui/consts/min_const_fn/cast_errors.rs:11:1
2019-12-01T21:33:41.4132372Z    |
2019-12-01T21:33:41.4132372Z    |
2019-12-01T21:33:41.4132573Z LL | const fn reify(f: fn()) -> unsafe fn() { f }
2019-12-01T21:33:41.4132651Z    |
2019-12-01T21:33:41.4132651Z    |
2019-12-01T21:33:41.4132903Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4132955Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4133048Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4133292Z   --> /checkout/src/test/ui/consts/min_const_fn/cast_errors.rs:13:19
2019-12-01T21:33:41.4133335Z    |
2019-12-01T21:33:41.4133335Z    |
2019-12-01T21:33:41.4133391Z LL | const fn reify2() { main as unsafe fn(); }
2019-12-01T21:33:41.4133470Z    |
2019-12-01T21:33:41.4133470Z    |
2019-12-01T21:33:41.4133746Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4133805Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4133871Z error: aborting due to 5 previous errors
2019-12-01T21:33:41.4133914Z 
2019-12-01T21:33:41.4134145Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T21:33:41.4134176Z 
---
2019-12-01T21:33:41.4134993Z 1 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4135587Z -   --> $DIR/cmp_fn_pointers.rs:1:14
2019-12-01T21:33:41.4135830Z +   --> $DIR/cmp_fn_pointers.rs:1:35
2019-12-01T21:33:41.4135879Z 3    |
2019-12-01T21:33:41.4136103Z 4 LL | const fn cmp(x: fn(), y: fn()) -> bool {
2019-12-01T21:33:41.4136482Z +    |                                   ^^^^
2019-12-01T21:33:41.4136525Z 6    |
2019-12-01T21:33:41.4136525Z 6    |
2019-12-01T21:33:41.4136850Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4136927Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4136990Z 
2019-12-01T21:33:41.4137094Z The actual stderr differed from the expected stderr.
2019-12-01T21:33:41.4137500Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cmp_fn_pointers/cmp_fn_pointers.stderr
2019-12-01T21:33:41.4137500Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cmp_fn_pointers/cmp_fn_pointers.stderr
2019-12-01T21:33:41.4137786Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T21:33:41.4138084Z To only update this specific test, also pass `--test-args consts/min_const_fn/cmp_fn_pointers.rs`
2019-12-01T21:33:41.4138166Z error: 1 errors occurred comparing output.
2019-12-01T21:33:41.4138226Z status: exit code: 1
2019-12-01T21:33:41.4138226Z status: exit code: 1
2019-12-01T21:33:41.4139122Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/cmp_fn_pointers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cmp_fn_pointers" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cmp_fn_pointers/auxiliary" "-A" "unused"
2019-12-01T21:33:41.4139411Z ------------------------------------------
2019-12-01T21:33:41.4139441Z 
2019-12-01T21:33:41.4139636Z ------------------------------------------
2019-12-01T21:33:41.4139676Z stderr:
2019-12-01T21:33:41.4139676Z stderr:
2019-12-01T21:33:41.4139853Z ------------------------------------------
2019-12-01T21:33:41.4139896Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4140135Z   --> /checkout/src/test/ui/consts/min_const_fn/cmp_fn_pointers.rs:1:35
2019-12-01T21:33:41.4140178Z    |
2019-12-01T21:33:41.4140402Z LL | const fn cmp(x: fn(), y: fn()) -> bool { //~ ERROR function pointers in const fn are unstable
2019-12-01T21:33:41.4140505Z    |
2019-12-01T21:33:41.4140505Z    |
2019-12-01T21:33:41.4140766Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4140823Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4140887Z error: aborting due to previous error
2019-12-01T21:33:41.4140926Z 
2019-12-01T21:33:41.4141139Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T21:33:41.4141170Z 
---
2019-12-01T21:33:41.4141741Z 1 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4141924Z -   --> $DIR/min_const_fn_dyn.rs:9:5
2019-12-01T21:33:41.4142117Z +   --> $DIR/min_const_fn_dyn.rs:8:30
2019-12-01T21:33:41.4142155Z 3    |
2019-12-01T21:33:41.4142320Z - LL |     x.0.field;
2019-12-01T21:33:41.4142483Z -    |     ^^^^^^^^^
2019-12-01T21:33:41.4142546Z + LL | const fn no_inner_dyn_trait2(x: Hide) {
2019-12-01T21:33:41.4142621Z 6    |
2019-12-01T21:33:41.4142621Z 6    |
2019-12-01T21:33:41.4142880Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4142930Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4143005Z 9 
2019-12-01T21:33:41.4143047Z 10 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4143341Z -   --> $DIR/min_const_fn_dyn.rs:12:66
2019-12-01T21:33:41.4143539Z +   --> $DIR/min_const_fn_dyn.rs:12:50
2019-12-01T21:33:41.4143539Z +   --> $DIR/min_const_fn_dyn.rs:12:50
2019-12-01T21:33:41.4143579Z 12    |
2019-12-01T21:33:41.4143795Z 13 LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
2019-12-01T21:33:41.4144069Z +    |                                                  ^^^^^^^^^^^^^^^^^^^^
2019-12-01T21:33:41.4144107Z 15    |
2019-12-01T21:33:41.4144107Z 15    |
2019-12-01T21:33:41.4144436Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4144494Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4144542Z 
2019-12-01T21:33:41.4144596Z The actual stderr differed from the expected stderr.
2019-12-01T21:33:41.4144892Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn/min_const_fn_dyn.stderr
2019-12-01T21:33:41.4144892Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn/min_const_fn_dyn.stderr
2019-12-01T21:33:41.4145586Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T21:33:41.4145899Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn_dyn.rs`
2019-12-01T21:33:41.4145982Z error: 1 errors occurred comparing output.
2019-12-01T21:33:41.4146041Z status: exit code: 1
2019-12-01T21:33:41.4146041Z status: exit code: 1
2019-12-01T21:33:41.4146851Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn/auxiliary" "-A" "unused"
2019-12-01T21:33:41.4147186Z ------------------------------------------
2019-12-01T21:33:41.4147223Z 
2019-12-01T21:33:41.4147434Z ------------------------------------------
2019-12-01T21:33:41.4147495Z stderr:
2019-12-01T21:33:41.4147495Z stderr:
2019-12-01T21:33:41.4147709Z ------------------------------------------
2019-12-01T21:33:41.4147764Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4148037Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs:8:30
2019-12-01T21:33:41.4148097Z    |
2019-12-01T21:33:41.4148141Z LL | const fn no_inner_dyn_trait2(x: Hide) {
2019-12-01T21:33:41.4148245Z    |
2019-12-01T21:33:41.4148245Z    |
2019-12-01T21:33:41.4148542Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4148619Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4148705Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4149128Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs:12:50
2019-12-01T21:33:41.4149171Z    |
2019-12-01T21:33:41.4149171Z    |
2019-12-01T21:33:41.4149380Z LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
2019-12-01T21:33:41.4149479Z    |
2019-12-01T21:33:41.4149479Z    |
2019-12-01T21:33:41.4149715Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4149788Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4149852Z error: aborting due to 2 previous errors
2019-12-01T21:33:41.4149876Z 
2019-12-01T21:33:41.4150102Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T21:33:41.4150132Z 
---
2019-12-01T21:33:41.4150787Z 1 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4150998Z -   --> $DIR/min_const_fn_fn_ptr.rs:11:5
2019-12-01T21:33:41.4151180Z +   --> $DIR/min_const_fn_fn_ptr.rs:10:30
2019-12-01T21:33:41.4151235Z 3    |
2019-12-01T21:33:41.4151394Z - LL |     x.0.field;
2019-12-01T21:33:41.4151553Z -    |     ^^^^^^^^^
2019-12-01T21:33:41.4151656Z + LL | const fn no_inner_dyn_trait2(x: Hide) {
2019-12-01T21:33:41.4151761Z 6    |
2019-12-01T21:33:41.4151761Z 6    |
2019-12-01T21:33:41.4152027Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4152092Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4152153Z 9 
2019-12-01T21:33:41.4152209Z 10 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4152413Z -   --> $DIR/min_const_fn_fn_ptr.rs:14:59
2019-12-01T21:33:41.4152593Z +   --> $DIR/min_const_fn_fn_ptr.rs:14:50
2019-12-01T21:33:41.4152593Z +   --> $DIR/min_const_fn_fn_ptr.rs:14:50
2019-12-01T21:33:41.4152646Z 12    |
2019-12-01T21:33:41.4152854Z 13 LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasPtr { field }) }
2019-12-01T21:33:41.4153122Z +    |                                                  ^^^^^^^^^^^^^^^^
2019-12-01T21:33:41.4153166Z 15    |
2019-12-01T21:33:41.4153166Z 15    |
2019-12-01T21:33:41.4160772Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4160877Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4160931Z 
2019-12-01T21:33:41.4160970Z The actual stderr differed from the expected stderr.
2019-12-01T21:33:41.4161303Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_fn_ptr/min_const_fn_fn_ptr.stderr
2019-12-01T21:33:41.4161303Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_fn_ptr/min_const_fn_fn_ptr.stderr
2019-12-01T21:33:41.4161548Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T21:33:41.4161792Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn_fn_ptr.rs`
2019-12-01T21:33:41.4161879Z error: 1 errors occurred comparing output.
2019-12-01T21:33:41.4161917Z status: exit code: 1
2019-12-01T21:33:41.4161917Z status: exit code: 1
2019-12-01T21:33:41.4162626Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn_fn_ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_fn_ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_fn_ptr/auxiliary" "-A" "unused"
2019-12-01T21:33:41.4162925Z ------------------------------------------
2019-12-01T21:33:41.4162955Z 
2019-12-01T21:33:41.4163137Z ------------------------------------------
2019-12-01T21:33:41.4163192Z stderr:
2019-12-01T21:33:41.4163192Z stderr:
2019-12-01T21:33:41.4163377Z ------------------------------------------
2019-12-01T21:33:41.4163421Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4163648Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_fn_ptr.rs:10:30
2019-12-01T21:33:41.4163714Z    |
2019-12-01T21:33:41.4163753Z LL | const fn no_inner_dyn_trait2(x: Hide) {
2019-12-01T21:33:41.4163846Z    |
2019-12-01T21:33:41.4163846Z    |
2019-12-01T21:33:41.4164099Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4164165Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4164230Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4164639Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_fn_ptr.rs:14:50
2019-12-01T21:33:41.4164700Z    |
2019-12-01T21:33:41.4164700Z    |
2019-12-01T21:33:41.4165405Z LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasPtr { field }) }
2019-12-01T21:33:41.4166303Z    |
2019-12-01T21:33:41.4166303Z    |
2019-12-01T21:33:41.4166655Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4166831Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4166942Z error: aborting due to 2 previous errors
2019-12-01T21:33:41.4166971Z 
2019-12-01T21:33:41.4167258Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T21:33:41.4167311Z 
---
2019-12-01T21:33:41.4168086Z 7 error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4168304Z -   --> $DIR/min_const_fn.rs:39:36
2019-12-01T21:33:41.4168528Z +   --> $DIR/min_const_fn.rs:39:5
2019-12-01T21:33:41.4168574Z 9    |
2019-12-01T21:33:41.4168822Z 10 LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
2019-12-01T21:33:41.4169124Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T21:33:41.4169333Z 12    |
2019-12-01T21:33:41.4169333Z 12    |
2019-12-01T21:33:41.4169758Z 13    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4169806Z 14    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4169880Z 20    |                            ^^^^ constant functions cannot evaluate destructors
2019-12-01T21:33:41.4169935Z 21 
2019-12-01T21:33:41.4169973Z 22 error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4170163Z -   --> $DIR/min_const_fn.rs:46:42
2019-12-01T21:33:41.4170163Z -   --> $DIR/min_const_fn.rs:46:42
2019-12-01T21:33:41.4180538Z +   --> $DIR/min_const_fn.rs:46:5
2019-12-01T21:33:41.4180618Z 24    |
2019-12-01T21:33:41.4181442Z 25 LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
2019-12-01T21:33:41.4181780Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T21:33:41.4181825Z 27    |
2019-12-01T21:33:41.4181825Z 27    |
2019-12-01T21:33:41.4182175Z 28    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4182237Z 29    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4182393Z 35    |                           ^^^^ constant functions cannot evaluate destructors
2019-12-01T21:33:41.4182451Z 36 
2019-12-01T21:33:41.4182497Z 37 error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4182752Z -   --> $DIR/min_const_fn.rs:53:38
2019-12-01T21:33:41.4182752Z -   --> $DIR/min_const_fn.rs:53:38
2019-12-01T21:33:41.4182969Z +   --> $DIR/min_const_fn.rs:53:5
2019-12-01T21:33:41.4183016Z 39    |
2019-12-01T21:33:41.4183256Z 40 LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
2019-12-01T21:33:41.4183564Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T21:33:41.4183608Z 42    |
2019-12-01T21:33:41.4183608Z 42    |
2019-12-01T21:33:41.4183919Z 43    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4183977Z 44    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4184066Z 45 
2019-12-01T21:33:41.4184113Z 46 error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4186421Z -   --> $DIR/min_const_fn.rs:58:39
2019-12-01T21:33:41.4186810Z +   --> $DIR/min_const_fn.rs:58:5
2019-12-01T21:33:41.4186810Z +   --> $DIR/min_const_fn.rs:58:5
2019-12-01T21:33:41.4186863Z 48    |
2019-12-01T21:33:41.4187107Z 49 LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
2019-12-01T21:33:41.4187407Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T21:33:41.4187452Z 51    |
2019-12-01T21:33:41.4187452Z 51    |
2019-12-01T21:33:41.4187899Z 52    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4187980Z 53    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4188063Z 179    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4188127Z 180 
2019-12-01T21:33:41.4188174Z 181 error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4188433Z -   --> $DIR/min_const_fn.rs:105:14
2019-12-01T21:33:41.4188433Z -   --> $DIR/min_const_fn.rs:105:14
2019-12-01T21:33:41.4188662Z +   --> $DIR/min_const_fn.rs:105:27
2019-12-01T21:33:41.4188719Z 183    |
2019-12-01T21:33:41.4188764Z 184 LL | const fn inc(x: &mut i32) { *x += 1 }
2019-12-01T21:33:41.4189024Z +    |                           ^
2019-12-01T21:33:41.4189068Z 186    |
2019-12-01T21:33:41.4189068Z 186    |
2019-12-01T21:33:41.4189905Z 187    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4189958Z 188    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4190037Z 251    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4190093Z 252 
2019-12-01T21:33:41.4190137Z 253 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4190336Z -   --> $DIR/min_const_fn.rs:132:23
2019-12-01T21:33:41.4190336Z -   --> $DIR/min_const_fn.rs:132:23
2019-12-01T21:33:41.4190704Z +   --> $DIR/min_const_fn.rs:132:49
2019-12-01T21:33:41.4190742Z 255    |
2019-12-01T21:33:41.4190779Z 256 LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {}
2019-12-01T21:33:41.4191020Z +    |                                                 ^
2019-12-01T21:33:41.4191056Z 258    |
2019-12-01T21:33:41.4191056Z 258    |
2019-12-01T21:33:41.4191313Z 259    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4191362Z 260    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4191437Z 261 
2019-12-01T21:33:41.4191485Z 262 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4191675Z -   --> $DIR/min_const_fn.rs:133:32
2019-12-01T21:33:41.4191849Z +   --> $DIR/min_const_fn.rs:133:1
2019-12-01T21:33:41.4191849Z +   --> $DIR/min_const_fn.rs:133:1
2019-12-01T21:33:41.4191903Z 264    |
2019-12-01T21:33:41.4192114Z 265 LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-12-01T21:33:41.4192379Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T21:33:41.4192424Z 267    |
2019-12-01T21:33:41.4192424Z 267    |
2019-12-01T21:33:41.4192666Z 268    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4192730Z 269    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4192789Z 270 
2019-12-01T21:33:41.4192846Z 271 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4201074Z -   --> $DIR/min_const_fn.rs:138:41
2019-12-01T21:33:41.4201499Z +   --> $DIR/min_const_fn.rs:138:39
2019-12-01T21:33:41.4201499Z +   --> $DIR/min_const_fn.rs:138:39
2019-12-01T21:33:41.4201585Z 273    |
2019-12-01T21:33:41.4201635Z 274 LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
2019-12-01T21:33:41.4201970Z +    |                                       ^
2019-12-01T21:33:41.4202013Z 276    |
2019-12-01T21:33:41.4202013Z 276    |
2019-12-01T21:33:41.4202512Z 277    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4202785Z 278    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4202861Z 279 
2019-12-01T21:33:41.4202907Z 280 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4203350Z -   --> $DIR/min_const_fn.rs:141:21
2019-12-01T21:33:41.4203548Z +   --> $DIR/min_const_fn.rs:141:31
2019-12-01T21:33:41.4203548Z +   --> $DIR/min_const_fn.rs:141:31
2019-12-01T21:33:41.4203590Z 282    |
2019-12-01T21:33:41.4203812Z 283 LL | const fn no_fn_ptrs(_x: fn()) {}
2019-12-01T21:33:41.4204131Z +    |                               ^
2019-12-01T21:33:41.4204186Z 285    |
2019-12-01T21:33:41.4204186Z 285    |
2019-12-01T21:33:41.4204483Z 286    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4204539Z 287    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4204621Z 288 
2019-12-01T21:33:41.4205113Z 289 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4205515Z -   --> $DIR/min_const_fn.rs:143:27
2019-12-01T21:33:41.4205750Z +   --> $DIR/min_const_fn.rs:143:1
2019-12-01T21:33:41.4205750Z +   --> $DIR/min_const_fn.rs:143:1
2019-12-01T21:33:41.4205797Z 291    |
2019-12-01T21:33:41.4206029Z 292 LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
2019-12-01T21:33:41.4206313Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T21:33:41.4206355Z 294    |
2019-12-01T21:33:41.4206355Z 294    |
2019-12-01T21:33:41.4206681Z 295    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4206745Z 296    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4206806Z 
2019-12-01T21:33:41.4206870Z The actual stderr differed from the expected stderr.
2019-12-01T21:33:41.4207229Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/min_const_fn.stderr
2019-12-01T21:33:41.4207229Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/min_const_fn.stderr
2019-12-01T21:33:41.4207506Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T21:33:41.4207841Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`
2019-12-01T21:33:41.4207927Z error: 1 errors occurred comparing output.
2019-12-01T21:33:41.4207991Z status: exit code: 1
2019-12-01T21:33:41.4207991Z status: exit code: 1
2019-12-01T21:33:41.4208896Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/auxiliary" "-A" "unused"
2019-12-01T21:33:41.4209200Z ------------------------------------------
2019-12-01T21:33:41.4209240Z 
2019-12-01T21:33:41.4209448Z ------------------------------------------
2019-12-01T21:33:41.4209490Z stderr:
2019-12-01T21:33:41.4209490Z stderr:
2019-12-01T21:33:41.4209675Z ------------------------------------------
2019-12-01T21:33:41.4209900Z error[E0493]: destructors cannot be evaluated at compile-time
2019-12-01T21:33:41.4210128Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:37:25
2019-12-01T21:33:41.4210175Z    |
2019-12-01T21:33:41.4210419Z LL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-12-01T21:33:41.4210504Z 
2019-12-01T21:33:41.4210544Z error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4210784Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:5
2019-12-01T21:33:41.4210828Z    |
2019-12-01T21:33:41.4210828Z    |
2019-12-01T21:33:41.4211028Z LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
2019-12-01T21:33:41.4211429Z    |
2019-12-01T21:33:41.4211429Z    |
2019-12-01T21:33:41.4211724Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4211795Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4212047Z error[E0493]: destructors cannot be evaluated at compile-time
2019-12-01T21:33:41.4212357Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:44:28
2019-12-01T21:33:41.4212409Z    |
2019-12-01T21:33:41.4212409Z    |
2019-12-01T21:33:41.4212668Z LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-12-01T21:33:41.4212769Z 
2019-12-01T21:33:41.4212810Z error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4213032Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:46:5
2019-12-01T21:33:41.4213102Z    |
2019-12-01T21:33:41.4213102Z    |
2019-12-01T21:33:41.4213319Z LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
2019-12-01T21:33:41.4213425Z    |
2019-12-01T21:33:41.4213425Z    |
2019-12-01T21:33:41.4213685Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4213736Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4214011Z error[E0493]: destructors cannot be evaluated at compile-time
2019-12-01T21:33:41.4214237Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:51:27
2019-12-01T21:33:41.4214295Z    |
2019-12-01T21:33:41.4214295Z    |
2019-12-01T21:33:41.4214519Z LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructors
2019-12-01T21:33:41.4214599Z 
2019-12-01T21:33:41.4214655Z error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4215530Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:53:5
2019-12-01T21:33:41.4215585Z    |
2019-12-01T21:33:41.4215585Z    |
2019-12-01T21:33:41.4215844Z LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
2019-12-01T21:33:41.4215941Z    |
2019-12-01T21:33:41.4215941Z    |
2019-12-01T21:33:41.4216246Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4216314Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4216407Z error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4216667Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:58:5
2019-12-01T21:33:41.4216716Z    |
2019-12-01T21:33:41.4216716Z    |
2019-12-01T21:33:41.4216965Z LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
2019-12-01T21:33:41.4217063Z    |
2019-12-01T21:33:41.4217063Z    |
2019-12-01T21:33:41.4217372Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4217430Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4217511Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4217791Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:76:16
2019-12-01T21:33:41.4217839Z    |
2019-12-01T21:33:41.4217839Z    |
2019-12-01T21:33:41.4218075Z LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
2019-12-01T21:33:41.4218183Z    |
2019-12-01T21:33:41.4218183Z    |
2019-12-01T21:33:41.4218471Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4218709Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4218783Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4219205Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:78:18
2019-12-01T21:33:41.4219377Z    |
2019-12-01T21:33:41.4219377Z    |
2019-12-01T21:33:41.4219607Z LL | const fn foo11_2<T: Send>(t: T) -> T { t }
2019-12-01T21:33:41.4219707Z    |
2019-12-01T21:33:41.4219707Z    |
2019-12-01T21:33:41.4219966Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4220033Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4220062Z 
2019-12-01T21:33:41.4220176Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-12-01T21:33:41.4220503Z    |
2019-12-01T21:33:41.4220503Z    |
2019-12-01T21:33:41.4220706Z LL | const fn foo19(f: f32) -> f32 { f * 2.0 }
2019-12-01T21:33:41.4220806Z    |
2019-12-01T21:33:41.4220806Z    |
2019-12-01T21:33:41.4221065Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4221280Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4221334Z 
2019-12-01T21:33:41.4221375Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-12-01T21:33:41.4221662Z    |
2019-12-01T21:33:41.4221662Z    |
2019-12-01T21:33:41.4221860Z LL | const fn foo19_2(f: f32) -> f32 { 2.0 - f }
2019-12-01T21:33:41.4221955Z    |
2019-12-01T21:33:41.4221955Z    |
2019-12-01T21:33:41.4222395Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4222446Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4222531Z error[E0723]: only int and `bool` operations are stable in const fn
2019-12-01T21:33:41.4222950Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:84:35
2019-12-01T21:33:41.4222996Z    |
2019-12-01T21:33:41.4222996Z    |
2019-12-01T21:33:41.4223215Z LL | const fn foo19_3(f: f32) -> f32 { -f }
2019-12-01T21:33:41.4223310Z    |
2019-12-01T21:33:41.4223310Z    |
2019-12-01T21:33:41.4223589Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4223646Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4223676Z 
2019-12-01T21:33:41.4223739Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-12-01T21:33:41.4224060Z    |
2019-12-01T21:33:41.4224060Z    |
2019-12-01T21:33:41.4224469Z LL | const fn foo19_4(f: f32, g: f32) -> f32 { f / g }
2019-12-01T21:33:41.4224559Z    |
2019-12-01T21:33:41.4224559Z    |
2019-12-01T21:33:41.4225227Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4225293Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4225370Z error[E0723]: cannot access `static` items in const fn
2019-12-01T21:33:41.4225671Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:90:27
2019-12-01T21:33:41.4225720Z    |
2019-12-01T21:33:41.4225720Z    |
2019-12-01T21:33:41.4225979Z LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot access `static` items in const fn
2019-12-01T21:33:41.4226093Z    |
2019-12-01T21:33:41.4226093Z    |
2019-12-01T21:33:41.4226392Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4226458Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4226534Z error[E0723]: cannot access `static` items in const fn
2019-12-01T21:33:41.4226811Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:91:37
2019-12-01T21:33:41.4226860Z    |
2019-12-01T21:33:41.4226860Z    |
2019-12-01T21:33:41.4227118Z LL | const fn foo26() -> &'static u32 { &BAR } //~ ERROR cannot access `static` items
2019-12-01T21:33:41.4227344Z    |
2019-12-01T21:33:41.4227344Z    |
2019-12-01T21:33:41.4227660Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4227736Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4227815Z error[E0723]: casting pointers to ints is unstable in const fn
2019-12-01T21:33:41.4228092Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:92:42
2019-12-01T21:33:41.4228141Z    |
2019-12-01T21:33:41.4228141Z    |
2019-12-01T21:33:41.4228585Z LL | const fn foo30(x: *const u32) -> usize { x as usize }
2019-12-01T21:33:41.4228694Z    |
2019-12-01T21:33:41.4228694Z    |
2019-12-01T21:33:41.4228974Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4229039Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4229106Z error[E0723]: casting pointers to ints is unstable in const fn
2019-12-01T21:33:41.4229342Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:94:63
2019-12-01T21:33:41.4229398Z    |
2019-12-01T21:33:41.4229398Z    |
2019-12-01T21:33:41.4229621Z LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
2019-12-01T21:33:41.4229725Z    |
2019-12-01T21:33:41.4229725Z    |
2019-12-01T21:33:41.4229967Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4230024Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4230107Z error[E0723]: casting pointers to ints is unstable in const fn
2019-12-01T21:33:41.4230331Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:96:42
2019-12-01T21:33:41.4230388Z    |
2019-12-01T21:33:41.4230388Z    |
2019-12-01T21:33:41.4230594Z LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
2019-12-01T21:33:41.4230678Z    |
2019-12-01T21:33:41.4230678Z    |
2019-12-01T21:33:41.4231237Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4231291Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4231383Z error[E0723]: casting pointers to ints is unstable in const fn
2019-12-01T21:33:41.4231634Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:98:63
2019-12-01T21:33:41.4231680Z    |
2019-12-01T21:33:41.4231680Z    |
2019-12-01T21:33:41.4231955Z LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
2019-12-01T21:33:41.4232052Z    |
2019-12-01T21:33:41.4232052Z    |
2019-12-01T21:33:41.4232345Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4232401Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4232493Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-12-01T21:33:41.4232756Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:101:44
2019-12-01T21:33:41.4232802Z    |
2019-12-01T21:33:41.4232802Z    |
2019-12-01T21:33:41.4233044Z LL | const fn foo36(a: bool, b: bool) -> bool { a && b }
2019-12-01T21:33:41.4233135Z    |
2019-12-01T21:33:41.4233135Z    |
2019-12-01T21:33:41.4233694Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4233754Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4233857Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-12-01T21:33:41.4234505Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:103:44
2019-12-01T21:33:41.4234554Z    |
2019-12-01T21:33:41.4234554Z    |
2019-12-01T21:33:41.4235367Z LL | const fn foo37(a: bool, b: bool) -> bool { a || b }
2019-12-01T21:33:41.4235490Z    |
2019-12-01T21:33:41.4235490Z    |
2019-12-01T21:33:41.4235816Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4236009Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4236087Z error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4236393Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:105:27
2019-12-01T21:33:41.4236443Z    |
2019-12-01T21:33:41.4236443Z    |
2019-12-01T21:33:41.4236487Z LL | const fn inc(x: &mut i32) { *x += 1 }
2019-12-01T21:33:41.4236679Z    |
2019-12-01T21:33:41.4236679Z    |
2019-12-01T21:33:41.4236991Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4237069Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4237149Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4237426Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:110:6
2019-12-01T21:33:41.4237485Z    |
2019-12-01T21:33:41.4237485Z    |
2019-12-01T21:33:41.4237529Z LL | impl<T: std::fmt::Debug> Foo<T> {
2019-12-01T21:33:41.4237632Z    |
2019-12-01T21:33:41.4237632Z    |
2019-12-01T21:33:41.4237919Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4237977Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4238074Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4238343Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:115:6
2019-12-01T21:33:41.4238409Z    |
2019-12-01T21:33:41.4238409Z    |
2019-12-01T21:33:41.4238453Z LL | impl<T: std::fmt::Debug + Sized> Foo<T> {
2019-12-01T21:33:41.4238555Z    |
2019-12-01T21:33:41.4238555Z    |
2019-12-01T21:33:41.4239220Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4239332Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4239436Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4239690Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:120:6
2019-12-01T21:33:41.4239752Z    |
2019-12-01T21:33:41.4239752Z    |
2019-12-01T21:33:41.4239793Z LL | impl<T: Sync + Sized> Foo<T> {
2019-12-01T21:33:41.4239920Z    |
2019-12-01T21:33:41.4239920Z    |
2019-12-01T21:33:41.4240621Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4240685Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4240775Z error[E0723]: `impl Trait` in const fn is unstable
2019-12-01T21:33:41.4241024Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:126:24
2019-12-01T21:33:41.4241070Z    |
2019-12-01T21:33:41.4241070Z    |
2019-12-01T21:33:41.4241509Z LL | const fn no_rpit2() -> AlanTuring<impl std::fmt::Debug> { AlanTuring(0) }
2019-12-01T21:33:41.4241608Z    |
2019-12-01T21:33:41.4241608Z    |
2019-12-01T21:33:41.4241914Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4241970Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4242065Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4242324Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:128:34
2019-12-01T21:33:41.4242372Z    |
2019-12-01T21:33:41.4242372Z    |
2019-12-01T21:33:41.4242427Z LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
2019-12-01T21:33:41.4242539Z    |
2019-12-01T21:33:41.4242539Z    |
2019-12-01T21:33:41.4242847Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4242905Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4242984Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4243449Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:130:22
2019-12-01T21:33:41.4243498Z    |
2019-12-01T21:33:41.4243498Z    |
2019-12-01T21:33:41.4243549Z LL | const fn no_apit(_x: impl std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
2019-12-01T21:33:41.4243659Z    |
2019-12-01T21:33:41.4243659Z    |
2019-12-01T21:33:41.4243947Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4244096Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4244180Z error[E0723]: `impl Trait` in const fn is unstable
2019-12-01T21:33:41.4244480Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:131:23
2019-12-01T21:33:41.4244530Z    |
2019-12-01T21:33:41.4244530Z    |
2019-12-01T21:33:41.4245700Z LL | const fn no_rpit() -> impl std::fmt::Debug {} //~ ERROR `impl Trait` in const fn is unstable
2019-12-01T21:33:41.4245823Z    |
2019-12-01T21:33:41.4245823Z    |
2019-12-01T21:33:41.4246133Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4246207Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4246287Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4246546Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:132:49
2019-12-01T21:33:41.4246611Z    |
2019-12-01T21:33:41.4246611Z    |
2019-12-01T21:33:41.4246671Z LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
2019-12-01T21:33:41.4246784Z    |
2019-12-01T21:33:41.4246784Z    |
2019-12-01T21:33:41.4247072Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4247144Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4247225Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4247494Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:133:1
2019-12-01T21:33:41.4247560Z    |
2019-12-01T21:33:41.4247560Z    |
2019-12-01T21:33:41.4247802Z LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-12-01T21:33:41.4247915Z    |
2019-12-01T21:33:41.4247915Z    |
2019-12-01T21:33:41.4248194Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4248258Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4248353Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T21:33:41.4248781Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:138:39
2019-12-01T21:33:41.4248839Z    |
2019-12-01T21:33:41.4248839Z    |
2019-12-01T21:33:41.4248882Z LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
2019-12-01T21:33:41.4249092Z    |
2019-12-01T21:33:41.4249092Z    |
2019-12-01T21:33:41.4249487Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4249538Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4249623Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4249855Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:141:31
2019-12-01T21:33:41.4249898Z    |
2019-12-01T21:33:41.4249898Z    |
2019-12-01T21:33:41.4249960Z LL | const fn no_fn_ptrs(_x: fn()) {}
2019-12-01T21:33:41.4250039Z    |
2019-12-01T21:33:41.4250039Z    |
2019-12-01T21:33:41.4250307Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4250358Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4250442Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4250670Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:143:1
2019-12-01T21:33:41.4250817Z    |
2019-12-01T21:33:41.4250817Z    |
2019-12-01T21:33:41.4251066Z LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
2019-12-01T21:33:41.4251152Z    |
2019-12-01T21:33:41.4251152Z    |
2019-12-01T21:33:41.4251433Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4251486Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4251628Z error: aborting due to 34 previous errors
2019-12-01T21:33:41.4251680Z 
2019-12-01T21:33:41.4251721Z Some errors have detailed explanations: E0493, E0723.
2019-12-01T21:33:41.4251969Z For more information about an error, try `rustc --explain E0493`.
---
2019-12-01T21:33:41.4253609Z -    |         ^
2019-12-01T21:33:41.4253653Z + LL |     let mut a = 0;
2019-12-01T21:33:41.4253700Z +    |         ^^^^^
2019-12-01T21:33:41.4253758Z 6    |
2019-12-01T21:33:41.4254033Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4254088Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4254171Z 9 
2019-12-01T21:33:41.4254214Z 10 error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4254422Z -   --> $DIR/mutable_borrow.rs:12:13
2019-12-01T21:33:41.4254653Z +   --> $DIR/mutable_borrow.rs:11:13
2019-12-01T21:33:41.4254653Z +   --> $DIR/mutable_borrow.rs:11:13
2019-12-01T21:33:41.4254696Z 12    |
2019-12-01T21:33:41.4254881Z - LL |         let b = &mut a;
2019-12-01T21:33:41.4255077Z -    |             ^
2019-12-01T21:33:41.4255121Z + LL |         let mut a = 0;
2019-12-01T21:33:41.4255321Z +    |             ^^^^^
2019-12-01T21:33:41.4255570Z 15    |
2019-12-01T21:33:41.4255906Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4255977Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4256052Z 
2019-12-01T21:33:41.4256098Z The actual stderr differed from the expected stderr.
2019-12-01T21:33:41.4256421Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/mutable_borrow/mutable_borrow.stderr
2019-12-01T21:33:41.4256421Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/mutable_borrow/mutable_borrow.stderr
2019-12-01T21:33:41.4256691Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T21:33:41.4256971Z To only update this specific test, also pass `--test-args consts/min_const_fn/mutable_borrow.rs`
2019-12-01T21:33:41.4257077Z error: 1 errors occurred comparing output.
2019-12-01T21:33:41.4257121Z status: exit code: 1
2019-12-01T21:33:41.4257121Z status: exit code: 1
2019-12-01T21:33:41.4257922Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/mutable_borrow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/mutable_borrow" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/mutable_borrow/auxiliary" "-A" "unused"
2019-12-01T21:33:41.4258302Z ------------------------------------------
2019-12-01T21:33:41.4258358Z 
2019-12-01T21:33:41.4258579Z ------------------------------------------
2019-12-01T21:33:41.4259057Z stderr:
2019-12-01T21:33:41.4259057Z stderr:
2019-12-01T21:33:41.4259300Z ------------------------------------------
2019-12-01T21:33:41.4259349Z error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4259587Z   --> /checkout/src/test/ui/consts/min_const_fn/mutable_borrow.rs:2:9
2019-12-01T21:33:41.4259651Z    |
2019-12-01T21:33:41.4259690Z LL |     let mut a = 0;
2019-12-01T21:33:41.4259729Z    |         ^^^^^
2019-12-01T21:33:41.4259766Z    |
2019-12-01T21:33:41.4260127Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4260191Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4260280Z error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4261369Z   --> /checkout/src/test/ui/consts/min_const_fn/mutable_borrow.rs:11:13
2019-12-01T21:33:41.4261428Z    |
2019-12-01T21:33:41.4261486Z LL |         let mut a = 0;
2019-12-01T21:33:41.4261486Z LL |         let mut a = 0;
2019-12-01T21:33:41.4261529Z    |             ^^^^^
2019-12-01T21:33:41.4261578Z    |
2019-12-01T21:33:41.4261946Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4262002Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4262072Z error: aborting due to 2 previous errors
2019-12-01T21:33:41.4262116Z 
2019-12-01T21:33:41.4262355Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T21:33:41.4262387Z 
---
2019-12-01T21:33:41.4263008Z 1 error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4263205Z -   --> $DIR/issue-37550.rs:3:9
2019-12-01T21:33:41.4263414Z +   --> $DIR/issue-37550.rs:2:9
2019-12-01T21:33:41.4263456Z 3    |
2019-12-01T21:33:41.4263650Z - LL |     let x = || t;
2019-12-01T21:33:41.4263695Z + LL |     let t = true;
2019-12-01T21:33:41.4263787Z 6    |
2019-12-01T21:33:41.4263787Z 6    |
2019-12-01T21:33:41.4264056Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4264128Z 
2019-12-01T21:33:41.4264171Z The actual stderr differed from the expected stderr.
2019-12-01T21:33:41.4264462Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37550/issue-37550.stderr
2019-12-01T21:33:41.4264462Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37550/issue-37550.stderr
2019-12-01T21:33:41.4265084Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T21:33:41.4265419Z To only update this specific test, also pass `--test-args issues/issue-37550.rs`
2019-12-01T21:33:41.4265521Z error: 1 errors occurred comparing output.
2019-12-01T21:33:41.4265569Z status: exit code: 1
2019-12-01T21:33:41.4265569Z status: exit code: 1
2019-12-01T21:33:41.4266352Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37550.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37550" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37550/auxiliary" "-A" "unused"
2019-12-01T21:33:41.4266727Z ------------------------------------------
2019-12-01T21:33:41.4266781Z 
2019-12-01T21:33:41.4267020Z ------------------------------------------
2019-12-01T21:33:41.4267069Z stderr:
2019-12-01T21:33:41.4267069Z stderr:
2019-12-01T21:33:41.4267313Z ------------------------------------------
2019-12-01T21:33:41.4267367Z error[E0723]: function pointers in const fn are unstable
2019-12-01T21:33:41.4267625Z   --> /checkout/src/test/ui/issues/issue-37550.rs:2:9
2019-12-01T21:33:41.4267694Z    |
2019-12-01T21:33:41.4267740Z LL |     let t = true;
2019-12-01T21:33:41.4267924Z    |         ^
2019-12-01T21:33:41.4267967Z    |
2019-12-01T21:33:41.4268343Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4268630Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4268718Z error: aborting due to previous error
2019-12-01T21:33:41.4268745Z 
2019-12-01T21:33:41.4268990Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T21:33:41.4269106Z 
---
2019-12-01T21:33:41.4269757Z 1 error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4269959Z -   --> $DIR/ranged_ints2_const.rs:11:9
2019-12-01T21:33:41.4270177Z +   --> $DIR/ranged_ints2_const.rs:10:9
2019-12-01T21:33:41.4270229Z 3    |
2019-12-01T21:33:41.4270415Z - LL |     let y = &mut x.0;
2019-12-01T21:33:41.4270658Z + LL |     let mut x = unsafe { NonZero(1) };
2019-12-01T21:33:41.4270700Z +    |         ^^^^^
2019-12-01T21:33:41.4270756Z 6    |
2019-12-01T21:33:41.4270756Z 6    |
2019-12-01T21:33:41.4271038Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4271102Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4271191Z 9 
2019-12-01T21:33:41.4271234Z 10 error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4271446Z -   --> $DIR/ranged_ints2_const.rs:18:9
2019-12-01T21:33:41.4271667Z +   --> $DIR/ranged_ints2_const.rs:17:9
2019-12-01T21:33:41.4271667Z +   --> $DIR/ranged_ints2_const.rs:17:9
2019-12-01T21:33:41.4271709Z 12    |
2019-12-01T21:33:41.4271901Z - LL |     let y = unsafe { &mut x.0 };
2019-12-01T21:33:41.4272142Z + LL |     let mut x = unsafe { NonZero(1) };
2019-12-01T21:33:41.4272192Z +    |         ^^^^^
2019-12-01T21:33:41.4272249Z 15    |
2019-12-01T21:33:41.4272249Z 15    |
2019-12-01T21:33:41.4272521Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4272575Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4272645Z 
2019-12-01T21:33:41.4272687Z The actual stderr differed from the expected stderr.
2019-12-01T21:33:41.4272988Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const/ranged_ints2_const.stderr
2019-12-01T21:33:41.4272988Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const/ranged_ints2_const.stderr
2019-12-01T21:33:41.4273381Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T21:33:41.4273656Z To only update this specific test, also pass `--test-args unsafe/ranged_ints2_const.rs`
2019-12-01T21:33:41.4273750Z error: 1 errors occurred comparing output.
2019-12-01T21:33:41.4273792Z status: exit code: 1
2019-12-01T21:33:41.4273792Z status: exit code: 1
2019-12-01T21:33:41.4274514Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints2_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const/auxiliary" "-A" "unused"
2019-12-01T21:33:41.4274836Z ------------------------------------------
2019-12-01T21:33:41.4274884Z 
2019-12-01T21:33:41.4275471Z ------------------------------------------
2019-12-01T21:33:41.4275522Z stderr:
2019-12-01T21:33:41.4275522Z stderr:
2019-12-01T21:33:41.4275735Z ------------------------------------------
2019-12-01T21:33:41.4275808Z error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4276059Z   --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:10:9
2019-12-01T21:33:41.4276249Z    |
2019-12-01T21:33:41.4276314Z LL |     let mut x = unsafe { NonZero(1) };
2019-12-01T21:33:41.4276359Z    |         ^^^^^
2019-12-01T21:33:41.4276399Z    |
2019-12-01T21:33:41.4276753Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4276813Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4276906Z error[E0723]: mutable references in const fn are unstable
2019-12-01T21:33:41.4277242Z   --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:17:9
2019-12-01T21:33:41.4277299Z    |
2019-12-01T21:33:41.4277360Z LL |     let mut x = unsafe { NonZero(1) };
2019-12-01T21:33:41.4277360Z LL |     let mut x = unsafe { NonZero(1) };
2019-12-01T21:33:41.4277404Z    |         ^^^^^
2019-12-01T21:33:41.4277444Z    |
2019-12-01T21:33:41.4277772Z    = note: for more information, see issue ***/issues/57563
2019-12-01T21:33:41.4277831Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T21:33:41.4277924Z error[E0133]: mutation of layout constrained field is unsafe and requires unsafe function or block
2019-12-01T21:33:41.4278204Z   --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:11:13
2019-12-01T21:33:41.4278254Z    |
2019-12-01T21:33:41.4278254Z    |
2019-12-01T21:33:41.4278302Z LL |     let y = &mut x.0; //~ ERROR references in const fn are unstable
2019-12-01T21:33:41.4278369Z    |             ^^^^^^^^ mutation of layout constrained field
2019-12-01T21:33:41.4278470Z    = note: mutating layout constrained fields cannot statically be checked for valid values
2019-12-01T21:33:41.4278522Z 
2019-12-01T21:33:41.4278567Z error: aborting due to 3 previous errors
2019-12-01T21:33:41.4278759Z 
---
2019-12-01T21:33:41.4280856Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-01T21:33:41.4281001Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-01T21:33:41.4281031Z 
2019-12-01T21:33:41.4281055Z 
2019-12-01T21:33:41.4282597Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-01T21:33:41.4282871Z 
2019-12-01T21:33:41.4282898Z 
2019-12-01T21:33:41.4282941Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-01T21:33:41.4283003Z Build completed unsuccessfully in 1:04:18
2019-12-01T21:33:41.4283003Z Build completed unsuccessfully in 1:04:18
2019-12-01T21:33:41.4283103Z == clock drift check ==
2019-12-01T21:33:41.4283171Z   local time: Sun Dec  1 21:33:41 UTC 2019
2019-12-01T21:33:41.9667397Z   network time: Sun, 01 Dec 2019 21:33:41 GMT
2019-12-01T21:33:41.9667930Z == end clock drift check ==
2019-12-01T21:33:42.8153955Z 
2019-12-01T21:33:42.8253181Z ##[error]Bash exited with code '1'.
2019-12-01T21:33:42.8289600Z ##[section]Starting: Checkout
2019-12-01T21:33:42.8291143Z ==============================================================================
2019-12-01T21:33:42.8291194Z Task         : Get sources
2019-12-01T21:33:42.8291251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
