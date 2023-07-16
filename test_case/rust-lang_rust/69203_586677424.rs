plain
2020-02-16T06:23:41.3155172Z ========================== Starting Command Output ===========================
2020-02-16T06:23:41.3156632Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/17cf9447-3f5c-4e8d-b138-2ac6f3e360cf.sh
2020-02-16T06:23:41.3156666Z 
2020-02-16T06:23:41.3162232Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-16T06:23:41.3168065Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69203/merge to s
2020-02-16T06:23:41.3170372Z Task         : Get sources
2020-02-16T06:23:41.3170410Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T06:23:41.3170492Z Version      : 1.0.0
2020-02-16T06:23:41.3170527Z Author       : Microsoft
---
2020-02-16T06:23:42.1547080Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-16T06:23:42.1646346Z ##[command]git config gc.auto 0
2020-02-16T06:23:42.1701510Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-16T06:23:42.1742636Z ##[command]git config --get-all http.proxy
2020-02-16T06:23:42.1882949Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69203/merge:refs/remotes/pull/69203/merge
---
2020-02-16T07:19:08.1814496Z .................................................................................................... 1700/9650
2020-02-16T07:19:13.0737499Z .................................................................................................... 1800/9650
2020-02-16T07:19:24.8129986Z ..................................i.............F................................................... 1900/9650
2020-02-16T07:19:32.3483410Z .................................................................................................... 2000/9650
2020-02-16T07:19:46.1131742Z ........................iiiii....................................................................... 2100/9650
2020-02-16T07:19:55.9479996Z .................................................................................................... 2300/9650
2020-02-16T07:19:58.3294449Z .................................................................................................... 2400/9650
2020-02-16T07:20:02.9637325Z .................................................................................................... 2500/9650
2020-02-16T07:20:23.1643106Z .................................................................................................... 2600/9650
---
2020-02-16T07:23:34.5753125Z .................................................................................................... 5600/9650
2020-02-16T07:23:44.3060901Z .......................................................................................i............ 5700/9650
2020-02-16T07:23:51.7204608Z .................................................................................................... 5800/9650
2020-02-16T07:23:56.7650204Z .....................................................................................i.............. 5900/9650
2020-02-16T07:24:06.0126098Z ...............................................................................ii...i..ii........... 6000/9650
2020-02-16T07:24:17.9809799Z i................................................................................................... 6100/9650
2020-02-16T07:24:32.2752192Z .................................................................................................... 6300/9650
2020-02-16T07:24:36.1008545Z .................................................................................................... 6400/9650
2020-02-16T07:24:36.1008545Z .................................................................................................... 6400/9650
2020-02-16T07:24:48.2679031Z .......i..ii........................................................................................ 6500/9650
2020-02-16T07:25:07.0120919Z ...............................................................................................i.... 6700/9650
2020-02-16T07:25:09.1291153Z .................................................................................................... 6800/9650
2020-02-16T07:25:11.3044234Z .................................................................................................... 6900/9650
2020-02-16T07:25:13.6678368Z .....i.............................................................................................. 7000/9650
---
2020-02-16T07:26:45.7135406Z .................................................................................................... 7600/9650
2020-02-16T07:26:50.1402318Z .................................................................................................... 7700/9650
2020-02-16T07:26:55.9341977Z .................................................................................................... 7800/9650
2020-02-16T07:27:02.3162022Z .................................................................................................... 7900/9650
2020-02-16T07:27:11.5911004Z .....................................................FF...F............................iiiiiii.i.... 8000/9650
2020-02-16T07:27:26.9494577Z ...........................i......i................................................................. 8200/9650
2020-02-16T07:27:31.7359883Z .................................................................................................... 8300/9650
2020-02-16T07:27:42.6700721Z .................................................................................................... 8400/9650
2020-02-16T07:27:53.9077556Z .................................................................................................... 8500/9650
---
2020-02-16T07:29:47.5633833Z failures:
2020-02-16T07:29:47.5661971Z 
2020-02-16T07:29:47.5671987Z ---- [ui] ui/custom_test_frameworks/mismatch.rs stdout ----
2020-02-16T07:29:47.5672350Z 
2020-02-16T07:29:47.5672618Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-02-16T07:29:47.5672846Z status: exit code: 101
2020-02-16T07:29:47.5673897Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom_test_frameworks/mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch/auxiliary" "-A" "unused"
2020-02-16T07:29:47.5674615Z ------------------------------------------
2020-02-16T07:29:47.5674829Z 
2020-02-16T07:29:47.5675218Z ------------------------------------------
2020-02-16T07:29:47.5675433Z stderr:
2020-02-16T07:29:47.5675433Z stderr:
2020-02-16T07:29:47.5675836Z ------------------------------------------
2020-02-16T07:29:47.5676326Z thread 'rustc' panicked at 'attempted .def_id() on invalid res: NonMacroAttr(Builtin)', <::std::macros::panic macros>:5:6
2020-02-16T07:29:47.5676773Z 
2020-02-16T07:29:47.5677230Z error: internal compiler error: unexpected panic
2020-02-16T07:29:47.5677407Z 
2020-02-16T07:29:47.5677622Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-16T07:29:47.5677622Z note: the compiler unexpectedly panicked. this is a bug.
2020-02-16T07:29:47.5677795Z 
2020-02-16T07:29:47.5678903Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-02-16T07:29:47.5679098Z 
2020-02-16T07:29:47.5679548Z note: rustc 1.43.0-nightly (88ba233d1 2020-02-16) running on x86_64-unknown-linux-gnu
2020-02-16T07:29:47.5679684Z 
2020-02-16T07:29:47.5680316Z note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2020-02-16T07:29:47.5680594Z 
2020-02-16T07:29:47.5680943Z ------------------------------------------
2020-02-16T07:29:47.5681075Z 
2020-02-16T07:29:47.5681187Z 
2020-02-16T07:29:47.5681187Z 
2020-02-16T07:29:47.5681559Z ---- [ui] ui/rust-2018/issue-54400-unused-extern-crate-attr-span.rs stdout ----
2020-02-16T07:29:47.5681803Z 
2020-02-16T07:29:47.5681942Z error: fixed code is still producing diagnostics
2020-02-16T07:29:47.5682095Z status: exit code: 0
2020-02-16T07:29:47.5683183Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span.fixed" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "edition_lint_paths" "--cfg" "blandiloquence" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/issue-54400-unused-extern-crate-attr-span/auxiliary"
2020-02-16T07:29:47.5683741Z ------------------------------------------
2020-02-16T07:29:47.5683891Z 
2020-02-16T07:29:47.5684211Z ------------------------------------------
2020-02-16T07:29:47.5684358Z stderr:
2020-02-16T07:29:47.5684358Z stderr:
2020-02-16T07:29:47.5684688Z ------------------------------------------
2020-02-16T07:29:47.5684849Z warning: crate `edition_lint_paths` is unused in crate `issue_54400_unused_extern_crate_attr_span`
2020-02-16T07:29:47.5684990Z    |
2020-02-16T07:29:47.5685155Z    = help: try removing `edition_lint_paths` from your Cargo.toml
2020-02-16T07:29:47.5685386Z 
2020-02-16T07:29:47.5685718Z ------------------------------------------
2020-02-16T07:29:47.5685847Z 
2020-02-16T07:29:47.5685960Z 
2020-02-16T07:29:47.5685960Z 
2020-02-16T07:29:47.5686328Z ---- [ui] ui/rust-2018/local-path-suggestions-2018.rs stdout ----
2020-02-16T07:29:47.5686498Z diff of stderr:
2020-02-16T07:29:47.5686614Z 
2020-02-16T07:29:47.5686750Z 12 LL | use foobar::Baz;
2020-02-16T07:29:47.5686908Z 13    |     ^^^^^^ help: a similar path exists: `baz::foobar`
2020-02-16T07:29:47.5687055Z 14 
2020-02-16T07:29:47.5687197Z + warning: crate `baz` is unused in crate `local_path_suggestions_2018`
2020-02-16T07:29:47.5687352Z +    |
2020-02-16T07:29:47.5687492Z +    = help: try removing `baz` from your Cargo.toml
2020-02-16T07:29:47.5687942Z 15 error: aborting due to 2 previous errors
2020-02-16T07:29:47.5689346Z 16 
2020-02-16T07:29:47.5691361Z 17 For more information about this error, try `rustc --explain E0432`.
2020-02-16T07:29:47.5692500Z 
2020-02-16T07:29:47.5692500Z 
2020-02-16T07:29:47.5692655Z 
2020-02-16T07:29:47.5693750Z The actual stderr differed from the expected stderr.
2020-02-16T07:29:47.5694420Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/local-path-suggestions-2018/local-path-suggestions-2018.stderr
2020-02-16T07:29:47.5694797Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T07:29:47.5695212Z To only update this specific test, also pass `--test-args rust-2018/local-path-suggestions-2018.rs`
2020-02-16T07:29:47.5695493Z error: 1 errors occurred comparing output.
2020-02-16T07:29:47.5695648Z status: exit code: 1
2020-02-16T07:29:47.5695648Z status: exit code: 1
2020-02-16T07:29:47.5696874Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/local-path-suggestions-2018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/local-path-suggestions-2018" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--extern" "baz" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/local-path-suggestions-2018/auxiliary" "-A" "unused"
2020-02-16T07:29:47.5697450Z ------------------------------------------
2020-02-16T07:29:47.5697668Z 
2020-02-16T07:29:47.5698017Z ------------------------------------------
2020-02-16T07:29:47.5698166Z stderr:
---
2020-02-16T07:29:47.5701247Z    |     ^^^^^^ help: a similar path exists: `baz::foobar`
2020-02-16T07:29:47.5701381Z 
2020-02-16T07:29:47.5701521Z warning: crate `baz` is unused in crate `local_path_suggestions_2018`
2020-02-16T07:29:47.5701655Z    |
2020-02-16T07:29:47.5701810Z    = help: try removing `baz` from your Cargo.toml
2020-02-16T07:29:47.5702062Z error: aborting due to 2 previous errors
2020-02-16T07:29:47.5702176Z 
2020-02-16T07:29:47.5702806Z For more information about this error, try `rustc --explain E0432`.
2020-02-16T07:29:47.5702954Z 
---
2020-02-16T07:29:47.5708921Z 
2020-02-16T07:29:47.5708967Z 
2020-02-16T07:29:47.5709016Z The actual stderr differed from the expected stderr.
2020-02-16T07:29:47.5709355Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/trait-import-suggestions.stderr
2020-02-16T07:29:47.5709622Z To update references, rerun the tests and pass the `--bless` flag
2020-02-16T07:29:47.5709905Z To only update this specific test, also pass `--test-args rust-2018/trait-import-suggestions.rs`
2020-02-16T07:29:47.5709988Z error: 1 errors occurred comparing output.
2020-02-16T07:29:47.5710051Z status: exit code: 1
2020-02-16T07:29:47.5710051Z status: exit code: 1
2020-02-16T07:29:47.5711039Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/trait-import-suggestions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--extern" "trait-import-suggestions" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/trait-import-suggestions/auxiliary" "-A" "unused"
2020-02-16T07:29:47.5711462Z ------------------------------------------
2020-02-16T07:29:47.5711512Z 
2020-02-16T07:29:47.5711726Z ------------------------------------------
2020-02-16T07:29:47.5711774Z stderr:
2020-02-16T07:29:47.5711774Z stderr:
2020-02-16T07:29:47.5711984Z ------------------------------------------
2020-02-16T07:29:47.5712058Z error[E0599]: no method named `foobar` found for type `u32` in the current scope
2020-02-16T07:29:47.5712314Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:22:11
2020-02-16T07:29:47.5712367Z    |
2020-02-16T07:29:47.5712433Z LL |         x.foobar(); //~ ERROR no method named `foobar`
2020-02-16T07:29:47.5712531Z    |
2020-02-16T07:29:47.5712607Z    = help: items from traits can only be used if the trait is in scope
2020-02-16T07:29:47.5712665Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-02-16T07:29:47.5712665Z    = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-02-16T07:29:47.5712720Z            `use crate::foo::foobar::Foobar;`
2020-02-16T07:29:47.5712828Z error[E0599]: no method named `bar` found for type `u32` in the current scope
2020-02-16T07:29:47.5713078Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:28:7
2020-02-16T07:29:47.5713128Z    |
2020-02-16T07:29:47.5713192Z LL |     x.bar(); //~ ERROR no method named `bar`
---
2020-02-16T07:29:47.5713598Z 
2020-02-16T07:29:47.5713649Z error[E0599]: no method named `baz` found for type `u32` in the current scope
2020-02-16T07:29:47.5713913Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:29:7
2020-02-16T07:29:47.5713970Z    |
2020-02-16T07:29:47.5714017Z LL |     x.baz(); //~ ERROR no method named `baz`
2020-02-16T07:29:47.5714116Z 
2020-02-16T07:29:47.5714168Z error[E0599]: no function or associated item named `from_str` found for type `u32` in the current scope
2020-02-16T07:29:47.5714439Z   --> /checkout/src/test/ui/rust-2018/trait-import-suggestions.rs:30:18
2020-02-16T07:29:47.5714490Z    |
2020-02-16T07:29:47.5714490Z    |
2020-02-16T07:29:47.5714542Z LL |     let y = u32::from_str("33"); //~ ERROR no function or associated item named `from_str`
2020-02-16T07:29:47.5714615Z    |                  ^^^^^^^^ function or associated item not found in `u32`
2020-02-16T07:29:47.5714720Z    = help: items from traits can only be used if the trait is in scope
2020-02-16T07:29:47.5714797Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2020-02-16T07:29:47.5714846Z    |
2020-02-16T07:29:47.5714898Z LL | use std::str::FromStr;
---
2020-02-16T07:29:47.5719361Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-16T07:29:47.5719423Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-16T07:29:47.5719473Z 
2020-02-16T07:29:47.5719500Z 
2020-02-16T07:29:47.5721548Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-16T07:29:47.5721798Z 
2020-02-16T07:29:47.5721828Z 
2020-02-16T07:29:47.5722672Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-16T07:29:47.5722775Z Build completed unsuccessfully in 0:59:24
2020-02-16T07:29:47.5722775Z Build completed unsuccessfully in 0:59:24
2020-02-16T07:29:47.5819588Z == clock drift check ==
2020-02-16T07:29:47.5853703Z   local time: Sun Feb 16 07:29:47 UTC 2020
2020-02-16T07:29:48.1262844Z   network time: Sun, 16 Feb 2020 07:29:48 GMT
2020-02-16T07:29:48.1263388Z == end clock drift check ==
2020-02-16T07:29:48.5544319Z 
2020-02-16T07:29:48.5657440Z ##[error]Bash exited with code '1'.
2020-02-16T07:29:48.5670829Z ##[section]Finishing: Run build
2020-02-16T07:29:48.5689392Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69203/merge to s
2020-02-16T07:29:48.5691238Z Task         : Get sources
2020-02-16T07:29:48.5691289Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-16T07:29:48.5691361Z Version      : 1.0.0
2020-02-16T07:29:48.5691405Z Author       : Microsoft
2020-02-16T07:29:48.5691405Z Author       : Microsoft
2020-02-16T07:29:48.5691470Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-16T07:29:48.5691540Z ==============================================================================
2020-02-16T07:29:48.9775193Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-16T07:29:48.9820783Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69203/merge to s
2020-02-16T07:29:48.9925757Z Cleaning up task key
2020-02-16T07:29:48.9926570Z Start cleaning up orphan processes.
2020-02-16T07:29:49.0028845Z Terminate orphan process: pid (3471) (python)
2020-02-16T07:29:49.0294144Z ##[section]Finishing: Finalize Job
