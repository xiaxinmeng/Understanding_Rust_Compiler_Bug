plain
2020-02-01T01:15:11.3948654Z ========================== Starting Command Output ===========================
2020-02-01T01:15:11.3950454Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/af415ec5-73f0-4d42-ad43-8579b1f114cc.sh
2020-02-01T01:15:11.3950505Z 
2020-02-01T01:15:11.3953027Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-01T01:15:11.3961573Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68723/merge to s
2020-02-01T01:15:11.3964008Z Task         : Get sources
2020-02-01T01:15:11.3964050Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T01:15:11.3964080Z Version      : 1.0.0
2020-02-01T01:15:11.3964110Z Author       : Microsoft
---
2020-02-01T01:15:12.3762662Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-01T01:15:12.3774238Z ##[command]git config gc.auto 0
2020-02-01T01:15:12.3776865Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-01T01:15:12.3779079Z ##[command]git config --get-all http.proxy
2020-02-01T01:15:12.3785714Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68723/merge:refs/remotes/pull/68723/merge
---
2020-02-01T02:10:34.4047871Z .................................................................................................... 1700/9560
2020-02-01T02:10:39.2593395Z .................................................................................................... 1800/9560
2020-02-01T02:10:51.4156542Z ..........................i......................................................................... 1900/9560
2020-02-01T02:10:58.1671773Z .................................................................................................... 2000/9560
2020-02-01T02:11:12.2533087Z ................iiiii............................................................................... 2100/9560
2020-02-01T02:11:21.5974289Z .................................................................................................... 2300/9560
2020-02-01T02:11:24.3498050Z .................................................................................................... 2400/9560
2020-02-01T02:11:29.1895452Z .................................................................................................... 2500/9560
2020-02-01T02:11:49.7330143Z .................................................................................................... 2600/9560
---
2020-02-01T02:14:19.5965027Z .................................................................................................... 4800/9560
2020-02-01T02:14:24.4850949Z ...........................................................i...............i........................ 4900/9560
2020-02-01T02:14:32.2204673Z .................................................................................................... 5000/9560
2020-02-01T02:14:40.0984936Z .................................................................................................... 5100/9560
2020-02-01T02:14:44.7106456Z ..i................................................................................................. 5200/9560
2020-02-01T02:14:55.1155790Z ...........................................................................ii.ii........i...i....... 5300/9560
2020-02-01T02:15:03.3089008Z .............i...................................................................................... 5500/9560
2020-02-01T02:15:12.8960445Z .................................................................................................... 5600/9560
2020-02-01T02:15:18.9293742Z ..............................................................i..................................... 5700/9560
2020-02-01T02:15:25.8950563Z .................................................................................................... 5800/9560
2020-02-01T02:15:25.8950563Z .................................................................................................... 5800/9560
2020-02-01T02:15:33.3888589Z .................................................................................................... 5900/9560
2020-02-01T02:15:42.2241971Z .....................................................ii...i..ii...........i......................... 6000/9560
2020-02-01T02:16:03.2343151Z .................................................................................................... 6200/9560
2020-02-01T02:16:10.5445402Z .................................................................................................... 6300/9560
2020-02-01T02:16:10.5445402Z .................................................................................................... 6300/9560
2020-02-01T02:16:18.7054035Z .................................................................................i..ii.............. 6400/9560
2020-02-01T02:16:45.4252540Z .................................................................................................... 6600/9560
2020-02-01T02:16:50.8421652Z .........................................................i.......................................... 6700/9560
2020-02-01T02:16:52.9898957Z .................................................................................................... 6800/9560
2020-02-01T02:16:55.2254248Z .........................................................i.......................................... 6900/9560
---
2020-02-01T02:18:34.1844182Z .................................................................................................... 7600/9560
2020-02-01T02:18:39.5218436Z .................................................................................................... 7700/9560
2020-02-01T02:18:46.0529710Z .................................................................................................... 7800/9560
2020-02-01T02:18:56.5907495Z .................................................................................................... 7900/9560
2020-02-01T02:19:02.4926526Z .............iiiiiii.i.............................................................................. 8000/9560
2020-02-01T02:19:16.8347224Z .................................................................................................... 8200/9560
2020-02-01T02:19:27.0673756Z .................................................................................................... 8300/9560
2020-02-01T02:19:39.8228929Z .................................................................................................... 8400/9560
2020-02-01T02:19:46.4881317Z .................................................................................................... 8500/9560
---
2020-02-01T02:21:38.5337548Z 5    |         ^^^^^^
2020-02-01T02:21:38.5337577Z 
2020-02-01T02:21:38.5337603Z 
2020-02-01T02:21:38.5337648Z The actual stderr differed from the expected stderr.
2020-02-01T02:21:38.5338265Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-68697-remove-mut-ref/issue-68697-remove-mut-ref.stderr
2020-02-01T02:21:38.5338524Z To update references, rerun the tests and pass the `--bless` flag
2020-02-01T02:21:38.5338846Z To only update this specific test, also pass `--test-args borrowck/issue-68697-remove-mut-ref.rs`
2020-02-01T02:21:38.5338952Z error: 1 errors occurred comparing output.
2020-02-01T02:21:38.5339123Z status: exit code: 1
2020-02-01T02:21:38.5339123Z status: exit code: 1
2020-02-01T02:21:38.5340050Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-68697-remove-mut-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-68697-remove-mut-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-68697-remove-mut-ref/auxiliary" "-A" "unused"
2020-02-01T02:21:38.5340425Z ------------------------------------------
2020-02-01T02:21:38.5340462Z 
2020-02-01T02:21:38.5340701Z ------------------------------------------
2020-02-01T02:21:38.5340766Z stderr:
2020-02-01T02:21:38.5340766Z stderr:
2020-02-01T02:21:38.5341090Z ------------------------------------------
2020-02-01T02:21:38.5341152Z error[E0596]: cannot borrow `x` as mutable, as it is not declared as mutable
2020-02-01T02:21:38.5341419Z   --> /checkout/src/test/ui/borrowck/issue-68697-remove-mut-ref.rs:9:9
2020-02-01T02:21:38.5341469Z    |
2020-02-01T02:21:38.5341509Z LL |     bar(&mut x);
2020-02-01T02:21:38.5341550Z    |         ^^^^^^
2020-02-01T02:21:38.5341728Z    |         |
2020-02-01T02:21:38.5341774Z    |         cannot borrow as mutable
2020-02-01T02:21:38.5341831Z    |         help: remove the unnecessary `&mut` here: `x`
2020-02-01T02:21:38.5341922Z error: aborting due to previous error
2020-02-01T02:21:38.5341953Z 
2020-02-01T02:21:38.5342203Z For more information about this error, try `rustc --explain E0596`.
2020-02-01T02:21:38.5342255Z 
2020-02-01T02:21:38.5342255Z 
2020-02-01T02:21:38.5342469Z ------------------------------------------
2020-02-01T02:21:38.5342502Z 
2020-02-01T02:21:38.5342527Z 
2020-02-01T02:21:38.5342771Z ---- [ui] ui/borrowck/mut-borrow-of-mut-ref.rs stdout ----
2020-02-01T02:21:38.5342828Z diff of stderr:
2020-02-01T02:21:38.5342856Z 
2020-02-01T02:21:38.5342903Z 1 error[E0596]: cannot borrow `b` as mutable, as it is not declared as mutable
2020-02-01T02:21:38.5343192Z 3    |
2020-02-01T02:21:38.5343192Z 3    |
2020-02-01T02:21:38.5343391Z - LL | fn f(b: &mut i32) {
2020-02-01T02:21:38.5343647Z -    |      - help: consider changing this to be mutable: `mut b`
2020-02-01T02:21:38.5343697Z 6 LL |     g(&mut b)
2020-02-01T02:21:38.5343976Z +    |       ^^^^^^
2020-02-01T02:21:38.5344037Z +    |       |
2020-02-01T02:21:38.5344080Z +    |       cannot borrow as mutable
2020-02-01T02:21:38.5344080Z +    |       cannot borrow as mutable
2020-02-01T02:21:38.5344129Z +    |       help: remove the unnecessary `&mut` here: `b`
2020-02-01T02:21:38.5344232Z 9 error: aborting due to previous error
2020-02-01T02:21:38.5344273Z 10 
2020-02-01T02:21:38.5344299Z 
2020-02-01T02:21:38.5344341Z 
2020-02-01T02:21:38.5344341Z 
2020-02-01T02:21:38.5344393Z The actual stderr differed from the expected stderr.
2020-02-01T02:21:38.5344715Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-of-mut-ref/mut-borrow-of-mut-ref.stderr
2020-02-01T02:21:38.5345092Z To update references, rerun the tests and pass the `--bless` flag
2020-02-01T02:21:38.5345380Z To only update this specific test, also pass `--test-args borrowck/mut-borrow-of-mut-ref.rs`
2020-02-01T02:21:38.5345573Z error: 1 errors occurred comparing output.
2020-02-01T02:21:38.5345617Z status: exit code: 1
2020-02-01T02:21:38.5345617Z status: exit code: 1
2020-02-01T02:21:38.5346523Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/mut-borrow-of-mut-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-of-mut-ref" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/mut-borrow-of-mut-ref/auxiliary" "-A" "unused"
2020-02-01T02:21:38.5346904Z ------------------------------------------
2020-02-01T02:21:38.5346956Z 
2020-02-01T02:21:38.5347187Z ------------------------------------------
2020-02-01T02:21:38.5347244Z stderr:
2020-02-01T02:21:38.5347244Z stderr:
2020-02-01T02:21:38.5347471Z ------------------------------------------
2020-02-01T02:21:38.5347541Z error[E0596]: cannot borrow `b` as mutable, as it is not declared as mutable
2020-02-01T02:21:38.5347850Z    |
2020-02-01T02:21:38.5347850Z    |
2020-02-01T02:21:38.5347911Z LL |     g(&mut b) //~ ERROR cannot borrow
2020-02-01T02:21:38.5347996Z    |       |
2020-02-01T02:21:38.5348054Z    |       cannot borrow as mutable
2020-02-01T02:21:38.5348054Z    |       cannot borrow as mutable
2020-02-01T02:21:38.5348110Z    |       help: remove the unnecessary `&mut` here: `b`
2020-02-01T02:21:38.5348185Z error: aborting due to previous error
2020-02-01T02:21:38.5348230Z 
2020-02-01T02:21:38.5348492Z For more information about this error, try `rustc --explain E0596`.
2020-02-01T02:21:38.5348526Z 
---
2020-02-01T02:21:38.5360162Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-01T02:21:38.5360258Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-01T02:21:38.5373315Z 
2020-02-01T02:21:38.5373393Z 
2020-02-01T02:21:38.5375150Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-01T02:21:38.5375590Z 
2020-02-01T02:21:38.5375620Z 
2020-02-01T02:21:38.5383227Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-01T02:21:38.5383382Z Build completed unsuccessfully in 1:00:45
2020-02-01T02:21:38.5383382Z Build completed unsuccessfully in 1:00:45
2020-02-01T02:21:38.5434266Z == clock drift check ==
2020-02-01T02:21:38.5473304Z   local time: Sat Feb  1 02:21:38 UTC 2020
2020-02-01T02:21:39.0936713Z   network time: Sat, 01 Feb 2020 02:21:39 GMT
2020-02-01T02:21:39.0941178Z == end clock drift check ==
2020-02-01T02:21:39.4547913Z 
2020-02-01T02:21:39.4664710Z ##[error]Bash exited with code '1'.
2020-02-01T02:21:39.4677268Z ##[section]Finishing: Run build
2020-02-01T02:21:39.4699965Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68723/merge to s
2020-02-01T02:21:39.4701888Z Task         : Get sources
2020-02-01T02:21:39.4701954Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-01T02:21:39.4702017Z Version      : 1.0.0
2020-02-01T02:21:39.4702059Z Author       : Microsoft
2020-02-01T02:21:39.4702059Z Author       : Microsoft
2020-02-01T02:21:39.4702105Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-01T02:21:39.4702171Z ==============================================================================
2020-02-01T02:21:39.8920523Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-01T02:21:39.8962631Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68723/merge to s
2020-02-01T02:21:39.9083076Z Cleaning up task key
2020-02-01T02:21:39.9083918Z Start cleaning up orphan processes.
2020-02-01T02:21:39.9194662Z Terminate orphan process: pid (5152) (python)
2020-02-01T02:21:39.9428038Z ##[section]Finishing: Finalize Job
