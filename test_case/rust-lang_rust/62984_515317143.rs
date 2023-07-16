plain
2019-07-26T04:37:59.5188285Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T04:38:00.4102207Z ##[command]git config gc.auto 0
2019-07-26T04:38:00.4105439Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T04:38:00.4108009Z ##[command]git config --get-all http.proxy
2019-07-26T04:38:00.4113920Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62984/merge:refs/remotes/pull/62984/merge
---
2019-07-26T04:38:33.1870291Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T04:38:33.1870342Z 
2019-07-26T04:38:33.1870550Z   git checkout -b <new-branch-name>
2019-07-26T04:38:33.1870580Z 
2019-07-26T04:38:33.1870629Z HEAD is now at 8f9b513d9 Merge 919398063da344c697b5aa612366586da98a1071 into 18630677cf6c7ac50e6786c504b35bc09501dbe2
2019-07-26T04:38:33.2009486Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T04:38:33.2012564Z ==============================================================================
2019-07-26T04:38:33.2012616Z Task         : Bash
2019-07-26T04:38:33.2012657Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T05:38:11.6548744Z .................................................................................................... 200/5862
2019-07-26T05:38:15.8998012Z .................................................................................................... 300/5862
2019-07-26T05:38:19.5908033Z .................................................................................................... 400/5862
2019-07-26T05:38:23.3025380Z .................................................................................................... 500/5862
2019-07-26T05:38:27.0879419Z ........................................................................i........................... 600/5862
2019-07-26T05:38:36.0492421Z .................................................................................................... 800/5862
2019-07-26T05:38:41.6652325Z .................................................................................................... 900/5862
2019-07-26T05:38:46.7158692Z .................................................................................................... 1000/5862
2019-07-26T05:38:46.7158692Z .................................................................................................... 1000/5862
2019-07-26T05:38:52.2875023Z i...........i....................................................................................... 1100/5862
2019-07-26T05:38:56.1841130Z ..................F...........iiiii................................................................. 1200/5862
2019-07-26T05:39:02.2072710Z .................................................................................................... 1400/5862
2019-07-26T05:39:05.0010059Z .................................................................................................... 1500/5862
2019-07-26T05:39:08.7496791Z .................................................................................................... 1600/5862
2019-07-26T05:39:11.9324006Z .................................................................................................... 1700/5862
2019-07-26T05:39:11.9324006Z .................................................................................................... 1700/5862
2019-07-26T05:39:15.3482989Z ......................................................................i............................. 1800/5862
2019-07-26T05:39:23.9402631Z .................................................................................................... 2000/5862
2019-07-26T05:39:28.2716775Z .................................................................................................... 2100/5862
2019-07-26T05:39:32.0954542Z .................................................................................................... 2200/5862
2019-07-26T05:39:32.0954542Z .................................................................................................... 2200/5862
2019-07-26T05:39:35.9761238Z ......................................................i............................................. 2300/5862
2019-07-26T05:39:45.8115456Z .................................................................................................... 2500/5862
2019-07-26T05:39:50.0358641Z .................................................................................................... 2600/5862
2019-07-26T05:39:55.1935323Z .................................................................................................... 2700/5862
2019-07-26T05:39:59.0922590Z .................................................................................................... 2800/5862
2019-07-26T05:39:59.0922590Z .................................................................................................... 2800/5862
2019-07-26T05:40:03.5174903Z .................................................................................................... 2900/5862
2019-07-26T05:40:08.9541900Z .................................................................................................... 3000/5862
2019-07-26T05:40:13.3822858Z .................................................................................................... 3100/5862
2019-07-26T05:40:18.7200989Z .................................................................................................... 3200/5862
2019-07-26T05:40:22.2402315Z .................................................................................................... 3300/5862
2019-07-26T05:40:26.0054639Z .................................................................................................... 3400/5862
2019-07-26T05:40:31.1821845Z .................................................................................................... 3500/5862
2019-07-26T05:40:34.9064380Z .....................i.............................................................................. 3600/5862
2019-07-26T05:40:39.1828648Z .................................................................................................ii. 3700/5862
2019-07-26T05:40:42.9450040Z ..i..ii............................................................................................. 3800/5862
2019-07-26T05:40:51.6115192Z .................................................................................................... 4000/5862
2019-07-26T05:40:51.6115192Z .................................................................................................... 4000/5862
2019-07-26T05:40:55.6097483Z ................ii.................................................................................. 4100/5862
2019-07-26T05:40:57.6977822Z .....................................i...............F.............................................. 4200/5862
2019-07-26T05:40:59.7498446Z .................................................................................................... 4300/5862
2019-07-26T05:41:01.9744125Z ......i............................................................................................. 4400/5862
2019-07-26T05:41:23.9364024Z ..............................................................................F..................... 4600/5862
2019-07-26T05:41:27.6835155Z .................................................................................................... 4700/5862
2019-07-26T05:41:31.3708063Z .................................................................................................... 4800/5862
2019-07-26T05:41:36.1214837Z .................................................................................................... 4900/5862
---
2019-07-26T05:42:09.0540052Z .................................................................................................... 5500/5862
2019-07-26T05:42:13.4914585Z .................................................................................................... 5600/5862
2019-07-26T05:42:16.6060176Z .................................................................................................... 5700/5862
2019-07-26T05:42:19.7013612Z .................................................................................................... 5800/5862
2019-07-26T05:42:21.8845044Z ..i...........................................................
2019-07-26T05:42:21.8889334Z 
2019-07-26T05:42:21.8890299Z ---- [ui] ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs stdout ----
2019-07-26T05:42:21.8890360Z diff of stderr:
2019-07-26T05:42:21.8890495Z 
2019-07-26T05:42:21.8890495Z 
2019-07-26T05:42:21.8890536Z 28 LL |     if not  // lack of braces is [sic]
2019-07-26T05:42:21.8890773Z 29    |     -- this `if` statement has a condition, but no block
2019-07-26T05:42:21.8890840Z 30 LL |         println!("Then when?");
2019-07-26T05:42:21.8891046Z -    |                               ^ expected `{`
2019-07-26T05:42:21.8891164Z +    |                               |
2019-07-26T05:42:21.8891208Z +    |                               expected `{`
2019-07-26T05:42:21.8891208Z +    |                               expected `{`
2019-07-26T05:42:21.8891256Z +    |                               help: try placing this code inside a block: `{ ; }`
2019-07-26T05:42:21.8891318Z 32 
2019-07-26T05:42:21.8891362Z 33 error: unexpected `2` after identifier
2019-07-26T05:42:21.8891632Z 
2019-07-26T05:42:21.8891674Z 
2019-07-26T05:42:21.8891716Z The actual stderr differed from the expected stderr.
2019-07-26T05:42:21.8892052Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation/issue-46836-identifier-not-instead-of-negation.stderr
2019-07-26T05:42:21.8892052Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation/issue-46836-identifier-not-instead-of-negation.stderr
2019-07-26T05:42:21.8892293Z To update references, rerun the tests and pass the `--bless` flag
2019-07-26T05:42:21.8892565Z To only update this specific test, also pass `--test-args did_you_mean/issue-46836-identifier-not-instead-of-negation.rs`
2019-07-26T05:42:21.8892665Z error: 1 errors occurred comparing output.
2019-07-26T05:42:21.8892706Z status: exit code: 1
2019-07-26T05:42:21.8892706Z status: exit code: 1
2019-07-26T05:42:21.8893611Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-46836-identifier-not-instead-of-negation/auxiliary" "-A" "unused"
2019-07-26T05:42:21.8894151Z ------------------------------------------
2019-07-26T05:42:21.8894203Z 
2019-07-26T05:42:21.8894404Z ------------------------------------------
2019-07-26T05:42:21.8894447Z stderr:
2019-07-26T05:42:21.8894447Z stderr:
2019-07-26T05:42:21.8894639Z ------------------------------------------
2019-07-26T05:42:21.8894708Z error: unexpected `for_you` after identifier
2019-07-26T05:42:21.8895008Z    |
2019-07-26T05:42:21.8895008Z    |
2019-07-26T05:42:21.8895078Z LL |     if not for_you {
2019-07-26T05:42:21.8895307Z    |        |
2019-07-26T05:42:21.8895307Z    |        |
2019-07-26T05:42:21.8895371Z    |        help: use `!` to perform logical negation
2019-07-26T05:42:21.8895400Z 
2019-07-26T05:42:21.8895441Z error: unexpected `the_worst` after identifier
2019-07-26T05:42:21.8896171Z    |
2019-07-26T05:42:21.8896215Z LL |     while not the_worst {
2019-07-26T05:42:21.8896597Z    |           ----^^^^^^^^^
2019-07-26T05:42:21.8896667Z    |           |
2019-07-26T05:42:21.8896667Z    |           |
2019-07-26T05:42:21.8896716Z    |           help: use `!` to perform logical negation
2019-07-26T05:42:21.8896747Z 
2019-07-26T05:42:21.8896809Z error: unexpected `println` after identifier
2019-07-26T05:42:21.8900309Z    |
2019-07-26T05:42:21.8900309Z    |
2019-07-26T05:42:21.8900367Z LL |     if not  // lack of braces is [sic]
2019-07-26T05:42:21.8900629Z    |        ----- help: use `!` to perform logical negation
2019-07-26T05:42:21.8900678Z LL |         println!("Then when?");
2019-07-26T05:42:21.8900768Z 
2019-07-26T05:42:21.8900768Z 
2019-07-26T05:42:21.8900807Z error: expected `{`, found `;`
2019-07-26T05:42:21.8901125Z    |
2019-07-26T05:42:21.8901125Z    |
2019-07-26T05:42:21.8901177Z LL |     if not  // lack of braces is [sic]
2019-07-26T05:42:21.8901391Z    |     -- this `if` statement has a condition, but no block
2019-07-26T05:42:21.8901439Z LL |         println!("Then when?");
2019-07-26T05:42:21.8901542Z    |                               |
2019-07-26T05:42:21.8901585Z    |                               expected `{`
2019-07-26T05:42:21.8901585Z    |                               expected `{`
2019-07-26T05:42:21.8901653Z    |                               help: try placing this code inside a block: `{ ; }`
2019-07-26T05:42:21.8901692Z 
2019-07-26T05:42:21.8901733Z error: unexpected `2` after identifier
2019-07-26T05:42:21.8902053Z    |
2019-07-26T05:42:21.8902093Z LL |     let resource = not 2;
2019-07-26T05:42:21.8902279Z    |                    ----^
2019-07-26T05:42:21.8902342Z    |                    |
2019-07-26T05:42:21.8902342Z    |                    |
2019-07-26T05:42:21.8902387Z    |                    help: use `!` to perform logical negation
2019-07-26T05:42:21.8902425Z 
2019-07-26T05:42:21.8902486Z error: unexpected `be_smothered_out_before` after identifier
2019-07-26T05:42:21.8902826Z    |
2019-07-26T05:42:21.8902826Z    |
2019-07-26T05:42:21.8902871Z LL |     let young_souls = not be_smothered_out_before;
2019-07-26T05:42:21.8903299Z    |                       |
2019-07-26T05:42:21.8903299Z    |                       |
2019-07-26T05:42:21.8903358Z    |                       help: use `!` to perform logical negation
2019-07-26T05:42:21.8903456Z error: aborting due to 6 previous errors
2019-07-26T05:42:21.8903485Z 
2019-07-26T05:42:21.8903511Z 
2019-07-26T05:42:21.8903786Z ------------------------------------------
2019-07-26T05:42:21.8903786Z ------------------------------------------
2019-07-26T05:42:21.8903818Z 
2019-07-26T05:42:21.8903844Z 
2019-07-26T05:42:21.8904063Z ---- [ui] ui/parser/doc-before-semi.rs stdout ----
2019-07-26T05:42:21.8904139Z diff of stderr:
2019-07-26T05:42:21.8904168Z 
2019-07-26T05:42:21.8904208Z 6    |
2019-07-26T05:42:21.8904258Z 7    = help: doc comments must come before what they document, maybe a comment was intended with `//`?
2019-07-26T05:42:21.8904367Z + warning: unnecessary trailing semicolon
2019-07-26T05:42:21.8904584Z +   --> $DIR/doc-before-semi.rs:5:5
2019-07-26T05:42:21.8904648Z +    |
2019-07-26T05:42:21.8904689Z + LL |     ;
2019-07-26T05:42:21.8904689Z + LL |     ;
2019-07-26T05:42:21.8904742Z +    |     ^ help: remove this semicolon
2019-07-26T05:42:21.8904783Z +    |
2019-07-26T05:42:21.8904847Z +    = note: `#[warn(redundant_semicolon)]` on by default
2019-07-26T05:42:21.8904933Z 9 error: aborting due to previous error
2019-07-26T05:42:21.8904993Z 10 
2019-07-26T05:42:21.8905239Z 11 For more information about this error, try `rustc --explain E0585`.
2019-07-26T05:42:21.8905274Z 
2019-07-26T05:42:21.8905274Z 
2019-07-26T05:42:21.8905300Z 
2019-07-26T05:42:21.8905363Z The actual stderr differed from the expected stderr.
2019-07-26T05:42:21.8906667Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/doc-before-semi/doc-before-semi.stderr
2019-07-26T05:42:21.8906925Z To update references, rerun the tests and pass the `--bless` flag
2019-07-26T05:42:21.8907214Z To only update this specific test, also pass `--test-args parser/doc-before-semi.rs`
2019-07-26T05:42:21.8907299Z error: 1 errors occurred comparing output.
2019-07-26T05:42:21.8907364Z status: exit code: 1
2019-07-26T05:42:21.8907364Z status: exit code: 1
2019-07-26T05:42:21.8908090Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/doc-before-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/doc-before-semi" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/doc-before-semi/auxiliary" "-A" "unused"
2019-07-26T05:42:21.8908426Z ------------------------------------------
2019-07-26T05:42:21.8908462Z 
2019-07-26T05:42:21.8908674Z ------------------------------------------
2019-07-26T05:42:21.8908739Z stderr:
2019-07-26T05:42:21.8908739Z stderr:
2019-07-26T05:42:21.8908950Z ------------------------------------------
2019-07-26T05:42:21.8909194Z error[E0585]: found a documentation comment that doesn't document anything
2019-07-26T05:42:21.8909812Z   --> /checkout/src/test/ui/parser/doc-before-semi.rs:2:5
2019-07-26T05:42:21.8909861Z    |
2019-07-26T05:42:21.8909901Z LL |     /// hi
2019-07-26T05:42:21.8909959Z    |     ^^^^^^
2019-07-26T05:42:21.8909997Z    |
2019-07-26T05:42:21.8910045Z    = help: doc comments must come before what they document, maybe a comment was intended with `//`?
2019-07-26T05:42:21.8910141Z warning: unnecessary trailing semicolon
2019-07-26T05:42:21.8910367Z   --> /checkout/src/test/ui/parser/doc-before-semi.rs:5:5
2019-07-26T05:42:21.8910410Z    |
2019-07-26T05:42:21.8910468Z LL |     ;
2019-07-26T05:42:21.8910468Z LL |     ;
2019-07-26T05:42:21.8910508Z    |     ^ help: remove this semicolon
2019-07-26T05:42:21.8910548Z    |
2019-07-26T05:42:21.8910608Z    = note: `#[warn(redundant_semicolon)]` on by default
2019-07-26T05:42:21.8910679Z error: aborting due to previous error
2019-07-26T05:42:21.8910706Z 
2019-07-26T05:42:21.8911069Z For more information about this error, try `rustc --explain E0585`.
2019-07-26T05:42:21.8911112Z 
---
2019-07-26T05:42:21.8912147Z - error[E0308]: mismatched types
2019-07-26T05:42:21.8912197Z + warning: unnecessary trailing semicolon
2019-07-26T05:42:21.8912247Z 2    |
2019-07-26T05:42:21.8912443Z -    = note: expected type `()`
2019-07-26T05:42:21.8912843Z -               found type `{integer}`
2019-07-26T05:42:21.8912896Z +    = note: `#[warn(redundant_semicolon)]` on by default
2019-07-26T05:42:21.8913002Z 6 error[E0308]: mismatched types
2019-07-26T05:42:21.8913214Z -   --> $DIR/span-preservation.rs:12:20
2019-07-26T05:42:21.8913260Z 8    |
2019-07-26T05:42:21.8913260Z 8    |
2019-07-26T05:42:21.8913481Z - LL |     let x: usize = "hello";;;;;
2019-07-26T05:42:21.8914250Z -    |
2019-07-26T05:42:21.8914294Z 12    = note: expected type `usize`
2019-07-26T05:42:21.8914564Z 13               found type `&'static str`
2019-07-26T05:42:21.8914608Z + 
2019-07-26T05:42:21.8914608Z + 
2019-07-26T05:42:21.8914648Z + error[E0308]: mismatched types
2019-07-26T05:42:21.8914709Z +    |
2019-07-26T05:42:21.8914748Z +    = note: expected type `()`
2019-07-26T05:42:21.8914792Z +               found type `{integer}`
2019-07-26T05:42:21.8914974Z 15 error[E0308]: mismatched types
2019-07-26T05:42:21.8917203Z 16   --> $DIR/span-preservation.rs:18:29
2019-07-26T05:42:21.8917256Z 
2019-07-26T05:42:21.8917307Z 
2019-07-26T05:42:21.8917307Z 
2019-07-26T05:42:21.8917360Z The actual stderr differed from the expected stderr.
2019-07-26T05:42:21.8917677Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/span-preservation/span-preservation.stderr
2019-07-26T05:42:21.8917957Z To update references, rerun the tests and pass the `--bless` flag
2019-07-26T05:42:21.8918231Z To only update this specific test, also pass `--test-args proc-macro/span-preservation.rs`
2019-07-26T05:42:21.8918312Z error: 1 errors occurred comparing output.
2019-07-26T05:42:21.8918376Z status: exit code: 1
2019-07-26T05:42:21.8918376Z status: exit code: 1
2019-07-26T05:42:21.8920401Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/span-preservation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/span-preservation" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/span-preservation/auxiliary" "-A" "unused"
2019-07-26T05:42:21.8920820Z ------------------------------------------
2019-07-26T05:42:21.8920864Z 
2019-07-26T05:42:21.8921096Z ------------------------------------------
2019-07-26T05:42:21.8921142Z stderr:
2019-07-26T05:42:21.8921142Z stderr:
2019-07-26T05:42:21.8921343Z ------------------------------------------
2019-07-26T05:42:21.8921409Z warning: unnecessary trailing semicolon
2019-07-26T05:42:21.8921450Z    |
2019-07-26T05:42:21.8921495Z    = note: `#[warn(redundant_semicolon)]` on by default
2019-07-26T05:42:21.8921587Z error[E0308]: mismatched types
2019-07-26T05:42:21.8921637Z    |
2019-07-26T05:42:21.8921678Z    = note: expected type `usize`
2019-07-26T05:42:21.8921905Z               found type `&'static str`
2019-07-26T05:42:21.8921905Z               found type `&'static str`
2019-07-26T05:42:21.8921937Z 
2019-07-26T05:42:21.8921978Z error[E0308]: mismatched types
2019-07-26T05:42:21.8922018Z    |
2019-07-26T05:42:21.8922078Z    = note: expected type `()`
2019-07-26T05:42:21.8922121Z               found type `{integer}`
2019-07-26T05:42:21.8922189Z error[E0308]: mismatched types
2019-07-26T05:42:21.8923734Z   --> /checkout/src/test/ui/proc-macro/span-preservation.rs:18:29
2019-07-26T05:42:21.8923841Z    |
2019-07-26T05:42:21.8923841Z    |
2019-07-26T05:42:21.8924160Z LL | fn b(x: Option<isize>) -> usize {
2019-07-26T05:42:21.8925294Z    |                           ----- expected `usize` because of return type
2019-07-26T05:42:21.8925353Z LL |     match x {
2019-07-26T05:42:21.8926362Z LL |         Some(x) => { return x }, //~ ERROR mismatched types
2019-07-26T05:42:21.8926459Z    |                             ^ expected usize, found isize
2019-07-26T05:42:21.8926547Z error[E0308]: mismatched types
2019-07-26T05:42:21.8926884Z   --> /checkout/src/test/ui/proc-macro/span-preservation.rs:34:22
2019-07-26T05:42:21.8926935Z    |
2019-07-26T05:42:21.8926935Z    |
2019-07-26T05:42:21.8926983Z LL |     let x = Foo { a: 10isize }; //~ ERROR mismatched types
2019-07-26T05:42:21.8927034Z    |                      ^^^^^^^ expected usize, found isize
2019-07-26T05:42:21.8927091Z 
2019-07-26T05:42:21.8927135Z error[E0560]: struct `c::Foo` has no field named `b`
2019-07-26T05:42:21.8927455Z    |
2019-07-26T05:42:21.8927455Z    |
2019-07-26T05:42:21.8927504Z LL |     let y = Foo { a: 10, b: 10isize }; //~ ERROR has no field named `b`
2019-07-26T05:42:21.8927558Z    |                          ^ `c::Foo` does not have this field
2019-07-26T05:42:21.8927664Z    = note: available fields are: `a`
2019-07-26T05:42:21.8927693Z 
2019-07-26T05:42:21.8927883Z error[E0308]: mismatched types
2019-07-26T05:42:21.8928174Z   --> /checkout/src/test/ui/proc-macro/span-preservation.rs:48:5
2019-07-26T05:42:21.8928174Z   --> /checkout/src/test/ui/proc-macro/span-preservation.rs:48:5
2019-07-26T05:42:21.8928223Z    |
2019-07-26T05:42:21.8928266Z LL | extern "C" fn baz() {
2019-07-26T05:42:21.8928514Z    |                     - possibly return type missing here?
2019-07-26T05:42:21.8928565Z LL |     0 //~ ERROR mismatched types
2019-07-26T05:42:21.8928611Z    |     ^ expected (), found integer
2019-07-26T05:42:21.8928712Z    = note: expected type `()`
2019-07-26T05:42:21.8928712Z    = note: expected type `()`
2019-07-26T05:42:21.8928765Z               found type `{integer}`
2019-07-26T05:42:21.8928856Z error: aborting due to 6 previous errors
2019-07-26T05:42:21.8928886Z 
2019-07-26T05:42:21.8928931Z Some errors have detailed explanations: E0308, E0560.
2019-07-26T05:42:21.8929173Z For more information about an error, try `rustc --explain E0308`.
---
2019-07-26T05:42:21.8930702Z test result: FAILED. 5838 passed; 3 failed; 21 ignored; 0 measured; 0 filtered out
2019-07-26T05:42:21.8930736Z 
2019-07-26T05:42:21.8930849Z 
2019-07-26T05:42:21.8930878Z 
2019-07-26T05:42:21.8932398Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-26T05:42:21.8932680Z 
2019-07-26T05:42:21.8932760Z 
2019-07-26T05:42:21.8933045Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-26T05:42:21.8957233Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-26T05:42:21.8957233Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-26T05:42:21.8957311Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-26T05:42:21.8957362Z Build completed unsuccessfully in 0:57:23
2019-07-26T05:42:22.9887162Z ##[error]Bash exited with code '1'.
2019-07-26T05:42:22.9924597Z ##[section]Starting: Checkout
2019-07-26T05:42:22.9927187Z ==============================================================================
2019-07-26T05:42:22.9927262Z Task         : Get sources
2019-07-26T05:42:22.9927310Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
