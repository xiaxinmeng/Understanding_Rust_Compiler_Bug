plain
2019-11-27T03:44:39.5124345Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-27T03:44:39.5347921Z ##[command]git config gc.auto 0
2019-11-27T03:44:40.5380989Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-27T03:44:40.5389260Z ##[command]git config --get-all http.proxy
2019-11-27T03:44:40.5395396Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66606/merge:refs/remotes/pull/66606/merge
---
2019-11-27T03:49:20.3859141Z Successfully built 91817817496b
2019-11-27T03:49:20.4660158Z Successfully tagged rust-ci:latest
2019-11-27T03:49:20.5358490Z Built container sha256:91817817496b887c0d78e5c266c845e3af407d0ed64b8c6401584401fd133e5c
2019-11-27T03:49:20.5376385Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/6b5affe011ff1d5f3b21dfc83ec51b259362cc85cfb8f36f49672c02efc91e505476306baee6903f09190f74e5a67b9efc5b48aff5c2d6d0c38ad646ffda3ea7
2019-11-27T03:50:04.7185010Z upload failed: - to s3://rust-lang-ci-sccache2/docker/6b5affe011ff1d5f3b21dfc83ec51b259362cc85cfb8f36f49672c02efc91e505476306baee6903f09190f74e5a67b9efc5b48aff5c2d6d0c38ad646ffda3ea7 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2019-11-27T03:50:05.8187046Z [CI_JOB_NAME=x86_64-gnu-llvm-6.0]
2019-11-27T03:50:05.8214724Z == clock drift check ==
2019-11-27T03:50:05.8224321Z   local time: Wed Nov 27 03:50:05 UTC 2019
2019-11-27T03:50:06.0999934Z   network time: Wed, 27 Nov 2019 03:50:06 GMT
---
2019-11-27T04:46:37.7146633Z .................................................................................................... 1600/9297
2019-11-27T04:46:42.3976803Z .................................................................................................... 1700/9297
2019-11-27T04:46:54.8395376Z ..................................i................................................................. 1800/9297
2019-11-27T04:47:02.4972513Z .................................................................................................... 1900/9297
2019-11-27T04:47:16.2255676Z ...................iiiii............................................................................ 2000/9297
2019-11-27T04:47:26.0929044Z .................................................................................................... 2200/9297
2019-11-27T04:47:28.5615267Z ............F....................................................................................... 2300/9297
2019-11-27T04:47:33.4700861Z .................................................................................................... 2400/9297
2019-11-27T04:47:54.6679993Z .................................................................................................... 2500/9297
---
2019-11-27T04:50:33.5653804Z ....................i...............i............................................................... 4800/9297
2019-11-27T04:50:43.7074670Z .................................................................................................... 4900/9297
2019-11-27T04:50:49.5200170Z .................................................................................................... 5000/9297
2019-11-27T04:50:57.9021573Z .................................................................................................... 5100/9297
2019-11-27T04:51:05.2620786Z .........................ii.ii...........i.......................................................... 5200/9297
2019-11-27T04:51:14.4683211Z .................................................................................................... 5400/9297
2019-11-27T04:51:25.1246353Z .................................................................................................... 5500/9297
2019-11-27T04:51:32.1372667Z .......i............................................................................................ 5600/9297
2019-11-27T04:51:38.3858375Z .................................................................................................... 5700/9297
2019-11-27T04:51:38.3858375Z .................................................................................................... 5700/9297
2019-11-27T04:51:49.2302355Z .............................................................................................ii...i. 5800/9297
2019-11-27T04:52:02.0625557Z .ii...........i..................................................................................... 5900/9297
2019-11-27T04:52:20.0253979Z .................................................................................................... 6100/9297
2019-11-27T04:52:26.2294593Z .................................................................................................... 6200/9297
2019-11-27T04:52:26.2294593Z .................................................................................................... 6200/9297
2019-11-27T04:52:40.0787977Z ................i..ii............................................................................... 6300/9297
2019-11-27T04:52:59.6009745Z ....................................................................................i............... 6500/9297
2019-11-27T04:53:01.9302892Z .................................................................................................... 6600/9297
2019-11-27T04:53:04.3202190Z ...........................................................................i........................ 6700/9297
2019-11-27T04:53:07.0737686Z .................................................................................................... 6800/9297
---
2019-11-27T04:57:53.7539023Z - error[E0017]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7539199Z + error[E0658]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7539573Z 2   --> $DIR/check-static-immutable-mut-slices.rs:3:37
2019-11-27T04:57:53.7539756Z 3    |
2019-11-27T04:57:53.7540103Z 4 LL | static TEST: &'static mut [isize] = &mut [];
2019-11-27T04:57:53.7540404Z 5    |                                     ^^^^^^^ statics require immutable values
2019-11-27T04:57:53.7540527Z +    |
2019-11-27T04:57:53.7540527Z +    |
2019-11-27T04:57:53.7541022Z +    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7541206Z +    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7541471Z 7 error: aborting due to previous error
2019-11-27T04:57:53.7541593Z 8 
2019-11-27T04:57:53.7541695Z 
2019-11-27T04:57:53.7542578Z - For more information about this error, try `rustc --explain E0017`.
2019-11-27T04:57:53.7542578Z - For more information about this error, try `rustc --explain E0017`.
2019-11-27T04:57:53.7543052Z + For more information about this error, try `rustc --explain E0658`.
2019-11-27T04:57:53.7543222Z 10 
2019-11-27T04:57:53.7543374Z 
2019-11-27T04:57:53.7543486Z 
2019-11-27T04:57:53.7543618Z The actual stderr differed from the expected stderr.
2019-11-27T04:57:53.7544090Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-immutable-mut-slices/check-static-immutable-mut-slices.stderr
2019-11-27T04:57:53.7544532Z To update references, rerun the tests and pass the `--bless` flag
2019-11-27T04:57:53.7547842Z To only update this specific test, also pass `--test-args check-static-immutable-mut-slices.rs`
2019-11-27T04:57:53.7550298Z error: 1 errors occurred comparing output.
2019-11-27T04:57:53.7550359Z status: exit code: 1
2019-11-27T04:57:53.7550359Z status: exit code: 1
2019-11-27T04:57:53.7551330Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-static-immutable-mut-slices.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-immutable-mut-slices" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-immutable-mut-slices/auxiliary" "-A" "unused"
2019-11-27T04:57:53.7551850Z ------------------------------------------
2019-11-27T04:57:53.7552352Z 
2019-11-27T04:57:53.7552665Z ------------------------------------------
2019-11-27T04:57:53.7552713Z stderr:
2019-11-27T04:57:53.7552713Z stderr:
2019-11-27T04:57:53.7552931Z ------------------------------------------
2019-11-27T04:57:53.7553001Z error[E0658]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7553260Z   --> /checkout/src/test/ui/check-static-immutable-mut-slices.rs:3:37
2019-11-27T04:57:53.7553311Z    |
2019-11-27T04:57:53.7553553Z LL | static TEST: &'static mut [isize] = &mut [];
2019-11-27T04:57:53.7553608Z    |                                     ^^^^^^^ statics require immutable values
2019-11-27T04:57:53.7553664Z    |
2019-11-27T04:57:53.7553994Z    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7554178Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7554263Z error: aborting due to previous error
2019-11-27T04:57:53.7554309Z 
2019-11-27T04:57:53.7554602Z For more information about this error, try `rustc --explain E0658`.
2019-11-27T04:57:53.7554636Z 
---
2019-11-27T04:57:53.7555602Z - error[E0017]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7555652Z + error[E0658]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7555859Z 2   --> $DIR/E0017.rs:4:30
2019-11-27T04:57:53.7555910Z 3    |
2019-11-27T04:57:53.7556234Z 4 LL | const CR: &'static mut i32 = &mut C;
2019-11-27T04:57:53.7556333Z 5    |                              ^^^^^^ constants require immutable values
2019-11-27T04:57:53.7556376Z +    |
2019-11-27T04:57:53.7556376Z +    |
2019-11-27T04:57:53.7556663Z +    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7556739Z +    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7557075Z - error[E0017]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7557146Z + error[E0658]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7557371Z 8   --> $DIR/E0017.rs:5:39
2019-11-27T04:57:53.7557415Z 9    |
2019-11-27T04:57:53.7557415Z 9    |
2019-11-27T04:57:53.7557673Z 10 LL | static STATIC_REF: &'static mut i32 = &mut X;
2019-11-27T04:57:53.7557708Z 
2019-11-27T04:57:53.7557756Z 11    |                                       ^^^^^^ statics require immutable values
2019-11-27T04:57:53.7557810Z +    |
2019-11-27T04:57:53.7558131Z +    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7558196Z +    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7558305Z 13 error[E0596]: cannot borrow immutable static item `X` as mutable
2019-11-27T04:57:53.7558544Z 14   --> $DIR/E0017.rs:5:39
2019-11-27T04:57:53.7558576Z 
2019-11-27T04:57:53.7558830Z 16 LL | static STATIC_REF: &'static mut i32 = &mut X;
---
2019-11-27T04:57:53.7559848Z 22 LL | static CONST_REF: &'static mut i32 = &mut C;
2019-11-27T04:57:53.7559968Z 
2019-11-27T04:57:53.7560013Z 23    |                                      ^^^^^^ statics require immutable values
2019-11-27T04:57:53.7560080Z +    |
2019-11-27T04:57:53.7560379Z +    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7560433Z +    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7560531Z 25 error: aborting due to 4 previous errors
2019-11-27T04:57:53.7560569Z 26 
2019-11-27T04:57:53.7560593Z 
2019-11-27T04:57:53.7560838Z - Some errors have detailed explanations: E0017, E0596.
2019-11-27T04:57:53.7560838Z - Some errors have detailed explanations: E0017, E0596.
2019-11-27T04:57:53.7561066Z - For more information about an error, try `rustc --explain E0017`.
2019-11-27T04:57:53.7561113Z + Some errors have detailed explanations: E0596, E0658.
2019-11-27T04:57:53.7561354Z + For more information about an error, try `rustc --explain E0596`.
2019-11-27T04:57:53.7561396Z 29 
2019-11-27T04:57:53.7561419Z 
2019-11-27T04:57:53.7561451Z 
2019-11-27T04:57:53.7561509Z The actual stderr differed from the expected stderr.
2019-11-27T04:57:53.7562329Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/E0017.stderr
2019-11-27T04:57:53.7562693Z To update references, rerun the tests and pass the `--bless` flag
2019-11-27T04:57:53.7562982Z To only update this specific test, also pass `--test-args error-codes/E0017.rs`
2019-11-27T04:57:53.7563061Z error: 1 errors occurred comparing output.
2019-11-27T04:57:53.7563122Z status: exit code: 1
2019-11-27T04:57:53.7563122Z status: exit code: 1
2019-11-27T04:57:53.7563855Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0017.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0017/auxiliary" "-A" "unused"
2019-11-27T04:57:53.7564195Z ------------------------------------------
2019-11-27T04:57:53.7564229Z 
2019-11-27T04:57:53.7564442Z ------------------------------------------
2019-11-27T04:57:53.7564504Z stderr:
2019-11-27T04:57:53.7564504Z stderr:
2019-11-27T04:57:53.7564714Z ------------------------------------------
2019-11-27T04:57:53.7564766Z error[E0658]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7565017Z   --> /checkout/src/test/ui/error-codes/E0017.rs:4:30
2019-11-27T04:57:53.7565065Z    |
2019-11-27T04:57:53.7565409Z LL | const CR: &'static mut i32 = &mut C; //~ ERROR E0017
2019-11-27T04:57:53.7565476Z    |                              ^^^^^^ constants require immutable values
2019-11-27T04:57:53.7565517Z    |
2019-11-27T04:57:53.7565789Z    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7565868Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7565939Z error[E0658]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7566196Z   --> /checkout/src/test/ui/error-codes/E0017.rs:5:39
2019-11-27T04:57:53.7566240Z    |
2019-11-27T04:57:53.7566240Z    |
2019-11-27T04:57:53.7566466Z LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
2019-11-27T04:57:53.7566534Z    |                                       ^^^^^^ statics require immutable values
2019-11-27T04:57:53.7566576Z    |
2019-11-27T04:57:53.7566836Z    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7566904Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7566976Z error[E0596]: cannot borrow immutable static item `X` as mutable
2019-11-27T04:57:53.7567324Z   --> /checkout/src/test/ui/error-codes/E0017.rs:5:39
2019-11-27T04:57:53.7567385Z    |
2019-11-27T04:57:53.7567385Z    |
2019-11-27T04:57:53.7567748Z LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
2019-11-27T04:57:53.7567848Z 
2019-11-27T04:57:53.7567899Z error[E0658]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7568127Z   --> /checkout/src/test/ui/error-codes/E0017.rs:7:38
2019-11-27T04:57:53.7601422Z    |
2019-11-27T04:57:53.7601422Z    |
2019-11-27T04:57:53.7602104Z LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017
2019-11-27T04:57:53.7602170Z    |                                      ^^^^^^ statics require immutable values
2019-11-27T04:57:53.7602229Z    |
2019-11-27T04:57:53.7602579Z    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7602640Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7602732Z error: aborting due to 4 previous errors
2019-11-27T04:57:53.7602761Z 
2019-11-27T04:57:53.7602804Z Some errors have detailed explanations: E0596, E0658.
2019-11-27T04:57:53.7603105Z For more information about an error, try `rustc --explain E0596`.
---
2019-11-27T04:57:53.7604172Z - error[E0017]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7604226Z + error[E0658]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7604432Z 2   --> $DIR/E0388.rs:4:30
2019-11-27T04:57:53.7604475Z 3    |
2019-11-27T04:57:53.7604709Z 4 LL | const CR: &'static mut i32 = &mut C;
2019-11-27T04:57:53.7604788Z 5    |                              ^^^^^^ constants require immutable values
2019-11-27T04:57:53.7604843Z +    |
2019-11-27T04:57:53.7604843Z +    |
2019-11-27T04:57:53.7605254Z +    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7605437Z +    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7605738Z - error[E0017]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7605789Z + error[E0658]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7605987Z 8   --> $DIR/E0388.rs:5:39
2019-11-27T04:57:53.7606027Z 9    |
2019-11-27T04:57:53.7606027Z 9    |
2019-11-27T04:57:53.7606238Z 10 LL | static STATIC_REF: &'static mut i32 = &mut X;
2019-11-27T04:57:53.7606268Z 
2019-11-27T04:57:53.7606319Z 11    |                                       ^^^^^^ statics require immutable values
2019-11-27T04:57:53.7606360Z +    |
2019-11-27T04:57:53.7606621Z +    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7606684Z +    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7606783Z 13 error[E0596]: cannot borrow immutable static item `X` as mutable
2019-11-27T04:57:53.7607123Z 14   --> $DIR/E0388.rs:5:39
2019-11-27T04:57:53.7607153Z 
2019-11-27T04:57:53.7607376Z 16 LL | static STATIC_REF: &'static mut i32 = &mut X;
---
2019-11-27T04:57:53.7609030Z 22 LL | static CONST_REF: &'static mut i32 = &mut C;
2019-11-27T04:57:53.7609133Z 
2019-11-27T04:57:53.7609197Z 23    |                                      ^^^^^^ statics require immutable values
2019-11-27T04:57:53.7609250Z +    |
2019-11-27T04:57:53.7609683Z +    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7609957Z +    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7610071Z 25 error: aborting due to 4 previous errors
2019-11-27T04:57:53.7610130Z 26 
2019-11-27T04:57:53.7610160Z 
2019-11-27T04:57:53.7610566Z - Some errors have detailed explanations: E0017, E0596.
2019-11-27T04:57:53.7610566Z - Some errors have detailed explanations: E0017, E0596.
2019-11-27T04:57:53.7610907Z - For more information about an error, try `rustc --explain E0017`.
2019-11-27T04:57:53.7610966Z + Some errors have detailed explanations: E0596, E0658.
2019-11-27T04:57:53.7611238Z + For more information about an error, try `rustc --explain E0596`.
2019-11-27T04:57:53.7611304Z 29 
2019-11-27T04:57:53.7611332Z 
2019-11-27T04:57:53.7611360Z 
2019-11-27T04:57:53.7611408Z The actual stderr differed from the expected stderr.
2019-11-27T04:57:53.7611749Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/E0388.stderr
2019-11-27T04:57:53.7612031Z To update references, rerun the tests and pass the `--bless` flag
2019-11-27T04:57:53.7612891Z To only update this specific test, also pass `--test-args error-codes/E0388.rs`
2019-11-27T04:57:53.7613142Z error: 1 errors occurred comparing output.
2019-11-27T04:57:53.7613202Z status: exit code: 1
2019-11-27T04:57:53.7613202Z status: exit code: 1
2019-11-27T04:57:53.7614078Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0388.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0388/auxiliary" "-A" "unused"
2019-11-27T04:57:53.7614439Z ------------------------------------------
2019-11-27T04:57:53.7614477Z 
2019-11-27T04:57:53.7614737Z ------------------------------------------
2019-11-27T04:57:53.7614786Z stderr:
2019-11-27T04:57:53.7614786Z stderr:
2019-11-27T04:57:53.7615039Z ------------------------------------------
2019-11-27T04:57:53.7615106Z error[E0658]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7615373Z   --> /checkout/src/test/ui/error-codes/E0388.rs:4:30
2019-11-27T04:57:53.7615439Z    |
2019-11-27T04:57:53.7615701Z LL | const CR: &'static mut i32 = &mut C; //~ ERROR E0017
2019-11-27T04:57:53.7615802Z    |                              ^^^^^^ constants require immutable values
2019-11-27T04:57:53.7615861Z    |
2019-11-27T04:57:53.7616198Z    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7616263Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7616363Z error[E0658]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7616646Z   --> /checkout/src/test/ui/error-codes/E0388.rs:5:39
2019-11-27T04:57:53.7616718Z    |
2019-11-27T04:57:53.7616718Z    |
2019-11-27T04:57:53.7616989Z LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
2019-11-27T04:57:53.7617051Z    |                                       ^^^^^^ statics require immutable values
2019-11-27T04:57:53.7617122Z    |
2019-11-27T04:57:53.7617438Z    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7617500Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7617599Z error[E0596]: cannot borrow immutable static item `X` as mutable
2019-11-27T04:57:53.7617876Z   --> /checkout/src/test/ui/error-codes/E0388.rs:5:39
2019-11-27T04:57:53.7617926Z    |
2019-11-27T04:57:53.7617926Z    |
2019-11-27T04:57:53.7618203Z LL | static STATIC_REF: &'static mut i32 = &mut X; //~ ERROR E0017
2019-11-27T04:57:53.7618296Z 
2019-11-27T04:57:53.7618357Z error[E0658]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7618760Z   --> /checkout/src/test/ui/error-codes/E0388.rs:7:38
2019-11-27T04:57:53.7618810Z    |
2019-11-27T04:57:53.7618810Z    |
2019-11-27T04:57:53.7619084Z LL | static CONST_REF: &'static mut i32 = &mut C; //~ ERROR E0017
2019-11-27T04:57:53.7619154Z    |                                      ^^^^^^ statics require immutable values
2019-11-27T04:57:53.7619231Z    |
2019-11-27T04:57:53.7619572Z    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7619633Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7619717Z error: aborting due to 4 previous errors
2019-11-27T04:57:53.7619760Z 
2019-11-27T04:57:53.7619808Z Some errors have detailed explanations: E0596, E0658.
2019-11-27T04:57:53.7620100Z For more information about an error, try `rustc --explain E0596`.
---
2019-11-27T04:57:53.7621198Z - error[E0017]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7621284Z + error[E0658]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7621573Z 2   --> $DIR/issue-17718-const-bad-values.rs:1:34
2019-11-27T04:57:53.7621623Z 3    |
2019-11-27T04:57:53.7621882Z 4 LL | const C1: &'static mut [usize] = &mut [];
2019-11-27T04:57:53.7621971Z 5    |                                  ^^^^^^^ constants require immutable values
2019-11-27T04:57:53.7622021Z +    |
2019-11-27T04:57:53.7622021Z +    |
2019-11-27T04:57:53.7622732Z +    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7622804Z +    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7622916Z 7 error[E0013]: constants cannot refer to statics, use a constant instead
2019-11-27T04:57:53.7623216Z 8   --> $DIR/issue-17718-const-bad-values.rs:5:46
2019-11-27T04:57:53.7623254Z 
2019-11-27T04:57:53.7623254Z 
2019-11-27T04:57:53.7623529Z 10 LL | const C2: &'static mut usize = unsafe { &mut S };
2019-11-27T04:57:53.7623633Z 12 
2019-11-27T04:57:53.7623914Z - error[E0017]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7623984Z + error[E0658]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7624242Z 14   --> $DIR/issue-17718-const-bad-values.rs:5:41
2019-11-27T04:57:53.7624242Z 14   --> $DIR/issue-17718-const-bad-values.rs:5:41
2019-11-27T04:57:53.7624292Z 15    |
2019-11-27T04:57:53.7624558Z 16 LL | const C2: &'static mut usize = unsafe { &mut S };
2019-11-27T04:57:53.7624647Z 17    |                                         ^^^^^^ constants require immutable values
2019-11-27T04:57:53.7624709Z +    |
2019-11-27T04:57:53.7624709Z +    |
2019-11-27T04:57:53.7625022Z +    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7625095Z +    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7625213Z 19 error: aborting due to 3 previous errors
2019-11-27T04:57:53.7625259Z 20 
2019-11-27T04:57:53.7625286Z 
2019-11-27T04:57:53.7625577Z - Some errors have detailed explanations: E0013, E0017.
2019-11-27T04:57:53.7625577Z - Some errors have detailed explanations: E0013, E0017.
2019-11-27T04:57:53.7625633Z + Some errors have detailed explanations: E0013, E0658.
2019-11-27T04:57:53.7625905Z 22 For more information about an error, try `rustc --explain E0013`.
2019-11-27T04:57:53.7625966Z 23 
2019-11-27T04:57:53.7625994Z 
2019-11-27T04:57:53.7626022Z 
2019-11-27T04:57:53.7626069Z The actual stderr differed from the expected stderr.
2019-11-27T04:57:53.7626448Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-const-bad-values/issue-17718-const-bad-values.stderr
2019-11-27T04:57:53.7626733Z To update references, rerun the tests and pass the `--bless` flag
2019-11-27T04:57:53.7627213Z To only update this specific test, also pass `--test-args issues/issue-17718-const-bad-values.rs`
2019-11-27T04:57:53.7627314Z error: 1 errors occurred comparing output.
2019-11-27T04:57:53.7627371Z status: exit code: 1
2019-11-27T04:57:53.7627371Z status: exit code: 1
2019-11-27T04:57:53.7635655Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17718-const-bad-values.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-const-bad-values" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17718-const-bad-values/auxiliary" "-A" "unused"
2019-11-27T04:57:53.7636276Z ------------------------------------------
2019-11-27T04:57:53.7636336Z 
2019-11-27T04:57:53.7636593Z ------------------------------------------
2019-11-27T04:57:53.7636662Z stderr:
2019-11-27T04:57:53.7636662Z stderr:
2019-11-27T04:57:53.7636904Z ------------------------------------------
2019-11-27T04:57:53.7637127Z error[E0658]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7637471Z   --> /checkout/src/test/ui/issues/issue-17718-const-bad-values.rs:1:34
2019-11-27T04:57:53.7637547Z    |
2019-11-27T04:57:53.7637799Z LL | const C1: &'static mut [usize] = &mut [];
2019-11-27T04:57:53.7637859Z    |                                  ^^^^^^^ constants require immutable values
2019-11-27T04:57:53.7637924Z    |
2019-11-27T04:57:53.7638278Z    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7638356Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7638447Z error[E0013]: constants cannot refer to statics, use a constant instead
2019-11-27T04:57:53.7638749Z   --> /checkout/src/test/ui/issues/issue-17718-const-bad-values.rs:5:46
2019-11-27T04:57:53.7638829Z    |
2019-11-27T04:57:53.7638829Z    |
2019-11-27T04:57:53.7639085Z LL | const C2: &'static mut usize = unsafe { &mut S };
2019-11-27T04:57:53.7639195Z 
2019-11-27T04:57:53.7639246Z error[E0658]: references in constants may only refer to immutable values
2019-11-27T04:57:53.7639892Z   --> /checkout/src/test/ui/issues/issue-17718-const-bad-values.rs:5:41
2019-11-27T04:57:53.7639952Z    |
2019-11-27T04:57:53.7639952Z    |
2019-11-27T04:57:53.7640225Z LL | const C2: &'static mut usize = unsafe { &mut S };
2019-11-27T04:57:53.7640285Z    |                                         ^^^^^^ constants require immutable values
2019-11-27T04:57:53.7640335Z    |
2019-11-27T04:57:53.7640677Z    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7640740Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7640834Z error: aborting due to 3 previous errors
2019-11-27T04:57:53.7640878Z 
2019-11-27T04:57:53.7640926Z Some errors have detailed explanations: E0013, E0658.
2019-11-27T04:57:53.7641221Z For more information about an error, try `rustc --explain E0013`.
---
2019-11-27T04:57:53.7642683Z - error[E0017]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7642775Z + error[E0658]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7643031Z 2   --> $DIR/issue-46604.rs:1:25
2019-11-27T04:57:53.7643081Z 3    |
2019-11-27T04:57:53.7643143Z 4 LL | static buf: &mut [u8] = &mut [1u8,2,3,4,5,7];
2019-11-27T04:57:53.7643228Z 5    |                         ^^^^^^^^^^^^^^^^^^^^ statics require immutable values
2019-11-27T04:57:53.7643452Z +    |
2019-11-27T04:57:53.7643452Z +    |
2019-11-27T04:57:53.7643844Z +    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7643919Z +    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7643982Z 6 
2019-11-27T04:57:53.7644035Z 7 error[E0594]: cannot assign to `buf[_]`, as `buf` is an immutable static item
2019-11-27T04:57:53.7648040Z 8   --> $DIR/issue-46604.rs:6:5
2019-11-27T04:57:53.7648636Z 12 
2019-11-27T04:57:53.7648691Z 13 error: aborting due to 2 previous errors
2019-11-27T04:57:53.7665405Z 14 
2019-11-27T04:57:53.7665975Z - Some errors have detailed explanations: E0017, E0594.
---
2019-11-27T04:57:53.7666721Z 
2019-11-27T04:57:53.7666749Z 
2019-11-27T04:57:53.7666813Z The actual stderr differed from the expected stderr.
2019-11-27T04:57:53.7667310Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604/issue-46604.stderr
2019-11-27T04:57:53.7667645Z To update references, rerun the tests and pass the `--bless` flag
2019-11-27T04:57:53.7667957Z To only update this specific test, also pass `--test-args issues/issue-46604.rs`
2019-11-27T04:57:53.7668046Z error: 1 errors occurred comparing output.
2019-11-27T04:57:53.7668095Z status: exit code: 1
2019-11-27T04:57:53.7668095Z status: exit code: 1
2019-11-27T04:57:53.7669100Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46604.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46604/auxiliary" "-A" "unused"
2019-11-27T04:57:53.7669517Z ------------------------------------------
2019-11-27T04:57:53.7669556Z 
2019-11-27T04:57:53.7669794Z ------------------------------------------
2019-11-27T04:57:53.7669856Z stderr:
2019-11-27T04:57:53.7669856Z stderr:
2019-11-27T04:57:53.7670091Z ------------------------------------------
2019-11-27T04:57:53.7670149Z error[E0658]: references in statics may only refer to immutable values
2019-11-27T04:57:53.7670422Z   --> /checkout/src/test/ui/issues/issue-46604.rs:1:25
2019-11-27T04:57:53.7670479Z    |
2019-11-27T04:57:53.7670535Z LL | static buf: &mut [u8] = &mut [1u8,2,3,4,5,7];   //~ ERROR E0017
2019-11-27T04:57:53.7670612Z    |                         ^^^^^^^^^^^^^^^^^^^^ statics require immutable values
2019-11-27T04:57:53.7670665Z    |
2019-11-27T04:57:53.7671067Z    = note: for more information, see ***/issues/57349
2019-11-27T04:57:53.7671147Z    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
2019-11-27T04:57:53.7671185Z 
2019-11-27T04:57:53.7671250Z error[E0594]: cannot assign to `buf[_]`, as `buf` is an immutable static item
2019-11-27T04:57:53.7671581Z   --> /checkout/src/test/ui/issues/issue-46604.rs:6:5
2019-11-27T04:57:53.7671637Z    |
2019-11-27T04:57:53.7671691Z LL |     buf[0]=2;                                   //~ ERROR E0594
2019-11-27T04:57:53.7671794Z 
2019-11-27T04:57:53.7671844Z error: aborting due to 2 previous errors
2019-11-27T04:57:53.7671878Z 
2019-11-27T04:57:53.7671929Z Some errors have detailed explanations: E0594, E0658.
---
2019-11-27T04:57:53.7675194Z test result: FAILED. 9249 passed; 5 failed; 43 ignored; 0 measured; 0 filtered out
2019-11-27T04:57:53.7675236Z 
2019-11-27T04:57:53.7677438Z 
2019-11-27T04:57:53.7677488Z 
2019-11-27T04:57:53.7679382Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-27T04:57:53.7679714Z 
2019-11-27T04:57:53.7679744Z 
2019-11-27T04:57:53.7679795Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-27T04:57:53.7679869Z Build completed unsuccessfully in 1:04:36
2019-11-27T04:57:53.7679869Z Build completed unsuccessfully in 1:04:36
2019-11-27T04:57:53.7680214Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-27T04:57:53.7680281Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-27T04:57:53.7680352Z == clock drift check ==
2019-11-27T04:57:53.7680402Z   local time: Wed Nov 27 04:57:53 UTC 2019
2019-11-27T04:57:54.2991671Z   network time: Wed, 27 Nov 2019 04:57:54 GMT
2019-11-27T04:57:54.2995399Z == end clock drift check ==
2019-11-27T04:57:55.1466194Z 
2019-11-27T04:57:55.1574089Z ##[error]Bash exited with code '1'.
2019-11-27T04:57:55.1612190Z ##[section]Starting: Checkout
2019-11-27T04:57:55.1614253Z ==============================================================================
2019-11-27T04:57:55.1614322Z Task         : Get sources
2019-11-27T04:57:55.1614385Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
