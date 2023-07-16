plain
2019-12-11T15:07:08.9531614Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-11T15:07:08.9730792Z ##[command]git config gc.auto 0
2019-12-11T15:07:08.9808308Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-11T15:07:09.9553354Z ##[command]git config --get-all http.proxy
2019-12-11T15:07:09.9561090Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67224/merge:refs/remotes/pull/67224/merge
---
2019-12-11T16:06:35.2032070Z .................................................................................................... 1600/9353
2019-12-11T16:06:39.7485136Z .................................................................................................... 1700/9353
2019-12-11T16:06:51.7866712Z ......................................................i............................................. 1800/9353
2019-12-11T16:07:00.0413766Z .................................................................................................... 1900/9353
2019-12-11T16:07:14.2724365Z .......................................iiiii........................................................ 2000/9353
2019-12-11T16:07:24.4290201Z .................................................................................................... 2200/9353
2019-12-11T16:07:26.9921538Z .................................................................................................... 2300/9353
2019-12-11T16:07:31.0407131Z .................................................................................................... 2400/9353
2019-12-11T16:07:53.1034719Z .................................................................................................... 2500/9353
---
2019-12-11T16:10:31.6851073Z ................................................i...............i................................... 4800/9353
2019-12-11T16:10:39.7048112Z .................................................................................................... 4900/9353
2019-12-11T16:10:47.8219136Z ............................................................................................i....... 5000/9353
2019-12-11T16:10:53.1591043Z .................................................................................................... 5100/9353
2019-12-11T16:11:03.5229399Z ..........................................................ii.ii...........i......................... 5200/9353
2019-12-11T16:11:12.6508316Z .................................................................................................... 5400/9353
2019-12-11T16:11:22.6815347Z ..........................................................F......................................... 5500/9353
2019-12-11T16:11:29.2755627Z ........................................i........................................................... 5600/9353
2019-12-11T16:11:36.1298638Z .................................................................................................... 5700/9353
2019-12-11T16:11:36.1298638Z .................................................................................................... 5700/9353
2019-12-11T16:11:47.4442257Z .................................................................................................... 5800/9353
2019-12-11T16:11:58.4248729Z ............................ii...i..ii...........i.................................................. 5900/9353
2019-12-11T16:12:16.4746686Z .................................................................................................... 6100/9353
2019-12-11T16:12:24.7713985Z .................................................................................................... 6200/9353
2019-12-11T16:12:24.7713985Z .................................................................................................... 6200/9353
2019-12-11T16:12:34.2310740Z ....................................................i..ii........................................... 6300/9353
2019-12-11T16:13:00.8536890Z .................................................................................................... 6500/9353
2019-12-11T16:13:03.0645166Z ........................i........................................................................... 6600/9353
2019-12-11T16:13:05.4299374Z .................................................................................................... 6700/9353
2019-12-11T16:13:07.9368886Z ...............i.................................................................................... 6800/9353
---
2019-12-11T16:14:45.9286514Z .................................................................................................... 7400/9353
2019-12-11T16:14:50.7981362Z .................................................................................................... 7500/9353
2019-12-11T16:14:57.9511199Z .................................................................................................... 7600/9353
2019-12-11T16:15:06.8290404Z .................................................................................................... 7700/9353
2019-12-11T16:15:14.4565856Z ...............................iiii................................................................. 7800/9353
2019-12-11T16:15:28.9966589Z .................................................................................................... 8000/9353
2019-12-11T16:15:37.7873938Z .................................................................................................... 8100/9353
2019-12-11T16:15:52.0497466Z .................................................................................................... 8200/9353
2019-12-11T16:15:59.8643398Z .................................................................................................... 8300/9353
---
2019-12-11T16:17:54.3825217Z -   --> $DIR/index-out-of-bounds-never-type.rs:7:61
2019-12-11T16:17:54.3825503Z + error[E0658]: The `!` type is experimental
2019-12-11T16:17:54.3825910Z +   --> $DIR/index-out-of-bounds-never-type.rs:7:17
2019-12-11T16:17:54.3826191Z 3    |
2019-12-11T16:17:54.3826412Z 4 LL |     const VOID: ! = { let x = 0 * std::mem::size_of::<T>(); [][x] };
2019-12-11T16:17:54.3827354Z -    |                                                             |
2019-12-11T16:17:54.3828437Z -    |                                                             index out of bounds: the len is 0 but the index is 0
2019-12-11T16:17:54.3828798Z +    |                 ^
2019-12-11T16:17:54.3829030Z 8    |
2019-12-11T16:17:54.3829030Z 8    |
2019-12-11T16:17:54.3829478Z - note: lint level defined here
2019-12-11T16:17:54.3831775Z -   --> $DIR/index-out-of-bounds-never-type.rs:2:9
2019-12-11T16:17:54.3832408Z -    |
2019-12-11T16:17:54.3832856Z - LL | #![warn(const_err)]
2019-12-11T16:17:54.3833295Z -    |         ^^^^^^^^^
2019-12-11T16:17:54.3833907Z +    = note: for more information, see ***/issues/35121
2019-12-11T16:17:54.3834388Z +    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-11T16:17:54.3835058Z - error: erroneous constant encountered
2019-12-11T16:17:54.3835521Z -   --> $DIR/index-out-of-bounds-never-type.rs:12:13
2019-12-11T16:17:54.3836099Z -    |
2019-12-11T16:17:54.3836099Z -    |
2019-12-11T16:17:54.3836520Z - LL |     let _ = PrintName::<T>::VOID;
2019-12-11T16:17:54.3837561Z - 
2019-12-11T16:17:54.3837803Z 21 error: aborting due to previous error
2019-12-11T16:17:54.3838472Z 22 
2019-12-11T16:17:54.3838996Z + For more information about this error, try `rustc --explain E0658`.
2019-12-11T16:17:54.3838996Z + For more information about this error, try `rustc --explain E0658`.
2019-12-11T16:17:54.3839265Z 23 
2019-12-11T16:17:54.3839490Z 
2019-12-11T16:17:54.3839678Z 
2019-12-11T16:17:54.3839888Z The actual stderr differed from the expected stderr.
2019-12-11T16:17:54.3840474Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index-out-of-bounds-never-type/index-out-of-bounds-never-type.stderr
2019-12-11T16:17:54.3841069Z To update references, rerun the tests and pass the `--bless` flag
2019-12-11T16:17:54.3841986Z To only update this specific test, also pass `--test-args consts/const-eval/index-out-of-bounds-never-type.rs`
2019-12-11T16:17:54.3842461Z error: 1 errors occurred comparing output.
2019-12-11T16:17:54.3844888Z status: exit code: 1
2019-12-11T16:17:54.3844888Z status: exit code: 1
2019-12-11T16:17:54.3846267Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index-out-of-bounds-never-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/index-out-of-bounds-never-type/auxiliary" "-A" "unused"
2019-12-11T16:17:54.3848296Z ------------------------------------------
2019-12-11T16:17:54.3848484Z 
2019-12-11T16:17:54.3849271Z ------------------------------------------
2019-12-11T16:17:54.3849464Z stderr:
2019-12-11T16:17:54.3849464Z stderr:
2019-12-11T16:17:54.3849827Z ------------------------------------------
2019-12-11T16:17:54.3855551Z error[E0658]: The `!` type is experimental
2019-12-11T16:17:54.3857021Z   --> /checkout/src/test/ui/consts/const-eval/index-out-of-bounds-never-type.rs:7:17
2019-12-11T16:17:54.3857291Z    |
2019-12-11T16:17:54.3857449Z LL |     const VOID: ! = { let x = 0 * std::mem::size_of::<T>(); [][x] };
2019-12-11T16:17:54.3858617Z    |
2019-12-11T16:17:54.3858617Z    |
2019-12-11T16:17:54.3859217Z    = note: for more information, see ***/issues/35121
2019-12-11T16:17:54.3859436Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-11T16:17:54.3859815Z error: aborting due to previous error
2019-12-11T16:17:54.3860056Z 
2019-12-11T16:17:54.3860551Z For more information about this error, try `rustc --explain E0658`.
2019-12-11T16:17:54.3861292Z 
---
2019-12-11T16:17:54.3864565Z -   --> $DIR/panic-assoc-never-type.rs:8:21
2019-12-11T16:17:54.3864610Z + error[E0658]: The `!` type is experimental
2019-12-11T16:17:54.3864806Z +   --> $DIR/panic-assoc-never-type.rs:8:17
2019-12-11T16:17:54.3864861Z 3    |
2019-12-11T16:17:54.3864902Z 4 LL |     const VOID: ! = panic!();
2019-12-11T16:17:54.3865269Z -    |                     |
2019-12-11T16:17:54.3865707Z -    |                     the evaluated program panicked at 'explicit panic', $DIR/panic-assoc-never-type.rs:8:21
2019-12-11T16:17:54.3865771Z +    |                 ^
2019-12-11T16:17:54.3865813Z 8    |
2019-12-11T16:17:54.3865813Z 8    |
2019-12-11T16:17:54.3866131Z - note: lint level defined here
2019-12-11T16:17:54.3866320Z -   --> $DIR/panic-assoc-never-type.rs:2:9
2019-12-11T16:17:54.3866475Z -    |
2019-12-11T16:17:54.3866667Z - LL | #![warn(const_err)]
2019-12-11T16:17:54.3866836Z -    |         ^^^^^^^^^
2019-12-11T16:17:54.3867122Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-11T16:17:54.3867461Z +    = note: for more information, see ***/issues/35121
2019-12-11T16:17:54.3867520Z +    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-11T16:17:54.3867808Z - error[E0080]: erroneous constant used
2019-12-11T16:17:54.3868019Z -   --> $DIR/panic-assoc-never-type.rs:13:13
2019-12-11T16:17:54.3868198Z -    |
2019-12-11T16:17:54.3868198Z -    |
2019-12-11T16:17:54.3868414Z - LL |     let _ = PrintName::VOID;
2019-12-11T16:17:54.3868643Z -    |             ^^^^^^^^^^^^^^^ referenced constant has errors
2019-12-11T16:17:54.3868879Z 22 error: aborting due to previous error
2019-12-11T16:17:54.3868920Z 23 
2019-12-11T16:17:54.3869152Z - For more information about this error, try `rustc --explain E0080`.
2019-12-11T16:17:54.3869403Z + For more information about this error, try `rustc --explain E0658`.
2019-12-11T16:17:54.3869403Z + For more information about this error, try `rustc --explain E0658`.
2019-12-11T16:17:54.3869446Z 25 
2019-12-11T16:17:54.3869472Z 
2019-12-11T16:17:54.3869499Z 
2019-12-11T16:17:54.3869558Z The actual stderr differed from the expected stderr.
2019-12-11T16:17:54.3869865Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-assoc-never-type/panic-assoc-never-type.stderr
2019-12-11T16:17:54.3870253Z To update references, rerun the tests and pass the `--bless` flag
2019-12-11T16:17:54.3870524Z To only update this specific test, also pass `--test-args consts/const-eval/panic-assoc-never-type.rs`
2019-12-11T16:17:54.3870596Z error: 1 errors occurred comparing output.
2019-12-11T16:17:54.3870641Z status: exit code: 1
2019-12-11T16:17:54.3870641Z status: exit code: 1
2019-12-11T16:17:54.3871592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-assoc-never-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-assoc-never-type/auxiliary" "-A" "unused"
2019-12-11T16:17:54.3871917Z ------------------------------------------
2019-12-11T16:17:54.3872119Z 
2019-12-11T16:17:54.3872496Z ------------------------------------------
2019-12-11T16:17:54.3872559Z stderr:
2019-12-11T16:17:54.3872559Z stderr:
2019-12-11T16:17:54.3872762Z ------------------------------------------
2019-12-11T16:17:54.3872816Z error[E0658]: The `!` type is experimental
2019-12-11T16:17:54.3873077Z   --> /checkout/src/test/ui/consts/const-eval/panic-assoc-never-type.rs:8:17
2019-12-11T16:17:54.3873125Z    |
2019-12-11T16:17:54.3873170Z LL |     const VOID: ! = panic!();
2019-12-11T16:17:54.3873269Z    |
2019-12-11T16:17:54.3873269Z    |
2019-12-11T16:17:54.3873542Z    = note: for more information, see ***/issues/35121
2019-12-11T16:17:54.3873611Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-11T16:17:54.3873684Z error: aborting due to previous error
2019-12-11T16:17:54.3873712Z 
2019-12-11T16:17:54.3874284Z For more information about this error, try `rustc --explain E0658`.
2019-12-11T16:17:54.3874314Z 
---
2019-12-11T16:17:54.3875599Z -   --> $DIR/panic-never-type.rs:5:17
2019-12-11T16:17:54.3875641Z + error[E0658]: The `!` type is experimental
2019-12-11T16:17:54.3875833Z +   --> $DIR/panic-never-type.rs:5:13
2019-12-11T16:17:54.3875872Z 3    |
2019-12-11T16:17:54.3875909Z 4 LL | const VOID: ! = panic!();
2019-12-11T16:17:54.3876270Z -    |                 |
2019-12-11T16:17:54.3876510Z -    |                 the evaluated program panicked at 'explicit panic', $DIR/panic-never-type.rs:5:17
2019-12-11T16:17:54.3876557Z +    |             ^
2019-12-11T16:17:54.3876611Z 8    |
2019-12-11T16:17:54.3876611Z 8    |
2019-12-11T16:17:54.3876784Z - note: lint level defined here
2019-12-11T16:17:54.3876968Z -   --> $DIR/panic-never-type.rs:2:9
2019-12-11T16:17:54.3877136Z -    |
2019-12-11T16:17:54.3877304Z - LL | #![warn(const_err)]
2019-12-11T16:17:54.3877652Z -    |         ^^^^^^^^^
2019-12-11T16:17:54.3879391Z -    = note: this warning originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)
2019-12-11T16:17:54.3879803Z +    = note: for more information, see ***/issues/35121
2019-12-11T16:17:54.3879869Z +    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-11T16:17:54.3880313Z - error[E0080]: erroneous constant used
2019-12-11T16:17:54.3880876Z -   --> $DIR/panic-never-type.rs:9:13
2019-12-11T16:17:54.3881223Z -    |
2019-12-11T16:17:54.3881556Z - LL |     let _ = VOID;
---
2019-12-11T16:17:54.3882548Z 
2019-12-11T16:17:54.3882571Z 
2019-12-11T16:17:54.3882627Z The actual stderr differed from the expected stderr.
2019-12-11T16:17:54.3882909Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-never-type/panic-never-type.stderr
2019-12-11T16:17:54.3883126Z To update references, rerun the tests and pass the `--bless` flag
2019-12-11T16:17:54.3883388Z To only update this specific test, also pass `--test-args consts/const-eval/panic-never-type.rs`
2019-12-11T16:17:54.3883459Z error: 1 errors occurred comparing output.
2019-12-11T16:17:54.3883515Z status: exit code: 1
2019-12-11T16:17:54.3883515Z status: exit code: 1
2019-12-11T16:17:54.3884228Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/panic-never-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-never-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/panic-never-type/auxiliary" "-A" "unused"
2019-12-11T16:17:54.3884527Z ------------------------------------------
2019-12-11T16:17:54.3884556Z 
2019-12-11T16:17:54.3884759Z ------------------------------------------
2019-12-11T16:17:54.3884798Z stderr:
2019-12-11T16:17:54.3884798Z stderr:
2019-12-11T16:17:54.3884985Z ------------------------------------------
2019-12-11T16:17:54.3885046Z error[E0658]: The `!` type is experimental
2019-12-11T16:17:54.3885379Z   --> /checkout/src/test/ui/consts/const-eval/panic-never-type.rs:5:13
2019-12-11T16:17:54.3885434Z    |
2019-12-11T16:17:54.3885475Z LL | const VOID: ! = panic!();
2019-12-11T16:17:54.3885630Z    |
2019-12-11T16:17:54.3885630Z    |
2019-12-11T16:17:54.3885947Z    = note: for more information, see ***/issues/35121
2019-12-11T16:17:54.3886000Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-11T16:17:54.3886068Z error: aborting due to previous error
2019-12-11T16:17:54.3886110Z 
2019-12-11T16:17:54.3886335Z For more information about this error, try `rustc --explain E0658`.
2019-12-11T16:17:54.3886365Z 
---
2019-12-11T16:17:54.3886852Z diff of stderr:
2019-12-11T16:17:54.3886877Z 
2019-12-11T16:17:54.3886930Z 29    |             ^^^^^
2019-12-11T16:17:54.3886967Z 30 
2019-12-11T16:17:54.3887017Z 31 error[E0571]: `break` with value from a `while` loop
2019-12-11T16:17:54.3887418Z +   --> $DIR/loop-break-value.rs:38:12
2019-12-11T16:17:54.3887465Z 33    |
2019-12-11T16:17:54.3887519Z 34 LL |         if break () {
2019-12-11T16:17:54.3887565Z 35    |            ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3887565Z 35    |            ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3887599Z 
2019-12-11T16:17:54.3887635Z 40    |            ^^^^^
2019-12-11T16:17:54.3887690Z 41 
2019-12-11T16:17:54.3887730Z 42 error[E0571]: `break` with value from a `while` loop
2019-12-11T16:17:54.3888123Z +   --> $DIR/loop-break-value.rs:43:9
2019-12-11T16:17:54.3888162Z 44    |
2019-12-11T16:17:54.3888200Z 45 LL |         break None;
2019-12-11T16:17:54.3888245Z 46    |         ^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3888245Z 46    |         ^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3888290Z 
2019-12-11T16:17:54.3888335Z 51    |         ^^^^^
2019-12-11T16:17:54.3888371Z 52 
2019-12-11T16:17:54.3888428Z 53 error[E0571]: `break` with value from a `while` loop
2019-12-11T16:17:54.3889002Z +   --> $DIR/loop-break-value.rs:49:13
2019-12-11T16:17:54.3889044Z 55    |
2019-12-11T16:17:54.3889044Z 55    |
2019-12-11T16:17:54.3889261Z 56 LL |             break 'while_let_loop "nope";
2019-12-11T16:17:54.3889315Z 57    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3889391Z 
2019-12-11T16:17:54.3889432Z The actual stderr differed from the expected stderr.
2019-12-11T16:17:54.3889712Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value/loop-break-value.stderr
2019-12-11T16:17:54.3889712Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value/loop-break-value.stderr
2019-12-11T16:17:54.3889955Z To update references, rerun the tests and pass the `--bless` flag
2019-12-11T16:17:54.3890357Z To only update this specific test, also pass `--test-args loops/loop-break-value.rs`
2019-12-11T16:17:54.3890428Z error: 1 errors occurred comparing output.
2019-12-11T16:17:54.3890482Z status: exit code: 1
2019-12-11T16:17:54.3890482Z status: exit code: 1
2019-12-11T16:17:54.3891638Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/loops/loop-break-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/loops/loop-break-value/auxiliary" "-A" "unused"
2019-12-11T16:17:54.3891990Z ------------------------------------------
2019-12-11T16:17:54.3892020Z 
2019-12-11T16:17:54.3892228Z ------------------------------------------
2019-12-11T16:17:54.3892268Z stderr:
2019-12-11T16:17:54.3892268Z stderr:
2019-12-11T16:17:54.3892562Z ------------------------------------------
2019-12-11T16:17:54.3892634Z warning: denote infinite loops with `loop { ... }`
2019-12-11T16:17:54.3892869Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:26:5
2019-12-11T16:17:54.3892987Z    |
2019-12-11T16:17:54.3893242Z LL |     'while_loop: while true { //~ WARN denote infinite loops with
2019-12-11T16:17:54.3893291Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ help: use `loop`
2019-12-11T16:17:54.3893329Z    |
2019-12-11T16:17:54.3893370Z    = note: `#[warn(while_true)]` on by default
2019-12-11T16:17:54.3893439Z 
2019-12-11T16:17:54.3893479Z error[E0571]: `break` with value from a `while` loop
2019-12-11T16:17:54.3893745Z    |
2019-12-11T16:17:54.3893745Z    |
2019-12-11T16:17:54.3893789Z LL |         break (); //~ ERROR `break` with value from a `while` loop
2019-12-11T16:17:54.3893837Z    |         ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3893940Z    |
2019-12-11T16:17:54.3894264Z help: instead, use `break` on its own without a value inside this `while` loop
2019-12-11T16:17:54.3894529Z    |
2019-12-11T16:17:54.3894588Z LL |         break; //~ ERROR `break` with value from a `while` loop
2019-12-11T16:17:54.3894665Z 
2019-12-11T16:17:54.3894665Z 
2019-12-11T16:17:54.3894706Z error[E0571]: `break` with value from a `while` loop
2019-12-11T16:17:54.3896400Z    |
2019-12-11T16:17:54.3896608Z LL |             break 'while_loop 123;
2019-12-11T16:17:54.3896682Z    |             ^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3896731Z    |
2019-12-11T16:17:54.3896731Z    |
2019-12-11T16:17:54.3896779Z help: instead, use `break` on its own without a value inside this `while` loop
2019-12-11T16:17:54.3896885Z LL |             break;
2019-12-11T16:17:54.3896928Z    |             ^^^^^
2019-12-11T16:17:54.3896966Z 
2019-12-11T16:17:54.3896966Z 
2019-12-11T16:17:54.3897027Z error[E0571]: `break` with value from a `while` loop
2019-12-11T16:17:54.3897322Z    |
2019-12-11T16:17:54.3897322Z    |
2019-12-11T16:17:54.3897386Z LL |         if break () { //~ ERROR `break` with value from a `while` loop
2019-12-11T16:17:54.3897441Z    |            ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3897486Z    |
2019-12-11T16:17:54.3897550Z help: instead, use `break` on its own without a value inside this `while` loop
2019-12-11T16:17:54.3897595Z    |
2019-12-11T16:17:54.3897642Z LL |         if break { //~ ERROR `break` with value from a `while` loop
2019-12-11T16:17:54.3897734Z 
2019-12-11T16:17:54.3897734Z 
2019-12-11T16:17:54.3900600Z error[E0571]: `break` with value from a `while` loop
2019-12-11T16:17:54.3901226Z    |
2019-12-11T16:17:54.3901447Z LL |         break None;
2019-12-11T16:17:54.3901496Z    |         ^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3901539Z    |
2019-12-11T16:17:54.3901539Z    |
2019-12-11T16:17:54.3901608Z help: instead, use `break` on its own without a value inside this `while` loop
2019-12-11T16:17:54.3901689Z LL |         break;
2019-12-11T16:17:54.3901744Z    |         ^^^^^
2019-12-11T16:17:54.3901771Z 
2019-12-11T16:17:54.3901771Z 
2019-12-11T16:17:54.3901811Z error[E0571]: `break` with value from a `while` loop
2019-12-11T16:17:54.3902107Z    |
2019-12-11T16:17:54.3902107Z    |
2019-12-11T16:17:54.3902307Z LL |             break 'while_let_loop "nope";
2019-12-11T16:17:54.3902360Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3902430Z    |
2019-12-11T16:17:54.3902480Z help: instead, use `break` on its own without a value inside this `while` loop
2019-12-11T16:17:54.3902736Z LL |             break;
2019-12-11T16:17:54.3902776Z    |             ^^^^^
2019-12-11T16:17:54.3902802Z 
2019-12-11T16:17:54.3902929Z error[E0571]: `break` with value from a `for` loop
2019-12-11T16:17:54.3902929Z error[E0571]: `break` with value from a `for` loop
2019-12-11T16:17:54.3903189Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:56:9
2019-12-11T16:17:54.3903232Z    |
2019-12-11T16:17:54.3903276Z LL |         break (); //~ ERROR `break` with value from a `for` loop
2019-12-11T16:17:54.3903341Z    |         ^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3903382Z    |
2019-12-11T16:17:54.3903428Z help: instead, use `break` on its own without a value inside this `for` loop
2019-12-11T16:17:54.3903486Z    |
2019-12-11T16:17:54.3903529Z LL |         break; //~ ERROR `break` with value from a `for` loop
2019-12-11T16:17:54.3903614Z 
2019-12-11T16:17:54.3903656Z error[E0571]: `break` with value from a `for` loop
2019-12-11T16:17:54.3903884Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:57:9
2019-12-11T16:17:54.3903928Z    |
2019-12-11T16:17:54.3903928Z    |
2019-12-11T16:17:54.3903983Z LL |         break [()];
2019-12-11T16:17:54.3904029Z    |         ^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3904078Z    |
2019-12-11T16:17:54.3904137Z help: instead, use `break` on its own without a value inside this `for` loop
2019-12-11T16:17:54.3904216Z LL |         break;
2019-12-11T16:17:54.3904272Z    |         ^^^^^
2019-12-11T16:17:54.3904298Z 
2019-12-11T16:17:54.3904338Z error[E0571]: `break` with value from a `for` loop
2019-12-11T16:17:54.3904338Z error[E0571]: `break` with value from a `for` loop
2019-12-11T16:17:54.3904561Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:64:13
2019-12-11T16:17:54.3904619Z    |
2019-12-11T16:17:54.3904815Z LL |             break 'for_loop Some(17);
2019-12-11T16:17:54.3904867Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^ can only break with a value inside `loop` or breakable block
2019-12-11T16:17:54.3904926Z    |
2019-12-11T16:17:54.3904978Z help: instead, use `break` on its own without a value inside this `for` loop
2019-12-11T16:17:54.3905074Z LL |             break;
2019-12-11T16:17:54.3905121Z    |             ^^^^^
2019-12-11T16:17:54.3905146Z 
2019-12-11T16:17:54.3905185Z error[E0308]: mismatched types
2019-12-11T16:17:54.3905185Z error[E0308]: mismatched types
2019-12-11T16:17:54.3906033Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:4:31
2019-12-11T16:17:54.3906084Z    |
2019-12-11T16:17:54.3906125Z LL |     let val: ! = loop { break break; };
2019-12-11T16:17:54.3906171Z    |                               ^^^^^ expected `!`, found `()`
2019-12-11T16:17:54.3906269Z    = note:   expected type `!`
2019-12-11T16:17:54.3906311Z            found unit type `()`
2019-12-11T16:17:54.3906354Z 
2019-12-11T16:17:54.3906392Z error[E0308]: mismatched types
---
2019-12-11T16:17:54.3906803Z 
2019-12-11T16:17:54.3906857Z error[E0308]: mismatched types
2019-12-11T16:17:54.3907081Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:16:15
2019-12-11T16:17:54.3907123Z    |
2019-12-11T16:17:54.3907182Z LL |         break "asdf"; //~ ERROR mismatched types
2019-12-11T16:17:54.3907228Z    |               ^^^^^^ expected `i32`, found `&str`
2019-12-11T16:17:54.3907293Z error[E0308]: mismatched types
2019-12-11T16:17:54.3907524Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:21:31
2019-12-11T16:17:54.3907566Z    |
2019-12-11T16:17:54.3907566Z    |
2019-12-11T16:17:54.3907787Z LL |             break 'outer_loop "nope"; //~ ERROR mismatched types
2019-12-11T16:17:54.3907853Z    |                               ^^^^^^ expected `i32`, found `&str`
2019-12-11T16:17:54.3908086Z error[E0308]: mismatched types
2019-12-11T16:17:54.3908950Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:73:26
2019-12-11T16:17:54.3909032Z    |
2019-12-11T16:17:54.3909032Z    |
2019-12-11T16:17:54.3909313Z LL |                 break 'c 123; //~ ERROR mismatched types
2019-12-11T16:17:54.3909444Z    |                          ^^^ expected `()`, found integer
2019-12-11T16:17:54.3909534Z error[E0308]: mismatched types
2019-12-11T16:17:54.3909789Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:80:15
2019-12-11T16:17:54.3909835Z    |
2019-12-11T16:17:54.3909835Z    |
2019-12-11T16:17:54.3909898Z LL |         break (break, break); //~ ERROR mismatched types
2019-12-11T16:17:54.3909947Z    |               ^^^^^^^^^^^^^^ expected `()`, found tuple
2019-12-11T16:17:54.3910049Z    = note: expected unit type `()`
2019-12-11T16:17:54.3910049Z    = note: expected unit type `()`
2019-12-11T16:17:54.3910095Z                   found tuple `(!, !)`
2019-12-11T16:17:54.3910166Z error[E0308]: mismatched types
2019-12-11T16:17:54.3910417Z   --> /checkout/src/test/ui/loops/loop-break-value.rs:85:15
2019-12-11T16:17:54.3910472Z    |
2019-12-11T16:17:54.3910518Z LL |         break 2; //~ ERROR mismatched types
---
2019-12-11T16:17:54.3910955Z    |
2019-12-11T16:17:54.3910999Z LL |         break; //~ ERROR mismatched types
2019-12-11T16:17:54.3911045Z    |         ^^^^^
2019-12-11T16:17:54.3911103Z    |         |
2019-12-11T16:17:54.3911149Z    |         expected integer, found `()`
2019-12-11T16:17:54.3911199Z    |         help: give it a value of the expected type: `break value`
2019-12-11T16:17:54.3911290Z error: aborting due to 16 previous errors
2019-12-11T16:17:54.3911319Z 
2019-12-11T16:17:54.3911364Z Some errors have detailed explanations: E0308, E0571.
2019-12-11T16:17:54.3911629Z For more information about an error, try `rustc --explain E0308`.
---
2019-12-11T16:17:54.3912321Z ---- [ui] ui/never_type/auto-traits.rs stdout ----
2019-12-11T16:17:54.3912351Z 
2019-12-11T16:17:54.3912552Z error: test compilation failed although it shouldn't!
2019-12-11T16:17:54.3912596Z status: exit code: 1
2019-12-11T16:17:54.3913337Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/never_type/auto-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/auto-traits" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/auto-traits/auxiliary" "-A" "unused"
2019-12-11T16:17:54.3913635Z ------------------------------------------
2019-12-11T16:17:54.3913665Z 
2019-12-11T16:17:54.3913869Z ------------------------------------------
2019-12-11T16:17:54.3913925Z stderr:
2019-12-11T16:17:54.3913925Z stderr:
2019-12-11T16:17:54.3914119Z ------------------------------------------
2019-12-11T16:17:54.3914163Z error[E0658]: The `!` type is experimental
2019-12-11T16:17:54.3914395Z   --> /checkout/src/test/ui/never_type/auto-traits.rs:11:19
2019-12-11T16:17:54.3914440Z    |
2019-12-11T16:17:54.3914482Z LL |     assert_auto::<!>();
2019-12-11T16:17:54.3921645Z    |
2019-12-11T16:17:54.3921645Z    |
2019-12-11T16:17:54.3922138Z    = note: for more information, see ***/issues/35121
2019-12-11T16:17:54.3922221Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-11T16:17:54.3922570Z error[E0658]: The `!` type is experimental
2019-12-11T16:17:54.3922979Z   --> /checkout/src/test/ui/never_type/auto-traits.rs:15:19
2019-12-11T16:17:54.3923039Z    |
2019-12-11T16:17:54.3923039Z    |
2019-12-11T16:17:54.3923081Z LL |     assert_send::<!>();
2019-12-11T16:17:54.3923245Z    |
2019-12-11T16:17:54.3923245Z    |
2019-12-11T16:17:54.3923565Z    = note: for more information, see ***/issues/35121
2019-12-11T16:17:54.3923620Z    = help: add `#![feature(never_type)]` to the crate attributes to enable
2019-12-11T16:17:54.3923710Z error: aborting due to 2 previous errors
2019-12-11T16:17:54.3923737Z 
2019-12-11T16:17:54.3923979Z For more information about this error, try `rustc --explain E0658`.
2019-12-11T16:17:54.3924036Z 
---
2019-12-11T16:17:54.3934433Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-11T16:17:54.3934666Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-11T16:17:54.3934703Z 
2019-12-11T16:17:54.3934728Z 
2019-12-11T16:17:54.3936673Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-11T16:17:54.3937027Z 
2019-12-11T16:17:54.3937055Z 
2019-12-11T16:17:54.3937163Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-11T16:17:54.3937219Z Build completed unsuccessfully in 1:04:36
2019-12-11T16:17:54.3937219Z Build completed unsuccessfully in 1:04:36
2019-12-11T16:17:54.3944899Z == clock drift check ==
2019-12-11T16:17:54.3952605Z   local time: Wed Dec 11 16:17:54 UTC 2019
2019-12-11T16:17:54.6796292Z   network time: Wed, 11 Dec 2019 16:17:54 GMT
2019-12-11T16:17:54.6798134Z == end clock drift check ==
2019-12-11T16:17:55.5932525Z 
2019-12-11T16:17:55.6039434Z ##[error]Bash exited with code '1'.
2019-12-11T16:17:55.6073461Z ##[section]Starting: Checkout
2019-12-11T16:17:55.6075057Z ==============================================================================
2019-12-11T16:17:55.6075106Z Task         : Get sources
2019-12-11T16:17:55.6075148Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
