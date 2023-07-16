plain
2020-02-23T06:24:08.5785371Z + 
2020-02-23T06:24:08.5785645Z + warning: any use of this value will cause an error
2020-02-23T06:24:08.5786192Z +   --> $DIR/validate_uninhabited_zsts.rs:14:26
2020-02-23T06:24:08.5786473Z +    |
2020-02-23T06:24:08.5786722Z 10 LL | const FOO: [Empty; 3] = [foo(); 3];
2020-02-23T06:24:08.5787935Z +    | -------------------------^^^^^-----
2020-02-23T06:24:08.5788248Z +    |                          |
2020-02-23T06:24:08.5788594Z +    |                          referenced constant has errors
2020-02-23T06:24:08.5788879Z 12    |
2020-02-23T06:24:08.5788879Z 12    |
2020-02-23T06:24:08.5789116Z 13 note: lint level defined here
2020-02-23T06:24:08.5789631Z 14   --> $DIR/validate_uninhabited_zsts.rs:13:8
2020-02-23T06:24:08.5789859Z 
2020-02-23T06:24:08.5790030Z 47    |
2020-02-23T06:24:08.5790498Z 48    = note: 0-variant enums have no valid value
2020-02-23T06:24:08.5791485Z - error: aborting due to previous error
2020-02-23T06:24:08.5791806Z + error: aborting due to 2 previous errors
2020-02-23T06:24:08.5792057Z 51 
2020-02-23T06:24:08.5792558Z 52 For more information about this error, try `rustc --explain E0080`.
2020-02-23T06:24:08.5792558Z 52 For more information about this error, try `rustc --explain E0080`.
2020-02-23T06:24:08.5792864Z 53 
2020-02-23T06:24:08.5792984Z 
2020-02-23T06:24:08.5793097Z 
2020-02-23T06:24:08.5793362Z The actual stderr differed from the expected stderr.
2020-02-23T06:24:08.5794329Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/validate_uninhabited_zsts.stderr
2020-02-23T06:24:08.5795139Z To update references, rerun the tests and pass the `--bless` flag
2020-02-23T06:24:08.5795888Z To only update this specific test, also pass `--test-args consts/const-eval/validate_uninhabited_zsts.rs`
2020-02-23T06:24:08.5796461Z error: 1 errors occurred comparing output.
2020-02-23T06:24:08.5796741Z status: exit code: 1
2020-02-23T06:24:08.5796741Z status: exit code: 1
2020-02-23T06:24:08.5798949Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/validate_uninhabited_zsts/auxiliary" "-A" "unused"
2020-02-23T06:24:08.5800783Z ------------------------------------------
2020-02-23T06:24:08.5800999Z 
2020-02-23T06:24:08.5801400Z ------------------------------------------
2020-02-23T06:24:08.5801660Z stderr:
---
2020-02-23T06:24:08.5804349Z 
2020-02-23T06:24:08.5804598Z warning: any use of this value will cause an error
2020-02-23T06:24:08.5805255Z   --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:14:26
2020-02-23T06:24:08.5805585Z    |
2020-02-23T06:24:08.5805837Z LL | const FOO: [Empty; 3] = [foo(); 3];
2020-02-23T06:24:08.5806622Z    |                          |
2020-02-23T06:24:08.5806960Z    |                          referenced constant has errors
2020-02-23T06:24:08.5807231Z    |
2020-02-23T06:24:08.5807454Z note: lint level defined here
---
2020-02-23T06:24:08.5808987Z 
2020-02-23T06:24:08.5809262Z error[E0080]: it is undefined behavior to use this value
2020-02-23T06:24:08.5809907Z   --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:17:1
2020-02-23T06:24:08.5810244Z    |
2020-02-23T06:24:08.5810632Z LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
2020-02-23T06:24:08.5811272Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered a value of an uninhabited type
2020-02-23T06:24:08.5811753Z    |
2020-02-23T06:24:08.5812708Z    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
2020-02-23T06:24:08.5813750Z warning: the type `!` does not permit zero-initialization
2020-02-23T06:24:08.5814417Z   --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:5:14
2020-02-23T06:24:08.5814746Z    |
2020-02-23T06:24:08.5815012Z LL |     unsafe { std::mem::transmute(()) }
2020-02-23T06:24:08.5815012Z LL |     unsafe { std::mem::transmute(()) }
2020-02-23T06:24:08.5815351Z    |              ^^^^^^^^^^^^^^^^^^^^^^^
2020-02-23T06:24:08.5815613Z    |              |
2020-02-23T06:24:08.5815942Z    |              this code causes undefined behavior when executed
2020-02-23T06:24:08.5816512Z    |              help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
2020-02-23T06:24:08.5816919Z    |
2020-02-23T06:24:08.5817177Z    = note: `#[warn(invalid_value)]` on by default
2020-02-23T06:24:08.5817548Z    = note: The never type (`!`) has no valid value
2020-02-23T06:24:08.5818273Z warning: the type `Empty` does not permit zero-initialization
2020-02-23T06:24:08.5818932Z   --> /checkout/src/test/ui/consts/const-eval/validate_uninhabited_zsts.rs:17:35
2020-02-23T06:24:08.5819272Z    |
2020-02-23T06:24:08.5819272Z    |
2020-02-23T06:24:08.5819608Z LL | const BAR: [Empty; 3] = [unsafe { std::mem::transmute(()) }; 3];
2020-02-23T06:24:08.5820393Z    |                                   |
2020-02-23T06:24:08.5820787Z    |                                   this code causes undefined behavior when executed
2020-02-23T06:24:08.5821407Z    |                                   help: use `MaybeUninit<T>` instead, and only call `assume_init` after initialization is done
2020-02-23T06:24:08.5821839Z    |
2020-02-23T06:24:08.5821839Z    |
2020-02-23T06:24:08.5822290Z    = note: 0-variant enums have no valid value
2020-02-23T06:24:08.5822744Z error: aborting due to 2 previous errors
2020-02-23T06:24:08.5822943Z 
2020-02-23T06:24:08.5823438Z For more information about this error, try `rustc --explain E0080`.
2020-02-23T06:24:08.5823691Z 
---
2020-02-23T06:24:08.5826786Z 10 
2020-02-23T06:24:08.5826924Z 
2020-02-23T06:24:08.5827039Z 
2020-02-23T06:24:08.5827284Z The actual stderr differed from the expected stderr.
2020-02-23T06:24:08.5828135Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/async-generator-issue-67158/async-generator-issue-67158.stderr
2020-02-23T06:24:08.5828918Z To update references, rerun the tests and pass the `--bless` flag
2020-02-23T06:24:08.5829629Z To only update this specific test, also pass `--test-args generator/async-generator-issue-67158.rs`
2020-02-23T06:24:08.5830185Z error: 1 errors occurred comparing output.
2020-02-23T06:24:08.5830477Z status: exit code: 1
2020-02-23T06:24:08.5830477Z status: exit code: 1
2020-02-23T06:24:08.5832794Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/async-generator-issue-67158.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/async-generator-issue-67158" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/async-generator-issue-67158/auxiliary" "-A" "unused"
2020-02-23T06:24:08.5834739Z ------------------------------------------
2020-02-23T06:24:08.5834942Z 
2020-02-23T06:24:08.5835354Z ------------------------------------------
2020-02-23T06:24:08.5835597Z stderr:
2020-02-23T06:24:08.5835597Z stderr:
2020-02-23T06:24:08.5836022Z ------------------------------------------
2020-02-23T06:24:08.5836365Z error[E0727]: `async` generators are not yet supported
2020-02-23T06:24:08.5837010Z   --> /checkout/src/test/ui/generator/async-generator-issue-67158.rs:5:13
2020-02-23T06:24:08.5837347Z    |
2020-02-23T06:24:08.5837700Z LL |     async { yield print!(":C") }; //~ ERROR `async` generators are not yet supported
2020-02-23T06:24:08.5838378Z 
2020-02-23T06:24:08.5838611Z error: aborting due to previous error
2020-02-23T06:24:08.5838804Z 
2020-02-23T06:24:08.5838918Z 
---
2020-02-23T06:24:08.5880355Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2020-02-23T06:24:08.5880872Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-02-23T06:24:08.5884853Z 
2020-02-23T06:24:08.5885187Z 
2020-02-23T06:24:08.5889756Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-23T06:24:08.5893075Z 
2020-02-23T06:24:08.5893319Z 
2020-02-23T06:24:08.5898425Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-23T06:24:08.5898825Z Build completed unsuccessfully in 0:59:30
2020-02-23T06:24:08.5898825Z Build completed unsuccessfully in 0:59:30
2020-02-23T06:24:08.5945175Z == clock drift check ==
2020-02-23T06:24:08.5994096Z   local time: Sun Feb 23 06:24:08 UTC 2020
2020-02-23T06:24:08.8618464Z   network time: Sun, 23 Feb 2020 06:24:08 GMT
2020-02-23T06:24:08.8618922Z == end clock drift check ==
2020-02-23T06:24:09.3251730Z 
2020-02-23T06:24:09.3331941Z ##[error]Bash exited with code '1'.
2020-02-23T06:24:09.3405248Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-23T06:24:09.3412833Z ==============================================================================
2020-02-23T06:24:09.3413575Z Task         : Get sources
2020-02-23T06:24:09.3414315Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
