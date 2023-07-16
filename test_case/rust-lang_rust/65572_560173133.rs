plain
2019-12-01T22:08:09.3890405Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-01T22:08:09.4079233Z ##[command]git config gc.auto 0
2019-12-01T22:08:09.4149764Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-01T22:08:09.4202508Z ##[command]git config --get-all http.proxy
2019-12-01T22:08:09.9707916Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65572/merge:refs/remotes/pull/65572/merge
---
2019-12-01T23:08:58.0769072Z .................................................................................................... 1300/9316
2019-12-01T23:09:05.4222227Z .................................................................................................... 1400/9316
2019-12-01T23:09:11.8919132Z .................................................................................F.................. 1500/9316
2019-12-01T23:09:18.6054920Z .................................................................................................... 1600/9316
2019-12-01T23:09:23.5299211Z ...................................F...F.FFFF......F................................................ 1700/9316
2019-12-01T23:09:45.3633248Z .................................................................................................... 1900/9316
2019-12-01T23:09:45.3633248Z .................................................................................................... 1900/9316
2019-12-01T23:09:59.4314026Z .........................iiiii...................................................................... 2000/9316
2019-12-01T23:10:10.0081869Z .................................................................................................... 2200/9316
2019-12-01T23:10:12.6765608Z .................................................................................................... 2300/9316
2019-12-01T23:10:17.3955366Z .................................................................................................... 2400/9316
2019-12-01T23:10:39.8485186Z .................................................................................................... 2500/9316
---
2019-12-01T23:13:26.4696321Z ...........................i...............i........................................................ 4800/9316
2019-12-01T23:13:37.5370528Z .................................................................................................... 4900/9316
2019-12-01T23:13:43.8427924Z .................................................................................................... 5000/9316
2019-12-01T23:13:52.2471565Z .................................................................................................... 5100/9316
2019-12-01T23:14:00.1357783Z .................................ii.ii...........i.................................................. 5200/9316
2019-12-01T23:14:10.4776201Z .................................................................................................... 5400/9316
2019-12-01T23:14:20.9688618Z .................................................................................................... 5500/9316
2019-12-01T23:14:28.6259111Z ...............i.................................................................................... 5600/9316
2019-12-01T23:14:35.0106871Z .................................................................................................... 5700/9316
2019-12-01T23:14:35.0106871Z .................................................................................................... 5700/9316
2019-12-01T23:14:46.8796443Z .................................................................................................... 5800/9316
2019-12-01T23:14:59.5857792Z .ii...i..ii...........i............................................................................. 5900/9316
2019-12-01T23:15:18.7402106Z .................................................................................................... 6100/9316
2019-12-01T23:15:26.1689422Z .................................................................................................... 6200/9316
2019-12-01T23:15:26.1689422Z .................................................................................................... 6200/9316
2019-12-01T23:15:40.5266854Z ........................i..ii....................................................................... 6300/9316
2019-12-01T23:16:01.5640342Z ...............................................................................................i.... 6500/9316
2019-12-01T23:16:03.9482928Z .................................................................................................... 6600/9316
2019-12-01T23:16:06.2816842Z ......................................................................................i............. 6700/9316
2019-12-01T23:16:09.1380307Z .................................................................................................... 6800/9316
---
2019-12-01T23:21:12.2902263Z 10 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2902507Z -   --> $DIR/const-extern-fn-min-const-fn.rs:5:41
2019-12-01T23:21:12.2902737Z +   --> $DIR/const-extern-fn-min-const-fn.rs:5:1
2019-12-01T23:21:12.2902781Z 12    |
2019-12-01T23:21:12.2903211Z 13 LL | const unsafe extern "C" fn closure() -> fn() { || {} }
2019-12-01T23:21:12.2903894Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T23:21:12.2903942Z 15    |
2019-12-01T23:21:12.2903942Z 15    |
2019-12-01T23:21:12.2904465Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2904748Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2904809Z 
2019-12-01T23:21:12.2904873Z The actual stderr differed from the expected stderr.
2019-12-01T23:21:12.2905278Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn/const-extern-fn-min-const-fn.stderr
2019-12-01T23:21:12.2905278Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn/const-extern-fn-min-const-fn.stderr
2019-12-01T23:21:12.2905534Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T23:21:12.2905849Z To only update this specific test, also pass `--test-args consts/const-extern-fn/const-extern-fn-min-const-fn.rs`
2019-12-01T23:21:12.2906050Z error: 1 errors occurred comparing output.
2019-12-01T23:21:12.2906111Z status: exit code: 1
2019-12-01T23:21:12.2906111Z status: exit code: 1
2019-12-01T23:21:12.2907169Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn/auxiliary" "-A" "unused"
2019-12-01T23:21:12.2907539Z ------------------------------------------
2019-12-01T23:21:12.2907570Z 
2019-12-01T23:21:12.2907799Z ------------------------------------------
2019-12-01T23:21:12.2907841Z stderr:
2019-12-01T23:21:12.2907841Z stderr:
2019-12-01T23:21:12.2908040Z ------------------------------------------
2019-12-01T23:21:12.2908087Z error[E0723]: unsizing casts are not allowed in const fn
2019-12-01T23:21:12.2908354Z   --> /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs:3:48
2019-12-01T23:21:12.2908410Z    |
2019-12-01T23:21:12.2908647Z LL | const extern fn unsize(x: &[u8; 3]) -> &[u8] { x }
2019-12-01T23:21:12.2908736Z    |
2019-12-01T23:21:12.2908736Z    |
2019-12-01T23:21:12.2909039Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2909097Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2909174Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2909485Z   --> /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs:5:1
2019-12-01T23:21:12.2909535Z    |
2019-12-01T23:21:12.2909535Z    |
2019-12-01T23:21:12.2909784Z LL | const unsafe extern "C" fn closure() -> fn() { || {} }
2019-12-01T23:21:12.2909895Z    |
2019-12-01T23:21:12.2909895Z    |
2019-12-01T23:21:12.2910188Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2910259Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2910299Z 
2019-12-01T23:21:12.2910345Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-12-01T23:21:12.2910695Z    |
2019-12-01T23:21:12.2910695Z    |
2019-12-01T23:21:12.2910739Z LL | const unsafe extern fn use_float() { 1.0 + 1.0; }
2019-12-01T23:21:12.2910847Z    |
2019-12-01T23:21:12.2910847Z    |
2019-12-01T23:21:12.2911136Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2911212Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2911290Z error[E0723]: casting pointers to ints is unstable in const fn
2019-12-01T23:21:12.2911569Z   --> /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs:9:48
2019-12-01T23:21:12.2911636Z    |
2019-12-01T23:21:12.2911636Z    |
2019-12-01T23:21:12.2911777Z LL | const extern "C" fn ptr_cast(val: *const u8) { val as usize; }
2019-12-01T23:21:12.2911885Z    |
2019-12-01T23:21:12.2911885Z    |
2019-12-01T23:21:12.2912207Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2912278Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2912351Z error: aborting due to 4 previous errors
2019-12-01T23:21:12.2912380Z 
2019-12-01T23:21:12.2912652Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T23:21:12.2912685Z 
---
2019-12-01T23:21:12.2913448Z 1 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2913701Z -   --> $DIR/allow_const_fn_ptr.rs:4:16
2019-12-01T23:21:12.2914462Z +   --> $DIR/allow_const_fn_ptr.rs:4:25
2019-12-01T23:21:12.2914514Z 3    |
2019-12-01T23:21:12.2914577Z 4 LL | const fn error(_: fn()) {}
2019-12-01T23:21:12.2914833Z +    |                         ^
2019-12-01T23:21:12.2914892Z 6    |
2019-12-01T23:21:12.2914892Z 6    |
2019-12-01T23:21:12.2915195Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2915255Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2915332Z 
2019-12-01T23:21:12.2915388Z The actual stderr differed from the expected stderr.
2019-12-01T23:21:12.2915732Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/allow_const_fn_ptr.stderr
2019-12-01T23:21:12.2915732Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/allow_const_fn_ptr.stderr
2019-12-01T23:21:12.2916007Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T23:21:12.2916323Z To only update this specific test, also pass `--test-args consts/min_const_fn/allow_const_fn_ptr.rs`
2019-12-01T23:21:12.2916426Z error: 1 errors occurred comparing output.
2019-12-01T23:21:12.2916472Z status: exit code: 1
2019-12-01T23:21:12.2916472Z status: exit code: 1
2019-12-01T23:21:12.2918058Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/allow_const_fn_ptr/auxiliary" "-A" "unused"
2019-12-01T23:21:12.2918426Z ------------------------------------------
2019-12-01T23:21:12.2918650Z 
2019-12-01T23:21:12.2918867Z ------------------------------------------
2019-12-01T23:21:12.2918911Z stderr:
2019-12-01T23:21:12.2918911Z stderr:
2019-12-01T23:21:12.2919138Z ------------------------------------------
2019-12-01T23:21:12.2919186Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2919433Z   --> /checkout/src/test/ui/consts/min_const_fn/allow_const_fn_ptr.rs:4:25
2019-12-01T23:21:12.2919499Z    |
2019-12-01T23:21:12.2919551Z LL | const fn error(_: fn()) {} //~ ERROR function pointers in const fn are unstable
2019-12-01T23:21:12.2919655Z    |
2019-12-01T23:21:12.2919655Z    |
2019-12-01T23:21:12.2920138Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2920197Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2920289Z error: aborting due to previous error
2019-12-01T23:21:12.2920318Z 
2019-12-01T23:21:12.2920575Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T23:21:12.2920777Z 
---
2019-12-01T23:21:12.2921575Z 10 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2921789Z -   --> $DIR/cast_errors.rs:5:23
2019-12-01T23:21:12.2922092Z +   --> $DIR/cast_errors.rs:5:1
2019-12-01T23:21:12.2922145Z 12    |
2019-12-01T23:21:12.2922385Z 13 LL | const fn closure() -> fn() { || {} }
2019-12-01T23:21:12.2922663Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T23:21:12.2922705Z 15    |
2019-12-01T23:21:12.2922705Z 15    |
2019-12-01T23:21:12.2923007Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2923092Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2923164Z 18 
2019-12-01T23:21:12.2923361Z 19 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2924355Z -   --> $DIR/cast_errors.rs:8:5
2019-12-01T23:21:12.2924601Z +   --> $DIR/cast_errors.rs:7:21
2019-12-01T23:21:12.2924601Z +   --> $DIR/cast_errors.rs:7:21
2019-12-01T23:21:12.2924667Z 21    |
2019-12-01T23:21:12.2924868Z - LL |     (|| {}) as fn();
2019-12-01T23:21:12.2925112Z + LL | const fn closure2() {
2019-12-01T23:21:12.2925184Z +    |                     ^
2019-12-01T23:21:12.2925225Z 24    |
2019-12-01T23:21:12.2925225Z 24    |
2019-12-01T23:21:12.2925537Z 25    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2925611Z 26    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2925682Z 27 
2019-12-01T23:21:12.2925754Z 28 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2925986Z -   --> $DIR/cast_errors.rs:11:28
2019-12-01T23:21:12.2926195Z +   --> $DIR/cast_errors.rs:11:1
2019-12-01T23:21:12.2926195Z +   --> $DIR/cast_errors.rs:11:1
2019-12-01T23:21:12.2926239Z 30    |
2019-12-01T23:21:12.2926488Z 31 LL | const fn reify(f: fn()) -> unsafe fn() { f }
2019-12-01T23:21:12.2926761Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T23:21:12.2926820Z 33    |
2019-12-01T23:21:12.2926820Z 33    |
2019-12-01T23:21:12.2927489Z 34    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2927574Z 35    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2927646Z 36 
2019-12-01T23:21:12.2927691Z 37 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2927938Z -   --> $DIR/cast_errors.rs:13:21
2019-12-01T23:21:12.2928153Z +   --> $DIR/cast_errors.rs:13:19
2019-12-01T23:21:12.2928153Z +   --> $DIR/cast_errors.rs:13:19
2019-12-01T23:21:12.2928198Z 39    |
2019-12-01T23:21:12.2928269Z 40 LL | const fn reify2() { main as unsafe fn(); }
2019-12-01T23:21:12.2928544Z +    |                   ^
2019-12-01T23:21:12.2928584Z 42    |
2019-12-01T23:21:12.2928584Z 42    |
2019-12-01T23:21:12.2928890Z 43    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2928946Z 44    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2929021Z 
2019-12-01T23:21:12.2929066Z The actual stderr differed from the expected stderr.
2019-12-01T23:21:12.2929396Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cast_errors/cast_errors.stderr
2019-12-01T23:21:12.2929396Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cast_errors/cast_errors.stderr
2019-12-01T23:21:12.2929666Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T23:21:12.2929942Z To only update this specific test, also pass `--test-args consts/min_const_fn/cast_errors.rs`
2019-12-01T23:21:12.2930038Z error: 1 errors occurred comparing output.
2019-12-01T23:21:12.2930213Z status: exit code: 1
2019-12-01T23:21:12.2930213Z status: exit code: 1
2019-12-01T23:21:12.2931432Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/cast_errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cast_errors" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cast_errors/auxiliary" "-A" "unused"
2019-12-01T23:21:12.2931778Z ------------------------------------------
2019-12-01T23:21:12.2931808Z 
2019-12-01T23:21:12.2932028Z ------------------------------------------
2019-12-01T23:21:12.2932069Z stderr:
2019-12-01T23:21:12.2932069Z stderr:
2019-12-01T23:21:12.2932267Z ------------------------------------------
2019-12-01T23:21:12.2932339Z error[E0723]: unsizing casts are not allowed in const fn
2019-12-01T23:21:12.2932573Z   --> /checkout/src/test/ui/consts/min_const_fn/cast_errors.rs:3:41
2019-12-01T23:21:12.2932618Z    |
2019-12-01T23:21:12.2932845Z LL | const fn unsize(x: &[u8; 3]) -> &[u8] { x }
2019-12-01T23:21:12.2932931Z    |
2019-12-01T23:21:12.2932931Z    |
2019-12-01T23:21:12.2933224Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2933276Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2933371Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2933795Z   --> /checkout/src/test/ui/consts/min_const_fn/cast_errors.rs:5:1
2019-12-01T23:21:12.2933841Z    |
2019-12-01T23:21:12.2933841Z    |
2019-12-01T23:21:12.2934539Z LL | const fn closure() -> fn() { || {} }
2019-12-01T23:21:12.2934655Z    |
2019-12-01T23:21:12.2934655Z    |
2019-12-01T23:21:12.2934992Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2935049Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2935126Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2935403Z   --> /checkout/src/test/ui/consts/min_const_fn/cast_errors.rs:7:21
2019-12-01T23:21:12.2935450Z    |
2019-12-01T23:21:12.2935492Z LL | const fn closure2() {
2019-12-01T23:21:12.2935492Z LL | const fn closure2() {
2019-12-01T23:21:12.2935551Z    |                     ^
2019-12-01T23:21:12.2935593Z    |
2019-12-01T23:21:12.2935876Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2935956Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2936033Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2936308Z   --> /checkout/src/test/ui/consts/min_const_fn/cast_errors.rs:11:1
2019-12-01T23:21:12.2936354Z    |
2019-12-01T23:21:12.2936354Z    |
2019-12-01T23:21:12.2936587Z LL | const fn reify(f: fn()) -> unsafe fn() { f }
2019-12-01T23:21:12.2936696Z    |
2019-12-01T23:21:12.2936696Z    |
2019-12-01T23:21:12.2936977Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2937046Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2937122Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2937596Z   --> /checkout/src/test/ui/consts/min_const_fn/cast_errors.rs:13:19
2019-12-01T23:21:12.2937660Z    |
2019-12-01T23:21:12.2937660Z    |
2019-12-01T23:21:12.2937712Z LL | const fn reify2() { main as unsafe fn(); }
2019-12-01T23:21:12.2937810Z    |
2019-12-01T23:21:12.2937810Z    |
2019-12-01T23:21:12.2938085Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2938138Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2938331Z error: aborting due to 5 previous errors
2019-12-01T23:21:12.2938360Z 
2019-12-01T23:21:12.2938628Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T23:21:12.2938677Z 
---
2019-12-01T23:21:12.2939482Z 1 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2939717Z -   --> $DIR/cmp_fn_pointers.rs:1:14
2019-12-01T23:21:12.2939999Z +   --> $DIR/cmp_fn_pointers.rs:1:35
2019-12-01T23:21:12.2940050Z 3    |
2019-12-01T23:21:12.2940290Z 4 LL | const fn cmp(x: fn(), y: fn()) -> bool {
2019-12-01T23:21:12.2940678Z +    |                                   ^^^^
2019-12-01T23:21:12.2940721Z 6    |
2019-12-01T23:21:12.2940721Z 6    |
2019-12-01T23:21:12.2941280Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2941334Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2941409Z 
2019-12-01T23:21:12.2941497Z The actual stderr differed from the expected stderr.
2019-12-01T23:21:12.2941819Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cmp_fn_pointers/cmp_fn_pointers.stderr
2019-12-01T23:21:12.2941819Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cmp_fn_pointers/cmp_fn_pointers.stderr
2019-12-01T23:21:12.2942081Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T23:21:12.2942357Z To only update this specific test, also pass `--test-args consts/min_const_fn/cmp_fn_pointers.rs`
2019-12-01T23:21:12.2942452Z error: 1 errors occurred comparing output.
2019-12-01T23:21:12.2942496Z status: exit code: 1
2019-12-01T23:21:12.2942496Z status: exit code: 1
2019-12-01T23:21:12.2943266Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/cmp_fn_pointers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cmp_fn_pointers" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cmp_fn_pointers/auxiliary" "-A" "unused"
2019-12-01T23:21:12.2943597Z ------------------------------------------
2019-12-01T23:21:12.2943645Z 
2019-12-01T23:21:12.2944261Z ------------------------------------------
2019-12-01T23:21:12.2944322Z stderr:
2019-12-01T23:21:12.2944322Z stderr:
2019-12-01T23:21:12.2944592Z ------------------------------------------
2019-12-01T23:21:12.2944665Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2944920Z   --> /checkout/src/test/ui/consts/min_const_fn/cmp_fn_pointers.rs:1:35
2019-12-01T23:21:12.2944968Z    |
2019-12-01T23:21:12.2945271Z LL | const fn cmp(x: fn(), y: fn()) -> bool { //~ ERROR function pointers in const fn are unstable
2019-12-01T23:21:12.2945390Z    |
2019-12-01T23:21:12.2945390Z    |
2019-12-01T23:21:12.2945686Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2945743Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2945835Z error: aborting due to previous error
2019-12-01T23:21:12.2945864Z 
2019-12-01T23:21:12.2946118Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T23:21:12.2946150Z 
---
2019-12-01T23:21:12.2947099Z 1 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.2947631Z -   --> $DIR/min_const_fn_dyn.rs:9:5
2019-12-01T23:21:12.2947853Z +   --> $DIR/min_const_fn_dyn.rs:8:30
2019-12-01T23:21:12.2947897Z 3    |
2019-12-01T23:21:12.2948090Z - LL |     x.0.field;
2019-12-01T23:21:12.2948301Z -    |     ^^^^^^^^^
2019-12-01T23:21:12.2948349Z + LL | const fn no_inner_dyn_trait2(x: Hide) {
2019-12-01T23:21:12.2948454Z 6    |
2019-12-01T23:21:12.2948454Z 6    |
2019-12-01T23:21:12.2948759Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2948902Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2949007Z 9 
2019-12-01T23:21:12.2949059Z 10 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.2949366Z -   --> $DIR/min_const_fn_dyn.rs:12:66
2019-12-01T23:21:12.2949607Z +   --> $DIR/min_const_fn_dyn.rs:12:50
2019-12-01T23:21:12.2949607Z +   --> $DIR/min_const_fn_dyn.rs:12:50
2019-12-01T23:21:12.2949665Z 12    |
2019-12-01T23:21:12.2949943Z 13 LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
2019-12-01T23:21:12.2950296Z +    |                                                  ^^^^^^^^^^^^^^^^^^^^
2019-12-01T23:21:12.2950344Z 15    |
2019-12-01T23:21:12.2950344Z 15    |
2019-12-01T23:21:12.2950684Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2950743Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2950821Z 
2019-12-01T23:21:12.2950877Z The actual stderr differed from the expected stderr.
2019-12-01T23:21:12.2951229Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn/min_const_fn_dyn.stderr
2019-12-01T23:21:12.2951229Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn/min_const_fn_dyn.stderr
2019-12-01T23:21:12.2951517Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T23:21:12.2951816Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn_dyn.rs`
2019-12-01T23:21:12.2951925Z error: 1 errors occurred comparing output.
2019-12-01T23:21:12.2951974Z status: exit code: 1
2019-12-01T23:21:12.2951974Z status: exit code: 1
2019-12-01T23:21:12.2952802Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn/auxiliary" "-A" "unused"
2019-12-01T23:21:12.2953297Z ------------------------------------------
2019-12-01T23:21:12.2953347Z 
2019-12-01T23:21:12.2954298Z ------------------------------------------
2019-12-01T23:21:12.2954351Z stderr:
2019-12-01T23:21:12.2954351Z stderr:
2019-12-01T23:21:12.2954576Z ------------------------------------------
2019-12-01T23:21:12.2954651Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.2954910Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs:8:30
2019-12-01T23:21:12.2954962Z    |
2019-12-01T23:21:12.2955024Z LL | const fn no_inner_dyn_trait2(x: Hide) {
2019-12-01T23:21:12.2955112Z    |
2019-12-01T23:21:12.2955112Z    |
2019-12-01T23:21:12.2955430Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2955498Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2955597Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.2955865Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs:12:50
2019-12-01T23:21:12.2956032Z    |
2019-12-01T23:21:12.2956032Z    |
2019-12-01T23:21:12.2956326Z LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
2019-12-01T23:21:12.2956426Z    |
2019-12-01T23:21:12.2956426Z    |
2019-12-01T23:21:12.2956736Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2956792Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2956883Z error: aborting due to 2 previous errors
2019-12-01T23:21:12.2956912Z 
2019-12-01T23:21:12.2957256Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T23:21:12.2957297Z 
---
2019-12-01T23:21:12.2958001Z 1 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2958224Z -   --> $DIR/min_const_fn_fn_ptr.rs:11:5
2019-12-01T23:21:12.2958460Z +   --> $DIR/min_const_fn_fn_ptr.rs:10:30
2019-12-01T23:21:12.2958505Z 3    |
2019-12-01T23:21:12.2958701Z - LL |     x.0.field;
2019-12-01T23:21:12.2958895Z -    |     ^^^^^^^^^
2019-12-01T23:21:12.2958961Z + LL | const fn no_inner_dyn_trait2(x: Hide) {
2019-12-01T23:21:12.2959049Z 6    |
2019-12-01T23:21:12.2959049Z 6    |
2019-12-01T23:21:12.2959371Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2959436Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2959524Z 9 
2019-12-01T23:21:12.2959571Z 10 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2959803Z -   --> $DIR/min_const_fn_fn_ptr.rs:14:59
2019-12-01T23:21:12.2960039Z +   --> $DIR/min_const_fn_fn_ptr.rs:14:50
2019-12-01T23:21:12.2960039Z +   --> $DIR/min_const_fn_fn_ptr.rs:14:50
2019-12-01T23:21:12.2960092Z 12    |
2019-12-01T23:21:12.2960346Z 13 LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasPtr { field }) }
2019-12-01T23:21:12.2960665Z +    |                                                  ^^^^^^^^^^^^^^^^
2019-12-01T23:21:12.2960709Z 15    |
2019-12-01T23:21:12.2960709Z 15    |
2019-12-01T23:21:12.2961017Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2961074Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2961132Z 
2019-12-01T23:21:12.2961201Z The actual stderr differed from the expected stderr.
2019-12-01T23:21:12.2961543Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_fn_ptr/min_const_fn_fn_ptr.stderr
2019-12-01T23:21:12.2961543Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_fn_ptr/min_const_fn_fn_ptr.stderr
2019-12-01T23:21:12.2961795Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T23:21:12.2962108Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn_fn_ptr.rs`
2019-12-01T23:21:12.2962189Z error: 1 errors occurred comparing output.
2019-12-01T23:21:12.2962251Z status: exit code: 1
2019-12-01T23:21:12.2962251Z status: exit code: 1
2019-12-01T23:21:12.2963068Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn_fn_ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_fn_ptr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_fn_ptr/auxiliary" "-A" "unused"
2019-12-01T23:21:12.2963397Z ------------------------------------------
2019-12-01T23:21:12.2963510Z 
2019-12-01T23:21:12.2963768Z ------------------------------------------
2019-12-01T23:21:12.2963814Z stderr:
2019-12-01T23:21:12.2963814Z stderr:
2019-12-01T23:21:12.2978942Z ------------------------------------------
2019-12-01T23:21:12.2979024Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2979555Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_fn_ptr.rs:10:30
2019-12-01T23:21:12.2979623Z    |
2019-12-01T23:21:12.2979694Z LL | const fn no_inner_dyn_trait2(x: Hide) {
2019-12-01T23:21:12.2979784Z    |
2019-12-01T23:21:12.2979784Z    |
2019-12-01T23:21:12.2980141Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2980355Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2980448Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2980929Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_fn_ptr.rs:14:50
2019-12-01T23:21:12.2980988Z    |
2019-12-01T23:21:12.2980988Z    |
2019-12-01T23:21:12.2981398Z LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasPtr { field }) }
2019-12-01T23:21:12.2981516Z    |
2019-12-01T23:21:12.2981516Z    |
2019-12-01T23:21:12.2981814Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2981897Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2981980Z error: aborting due to 2 previous errors
2019-12-01T23:21:12.2982010Z 
2019-12-01T23:21:12.2982329Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T23:21:12.2982373Z 
---
2019-12-01T23:21:12.2983206Z 7 error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.2983461Z -   --> $DIR/min_const_fn.rs:39:36
2019-12-01T23:21:12.2983693Z +   --> $DIR/min_const_fn.rs:39:5
2019-12-01T23:21:12.2983739Z 9    |
2019-12-01T23:21:12.2984016Z 10 LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
2019-12-01T23:21:12.2984317Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T23:21:12.2984381Z 12    |
2019-12-01T23:21:12.2984381Z 12    |
2019-12-01T23:21:12.2984858Z 13    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2984914Z 14    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2985012Z 20    |                            ^^^^ constant functions cannot evaluate destructors
2019-12-01T23:21:12.2985063Z 21 
2019-12-01T23:21:12.2985108Z 22 error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.2985355Z -   --> $DIR/min_const_fn.rs:46:42
2019-12-01T23:21:12.2985355Z -   --> $DIR/min_const_fn.rs:46:42
2019-12-01T23:21:12.2985561Z +   --> $DIR/min_const_fn.rs:46:5
2019-12-01T23:21:12.2985604Z 24    |
2019-12-01T23:21:12.2985861Z 25 LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
2019-12-01T23:21:12.2986136Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T23:21:12.2986195Z 27    |
2019-12-01T23:21:12.2986195Z 27    |
2019-12-01T23:21:12.2986480Z 28    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2986535Z 29    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2986630Z 35    |                           ^^^^ constant functions cannot evaluate destructors
2019-12-01T23:21:12.2986835Z 36 
2019-12-01T23:21:12.2986897Z 37 error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.2987238Z -   --> $DIR/min_const_fn.rs:53:38
2019-12-01T23:21:12.2987238Z -   --> $DIR/min_const_fn.rs:53:38
2019-12-01T23:21:12.2987611Z +   --> $DIR/min_const_fn.rs:53:5
2019-12-01T23:21:12.2987653Z 39    |
2019-12-01T23:21:12.2987907Z 40 LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
2019-12-01T23:21:12.2988177Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T23:21:12.2988237Z 42    |
2019-12-01T23:21:12.2988237Z 42    |
2019-12-01T23:21:12.2988524Z 43    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2988659Z 44    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2988755Z 45 
2019-12-01T23:21:12.2988799Z 46 error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.2989059Z -   --> $DIR/min_const_fn.rs:58:39
2019-12-01T23:21:12.2989266Z +   --> $DIR/min_const_fn.rs:58:5
2019-12-01T23:21:12.2989266Z +   --> $DIR/min_const_fn.rs:58:5
2019-12-01T23:21:12.2989317Z 48    |
2019-12-01T23:21:12.2989571Z 49 LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
2019-12-01T23:21:12.2990002Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T23:21:12.2990042Z 51    |
2019-12-01T23:21:12.2990042Z 51    |
2019-12-01T23:21:12.2990514Z 52    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2990568Z 53    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2990661Z 179    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2990713Z 180 
2019-12-01T23:21:12.2990757Z 181 error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.2991145Z -   --> $DIR/min_const_fn.rs:105:14
2019-12-01T23:21:12.2991145Z -   --> $DIR/min_const_fn.rs:105:14
2019-12-01T23:21:12.2991341Z +   --> $DIR/min_const_fn.rs:105:27
2019-12-01T23:21:12.2991382Z 183    |
2019-12-01T23:21:12.2991441Z 184 LL | const fn inc(x: &mut i32) { *x += 1 }
2019-12-01T23:21:12.2991680Z +    |                           ^
2019-12-01T23:21:12.2991720Z 186    |
2019-12-01T23:21:12.2991720Z 186    |
2019-12-01T23:21:12.2992008Z 187    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2992064Z 188    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2992161Z 251    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2992378Z 252 
2019-12-01T23:21:12.2992425Z 253 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.2992672Z -   --> $DIR/min_const_fn.rs:132:23
2019-12-01T23:21:12.2992672Z -   --> $DIR/min_const_fn.rs:132:23
2019-12-01T23:21:12.2993935Z +   --> $DIR/min_const_fn.rs:132:49
2019-12-01T23:21:12.2993988Z 255    |
2019-12-01T23:21:12.2994054Z 256 LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {}
2019-12-01T23:21:12.2994343Z +    |                                                 ^
2019-12-01T23:21:12.2994419Z 258    |
2019-12-01T23:21:12.2994419Z 258    |
2019-12-01T23:21:12.2994720Z 259    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2994777Z 260    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2994867Z 261 
2019-12-01T23:21:12.2994917Z 262 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.2995144Z -   --> $DIR/min_const_fn.rs:133:32
2019-12-01T23:21:12.2995379Z +   --> $DIR/min_const_fn.rs:133:1
2019-12-01T23:21:12.2995379Z +   --> $DIR/min_const_fn.rs:133:1
2019-12-01T23:21:12.2995423Z 264    |
2019-12-01T23:21:12.2995684Z 265 LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-12-01T23:21:12.2996005Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T23:21:12.2996049Z 267    |
2019-12-01T23:21:12.2996049Z 267    |
2019-12-01T23:21:12.2996353Z 268    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2996527Z 269    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2996617Z 270 
2019-12-01T23:21:12.2996666Z 271 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.2996925Z -   --> $DIR/min_const_fn.rs:138:41
2019-12-01T23:21:12.2997477Z +   --> $DIR/min_const_fn.rs:138:39
2019-12-01T23:21:12.2997477Z +   --> $DIR/min_const_fn.rs:138:39
2019-12-01T23:21:12.2997687Z 273    |
2019-12-01T23:21:12.2997734Z 274 LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
2019-12-01T23:21:12.2998136Z +    |                                       ^
2019-12-01T23:21:12.2998177Z 276    |
2019-12-01T23:21:12.2998177Z 276    |
2019-12-01T23:21:12.2998575Z 277    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2998632Z 278    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.2998708Z 279 
2019-12-01T23:21:12.2998770Z 280 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.2998989Z -   --> $DIR/min_const_fn.rs:141:21
2019-12-01T23:21:12.2999195Z +   --> $DIR/min_const_fn.rs:141:31
2019-12-01T23:21:12.2999195Z +   --> $DIR/min_const_fn.rs:141:31
2019-12-01T23:21:12.2999256Z 282    |
2019-12-01T23:21:12.2999298Z 283 LL | const fn no_fn_ptrs(_x: fn()) {}
2019-12-01T23:21:12.2999561Z +    |                               ^
2019-12-01T23:21:12.2999601Z 285    |
2019-12-01T23:21:12.2999601Z 285    |
2019-12-01T23:21:12.2999880Z 286    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.2999962Z 287    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3000034Z 288 
2019-12-01T23:21:12.3000080Z 289 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.3000381Z -   --> $DIR/min_const_fn.rs:143:27
2019-12-01T23:21:12.3000608Z +   --> $DIR/min_const_fn.rs:143:1
2019-12-01T23:21:12.3000608Z +   --> $DIR/min_const_fn.rs:143:1
2019-12-01T23:21:12.3000662Z 291    |
2019-12-01T23:21:12.3000931Z 292 LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
2019-12-01T23:21:12.3001208Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-01T23:21:12.3001269Z 294    |
2019-12-01T23:21:12.3001269Z 294    |
2019-12-01T23:21:12.3001571Z 295    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3001627Z 296    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3001703Z 
2019-12-01T23:21:12.3001749Z The actual stderr differed from the expected stderr.
2019-12-01T23:21:12.3002088Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/min_const_fn.stderr
2019-12-01T23:21:12.3002088Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/min_const_fn.stderr
2019-12-01T23:21:12.3002375Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T23:21:12.3002656Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`
2019-12-01T23:21:12.3002762Z error: 1 errors occurred comparing output.
2019-12-01T23:21:12.3002807Z status: exit code: 1
2019-12-01T23:21:12.3002807Z status: exit code: 1
2019-12-01T23:21:12.3003788Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/auxiliary" "-A" "unused"
2019-12-01T23:21:12.3005027Z ------------------------------------------
2019-12-01T23:21:12.3005086Z 
2019-12-01T23:21:12.3007756Z ------------------------------------------
2019-12-01T23:21:12.3007980Z stderr:
2019-12-01T23:21:12.3007980Z stderr:
2019-12-01T23:21:12.3008299Z ------------------------------------------
2019-12-01T23:21:12.3008763Z error[E0493]: destructors cannot be evaluated at compile-time
2019-12-01T23:21:12.3009035Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:37:25
2019-12-01T23:21:12.3009111Z    |
2019-12-01T23:21:12.3009565Z LL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-12-01T23:21:12.3009690Z 
2019-12-01T23:21:12.3009737Z error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3010250Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:5
2019-12-01T23:21:12.3010323Z    |
2019-12-01T23:21:12.3010323Z    |
2019-12-01T23:21:12.3011330Z LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
2019-12-01T23:21:12.3011433Z    |
2019-12-01T23:21:12.3011433Z    |
2019-12-01T23:21:12.3011952Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3012014Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3012317Z error[E0493]: destructors cannot be evaluated at compile-time
2019-12-01T23:21:12.3012759Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:44:28
2019-12-01T23:21:12.3012812Z    |
2019-12-01T23:21:12.3012812Z    |
2019-12-01T23:21:12.3013094Z LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-12-01T23:21:12.3013181Z 
2019-12-01T23:21:12.3013237Z error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3013502Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:46:5
2019-12-01T23:21:12.3013549Z    |
2019-12-01T23:21:12.3013549Z    |
2019-12-01T23:21:12.3013780Z LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
2019-12-01T23:21:12.3045485Z    |
2019-12-01T23:21:12.3045485Z    |
2019-12-01T23:21:12.3046082Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3046172Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3046480Z error[E0493]: destructors cannot be evaluated at compile-time
2019-12-01T23:21:12.3046751Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:51:27
2019-12-01T23:21:12.3046801Z    |
2019-12-01T23:21:12.3046801Z    |
2019-12-01T23:21:12.3047051Z LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructors
2019-12-01T23:21:12.3047185Z 
2019-12-01T23:21:12.3047233Z error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3047487Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:53:5
2019-12-01T23:21:12.3047716Z    |
2019-12-01T23:21:12.3047716Z    |
2019-12-01T23:21:12.3047950Z LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
2019-12-01T23:21:12.3048061Z    |
2019-12-01T23:21:12.3048061Z    |
2019-12-01T23:21:12.3048337Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3048392Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3048484Z error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3048730Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:58:5
2019-12-01T23:21:12.3048792Z    |
2019-12-01T23:21:12.3048792Z    |
2019-12-01T23:21:12.3049023Z LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
2019-12-01T23:21:12.3049134Z    |
2019-12-01T23:21:12.3049134Z    |
2019-12-01T23:21:12.3049403Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3049456Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3050005Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.3050303Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:76:16
2019-12-01T23:21:12.3050350Z    |
2019-12-01T23:21:12.3050350Z    |
2019-12-01T23:21:12.3050596Z LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
2019-12-01T23:21:12.3050684Z    |
2019-12-01T23:21:12.3050684Z    |
2019-12-01T23:21:12.3096977Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3097085Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3097409Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.3098243Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:78:18
2019-12-01T23:21:12.3098296Z    |
2019-12-01T23:21:12.3098296Z    |
2019-12-01T23:21:12.3098555Z LL | const fn foo11_2<T: Send>(t: T) -> T { t }
2019-12-01T23:21:12.3098661Z    |
2019-12-01T23:21:12.3098661Z    |
2019-12-01T23:21:12.3098990Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3099051Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3099084Z 
2019-12-01T23:21:12.3099134Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-12-01T23:21:12.3099466Z    |
2019-12-01T23:21:12.3099466Z    |
2019-12-01T23:21:12.3099691Z LL | const fn foo19(f: f32) -> f32 { f * 2.0 }
2019-12-01T23:21:12.3099812Z    |
2019-12-01T23:21:12.3099812Z    |
2019-12-01T23:21:12.3100101Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3100174Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3100206Z 
2019-12-01T23:21:12.3100253Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-12-01T23:21:12.3100590Z    |
2019-12-01T23:21:12.3100590Z    |
2019-12-01T23:21:12.3100818Z LL | const fn foo19_2(f: f32) -> f32 { 2.0 - f }
2019-12-01T23:21:12.3100921Z    |
2019-12-01T23:21:12.3100921Z    |
2019-12-01T23:21:12.3101209Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3101280Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3101363Z error[E0723]: only int and `bool` operations are stable in const fn
2019-12-01T23:21:12.3101667Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:84:35
2019-12-01T23:21:12.3101892Z    |
2019-12-01T23:21:12.3101892Z    |
2019-12-01T23:21:12.3102115Z LL | const fn foo19_3(f: f32) -> f32 { -f }
2019-12-01T23:21:12.3102222Z    |
2019-12-01T23:21:12.3102222Z    |
2019-12-01T23:21:12.3102509Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3102564Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3102611Z 
2019-12-01T23:21:12.3102657Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-12-01T23:21:12.3102973Z    |
2019-12-01T23:21:12.3102973Z    |
2019-12-01T23:21:12.3103200Z LL | const fn foo19_4(f: f32, g: f32) -> f32 { f / g }
2019-12-01T23:21:12.3103305Z    |
2019-12-01T23:21:12.3103305Z    |
2019-12-01T23:21:12.3103761Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3103818Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3104117Z error[E0723]: cannot access `static` items in const fn
2019-12-01T23:21:12.3104447Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:90:27
2019-12-01T23:21:12.3104652Z    |
2019-12-01T23:21:12.3104652Z    |
2019-12-01T23:21:12.3104964Z LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot access `static` items in const fn
2019-12-01T23:21:12.3105063Z    |
2019-12-01T23:21:12.3105063Z    |
2019-12-01T23:21:12.3105384Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3105442Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3105531Z error[E0723]: cannot access `static` items in const fn
2019-12-01T23:21:12.3105794Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:91:37
2019-12-01T23:21:12.3105937Z    |
2019-12-01T23:21:12.3105937Z    |
2019-12-01T23:21:12.3106253Z LL | const fn foo26() -> &'static u32 { &BAR } //~ ERROR cannot access `static` items
2019-12-01T23:21:12.3106352Z    |
2019-12-01T23:21:12.3106352Z    |
2019-12-01T23:21:12.3106660Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3106727Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3106824Z error[E0723]: casting pointers to ints is unstable in const fn
2019-12-01T23:21:12.3107091Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:92:42
2019-12-01T23:21:12.3107139Z    |
2019-12-01T23:21:12.3107139Z    |
2019-12-01T23:21:12.3107372Z LL | const fn foo30(x: *const u32) -> usize { x as usize }
2019-12-01T23:21:12.3107483Z    |
2019-12-01T23:21:12.3107483Z    |
2019-12-01T23:21:12.3107779Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3107845Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3107924Z error[E0723]: casting pointers to ints is unstable in const fn
2019-12-01T23:21:12.3108351Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:94:63
2019-12-01T23:21:12.3108398Z    |
2019-12-01T23:21:12.3108398Z    |
2019-12-01T23:21:12.3108654Z LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
2019-12-01T23:21:12.3108768Z    |
2019-12-01T23:21:12.3108768Z    |
2019-12-01T23:21:12.3109203Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3109268Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3109340Z error[E0723]: casting pointers to ints is unstable in const fn
2019-12-01T23:21:12.3109598Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:96:42
2019-12-01T23:21:12.3109642Z    |
2019-12-01T23:21:12.3109642Z    |
2019-12-01T23:21:12.3109867Z LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
2019-12-01T23:21:12.3109976Z    |
2019-12-01T23:21:12.3109976Z    |
2019-12-01T23:21:12.3110439Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3110696Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3110957Z error[E0723]: casting pointers to ints is unstable in const fn
2019-12-01T23:21:12.3111419Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:98:63
2019-12-01T23:21:12.3111476Z    |
2019-12-01T23:21:12.3111476Z    |
2019-12-01T23:21:12.3111728Z LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
2019-12-01T23:21:12.3112012Z    |
2019-12-01T23:21:12.3112012Z    |
2019-12-01T23:21:12.3112450Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3112512Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3112606Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-12-01T23:21:12.3112863Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:101:44
2019-12-01T23:21:12.3112922Z    |
2019-12-01T23:21:12.3112922Z    |
2019-12-01T23:21:12.3113649Z LL | const fn foo36(a: bool, b: bool) -> bool { a && b }
2019-12-01T23:21:12.3113776Z    |
2019-12-01T23:21:12.3113776Z    |
2019-12-01T23:21:12.3114114Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3114172Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3114265Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-12-01T23:21:12.3114528Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:103:44
2019-12-01T23:21:12.3114575Z    |
2019-12-01T23:21:12.3114575Z    |
2019-12-01T23:21:12.3114933Z LL | const fn foo37(a: bool, b: bool) -> bool { a || b }
2019-12-01T23:21:12.3115040Z    |
2019-12-01T23:21:12.3115040Z    |
2019-12-01T23:21:12.3115379Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3115439Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3115544Z error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3115839Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:105:27
2019-12-01T23:21:12.3115890Z    |
2019-12-01T23:21:12.3115890Z    |
2019-12-01T23:21:12.3115947Z LL | const fn inc(x: &mut i32) { *x += 1 }
2019-12-01T23:21:12.3116041Z    |
2019-12-01T23:21:12.3116041Z    |
2019-12-01T23:21:12.3116361Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3116421Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3116531Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.3116823Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:110:6
2019-12-01T23:21:12.3116874Z    |
2019-12-01T23:21:12.3116874Z    |
2019-12-01T23:21:12.3116920Z LL | impl<T: std::fmt::Debug> Foo<T> {
2019-12-01T23:21:12.3117033Z    |
2019-12-01T23:21:12.3117033Z    |
2019-12-01T23:21:12.3117345Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3117422Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3117506Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.3117809Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:115:6
2019-12-01T23:21:12.3117860Z    |
2019-12-01T23:21:12.3117860Z    |
2019-12-01T23:21:12.3117907Z LL | impl<T: std::fmt::Debug + Sized> Foo<T> {
2019-12-01T23:21:12.3118011Z    |
2019-12-01T23:21:12.3118011Z    |
2019-12-01T23:21:12.3118527Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3118648Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3118728Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.3119026Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:120:6
2019-12-01T23:21:12.3119084Z    |
2019-12-01T23:21:12.3119084Z    |
2019-12-01T23:21:12.3119128Z LL | impl<T: Sync + Sized> Foo<T> {
2019-12-01T23:21:12.3119224Z    |
2019-12-01T23:21:12.3119224Z    |
2019-12-01T23:21:12.3119528Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3119585Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3119675Z error[E0723]: `impl Trait` in const fn is unstable
2019-12-01T23:21:12.3120843Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:126:24
2019-12-01T23:21:12.3120940Z    |
2019-12-01T23:21:12.3120940Z    |
2019-12-01T23:21:12.3121228Z LL | const fn no_rpit2() -> AlanTuring<impl std::fmt::Debug> { AlanTuring(0) }
2019-12-01T23:21:12.3121345Z    |
2019-12-01T23:21:12.3121345Z    |
2019-12-01T23:21:12.3121658Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3121715Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3122022Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.3122319Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:128:34
2019-12-01T23:21:12.3122381Z    |
2019-12-01T23:21:12.3122381Z    |
2019-12-01T23:21:12.3122427Z LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
2019-12-01T23:21:12.3122519Z    |
2019-12-01T23:21:12.3122519Z    |
2019-12-01T23:21:12.3122823Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3122969Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3123068Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.3123348Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:130:22
2019-12-01T23:21:12.3123393Z    |
2019-12-01T23:21:12.3123393Z    |
2019-12-01T23:21:12.3123463Z LL | const fn no_apit(_x: impl std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
2019-12-01T23:21:12.3123556Z    |
2019-12-01T23:21:12.3123556Z    |
2019-12-01T23:21:12.3127487Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3127735Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3128405Z error[E0723]: `impl Trait` in const fn is unstable
2019-12-01T23:21:12.3128784Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:131:23
2019-12-01T23:21:12.3128834Z    |
2019-12-01T23:21:12.3128834Z    |
2019-12-01T23:21:12.3129144Z LL | const fn no_rpit() -> impl std::fmt::Debug {} //~ ERROR `impl Trait` in const fn is unstable
2019-12-01T23:21:12.3129249Z    |
2019-12-01T23:21:12.3129249Z    |
2019-12-01T23:21:12.3129588Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3129650Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3129741Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.3130218Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:132:49
2019-12-01T23:21:12.3130269Z    |
2019-12-01T23:21:12.3130269Z    |
2019-12-01T23:21:12.3130321Z LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
2019-12-01T23:21:12.3130440Z    |
2019-12-01T23:21:12.3130440Z    |
2019-12-01T23:21:12.3130759Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3130829Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3130912Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.3131194Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:133:1
2019-12-01T23:21:12.3131252Z    |
2019-12-01T23:21:12.3131252Z    |
2019-12-01T23:21:12.3131495Z LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-12-01T23:21:12.3131607Z    |
2019-12-01T23:21:12.3131607Z    |
2019-12-01T23:21:12.3131891Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3131960Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3132040Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-12-01T23:21:12.3132474Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:138:39
2019-12-01T23:21:12.3132526Z    |
2019-12-01T23:21:12.3132526Z    |
2019-12-01T23:21:12.3132575Z LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
2019-12-01T23:21:12.3132676Z    |
2019-12-01T23:21:12.3132676Z    |
2019-12-01T23:21:12.3132957Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3133514Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3133768Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.3134359Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:141:31
2019-12-01T23:21:12.3134430Z    |
2019-12-01T23:21:12.3134430Z    |
2019-12-01T23:21:12.3134473Z LL | const fn no_fn_ptrs(_x: fn()) {}
2019-12-01T23:21:12.3134579Z    |
2019-12-01T23:21:12.3134579Z    |
2019-12-01T23:21:12.3134888Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3135070Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3135181Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.3135503Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:143:1
2019-12-01T23:21:12.3135571Z    |
2019-12-01T23:21:12.3135571Z    |
2019-12-01T23:21:12.3135826Z LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
2019-12-01T23:21:12.3135935Z    |
2019-12-01T23:21:12.3135935Z    |
2019-12-01T23:21:12.3136900Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3137000Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3137102Z error: aborting due to 34 previous errors
2019-12-01T23:21:12.3137133Z 
2019-12-01T23:21:12.3137180Z Some errors have detailed explanations: E0493, E0723.
2019-12-01T23:21:12.3137714Z For more information about an error, try `rustc --explain E0493`.
---
2019-12-01T23:21:12.3139277Z -    |         ^
2019-12-01T23:21:12.3139321Z + LL |     let mut a = 0;
2019-12-01T23:21:12.3139379Z +    |         ^^^^^
2019-12-01T23:21:12.3139420Z 6    |
2019-12-01T23:21:12.3139728Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3139800Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3139879Z 9 
2019-12-01T23:21:12.3139925Z 10 error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3140168Z -   --> $DIR/mutable_borrow.rs:12:13
2019-12-01T23:21:12.3140539Z +   --> $DIR/mutable_borrow.rs:11:13
2019-12-01T23:21:12.3140539Z +   --> $DIR/mutable_borrow.rs:11:13
2019-12-01T23:21:12.3140589Z 12    |
2019-12-01T23:21:12.3140983Z - LL |         let b = &mut a;
2019-12-01T23:21:12.3141178Z -    |             ^
2019-12-01T23:21:12.3141222Z + LL |         let mut a = 0;
2019-12-01T23:21:12.3141280Z +    |             ^^^^^
2019-12-01T23:21:12.3141320Z 15    |
2019-12-01T23:21:12.3141605Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3141674Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3141730Z 
2019-12-01T23:21:12.3141772Z The actual stderr differed from the expected stderr.
2019-12-01T23:21:12.3142108Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/mutable_borrow/mutable_borrow.stderr
2019-12-01T23:21:12.3142108Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/mutable_borrow/mutable_borrow.stderr
2019-12-01T23:21:12.3142348Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T23:21:12.3143075Z To only update this specific test, also pass `--test-args consts/min_const_fn/mutable_borrow.rs`
2019-12-01T23:21:12.3143193Z error: 1 errors occurred comparing output.
2019-12-01T23:21:12.3143364Z status: exit code: 1
2019-12-01T23:21:12.3143364Z status: exit code: 1
2019-12-01T23:21:12.3144951Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/mutable_borrow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/mutable_borrow" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/mutable_borrow/auxiliary" "-A" "unused"
2019-12-01T23:21:12.3145451Z ------------------------------------------
2019-12-01T23:21:12.3145487Z 
2019-12-01T23:21:12.3145709Z ------------------------------------------
2019-12-01T23:21:12.3145768Z stderr:
2019-12-01T23:21:12.3145768Z stderr:
2019-12-01T23:21:12.3145986Z ------------------------------------------
2019-12-01T23:21:12.3146048Z error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3146733Z   --> /checkout/src/test/ui/consts/min_const_fn/mutable_borrow.rs:2:9
2019-12-01T23:21:12.3146794Z    |
2019-12-01T23:21:12.3146836Z LL |     let mut a = 0;
2019-12-01T23:21:12.3146881Z    |         ^^^^^
2019-12-01T23:21:12.3146938Z    |
2019-12-01T23:21:12.3147422Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3147503Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3147583Z error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3147897Z   --> /checkout/src/test/ui/consts/min_const_fn/mutable_borrow.rs:11:13
2019-12-01T23:21:12.3147967Z    |
2019-12-01T23:21:12.3148012Z LL |         let mut a = 0;
2019-12-01T23:21:12.3148012Z LL |         let mut a = 0;
2019-12-01T23:21:12.3148058Z    |             ^^^^^
2019-12-01T23:21:12.3148117Z    |
2019-12-01T23:21:12.3148410Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3148476Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3148566Z error: aborting due to 2 previous errors
2019-12-01T23:21:12.3148595Z 
2019-12-01T23:21:12.3148873Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T23:21:12.3148919Z 
---
2019-12-01T23:21:12.3149731Z 1 error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.3150418Z -   --> $DIR/issue-37550.rs:3:9
2019-12-01T23:21:12.3150628Z +   --> $DIR/issue-37550.rs:2:9
2019-12-01T23:21:12.3150671Z 3    |
2019-12-01T23:21:12.3150855Z - LL |     let x = || t;
2019-12-01T23:21:12.3150916Z + LL |     let t = true;
2019-12-01T23:21:12.3151004Z 6    |
2019-12-01T23:21:12.3151004Z 6    |
2019-12-01T23:21:12.3151313Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3151372Z 
2019-12-01T23:21:12.3151416Z The actual stderr differed from the expected stderr.
2019-12-01T23:21:12.3151733Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37550/issue-37550.stderr
2019-12-01T23:21:12.3151733Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37550/issue-37550.stderr
2019-12-01T23:21:12.3151973Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T23:21:12.3152234Z To only update this specific test, also pass `--test-args issues/issue-37550.rs`
2019-12-01T23:21:12.3152316Z error: 1 errors occurred comparing output.
2019-12-01T23:21:12.3152359Z status: exit code: 1
2019-12-01T23:21:12.3152359Z status: exit code: 1
2019-12-01T23:21:12.3153078Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37550.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37550" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37550/auxiliary" "-A" "unused"
2019-12-01T23:21:12.3153945Z ------------------------------------------
2019-12-01T23:21:12.3153984Z 
2019-12-01T23:21:12.3154225Z ------------------------------------------
2019-12-01T23:21:12.3154289Z stderr:
2019-12-01T23:21:12.3154289Z stderr:
2019-12-01T23:21:12.3154508Z ------------------------------------------
2019-12-01T23:21:12.3154658Z error[E0723]: function pointers in const fn are unstable
2019-12-01T23:21:12.3154952Z   --> /checkout/src/test/ui/issues/issue-37550.rs:2:9
2019-12-01T23:21:12.3155003Z    |
2019-12-01T23:21:12.3155046Z LL |     let t = true;
2019-12-01T23:21:12.3155091Z    |         ^
2019-12-01T23:21:12.3155146Z    |
2019-12-01T23:21:12.3155451Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3155534Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3155612Z error: aborting due to previous error
2019-12-01T23:21:12.3155641Z 
2019-12-01T23:21:12.3155919Z For more information about this error, try `rustc --explain E0723`.
2019-12-01T23:21:12.3155955Z 
---
2019-12-01T23:21:12.3156607Z 1 error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3156847Z -   --> $DIR/ranged_ints2_const.rs:11:9
2019-12-01T23:21:12.3157067Z +   --> $DIR/ranged_ints2_const.rs:10:9
2019-12-01T23:21:12.3157275Z 3    |
2019-12-01T23:21:12.3157483Z - LL |     let y = &mut x.0;
2019-12-01T23:21:12.3157719Z + LL |     let mut x = unsafe { NonZero(1) };
2019-12-01T23:21:12.3157775Z +    |         ^^^^^
2019-12-01T23:21:12.3157813Z 6    |
2019-12-01T23:21:12.3157813Z 6    |
2019-12-01T23:21:12.3158477Z 7    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3158557Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3158630Z 9 
2019-12-01T23:21:12.3158691Z 10 error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3158955Z -   --> $DIR/ranged_ints2_const.rs:18:9
2019-12-01T23:21:12.3159168Z +   --> $DIR/ranged_ints2_const.rs:17:9
2019-12-01T23:21:12.3159168Z +   --> $DIR/ranged_ints2_const.rs:17:9
2019-12-01T23:21:12.3159221Z 12    |
2019-12-01T23:21:12.3159449Z - LL |     let y = unsafe { &mut x.0 };
2019-12-01T23:21:12.3159680Z + LL |     let mut x = unsafe { NonZero(1) };
2019-12-01T23:21:12.3159740Z +    |         ^^^^^
2019-12-01T23:21:12.3159780Z 15    |
2019-12-01T23:21:12.3159780Z 15    |
2019-12-01T23:21:12.3160222Z 16    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3160302Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3160357Z 
2019-12-01T23:21:12.3160399Z The actual stderr differed from the expected stderr.
2019-12-01T23:21:12.3160719Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const/ranged_ints2_const.stderr
2019-12-01T23:21:12.3160719Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const/ranged_ints2_const.stderr
2019-12-01T23:21:12.3160955Z To update references, rerun the tests and pass the `--bless` flag
2019-12-01T23:21:12.3161222Z To only update this specific test, also pass `--test-args unsafe/ranged_ints2_const.rs`
2019-12-01T23:21:12.3161305Z error: 1 errors occurred comparing output.
2019-12-01T23:21:12.3161346Z status: exit code: 1
2019-12-01T23:21:12.3161346Z status: exit code: 1
2019-12-01T23:21:12.3162080Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints2_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints2_const/auxiliary" "-A" "unused"
2019-12-01T23:21:12.3162526Z ------------------------------------------
2019-12-01T23:21:12.3162558Z 
2019-12-01T23:21:12.3162761Z ------------------------------------------
2019-12-01T23:21:12.3162820Z stderr:
2019-12-01T23:21:12.3162820Z stderr:
2019-12-01T23:21:12.3163085Z ------------------------------------------
2019-12-01T23:21:12.3163141Z error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3163406Z   --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:10:9
2019-12-01T23:21:12.3163454Z    |
2019-12-01T23:21:12.3163496Z LL |     let mut x = unsafe { NonZero(1) };
2019-12-01T23:21:12.3170860Z    |         ^^^^^
2019-12-01T23:21:12.3170950Z    |
2019-12-01T23:21:12.3171491Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3171574Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3171657Z error[E0723]: mutable references in const fn are unstable
2019-12-01T23:21:12.3171928Z   --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:17:9
2019-12-01T23:21:12.3171997Z    |
2019-12-01T23:21:12.3172044Z LL |     let mut x = unsafe { NonZero(1) };
2019-12-01T23:21:12.3172044Z LL |     let mut x = unsafe { NonZero(1) };
2019-12-01T23:21:12.3172090Z    |         ^^^^^
2019-12-01T23:21:12.3172147Z    |
2019-12-01T23:21:12.3172458Z    = note: for more information, see issue ***/issues/57563
2019-12-01T23:21:12.3172517Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-12-01T23:21:12.3172616Z error[E0133]: mutation of layout constrained field is unsafe and requires unsafe function or block
2019-12-01T23:21:12.3173041Z   --> /checkout/src/test/ui/unsafe/ranged_ints2_const.rs:11:13
2019-12-01T23:21:12.3173107Z    |
2019-12-01T23:21:12.3173107Z    |
2019-12-01T23:21:12.3173155Z LL |     let y = &mut x.0; //~ ERROR references in const fn are unstable
2019-12-01T23:21:12.3173205Z    |             ^^^^^^^^ mutation of layout constrained field
2019-12-01T23:21:12.3173317Z    = note: mutating layout constrained fields cannot statically be checked for valid values
2019-12-01T23:21:12.3173351Z 
2019-12-01T23:21:12.3173393Z error: aborting due to 3 previous errors
2019-12-01T23:21:12.3173437Z 
---
2019-12-01T23:21:12.3176357Z test result: FAILED. 9263 passed; 10 failed; 43 ignored; 0 measured; 0 filtered out
2019-12-01T23:21:12.3176394Z 
2019-12-01T23:21:12.3176520Z 
2019-12-01T23:21:12.3176549Z 
2019-12-01T23:21:12.3178416Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-01T23:21:12.3178686Z 
2019-12-01T23:21:12.3178717Z 
2019-12-01T23:21:12.3179021Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-01T23:21:12.3179100Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-01T23:21:12.3179100Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-01T23:21:12.3179166Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-01T23:21:12.3179216Z Build completed unsuccessfully in 1:07:01
2019-12-01T23:21:12.3179278Z == clock drift check ==
2019-12-01T23:21:12.3179324Z   local time: Sun Dec  1 23:21:12 UTC 2019
2019-12-01T23:21:12.8477335Z   network time: Sun, 01 Dec 2019 23:21:12 GMT
2019-12-01T23:21:12.8478024Z == end clock drift check ==
2019-12-01T23:21:13.6175372Z 
2019-12-01T23:21:13.6310950Z ##[error]Bash exited with code '1'.
2019-12-01T23:21:13.6344848Z ##[section]Starting: Checkout
2019-12-01T23:21:13.6346634Z ==============================================================================
2019-12-01T23:21:13.6346708Z Task         : Get sources
2019-12-01T23:21:13.6346759Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
