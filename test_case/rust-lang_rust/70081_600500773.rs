plain
2020-03-18T08:02:09.3390547Z ========================== Starting Command Output ===========================
2020-03-18T08:02:09.3393904Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/74bc7dc4-a4e8-41b5-8d58-6c2d38d2f566.sh
2020-03-18T08:02:09.3394315Z 
2020-03-18T08:02:09.3398620Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T08:02:09.3413704Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-18T08:02:09.3416023Z Task         : Get sources
2020-03-18T08:02:09.3416251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T08:02:09.3416463Z Version      : 1.0.0
2020-03-18T08:02:09.3416602Z Author       : Microsoft
---
2020-03-18T08:02:10.3360350Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T08:02:10.3364948Z ##[command]git config gc.auto 0
2020-03-18T08:02:10.3367867Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T08:02:10.3370544Z ##[command]git config --get-all http.proxy
2020-03-18T08:02:10.3375418Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70081/merge:refs/remotes/pull/70081/merge
---
2020-03-18T08:48:35.4008460Z .................................................................................................... 1700/9800
2020-03-18T08:48:39.1701741Z .................................................................................................... 1800/9800
2020-03-18T08:48:48.9150097Z ............................................................................i....................... 1900/9800
2020-03-18T08:48:54.4011044Z .................................................................................................... 2000/9800
2020-03-18T08:49:00.8984177Z ..................................................................iiiii............................. 2100/9800
2020-03-18T08:49:16.3887506Z .................................................................................................... 2300/9800
2020-03-18T08:49:18.3150137Z .................................................................................................... 2400/9800
2020-03-18T08:49:20.7673690Z .................................................................................................... 2500/9800
2020-03-18T08:49:37.6213273Z .................................................................................................... 2600/9800
---
2020-03-18T08:51:48.6370177Z ......................................i...............i............................................. 5000/9800
2020-03-18T08:51:56.0701453Z .................................................................................................... 5100/9800
2020-03-18T08:52:01.2049585Z .................................................................................i.................. 5200/9800
2020-03-18T08:52:05.6967266Z .................................................................................................... 5300/9800
2020-03-18T08:52:14.0343326Z ..............................................................ii.ii........i...i.................... 5400/9800
2020-03-18T08:52:20.6645924Z .i................................................................................................F. 5600/9800
2020-03-18T08:52:28.4913300Z ..................................F................................................................. 5700/9800
2020-03-18T08:52:33.6803837Z .........................................................i.......................................... 5800/9800
2020-03-18T08:52:39.1216868Z .................................................................................................... 5900/9800
2020-03-18T08:52:39.1216868Z .................................................................................................... 5900/9800
2020-03-18T08:52:45.6107279Z .................................................................................................... 6000/9800
2020-03-18T08:52:52.3027485Z ...................................................ii...i..ii...........i........................... 6100/9800
2020-03-18T08:53:07.8942578Z .................................................................................................... 6300/9800
2020-03-18T08:53:10.8325068Z .................................................................................................... 6400/9800
2020-03-18T08:53:10.8325068Z .................................................................................................... 6400/9800
2020-03-18T08:53:13.9922051Z .................................................................................i..ii.............. 6500/9800
2020-03-18T08:53:32.5194222Z .................................................................................................... 6700/9800
2020-03-18T08:53:40.0390455Z ...............................................................................i.................... 6800/9800
2020-03-18T08:53:41.7665094Z .................................................................................................... 6900/9800
2020-03-18T08:53:43.4995138Z .................................................................................................... 7000/9800
---
2020-03-18T08:55:06.1155680Z .................................................................................................... 7800/9800
2020-03-18T08:55:10.4372412Z .................................................................................................... 7900/9800
2020-03-18T08:55:15.2380708Z .................................................................i.................................. 8000/9800
2020-03-18T08:55:23.5830714Z .................................................................................................... 8100/9800
2020-03-18T08:55:27.9638403Z ..............iiiiiiiiii.i.......................................................................... 8200/9800
2020-03-18T08:55:39.1879351Z .................................................................................................... 8400/9800
2020-03-18T08:55:45.3213014Z .................................................................................................... 8500/9800
2020-03-18T08:55:56.2396648Z .................................................................................................... 8600/9800
2020-03-18T08:56:01.4209443Z .................................................................................................... 8700/9800
---
2020-03-18T08:57:32.6689986Z normalized stderr:
2020-03-18T08:57:32.6690314Z warning: unnecessary parentheses around const expression
2020-03-18T08:57:32.6690798Z   --> $DIR/vec-fixed-length.rs:12:31
2020-03-18T08:57:32.6691073Z    |
2020-03-18T08:57:32.6691385Z LL |     assert_eq!(size_of::<[u8; (1 << 32)]>(), (1 << 32));
2020-03-18T08:57:32.6692099Z    |
2020-03-18T08:57:32.6692371Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T08:57:32.6692616Z 
2020-03-18T08:57:32.6692798Z 
2020-03-18T08:57:32.6692798Z 
2020-03-18T08:57:32.6692995Z 
2020-03-18T08:57:32.6693175Z 
2020-03-18T08:57:32.6693438Z The actual stderr differed from the expected stderr.
2020-03-18T08:57:32.6694088Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-fixed-length/vec-fixed-length.stderr
2020-03-18T08:57:32.6694697Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T08:57:32.6695689Z To only update this specific test, also pass `--test-args array-slice-vec/vec-fixed-length.rs`
2020-03-18T08:57:32.6696252Z error: 1 errors occurred comparing output.
2020-03-18T08:57:32.6696496Z status: exit code: 0
2020-03-18T08:57:32.6696496Z status: exit code: 0
2020-03-18T08:57:32.6698100Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/vec-fixed-length.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-fixed-length/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/vec-fixed-length/auxiliary"
2020-03-18T08:57:32.6699515Z ------------------------------------------
2020-03-18T08:57:32.6699730Z 
2020-03-18T08:57:32.6700075Z ------------------------------------------
2020-03-18T08:57:32.6707836Z stderr:
2020-03-18T08:57:32.6707836Z stderr:
2020-03-18T08:57:32.6708443Z ------------------------------------------
2020-03-18T08:57:32.6708773Z warning: unnecessary parentheses around const expression
2020-03-18T08:57:32.6709269Z   --> /checkout/src/test/ui/array-slice-vec/vec-fixed-length.rs:12:31
2020-03-18T08:57:32.6709541Z    |
2020-03-18T08:57:32.6709807Z LL |     assert_eq!(size_of::<[u8; (1 << 32)]>(), (1 << 32));
2020-03-18T08:57:32.6710431Z    |
2020-03-18T08:57:32.6710676Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T08:57:32.6710885Z 
2020-03-18T08:57:32.6711029Z 
---
2020-03-18T08:57:32.6716568Z 8 
2020-03-18T08:57:32.6716952Z + warning: unnecessary parentheses around const expression
2020-03-18T08:57:32.6717765Z +   --> $DIR/issue-62579-no-match.rs:14:11
2020-03-18T08:57:32.6718112Z +    |
2020-03-18T08:57:32.6718431Z + LL |     foo::<{NoMatch}>();
2020-03-18T08:57:32.6729325Z +    |
2020-03-18T08:57:32.6729516Z +    = note: `#[warn(unused_parens)]` on by default
2020-03-18T08:57:32.6729684Z + 
2020-03-18T08:57:32.6729771Z 9 
2020-03-18T08:57:32.6729771Z 9 
2020-03-18T08:57:32.6729844Z 
2020-03-18T08:57:32.6729912Z 
2020-03-18T08:57:32.6730081Z The actual stderr differed from the expected stderr.
2020-03-18T08:57:32.6730782Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62579-no-match/issue-62579-no-match.stderr
2020-03-18T08:57:32.6731288Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T08:57:32.6731742Z To only update this specific test, also pass `--test-args const-generics/issues/issue-62579-no-match.rs`
2020-03-18T08:57:32.6732074Z error: 1 errors occurred comparing output.
2020-03-18T08:57:32.6732257Z status: exit code: 0
2020-03-18T08:57:32.6732257Z status: exit code: 0
2020-03-18T08:57:32.6733647Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-62579-no-match.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62579-no-match/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-62579-no-match/auxiliary"
2020-03-18T08:57:32.6734789Z ------------------------------------------
2020-03-18T08:57:32.6734925Z 
2020-03-18T08:57:32.6735177Z ------------------------------------------
2020-03-18T08:57:32.6735321Z stderr:
---
2020-03-18T08:57:32.6737319Z 
2020-03-18T08:57:32.6737476Z warning: unnecessary parentheses around const expression
2020-03-18T08:57:32.6737970Z   --> /checkout/src/test/ui/const-generics/issues/issue-62579-no-match.rs:14:11
2020-03-18T08:57:32.6738161Z    |
2020-03-18T08:57:32.6738288Z LL |     foo::<{NoMatch}>();
2020-03-18T08:57:32.6738665Z    |
2020-03-18T08:57:32.6738817Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T08:57:32.6738949Z 
2020-03-18T08:57:32.6739031Z 
2020-03-18T08:57:32.6739031Z 
2020-03-18T08:57:32.6739283Z ------------------------------------------
2020-03-18T08:57:32.6739403Z 
2020-03-18T08:57:32.6739472Z 
2020-03-18T08:57:32.6739760Z ---- [ui] ui/const-generics/unused_parens.rs stdout ----
2020-03-18T08:57:32.6739896Z 
2020-03-18T08:57:32.6740407Z error: test compilation failed although it shouldn't!
2020-03-18T08:57:32.6740590Z status: exit code: 1
2020-03-18T08:57:32.6741983Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/unused_parens.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/unused_parens" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/unused_parens/auxiliary"
2020-03-18T08:57:32.6743108Z ------------------------------------------
2020-03-18T08:57:32.6743229Z 
2020-03-18T08:57:32.6743479Z ------------------------------------------
2020-03-18T08:57:32.6743634Z stderr:
2020-03-18T08:57:32.6743634Z stderr:
2020-03-18T08:57:32.6743893Z ------------------------------------------
2020-03-18T08:57:32.6744132Z error: expected one of `!`, `(`, `+`, `,`, `::`, `<`, or `>`, found `=`
2020-03-18T08:57:32.6744728Z    |
2020-03-18T08:57:32.6744728Z    |
2020-03-18T08:57:32.6744868Z LL | struct A<const N: usize = { 7 }> {
2020-03-18T08:57:32.6745096Z    |                         ^ expected one of 7 possible tokens
2020-03-18T08:57:32.6745369Z error: aborting due to previous error
2020-03-18T08:57:32.6745485Z 
2020-03-18T08:57:32.6745567Z 
2020-03-18T08:57:32.6745815Z ------------------------------------------
2020-03-18T08:57:32.6745815Z ------------------------------------------
2020-03-18T08:57:32.6746056Z 
2020-03-18T08:57:32.6746125Z 
2020-03-18T08:57:32.6746408Z ---- [ui] ui/issues/issue-23898.rs stdout ----
2020-03-18T08:57:32.6746570Z normalized stderr:
2020-03-18T08:57:32.6746741Z warning: unnecessary parentheses around const expression
2020-03-18T08:57:32.6747069Z   --> $DIR/issue-23898.rs:9:22
2020-03-18T08:57:32.6747197Z    |
2020-03-18T08:57:32.6747370Z LL |     [State::ST_NULL; (State::ST_WHITESPACE as usize)];
2020-03-18T08:57:32.6747890Z    |
2020-03-18T08:57:32.6748041Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T08:57:32.6748172Z 
2020-03-18T08:57:32.6748255Z 
2020-03-18T08:57:32.6748255Z 
2020-03-18T08:57:32.6748323Z 
2020-03-18T08:57:32.6748390Z 
2020-03-18T08:57:32.6748534Z The actual stderr differed from the expected stderr.
2020-03-18T08:57:32.6748999Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23898/issue-23898.stderr
2020-03-18T08:57:32.6749433Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T08:57:32.6749834Z To only update this specific test, also pass `--test-args issues/issue-23898.rs`
2020-03-18T08:57:32.6750149Z error: 1 errors occurred comparing output.
2020-03-18T08:57:32.6750315Z status: exit code: 0
2020-03-18T08:57:32.6750315Z status: exit code: 0
2020-03-18T08:57:32.6751634Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23898.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23898/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23898/auxiliary"
2020-03-18T08:57:32.6752731Z ------------------------------------------
2020-03-18T08:57:32.6752855Z 
2020-03-18T08:57:32.6753105Z ------------------------------------------
2020-03-18T08:57:32.6753244Z stderr:
2020-03-18T08:57:32.6753244Z stderr:
2020-03-18T08:57:32.6753515Z ------------------------------------------
2020-03-18T08:57:32.6753717Z warning: unnecessary parentheses around const expression
2020-03-18T08:57:32.6754068Z   --> /checkout/src/test/ui/issues/issue-23898.rs:9:22
2020-03-18T08:57:32.6754242Z    |
2020-03-18T08:57:32.6754417Z LL |     [State::ST_NULL; (State::ST_WHITESPACE as usize)];
2020-03-18T08:57:32.6754932Z    |
2020-03-18T08:57:32.6755084Z    = note: `#[warn(unused_parens)]` on by default
2020-03-18T08:57:32.6755214Z 
2020-03-18T08:57:32.6755297Z 
---
2020-03-18T08:57:32.6758196Z - error: unnecessary parentheses around `return` value
2020-03-18T08:57:32.6758411Z + error: unnecessary parentheses around return value
2020-03-18T08:57:32.6758931Z 14   --> $DIR/lint-unnecessary-parens.rs:13:12
2020-03-18T08:57:32.6759113Z 15    |
2020-03-18T08:57:32.6759259Z 16 LL |     return (X { y });
2020-03-18T08:57:32.6759479Z 
2020-03-18T08:57:32.6759645Z The actual stderr differed from the expected stderr.
2020-03-18T08:57:32.6760210Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unnecessary-parens/lint-unnecessary-parens.stderr
2020-03-18T08:57:32.6760210Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unnecessary-parens/lint-unnecessary-parens.stderr
2020-03-18T08:57:32.6760761Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T08:57:32.6761248Z To only update this specific test, also pass `--test-args lint/lint-unnecessary-parens.rs`
2020-03-18T08:57:32.6761632Z error: 1 errors occurred comparing output.
2020-03-18T08:57:32.6761823Z status: exit code: 1
2020-03-18T08:57:32.6761823Z status: exit code: 1
2020-03-18T08:57:32.6763404Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unnecessary-parens.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unnecessary-parens" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unnecessary-parens/auxiliary"
2020-03-18T08:57:32.6765102Z ------------------------------------------
2020-03-18T08:57:32.6765302Z 
2020-03-18T08:57:32.6765789Z ------------------------------------------
2020-03-18T08:57:32.6766002Z stderr:
---
2020-03-18T08:57:32.6770952Z 
2020-03-18T08:57:32.6771279Z error: unnecessary parentheses around return value
2020-03-18T08:57:32.6771981Z   --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:13:12
2020-03-18T08:57:32.6772483Z    |
2020-03-18T08:57:32.6772780Z LL |     return (X { y }); //~ ERROR unnecessary parentheses around `return` value
2020-03-18T08:57:32.6773496Z 
2020-03-18T08:57:32.6773690Z error: unnecessary parentheses around type
2020-03-18T08:57:32.6774384Z   --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:16:42
2020-03-18T08:57:32.6774768Z    |
2020-03-18T08:57:32.6774768Z    |
2020-03-18T08:57:32.6775436Z LL | fn unused_parens_around_return_type() -> (u32) { //~ ERROR unnecessary parentheses around type
2020-03-18T08:57:32.6776191Z 
2020-03-18T08:57:32.6776407Z error: unnecessary parentheses around block return value
2020-03-18T08:57:32.6776991Z   --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:22:9
2020-03-18T08:57:32.6777240Z    |
---
2020-03-18T08:57:32.6784931Z 
2020-03-18T08:57:32.6785114Z error: unnecessary parentheses around function argument
2020-03-18T08:57:32.6785560Z   --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:46:9
2020-03-18T08:57:32.6785761Z    |
2020-03-18T08:57:32.6785995Z LL |     bar((true)); //~ ERROR unnecessary parentheses around function argument
2020-03-18T08:57:32.6786444Z 
2020-03-18T08:57:32.6786614Z error: unnecessary parentheses around `if` condition
2020-03-18T08:57:32.6787164Z   --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:48:8
2020-03-18T08:57:32.6787370Z    |
---
2020-03-18T08:57:32.6791284Z    |
2020-03-18T08:57:32.6791506Z LL |     while (true) {} //~ ERROR unnecessary parentheses around `while` condition
2020-03-18T08:57:32.6791787Z    |     ^^^^^^^^^^^^ help: use `loop`
2020-03-18T08:57:32.6792078Z    |
2020-03-18T08:57:32.6792251Z    = note: `#[warn(while_true)]` on by default
2020-03-18T08:57:32.6792601Z error: unnecessary parentheses around `match` head expression
2020-03-18T08:57:32.6793191Z   --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:51:11
2020-03-18T08:57:32.6793400Z    |
2020-03-18T08:57:32.6793400Z    |
2020-03-18T08:57:32.6793643Z LL |     match (true) { //~ ERROR unnecessary parentheses around `match` head expression
2020-03-18T08:57:32.6794596Z 
2020-03-18T08:57:32.6794823Z error: unnecessary parentheses around `let` head expression
2020-03-18T08:57:32.6795433Z   --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:54:16
2020-03-18T08:57:32.6795686Z    |
2020-03-18T08:57:32.6795686Z    |
2020-03-18T08:57:32.6795967Z LL |     if let 1 = (1) {} //~ ERROR unnecessary parentheses around `let` head expression
2020-03-18T08:57:32.6796589Z 
2020-03-18T08:57:32.6796812Z error: unnecessary parentheses around `let` head expression
2020-03-18T08:57:32.6797374Z   --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:55:19
2020-03-18T08:57:32.6797627Z    |
2020-03-18T08:57:32.6797627Z    |
2020-03-18T08:57:32.6797913Z LL |     while let 1 = (2) {} //~ ERROR unnecessary parentheses around `let` head expression
2020-03-18T08:57:32.6798533Z 
2020-03-18T08:57:32.6798743Z error: unnecessary parentheses around method argument
2020-03-18T08:57:32.6799276Z   --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:69:24
2020-03-18T08:57:32.6799543Z    |
2020-03-18T08:57:32.6799543Z    |
2020-03-18T08:57:32.6799839Z LL |     X { y: false }.foo((true)); //~ ERROR unnecessary parentheses around method argument
2020-03-18T08:57:32.6800504Z 
2020-03-18T08:57:32.6800719Z error: unnecessary parentheses around assigned value
2020-03-18T08:57:32.6801259Z   --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:71:18
2020-03-18T08:57:32.6801526Z    |
---
2020-03-18T08:57:32.6804365Z 
2020-03-18T08:57:32.6805337Z error: unnecessary parentheses around assigned value
2020-03-18T08:57:32.6806092Z   --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:73:11
2020-03-18T08:57:32.6806453Z    |
2020-03-18T08:57:32.6806714Z LL |     _a += (1); //~ ERROR unnecessary parentheses around assigned value
2020-03-18T08:57:32.6807326Z 
2020-03-18T08:57:32.6807605Z error: aborting due to 17 previous errors
2020-03-18T08:57:32.6808188Z 
2020-03-18T08:57:32.6808291Z 
2020-03-18T08:57:32.6808291Z 
2020-03-18T08:57:32.6809119Z ------------------------------------------
2020-03-18T08:57:32.6809299Z 
2020-03-18T08:57:32.6809396Z 
2020-03-18T08:57:32.6809810Z ---- [ui] ui/lint/unused_parens_brace.rs stdout ----
2020-03-18T08:57:32.6810048Z diff of stderr:
2020-03-18T08:57:32.6810172Z 
2020-03-18T08:57:32.6810345Z 22 LL |     if let 7 = { 7 } {
2020-03-18T08:57:32.6811138Z 24 
2020-03-18T08:57:32.6811365Z + warning: unnecessary parentheses around const expression
2020-03-18T08:57:32.6811885Z +   --> $DIR/unused_parens_brace.rs:26:17
2020-03-18T08:57:32.6812109Z +    |
2020-03-18T08:57:32.6812109Z +    |
2020-03-18T08:57:32.6812301Z + LL |     let _: [u8; { 7 }];
2020-03-18T08:57:32.6812866Z + 
2020-03-18T08:57:32.6812992Z 25 
2020-03-18T08:57:32.6813097Z 
2020-03-18T08:57:32.6813212Z 
2020-03-18T08:57:32.6813212Z 
2020-03-18T08:57:32.6813421Z The actual stderr differed from the expected stderr.
2020-03-18T08:57:32.6814109Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_brace/unused_parens_brace.stderr
2020-03-18T08:57:32.6814773Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T08:57:32.6815367Z To only update this specific test, also pass `--test-args lint/unused_parens_brace.rs`
2020-03-18T08:57:32.6815961Z error: 1 errors occurred comparing output.
2020-03-18T08:57:32.6816199Z status: exit code: 0
2020-03-18T08:57:32.6816199Z status: exit code: 0
2020-03-18T08:57:32.6818249Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused_parens_brace.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_brace" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_brace/auxiliary"
2020-03-18T08:57:32.6819860Z ------------------------------------------
2020-03-18T08:57:32.6820056Z 
2020-03-18T08:57:32.6820643Z ------------------------------------------
2020-03-18T08:57:32.6820846Z stderr:
---
2020-03-18T08:57:32.6826084Z 
2020-03-18T08:57:32.6826310Z warning: unnecessary parentheses around `let` head expression
2020-03-18T08:57:32.6826972Z   --> /checkout/src/test/ui/lint/unused_parens_brace.rs:22:16
2020-03-18T08:57:32.6827222Z    |
2020-03-18T08:57:32.6827450Z LL |     if let 7 = { 7 } {
2020-03-18T08:57:32.6828199Z 
2020-03-18T08:57:32.6828399Z warning: unnecessary parentheses around const expression
2020-03-18T08:57:32.6828930Z   --> /checkout/src/test/ui/lint/unused_parens_brace.rs:26:17
2020-03-18T08:57:32.6829154Z    |
2020-03-18T08:57:32.6829154Z    |
2020-03-18T08:57:32.6829325Z LL |     let _: [u8; { 7 }];
2020-03-18T08:57:32.6829804Z 
2020-03-18T08:57:32.6829894Z 
2020-03-18T08:57:32.6830227Z ------------------------------------------
2020-03-18T08:57:32.6830387Z 
---
2020-03-18T08:57:32.6831537Z - warning: unnecessary parentheses around `return` value
2020-03-18T08:57:32.6831832Z + warning: unnecessary parentheses around return value
2020-03-18T08:57:32.6832494Z 2   --> $DIR/path-lookahead.rs:8:10
2020-03-18T08:57:32.6832847Z 3    |
2020-03-18T08:57:32.6833084Z 4 LL |   return (<T as ToString>::to_string(&arg));
2020-03-18T08:57:32.6833386Z 
2020-03-18T08:57:32.6833607Z The actual stderr differed from the expected stderr.
2020-03-18T08:57:32.6834387Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/path-lookahead/path-lookahead.stderr
2020-03-18T08:57:32.6834387Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/path-lookahead/path-lookahead.stderr
2020-03-18T08:57:32.6835548Z To update references, rerun the tests and pass the `--bless` flag
2020-03-18T08:57:32.6836264Z To only update this specific test, also pass `--test-args path-lookahead.rs`
2020-03-18T08:57:32.6836663Z error: 1 errors occurred comparing output.
2020-03-18T08:57:32.6836899Z status: exit code: 0
2020-03-18T08:57:32.6836899Z status: exit code: 0
2020-03-18T08:57:32.6838541Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/path-lookahead.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/path-lookahead/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/path-lookahead/auxiliary"
2020-03-18T08:57:32.6840063Z ------------------------------------------
2020-03-18T08:57:32.6840224Z 
2020-03-18T08:57:32.6840561Z ------------------------------------------
2020-03-18T08:57:32.6840764Z stderr:
2020-03-18T08:57:32.6840764Z stderr:
2020-03-18T08:57:32.6841105Z ------------------------------------------
2020-03-18T08:57:32.6841370Z warning: unnecessary parentheses around return value
2020-03-18T08:57:32.6841841Z   --> /checkout/src/test/ui/path-lookahead.rs:8:10
2020-03-18T08:57:32.6842047Z    |
2020-03-18T08:57:32.6842462Z LL |   return (<T as ToString>::to_string(&arg)); //~WARN unnecessary parentheses around `return` value
2020-03-18T08:57:32.6843188Z    |
2020-03-18T08:57:32.6843474Z note: the lint level is defined here
2020-03-18T08:57:32.6843870Z   --> /checkout/src/test/ui/path-lookahead.rs:3:9
2020-03-18T08:57:32.6844050Z    |
---
2020-03-18T08:57:32.6849625Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-03-18T08:57:32.6850175Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-03-18T08:57:32.6850392Z 
2020-03-18T08:57:32.6850481Z 
2020-03-18T08:57:32.6854080Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-03-18T08:57:32.6856685Z 
2020-03-18T08:57:32.6856793Z 
2020-03-18T08:57:32.6857319Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --exclude src/tools/tidy
2020-03-18T08:57:32.6857661Z Build completed unsuccessfully in 0:51:25
2020-03-18T08:57:32.6857661Z Build completed unsuccessfully in 0:51:25
2020-03-18T08:57:32.6857909Z == clock drift check ==
2020-03-18T08:57:32.6858141Z   local time: Wed Mar 18 08:57:32 UTC 2020
2020-03-18T08:57:32.9752376Z   network time: Wed, 18 Mar 2020 08:57:32 GMT
2020-03-18T08:57:32.9757379Z == end clock drift check ==
2020-03-18T08:57:33.5052839Z 
2020-03-18T08:57:33.5106458Z ##[error]Bash exited with code '1'.
2020-03-18T08:57:33.5126727Z ##[section]Finishing: Run build
2020-03-18T08:57:33.5166260Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-18T08:57:33.5170569Z Task         : Get sources
2020-03-18T08:57:33.5170813Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T08:57:33.5171049Z Version      : 1.0.0
2020-03-18T08:57:33.5171225Z Author       : Microsoft
2020-03-18T08:57:33.5171225Z Author       : Microsoft
2020-03-18T08:57:33.5171480Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T08:57:33.5171764Z ==============================================================================
2020-03-18T08:57:33.7941266Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T08:57:33.7995850Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70081/merge to s
2020-03-18T08:57:33.8080122Z Cleaning up task key
2020-03-18T08:57:33.8081143Z Start cleaning up orphan processes.
2020-03-18T08:57:33.8328554Z Terminate orphan process: pid (4833) (python)
2020-03-18T08:57:33.8352312Z ##[section]Finishing: Finalize Job
