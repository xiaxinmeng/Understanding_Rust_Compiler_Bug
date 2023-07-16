plain
2020-02-06T06:19:51.0317583Z ========================== Starting Command Output ===========================
2020-02-06T06:19:51.0320090Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/07a29866-baf1-4752-b651-5e2a809c9574.sh
2020-02-06T06:19:51.0320129Z 
2020-02-06T06:19:51.0327558Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-06T06:19:51.0336225Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68879/merge to s
2020-02-06T06:19:51.0338041Z Task         : Get sources
2020-02-06T06:19:51.0338077Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T06:19:51.0338113Z Version      : 1.0.0
2020-02-06T06:19:51.0338187Z Author       : Microsoft
---
2020-02-06T06:19:51.7885052Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-06T06:19:51.7967287Z ##[command]git config gc.auto 0
2020-02-06T06:19:51.8035537Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-06T06:19:51.8150013Z ##[command]git config --get-all http.proxy
2020-02-06T06:19:51.8261504Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68879/merge:refs/remotes/pull/68879/merge
---
2020-02-06T07:09:47.1057504Z .................................................................................................... 1700/9586
2020-02-06T07:09:51.2399702Z .................................................................................................... 1800/9586
2020-02-06T07:10:02.1231249Z .............................i...................................................................... 1900/9586
2020-02-06T07:10:08.3610735Z .................................................................................................... 2000/9586
2020-02-06T07:10:20.9565261Z ...................iiiii............................................................................ 2100/9586
2020-02-06T07:10:29.8023471Z .................................................................................................... 2300/9586
2020-02-06T07:10:31.8803790Z .................................................................................................... 2400/9586
2020-02-06T07:10:36.1577347Z .................................................................................................... 2500/9586
2020-02-06T07:10:54.9926568Z .................................................................................................... 2600/9586
---
2020-02-06T07:13:11.0992004Z ..............................................................i...............i..................... 4900/9586
2020-02-06T07:13:17.4240111Z .................................................................................................... 5000/9586
2020-02-06T07:13:24.4771454Z .................................................................................................... 5100/9586
2020-02-06T07:13:28.3618819Z .....i.............................................................................................. 5200/9586
2020-02-06T07:13:37.8315266Z ...............................................................................ii.ii........i...i... 5300/9586
2020-02-06T07:13:44.7983522Z .................i.................................................................................. 5500/9586
2020-02-06T07:13:52.7386467Z .................................................................................................... 5600/9586
2020-02-06T07:13:58.4657788Z ..................................................................i................................. 5700/9586
2020-02-06T07:14:04.9477545Z .................................................................................................... 5800/9586
2020-02-06T07:14:04.9477545Z .................................................................................................... 5800/9586
2020-02-06T07:14:11.5216628Z .................................................................................................... 5900/9586
2020-02-06T07:14:20.1973108Z .........................................................ii...i..ii...........i..................... 6000/9586
2020-02-06T07:14:39.4295716Z .................................................................................................... 6200/9586
2020-02-06T07:14:45.7426762Z .................................................................................................... 6300/9586
2020-02-06T07:14:45.7426762Z .................................................................................................... 6300/9586
2020-02-06T07:14:49.4616802Z .....................................................................................i..ii.......... 6400/9586
2020-02-06T07:15:10.5209951Z .................................................................................................... 6600/9586
2020-02-06T07:15:18.9469177Z .......................................................................i............................ 6700/9586
2020-02-06T07:15:20.8291800Z .................................................................................................... 6800/9586
2020-02-06T07:15:22.7376916Z .........................................................................i.......................... 6900/9586
---
2020-02-06T07:16:49.2052831Z .................................................................................................... 7600/9586
2020-02-06T07:16:53.1770081Z .................................................................................................... 7700/9586
2020-02-06T07:16:59.0087416Z .................................................................................................... 7800/9586
2020-02-06T07:17:06.2059780Z .................................................................................................... 7900/9586
2020-02-06T07:17:12.5000994Z ...................................iiiiiii.i........................................................ 8000/9586
2020-02-06T07:17:24.8109228Z .................................................................................................... 8200/9586
2020-02-06T07:17:31.7888726Z .................................................................................................... 8300/9586
2020-02-06T07:17:43.7684732Z .................................................................................................... 8400/9586
2020-02-06T07:17:50.1481162Z .................................................................................................... 8500/9586
---
2020-02-06T07:19:54.4668076Z  finished in 6.622
2020-02-06T07:19:54.4849824Z Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T07:19:54.6305189Z 
2020-02-06T07:19:54.6306150Z running 172 tests
2020-02-06T07:19:57.1837126Z iiii......i...........ii..iiii...i....i...........i............i..i..................i....i......... 100/172
2020-02-06T07:19:59.2171075Z ...i.i.i...iii..iiiiiiiiii.......................iii............ii......
2020-02-06T07:19:59.2172982Z 
2020-02-06T07:19:59.2177715Z  finished in 4.732
2020-02-06T07:19:59.2357500Z Check compiletest suite=codegen-units mode=codegen-units (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T07:19:59.3784498Z 
---
2020-02-06T07:20:01.1869155Z  finished in 1.951
2020-02-06T07:20:01.2057019Z Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T07:20:01.3500507Z 
2020-02-06T07:20:01.3501196Z running 9 tests
2020-02-06T07:20:01.3502091Z iiiiiiiii
2020-02-06T07:20:01.3502866Z 
2020-02-06T07:20:01.3507449Z  finished in 0.145
2020-02-06T07:20:01.3706592Z Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T07:20:01.5200100Z 
---
2020-02-06T07:20:19.5424308Z  finished in 18.171
2020-02-06T07:20:19.5620687Z Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T07:20:19.7157204Z 
2020-02-06T07:20:19.7158086Z running 116 tests
2020-02-06T07:20:32.3634488Z iiiii..i.....i..i...i..i.i.i..i..i..ii....i.i....ii..........iiii..........i.....i..i.......ii.i.ii. 100/116
2020-02-06T07:20:34.0409895Z ....iiii.....ii.
2020-02-06T07:20:34.0410400Z 
2020-02-06T07:20:34.0423023Z  finished in 14.479
2020-02-06T07:20:34.0423692Z Uplifting stage1 rustc (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-02-06T07:20:34.0424077Z Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
---
2020-02-06T07:22:57.4734736Z failures:
2020-02-06T07:22:57.4743019Z 
2020-02-06T07:22:57.4743578Z ---- [ui] ui-fulldeps/issue-15778-fail.rs stdout ----
2020-02-06T07:22:57.4743630Z 
2020-02-06T07:22:57.4743685Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-02-06T07:22:57.4743737Z status: exit code: 101
2020-02-06T07:22:57.4744570Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-15778-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "crate-not-okay" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-A" "unused"
2020-02-06T07:22:57.4744907Z ------------------------------------------
2020-02-06T07:22:57.4744939Z 
2020-02-06T07:22:57.4745150Z ------------------------------------------
2020-02-06T07:22:57.4745211Z stderr:
2020-02-06T07:22:57.4745211Z stderr:
2020-02-06T07:22:57.4745423Z ------------------------------------------
2020-02-06T07:22:57.4745874Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4746368Z    |
2020-02-06T07:22:57.4746368Z    |
2020-02-06T07:22:57.4746413Z LL | #![plugin(lint_for_crate)]
2020-02-06T07:22:57.4746527Z    |
2020-02-06T07:22:57.4746650Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4746694Z 
2020-02-06T07:22:57.4746694Z 
2020-02-06T07:22:57.4747011Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4747126Z 
2020-02-06T07:22:57.4747339Z ------------------------------------------
2020-02-06T07:22:57.4747369Z 
2020-02-06T07:22:57.4747394Z 
2020-02-06T07:22:57.4747394Z 
2020-02-06T07:22:57.4747615Z ---- [ui] ui-fulldeps/issue-15778-pass.rs stdout ----
2020-02-06T07:22:57.4747664Z 
2020-02-06T07:22:57.4747890Z error: test compilation failed although it shouldn't!
2020-02-06T07:22:57.4747938Z status: exit code: 101
2020-02-06T07:22:57.4748694Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-15778-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "crate-not-okay" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-pass/auxiliary"
2020-02-06T07:22:57.4749018Z ------------------------------------------
2020-02-06T07:22:57.4749050Z 
2020-02-06T07:22:57.4749260Z ------------------------------------------
2020-02-06T07:22:57.4749320Z stderr:
2020-02-06T07:22:57.4749320Z stderr:
2020-02-06T07:22:57.4749741Z ------------------------------------------
2020-02-06T07:22:57.4750070Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4750378Z    |
2020-02-06T07:22:57.4750378Z    |
2020-02-06T07:22:57.4750426Z LL | #![plugin(lint_for_crate_rpass)] //~ WARNING compiler plugins are deprecated
2020-02-06T07:22:57.4750551Z    |
2020-02-06T07:22:57.4750595Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4750641Z 
2020-02-06T07:22:57.4750641Z 
2020-02-06T07:22:57.4750944Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4751056Z 
2020-02-06T07:22:57.4751265Z ------------------------------------------
2020-02-06T07:22:57.4751303Z 
2020-02-06T07:22:57.4751328Z 
2020-02-06T07:22:57.4751328Z 
2020-02-06T07:22:57.4751544Z ---- [ui] ui-fulldeps/issue-40001.rs stdout ----
2020-02-06T07:22:57.4751593Z 
2020-02-06T07:22:57.4751811Z error: test compilation failed although it shouldn't!
2020-02-06T07:22:57.4751858Z status: exit code: 101
2020-02-06T07:22:57.4752590Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/issue-40001.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-40001/auxiliary"
2020-02-06T07:22:57.4752901Z ------------------------------------------
2020-02-06T07:22:57.4752933Z 
2020-02-06T07:22:57.4753211Z ------------------------------------------
2020-02-06T07:22:57.4753275Z stderr:
2020-02-06T07:22:57.4753275Z stderr:
2020-02-06T07:22:57.4753662Z ------------------------------------------
2020-02-06T07:22:57.4754002Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4754509Z    |
2020-02-06T07:22:57.4754509Z    |
2020-02-06T07:22:57.4754557Z LL | #![plugin(issue_40001_plugin)] //~ WARNING compiler plugins are deprecated
2020-02-06T07:22:57.4754674Z    |
2020-02-06T07:22:57.4754718Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4754763Z 
2020-02-06T07:22:57.4754763Z 
2020-02-06T07:22:57.4755064Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4755168Z 
2020-02-06T07:22:57.4755391Z ------------------------------------------
2020-02-06T07:22:57.4755421Z 
2020-02-06T07:22:57.4755445Z 
2020-02-06T07:22:57.4755445Z 
2020-02-06T07:22:57.4755677Z ---- [ui] ui-fulldeps/lint-group-plugin-deny-cmdline.rs stdout ----
2020-02-06T07:22:57.4755733Z 
2020-02-06T07:22:57.4755779Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-02-06T07:22:57.4755826Z status: exit code: 101
2020-02-06T07:22:57.4756648Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "lint-me" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin-deny-cmdline/auxiliary" "-A" "unused"
2020-02-06T07:22:57.4756971Z ------------------------------------------
2020-02-06T07:22:57.4757011Z 
2020-02-06T07:22:57.4757221Z ------------------------------------------
2020-02-06T07:22:57.4757283Z stderr:
2020-02-06T07:22:57.4757283Z stderr:
2020-02-06T07:22:57.4757493Z ------------------------------------------
2020-02-06T07:22:57.4757804Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4758126Z    |
2020-02-06T07:22:57.4758126Z    |
2020-02-06T07:22:57.4758170Z LL | #![plugin(lint_group_plugin_test)]
2020-02-06T07:22:57.4758280Z    |
2020-02-06T07:22:57.4758330Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4758375Z 
2020-02-06T07:22:57.4758375Z 
2020-02-06T07:22:57.4758677Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4758797Z 
2020-02-06T07:22:57.4759007Z ------------------------------------------
2020-02-06T07:22:57.4759037Z 
2020-02-06T07:22:57.4759062Z 
2020-02-06T07:22:57.4759062Z 
2020-02-06T07:22:57.4759295Z ---- [ui] ui-fulldeps/lint-group-plugin.rs stdout ----
2020-02-06T07:22:57.4759328Z 
2020-02-06T07:22:57.4759546Z error: test compilation failed although it shouldn't!
2020-02-06T07:22:57.4759592Z status: exit code: 101
2020-02-06T07:22:57.4760382Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-group-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-plugin/auxiliary"
2020-02-06T07:22:57.4760749Z ------------------------------------------
2020-02-06T07:22:57.4760781Z 
2020-02-06T07:22:57.4761161Z ------------------------------------------
2020-02-06T07:22:57.4761224Z stderr:
2020-02-06T07:22:57.4761224Z stderr:
2020-02-06T07:22:57.4761451Z ------------------------------------------
2020-02-06T07:22:57.4761787Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4762119Z    |
2020-02-06T07:22:57.4762119Z    |
2020-02-06T07:22:57.4762178Z LL | #![plugin(lint_group_plugin_test)] //~ WARNING use of deprecated attribute
2020-02-06T07:22:57.4762304Z    |
2020-02-06T07:22:57.4762351Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4762399Z 
2020-02-06T07:22:57.4762399Z 
2020-02-06T07:22:57.4762725Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4762851Z 
2020-02-06T07:22:57.4763078Z ------------------------------------------
2020-02-06T07:22:57.4763110Z 
2020-02-06T07:22:57.4763138Z 
2020-02-06T07:22:57.4763138Z 
2020-02-06T07:22:57.4763397Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-allow.rs stdout ----
2020-02-06T07:22:57.4763433Z 
2020-02-06T07:22:57.4763670Z error: test compilation failed although it shouldn't!
2020-02-06T07:22:57.4763720Z status: exit code: 101
2020-02-06T07:22:57.4764592Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-allow/auxiliary" "-A" "unused"
2020-02-06T07:22:57.4764943Z ------------------------------------------
2020-02-06T07:22:57.4764976Z 
2020-02-06T07:22:57.4765200Z ------------------------------------------
2020-02-06T07:22:57.4765264Z stderr:
2020-02-06T07:22:57.4765264Z stderr:
2020-02-06T07:22:57.4765490Z ------------------------------------------
2020-02-06T07:22:57.4765830Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4766170Z    |
2020-02-06T07:22:57.4766170Z    |
2020-02-06T07:22:57.4766222Z LL | #![plugin(lint_plugin_test)] //~ WARNING compiler plugins are deprecated
2020-02-06T07:22:57.4766354Z    |
2020-02-06T07:22:57.4766402Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4766449Z 
2020-02-06T07:22:57.4766449Z 
2020-02-06T07:22:57.4766776Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4766892Z 
2020-02-06T07:22:57.4767120Z ------------------------------------------
2020-02-06T07:22:57.4767152Z 
2020-02-06T07:22:57.4767179Z 
2020-02-06T07:22:57.4767179Z 
2020-02-06T07:22:57.4767492Z ---- [ui] ui-fulldeps/lint-plugin-cmdline-load.rs stdout ----
2020-02-06T07:22:57.4767531Z 
2020-02-06T07:22:57.4767778Z error: test compilation failed although it shouldn't!
2020-02-06T07:22:57.4767828Z status: exit code: 101
2020-02-06T07:22:57.4768715Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-cmdline-load.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "crate-attr=plugin(lint_plugin_test)" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-cmdline-load/auxiliary" "-A" "unused"
2020-02-06T07:22:57.4769129Z ------------------------------------------
2020-02-06T07:22:57.4769162Z 
2020-02-06T07:22:57.4769390Z ------------------------------------------
2020-02-06T07:22:57.4769451Z stderr:
2020-02-06T07:22:57.4769451Z stderr:
2020-02-06T07:22:57.4769680Z ------------------------------------------
2020-02-06T07:22:57.4770031Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4770267Z   --> <crate attribute>:1:1
2020-02-06T07:22:57.4770380Z LL | plugin(lint_plugin_test)
2020-02-06T07:22:57.4770435Z    | ^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-06T07:22:57.4770483Z    |
2020-02-06T07:22:57.4770530Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4770530Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4770578Z 
2020-02-06T07:22:57.4770903Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4771027Z 
2020-02-06T07:22:57.4771254Z ------------------------------------------
2020-02-06T07:22:57.4771286Z 
2020-02-06T07:22:57.4771313Z 
2020-02-06T07:22:57.4771313Z 
2020-02-06T07:22:57.4771571Z ---- [ui] ui-fulldeps/lint-plugin-deny-attr.rs stdout ----
2020-02-06T07:22:57.4771606Z 
2020-02-06T07:22:57.4771665Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-02-06T07:22:57.4771715Z status: exit code: 101
2020-02-06T07:22:57.4772562Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-attr/auxiliary" "-A" "unused"
2020-02-06T07:22:57.4772902Z ------------------------------------------
2020-02-06T07:22:57.4772936Z 
2020-02-06T07:22:57.4773161Z ------------------------------------------
2020-02-06T07:22:57.4773233Z stderr:
2020-02-06T07:22:57.4773233Z stderr:
2020-02-06T07:22:57.4773459Z ------------------------------------------
2020-02-06T07:22:57.4773807Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4774127Z    |
2020-02-06T07:22:57.4774190Z LL | #![plugin(lint_plugin_test)]
2020-02-06T07:22:57.4774247Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-06T07:22:57.4774295Z    |
2020-02-06T07:22:57.4774295Z    |
2020-02-06T07:22:57.4774342Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4774390Z 
2020-02-06T07:22:57.4774767Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4774887Z 
2020-02-06T07:22:57.4775171Z ------------------------------------------
2020-02-06T07:22:57.4775203Z 
2020-02-06T07:22:57.4775230Z 
2020-02-06T07:22:57.4775230Z 
2020-02-06T07:22:57.4775490Z ---- [ui] ui-fulldeps/lint-plugin-deny-cmdline.rs stdout ----
2020-02-06T07:22:57.4775525Z 
2020-02-06T07:22:57.4775576Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-02-06T07:22:57.4775626Z status: exit code: 101
2020-02-06T07:22:57.4776493Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-deny-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-deny-cmdline/auxiliary" "-A" "unused"
2020-02-06T07:22:57.4776847Z ------------------------------------------
2020-02-06T07:22:57.4776880Z 
2020-02-06T07:22:57.4777104Z ------------------------------------------
2020-02-06T07:22:57.4777168Z stderr:
2020-02-06T07:22:57.4777168Z stderr:
2020-02-06T07:22:57.4777391Z ------------------------------------------
2020-02-06T07:22:57.4777740Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4778061Z    |
2020-02-06T07:22:57.4778125Z LL | #![plugin(lint_plugin_test)]
2020-02-06T07:22:57.4778187Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-06T07:22:57.4778235Z    |
2020-02-06T07:22:57.4778235Z    |
2020-02-06T07:22:57.4778300Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4778333Z 
2020-02-06T07:22:57.4778657Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4778782Z 
2020-02-06T07:22:57.4779009Z ------------------------------------------
2020-02-06T07:22:57.4779042Z 
2020-02-06T07:22:57.4779069Z 
2020-02-06T07:22:57.4779069Z 
2020-02-06T07:22:57.4779326Z ---- [ui] ui-fulldeps/lint-plugin-forbid-attrs.rs stdout ----
2020-02-06T07:22:57.4779361Z 
2020-02-06T07:22:57.4779411Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-02-06T07:22:57.4779477Z status: exit code: 101
2020-02-06T07:22:57.4780312Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-attrs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-attrs/auxiliary" "-A" "unused"
2020-02-06T07:22:57.4780659Z ------------------------------------------
2020-02-06T07:22:57.4780692Z 
2020-02-06T07:22:57.4780933Z ------------------------------------------
2020-02-06T07:22:57.4780982Z stderr:
2020-02-06T07:22:57.4780982Z stderr:
2020-02-06T07:22:57.4781206Z ------------------------------------------
2020-02-06T07:22:57.4781261Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-02-06T07:22:57.4781646Z    |
2020-02-06T07:22:57.4781646Z    |
2020-02-06T07:22:57.4781692Z LL | #![forbid(test_lint)]
2020-02-06T07:22:57.4781953Z    |           --------- `forbid` level set here
2020-02-06T07:22:57.4782097Z LL | #[allow(test_lint)]
2020-02-06T07:22:57.4782097Z LL | #[allow(test_lint)]
2020-02-06T07:22:57.4782166Z    |         ^^^^^^^^^ overruled by previous forbid
2020-02-06T07:22:57.4782199Z 
2020-02-06T07:22:57.4782248Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-02-06T07:22:57.4782585Z    |
2020-02-06T07:22:57.4782585Z    |
2020-02-06T07:22:57.4782631Z LL | #![forbid(test_lint)]
2020-02-06T07:22:57.4782863Z    |           --------- `forbid` level set here
2020-02-06T07:22:57.4782974Z LL | #[allow(test_lint)]
2020-02-06T07:22:57.4782974Z LL | #[allow(test_lint)]
2020-02-06T07:22:57.4783023Z    |         ^^^^^^^^^ overruled by previous forbid
2020-02-06T07:22:57.4783055Z 
2020-02-06T07:22:57.4783449Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4783785Z    |
2020-02-06T07:22:57.4783831Z LL | #![plugin(lint_plugin_test)]
2020-02-06T07:22:57.4783893Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-06T07:22:57.4783955Z    |
2020-02-06T07:22:57.4783955Z    |
2020-02-06T07:22:57.4784002Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4784034Z 
2020-02-06T07:22:57.4784358Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4784493Z error: aborting due to 2 previous errors
2020-02-06T07:22:57.4784524Z 
2020-02-06T07:22:57.4784801Z For more information about this error, try `rustc --explain E0453`.
2020-02-06T07:22:57.4784836Z 
2020-02-06T07:22:57.4784836Z 
2020-02-06T07:22:57.4785059Z ------------------------------------------
2020-02-06T07:22:57.4785092Z 
2020-02-06T07:22:57.4785133Z 
2020-02-06T07:22:57.4785378Z ---- [ui] ui-fulldeps/lint-plugin-forbid-cmdline.rs stdout ----
2020-02-06T07:22:57.4785421Z 
2020-02-06T07:22:57.4785472Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-02-06T07:22:57.4785535Z status: exit code: 101
2020-02-06T07:22:57.4786392Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin-forbid-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin-forbid-cmdline/auxiliary" "-A" "unused"
2020-02-06T07:22:57.4786822Z ------------------------------------------
2020-02-06T07:22:57.4786854Z 
2020-02-06T07:22:57.4787085Z ------------------------------------------
2020-02-06T07:22:57.4787129Z stderr:
2020-02-06T07:22:57.4787129Z stderr:
2020-02-06T07:22:57.4787337Z ------------------------------------------
2020-02-06T07:22:57.4787403Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-02-06T07:22:57.4787692Z    |
2020-02-06T07:22:57.4787692Z    |
2020-02-06T07:22:57.4787757Z LL | #[allow(test_lint)] //~ ERROR allow(test_lint) overruled by outer forbid(test_lint)
2020-02-06T07:22:57.4787809Z    |         ^^^^^^^^^ overruled by previous forbid
2020-02-06T07:22:57.4787910Z    = note: `forbid` lint level was set on command line
2020-02-06T07:22:57.4787989Z 
2020-02-06T07:22:57.4787989Z 
2020-02-06T07:22:57.4788198Z error[E0453]: allow(test_lint) overruled by outer forbid(test_lint)
2020-02-06T07:22:57.4788525Z    |
2020-02-06T07:22:57.4788525Z    |
2020-02-06T07:22:57.4788574Z LL | #[allow(test_lint)] //~ ERROR allow(test_lint) overruled by outer forbid(test_lint)
2020-02-06T07:22:57.4788694Z    |         ^^^^^^^^^ overruled by previous forbid
2020-02-06T07:22:57.4788793Z    = note: `forbid` lint level was set on command line
2020-02-06T07:22:57.4788824Z 
2020-02-06T07:22:57.4788824Z 
2020-02-06T07:22:57.4789157Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4789697Z    |
2020-02-06T07:22:57.4789763Z LL | #![plugin(lint_plugin_test)]
2020-02-06T07:22:57.4789824Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-06T07:22:57.4789869Z    |
2020-02-06T07:22:57.4789869Z    |
2020-02-06T07:22:57.4789929Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4789959Z 
2020-02-06T07:22:57.4790314Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4790449Z error: aborting due to 2 previous errors
2020-02-06T07:22:57.4790477Z 
2020-02-06T07:22:57.4790712Z For more information about this error, try `rustc --explain E0453`.
2020-02-06T07:22:57.4790757Z 
2020-02-06T07:22:57.4790757Z 
2020-02-06T07:22:57.4791139Z ------------------------------------------
2020-02-06T07:22:57.4791170Z 
2020-02-06T07:22:57.4791197Z 
2020-02-06T07:22:57.4791441Z ---- [ui] ui-fulldeps/lint-plugin.rs stdout ----
2020-02-06T07:22:57.4791475Z 
2020-02-06T07:22:57.4791713Z error: test compilation failed although it shouldn't!
2020-02-06T07:22:57.4791771Z status: exit code: 101
2020-02-06T07:22:57.4792543Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-plugin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-plugin/auxiliary"
2020-02-06T07:22:57.4792882Z ------------------------------------------
2020-02-06T07:22:57.4792916Z 
2020-02-06T07:22:57.4793139Z ------------------------------------------
2020-02-06T07:22:57.4793201Z stderr:
2020-02-06T07:22:57.4793201Z stderr:
2020-02-06T07:22:57.4793427Z ------------------------------------------
2020-02-06T07:22:57.4793786Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4794106Z    |
2020-02-06T07:22:57.4794106Z    |
2020-02-06T07:22:57.4794157Z LL | #![plugin(lint_plugin_test)] //~ WARNING use of deprecated attribute
2020-02-06T07:22:57.4794283Z    |
2020-02-06T07:22:57.4794330Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4794375Z 
2020-02-06T07:22:57.4794375Z 
2020-02-06T07:22:57.4794697Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4794811Z 
2020-02-06T07:22:57.4795040Z ------------------------------------------
2020-02-06T07:22:57.4795072Z 
2020-02-06T07:22:57.4795099Z 
2020-02-06T07:22:57.4795099Z 
2020-02-06T07:22:57.4795428Z ---- [ui] ui-fulldeps/lint-tool-cmdline-allow.rs stdout ----
2020-02-06T07:22:57.4795466Z 
2020-02-06T07:22:57.4795713Z error: test compilation failed although it shouldn't!
2020-02-06T07:22:57.4795764Z status: exit code: 101
2020-02-06T07:22:57.4796621Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-cmdline-allow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-A" "test-lint" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-cmdline-allow/auxiliary" "-A" "unused"
2020-02-06T07:22:57.4797026Z ------------------------------------------
2020-02-06T07:22:57.4797058Z 
2020-02-06T07:22:57.4797289Z ------------------------------------------
2020-02-06T07:22:57.4797352Z stderr:
---
2020-02-06T07:22:57.4798059Z warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
2020-02-06T07:22:57.4798109Z    |
2020-02-06T07:22:57.4798350Z    = note: requested on the command line with `-A test_lint`
2020-02-06T07:22:57.4798384Z 
2020-02-06T07:22:57.4798728Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4799055Z    |
2020-02-06T07:22:57.4799055Z    |
2020-02-06T07:22:57.4799114Z LL | #![plugin(lint_tool_test)] //~ WARNING compiler plugins are deprecated
2020-02-06T07:22:57.4799219Z    |
2020-02-06T07:22:57.4799280Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4799319Z 
2020-02-06T07:22:57.4799372Z warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
2020-02-06T07:22:57.4799372Z warning: lint name `test_lint` is deprecated and does not have an effect anymore. Use: clippy::test_lint
2020-02-06T07:22:57.4799435Z    |
2020-02-06T07:22:57.4799678Z    = note: requested on the command line with `-A test_lint`
2020-02-06T07:22:57.4799713Z 
2020-02-06T07:22:57.4800035Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4800151Z 
2020-02-06T07:22:57.4800375Z ------------------------------------------
2020-02-06T07:22:57.4800429Z 
2020-02-06T07:22:57.4800457Z 
2020-02-06T07:22:57.4800457Z 
2020-02-06T07:22:57.4800691Z ---- [ui] ui-fulldeps/lint-tool-test.rs stdout ----
2020-02-06T07:22:57.4800724Z 
2020-02-06T07:22:57.4800789Z error: Error: expected failure status (Some(1)) but received status Some(101).
2020-02-06T07:22:57.4800839Z status: exit code: 101
2020-02-06T07:22:57.4801663Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/lint-tool-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-tool-test/auxiliary" "-A" "unused"
2020-02-06T07:22:57.4802048Z ------------------------------------------
2020-02-06T07:22:57.4802099Z 
2020-02-06T07:22:57.4802334Z ------------------------------------------
2020-02-06T07:22:57.4802382Z stderr:
2020-02-06T07:22:57.4802382Z stderr:
2020-02-06T07:22:57.4802604Z ------------------------------------------
2020-02-06T07:22:57.4802946Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-02-06T07:22:57.4803332Z    |
2020-02-06T07:22:57.4803332Z    |
2020-02-06T07:22:57.4803382Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-02-06T07:22:57.4803435Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-02-06T07:22:57.4803545Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2020-02-06T07:22:57.4803578Z 
2020-02-06T07:22:57.4803578Z 
2020-02-06T07:22:57.4803915Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-02-06T07:22:57.4804238Z    |
2020-02-06T07:22:57.4804283Z LL | #![deny(clippy_group)]
2020-02-06T07:22:57.4804350Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-02-06T07:22:57.4804391Z 
2020-02-06T07:22:57.4804391Z 
2020-02-06T07:22:57.4804717Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-02-06T07:22:57.4805035Z    |
2020-02-06T07:22:57.4805035Z    |
2020-02-06T07:22:57.4805080Z LL | #[allow(test_group)]
2020-02-06T07:22:57.4805131Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-02-06T07:22:57.4805178Z 
2020-02-06T07:22:57.4805227Z warning: unknown lint: `this_lint_does_not_exist`
2020-02-06T07:22:57.4805536Z    |
2020-02-06T07:22:57.4805536Z    |
2020-02-06T07:22:57.4805597Z LL | #[deny(this_lint_does_not_exist)] //~ WARNING unknown lint: `this_lint_does_not_exist`
2020-02-06T07:22:57.4805708Z    |
2020-02-06T07:22:57.4805756Z    = note: `#[warn(unknown_lints)]` on by default
2020-02-06T07:22:57.4805795Z 
2020-02-06T07:22:57.4805795Z 
2020-02-06T07:22:57.4806120Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-02-06T07:22:57.4806440Z    |
2020-02-06T07:22:57.4806440Z    |
2020-02-06T07:22:57.4806487Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-02-06T07:22:57.4806554Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-02-06T07:22:57.4806588Z 
2020-02-06T07:22:57.4806914Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-02-06T07:22:57.4807240Z    |
2020-02-06T07:22:57.4807285Z LL | #![deny(clippy_group)]
2020-02-06T07:22:57.4807335Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-02-06T07:22:57.4807382Z 
2020-02-06T07:22:57.4807382Z 
2020-02-06T07:22:57.4807717Z warning: lint name `test_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-02-06T07:22:57.4808035Z    |
2020-02-06T07:22:57.4808035Z    |
2020-02-06T07:22:57.4808080Z LL | #[allow(test_group)]
2020-02-06T07:22:57.4808131Z    |         ^^^^^^^^^^ help: change it to: `clippy::test_group`
2020-02-06T07:22:57.4808165Z 
2020-02-06T07:22:57.4808506Z warning: use of deprecated attribute `plugin`: compiler plugins are deprecated. See ***/pull/64675
2020-02-06T07:22:57.4808825Z    |
2020-02-06T07:22:57.4808921Z LL | #![plugin(lint_tool_test)]
2020-02-06T07:22:57.4808978Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: may be removed in a future compiler version
2020-02-06T07:22:57.4809027Z    |
2020-02-06T07:22:57.4809027Z    |
2020-02-06T07:22:57.4809088Z    = note: `#[warn(deprecated)]` on by default
2020-02-06T07:22:57.4809120Z 
2020-02-06T07:22:57.4809498Z warning: lint name `test_lint` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-02-06T07:22:57.4809819Z    |
2020-02-06T07:22:57.4809819Z    |
2020-02-06T07:22:57.4809865Z LL | #![cfg_attr(foo, warn(test_lint))]
2020-02-06T07:22:57.4809932Z    |                       ^^^^^^^^^ help: change it to: `clippy::test_lint`
2020-02-06T07:22:57.4809968Z 
2020-02-06T07:22:57.4810296Z warning: lint name `clippy_group` is deprecated and may not have an effect in the future. Also `cfg_attr(cargo-clippy)` won't be necessary anymore
2020-02-06T07:22:57.4810623Z    |
2020-02-06T07:22:57.4810669Z LL | #![deny(clippy_group)]
2020-02-06T07:22:57.4810719Z    |         ^^^^^^^^^^^^ help: change it to: `clippy::group`
2020-02-06T07:22:57.4810765Z 
2020-02-06T07:22:57.4810765Z 
2020-02-06T07:22:57.4811089Z thread '<unnamed>' panicked at 'cannot access a scoped thread local variable without calling `set` first', <::std::macros::panic macros>:2:4
2020-02-06T07:22:57.4811194Z 
2020-02-06T07:22:57.4811540Z ------------------------------------------
2020-02-06T07:22:57.4811570Z 
2020-02-06T07:22:57.4811595Z 
---
2020-02-06T07:22:57.4815385Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-06T07:22:57.4815441Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-06T07:22:57.4827021Z 
2020-02-06T07:22:57.4833990Z 
2020-02-06T07:22:57.4836167Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-06T07:22:57.4836506Z 
2020-02-06T07:22:57.4836539Z 
2020-02-06T07:22:57.4839471Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-02-06T07:22:57.4839669Z Build completed unsuccessfully in 0:57:30
2020-02-06T07:22:57.4839669Z Build completed unsuccessfully in 0:57:30
2020-02-06T07:22:57.4927807Z == clock drift check ==
2020-02-06T07:22:57.4946845Z   local time: Thu Feb  6 07:22:57 UTC 2020
2020-02-06T07:22:57.7848360Z   network time: Thu, 06 Feb 2020 07:22:57 GMT
2020-02-06T07:22:57.7854966Z == end clock drift check ==
2020-02-06T07:23:01.8625791Z 
2020-02-06T07:23:01.8714823Z ##[error]Bash exited with code '1'.
2020-02-06T07:23:01.8727613Z ##[section]Finishing: Run build
2020-02-06T07:23:01.8755685Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68879/merge to s
2020-02-06T07:23:01.8757671Z Task         : Get sources
2020-02-06T07:23:01.8757721Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T07:23:01.8757786Z Version      : 1.0.0
2020-02-06T07:23:01.8757835Z Author       : Microsoft
2020-02-06T07:23:01.8757835Z Author       : Microsoft
2020-02-06T07:23:01.8757886Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-06T07:23:01.8757955Z ==============================================================================
2020-02-06T07:23:02.3168595Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-06T07:23:02.3217162Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68879/merge to s
2020-02-06T07:23:02.3315058Z Cleaning up task key
2020-02-06T07:23:02.3315779Z Start cleaning up orphan processes.
2020-02-06T07:23:02.3411701Z Terminate orphan process: pid (4961) (python)
2020-02-06T07:23:02.4142720Z ##[section]Finishing: Finalize Job
