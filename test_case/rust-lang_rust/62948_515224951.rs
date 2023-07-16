plain
2019-07-25T20:38:43.0861835Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T20:38:43.1061229Z ##[command]git config gc.auto 0
2019-07-25T20:38:43.8786002Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T20:38:43.8793926Z ##[command]git config --get-all http.proxy
2019-07-25T20:38:43.8799049Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62948/merge:refs/remotes/pull/62948/merge
---
2019-07-25T20:39:17.2847546Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T20:39:17.2847750Z 
2019-07-25T20:39:17.2848117Z   git checkout -b <new-branch-name>
2019-07-25T20:39:17.2848319Z 
2019-07-25T20:39:17.2848499Z HEAD is now at fd0340313 Merge c4ccb429fd6b2c73b5022393fa7dfa912edb5291 into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T20:39:17.2971124Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T20:39:17.2974284Z ==============================================================================
2019-07-25T20:39:17.2974345Z Task         : Bash
2019-07-25T20:39:17.2974393Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T21:35:32.9033399Z .................................................................................................... 200/5856
2019-07-25T21:35:36.8686903Z .................................................................................................... 300/5856
2019-07-25T21:35:40.3955422Z .................................................................................................... 400/5856
2019-07-25T21:35:43.9197816Z .................................................................................................... 500/5856
2019-07-25T21:35:47.5007602Z ........................................................................i........................... 600/5856
2019-07-25T21:35:56.2697749Z .................................................................................................... 800/5856
2019-07-25T21:36:01.6416899Z .................................................................................................... 900/5856
2019-07-25T21:36:06.4763677Z ...................................................................................................i 1000/5856
2019-07-25T21:36:06.4763677Z ...................................................................................................i 1000/5856
2019-07-25T21:36:11.8252909Z ...........i........................................................................................ 1100/5856
2019-07-25T21:36:15.5625472Z .............................iiiii.................................................................. 1200/5856
2019-07-25T21:36:21.3158271Z .................................................................................................... 1400/5856
2019-07-25T21:36:23.9883443Z .................................................................................................... 1500/5856
2019-07-25T21:36:27.6728245Z .................................................................................................... 1600/5856
2019-07-25T21:36:30.2992037Z .................................................................................................... 1700/5856
2019-07-25T21:36:30.2992037Z .................................................................................................... 1700/5856
2019-07-25T21:36:33.5892028Z .....................................................................i.............................. 1800/5856
2019-07-25T21:36:41.8066087Z .................................................................................................... 2000/5856
2019-07-25T21:36:45.8160467Z .................................................................................................... 2100/5856
2019-07-25T21:36:49.3698661Z .................................................................................................... 2200/5856
2019-07-25T21:36:49.3698661Z .................................................................................................... 2200/5856
2019-07-25T21:36:53.0383274Z .....................................................i.............................................. 2300/5856
2019-07-25T21:37:02.3629243Z .................................................................................................... 2500/5856
2019-07-25T21:37:06.3293573Z .................................................................................................... 2600/5856
2019-07-25T21:37:11.2520317Z .................................................................................................... 2700/5856
2019-07-25T21:37:15.0511269Z .................................................................................................... 2800/5856
2019-07-25T21:37:15.0511269Z .................................................................................................... 2800/5856
2019-07-25T21:37:19.2707633Z .................................................................................................... 2900/5856
2019-07-25T21:37:24.2162293Z .................................................................................................... 3000/5856
2019-07-25T21:37:28.4562934Z .................................................................................................... 3100/5856
2019-07-25T21:37:33.4466239Z .................................................................................................... 3200/5856
2019-07-25T21:37:36.8569295Z .................................................................................................... 3300/5856
2019-07-25T21:37:40.4774083Z .................................................................................................... 3400/5856
2019-07-25T21:37:45.3011398Z .................................................................................................... 3500/5856
2019-07-25T21:37:48.9258387Z ....................i............................................................................... 3600/5856
2019-07-25T21:37:52.8546219Z ..............................................................................................ii...i 3700/5856
2019-07-25T21:37:56.4912962Z ...ii............................................................................................... 3800/5856
2019-07-25T21:38:04.8879179Z .................................................................................................... 4000/5856
2019-07-25T21:38:04.8879179Z .................................................................................................... 4000/5856
2019-07-25T21:38:08.5100706Z .............ii..................................................................................... 4100/5856
2019-07-25T21:38:10.4333254Z ..............F...................i................................................................. 4200/5856
2019-07-25T21:38:12.5799755Z .............................................................................F...................... 4300/5856
2019-07-25T21:38:14.7739667Z .i.......................................F...................................F........F............. 4400/5856
2019-07-25T21:38:35.3993084Z .................................................................................................... 4600/5856
2019-07-25T21:38:38.9078377Z .................................................................................................... 4700/5856
2019-07-25T21:38:42.4477798Z .................................................................................................... 4800/5856
2019-07-25T21:38:46.9887906Z .................................................................................................... 4900/5856
---
2019-07-25T21:39:31.0610625Z failures:
2019-07-25T21:39:31.0657446Z 
2019-07-25T21:39:31.0658000Z ---- [ui] ui/parser/bad-char-literals.rs stdout ----
2019-07-25T21:39:31.0658070Z 
2019-07-25T21:39:31.0658915Z error: /checkout/src/test/ui/parser/bad-char-literals.rs:16: unexpected error: '16:6: 16:6: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0658987Z 
2019-07-25T21:39:31.0659034Z error: 1 unexpected errors found, 0 expected errors not found
2019-07-25T21:39:31.0659075Z status: exit code: 1
2019-07-25T21:39:31.0659975Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/bad-char-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/bad-char-literals" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/bad-char-literals/auxiliary" "-A" "unused"
2019-07-25T21:39:31.0660159Z unexpected errors (from JSON output): [
2019-07-25T21:39:31.0660222Z     Error {
2019-07-25T21:39:31.0660266Z         line_num: 16,
2019-07-25T21:39:31.0660305Z         kind: Some(
2019-07-25T21:39:31.0660401Z         ),
2019-07-25T21:39:31.0660401Z         ),
2019-07-25T21:39:31.0660443Z         msg: "16:6: 16:6: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0660546Z ]
2019-07-25T21:39:31.0660571Z 
2019-07-25T21:39:31.0660909Z thread '[ui] ui/parser/bad-char-literals.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1505:13
2019-07-25T21:39:31.0660993Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-25T21:39:31.0660993Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-25T21:39:31.0661026Z 
2019-07-25T21:39:31.0662953Z ---- [ui] ui/parser/lex-bare-cr-string-literal-doc-comment.rs stdout ----
2019-07-25T21:39:31.0663027Z 
2019-07-25T21:39:31.0664347Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:5: unexpected error: '5:32: 5:32: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0664412Z 
2019-07-25T21:39:31.0664727Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:9: unexpected error: '9:38: 9:38: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0665298Z 
2019-07-25T21:39:31.0665735Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:14: unexpected error: '14:36: 14:36: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0665787Z 
2019-07-25T21:39:31.0666197Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:17: unexpected error: '17:42: 17:42: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0666247Z 
2019-07-25T21:39:31.0666608Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:21: unexpected error: '21:18: 21:18: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0666679Z 
2019-07-25T21:39:31.0667031Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:24: unexpected error: '24:19: 24:19: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0667089Z 
2019-07-25T21:39:31.0667467Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:27: unexpected error: '27:19: 27:19: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0667514Z 
2019-07-25T21:39:31.0667875Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:21: unexpected error: '21:18: 21:19: character constant must be escaped: \r'
2019-07-25T21:39:31.0667923Z 
2019-07-25T21:39:31.0668289Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:5: expected error not found: bare CR not allowed in doc-comment
2019-07-25T21:39:31.0668514Z 
2019-07-25T21:39:31.0668808Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:9: expected error not found: bare CR not allowed in block doc-comment
2019-07-25T21:39:31.0668847Z 
2019-07-25T21:39:31.0669155Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:14: expected error not found: bare CR not allowed in doc-comment
2019-07-25T21:39:31.0669204Z 
2019-07-25T21:39:31.0669501Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:17: expected error not found: bare CR not allowed in block doc-comment
2019-07-25T21:39:31.0669560Z 
2019-07-25T21:39:31.0669964Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:21: expected error not found: bare CR not allowed in string
2019-07-25T21:39:31.0670018Z 
2019-07-25T21:39:31.0670341Z error: /checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs:24: expected error not found: bare CR not allowed in raw string
2019-07-25T21:39:31.0670404Z 
2019-07-25T21:39:31.0670448Z error: 8 unexpected errors found, 6 expected errors not found
2019-07-25T21:39:31.0670490Z status: exit code: 1
2019-07-25T21:39:31.0671236Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/lex-bare-cr-string-literal-doc-comment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bare-cr-string-literal-doc-comment" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/lex-bare-cr-string-literal-doc-comment/auxiliary" "-A" "unused"
2019-07-25T21:39:31.0671426Z unexpected errors (from JSON output): [
2019-07-25T21:39:31.0671469Z     Error {
2019-07-25T21:39:31.0671513Z         line_num: 5,
2019-07-25T21:39:31.0671551Z         kind: Some(
2019-07-25T21:39:31.0671646Z         ),
2019-07-25T21:39:31.0671646Z         ),
2019-07-25T21:39:31.0671690Z         msg: "5:32: 5:32: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0671798Z     Error {
2019-07-25T21:39:31.0671836Z         line_num: 9,
2019-07-25T21:39:31.0671893Z         kind: Some(
2019-07-25T21:39:31.0671932Z             Error,
2019-07-25T21:39:31.0671932Z             Error,
2019-07-25T21:39:31.0671969Z         ),
2019-07-25T21:39:31.0672011Z         msg: "9:38: 9:38: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0672107Z     Error {
2019-07-25T21:39:31.0672145Z         line_num: 14,
2019-07-25T21:39:31.0672202Z         kind: Some(
2019-07-25T21:39:31.0672246Z             Error,
2019-07-25T21:39:31.0672246Z             Error,
2019-07-25T21:39:31.0672285Z         ),
2019-07-25T21:39:31.0672345Z         msg: "14:36: 14:36: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0672423Z     Error {
2019-07-25T21:39:31.0672461Z         line_num: 17,
2019-07-25T21:39:31.0672517Z         kind: Some(
2019-07-25T21:39:31.0672554Z             Error,
2019-07-25T21:39:31.0672554Z             Error,
2019-07-25T21:39:31.0672590Z         ),
2019-07-25T21:39:31.0672650Z         msg: "17:42: 17:42: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0672733Z     Error {
2019-07-25T21:39:31.0672788Z         line_num: 21,
2019-07-25T21:39:31.0672826Z         kind: Some(
2019-07-25T21:39:31.0672863Z             Error,
2019-07-25T21:39:31.0672863Z             Error,
2019-07-25T21:39:31.0672899Z         ),
2019-07-25T21:39:31.0672959Z         msg: "21:18: 21:18: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0673033Z     Error {
2019-07-25T21:39:31.0673087Z         line_num: 24,
2019-07-25T21:39:31.0673131Z         kind: Some(
2019-07-25T21:39:31.0673169Z             Error,
2019-07-25T21:39:31.0673169Z             Error,
2019-07-25T21:39:31.0675853Z         ),
2019-07-25T21:39:31.0675947Z         msg: "24:19: 24:19: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0676035Z     Error {
2019-07-25T21:39:31.0677111Z         line_num: 27,
2019-07-25T21:39:31.0677177Z         kind: Some(
2019-07-25T21:39:31.0677222Z             Error,
2019-07-25T21:39:31.0677222Z             Error,
2019-07-25T21:39:31.0677290Z         ),
2019-07-25T21:39:31.0677360Z         msg: "27:19: 27:19: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0677449Z     Error {
2019-07-25T21:39:31.0677514Z         line_num: 21,
2019-07-25T21:39:31.0677560Z         kind: Some(
2019-07-25T21:39:31.0677604Z             Error,
2019-07-25T21:39:31.0677604Z             Error,
2019-07-25T21:39:31.0677665Z         ),
2019-07-25T21:39:31.0677717Z         msg: "21:18: 21:19: character constant must be escaped: \\r",
2019-07-25T21:39:31.0677923Z ]
2019-07-25T21:39:31.0677990Z 
2019-07-25T21:39:31.0678039Z not found errors (from test file): [
2019-07-25T21:39:31.0678085Z     Error {
2019-07-25T21:39:31.0678085Z     Error {
2019-07-25T21:39:31.0678323Z         line_num: 5,
2019-07-25T21:39:31.0678540Z         kind: Some(
2019-07-25T21:39:31.0678579Z             Error,
2019-07-25T21:39:31.0678789Z         ),
2019-07-25T21:39:31.0679190Z         msg: "bare CR not allowed in doc-comment",
2019-07-25T21:39:31.0679278Z     Error {
2019-07-25T21:39:31.0679449Z         line_num: 9,
2019-07-25T21:39:31.0679489Z         kind: Some(
2019-07-25T21:39:31.0679526Z             Error,
2019-07-25T21:39:31.0679526Z             Error,
2019-07-25T21:39:31.0679563Z         ),
2019-07-25T21:39:31.0679850Z         msg: "bare CR not allowed in block doc-comment",
2019-07-25T21:39:31.0679932Z     Error {
2019-07-25T21:39:31.0679994Z         line_num: 14,
2019-07-25T21:39:31.0680033Z         kind: Some(
2019-07-25T21:39:31.0680070Z             Error,
2019-07-25T21:39:31.0680070Z             Error,
2019-07-25T21:39:31.0680125Z         ),
2019-07-25T21:39:31.0680354Z         msg: "bare CR not allowed in doc-comment",
2019-07-25T21:39:31.0680438Z     Error {
2019-07-25T21:39:31.0680498Z         line_num: 17,
2019-07-25T21:39:31.0680535Z         kind: Some(
2019-07-25T21:39:31.0680573Z             Error,
2019-07-25T21:39:31.0680573Z             Error,
2019-07-25T21:39:31.0680628Z         ),
2019-07-25T21:39:31.0680852Z         msg: "bare CR not allowed in block doc-comment",
2019-07-25T21:39:31.0680933Z     Error {
2019-07-25T21:39:31.0681003Z         line_num: 21,
2019-07-25T21:39:31.0681041Z         kind: Some(
2019-07-25T21:39:31.0681078Z             Error,
2019-07-25T21:39:31.0681078Z             Error,
2019-07-25T21:39:31.0681323Z         ),
2019-07-25T21:39:31.0681365Z         msg: "bare CR not allowed in string",
2019-07-25T21:39:31.0681457Z     Error {
2019-07-25T21:39:31.0681495Z         line_num: 24,
2019-07-25T21:39:31.0681531Z         kind: Some(
2019-07-25T21:39:31.0681569Z             Error,
2019-07-25T21:39:31.0681569Z             Error,
2019-07-25T21:39:31.0681630Z         ),
2019-07-25T21:39:31.0681671Z         msg: "bare CR not allowed in raw string",
2019-07-25T21:39:31.0681763Z ]
2019-07-25T21:39:31.0681788Z 
2019-07-25T21:39:31.0682366Z thread '[ui] ui/parser/lex-bare-cr-string-literal-doc-comment.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1505:13
2019-07-25T21:39:31.0682420Z 
2019-07-25T21:39:31.0682420Z 
2019-07-25T21:39:31.0682718Z ---- [ui] ui/parser/raw-byte-string-literals.rs stdout ----
2019-07-25T21:39:31.0682771Z 
2019-07-25T21:39:31.0686236Z error: /checkout/src/test/ui/parser/raw-byte-string-literals.rs:4: unexpected error: '4:9: 4:9: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0686302Z 
2019-07-25T21:39:31.0686689Z error: /checkout/src/test/ui/parser/raw-byte-string-literals.rs:4: expected error not found: bare CR not allowed in raw string
2019-07-25T21:39:31.0686734Z 
2019-07-25T21:39:31.0686787Z error: 1 unexpected errors found, 1 expected errors not found
2019-07-25T21:39:31.0686873Z status: exit code: 1
2019-07-25T21:39:31.0687696Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/raw-byte-string-literals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-literals" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "continue-parse-after-error" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/raw-byte-string-literals/auxiliary" "-A" "unused"
2019-07-25T21:39:31.0687841Z unexpected errors (from JSON output): [
2019-07-25T21:39:31.0687891Z     Error {
2019-07-25T21:39:31.0688110Z         line_num: 4,
2019-07-25T21:39:31.0688180Z         kind: Some(
2019-07-25T21:39:31.0688257Z         ),
2019-07-25T21:39:31.0688257Z         ),
2019-07-25T21:39:31.0688626Z         msg: "4:9: 4:9: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0688734Z ]
2019-07-25T21:39:31.0688759Z 
2019-07-25T21:39:31.0688823Z not found errors (from test file): [
2019-07-25T21:39:31.0688866Z     Error {
2019-07-25T21:39:31.0688866Z     Error {
2019-07-25T21:39:31.0688905Z         line_num: 4,
2019-07-25T21:39:31.0688963Z         kind: Some(
2019-07-25T21:39:31.0689004Z             Error,
2019-07-25T21:39:31.0689042Z         ),
2019-07-25T21:39:31.0689084Z         msg: "bare CR not allowed in raw string",
2019-07-25T21:39:31.0689243Z ]
2019-07-25T21:39:31.0689268Z 
2019-07-25T21:39:31.0689651Z thread '[ui] ui/parser/raw-byte-string-literals.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1505:13
2019-07-25T21:39:31.0689697Z 
2019-07-25T21:39:31.0689697Z 
2019-07-25T21:39:31.0689960Z ---- [ui] ui/parser/several-carriage-returns-in-doc-comment.rs stdout ----
2019-07-25T21:39:31.0689998Z 
2019-07-25T21:39:31.0690358Z error: /checkout/src/test/ui/parser/several-carriage-returns-in-doc-comment.rs:6: unexpected error: '6:12: 6:12: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0690404Z 
2019-07-25T21:39:31.0690731Z error: /checkout/src/test/ui/parser/several-carriage-returns-in-doc-comment.rs:6: unexpected error: '6:32: 6:32: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0690795Z 
2019-07-25T21:39:31.0691283Z error: /checkout/src/test/ui/parser/several-carriage-returns-in-doc-comment.rs:6: unexpected error: '6:52: 6:52: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0691335Z 
2019-07-25T21:39:31.0691640Z error: /checkout/src/test/ui/parser/several-carriage-returns-in-doc-comment.rs:6: expected error not found: bare CR not allowed in doc-comment
2019-07-25T21:39:31.0691703Z 
2019-07-25T21:39:31.0692005Z error: /checkout/src/test/ui/parser/several-carriage-returns-in-doc-comment.rs:6: expected error not found: bare CR not allowed in doc-comment
2019-07-25T21:39:31.0692046Z 
2019-07-25T21:39:31.0692735Z error: /checkout/src/test/ui/parser/several-carriage-returns-in-doc-comment.rs:6: expected error not found: bare CR not allowed in doc-comment
2019-07-25T21:39:31.0692788Z 
2019-07-25T21:39:31.0692835Z error: 3 unexpected errors found, 3 expected errors not found
2019-07-25T21:39:31.0692879Z status: exit code: 1
2019-07-25T21:39:31.0693693Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/several-carriage-returns-in-doc-comment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/several-carriage-returns-in-doc-comment" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/several-carriage-returns-in-doc-comment/auxiliary" "-A" "unused"
2019-07-25T21:39:31.0693829Z unexpected errors (from JSON output): [
2019-07-25T21:39:31.0693885Z     Error {
2019-07-25T21:39:31.0693933Z         line_num: 6,
2019-07-25T21:39:31.0693976Z         kind: Some(
2019-07-25T21:39:31.0694083Z         ),
2019-07-25T21:39:31.0694083Z         ),
2019-07-25T21:39:31.0694132Z         msg: "6:12: 6:12: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0694243Z     Error {
2019-07-25T21:39:31.0694285Z         line_num: 6,
2019-07-25T21:39:31.0694346Z         kind: Some(
2019-07-25T21:39:31.0694389Z             Error,
2019-07-25T21:39:31.0694389Z             Error,
2019-07-25T21:39:31.0694440Z         ),
2019-07-25T21:39:31.0694487Z         msg: "6:32: 6:32: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0694592Z     Error {
2019-07-25T21:39:31.0694635Z         line_num: 6,
2019-07-25T21:39:31.0694700Z         kind: Some(
2019-07-25T21:39:31.0694743Z             Error,
2019-07-25T21:39:31.0694743Z             Error,
2019-07-25T21:39:31.0694784Z         ),
2019-07-25T21:39:31.0694850Z         msg: "6:52: 6:52: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0695719Z ]
2019-07-25T21:39:31.0695746Z 
2019-07-25T21:39:31.0695816Z not found errors (from test file): [
2019-07-25T21:39:31.0695861Z     Error {
2019-07-25T21:39:31.0695861Z     Error {
2019-07-25T21:39:31.0695904Z         line_num: 6,
2019-07-25T21:39:31.0695965Z         kind: Some(
2019-07-25T21:39:31.0696010Z             Error,
2019-07-25T21:39:31.0696052Z         ),
2019-07-25T21:39:31.0696391Z         msg: "bare CR not allowed in doc-comment",
2019-07-25T21:39:31.0696595Z     Error {
2019-07-25T21:39:31.0696640Z         line_num: 6,
2019-07-25T21:39:31.0696704Z         kind: Some(
2019-07-25T21:39:31.0696750Z             Error,
2019-07-25T21:39:31.0696750Z             Error,
2019-07-25T21:39:31.0696794Z         ),
2019-07-25T21:39:31.0697079Z         msg: "bare CR not allowed in doc-comment",
2019-07-25T21:39:31.0697197Z     Error {
2019-07-25T21:39:31.0697242Z         line_num: 6,
2019-07-25T21:39:31.0697304Z         kind: Some(
2019-07-25T21:39:31.0697360Z             Error,
2019-07-25T21:39:31.0697360Z             Error,
2019-07-25T21:39:31.0697406Z         ),
2019-07-25T21:39:31.0697667Z         msg: "bare CR not allowed in doc-comment",
2019-07-25T21:39:31.0697782Z ]
2019-07-25T21:39:31.0697863Z 
2019-07-25T21:39:31.0698238Z thread '[ui] ui/parser/several-carriage-returns-in-doc-comment.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1505:13
2019-07-25T21:39:31.0698289Z 
2019-07-25T21:39:31.0698289Z 
2019-07-25T21:39:31.0699217Z ---- [ui] ui/parser/trailing-carriage-return-in-string.rs stdout ----
2019-07-25T21:39:31.0699273Z 
2019-07-25T21:39:31.0699630Z error: /checkout/src/test/ui/parser/trailing-carriage-return-in-string.rs:10: unexpected error: '10:25: 10:25: bare carriage return (\r) is not allowed'
2019-07-25T21:39:31.0699672Z 
2019-07-25T21:39:31.0699717Z error: 1 unexpected errors found, 0 expected errors not found
2019-07-25T21:39:31.0699782Z status: exit code: 1
2019-07-25T21:39:31.0700923Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/trailing-carriage-return-in-string.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trailing-carriage-return-in-string" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trailing-carriage-return-in-string/auxiliary" "-A" "unused"
2019-07-25T21:39:31.0701057Z unexpected errors (from JSON output): [
2019-07-25T21:39:31.0701103Z     Error {
2019-07-25T21:39:31.0701150Z         line_num: 10,
2019-07-25T21:39:31.0701212Z         kind: Some(
2019-07-25T21:39:31.0701297Z         ),
2019-07-25T21:39:31.0701297Z         ),
2019-07-25T21:39:31.0701364Z         msg: "10:25: 10:25: bare carriage return (\\r) is not allowed",
2019-07-25T21:39:31.0701453Z ]
2019-07-25T21:39:31.0701488Z 
2019-07-25T21:39:31.0701847Z thread '[ui] ui/parser/trailing-carriage-return-in-string.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1505:13
2019-07-25T21:39:31.0701892Z 
---
2019-07-25T21:39:31.0704062Z 
2019-07-25T21:39:31.0704372Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-25T21:39:31.0704408Z 
2019-07-25T21:39:31.0704551Z 
2019-07-25T21:39:31.0706516Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-25T21:39:31.0706923Z 
2019-07-25T21:39:31.0706957Z 
2019-07-25T21:39:31.0707007Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-25T21:39:31.0707070Z Build completed unsuccessfully in 0:53:55
2019-07-25T21:39:31.0707070Z Build completed unsuccessfully in 0:53:55
2019-07-25T21:39:32.2229294Z ##[error]Bash exited with code '1'.
2019-07-25T21:39:32.2263558Z ##[section]Starting: Checkout
2019-07-25T21:39:32.2265046Z ==============================================================================
2019-07-25T21:39:32.2265492Z Task         : Get sources
2019-07-25T21:39:32.2265554Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
