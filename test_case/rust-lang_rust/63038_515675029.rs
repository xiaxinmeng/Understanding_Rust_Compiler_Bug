plain
2019-07-27T09:49:57.4911973Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-27T09:49:58.1961114Z ##[command]git config gc.auto 0
2019-07-27T09:49:58.1964234Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-27T09:49:58.1966983Z ##[command]git config --get-all http.proxy
2019-07-27T09:49:58.1970172Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63038/merge:refs/remotes/pull/63038/merge
---
2019-07-27T09:50:31.1391152Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-27T09:50:31.1392400Z 
2019-07-27T09:50:31.1393922Z   git checkout -b <new-branch-name>
2019-07-27T09:50:31.1395775Z 
2019-07-27T09:50:31.1396559Z HEAD is now at 2d6e7efd9 Merge 8b3f28cf0f0cd30ddba884b10c395391ff22beb0 into 09e39897587dca70f0b15093d425a682c392349c
2019-07-27T09:50:31.1526720Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-27T09:50:31.1529399Z ==============================================================================
2019-07-27T09:50:31.1529449Z Task         : Bash
2019-07-27T09:50:31.1529489Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-27T10:47:15.0836029Z .................................................................................................... 700/5870
2019-07-27T10:47:18.9183286Z .................................................................................................... 800/5870
2019-07-27T10:47:24.0529315Z .................................................................................................... 900/5870
2019-07-27T10:47:28.7197457Z .................................................................................................... 1000/5870
2019-07-27T10:47:33.7413490Z i...........i....................................................................................... 1100/5870
2019-07-27T10:47:37.3948141Z ..............................iiiii................................................................. 1200/5870
2019-07-27T10:47:42.9578536Z .................................................................................................... 1400/5870
2019-07-27T10:47:45.4853697Z .................................................................................................... 1500/5870
2019-07-27T10:47:48.9289779Z .................................................................................................... 1600/5870
2019-07-27T10:47:51.3853810Z .................................................................................................... 1700/5870
---
2019-07-27T10:49:00.4230422Z .................................................................................................... 3400/5870
2019-07-27T10:49:05.1737541Z .................................................................................................... 3500/5870
2019-07-27T10:49:08.8742652Z ..........................i......................................................................... 3600/5870
2019-07-27T10:49:12.8991835Z .................................................................................................... 3700/5870
2019-07-27T10:49:16.2160077Z ....ii...i..ii...................................................................................... 3800/5870
2019-07-27T10:49:24.3537215Z .................................................................................................... 4000/5870
2019-07-27T10:49:27.9128197Z .......................ii........................................................................... 4100/5870
2019-07-27T10:49:30.0903909Z .......................F....................i...................................F................... 4200/5870
2019-07-27T10:49:32.0408780Z .................................................................................................... 4300/5870
---
2019-07-27T10:50:48.5026597Z 1 error: an inner attribute is not permitted in this context
2019-07-27T10:50:48.5026901Z -   --> $DIR/issue-45296.rs:4:7
2019-07-27T10:50:48.5027229Z +   --> $DIR/issue-45296.rs:4:5
2019-07-27T10:50:48.5027368Z 3    |
2019-07-27T10:50:48.5027478Z 4 LL |     #![allow(unused_variables)]
2019-07-27T10:50:48.5027888Z +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-07-27T10:50:48.5027997Z 6    |
2019-07-27T10:50:48.5027997Z 6    |
2019-07-27T10:50:48.5028140Z 7    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-07-27T10:50:48.5028573Z 
2019-07-27T10:50:48.5028664Z 
2019-07-27T10:50:48.5028771Z The actual stderr differed from the expected stderr.
2019-07-27T10:50:48.5029173Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45296/issue-45296.stderr
2019-07-27T10:50:48.5029173Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45296/issue-45296.stderr
2019-07-27T10:50:48.5029521Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T10:50:48.5030278Z To only update this specific test, also pass `--test-args issues/issue-45296.rs`
2019-07-27T10:50:48.5030659Z error: 1 errors occurred comparing output.
2019-07-27T10:50:48.5030800Z status: exit code: 1
2019-07-27T10:50:48.5030800Z status: exit code: 1
2019-07-27T10:50:48.5031679Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45296.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45296" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45296/auxiliary" "-A" "unused"
2019-07-27T10:50:48.5032418Z ------------------------------------------
2019-07-27T10:50:48.5032616Z 
2019-07-27T10:50:48.5032996Z ------------------------------------------
2019-07-27T10:50:48.5033193Z stderr:
2019-07-27T10:50:48.5033193Z stderr:
2019-07-27T10:50:48.5033632Z ------------------------------------------
2019-07-27T10:50:48.5033776Z error: an inner attribute is not permitted in this context
2019-07-27T10:50:48.5034093Z   --> /checkout/src/test/ui/issues/issue-45296.rs:4:5
2019-07-27T10:50:48.5034236Z    |
2019-07-27T10:50:48.5035822Z LL |     #![allow(unused_variables)] //~ ERROR not permitted in this context
2019-07-27T10:50:48.5036123Z    |
2019-07-27T10:50:48.5036123Z    |
2019-07-27T10:50:48.5036264Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-07-27T10:50:48.5036505Z error: aborting due to previous error
2019-07-27T10:50:48.5036623Z 
2019-07-27T10:50:48.5036718Z 
2019-07-27T10:50:48.5037078Z ------------------------------------------
---
2019-07-27T10:50:48.5038557Z 1 error: an inner attribute is not permitted in this context
2019-07-27T10:50:48.5038932Z -   --> $DIR/attr.rs:5:3
2019-07-27T10:50:48.5039311Z +   --> $DIR/attr.rs:5:1
2019-07-27T10:50:48.5039483Z 3    |
2019-07-27T10:50:48.5039621Z 4 LL | #![lang = "foo"]
2019-07-27T10:50:48.5039950Z -    |   ^
2019-07-27T10:50:48.5040258Z 6    |
2019-07-27T10:50:48.5040258Z 6    |
2019-07-27T10:50:48.5040461Z 7    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-07-27T10:50:48.5040762Z 
2019-07-27T10:50:48.5040887Z 
2019-07-27T10:50:48.5041028Z The actual stderr differed from the expected stderr.
2019-07-27T10:50:48.5041453Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr/attr.stderr
2019-07-27T10:50:48.5041453Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr/attr.stderr
2019-07-27T10:50:48.5042162Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T10:50:48.5042490Z To only update this specific test, also pass `--test-args parser/attr.rs`
2019-07-27T10:50:48.5042743Z error: 1 errors occurred comparing output.
2019-07-27T10:50:48.5042852Z status: exit code: 1
2019-07-27T10:50:48.5042852Z status: exit code: 1
2019-07-27T10:50:48.5043514Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/attr/auxiliary" "-A" "unused"
2019-07-27T10:50:48.5051122Z ------------------------------------------
2019-07-27T10:50:48.5051344Z 
2019-07-27T10:50:48.5051714Z ------------------------------------------
2019-07-27T10:50:48.5051920Z stderr:
2019-07-27T10:50:48.5051920Z stderr:
2019-07-27T10:50:48.5052264Z ------------------------------------------
2019-07-27T10:50:48.5052440Z error: an inner attribute is not permitted in this context
2019-07-27T10:50:48.5052971Z   --> /checkout/src/test/ui/parser/attr.rs:5:1
2019-07-27T10:50:48.5053131Z    |
2019-07-27T10:50:48.5053281Z LL | #![lang = "foo"] //~ ERROR an inner attribute is not permitted in this context
2019-07-27T10:50:48.5053553Z    |
2019-07-27T10:50:48.5053553Z    |
2019-07-27T10:50:48.5054007Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-07-27T10:50:48.5054330Z 
2019-07-27T10:50:48.5054469Z error[E0522]: definition of an unknown language item: `foo`
2019-07-27T10:50:48.5055012Z    |
2019-07-27T10:50:48.5055012Z    |
2019-07-27T10:50:48.5055322Z LL | #![lang = "foo"] //~ ERROR an inner attribute is not permitted in this context
2019-07-27T10:50:48.5055437Z    | ^^^^^^^^^^^^^^^^ definition of unknown language item `foo`
2019-07-27T10:50:48.5055658Z error: aborting due to 2 previous errors
2019-07-27T10:50:48.5055751Z 
2019-07-27T10:50:48.5056051Z For more information about this error, try `rustc --explain E0522`.
2019-07-27T10:50:48.5056189Z 
2019-07-27T10:50:48.5056189Z 
2019-07-27T10:50:48.5056485Z ------------------------------------------
2019-07-27T10:50:48.5056607Z 
2019-07-27T10:50:48.5056699Z 
2019-07-27T10:50:48.5057000Z ---- [ui] ui/parser/inner-attr-after-doc-comment.rs stdout ----
2019-07-27T10:50:48.5057147Z diff of stderr:
2019-07-27T10:50:48.5057246Z 
2019-07-27T10:50:48.5057373Z 1 error: an inner attribute is not permitted following an outer doc comment
2019-07-27T10:50:48.5058404Z +   --> $DIR/inner-attr-after-doc-comment.rs:6:1
2019-07-27T10:50:48.5058638Z 3    |
2019-07-27T10:50:48.5058638Z 3    |
2019-07-27T10:50:48.5058985Z - LL | #![recursion_limit="100"]
2019-07-27T10:50:48.5059324Z -    |   ^
2019-07-27T10:50:48.5059518Z + LL | / /**
2019-07-27T10:50:48.5059659Z + LL | |  * My module
2019-07-27T10:50:48.5059792Z + LL | |  */
2019-07-27T10:50:48.5060145Z +    | |___- previous outer attribute
2019-07-27T10:50:48.5060315Z + LL | 
2019-07-27T10:50:48.5060455Z + LL |   #![recursion_limit="100"]
2019-07-27T10:50:48.5060642Z +    |   ^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-07-27T10:50:48.5060784Z 6    |
2019-07-27T10:50:48.5060972Z 7    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-07-27T10:50:48.5061251Z 
2019-07-27T10:50:48.5061366Z 
2019-07-27T10:50:48.5061841Z The actual stderr differed from the expected stderr.
2019-07-27T10:50:48.5062437Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr-after-doc-comment/inner-attr-after-doc-comment.stderr
2019-07-27T10:50:48.5062437Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr-after-doc-comment/inner-attr-after-doc-comment.stderr
2019-07-27T10:50:48.5063307Z To update references, rerun the tests and pass the `--bless` flag
2019-07-27T10:50:48.5063888Z To only update this specific test, also pass `--test-args parser/inner-attr-after-doc-comment.rs`
2019-07-27T10:50:48.5064163Z error: 1 errors occurred comparing output.
2019-07-27T10:50:48.5064432Z status: exit code: 1
2019-07-27T10:50:48.5064432Z status: exit code: 1
2019-07-27T10:50:48.5065229Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/inner-attr-after-doc-comment.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr-after-doc-comment" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/inner-attr-after-doc-comment/auxiliary" "-A" "unused"
2019-07-27T10:50:48.5065752Z ------------------------------------------
2019-07-27T10:50:48.5065910Z 
2019-07-27T10:50:48.5066224Z ------------------------------------------
2019-07-27T10:50:48.5066526Z stderr:
2019-07-27T10:50:48.5066526Z stderr:
2019-07-27T10:50:48.5066979Z ------------------------------------------
2019-07-27T10:50:48.5067140Z error: an inner attribute is not permitted following an outer doc comment
2019-07-27T10:50:48.5067615Z    |
2019-07-27T10:50:48.5067808Z LL | / /**
2019-07-27T10:50:48.5067808Z LL | / /**
2019-07-27T10:50:48.5068297Z LL | |  * My module
2019-07-27T10:50:48.5068475Z LL | |  */
2019-07-27T10:50:48.5068860Z    | |___- previous outer attribute
2019-07-27T10:50:48.5069054Z LL | 
2019-07-27T10:50:48.5069198Z LL |   #![recursion_limit="100"]
2019-07-27T10:50:48.5069341Z    |   ^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attibute
2019-07-27T10:50:48.5069500Z    |
2019-07-27T10:50:48.5069659Z    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
2019-07-27T10:50:48.5069961Z error: aborting due to previous error
2019-07-27T10:50:48.5070101Z 
2019-07-27T10:50:48.5070216Z 
2019-07-27T10:50:48.5070583Z ------------------------------------------
---
2019-07-27T10:50:48.5073744Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:535:22
2019-07-27T10:50:48.5073796Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-27T10:50:48.5073824Z 
2019-07-27T10:50:48.5073864Z 
2019-07-27T10:50:48.5075050Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-27T10:50:48.5075400Z 
2019-07-27T10:50:48.5075426Z 
2019-07-27T10:50:48.5075466Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-27T10:50:48.5075524Z Build completed unsuccessfully in 0:53:18
2019-07-27T10:50:48.5075524Z Build completed unsuccessfully in 0:53:18
2019-07-27T10:50:49.5645802Z ##[error]Bash exited with code '1'.
2019-07-27T10:50:49.5702704Z ##[section]Starting: Checkout
2019-07-27T10:50:49.5704380Z ==============================================================================
2019-07-27T10:50:49.5704452Z Task         : Get sources
2019-07-27T10:50:49.5704493Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
