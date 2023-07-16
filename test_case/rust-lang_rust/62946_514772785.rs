plain
2019-07-24T18:41:13.2606905Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T18:41:13.8153634Z ##[command]git config gc.auto 0
2019-07-24T18:41:13.8158594Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T18:41:13.8160365Z ##[command]git config --get-all http.proxy
2019-07-24T18:41:13.8164085Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62946/merge:refs/remotes/pull/62946/merge
---
2019-07-24T18:41:48.3046639Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T18:41:48.3046671Z 
2019-07-24T18:41:48.3046900Z   git checkout -b <new-branch-name>
2019-07-24T18:41:48.3047123Z 
2019-07-24T18:41:48.3047171Z HEAD is now at 822247541 Merge 8f91bedff23a34b39ce3070c013a227dea66280d into 27a6a304e2baaabca88059753f020377f2476978
2019-07-24T18:41:48.3198630Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T18:41:48.3201551Z ==============================================================================
2019-07-24T18:41:48.3201605Z Task         : Bash
2019-07-24T18:41:48.3201673Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-24T19:39:47.4401187Z .................................................................................................... 200/5848
2019-07-24T19:39:51.5524611Z .................................................................................................... 300/5848
2019-07-24T19:39:55.2889387Z .................................................................................................... 400/5848
2019-07-24T19:39:59.0343909Z .................................................................................................... 500/5848
2019-07-24T19:40:02.8192303Z ........................................................................i........................... 600/5848
2019-07-24T19:40:11.6972246Z .................................................................................................... 800/5848
2019-07-24T19:40:11.6972246Z .................................................................................................... 800/5848
2019-07-24T19:40:17.2014138Z ..............F........................F.....F...................................................... 900/5848
2019-07-24T19:40:22.0738406Z ...................................................................................................i 1000/5848
2019-07-24T19:40:27.5526592Z ...........i........................................................................................ 1100/5848
2019-07-24T19:40:31.4893656Z .............................iiiii.................................................................. 1200/5848
2019-07-24T19:40:37.5014709Z .................................................................................................... 1400/5848
2019-07-24T19:40:40.2262599Z .................................................................................................... 1500/5848
2019-07-24T19:40:43.9882311Z .................................................................................................... 1600/5848
2019-07-24T19:40:46.6984687Z .................................................................................................... 1700/5848
2019-07-24T19:40:46.6984687Z .................................................................................................... 1700/5848
2019-07-24T19:40:50.1380878Z .....................................................................i.............................. 1800/5848
2019-07-24T19:40:58.7110322Z .................................................................................................... 2000/5848
2019-07-24T19:41:02.9857408Z .................................................................................................... 2100/5848
2019-07-24T19:41:06.6468968Z .................................................................................................... 2200/5848
2019-07-24T19:41:06.6468968Z .................................................................................................... 2200/5848
2019-07-24T19:41:10.5746706Z .....................................................i.............................................. 2300/5848
2019-07-24T19:41:20.4182788Z .................................................................................................... 2500/5848
2019-07-24T19:41:24.4988231Z .................................................................................................... 2600/5848
2019-07-24T19:41:29.5495380Z .................................................................................................... 2700/5848
2019-07-24T19:41:33.4332158Z .................................................................................................... 2800/5848
2019-07-24T19:41:33.4332158Z .................................................................................................... 2800/5848
2019-07-24T19:41:37.8102398Z .................................................................................................... 2900/5848
2019-07-24T19:41:42.9065703Z .................................................................................................... 3000/5848
2019-07-24T19:41:47.3648656Z ................................................................................F................... 3100/5848
2019-07-24T19:41:52.5617458Z .................................................................................................... 3200/5848
2019-07-24T19:41:56.0318602Z .................................................................................................... 3300/5848
2019-07-24T19:41:59.6657987Z .................................................................................................... 3400/5848
2019-07-24T19:42:04.7420866Z .................................................................................................... 3500/5848
2019-07-24T19:42:08.4832586Z ....................i............................................................................... 3600/5848
2019-07-24T19:42:12.5759816Z ..............................................................................................ii...i 3700/5848
2019-07-24T19:42:16.4394911Z ..ii................................................................................................ 3800/5848
2019-07-24T19:42:25.1983917Z .................................................................................................... 4000/5848
2019-07-24T19:42:25.1983917Z .................................................................................................... 4000/5848
2019-07-24T19:42:28.9395864Z ........ii.......................................................................................... 4100/5848
2019-07-24T19:42:31.0146530Z .............................i...................................................................... 4200/5848
2019-07-24T19:42:33.0249380Z ...............................................................................................i.... 4300/5848
2019-07-24T19:42:39.7090743Z .................................................................................................... 4500/5848
2019-07-24T19:42:57.2351898Z .................................................................................................... 4600/5848
2019-07-24T19:43:00.8354004Z .................................................................................................... 4700/5848
2019-07-24T19:43:04.4696343Z .................................................................................................... 4800/5848
---
2019-07-24T19:43:38.5751594Z .................................................................................................... 5400/5848
2019-07-24T19:43:42.4891756Z .................................................................................................... 5500/5848
2019-07-24T19:43:46.5771618Z .................................................................................................... 5600/5848
2019-07-24T19:43:49.7192817Z .................................................................................................... 5700/5848
2019-07-24T19:43:52.7310134Z ........................................................................................i........... 5800/5848
2019-07-24T19:43:54.4761031Z failures:
2019-07-24T19:43:54.4819068Z 
2019-07-24T19:43:54.4819561Z ---- [ui] ui/consts/const-eval/const_raw_ptr_ops.rs stdout ----
2019-07-24T19:43:54.4819622Z diff of stderr:
2019-07-24T19:43:54.4819622Z diff of stderr:
2019-07-24T19:43:54.4819654Z 
2019-07-24T19:43:54.4819697Z 9    = note: `#[deny(const_err)]` on by default
2019-07-24T19:43:54.4819759Z 10 
2019-07-24T19:43:54.4819803Z 11 error: any use of this value will cause an error
2019-07-24T19:43:54.4820030Z +   --> $DIR/const_raw_ptr_ops.rs:8:27
2019-07-24T19:43:54.4820094Z +    |
2019-07-24T19:43:54.4820139Z + LL | const X2: bool = unsafe { 42 as *const i32 == 43 as *const i32 };
2019-07-24T19:43:54.4820673Z +    | --------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
2019-07-24T19:43:54.4820755Z +    |                           |
2019-07-24T19:43:54.4820945Z +    |                           "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
2019-07-24T19:43:54.4821054Z + 
2019-07-24T19:43:54.4821123Z + error: any use of this value will cause an error
2019-07-24T19:43:54.4821667Z 13    |
2019-07-24T19:43:54.4821667Z 13    |
2019-07-24T19:43:54.4821711Z 14 LL | const Y2: usize = unsafe { &1 as *const i32 as usize + 1 };
2019-07-24T19:43:54.4821996Z -    | ---------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
2019-07-24T19:43:54.4822239Z +    | ---------------------------^^^^^^^^^^^^^^^^^^^^^^^^^-------
2019-07-24T19:43:54.4822286Z 16    |                            |
2019-07-24T19:43:54.4822286Z 16    |                            |
2019-07-24T19:43:54.4822767Z -    |                            "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
2019-07-24T19:43:54.4822986Z +    |                            a raw memory access tried to access part of a pointer value as raw bytes
2019-07-24T19:43:54.4823058Z 18 
2019-07-24T19:43:54.4823152Z 19 error: any use of this value will cause an error
2019-07-24T19:43:54.4823565Z 
2019-07-24T19:43:54.4823813Z 32    |                          |
2019-07-24T19:43:54.4823813Z 32    |                          |
2019-07-24T19:43:54.4823900Z 33    |                          a memory access tried to interpret some bytes as a pointer
2019-07-24T19:43:54.4824894Z - error: aborting due to 4 previous errors
2019-07-24T19:43:54.4825130Z + error: aborting due to 5 previous errors
2019-07-24T19:43:54.4825240Z 36 
2019-07-24T19:43:54.4825319Z 37 
2019-07-24T19:43:54.4825319Z 37 
2019-07-24T19:43:54.4825393Z 
2019-07-24T19:43:54.4825421Z 
2019-07-24T19:43:54.4825466Z The actual stderr differed from the expected stderr.
2019-07-24T19:43:54.4826026Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops/const_raw_ptr_ops.stderr
2019-07-24T19:43:54.4826304Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T19:43:54.4826570Z To only update this specific test, also pass `--test-args consts/const-eval/const_raw_ptr_ops.rs`
2019-07-24T19:43:54.4826839Z error: 1 errors occurred comparing output.
2019-07-24T19:43:54.4826891Z status: exit code: 1
2019-07-24T19:43:54.4826891Z status: exit code: 1
2019-07-24T19:43:54.4828006Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops/auxiliary" "-A" "unused"
2019-07-24T19:43:54.4828574Z ------------------------------------------
2019-07-24T19:43:54.4828608Z 
2019-07-24T19:43:54.4828824Z ------------------------------------------
2019-07-24T19:43:54.4829033Z stderr:
2019-07-24T19:43:54.4829033Z stderr:
2019-07-24T19:43:54.4829320Z ------------------------------------------
2019-07-24T19:43:54.4829505Z error: any use of this value will cause an error
2019-07-24T19:43:54.4829874Z    |
2019-07-24T19:43:54.4829874Z    |
2019-07-24T19:43:54.4830074Z LL | const X: bool = unsafe { &1 as *const i32 == &2 as *const i32 }; //~ ERROR any use of this
2019-07-24T19:43:54.4832151Z    | -------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
2019-07-24T19:43:54.4832370Z    |                          |
2019-07-24T19:43:54.4832477Z    |                          "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
2019-07-24T19:43:54.4832584Z    = note: `#[deny(const_err)]` on by default
2019-07-24T19:43:54.4832655Z 
2019-07-24T19:43:54.4832655Z 
2019-07-24T19:43:54.4832698Z error: any use of this value will cause an error
2019-07-24T19:43:54.4833201Z    |
2019-07-24T19:43:54.4833201Z    |
2019-07-24T19:43:54.4836157Z LL | const X2: bool = unsafe { 42 as *const i32 == 43 as *const i32 };
2019-07-24T19:43:54.4843015Z    | --------------------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^---
2019-07-24T19:43:54.4843101Z    |                           |
2019-07-24T19:43:54.4843333Z    |                           "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
2019-07-24T19:43:54.4843400Z 
2019-07-24T19:43:54.4843465Z error: any use of this value will cause an error
2019-07-24T19:43:54.4847515Z    |
2019-07-24T19:43:54.4847515Z    |
2019-07-24T19:43:54.4849899Z LL | const Y2: usize = unsafe { &1 as *const i32 as usize + 1 }; //~ ERROR any use of this
2019-07-24T19:43:54.4850450Z    | ---------------------------^^^^^^^^^^^^^^^^^^^^^^^^^-------
2019-07-24T19:43:54.4850547Z    |                            |
2019-07-24T19:43:54.4850743Z    |                            a raw memory access tried to access part of a pointer value as raw bytes
2019-07-24T19:43:54.4850805Z 
2019-07-24T19:43:54.4850896Z error: any use of this value will cause an error
2019-07-24T19:43:54.4851251Z    |
2019-07-24T19:43:54.4851251Z    |
2019-07-24T19:43:54.4851449Z LL | const Z2: i32 = unsafe { *(42 as *const i32) }; //~ ERROR any use of this value will cause
2019-07-24T19:43:54.4853210Z    | -------------------------^^^^^^^^^^^^^^^^^^^---
2019-07-24T19:43:54.4853382Z    |                          |
2019-07-24T19:43:54.4853482Z    |                          a memory access tried to interpret some bytes as a pointer
2019-07-24T19:43:54.4853532Z 
2019-07-24T19:43:54.4853707Z error: any use of this value will cause an error
2019-07-24T19:43:54.4854898Z    |
2019-07-24T19:43:54.4854898Z    |
2019-07-24T19:43:54.4854980Z LL | const Z3: i32 = unsafe { *(44 as *const i32) }; //~ ERROR any use of this value will cause
2019-07-24T19:43:54.4855323Z    | -------------------------^^^^^^^^^^^^^^^^^^^---
2019-07-24T19:43:54.4855512Z    |                          |
2019-07-24T19:43:54.4855614Z    |                          a memory access tried to interpret some bytes as a pointer
2019-07-24T19:43:54.4855714Z error: aborting due to 5 previous errors
2019-07-24T19:43:54.4855743Z 
2019-07-24T19:43:54.4855768Z 
2019-07-24T19:43:54.4856045Z ------------------------------------------
2019-07-24T19:43:54.4856045Z ------------------------------------------
2019-07-24T19:43:54.4856077Z 
2019-07-24T19:43:54.4856103Z 
2019-07-24T19:43:54.4856326Z ---- [ui] ui/consts/const-eval/issue-52442.rs stdout ----
2019-07-24T19:43:54.4856394Z diff of stderr:
2019-07-24T19:43:54.4856423Z 
2019-07-24T19:43:54.4857028Z 7    = note: for more information, see ***/issues/51910
2019-07-24T19:43:54.4857275Z 8    = help: add `#![feature(const_raw_ptr_to_usize_cast)]` to the crate attributes to enable
2019-07-24T19:43:54.4857779Z - error[E0080]: it is undefined behavior to use this value
2019-07-24T19:43:54.4858303Z -   --> $DIR/issue-52442.rs:2:11
2019-07-24T19:43:54.4858303Z -   --> $DIR/issue-52442.rs:2:11
2019-07-24T19:43:54.4858480Z + error[E0080]: evaluation of constant value failed
2019-07-24T19:43:54.4858759Z +   --> $DIR/issue-52442.rs:2:13
2019-07-24T19:43:54.4858804Z 12    |
2019-07-24T19:43:54.4858970Z 13 LL |     [();  { &loop { break } as *const _ as usize } ];
2019-07-24T19:43:54.4859350Z -    |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2019-07-24T19:43:54.4859684Z -    |
2019-07-24T19:43:54.4860227Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-24T19:43:54.4860428Z +    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a raw memory access tried to access part of a pointer value as raw bytes
2019-07-24T19:43:54.4860598Z 18 error: aborting due to 2 previous errors
2019-07-24T19:43:54.4860759Z 19 
2019-07-24T19:43:54.4860903Z 
2019-07-24T19:43:54.4860981Z 
2019-07-24T19:43:54.4860981Z 
2019-07-24T19:43:54.4861027Z The actual stderr differed from the expected stderr.
2019-07-24T19:43:54.4861397Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-52442/issue-52442.stderr
2019-07-24T19:43:54.4861807Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T19:43:54.4862208Z To only update this specific test, also pass `--test-args consts/const-eval/issue-52442.rs`
2019-07-24T19:43:54.4862452Z error: 1 errors occurred comparing output.
2019-07-24T19:43:54.4862513Z status: exit code: 1
2019-07-24T19:43:54.4862513Z status: exit code: 1
2019-07-24T19:43:54.4865607Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/issue-52442.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-52442" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-52442/auxiliary" "-A" "unused"
2019-07-24T19:43:54.4866943Z ------------------------------------------
2019-07-24T19:43:54.4867129Z 
2019-07-24T19:43:54.4867599Z ------------------------------------------
2019-07-24T19:43:54.4867665Z stderr:
2019-07-24T19:43:54.4867665Z stderr:
2019-07-24T19:43:54.4867865Z ------------------------------------------
2019-07-24T19:43:54.4867914Z error[E0658]: casting pointers to integers in constants is unstable
2019-07-24T19:43:54.4868351Z   --> /checkout/src/test/ui/consts/const-eval/issue-52442.rs:2:13
2019-07-24T19:43:54.4868706Z    |
2019-07-24T19:43:54.4868778Z LL |     [();  { &loop { break } as *const _ as usize } ];
2019-07-24T19:43:54.4868916Z    |
2019-07-24T19:43:54.4868916Z    |
2019-07-24T19:43:54.4869305Z    = note: for more information, see ***/issues/51910
2019-07-24T19:43:54.4869528Z    = help: add `#![feature(const_raw_ptr_to_usize_cast)]` to the crate attributes to enable
2019-07-24T19:43:54.4869663Z error[E0080]: evaluation of constant value failed
2019-07-24T19:43:54.4869970Z   --> /checkout/src/test/ui/consts/const-eval/issue-52442.rs:2:13
2019-07-24T19:43:54.4870158Z    |
2019-07-24T19:43:54.4870158Z    |
2019-07-24T19:43:54.4870262Z LL |     [();  { &loop { break } as *const _ as usize } ];
2019-07-24T19:43:54.4870320Z    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ a raw memory access tried to access part of a pointer value as raw bytes
2019-07-24T19:43:54.4870422Z error: aborting due to 2 previous errors
2019-07-24T19:43:54.4870462Z 
2019-07-24T19:43:54.4870506Z Some errors have detailed explanations: E0080, E0658.
2019-07-24T19:43:54.4870918Z For more information about an error, try `rustc --explain E0080`.
---
2019-07-24T19:43:54.4872285Z 22 error[E0080]: evaluation of constant value failed
2019-07-24T19:43:54.4872564Z -   --> $DIR/match-test-ptr-null.rs:10:13
2019-07-24T19:43:54.4872953Z +   --> $DIR/match-test-ptr-null.rs:6:15
2019-07-24T19:43:54.4873123Z 24    |
2019-07-24T19:43:54.4873404Z - LL |             0 => 42,
2019-07-24T19:43:54.4873798Z -    |             ^ "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
2019-07-24T19:43:54.4874394Z + LL |         match &1 as *const i32 as usize {
2019-07-24T19:43:54.4874660Z +    |               ^^^^^^^^^^^^^^^^^^^^^^^^^ a raw memory access tried to access part of a pointer value as raw bytes
2019-07-24T19:43:54.4874940Z 28 error: aborting due to 4 previous errors
2019-07-24T19:43:54.4875023Z 29 
2019-07-24T19:43:54.4875070Z 
2019-07-24T19:43:54.4875097Z 
2019-07-24T19:43:54.4875097Z 
2019-07-24T19:43:54.4875183Z The actual stderr differed from the expected stderr.
2019-07-24T19:43:54.4875623Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/match-test-ptr-null.stderr
2019-07-24T19:43:54.4876050Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T19:43:54.4876520Z To only update this specific test, also pass `--test-args consts/const-eval/match-test-ptr-null.rs`
2019-07-24T19:43:54.4876800Z error: 1 errors occurred comparing output.
2019-07-24T19:43:54.4876877Z status: exit code: 1
2019-07-24T19:43:54.4876877Z status: exit code: 1
2019-07-24T19:43:54.4878022Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/auxiliary" "-A" "unused"
2019-07-24T19:43:54.4878532Z ------------------------------------------
2019-07-24T19:43:54.4878589Z 
2019-07-24T19:43:54.4878793Z ------------------------------------------
2019-07-24T19:43:54.4878964Z stderr:
2019-07-24T19:43:54.4878964Z stderr:
2019-07-24T19:43:54.4879245Z ------------------------------------------
2019-07-24T19:43:54.4879407Z error[E0658]: casting pointers to integers in constants is unstable
2019-07-24T19:43:54.4879674Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6:15
2019-07-24T19:43:54.4879904Z    |
2019-07-24T19:43:54.4879980Z LL |         match &1 as *const i32 as usize {
2019-07-24T19:43:54.4880082Z    |
2019-07-24T19:43:54.4880082Z    |
2019-07-24T19:43:54.4880513Z    = note: for more information, see ***/issues/51910
2019-07-24T19:43:54.4880703Z    = help: add `#![feature(const_raw_ptr_to_usize_cast)]` to the crate attributes to enable
2019-07-24T19:43:54.4880853Z error[E0019]: constant contains unimplemented expression type
2019-07-24T19:43:54.4881151Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6:15
2019-07-24T19:43:54.4881199Z    |
2019-07-24T19:43:54.4881199Z    |
2019-07-24T19:43:54.4881369Z LL |         match &1 as *const i32 as usize {
2019-07-24T19:43:54.4881498Z 
2019-07-24T19:43:54.4881567Z error[E0019]: constant contains unimplemented expression type
2019-07-24T19:43:54.4881845Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:10:13
2019-07-24T19:43:54.4881910Z    |
2019-07-24T19:43:54.4881910Z    |
2019-07-24T19:43:54.4882079Z LL |             0 => 42, //~ ERROR constant contains unimplemented expression type
2019-07-24T19:43:54.4882196Z 
2019-07-24T19:43:54.4882264Z error[E0080]: evaluation of constant value failed
2019-07-24T19:43:54.4882531Z   --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6:15
2019-07-24T19:43:54.4882593Z    |
2019-07-24T19:43:54.4882593Z    |
2019-07-24T19:43:54.4882759Z LL |         match &1 as *const i32 as usize {
2019-07-24T19:43:54.4882841Z    |               ^^^^^^^^^^^^^^^^^^^^^^^^^ a raw memory access tried to access part of a pointer value as raw bytes
2019-07-24T19:43:54.4882937Z error: aborting due to 4 previous errors
2019-07-24T19:43:54.4882973Z 
2019-07-24T19:43:54.4883135Z Some errors have detailed explanations: E0019, E0080, E0658.
2019-07-24T19:43:54.4883427Z For more information about an error, try `rustc --explain E0019`.
2019-07-24T19:43:54.4883427Z For more information about an error, try `rustc --explain E0019`.
2019-07-24T19:43:54.4883784Z 
2019-07-24T19:43:54.4884591Z ------------------------------------------
2019-07-24T19:43:54.4884657Z 
2019-07-24T19:43:54.4884856Z 
2019-07-24T19:43:54.4885238Z ---- [ui] ui/issues/issue-52023-array-size-pointer-cast.rs stdout ----
2019-07-24T19:43:54.4885312Z diff of stderr:
2019-07-24T19:43:54.4885474Z 
2019-07-24T19:43:54.4885873Z 7    = note: for more information, see ***/issues/51910
2019-07-24T19:43:54.4885954Z 8    = help: add `#![feature(const_raw_ptr_to_usize_cast)]` to the crate attributes to enable
2019-07-24T19:43:54.4886456Z - error[E0080]: it is undefined behavior to use this value
2019-07-24T19:43:54.4886456Z - error[E0080]: it is undefined behavior to use this value
2019-07-24T19:43:54.4886531Z + error[E0080]: evaluation of constant value failed
2019-07-24T19:43:54.4887124Z 12    |
2019-07-24T19:43:54.4887124Z 12    |
2019-07-24T19:43:54.4887327Z 13 LL |     let _ = [0; (&0 as *const i32) as usize];
2019-07-24T19:43:54.4887356Z 
2019-07-24T19:43:54.4887884Z -    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a pointer, but expected initialized plain (non-pointer) bytes
2019-07-24T19:43:54.4888095Z -    |
2019-07-24T19:43:54.4888625Z -    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rust compiler repository if you believe it should not be considered undefined behavior
2019-07-24T19:43:54.4888849Z +    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ a raw memory access tried to access part of a pointer value as raw bytes
2019-07-24T19:43:54.4888991Z 18 error: aborting due to 2 previous errors
2019-07-24T19:43:54.4889031Z 19 
2019-07-24T19:43:54.4889155Z 
2019-07-24T19:43:54.4889231Z 
2019-07-24T19:43:54.4889231Z 
2019-07-24T19:43:54.4889375Z The actual stderr differed from the expected stderr.
2019-07-24T19:43:54.4890022Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52023-array-size-pointer-cast/issue-52023-array-size-pointer-cast.stderr
2019-07-24T19:43:54.4890266Z To update references, rerun the tests and pass the `--bless` flag
2019-07-24T19:43:54.4890762Z To only update this specific test, also pass `--test-args issues/issue-52023-array-size-pointer-cast.rs`
2019-07-24T19:43:54.4891048Z error: 1 errors occurred comparing output.
2019-07-24T19:43:54.4891093Z status: exit code: 1
2019-07-24T19:43:54.4891093Z status: exit code: 1
2019-07-24T19:43:54.4891904Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52023-array-size-pointer-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52023-array-size-pointer-cast" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52023-array-size-pointer-cast/auxiliary" "-A" "unused"
2019-07-24T19:43:54.4892420Z ------------------------------------------
2019-07-24T19:43:54.4892660Z 
2019-07-24T19:43:54.4892960Z ------------------------------------------
2019-07-24T19:43:54.4893005Z stderr:
2019-07-24T19:43:54.4893005Z stderr:
2019-07-24T19:43:54.4893240Z ------------------------------------------
2019-07-24T19:43:54.4893292Z error[E0658]: casting pointers to integers in constants is unstable
2019-07-24T19:43:54.4893830Z   --> /checkout/src/test/ui/issues/issue-52023-array-size-pointer-cast.rs:2:17
2019-07-24T19:43:54.4894437Z    |
2019-07-24T19:43:54.4894535Z LL |     let _ = [0; (&0 as *const i32) as usize]; //~ ERROR casting pointers to integers in constants
2019-07-24T19:43:54.4894720Z    |
2019-07-24T19:43:54.4894720Z    |
2019-07-24T19:43:54.4895148Z    = note: for more information, see ***/issues/51910
2019-07-24T19:43:54.4895365Z    = help: add `#![feature(const_raw_ptr_to_usize_cast)]` to the crate attributes to enable
2019-07-24T19:43:54.4895614Z error[E0080]: evaluation of constant value failed
2019-07-24T19:43:54.4896126Z   --> /checkout/src/test/ui/issues/issue-52023-array-size-pointer-cast.rs:2:17
2019-07-24T19:43:54.4896321Z    |
2019-07-24T19:43:54.4896321Z    |
2019-07-24T19:43:54.4896425Z LL |     let _ = [0; (&0 as *const i32) as usize]; //~ ERROR casting pointers to integers in constants
2019-07-24T19:43:54.4896490Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^ a raw memory access tried to access part of a pointer value as raw bytes
2019-07-24T19:43:54.4896763Z error: aborting due to 2 previous errors
2019-07-24T19:43:54.4896796Z 
2019-07-24T19:43:54.4896844Z Some errors have detailed explanations: E0080, E0658.
2019-07-24T19:43:54.4897270Z For more information about an error, try `rustc --explain E0080`.
---
2019-07-24T19:43:54.4900503Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-24T19:43:54.4900686Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-24T19:43:54.4900859Z 
2019-07-24T19:43:54.4900884Z 
2019-07-24T19:43:54.4902356Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-24T19:43:54.4902789Z 
2019-07-24T19:43:54.4902835Z 
2019-07-24T19:43:54.4902928Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-24T19:43:54.4902996Z Build completed unsuccessfully in 0:55:39
2019-07-24T19:43:54.4902996Z Build completed unsuccessfully in 0:55:39
2019-07-24T19:43:55.6180648Z ##[error]Bash exited with code '1'.
2019-07-24T19:43:55.6296901Z ##[section]Starting: Checkout
2019-07-24T19:43:55.6299038Z ==============================================================================
2019-07-24T19:43:55.6299088Z Task         : Get sources
2019-07-24T19:43:55.6299148Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
