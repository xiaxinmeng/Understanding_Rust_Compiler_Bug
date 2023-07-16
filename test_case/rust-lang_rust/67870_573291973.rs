plain
2020-01-11T06:29:14.9649810Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-11T06:29:14.9741113Z ##[command]git config gc.auto 0
2020-01-11T06:29:14.9798866Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-11T06:29:14.9861158Z ##[command]git config --get-all http.proxy
2020-01-11T06:29:15.0012861Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67870/merge:refs/remotes/pull/67870/merge
---
2020-01-11T07:31:06.6200375Z .......................................i...............i............................................ 4900/9515
2020-01-11T07:31:17.1042379Z .................................................................................................... 5000/9515
2020-01-11T07:31:23.8376838Z ....................................................................................i............... 5100/9515
2020-01-11T07:31:29.8444184Z .................................................................................................... 5200/9515
2020-01-11T07:31:41.0552572Z .......................................................ii.ii...........i............................ 5300/9515
2020-01-11T07:31:51.7362485Z ..............................FF......F............................................................. 5500/9515
2020-01-11T07:32:02.8020162Z ...FFF.............................................................................................. 5600/9515
2020-01-11T07:32:10.4034075Z .......................................i............................................................ 5700/9515
2020-01-11T07:32:17.9265252Z .................................................................................................... 5800/9515
2020-01-11T07:32:17.9265252Z .................................................................................................... 5800/9515
2020-01-11T07:32:30.7592653Z .................................................................................................... 5900/9515
2020-01-11T07:32:40.7061697Z ..............................ii...i..ii...........i................................................ 6000/9515
2020-01-11T07:33:00.9457498Z .................................................................................................... 6200/9515
2020-01-11T07:33:09.9650976Z .................................................................................................... 6300/9515
2020-01-11T07:33:09.9650976Z .................................................................................................... 6300/9515
2020-01-11T07:33:22.0597817Z .........................................................i..ii...................................... 6400/9515
2020-01-11T07:33:51.5094101Z .................................................................................................... 6600/9515
2020-01-11T07:33:53.9239101Z ................................i................................................................... 6700/9515
2020-01-11T07:33:56.5470837Z .................................................................................................... 6800/9515
2020-01-11T07:33:59.5240658Z ................................i................................................................... 6900/9515
---
2020-01-11T07:35:43.9213692Z .................................................................................................... 7500/9515
2020-01-11T07:35:48.8051398Z .................................................................................................... 7600/9515
2020-01-11T07:35:55.5079886Z .................................................................................................... 7700/9515
2020-01-11T07:36:03.5230849Z .................................................................................................... 7800/9515
2020-01-11T07:36:14.0628101Z .................................................................................iiii............... 7900/9515
2020-01-11T07:36:32.7821408Z ...............i......i............................................................................. 8100/9515
2020-01-11T07:36:38.4969629Z .................................................................................................... 8200/9515
2020-01-11T07:36:53.1603046Z .................................................................................................... 8300/9515
2020-01-11T07:37:03.7101778Z .................................................................................................... 8400/9515
---
2020-01-11T07:39:12.7191791Z failures:
2020-01-11T07:39:12.7232147Z 
2020-01-11T07:39:12.7233154Z ---- [ui] ui/deduplicate-diagnostics.rs#duplicate stdout ----
2020-01-11T07:39:12.7233387Z 
2020-01-11T07:39:12.7234663Z error in revision `duplicate`: /checkout/src/test/ui/deduplicate-diagnostics.rs:8: unexpected error: '8:8: 8:17: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7234777Z error in revision `duplicate`: 1 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7234851Z status: exit code: 1
2020-01-11T07:39:12.7234851Z status: exit code: 1
2020-01-11T07:39:12.7235753Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deduplicate-diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "duplicate" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deduplicate-diagnostics.duplicate" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deduplicate-diagnostics.duplicate/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7235938Z     Error {
2020-01-11T07:39:12.7236002Z         line_num: 8,
2020-01-11T07:39:12.7236045Z         kind: Some(
2020-01-11T07:39:12.7236089Z             Error,
2020-01-11T07:39:12.7236089Z             Error,
2020-01-11T07:39:12.7236130Z         ),
2020-01-11T07:39:12.7236199Z         msg: "8:8: 8:17: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7236892Z ]
2020-01-11T07:39:12.7236936Z 
2020-01-11T07:39:12.7237401Z thread '[ui] ui/deduplicate-diagnostics.rs#duplicate' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-11T07:39:12.7237554Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-11T07:39:12.7237554Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-11T07:39:12.7237613Z 
2020-01-11T07:39:12.7237919Z ---- [ui] ui/error-codes/E0452.rs stdout ----
2020-01-11T07:39:12.7237954Z 
2020-01-11T07:39:12.7238257Z error: /checkout/src/test/ui/error-codes/E0452.rs:1: unexpected error: '1:10: 1:18: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7238312Z 
2020-01-11T07:39:12.7238763Z error: /checkout/src/test/ui/error-codes/E0452.rs:1: unexpected error: '1:10: 1:18: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7238841Z error: 2 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7238902Z status: exit code: 1
2020-01-11T07:39:12.7238902Z status: exit code: 1
2020-01-11T07:39:12.7239880Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0452.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0452" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0452/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7240035Z     Error {
2020-01-11T07:39:12.7240097Z         line_num: 1,
2020-01-11T07:39:12.7240138Z         kind: Some(
2020-01-11T07:39:12.7240179Z             Error,
2020-01-11T07:39:12.7240179Z             Error,
2020-01-11T07:39:12.7240593Z         ),
2020-01-11T07:39:12.7240668Z         msg: "1:10: 1:18: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7240769Z     Error {
2020-01-11T07:39:12.7240829Z         line_num: 1,
2020-01-11T07:39:12.7240871Z         kind: Some(
2020-01-11T07:39:12.7240912Z             Error,
2020-01-11T07:39:12.7240912Z             Error,
2020-01-11T07:39:12.7240971Z         ),
2020-01-11T07:39:12.7241024Z         msg: "1:10: 1:18: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7241110Z ]
2020-01-11T07:39:12.7241154Z 
2020-01-11T07:39:12.7241500Z thread '[ui] ui/error-codes/E0452.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-11T07:39:12.7241540Z 
2020-01-11T07:39:12.7241540Z 
2020-01-11T07:39:12.7241785Z ---- [ui] ui/error-codes/E0453.rs stdout ----
2020-01-11T07:39:12.7241818Z 
2020-01-11T07:39:12.7242158Z error: /checkout/src/test/ui/error-codes/E0453.rs:3: unexpected error: '3:9: 3:23: allow(non_snake_case) overruled by outer forbid(non_snake_case) [E0453]'
2020-01-11T07:39:12.7242268Z error: 1 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7242323Z status: exit code: 1
2020-01-11T07:39:12.7242323Z status: exit code: 1
2020-01-11T07:39:12.7243138Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0453.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0453" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0453/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7243297Z     Error {
2020-01-11T07:39:12.7243363Z         line_num: 3,
2020-01-11T07:39:12.7243405Z         kind: Some(
2020-01-11T07:39:12.7243447Z             Error,
2020-01-11T07:39:12.7243447Z             Error,
2020-01-11T07:39:12.7243509Z         ),
2020-01-11T07:39:12.7243560Z         msg: "3:9: 3:23: allow(non_snake_case) overruled by outer forbid(non_snake_case) [E0453]",
2020-01-11T07:39:12.7243791Z ]
2020-01-11T07:39:12.7243817Z 
2020-01-11T07:39:12.7244431Z thread '[ui] ui/error-codes/E0453.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-11T07:39:12.7244476Z 
2020-01-11T07:39:12.7244476Z 
2020-01-11T07:39:12.7244753Z ---- [ui] ui/feature-gates/feature-gate-lint-reasons.rs stdout ----
2020-01-11T07:39:12.7244787Z 
2020-01-11T07:39:12.7245421Z error: /checkout/src/test/ui/feature-gates/feature-gate-lint-reasons.rs:1: unexpected error: '1:28: 1:71: lint reasons are experimental [E0658]'
2020-01-11T07:39:12.7245461Z 
2020-01-11T07:39:12.7245524Z error: 1 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7245565Z status: exit code: 1
2020-01-11T07:39:12.7246364Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-lint-reasons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-lint-reasons" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-lint-reasons/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7246517Z     Error {
2020-01-11T07:39:12.7246558Z         line_num: 1,
2020-01-11T07:39:12.7246597Z         kind: Some(
2020-01-11T07:39:12.7246656Z             Error,
2020-01-11T07:39:12.7246656Z             Error,
2020-01-11T07:39:12.7246693Z         ),
2020-01-11T07:39:12.7246735Z         msg: "1:28: 1:71: lint reasons are experimental [E0658]",
2020-01-11T07:39:12.7246830Z ]
2020-01-11T07:39:12.7246854Z 
2020-01-11T07:39:12.7247146Z thread '[ui] ui/feature-gates/feature-gate-lint-reasons.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-11T07:39:12.7247208Z 
2020-01-11T07:39:12.7247208Z 
2020-01-11T07:39:12.7247415Z ---- [ui] ui/lint/lint-forbid-attr.rs stdout ----
2020-01-11T07:39:12.7247446Z 
2020-01-11T07:39:12.7247750Z error: /checkout/src/test/ui/lint/lint-forbid-attr.rs:3: unexpected error: '3:9: 3:19: allow(deprecated) overruled by outer forbid(deprecated) [E0453]'
2020-01-11T07:39:12.7247843Z error: 1 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7247883Z status: exit code: 1
2020-01-11T07:39:12.7247883Z status: exit code: 1
2020-01-11T07:39:12.7248629Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-forbid-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-attr/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7248790Z     Error {
2020-01-11T07:39:12.7248828Z         line_num: 3,
2020-01-11T07:39:12.7248866Z         kind: Some(
2020-01-11T07:39:12.7248922Z             Error,
2020-01-11T07:39:12.7248922Z             Error,
2020-01-11T07:39:12.7248959Z         ),
2020-01-11T07:39:12.7249004Z         msg: "3:9: 3:19: allow(deprecated) overruled by outer forbid(deprecated) [E0453]",
2020-01-11T07:39:12.7249108Z ]
2020-01-11T07:39:12.7249132Z 
2020-01-11T07:39:12.7249407Z thread '[ui] ui/lint/lint-forbid-attr.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-11T07:39:12.7249460Z 
2020-01-11T07:39:12.7249460Z 
2020-01-11T07:39:12.7249667Z ---- [ui] ui/lint/lint-forbid-cmdline.rs stdout ----
2020-01-11T07:39:12.7249775Z 
2020-01-11T07:39:12.7250110Z error: /checkout/src/test/ui/lint/lint-forbid-cmdline.rs:3: unexpected error: '3:9: 3:19: allow(deprecated) overruled by outer forbid(deprecated) [E0453]'
2020-01-11T07:39:12.7250628Z error: 1 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7250711Z status: exit code: 1
2020-01-11T07:39:12.7250711Z status: exit code: 1
2020-01-11T07:39:12.7251603Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-forbid-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-cmdline" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "deprecated" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-forbid-cmdline/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7251773Z     Error {
2020-01-11T07:39:12.7251819Z         line_num: 3,
2020-01-11T07:39:12.7251880Z         kind: Some(
2020-01-11T07:39:12.7251922Z             Error,
2020-01-11T07:39:12.7251922Z             Error,
2020-01-11T07:39:12.7251970Z         ),
2020-01-11T07:39:12.7252039Z         msg: "3:9: 3:19: allow(deprecated) overruled by outer forbid(deprecated) [E0453]",
2020-01-11T07:39:12.7252125Z ]
2020-01-11T07:39:12.7252152Z 
2020-01-11T07:39:12.7252485Z thread '[ui] ui/lint/lint-forbid-cmdline.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-11T07:39:12.7252525Z 
2020-01-11T07:39:12.7252525Z 
2020-01-11T07:39:12.7252749Z ---- [ui] ui/lint/lint-malformed.rs stdout ----
2020-01-11T07:39:12.7252799Z 
2020-01-11T07:39:12.7253107Z error: /checkout/src/test/ui/lint/lint-malformed.rs:2: unexpected error: '2:10: 2:21: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7253147Z 
2020-01-11T07:39:12.7253451Z error: /checkout/src/test/ui/lint/lint-malformed.rs:2: unexpected error: '2:10: 2:21: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7253560Z error: 2 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7253613Z status: exit code: 1
2020-01-11T07:39:12.7253613Z status: exit code: 1
2020-01-11T07:39:12.7254543Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-malformed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-malformed" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-malformed/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7254694Z     Error {
2020-01-11T07:39:12.7254733Z         line_num: 2,
2020-01-11T07:39:12.7254770Z         kind: Some(
2020-01-11T07:39:12.7254826Z             Error,
2020-01-11T07:39:12.7254826Z             Error,
2020-01-11T07:39:12.7254863Z         ),
2020-01-11T07:39:12.7254911Z         msg: "2:10: 2:21: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7255007Z     Error {
2020-01-11T07:39:12.7255044Z         line_num: 2,
2020-01-11T07:39:12.7255081Z         kind: Some(
2020-01-11T07:39:12.7255136Z             Error,
2020-01-11T07:39:12.7255136Z             Error,
2020-01-11T07:39:12.7255174Z         ),
2020-01-11T07:39:12.7255216Z         msg: "2:10: 2:21: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7255309Z ]
2020-01-11T07:39:12.7255334Z 
2020-01-11T07:39:12.7255609Z thread '[ui] ui/lint/lint-malformed.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-11T07:39:12.7255661Z 
2020-01-11T07:39:12.7255661Z 
2020-01-11T07:39:12.7255862Z ---- [ui] ui/lint/outer-forbid.rs stdout ----
2020-01-11T07:39:12.7255975Z 
2020-01-11T07:39:12.7256526Z error: /checkout/src/test/ui/lint/outer-forbid.rs:9: unexpected error: '9:9: 9:25: allow(unused_variables) overruled by outer forbid(unused) [E0453]'
2020-01-11T07:39:12.7256973Z 
2020-01-11T07:39:12.7257508Z error: /checkout/src/test/ui/lint/outer-forbid.rs:14: unexpected error: '14:9: 14:15: allow(unused) overruled by outer forbid(unused) [E0453]'
2020-01-11T07:39:12.7257553Z 
2020-01-11T07:39:12.7257920Z error: /checkout/src/test/ui/lint/outer-forbid.rs:19: unexpected error: '19:9: 19:26: allow(nonstandard_style) overruled by outer forbid(non_snake_case) [E0453]'
2020-01-11T07:39:12.7258001Z error: 3 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7258062Z status: exit code: 1
2020-01-11T07:39:12.7258062Z status: exit code: 1
2020-01-11T07:39:12.7259290Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/outer-forbid.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/outer-forbid" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/outer-forbid/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7259451Z     Error {
2020-01-11T07:39:12.7259653Z         line_num: 9,
2020-01-11T07:39:12.7259708Z         kind: Some(
2020-01-11T07:39:12.7259746Z             Error,
2020-01-11T07:39:12.7259746Z             Error,
2020-01-11T07:39:12.7259783Z         ),
2020-01-11T07:39:12.7259846Z         msg: "9:9: 9:25: allow(unused_variables) overruled by outer forbid(unused) [E0453]",
2020-01-11T07:39:12.7259925Z     Error {
2020-01-11T07:39:12.7259980Z         line_num: 14,
2020-01-11T07:39:12.7260018Z         kind: Some(
2020-01-11T07:39:12.7260232Z             Error,
2020-01-11T07:39:12.7260232Z             Error,
2020-01-11T07:39:12.7260280Z         ),
2020-01-11T07:39:12.7260692Z         msg: "14:9: 14:15: allow(unused) overruled by outer forbid(unused) [E0453]",
2020-01-11T07:39:12.7260779Z     Error {
2020-01-11T07:39:12.7260846Z         line_num: 19,
2020-01-11T07:39:12.7260888Z         kind: Some(
2020-01-11T07:39:12.7260930Z             Error,
2020-01-11T07:39:12.7260930Z             Error,
2020-01-11T07:39:12.7260988Z         ),
2020-01-11T07:39:12.7261040Z         msg: "19:9: 19:26: allow(nonstandard_style) overruled by outer forbid(non_snake_case) [E0453]",
2020-01-11T07:39:12.7261125Z ]
2020-01-11T07:39:12.7261170Z 
2020-01-11T07:39:12.7261516Z thread '[ui] ui/lint/outer-forbid.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-11T07:39:12.7261556Z 
2020-01-11T07:39:12.7261556Z 
2020-01-11T07:39:12.7261781Z ---- [ui] ui/lint/reasons-forbidden.rs stdout ----
2020-01-11T07:39:12.7261833Z 
2020-01-11T07:39:12.7262175Z error: /checkout/src/test/ui/lint/reasons-forbidden.rs:16: unexpected error: '16:13: 16:24: allow(unsafe_code) overruled by outer forbid(unsafe_code) [E0453]'
2020-01-11T07:39:12.7262228Z 
2020-01-11T07:39:12.7262543Z error: /checkout/src/test/ui/lint/reasons-forbidden.rs:16: unexpected note: 'overruled by previous forbid'
2020-01-11T07:39:12.7262580Z 
2020-01-11T07:39:12.7262863Z error: /checkout/src/test/ui/lint/reasons-forbidden.rs:4: unexpected note: '`forbid` level set here'
2020-01-11T07:39:12.7262901Z 
2020-01-11T07:39:12.7263254Z error: /checkout/src/test/ui/lint/reasons-forbidden.rs:16: unexpected note: '16:13: 16:24: our errors & omissions insurance policy doesn't cover unsafe Rust'
2020-01-11T07:39:12.7263340Z error: 4 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7263404Z status: exit code: 1
2020-01-11T07:39:12.7263404Z status: exit code: 1
2020-01-11T07:39:12.7265083Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/reasons-forbidden.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-forbidden" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-forbidden/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7265326Z     Error {
2020-01-11T07:39:12.7265370Z         line_num: 16,
2020-01-11T07:39:12.7265428Z         kind: Some(
2020-01-11T07:39:12.7265469Z             Error,
2020-01-11T07:39:12.7265469Z             Error,
2020-01-11T07:39:12.7265509Z         ),
2020-01-11T07:39:12.7265575Z         msg: "16:13: 16:24: allow(unsafe_code) overruled by outer forbid(unsafe_code) [E0453]",
2020-01-11T07:39:12.7265661Z     Error {
2020-01-11T07:39:12.7265719Z         line_num: 16,
2020-01-11T07:39:12.7265760Z         kind: Some(
2020-01-11T07:39:12.7265810Z             Note,
2020-01-11T07:39:12.7265810Z             Note,
2020-01-11T07:39:12.7265849Z         ),
2020-01-11T07:39:12.7265910Z         msg: "overruled by previous forbid",
2020-01-11T07:39:12.7265990Z     Error {
2020-01-11T07:39:12.7266243Z         line_num: 4,
2020-01-11T07:39:12.7266285Z         kind: Some(
2020-01-11T07:39:12.7266323Z             Note,
2020-01-11T07:39:12.7266323Z             Note,
2020-01-11T07:39:12.7266519Z         ),
2020-01-11T07:39:12.7266575Z         msg: "`forbid` level set here",
2020-01-11T07:39:12.7266648Z     Error {
2020-01-11T07:39:12.7266703Z         line_num: 16,
2020-01-11T07:39:12.7266740Z         kind: Some(
2020-01-11T07:39:12.7266776Z             Note,
2020-01-11T07:39:12.7266776Z             Note,
2020-01-11T07:39:12.7266829Z         ),
2020-01-11T07:39:12.7267129Z         msg: "16:13: 16:24: our errors & omissions insurance policy doesn\'t cover unsafe Rust",
2020-01-11T07:39:12.7267209Z ]
2020-01-11T07:39:12.7268672Z 
2020-01-11T07:39:12.7269127Z thread '[ui] ui/lint/reasons-forbidden.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-11T07:39:12.7269181Z 
2020-01-11T07:39:12.7269181Z 
2020-01-11T07:39:12.7269393Z ---- [ui] ui/lint/reasons-erroneous.rs stdout ----
2020-01-11T07:39:12.7269444Z 
2020-01-11T07:39:12.7269735Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:3: unexpected error: '3:58: 3:59: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7271599Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:3: unexpected note: 'reason must be a string literal'
2020-01-11T07:39:12.7271654Z 
2020-01-11T07:39:12.7271654Z 
2020-01-11T07:39:12.7272076Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:10: unexpected error: '10:40: 10:85: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7272425Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:10: unexpected note: 'reason must be a string literal'
2020-01-11T07:39:12.7272463Z 
2020-01-11T07:39:12.7272463Z 
2020-01-11T07:39:12.7272775Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:17: unexpected error: '17:29: 17:92: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7274418Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:17: unexpected note: 'bad attribute argument'
2020-01-11T07:39:12.7274468Z 
2020-01-11T07:39:12.7274468Z 
2020-01-11T07:39:12.7274770Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:17: unexpected error: '17:29: 17:92: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7275082Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:17: unexpected note: 'bad attribute argument'
2020-01-11T07:39:12.7275115Z 
2020-01-11T07:39:12.7275115Z 
2020-01-11T07:39:12.7275396Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:30: unexpected error: '30:23: 30:86: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7276666Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:30: unexpected note: 'bad attribute argument'
2020-01-11T07:39:12.7276728Z 
2020-01-11T07:39:12.7276728Z 
2020-01-11T07:39:12.7278140Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:30: unexpected error: '30:23: 30:86: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7278619Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:30: unexpected note: 'bad attribute argument'
2020-01-11T07:39:12.7278663Z 
2020-01-11T07:39:12.7278663Z 
2020-01-11T07:39:12.7278998Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:43: unexpected error: '43:36: 43:98: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7279288Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:43: unexpected note: 'bad attribute argument'
2020-01-11T07:39:12.7279321Z 
2020-01-11T07:39:12.7279321Z 
2020-01-11T07:39:12.7279618Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:43: unexpected error: '43:36: 43:98: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7279902Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:43: unexpected note: 'bad attribute argument'
2020-01-11T07:39:12.7279947Z 
2020-01-11T07:39:12.7279947Z 
2020-01-11T07:39:12.7280250Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:56: unexpected error: '56:44: 56:66: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7281303Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:56: unexpected note: 'reason in lint attribute must come last'
2020-01-11T07:39:12.7281348Z 
2020-01-11T07:39:12.7281348Z 
2020-01-11T07:39:12.7281699Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:63: unexpected error: '63:25: 63:50: malformed lint attribute input [E0452]'
2020-01-11T07:39:12.7282032Z error: /checkout/src/test/ui/lint/reasons-erroneous.rs:63: unexpected note: 'reason in lint attribute must come last'
2020-01-11T07:39:12.7282087Z 
2020-01-11T07:39:12.7282135Z error: 20 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7282181Z status: exit code: 1
2020-01-11T07:39:12.7282181Z status: exit code: 1
2020-01-11T07:39:12.7283026Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/reasons-erroneous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-erroneous" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-erroneous/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7283198Z     Error {
2020-01-11T07:39:12.7283245Z         line_num: 3,
2020-01-11T07:39:12.7283288Z         kind: Some(
2020-01-11T07:39:12.7283350Z             Error,
2020-01-11T07:39:12.7283350Z             Error,
2020-01-11T07:39:12.7283393Z         ),
2020-01-11T07:39:12.7283440Z         msg: "3:58: 3:59: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7283548Z     Error {
2020-01-11T07:39:12.7283589Z         line_num: 3,
2020-01-11T07:39:12.7283639Z         kind: Some(
2020-01-11T07:39:12.7283701Z             Note,
---
2020-01-11T07:39:12.7283942Z         line_num: 10,
2020-01-11T07:39:12.7283984Z         kind: Some(
2020-01-11T07:39:12.7284047Z             Error,
2020-01-11T07:39:12.7284251Z         ),
2020-01-11T07:39:12.7284294Z         msg: "10:40: 10:85: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7284389Z     Error {
2020-01-11T07:39:12.7284425Z         line_num: 10,
2020-01-11T07:39:12.7284481Z         kind: Some(
2020-01-11T07:39:12.7284518Z             Note,
---
2020-01-11T07:39:12.7284829Z         line_num: 17,
2020-01-11T07:39:12.7284884Z         kind: Some(
2020-01-11T07:39:12.7284921Z             Error,
2020-01-11T07:39:12.7284958Z         ),
2020-01-11T07:39:12.7284999Z         msg: "17:29: 17:92: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7285150Z     Error {
2020-01-11T07:39:12.7285187Z         line_num: 17,
2020-01-11T07:39:12.7285240Z         kind: Some(
2020-01-11T07:39:12.7285277Z             Note,
2020-01-11T07:39:12.7285277Z             Note,
2020-01-11T07:39:12.7285314Z         ),
2020-01-11T07:39:12.7285372Z         msg: "bad attribute argument",
2020-01-11T07:39:12.7285677Z     Error {
2020-01-11T07:39:12.7285717Z         line_num: 17,
2020-01-11T07:39:12.7285777Z         kind: Some(
2020-01-11T07:39:12.7285815Z             Error,
2020-01-11T07:39:12.7285815Z             Error,
2020-01-11T07:39:12.7285852Z         ),
2020-01-11T07:39:12.7285913Z         msg: "17:29: 17:92: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7285991Z     Error {
2020-01-11T07:39:12.7286036Z         line_num: 17,
2020-01-11T07:39:12.7286094Z         kind: Some(
2020-01-11T07:39:12.7286132Z             Note,
2020-01-11T07:39:12.7286132Z             Note,
2020-01-11T07:39:12.7286169Z         ),
2020-01-11T07:39:12.7286227Z         msg: "bad attribute argument",
2020-01-11T07:39:12.7286309Z     Error {
2020-01-11T07:39:12.7286365Z         line_num: 30,
2020-01-11T07:39:12.7286404Z         kind: Some(
2020-01-11T07:39:12.7286441Z             Error,
2020-01-11T07:39:12.7286441Z             Error,
2020-01-11T07:39:12.7286479Z         ),
2020-01-11T07:39:12.7286540Z         msg: "30:23: 30:86: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7286617Z     Error {
2020-01-11T07:39:12.7286673Z         line_num: 30,
2020-01-11T07:39:12.7286712Z         kind: Some(
2020-01-11T07:39:12.7286749Z             Note,
2020-01-11T07:39:12.7286749Z             Note,
2020-01-11T07:39:12.7286787Z         ),
2020-01-11T07:39:12.7286849Z         msg: "bad attribute argument",
2020-01-11T07:39:12.7286924Z     Error {
2020-01-11T07:39:12.7286987Z         line_num: 30,
2020-01-11T07:39:12.7287025Z         kind: Some(
2020-01-11T07:39:12.7287064Z             Error,
2020-01-11T07:39:12.7287064Z             Error,
2020-01-11T07:39:12.7287118Z         ),
2020-01-11T07:39:12.7287169Z         msg: "30:23: 30:86: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7287245Z     Error {
2020-01-11T07:39:12.7287456Z         line_num: 30,
2020-01-11T07:39:12.7287493Z         kind: Some(
2020-01-11T07:39:12.7287530Z             Note,
2020-01-11T07:39:12.7287530Z             Note,
2020-01-11T07:39:12.7287585Z         ),
2020-01-11T07:39:12.7287624Z         msg: "bad attribute argument",
2020-01-11T07:39:12.7287696Z     Error {
2020-01-11T07:39:12.7287751Z         line_num: 43,
2020-01-11T07:39:12.7287788Z         kind: Some(
2020-01-11T07:39:12.7287824Z             Error,
2020-01-11T07:39:12.7287824Z             Error,
2020-01-11T07:39:12.7287877Z         ),
2020-01-11T07:39:12.7287919Z         msg: "43:36: 43:98: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7288018Z     Error {
2020-01-11T07:39:12.7288055Z         line_num: 43,
2020-01-11T07:39:12.7288092Z         kind: Some(
2020-01-11T07:39:12.7288129Z             Note,
2020-01-11T07:39:12.7288129Z             Note,
2020-01-11T07:39:12.7288183Z         ),
2020-01-11T07:39:12.7288228Z         msg: "bad attribute argument",
2020-01-11T07:39:12.7288319Z     Error {
2020-01-11T07:39:12.7288356Z         line_num: 43,
2020-01-11T07:39:12.7288393Z         kind: Some(
2020-01-11T07:39:12.7288430Z             Error,
2020-01-11T07:39:12.7288430Z             Error,
2020-01-11T07:39:12.7288483Z         ),
2020-01-11T07:39:12.7288524Z         msg: "43:36: 43:98: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7288615Z     Error {
2020-01-11T07:39:12.7288651Z         line_num: 43,
2020-01-11T07:39:12.7288688Z         kind: Some(
2020-01-11T07:39:12.7288742Z             Note,
2020-01-11T07:39:12.7288742Z             Note,
2020-01-11T07:39:12.7288778Z         ),
2020-01-11T07:39:12.7288816Z         msg: "bad attribute argument",
2020-01-11T07:39:12.7289131Z     Error {
2020-01-11T07:39:12.7289168Z         line_num: 56,
2020-01-11T07:39:12.7289203Z         kind: Some(
2020-01-11T07:39:12.7289258Z             Error,
2020-01-11T07:39:12.7289258Z             Error,
2020-01-11T07:39:12.7289293Z         ),
2020-01-11T07:39:12.7291029Z         msg: "56:44: 56:66: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7291187Z     Error {
2020-01-11T07:39:12.7291228Z         line_num: 56,
2020-01-11T07:39:12.7291270Z         kind: Some(
2020-01-11T07:39:12.7291330Z             Note,
---
2020-01-11T07:39:12.7291559Z         line_num: 63,
2020-01-11T07:39:12.7291600Z         kind: Some(
2020-01-11T07:39:12.7291661Z             Error,
2020-01-11T07:39:12.7291702Z         ),
2020-01-11T07:39:12.7291748Z         msg: "63:25: 63:50: malformed lint attribute input [E0452]",
2020-01-11T07:39:12.7291863Z     Error {
2020-01-11T07:39:12.7291903Z         line_num: 63,
2020-01-11T07:39:12.7291945Z         kind: Some(
2020-01-11T07:39:12.7292004Z             Note,
---
2020-01-11T07:39:12.7294677Z 22 For more information about an error, try `rustc --explain E0432`.
2020-01-11T07:39:12.7294711Z 
2020-01-11T07:39:12.7294737Z 
2020-01-11T07:39:12.7294800Z The actual stderr differed from the expected stderr.
2020-01-11T07:39:12.7295092Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/privacy2.stderr
2020-01-11T07:39:12.7295344Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T07:39:12.7295792Z To only update this specific test, also pass `--test-args privacy/privacy2.rs`
2020-01-11T07:39:12.7296028Z error: 1 errors occurred comparing output.
2020-01-11T07:39:12.7296097Z status: exit code: 1
2020-01-11T07:39:12.7296097Z status: exit code: 1
2020-01-11T07:39:12.7296818Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy2/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7297123Z ------------------------------------------
2020-01-11T07:39:12.7297153Z 
2020-01-11T07:39:12.7297367Z ------------------------------------------
2020-01-11T07:39:12.7297408Z stderr:
---
2020-01-11T07:39:12.7298238Z 
2020-01-11T07:39:12.7298276Z error[E0603]: function `foo` is private
2020-01-11T07:39:12.7298507Z   --> /checkout/src/test/ui/privacy/privacy2.rs:23:20
2020-01-11T07:39:12.7298566Z    |
2020-01-11T07:39:12.7298605Z LL |     use bar::glob::foo;
2020-01-11T07:39:12.7298688Z 
2020-01-11T07:39:12.7298727Z error: requires `sized` lang_item
2020-01-11T07:39:12.7298753Z 
2020-01-11T07:39:12.7298791Z error: aborting due to 3 previous errors
---
2020-01-11T07:39:12.7302227Z 16 
2020-01-11T07:39:12.7302272Z 
2020-01-11T07:39:12.7302299Z 
2020-01-11T07:39:12.7302357Z The actual stderr differed from the expected stderr.
2020-01-11T07:39:12.7302657Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/privacy3.stderr
2020-01-11T07:39:12.7302938Z To update references, rerun the tests and pass the `--bless` flag
2020-01-11T07:39:12.7303206Z To only update this specific test, also pass `--test-args privacy/privacy3.rs`
2020-01-11T07:39:12.7303303Z error: 1 errors occurred comparing output.
2020-01-11T07:39:12.7303347Z status: exit code: 1
2020-01-11T07:39:12.7303347Z status: exit code: 1
2020-01-11T07:39:12.7304139Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/privacy/privacy3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/privacy/privacy3/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7304488Z ------------------------------------------
2020-01-11T07:39:12.7304542Z 
2020-01-11T07:39:12.7304764Z ------------------------------------------
2020-01-11T07:39:12.7304811Z stderr:
2020-01-11T07:39:12.7304811Z stderr:
2020-01-11T07:39:12.7305179Z ------------------------------------------
2020-01-11T07:39:12.7305245Z error[E0432]: unresolved import `bar::gpriv`
2020-01-11T07:39:12.7305516Z    |
2020-01-11T07:39:12.7305516Z    |
2020-01-11T07:39:12.7305575Z LL |     use bar::gpriv;
2020-01-11T07:39:12.7305619Z    |         ^^^^^^^^^^ no `gpriv` in `bar`
2020-01-11T07:39:12.7305688Z error: requires `sized` lang_item
2020-01-11T07:39:12.7305734Z 
2020-01-11T07:39:12.7305774Z error: aborting due to 2 previous errors
2020-01-11T07:39:12.7305802Z 
2020-01-11T07:39:12.7305802Z 
2020-01-11T07:39:12.7306340Z For more information about this error, try `rustc --explain E0432`.
2020-01-11T07:39:12.7306395Z 
2020-01-11T07:39:12.7306757Z ------------------------------------------
2020-01-11T07:39:12.7306785Z 
2020-01-11T07:39:12.7306808Z 
2020-01-11T07:39:12.7307088Z ---- [ui] ui/tool_lints.rs stdout ----
2020-01-11T07:39:12.7307123Z 
2020-01-11T07:39:12.7307606Z error: /checkout/src/test/ui/tool_lints.rs:1: unexpected error: '1:8: 1:11: an unknown tool name found in scoped lint: `foo::bar` [E0710]'
2020-01-11T07:39:12.7307712Z error: 1 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7307755Z status: exit code: 1
2020-01-11T07:39:12.7307755Z status: exit code: 1
2020-01-11T07:39:12.7308506Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/tool_lints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/tool_lints/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7308813Z     Error {
2020-01-11T07:39:12.7308853Z         line_num: 1,
2020-01-11T07:39:12.7308891Z         kind: Some(
2020-01-11T07:39:12.7308929Z             Error,
2020-01-11T07:39:12.7308929Z             Error,
2020-01-11T07:39:12.7308984Z         ),
2020-01-11T07:39:12.7309029Z         msg: "1:8: 1:11: an unknown tool name found in scoped lint: `foo::bar` [E0710]",
2020-01-11T07:39:12.7309127Z ]
2020-01-11T07:39:12.7309150Z 
2020-01-11T07:39:12.7309417Z thread '[ui] ui/tool_lints.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-11T07:39:12.7309451Z 
2020-01-11T07:39:12.7309451Z 
2020-01-11T07:39:12.7309672Z ---- [ui] ui/unknown-lint-tool-name.rs stdout ----
2020-01-11T07:39:12.7309711Z 
2020-01-11T07:39:12.7310164Z error: /checkout/src/test/ui/unknown-lint-tool-name.rs:1: unexpected error: '1:9: 1:12: an unknown tool name found in scoped lint: `foo::bar` [E0710]'
2020-01-11T07:39:12.7310220Z 
2020-01-11T07:39:12.7311497Z error: /checkout/src/test/ui/unknown-lint-tool-name.rs:5: unexpected error: '5:9: 5:12: an unknown tool name found in scoped lint: `foo::bar` [E0710]'
2020-01-11T07:39:12.7311593Z error: 2 unexpected errors found, 0 expected errors not found
2020-01-11T07:39:12.7311659Z status: exit code: 1
2020-01-11T07:39:12.7311659Z status: exit code: 1
2020-01-11T07:39:12.7312506Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unknown-lint-tool-name.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unknown-lint-tool-name" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unknown-lint-tool-name/auxiliary" "-A" "unused"
2020-01-11T07:39:12.7312683Z     Error {
2020-01-11T07:39:12.7312749Z         line_num: 1,
2020-01-11T07:39:12.7312791Z         kind: Some(
2020-01-11T07:39:12.7312832Z             Error,
2020-01-11T07:39:12.7312832Z             Error,
2020-01-11T07:39:12.7312872Z         ),
2020-01-11T07:39:12.7312940Z         msg: "1:9: 1:12: an unknown tool name found in scoped lint: `foo::bar` [E0710]",
2020-01-11T07:39:12.7313028Z     Error {
2020-01-11T07:39:12.7313088Z         line_num: 5,
2020-01-11T07:39:12.7313130Z         kind: Some(
2020-01-11T07:39:12.7313173Z             Error,
2020-01-11T07:39:12.7313173Z             Error,
2020-01-11T07:39:12.7313234Z         ),
2020-01-11T07:39:12.7313284Z         msg: "5:9: 5:12: an unknown tool name found in scoped lint: `foo::bar` [E0710]",
2020-01-11T07:39:12.7313755Z ]
2020-01-11T07:39:12.7313802Z 
2020-01-11T07:39:12.7314163Z thread '[ui] ui/unknown-lint-tool-name.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1436:13
2020-01-11T07:39:12.7314205Z 
---
2020-01-11T07:39:12.7318441Z 
2020-01-11T07:39:12.7318693Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:386:22
2020-01-11T07:39:12.7318726Z 
2020-01-11T07:39:12.7318750Z 
2020-01-11T07:39:12.7321237Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-11T07:39:12.7321548Z 
2020-01-11T07:39:12.7321577Z 
2020-01-11T07:39:12.7321624Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-11T07:39:12.7321674Z Build completed unsuccessfully in 1:04:12
2020-01-11T07:39:12.7321674Z Build completed unsuccessfully in 1:04:12
2020-01-11T07:39:12.7353075Z == clock drift check ==
2020-01-11T07:39:12.7387997Z   local time: Sat Jan 11 07:39:12 UTC 2020
2020-01-11T07:39:13.0369406Z   network time: Sat, 11 Jan 2020 07:39:13 GMT
2020-01-11T07:39:13.0375435Z == end clock drift check ==
2020-01-11T07:39:17.6224631Z 
2020-01-11T07:39:17.6333170Z ##[error]Bash exited with code '1'.
2020-01-11T07:39:17.6378513Z ##[section]Starting: Checkout
2020-01-11T07:39:17.6380813Z ==============================================================================
2020-01-11T07:39:17.6380890Z Task         : Get sources
2020-01-11T07:39:17.6380937Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
