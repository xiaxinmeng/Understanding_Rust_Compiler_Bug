plain
2019-09-24T06:39:05.0680315Z 
2019-09-24T06:39:05.0680518Z - error[E0507]: cannot move out of static item `D`
2019-09-24T06:39:05.0680730Z -   --> $DIR/move-error-snippets.rs:16:18
2019-09-24T06:39:05.0680909Z -    |
2019-09-24T06:39:05.0681075Z - LL | | #[macro_use]
2019-09-24T06:39:05.0681519Z -    | |__________________^ move occurs because `D` has type `A`, which does not implement the `Copy` trait
2019-09-24T06:39:05.0681779Z - ...
2019-09-24T06:39:05.0681977Z - LL |               aaa!(D);
2019-09-24T06:39:05.0682513Z - ...
2019-09-24T06:39:05.0682513Z - ...
2019-09-24T06:39:05.0682679Z - LL |   sss!();
2019-09-24T06:39:05.0683190Z + error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0683262Z 12 
2019-09-24T06:39:05.0683338Z 13 error: aborting due to previous error
2019-09-24T06:39:05.0683403Z 14 
2019-09-24T06:39:05.0683403Z 14 
2019-09-24T06:39:05.0683454Z 
2019-09-24T06:39:05.0683707Z - For more information about this error, try `rustc --explain E0507`.
2019-09-24T06:39:05.0683797Z 16 
2019-09-24T06:39:05.0683831Z 
2019-09-24T06:39:05.0683863Z 
2019-09-24T06:39:05.0683944Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0684283Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets/move-error-snippets.stderr
2019-09-24T06:39:05.0684625Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0684957Z To only update this specific test, also pass `--test-args borrowck/move-error-snippets.rs`
2019-09-24T06:39:05.0685085Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0685171Z status: exit code: 1
2019-09-24T06:39:05.0685171Z status: exit code: 1
2019-09-24T06:39:05.0686339Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/move-error-snippets.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0687504Z ------------------------------------------
2019-09-24T06:39:05.0687560Z 
2019-09-24T06:39:05.0687820Z ------------------------------------------
2019-09-24T06:39:05.0687912Z stderr:
---
2019-09-24T06:39:05.0689109Z ---- [ui] ui/custom-test-frameworks-simple.rs stdout ----
2019-09-24T06:39:05.0689165Z 
2019-09-24T06:39:05.0689432Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0689513Z status: exit code: 1
2019-09-24T06:39:05.0690574Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom-test-frameworks-simple.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-test-frameworks-simple/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom-test-frameworks-simple/auxiliary"
2019-09-24T06:39:05.0691019Z ------------------------------------------
2019-09-24T06:39:05.0691060Z 
2019-09-24T06:39:05.0691235Z ------------------------------------------
2019-09-24T06:39:05.0691304Z stderr:
---
2019-09-24T06:39:05.0692193Z ---- [ui] ui/custom_test_frameworks/full.rs stdout ----
2019-09-24T06:39:05.0692249Z 
2019-09-24T06:39:05.0692442Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0692516Z status: exit code: 1
2019-09-24T06:39:05.0693814Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom_test_frameworks/full.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/full/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/full/auxiliary"
2019-09-24T06:39:05.0694303Z ------------------------------------------
2019-09-24T06:39:05.0694350Z 
2019-09-24T06:39:05.0694576Z ------------------------------------------
2019-09-24T06:39:05.0694642Z stderr:
---
2019-09-24T06:39:05.0695726Z ---- [ui] ui/custom_test_frameworks/dynamic.rs stdout ----
2019-09-24T06:39:05.0695777Z 
2019-09-24T06:39:05.0696373Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0696905Z status: exit code: 1
2019-09-24T06:39:05.0697855Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom_test_frameworks/dynamic.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/dynamic/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/dynamic/auxiliary"
2019-09-24T06:39:05.0698520Z ------------------------------------------
2019-09-24T06:39:05.0698588Z 
2019-09-24T06:39:05.0698825Z ------------------------------------------
2019-09-24T06:39:05.0698917Z stderr:
---
2019-09-24T06:39:05.0700546Z diff of stderr:
2019-09-24T06:39:05.0700587Z 
2019-09-24T06:39:05.0700660Z + error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0700720Z + 
2019-09-24T06:39:05.0700799Z 1 error[E0277]: the trait bound `test::TestDescAndFn: example_runner::Testable` is not satisfied
2019-09-24T06:39:05.0701098Z 3    |
2019-09-24T06:39:05.0701126Z 
2019-09-24T06:39:05.0701189Z 6    |
2019-09-24T06:39:05.0701250Z 7    = note: required for the cast to the object type `dyn example_runner::Testable`
---
2019-09-24T06:39:05.0701956Z 12 
2019-09-24T06:39:05.0701983Z 
2019-09-24T06:39:05.0702010Z 
2019-09-24T06:39:05.0702087Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0702364Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch/mismatch.stderr
2019-09-24T06:39:05.0702845Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0703367Z To only update this specific test, also pass `--test-args custom_test_frameworks/mismatch.rs`
2019-09-24T06:39:05.0703482Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0703563Z status: exit code: 1
2019-09-24T06:39:05.0703563Z status: exit code: 1
2019-09-24T06:39:05.0704371Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom_test_frameworks/mismatch.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0704841Z ------------------------------------------
2019-09-24T06:39:05.0704902Z 
2019-09-24T06:39:05.0705107Z ------------------------------------------
2019-09-24T06:39:05.0705187Z stderr:
2019-09-24T06:39:05.0705187Z stderr:
2019-09-24T06:39:05.0705388Z ------------------------------------------
2019-09-24T06:39:05.0705480Z error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0705528Z 
2019-09-24T06:39:05.0705624Z error[E0277]: the trait bound `test::TestDescAndFn: example_runner::Testable` is not satisfied
2019-09-24T06:39:05.0706544Z    |
2019-09-24T06:39:05.0706544Z    |
2019-09-24T06:39:05.0706768Z LL | fn wrong_kind(){}
2019-09-24T06:39:05.0706892Z    | ^^^^^^^^^^^^^^^^^ the trait `example_runner::Testable` is not implemented for `test::TestDescAndFn`
2019-09-24T06:39:05.0707081Z    = note: required for the cast to the object type `dyn example_runner::Testable`
2019-09-24T06:39:05.0707159Z 
2019-09-24T06:39:05.0707224Z error: aborting due to 2 previous errors
2019-09-24T06:39:05.0707271Z 
---
2019-09-24T06:39:05.0708017Z 
2019-09-24T06:39:05.0708287Z ---- [ui] ui/issues/issue-12997-1.rs stdout ----
2019-09-24T06:39:05.0708381Z diff of stderr:
2019-09-24T06:39:05.0708425Z 
2019-09-24T06:39:05.0708488Z 10 LL | fn bar(x: isize, y: isize) { }
2019-09-24T06:39:05.0708658Z 12 
2019-09-24T06:39:05.0708917Z - error: aborting due to 2 previous errors
2019-09-24T06:39:05.0709110Z + error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0709201Z + 
2019-09-24T06:39:05.0709201Z + 
2019-09-24T06:39:05.0709281Z + error: aborting due to 3 previous errors
2019-09-24T06:39:05.0709353Z 14 
2019-09-24T06:39:05.0709429Z 15 
2019-09-24T06:39:05.0709466Z 
2019-09-24T06:39:05.0709502Z 
2019-09-24T06:39:05.0709590Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0709978Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1/issue-12997-1.stderr
2019-09-24T06:39:05.0710423Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0710841Z To only update this specific test, also pass `--test-args issues/issue-12997-1.rs`
2019-09-24T06:39:05.0711114Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0711169Z status: exit code: 1
2019-09-24T06:39:05.0711169Z status: exit code: 1
2019-09-24T06:39:05.0712002Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12997-1.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-1/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0712382Z ------------------------------------------
2019-09-24T06:39:05.0712435Z 
2019-09-24T06:39:05.0712608Z ------------------------------------------
2019-09-24T06:39:05.0712879Z stderr:
2019-09-24T06:39:05.0712879Z stderr:
2019-09-24T06:39:05.0713082Z ------------------------------------------
2019-09-24T06:39:05.0713365Z error: functions used as benches must have signature `fn(&mut Bencher) -> impl Termination`
2019-09-24T06:39:05.0713625Z   --> /checkout/src/test/ui/issues/issue-12997-1.rs:8:1
2019-09-24T06:39:05.0713713Z    |
2019-09-24T06:39:05.0713791Z LL | fn foo() { } //~ ERROR functions used as benches
2019-09-24T06:39:05.0713902Z 
2019-09-24T06:39:05.0713902Z 
2019-09-24T06:39:05.0714176Z error: functions used as benches must have signature `fn(&mut Bencher) -> impl Termination`
2019-09-24T06:39:05.0714443Z   --> /checkout/src/test/ui/issues/issue-12997-1.rs:11:1
2019-09-24T06:39:05.0714512Z    |
2019-09-24T06:39:05.0714594Z LL | fn bar(x: isize, y: isize) { } //~ ERROR functions used as benches
2019-09-24T06:39:05.0714726Z 
2019-09-24T06:39:05.0714787Z error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0714835Z 
2019-09-24T06:39:05.0714906Z error: aborting due to 3 previous errors
---
2019-09-24T06:39:05.0717998Z 13 
2019-09-24T06:39:05.0718044Z 
2019-09-24T06:39:05.0718080Z 
2019-09-24T06:39:05.0718170Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0718620Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-2/issue-12997-2.stderr
2019-09-24T06:39:05.0718991Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0719321Z To only update this specific test, also pass `--test-args issues/issue-12997-2.rs`
2019-09-24T06:39:05.0719455Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0719547Z status: exit code: 1
2019-09-24T06:39:05.0719547Z status: exit code: 1
2019-09-24T06:39:05.0720589Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12997-2.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-2/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0721040Z ------------------------------------------
2019-09-24T06:39:05.0721099Z 
2019-09-24T06:39:05.0730596Z ------------------------------------------
2019-09-24T06:39:05.0730712Z stderr:
2019-09-24T06:39:05.0730712Z stderr:
2019-09-24T06:39:05.0730922Z ------------------------------------------
2019-09-24T06:39:05.0731007Z error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0731056Z 
2019-09-24T06:39:05.0731122Z error[E0308]: mismatched types
2019-09-24T06:39:05.0731335Z   --> /checkout/src/test/ui/issues/issue-12997-2.rs:8:1
2019-09-24T06:39:05.0731409Z    |
2019-09-24T06:39:05.0731455Z LL | fn bar(x: isize) { }
2019-09-24T06:39:05.0731531Z    | ^^^^^^^^^^^^^^^^^^^^ expected isize, found mutable reference
2019-09-24T06:39:05.0731652Z    = note: expected type `isize`
2019-09-24T06:39:05.0731740Z               found type `&mut test::Bencher`
2019-09-24T06:39:05.0731776Z 
2019-09-24T06:39:05.0731822Z error: aborting due to 2 previous errors
---
2019-09-24T06:39:05.0732586Z 
2019-09-24T06:39:05.0733254Z ---- [ui] ui/issues/issue-14772.rs stdout ----
2019-09-24T06:39:05.0733338Z diff of stderr:
2019-09-24T06:39:05.0733379Z 
2019-09-24T06:39:05.0733453Z 4 LL | mod foo {}
2019-09-24T06:39:05.0733592Z 6 
2019-09-24T06:39:05.0733806Z - error: aborting due to previous error
2019-09-24T06:39:05.0733901Z + error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0733971Z + 
2019-09-24T06:39:05.0733971Z + 
2019-09-24T06:39:05.0734044Z + error: aborting due to 2 previous errors
2019-09-24T06:39:05.0734296Z 8 
2019-09-24T06:39:05.0734350Z 9 
2019-09-24T06:39:05.0734382Z 
2019-09-24T06:39:05.0734431Z 
2019-09-24T06:39:05.0734504Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0767215Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772/issue-14772.stderr
2019-09-24T06:39:05.0767785Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0768135Z To only update this specific test, also pass `--test-args issues/issue-14772.rs`
2019-09-24T06:39:05.0768282Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0768356Z status: exit code: 1
2019-09-24T06:39:05.0768356Z status: exit code: 1
2019-09-24T06:39:05.0769396Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-14772.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-14772/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0769981Z ------------------------------------------
2019-09-24T06:39:05.0770036Z 
2019-09-24T06:39:05.0770427Z ------------------------------------------
2019-09-24T06:39:05.0770645Z stderr:
2019-09-24T06:39:05.0770645Z stderr:
2019-09-24T06:39:05.0770831Z ------------------------------------------
2019-09-24T06:39:05.0770893Z error: only functions may be used as tests
2019-09-24T06:39:05.0771107Z   --> /checkout/src/test/ui/issues/issue-14772.rs:4:1
2019-09-24T06:39:05.0771168Z    |
2019-09-24T06:39:05.0771228Z LL | mod foo {} //~ ERROR only functions may be used as tests
2019-09-24T06:39:05.0771330Z 
2019-09-24T06:39:05.0771386Z error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0771440Z 
2019-09-24T06:39:05.0771490Z error: aborting due to 2 previous errors
---
2019-09-24T06:39:05.0772027Z ---- [ui] ui/issues/issue-16597-empty.rs stdout ----
2019-09-24T06:39:05.0772081Z 
2019-09-24T06:39:05.0772274Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0772344Z status: exit code: 1
2019-09-24T06:39:05.0773531Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16597-empty.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597-empty/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597-empty/auxiliary"
2019-09-24T06:39:05.0773991Z ------------------------------------------
2019-09-24T06:39:05.0774045Z 
2019-09-24T06:39:05.0774265Z ------------------------------------------
2019-09-24T06:39:05.0774331Z stderr:
---
2019-09-24T06:39:05.0775547Z ---- [ui] ui/issues/issue-16597.rs stdout ----
2019-09-24T06:39:05.0775595Z 
2019-09-24T06:39:05.0775833Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0775908Z status: exit code: 1
2019-09-24T06:39:05.0777544Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16597.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597/auxiliary"
2019-09-24T06:39:05.0778052Z ------------------------------------------
2019-09-24T06:39:05.0778111Z 
2019-09-24T06:39:05.0778348Z ------------------------------------------
2019-09-24T06:39:05.0778431Z stderr:
---
2019-09-24T06:39:05.0779695Z ---- [ui] ui/issues/issue-20823.rs stdout ----
2019-09-24T06:39:05.0779749Z 
2019-09-24T06:39:05.0780009Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0780247Z status: exit code: 1
2019-09-24T06:39:05.0781076Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20823.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20823/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20823/auxiliary"
2019-09-24T06:39:05.0781501Z ------------------------------------------
2019-09-24T06:39:05.0781543Z 
2019-09-24T06:39:05.0781746Z ------------------------------------------
2019-09-24T06:39:05.0781812Z stderr:
---
2019-09-24T06:39:05.0783767Z 2   --> $DIR/issue-28134.rs:3:4
2019-09-24T06:39:05.0783834Z 3    |
2019-09-24T06:39:05.0783867Z 
2019-09-24T06:39:05.0784122Z 6    |
2019-09-24T06:39:05.0784218Z 7    = note: import resolution is stuck, try simplifying macro imports
2019-09-24T06:39:05.0784562Z - error: aborting due to previous error
2019-09-24T06:39:05.0784636Z + error: aborting due to 2 previous errors
2019-09-24T06:39:05.0784712Z 10 
2019-09-24T06:39:05.0784767Z 11 
2019-09-24T06:39:05.0784767Z 11 
2019-09-24T06:39:05.0784810Z 
2019-09-24T06:39:05.0784842Z 
2019-09-24T06:39:05.0784906Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0785225Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134/issue-28134.stderr
2019-09-24T06:39:05.0785702Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0786165Z To only update this specific test, also pass `--test-args issues/issue-28134.rs`
2019-09-24T06:39:05.0786986Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0787071Z status: exit code: 1
2019-09-24T06:39:05.0787071Z status: exit code: 1
2019-09-24T06:39:05.0790462Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28134.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28134/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0791143Z ------------------------------------------
2019-09-24T06:39:05.0791189Z 
2019-09-24T06:39:05.0791386Z ------------------------------------------
2019-09-24T06:39:05.0791447Z stderr:
2019-09-24T06:39:05.0791447Z stderr:
2019-09-24T06:39:05.0791636Z ------------------------------------------
2019-09-24T06:39:05.0791716Z error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0791768Z 
2019-09-24T06:39:05.0791952Z error: cannot determine resolution for the attribute macro `test`
2019-09-24T06:39:05.0792278Z   --> /checkout/src/test/ui/issues/issue-28134.rs:3:4
2019-09-24T06:39:05.0792343Z    |
2019-09-24T06:39:05.0792414Z LL | #![test] //~ ERROR cannot determine resolution for the attribute macro `test`
2019-09-24T06:39:05.0792545Z    |
2019-09-24T06:39:05.0792545Z    |
2019-09-24T06:39:05.0792887Z    = note: import resolution is stuck, try simplifying macro imports
2019-09-24T06:39:05.0793191Z error: aborting due to 2 previous errors
2019-09-24T06:39:05.0793236Z 
2019-09-24T06:39:05.0793269Z 
2019-09-24T06:39:05.0793502Z ------------------------------------------
2019-09-24T06:39:05.0793502Z ------------------------------------------
2019-09-24T06:39:05.0793550Z 
2019-09-24T06:39:05.0793584Z 
2019-09-24T06:39:05.0793824Z ---- [ui] ui/issues/issue-34932.rs stdout ----
2019-09-24T06:39:05.0793873Z 
2019-09-24T06:39:05.0794123Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0794197Z status: exit code: 1
2019-09-24T06:39:05.0794991Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34932.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34932/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34932/auxiliary"
2019-09-24T06:39:05.0795454Z ------------------------------------------
2019-09-24T06:39:05.0795514Z 
2019-09-24T06:39:05.0797083Z ------------------------------------------
2019-09-24T06:39:05.0797181Z stderr:
---
2019-09-24T06:39:05.0799348Z ---- [ui] ui/issues/issue-36768.rs stdout ----
2019-09-24T06:39:05.0799407Z 
2019-09-24T06:39:05.0799656Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0799746Z status: exit code: 1
2019-09-24T06:39:05.0800656Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-36768.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36768/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-36768/auxiliary"
2019-09-24T06:39:05.0801253Z ------------------------------------------
2019-09-24T06:39:05.0801403Z 
2019-09-24T06:39:05.0801634Z ------------------------------------------
2019-09-24T06:39:05.0801695Z stderr:
---
2019-09-24T06:39:05.0802634Z ---- [ui] ui/issues/issue-45731.rs stdout ----
2019-09-24T06:39:05.0802677Z 
2019-09-24T06:39:05.0803134Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0803231Z status: exit code: 1
2019-09-24T06:39:05.0804117Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-45731.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45731/a.wasm" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-g" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-45731/auxiliary"
2019-09-24T06:39:05.0804861Z ------------------------------------------
2019-09-24T06:39:05.0804920Z 
2019-09-24T06:39:05.0818231Z ------------------------------------------
2019-09-24T06:39:05.0818358Z stderr:
---
2019-09-24T06:39:05.0819849Z ---- [ui] ui/issues/issue-46519.rs stdout ----
2019-09-24T06:39:05.0819900Z 
2019-09-24T06:39:05.0820306Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0820374Z status: exit code: 1
2019-09-24T06:39:05.0821098Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46519.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46519/a.wasm" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46519/auxiliary"
2019-09-24T06:39:05.0821538Z ------------------------------------------
2019-09-24T06:39:05.0821582Z 
2019-09-24T06:39:05.0821790Z ------------------------------------------
2019-09-24T06:39:05.0821867Z stderr:
---
2019-09-24T06:39:05.0823012Z ---- [ui] ui/issues/issue-52557.rs stdout ----
2019-09-24T06:39:05.0823065Z 
2019-09-24T06:39:05.0823289Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0823542Z status: exit code: 1
2019-09-24T06:39:05.0824333Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52557.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52557/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52557/auxiliary"
2019-09-24T06:39:05.0825153Z ------------------------------------------
2019-09-24T06:39:05.0825201Z 
2019-09-24T06:39:05.0825594Z ------------------------------------------
2019-09-24T06:39:05.0825662Z stderr:
---
2019-09-24T06:39:05.0828690Z ---- [ui] ui/issues/issue-53675-a-test-called-panic.rs stdout ----
2019-09-24T06:39:05.0828781Z 
2019-09-24T06:39:05.0829093Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0829178Z status: exit code: 1
2019-09-24T06:39:05.0830118Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-53675-a-test-called-panic.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53675-a-test-called-panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-53675-a-test-called-panic/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0830761Z ------------------------------------------
2019-09-24T06:39:05.0830816Z 
2019-09-24T06:39:05.0831033Z ------------------------------------------
2019-09-24T06:39:05.0831110Z stderr:
---
2019-09-24T06:39:05.0832196Z 
2019-09-24T06:39:05.0832402Z - error: cannot test inner items
2019-09-24T06:39:05.0832612Z -   --> $DIR/test-inner-fn.rs:5:5
2019-09-24T06:39:05.0832806Z -    |
2019-09-24T06:39:05.0833195Z - LL |     #[test]
2019-09-24T06:39:05.0833581Z -    |
2019-09-24T06:39:05.0833850Z -    = note: requested on the command line with `-D unnameable-test-items`
2019-09-24T06:39:05.0833941Z + error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0834019Z 8 
2019-09-24T06:39:05.0834019Z 8 
2019-09-24T06:39:05.0834221Z - error: cannot test inner items
2019-09-24T06:39:05.0834606Z -   --> $DIR/test-inner-fn.rs:13:9
2019-09-24T06:39:05.0834794Z -    |
2019-09-24T06:39:05.0834979Z - LL |         #[test]
2019-09-24T06:39:05.0835352Z - 
2019-09-24T06:39:05.0835561Z - error: aborting due to 2 previous errors
2019-09-24T06:39:05.0835629Z + error: aborting due to previous error
2019-09-24T06:39:05.0835699Z 16 
2019-09-24T06:39:05.0835699Z 16 
2019-09-24T06:39:05.0835751Z 17 
2019-09-24T06:39:05.0835788Z 
2019-09-24T06:39:05.0835819Z 
2019-09-24T06:39:05.0835887Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0836342Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn/test-inner-fn.stderr
2019-09-24T06:39:05.0837047Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0837370Z To only update this specific test, also pass `--test-args lint/test-inner-fn.rs`
2019-09-24T06:39:05.0837509Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0837589Z status: exit code: 1
2019-09-24T06:39:05.0837589Z status: exit code: 1
2019-09-24T06:39:05.0838482Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/test-inner-fn.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-D" "unnameable_test_items" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0839011Z ------------------------------------------
2019-09-24T06:39:05.0839063Z 
2019-09-24T06:39:05.0839404Z ------------------------------------------
2019-09-24T06:39:05.0839494Z stderr:
---
2019-09-24T06:39:05.0840628Z 
2019-09-24T06:39:05.0840659Z 
2019-09-24T06:39:05.0840896Z ---- [ui] ui/macros/macro-comma-behavior-rpass.rs#core stdout ----
2019-09-24T06:39:05.0840946Z 
2019-09-24T06:39:05.0841187Z error in revision `core`: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0841405Z status: exit code: 1
2019-09-24T06:39:05.0842838Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--cfg" "core" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.core/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.core/auxiliary"
2019-09-24T06:39:05.0843839Z ------------------------------------------
2019-09-24T06:39:05.0843907Z 
2019-09-24T06:39:05.0844130Z ------------------------------------------
2019-09-24T06:39:05.0844209Z stderr:
---
2019-09-24T06:39:05.0845010Z 
2019-09-24T06:39:05.0845050Z 
2019-09-24T06:39:05.0845297Z ---- [ui] ui/macros/macro-comma-behavior-rpass.rs#std stdout ----
2019-09-24T06:39:05.0845358Z 
2019-09-24T06:39:05.0845607Z error in revision `std`: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0846305Z status: exit code: 1
2019-09-24T06:39:05.0848204Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--cfg" "std" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.std/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.std/auxiliary"
2019-09-24T06:39:05.0848930Z ------------------------------------------
2019-09-24T06:39:05.0848986Z 
2019-09-24T06:39:05.0849232Z ------------------------------------------
2019-09-24T06:39:05.0849306Z stderr:
---
2019-09-24T06:39:05.0850141Z 
2019-09-24T06:39:05.0850176Z 
2019-09-24T06:39:05.0850660Z ---- [ui] ui/macros/macro-comma-support-rpass.rs#core stdout ----
2019-09-24T06:39:05.0850721Z 
2019-09-24T06:39:05.0850974Z error in revision `core`: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0851053Z status: exit code: 1
2019-09-24T06:39:05.0851985Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--cfg" "core" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/auxiliary"
2019-09-24T06:39:05.0852497Z ------------------------------------------
2019-09-24T06:39:05.0852552Z 
2019-09-24T06:39:05.0852767Z ------------------------------------------
2019-09-24T06:39:05.0852851Z stderr:
---
2019-09-24T06:39:05.0853619Z 
2019-09-24T06:39:05.0853665Z 
2019-09-24T06:39:05.0853900Z ---- [ui] ui/macros/macro-comma-support-rpass.rs#std stdout ----
2019-09-24T06:39:05.0853960Z 
2019-09-24T06:39:05.0854201Z error in revision `std`: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0854291Z status: exit code: 1
2019-09-24T06:39:05.0855147Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--cfg" "std" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/auxiliary"
2019-09-24T06:39:05.0855642Z ------------------------------------------
2019-09-24T06:39:05.0855690Z 
2019-09-24T06:39:05.0856063Z ------------------------------------------
2019-09-24T06:39:05.0856128Z stderr:
---
2019-09-24T06:39:05.0859714Z 10 
2019-09-24T06:39:05.0859752Z 
2019-09-24T06:39:05.0859795Z 
2019-09-24T06:39:05.0859865Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0860435Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test/ambiguous-builtin-attrs-test.stderr
2019-09-24T06:39:05.0860749Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0861045Z To only update this specific test, also pass `--test-args proc-macro/ambiguous-builtin-attrs-test.rs`
2019-09-24T06:39:05.0861178Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0861255Z status: exit code: 1
2019-09-24T06:39:05.0861255Z status: exit code: 1
2019-09-24T06:39:05.0862067Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/ambiguous-builtin-attrs-test.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/ambiguous-builtin-attrs-test/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0862537Z ------------------------------------------
2019-09-24T06:39:05.0862584Z 
2019-09-24T06:39:05.0862794Z ------------------------------------------
2019-09-24T06:39:05.0863045Z stderr:
---
2019-09-24T06:39:05.0865608Z ---- [ui] ui/proc-macro/derive-test.rs stdout ----
2019-09-24T06:39:05.0865668Z 
2019-09-24T06:39:05.0865905Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0865989Z status: exit code: 1
2019-09-24T06:39:05.0867432Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/derive-test.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-test/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-test/auxiliary"
2019-09-24T06:39:05.0868091Z ------------------------------------------
2019-09-24T06:39:05.0868144Z 
2019-09-24T06:39:05.0868392Z ------------------------------------------
2019-09-24T06:39:05.0868467Z stderr:
---
2019-09-24T06:39:05.0869611Z ---- [ui] ui/reexport-test-harness-main.rs stdout ----
2019-09-24T06:39:05.0869666Z 
2019-09-24T06:39:05.0869914Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0870073Z status: exit code: 1
2019-09-24T06:39:05.0871050Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reexport-test-harness-main.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reexport-test-harness-main/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reexport-test-harness-main/auxiliary"
2019-09-24T06:39:05.0871502Z ------------------------------------------
2019-09-24T06:39:05.0871558Z 
2019-09-24T06:39:05.0871768Z ------------------------------------------
2019-09-24T06:39:05.0871855Z stderr:
---
2019-09-24T06:39:05.0874518Z 13 
2019-09-24T06:39:05.0874556Z 
2019-09-24T06:39:05.0874590Z 
2019-09-24T06:39:05.0874670Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0875094Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic/termination-trait-in-test-should-panic.stderr
2019-09-24T06:39:05.0875399Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0875734Z To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-in-test-should-panic.rs`
2019-09-24T06:39:05.0876022Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0876094Z status: exit code: 1
2019-09-24T06:39:05.0876094Z status: exit code: 1
2019-09-24T06:39:05.0877443Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test-should-panic/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0878143Z ------------------------------------------
2019-09-24T06:39:05.0878215Z 
2019-09-24T06:39:05.0878450Z ------------------------------------------
2019-09-24T06:39:05.0878533Z stderr:
2019-09-24T06:39:05.0878533Z stderr:
2019-09-24T06:39:05.0878764Z ------------------------------------------
2019-09-24T06:39:05.0878858Z error: functions using `#[should_panic]` must return `()`
2019-09-24T06:39:05.0879305Z    |
2019-09-24T06:39:05.0879305Z    |
2019-09-24T06:39:05.0879564Z LL | / fn not_a_num() -> Result<(), ParseIntError> {
2019-09-24T06:39:05.0879731Z LL | |     //~^ ERROR functions using `#[should_panic]` must return `()`
2019-09-24T06:39:05.0879836Z LL | |     let _: u32 = "abc".parse()?;
2019-09-24T06:39:05.0879910Z LL | |     Ok(())
2019-09-24T06:39:05.0880483Z    | |_^
2019-09-24T06:39:05.0880524Z 
2019-09-24T06:39:05.0880587Z error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0880645Z 
---
2019-09-24T06:39:05.0881766Z ---- [ui] ui/rfc-1937-termination-trait/termination-trait-in-test.rs stdout ----
2019-09-24T06:39:05.0881843Z 
2019-09-24T06:39:05.0882069Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0882155Z status: exit code: 1
2019-09-24T06:39:05.0883712Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-in-test.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/auxiliary"
2019-09-24T06:39:05.0884399Z ------------------------------------------
2019-09-24T06:39:05.0884449Z 
2019-09-24T06:39:05.0884868Z ------------------------------------------
2019-09-24T06:39:05.0884954Z stderr:
---
2019-09-24T06:39:05.0886351Z diff of stderr:
2019-09-24T06:39:05.0887075Z 
2019-09-24T06:39:05.0887154Z + error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0887243Z + 
2019-09-24T06:39:05.0887337Z 1 error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseFloatError>`
2019-09-24T06:39:05.0887749Z 3    |
2019-09-24T06:39:05.0887910Z 
2019-09-24T06:39:05.0887977Z 13    |
2019-09-24T06:39:05.0887977Z 13    |
2019-09-24T06:39:05.0888084Z 14    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-09-24T06:39:05.0888468Z - error: aborting due to previous error
2019-09-24T06:39:05.0888556Z + error: aborting due to 2 previous errors
2019-09-24T06:39:05.0888626Z 17 
2019-09-24T06:39:05.0888907Z 18 For more information about this error, try `rustc --explain E0277`.
2019-09-24T06:39:05.0888907Z 18 For more information about this error, try `rustc --explain E0277`.
2019-09-24T06:39:05.0888999Z 19 
2019-09-24T06:39:05.0889035Z 
2019-09-24T06:39:05.0889071Z 
2019-09-24T06:39:05.0889151Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0889578Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
2019-09-24T06:39:05.0889911Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0890495Z To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-test-wrong-type.rs`
2019-09-24T06:39:05.0890710Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0890786Z status: exit code: 1
2019-09-24T06:39:05.0890786Z status: exit code: 1
2019-09-24T06:39:05.0891725Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0892224Z ------------------------------------------
2019-09-24T06:39:05.0892283Z 
2019-09-24T06:39:05.0892732Z ------------------------------------------
2019-09-24T06:39:05.0892815Z stderr:
2019-09-24T06:39:05.0892815Z stderr:
2019-09-24T06:39:05.0893080Z ------------------------------------------
2019-09-24T06:39:05.0893173Z error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0893225Z 
2019-09-24T06:39:05.0893319Z error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseFloatError>`
2019-09-24T06:39:05.0894167Z    |
2019-09-24T06:39:05.0894167Z    |
2019-09-24T06:39:05.0894471Z LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> { //~ ERROR
2019-09-24T06:39:05.0894558Z LL | |     "0".parse()
2019-09-24T06:39:05.0894631Z LL | | }
2019-09-24T06:39:05.0894706Z    | |_^ `main` can only return types that implement `std::process::Termination`
2019-09-24T06:39:05.0894867Z   ::: /checkout/src/libtest/lib.rs:375:30
2019-09-24T06:39:05.0895126Z    |
2019-09-24T06:39:05.0895126Z    |
2019-09-24T06:39:05.0895201Z LL |   pub fn assert_test_result<T: Termination>(result: T) {
2019-09-24T06:39:05.0895514Z    |                                ----------- required by this bound in `test::assert_test_result`
2019-09-24T06:39:05.0895765Z    |
2019-09-24T06:39:05.0895853Z    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-09-24T06:39:05.0895990Z error: aborting due to 2 previous errors
2019-09-24T06:39:05.0896040Z 
2019-09-24T06:39:05.0896287Z For more information about this error, try `rustc --explain E0277`.
2019-09-24T06:39:05.0896341Z 
2019-09-24T06:39:05.0896341Z 
2019-09-24T06:39:05.0896941Z ------------------------------------------
2019-09-24T06:39:05.0896996Z 
2019-09-24T06:39:05.0897040Z 
2019-09-24T06:39:05.0897310Z ---- [ui] ui/test-allow-dead-extern-static-no-warning.rs stdout ----
2019-09-24T06:39:05.0897495Z 
2019-09-24T06:39:05.0897774Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0897868Z status: exit code: 1
2019-09-24T06:39:05.0898792Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-allow-dead-extern-static-no-warning.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-allow-dead-extern-static-no-warning/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-allow-dead-extern-static-no-warning/auxiliary"
2019-09-24T06:39:05.0899333Z ------------------------------------------
2019-09-24T06:39:05.0899386Z 
2019-09-24T06:39:05.0899630Z ------------------------------------------
2019-09-24T06:39:05.0899704Z stderr:
---
2019-09-24T06:39:05.0901047Z ---- [ui] ui/test-attrs/decl-macro-test.rs stdout ----
2019-09-24T06:39:05.0901096Z 
2019-09-24T06:39:05.0901316Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0901398Z status: exit code: 1
2019-09-24T06:39:05.0902229Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/decl-macro-test.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/decl-macro-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/decl-macro-test/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0902874Z ------------------------------------------
2019-09-24T06:39:05.0903123Z 
2019-09-24T06:39:05.0903529Z ------------------------------------------
2019-09-24T06:39:05.0903612Z stderr:
---
2019-09-24T06:39:05.0907204Z 22 
2019-09-24T06:39:05.0907250Z 
2019-09-24T06:39:05.0907286Z 
2019-09-24T06:39:05.0907494Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0907913Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/inaccessible-test-modules/inaccessible-test-modules.stderr
2019-09-24T06:39:05.0908238Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0908562Z To only update this specific test, also pass `--test-args test-attrs/inaccessible-test-modules.rs`
2019-09-24T06:39:05.0908707Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0908788Z status: exit code: 1
2019-09-24T06:39:05.0908788Z status: exit code: 1
2019-09-24T06:39:05.0909722Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/inaccessible-test-modules.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/inaccessible-test-modules" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/inaccessible-test-modules/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0910535Z ------------------------------------------
2019-09-24T06:39:05.0910584Z 
2019-09-24T06:39:05.0910811Z ------------------------------------------
2019-09-24T06:39:05.0910882Z stderr:
2019-09-24T06:39:05.0910882Z stderr:
2019-09-24T06:39:05.0911105Z ------------------------------------------
2019-09-24T06:39:05.0911193Z error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0911246Z 
2019-09-24T06:39:05.0911313Z error[E0432]: unresolved import `main`
2019-09-24T06:39:05.0911581Z   --> /checkout/src/test/ui/test-attrs/inaccessible-test-modules.rs:5:5
2019-09-24T06:39:05.0911668Z    |
2019-09-24T06:39:05.0911735Z LL | use main as x; //~ ERROR unresolved import `main`
2019-09-24T06:39:05.0912020Z    |     |
2019-09-24T06:39:05.0912096Z    |     no `main` in the root
2019-09-24T06:39:05.0912172Z    |     help: a similar name exists in the module: `main`
2019-09-24T06:39:05.0912229Z 
2019-09-24T06:39:05.0912229Z 
2019-09-24T06:39:05.0912294Z error[E0432]: unresolved import `test`
2019-09-24T06:39:05.0912569Z   --> /checkout/src/test/ui/test-attrs/inaccessible-test-modules.rs:6:5
2019-09-24T06:39:05.0912647Z    |
2019-09-24T06:39:05.0912720Z LL | use test as y; //~ ERROR unresolved import `test`
2019-09-24T06:39:05.0913003Z    |     |
2019-09-24T06:39:05.0913070Z    |     no `test` in the root
2019-09-24T06:39:05.0913146Z    |     help: a similar name exists in the module: `test`
2019-09-24T06:39:05.0913196Z 
---
2019-09-24T06:39:05.0914242Z ---- [ui] ui/test-attrs/test-fn-signature-verification-for-explicit-return-type.rs stdout ----
2019-09-24T06:39:05.0914310Z 
2019-09-24T06:39:05.0914562Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0914637Z status: exit code: 1
2019-09-24T06:39:05.0915760Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-fn-signature-verification-for-explicit-return-type.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-fn-signature-verification-for-explicit-return-type/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-fn-signature-verification-for-explicit-return-type/auxiliary"
2019-09-24T06:39:05.0917227Z ------------------------------------------
2019-09-24T06:39:05.0917282Z 
2019-09-24T06:39:05.0917539Z ------------------------------------------
2019-09-24T06:39:05.0917614Z stderr:
---
2019-09-24T06:39:05.0918757Z ---- [ui] ui/test-attrs/test-cant-be-shadowed.rs stdout ----
2019-09-24T06:39:05.0918814Z 
2019-09-24T06:39:05.0919061Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0919151Z status: exit code: 1
2019-09-24T06:39:05.0920307Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-cant-be-shadowed.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-cant-be-shadowed" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-cant-be-shadowed/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0920835Z ------------------------------------------
2019-09-24T06:39:05.0920884Z 
2019-09-24T06:39:05.0921113Z ------------------------------------------
2019-09-24T06:39:05.0921189Z stderr:
---
2019-09-24T06:39:05.0922432Z ---- [ui] ui/test-attrs/test-main-not-dead-attr.rs stdout ----
2019-09-24T06:39:05.0922487Z 
2019-09-24T06:39:05.0922729Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0922803Z status: exit code: 1
2019-09-24T06:39:05.0923639Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-main-not-dead-attr.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-main-not-dead-attr/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-main-not-dead-attr/auxiliary"
2019-09-24T06:39:05.0924137Z ------------------------------------------
2019-09-24T06:39:05.0924187Z 
2019-09-24T06:39:05.0924411Z ------------------------------------------
2019-09-24T06:39:05.0924489Z stderr:
---
2019-09-24T06:39:05.0925549Z ---- [ui] ui/test-attrs/test-main-not-dead.rs stdout ----
2019-09-24T06:39:05.0925607Z 
2019-09-24T06:39:05.0925836Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0925916Z status: exit code: 1
2019-09-24T06:39:05.0927572Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-main-not-dead.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-main-not-dead/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-main-not-dead/auxiliary"
2019-09-24T06:39:05.0928098Z ------------------------------------------
2019-09-24T06:39:05.0928151Z 
2019-09-24T06:39:05.0928391Z ------------------------------------------
2019-09-24T06:39:05.0928465Z stderr:
---
2019-09-24T06:39:05.0929907Z ---- [ui] ui/test-attrs/test-on-macro.rs stdout ----
2019-09-24T06:39:05.0929961Z 
2019-09-24T06:39:05.0930209Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0930287Z status: exit code: 1
2019-09-24T06:39:05.0931137Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-on-macro.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-on-macro" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-on-macro/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0931707Z ------------------------------------------
2019-09-24T06:39:05.0931765Z 
2019-09-24T06:39:05.0932001Z ------------------------------------------
2019-09-24T06:39:05.0932080Z stderr:
2019-09-24T06:39:05.0932080Z stderr:
2019-09-24T06:39:05.0932304Z ------------------------------------------
2019-09-24T06:39:05.0932404Z warning: `#[test]` attribute should not be used on macros. Use `#[cfg(test)]` instead.
2019-09-24T06:39:05.0932765Z    |
2019-09-24T06:39:05.0932830Z LL | foo!();
2019-09-24T06:39:05.0932893Z    | ^^^^^^^
2019-09-24T06:39:05.0932939Z 
---
2019-09-24T06:39:05.0933802Z ---- [ui] ui/test-attrs/test-runner-hides-buried-main.rs stdout ----
2019-09-24T06:39:05.0933870Z 
2019-09-24T06:39:05.0934116Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0934205Z status: exit code: 1
2019-09-24T06:39:05.0935083Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-runner-hides-buried-main.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-runner-hides-buried-main/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-runner-hides-buried-main/auxiliary"
2019-09-24T06:39:05.0935595Z ------------------------------------------
2019-09-24T06:39:05.0935930Z 
2019-09-24T06:39:05.0936185Z ------------------------------------------
2019-09-24T06:39:05.0936255Z stderr:
---
2019-09-24T06:39:05.0937807Z ---- [ui] ui/test-attrs/test-runner-hides-main.rs stdout ----
2019-09-24T06:39:05.0937865Z 
2019-09-24T06:39:05.0938119Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0938198Z status: exit code: 1
2019-09-24T06:39:05.0939182Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-runner-hides-main.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-runner-hides-main/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-runner-hides-main/auxiliary"
2019-09-24T06:39:05.0939740Z ------------------------------------------
2019-09-24T06:39:05.0939809Z 
2019-09-24T06:39:05.0940204Z ------------------------------------------
2019-09-24T06:39:05.0940282Z stderr:
---
2019-09-24T06:39:05.0941356Z ---- [ui] ui/test-attrs/test-runner-hides-start.rs stdout ----
2019-09-24T06:39:05.0941417Z 
2019-09-24T06:39:05.0941658Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0941739Z status: exit code: 1
2019-09-24T06:39:05.0942560Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-runner-hides-start.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-runner-hides-start/a.wasm" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-runner-hides-start/auxiliary"
2019-09-24T06:39:05.0943056Z ------------------------------------------
2019-09-24T06:39:05.0943105Z 
2019-09-24T06:39:05.0943331Z ------------------------------------------
2019-09-24T06:39:05.0943399Z stderr:
---
2019-09-24T06:39:05.0944821Z ---- [ui] ui/test-attrs/test-should-panic-attr.rs stdout ----
2019-09-24T06:39:05.0944875Z 
2019-09-24T06:39:05.0945104Z error: test compilation failed although it shouldn't!
2019-09-24T06:39:05.0945188Z status: exit code: 1
2019-09-24T06:39:05.0946056Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-should-panic-attr.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-panic-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-panic-attr/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0947017Z ------------------------------------------
2019-09-24T06:39:05.0947072Z 
2019-09-24T06:39:05.0947317Z ------------------------------------------
2019-09-24T06:39:05.0947398Z stderr:
2019-09-24T06:39:05.0947398Z stderr:
2019-09-24T06:39:05.0947630Z ------------------------------------------
2019-09-24T06:39:05.0947724Z warning: argument must be of the form: `expected = "error message"`
2019-09-24T06:39:05.0948110Z    |
2019-09-24T06:39:05.0948110Z    |
2019-09-24T06:39:05.0948184Z LL | #[should_panic(expected)]
2019-09-24T06:39:05.0948341Z    |
2019-09-24T06:39:05.0948520Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-09-24T06:39:05.0948619Z 
2019-09-24T06:39:05.0948619Z 
2019-09-24T06:39:05.0948696Z warning: argument must be of the form: `expected = "error message"`
2019-09-24T06:39:05.0949116Z    |
2019-09-24T06:39:05.0949116Z    |
2019-09-24T06:39:05.0949188Z LL | #[should_panic(expect)]
2019-09-24T06:39:05.0949336Z    |
2019-09-24T06:39:05.0949425Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-09-24T06:39:05.0949509Z 
2019-09-24T06:39:05.0949509Z 
2019-09-24T06:39:05.0949582Z warning: argument must be of the form: `expected = "error message"`
2019-09-24T06:39:05.0950130Z    |
2019-09-24T06:39:05.0950130Z    |
2019-09-24T06:39:05.0950200Z LL | #[should_panic(expected(foo, bar))]
2019-09-24T06:39:05.0950353Z    |
2019-09-24T06:39:05.0950444Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-09-24T06:39:05.0950511Z 
2019-09-24T06:39:05.0950511Z 
2019-09-24T06:39:05.0950588Z warning: argument must be of the form: `expected = "error message"`
2019-09-24T06:39:05.0950950Z    |
2019-09-24T06:39:05.0950950Z    |
2019-09-24T06:39:05.0951011Z LL | #[should_panic(expected = "foo", bar)]
2019-09-24T06:39:05.0951156Z    |
2019-09-24T06:39:05.0951249Z    = note: Errors in this attribute were erroneously allowed and will become a hard error in a future release.
2019-09-24T06:39:05.0951314Z 
2019-09-24T06:39:05.0951403Z error: building tests with panic=abort is not yet supported
---
2019-09-24T06:39:05.0953541Z -    |
2019-09-24T06:39:05.0953760Z - note: lint level defined here
2019-09-24T06:39:05.0953988Z -   --> $DIR/test-warns-dead-code.rs:3:9
2019-09-24T06:39:05.0954192Z -    |
2019-09-24T06:39:05.0954510Z - LL | #![deny(dead_code)]
2019-09-24T06:39:05.0954811Z + error: building tests with panic=abort is not yet supported
2019-09-24T06:39:05.0954890Z 12 
2019-09-24T06:39:05.0954966Z 13 error: aborting due to previous error
2019-09-24T06:39:05.0955033Z 14 
2019-09-24T06:39:05.0955033Z 14 
2019-09-24T06:39:05.0955067Z 
2019-09-24T06:39:05.0955108Z 
2019-09-24T06:39:05.0955175Z The actual stderr differed from the expected stderr.
2019-09-24T06:39:05.0955531Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-warns-dead-code/test-warns-dead-code.stderr
2019-09-24T06:39:05.0955818Z To update references, rerun the tests and pass the `--bless` flag
2019-09-24T06:39:05.0956120Z To only update this specific test, also pass `--test-args test-attrs/test-warns-dead-code.rs`
2019-09-24T06:39:05.0956251Z error: 1 errors occurred comparing output.
2019-09-24T06:39:05.0956319Z status: exit code: 1
2019-09-24T06:39:05.0956319Z status: exit code: 1
2019-09-24T06:39:05.0958057Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-warns-dead-code.rs" "-Zthreads=1" "--target=wasm32-unknown-unknown" "--error-format" "json" "-Zui-testing" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-warns-dead-code" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-warns-dead-code/auxiliary" "-A" "unused"
2019-09-24T06:39:05.0958648Z ------------------------------------------
2019-09-24T06:39:05.0958702Z 
2019-09-24T06:39:05.0958946Z ------------------------------------------
2019-09-24T06:39:05.0959020Z stderr:
---
2019-09-24T06:39:05.0971569Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-24T06:39:05.0971677Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-24T06:39:05.0971732Z 
2019-09-24T06:39:05.0971765Z 
2019-09-24T06:39:05.0973554Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v9.2.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-24T06:39:05.0974198Z 
2019-09-24T06:39:05.0974245Z 
2019-09-24T06:39:05.0974730Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/compile-fail src/test/mir-opt src/test/codegen-units src/libcore
2019-09-24T06:39:05.0974867Z Build completed unsuccessfully in 1:24:33
2019-09-24T06:39:05.0974867Z Build completed unsuccessfully in 1:24:33
2019-09-24T06:39:05.0974936Z == clock drift check ==
2019-09-24T06:39:05.0975008Z   local time: Tue Sep 24 06:39:05 UTC 2019
2019-09-24T06:39:05.1648556Z   network time: Tue, 24 Sep 2019 06:39:05 GMT
2019-09-24T06:39:05.1648768Z == end clock drift check ==
2019-09-24T06:39:05.9897145Z ##[error]Bash exited with code '1'.
2019-09-24T06:39:05.9940883Z ##[section]Starting: Upload CPU usage statistics
2019-09-24T06:39:05.9948040Z ==============================================================================
2019-09-24T06:39:05.9948160Z Task         : Bash
2019-09-24T06:39:05.9948239Z Description  : Run a Bash script on macOS, Linux, or Windows
