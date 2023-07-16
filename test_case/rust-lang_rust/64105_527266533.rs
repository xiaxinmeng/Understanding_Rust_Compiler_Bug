plain
2019-09-02T23:44:32.2596802Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-02T23:44:32.8904171Z ##[command]git config gc.auto 0
2019-09-02T23:44:32.8907525Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-02T23:44:32.8910421Z ##[command]git config --get-all http.proxy
2019-09-02T23:44:32.8915579Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64105/merge:refs/remotes/pull/64105/merge
---
2019-09-03T00:50:08.3087562Z .................................................................................................... 1500/8987
2019-09-03T00:50:14.3291396Z .................................................................................................... 1600/8987
2019-09-03T00:50:27.8083591Z .................................................i...............i.................................. 1700/8987
2019-09-03T00:50:36.5669672Z .................................................................................................... 1800/8987
2019-09-03T00:50:51.3728442Z ................................F.......iiiii....................................................... 1900/8987
2019-09-03T00:51:03.0518505Z .................................................................................................... 2100/8987
2019-09-03T00:51:05.8492063Z .................................................................................................... 2200/8987
2019-09-03T00:51:10.2811415Z .................................................................................................... 2300/8987
2019-09-03T00:51:18.3994344Z .........F.......................................................................................... 2400/8987
---
2019-09-03T00:54:29.9737117Z ...........................i...............i........................................................ 4700/8987
2019-09-03T00:54:42.7132729Z .................................................................................................... 4800/8987
2019-09-03T00:54:49.3228946Z .................................................................................................... 4900/8987
2019-09-03T00:55:01.0520234Z .................................................................................................... 5000/8987
2019-09-03T00:55:07.1147815Z ........ii.ii....................................................................................... 5100/8987
2019-09-03T00:55:21.2511147Z .................................................................................................... 5300/8987
2019-09-03T00:55:30.1410410Z .......................................................................i............................ 5400/8987
2019-09-03T00:55:37.9411557Z .................................................................................................... 5500/8987
2019-09-03T00:55:45.0437262Z .................................................................................................... 5600/8987
2019-09-03T00:55:45.0437262Z .................................................................................................... 5600/8987
2019-09-03T00:55:56.4666678Z .................................................................ii...i..ii...........i............. 5700/8987
2019-09-03T00:56:24.2393843Z .................................................................................................... 5900/8987
2019-09-03T00:56:35.1822909Z .................................................................................................... 6000/8987
2019-09-03T00:56:35.1822909Z .................................................................................................... 6000/8987
2019-09-03T00:56:42.4863032Z ...................................................................i..ii............................ 6100/8987
2019-09-03T00:57:13.8592807Z .................................................................................................... 6300/8987
2019-09-03T00:57:16.1670978Z ......................i............................................................................. 6400/8987
2019-09-03T00:57:18.5726968Z ..............................................................................................i..... 6500/8987
2019-09-03T00:57:21.5165018Z .................................................................................................... 6600/8987
---
2019-09-03T01:01:45.0498366Z 
2019-09-03T01:01:45.0499259Z ---- [ui] ui/did_you_mean/issue-54109-and_instead_of_ampersands.rs stdout ----
2019-09-03T01:01:45.0499785Z diff of stderr:
2019-09-03T01:01:45.0500061Z 
2019-09-03T01:01:45.0500542Z 27    |           expected one of 8 possible tokens here
2019-09-03T01:01:45.0500823Z 28    |           help: use `&&` instead of `and` for the boolean operator
2019-09-03T01:01:45.0501155Z 29 
2019-09-03T01:01:45.0501571Z + error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `b`
2019-09-03T01:01:45.0502239Z +   --> $DIR/issue-54109-and_instead_of_ampersands.rs:22:15
2019-09-03T01:01:45.0502737Z +    |
2019-09-03T01:01:45.0502995Z + LL |     if (a and b) {
2019-09-03T01:01:45.0503368Z +    |               ^ expected one of 8 possible tokens here
2019-09-03T01:01:45.0503811Z + 
2019-09-03T01:01:45.0504567Z 30 error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `or`
2019-09-03T01:01:45.0506582Z 32    |
2019-09-03T01:01:45.0506790Z 
2019-09-03T01:01:45.0507047Z 36    |           expected one of 8 possible tokens here
2019-09-03T01:01:45.0507047Z 36    |           expected one of 8 possible tokens here
2019-09-03T01:01:45.0507285Z 37    |           help: use `||` instead of `or` for the boolean operator
2019-09-03T01:01:45.0507662Z 38 
2019-09-03T01:01:45.0508076Z + error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `b`
2019-09-03T01:01:45.0508633Z +   --> $DIR/issue-54109-and_instead_of_ampersands.rs:31:14
2019-09-03T01:01:45.0508908Z +    |
2019-09-03T01:01:45.0509154Z + LL |     if (a or b) {
2019-09-03T01:01:45.0509372Z +    |              ^ expected one of 8 possible tokens here
2019-09-03T01:01:45.0509591Z + 
2019-09-03T01:01:45.0509836Z 39 error: expected one of `!`, `.`, `::`, `?`, `{`, or an operator, found `and`
2019-09-03T01:01:45.0510584Z 41    |
2019-09-03T01:01:45.0510800Z 
2019-09-03T01:01:45.0510800Z 
2019-09-03T01:01:45.0511548Z 54    |             expected one of `!`, `.`, `::`, `?`, `{`, or an operator here
2019-09-03T01:01:45.0511777Z 55    |             help: use `||` instead of `or` for the boolean operator
2019-09-03T01:01:45.0512411Z - error: aborting due to 6 previous errors
2019-09-03T01:01:45.0512411Z - error: aborting due to 6 previous errors
2019-09-03T01:01:45.0512599Z + error[E0425]: cannot find value `and` in this scope
2019-09-03T01:01:45.0512997Z +   --> $DIR/issue-54109-and_instead_of_ampersands.rs:22:11
2019-09-03T01:01:45.0513172Z +    |
2019-09-03T01:01:45.0513313Z + LL |     if (a and b) {
2019-09-03T01:01:45.0513915Z 58 
2019-09-03T01:01:45.0513915Z 58 
2019-09-03T01:01:45.0514851Z + error[E0425]: cannot find value `or` in this scope
2019-09-03T01:01:45.0515282Z +   --> $DIR/issue-54109-and_instead_of_ampersands.rs:31:11
2019-09-03T01:01:45.0515468Z +    |
2019-09-03T01:01:45.0515638Z + LL |     if (a or b) {
2019-09-03T01:01:45.0515982Z + 
2019-09-03T01:01:45.0516152Z + error: aborting due to 10 previous errors
2019-09-03T01:01:45.0516305Z + 
2019-09-03T01:01:45.0516698Z + For more information about this error, try `rustc --explain E0425`.
2019-09-03T01:01:45.0516698Z + For more information about this error, try `rustc --explain E0425`.
2019-09-03T01:01:45.0516903Z 59 
2019-09-03T01:01:45.0517039Z 
2019-09-03T01:01:45.0517171Z 
2019-09-03T01:01:45.0517324Z The actual stderr differed from the expected stderr.
2019-09-03T01:01:45.0517852Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands/issue-54109-and_instead_of_ampersands.stderr
2019-09-03T01:01:45.0518320Z To update references, rerun the tests and pass the `--bless` flag
2019-09-03T01:01:45.0518811Z To only update this specific test, also pass `--test-args did_you_mean/issue-54109-and_instead_of_ampersands.rs`
2019-09-03T01:01:45.0519138Z error: 1 errors occurred comparing output.
2019-09-03T01:01:45.0519321Z status: exit code: 1
2019-09-03T01:01:45.0519321Z status: exit code: 1
2019-09-03T01:01:45.0520301Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands/auxiliary" "-A" "unused"
2019-09-03T01:01:45.0521095Z ------------------------------------------
2019-09-03T01:01:45.0521272Z 
2019-09-03T01:01:45.0521793Z ------------------------------------------
2019-09-03T01:01:45.0521962Z stderr:
2019-09-03T01:01:45.0521962Z stderr:
2019-09-03T01:01:45.0522323Z ------------------------------------------
2019-09-03T01:01:45.0522689Z error: expected `{`, found `and`
2019-09-03T01:01:45.0523329Z    |
2019-09-03T01:01:45.0523691Z LL |     if a and b {
2019-09-03T01:01:45.0524415Z    |     --   ^^^
2019-09-03T01:01:45.0524653Z    |     |    |
2019-09-03T01:01:45.0524653Z    |     |    |
2019-09-03T01:01:45.0524812Z    |     |    expected `{`
2019-09-03T01:01:45.0525107Z    |     |    help: use `&&` instead of `and` for the boolean operator
2019-09-03T01:01:45.0525320Z    |     this `if` statement has a condition, but no block
2019-09-03T01:01:45.0525457Z 
2019-09-03T01:01:45.0525625Z error: expected `{`, found `or`
2019-09-03T01:01:45.0526282Z    |
2019-09-03T01:01:45.0526282Z    |
2019-09-03T01:01:45.0526455Z LL |     if a or b {
2019-09-03T01:01:45.0526786Z    |     --   ^^
2019-09-03T01:01:45.0527131Z    |     |    expected `{`
2019-09-03T01:01:45.0527131Z    |     |    expected `{`
2019-09-03T01:01:45.0527307Z    |     |    help: use `||` instead of `or` for the boolean operator
2019-09-03T01:01:45.0527467Z    |     this `if` statement has a condition, but no block
2019-09-03T01:01:45.0527962Z 
2019-09-03T01:01:45.0528137Z error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `and`
2019-09-03T01:01:45.0528745Z    |
2019-09-03T01:01:45.0528745Z    |
2019-09-03T01:01:45.0528893Z LL |     if (a and b) {
2019-09-03T01:01:45.0529201Z    |           |
2019-09-03T01:01:45.0529453Z    |           expected one of 8 possible tokens here
2019-09-03T01:01:45.0529453Z    |           expected one of 8 possible tokens here
2019-09-03T01:01:45.0529618Z    |           help: use `&&` instead of `and` for the boolean operator
2019-09-03T01:01:45.0529768Z 
2019-09-03T01:01:45.0529925Z error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `b`
2019-09-03T01:01:45.0530554Z    |
2019-09-03T01:01:45.0530554Z    |
2019-09-03T01:01:45.0530705Z LL |     if (a and b) {
2019-09-03T01:01:45.0530872Z    |               ^ expected one of 8 possible tokens here
2019-09-03T01:01:45.0531001Z 
2019-09-03T01:01:45.0531155Z error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `or`
2019-09-03T01:01:45.0531759Z    |
2019-09-03T01:01:45.0531759Z    |
2019-09-03T01:01:45.0531905Z LL |     if (a or b) {
2019-09-03T01:01:45.0532213Z    |           |
2019-09-03T01:01:45.0532364Z    |           expected one of 8 possible tokens here
2019-09-03T01:01:45.0532364Z    |           expected one of 8 possible tokens here
2019-09-03T01:01:45.0532530Z    |           help: use `||` instead of `or` for the boolean operator
2019-09-03T01:01:45.0532661Z 
2019-09-03T01:01:45.0532817Z error: expected one of `!`, `)`, `,`, `.`, `::`, `?`, `{`, or an operator, found `b`
2019-09-03T01:01:45.0533417Z    |
2019-09-03T01:01:45.0533417Z    |
2019-09-03T01:01:45.0533564Z LL |     if (a or b) {
2019-09-03T01:01:45.0533731Z    |              ^ expected one of 8 possible tokens here
2019-09-03T01:01:45.0533859Z 
2019-09-03T01:01:45.0534371Z error: expected one of `!`, `.`, `::`, `?`, `{`, or an operator, found `and`
2019-09-03T01:01:45.0536643Z    |
2019-09-03T01:01:45.0536889Z LL |     while a and b {
2019-09-03T01:01:45.0537049Z    |             ^^^
2019-09-03T01:01:45.0537226Z    |             |
2019-09-03T01:01:45.0537226Z    |             |
2019-09-03T01:01:45.0537387Z    |             expected one of `!`, `.`, `::`, `?`, `{`, or an operator here
2019-09-03T01:01:45.0537553Z    |             help: use `&&` instead of `and` for the boolean operator
2019-09-03T01:01:45.0537689Z 
2019-09-03T01:01:45.0537870Z error: expected one of `!`, `.`, `::`, `?`, `{`, or an operator, found `or`
2019-09-03T01:01:45.0538817Z    |
2019-09-03T01:01:45.0538817Z    |
2019-09-03T01:01:45.0538974Z LL |     while a or b {
2019-09-03T01:01:45.0539320Z    |             |
2019-09-03T01:01:45.0539320Z    |             |
2019-09-03T01:01:45.0539484Z    |             expected one of `!`, `.`, `::`, `?`, `{`, or an operator here
2019-09-03T01:01:45.0539764Z    |             help: use `||` instead of `or` for the boolean operator
2019-09-03T01:01:45.0540134Z error[E0425]: cannot find value `and` in this scope
2019-09-03T01:01:45.0546582Z   --> /checkout/src/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands.rs:22:11
2019-09-03T01:01:45.0546921Z    |
2019-09-03T01:01:45.0546921Z    |
2019-09-03T01:01:45.0547089Z LL |     if (a and b) {
2019-09-03T01:01:45.0547757Z 
2019-09-03T01:01:45.0547944Z error[E0425]: cannot find value `or` in this scope
2019-09-03T01:01:45.0548443Z   --> /checkout/src/test/ui/did_you_mean/issue-54109-and_instead_of_ampersands.rs:31:11
2019-09-03T01:01:45.0548634Z    |
2019-09-03T01:01:45.0548634Z    |
2019-09-03T01:01:45.0548787Z LL |     if (a or b) {
2019-09-03T01:01:45.0549080Z 
2019-09-03T01:01:45.0549231Z error: aborting due to 10 previous errors
2019-09-03T01:01:45.0549359Z 
2019-09-03T01:01:45.0549748Z For more information about this error, try `rustc --explain E0425`.
2019-09-03T01:01:45.0549748Z For more information about this error, try `rustc --explain E0425`.
2019-09-03T01:01:45.0549943Z 
2019-09-03T01:01:45.0550318Z ------------------------------------------
2019-09-03T01:01:45.0550499Z 
2019-09-03T01:01:45.0550633Z 
2019-09-03T01:01:45.0550987Z ---- [ui] ui/expr_attr_paren_order.rs stdout ----
2019-09-03T01:01:45.0551169Z diff of stderr:
2019-09-03T01:01:45.0551320Z 
2019-09-03T01:01:45.0551690Z - error: variable `X` should have a snake case name
2019-09-03T01:01:45.0552276Z + error: an inner attribute is not permitted in this context
2019-09-03T01:01:45.0552709Z +   --> $DIR/expr_attr_paren_order.rs:9:9
2019-09-03T01:01:45.0552924Z 3    |
2019-09-03T01:01:45.0553280Z - LL |             let X = 0;
2019-09-03T01:01:45.0553280Z - LL |             let X = 0;
2019-09-03T01:01:45.0553685Z -    |                 ^ help: convert the identifier to snake case: `x`
2019-09-03T01:01:45.0554047Z + LL |         #![allow(non_snake_case)]
2019-09-03T01:01:45.0554940Z 6    |
2019-09-03T01:01:45.0555329Z - note: lint level defined here
2019-09-03T01:01:45.0555768Z -   --> $DIR/expr_attr_paren_order.rs:17:17
2019-09-03T01:01:45.0555768Z -   --> $DIR/expr_attr_paren_order.rs:17:17
2019-09-03T01:01:45.0556302Z +    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-09-03T01:01:45.0556677Z + error: an inner attribute is not permitted in this context
2019-09-03T01:01:45.0557092Z +   --> $DIR/expr_attr_paren_order.rs:17:9
2019-09-03T01:01:45.0557285Z 9    |
2019-09-03T01:01:45.0557285Z 9    |
2019-09-03T01:01:45.0557466Z 10 LL |         #![deny(non_snake_case)]
2019-09-03T01:01:45.0564421Z +    |         ^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-03T01:01:45.0564504Z +    |
2019-09-03T01:01:45.0564504Z +    |
2019-09-03T01:01:45.0564585Z +    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-09-03T01:01:45.0565083Z - error: aborting due to previous error
2019-09-03T01:01:45.0565137Z + error: aborting due to 2 previous errors
2019-09-03T01:01:45.0565180Z 14 
2019-09-03T01:01:45.0565236Z 15 
2019-09-03T01:01:45.0565236Z 15 
2019-09-03T01:01:45.0565266Z 
2019-09-03T01:01:45.0565292Z 
2019-09-03T01:01:45.0565338Z The actual stderr differed from the expected stderr.
2019-09-03T01:01:45.0565679Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr_attr_paren_order/expr_attr_paren_order.stderr
2019-09-03T01:01:45.0566169Z To update references, rerun the tests and pass the `--bless` flag
2019-09-03T01:01:45.0566467Z To only update this specific test, also pass `--test-args expr_attr_paren_order.rs`
2019-09-03T01:01:45.0566574Z error: 1 errors occurred comparing output.
2019-09-03T01:01:45.0566623Z status: exit code: 1
2019-09-03T01:01:45.0566623Z status: exit code: 1
2019-09-03T01:01:45.0573884Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/expr_attr_paren_order.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr_attr_paren_order" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/expr_attr_paren_order/auxiliary" "-A" "unused"
2019-09-03T01:01:45.0575541Z ------------------------------------------
2019-09-03T01:01:45.0575608Z 
2019-09-03T01:01:45.0575853Z ------------------------------------------
2019-09-03T01:01:45.0575902Z stderr:
2019-09-03T01:01:45.0575902Z stderr:
2019-09-03T01:01:45.0576132Z ------------------------------------------
2019-09-03T01:01:45.0576185Z error: an inner attribute is not permitted in this context
2019-09-03T01:01:45.0576436Z   --> /checkout/src/test/ui/expr_attr_paren_order.rs:9:9
2019-09-03T01:01:45.0576505Z    |
2019-09-03T01:01:45.0576550Z LL |         #![allow(non_snake_case)]
2019-09-03T01:01:45.0576640Z    |
2019-09-03T01:01:45.0576640Z    |
2019-09-03T01:01:45.0576724Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-09-03T01:01:45.0576819Z error: an inner attribute is not permitted in this context
2019-09-03T01:01:45.0577089Z   --> /checkout/src/test/ui/expr_attr_paren_order.rs:17:9
2019-09-03T01:01:45.0577136Z    |
2019-09-03T01:01:45.0577136Z    |
2019-09-03T01:01:45.0577180Z LL |         #![deny(non_snake_case)]
2019-09-03T01:01:45.0577285Z    |
2019-09-03T01:01:45.0577285Z    |
2019-09-03T01:01:45.0577357Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-09-03T01:01:45.0577468Z error: aborting due to 2 previous errors
2019-09-03T01:01:45.0577497Z 
2019-09-03T01:01:45.0577523Z 
2019-09-03T01:01:45.0577758Z ------------------------------------------
---
2019-09-03T01:01:45.0579132Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-03T01:01:45.0579191Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-03T01:01:45.0579246Z 
2019-09-03T01:01:45.0579272Z 
2019-09-03T01:01:45.0580862Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-03T01:01:45.0581199Z 
2019-09-03T01:01:45.0581227Z 
2019-09-03T01:01:45.0588139Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-03T01:01:45.0588266Z Build completed unsuccessfully in 1:09:47
2019-09-03T01:01:45.0588266Z Build completed unsuccessfully in 1:09:47
2019-09-03T01:01:45.0632187Z == clock drift check ==
2019-09-03T01:01:45.0648740Z   local time: Tue Sep  3 01:01:45 UTC 2019
2019-09-03T01:01:45.2157573Z   network time: Tue, 03 Sep 2019 01:01:45 GMT
2019-09-03T01:01:45.2217008Z == end clock drift check ==
2019-09-03T01:01:46.0086208Z ##[error]Bash exited with code '1'.
2019-09-03T01:01:46.0125779Z ##[section]Starting: Checkout
2019-09-03T01:01:46.0127638Z ==============================================================================
2019-09-03T01:01:46.0127695Z Task         : Get sources
2019-09-03T01:01:46.0127742Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
