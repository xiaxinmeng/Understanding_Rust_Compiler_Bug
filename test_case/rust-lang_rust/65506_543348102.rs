plain
2019-10-17T19:18:41.1116872Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-17T19:18:41.1295365Z ##[command]git config gc.auto 0
2019-10-17T19:18:41.1362562Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-17T19:18:41.1413462Z ##[command]git config --get-all http.proxy
2019-10-17T19:18:41.1540137Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65506/merge:refs/remotes/pull/65506/merge
---
2019-10-17T20:16:23.2241817Z .................................................................................................... 1600/9248
2019-10-17T20:16:28.5689507Z .................................................................................................... 1700/9248
2019-10-17T20:16:41.4496861Z ..................................i...............i................................................. 1800/9248
2019-10-17T20:16:48.8218319Z .................................................................................................... 1900/9248
2019-10-17T20:17:02.7792209Z ........................iiiii....................................................................... 2000/9248
2019-10-17T20:17:13.1634294Z .................................................................................................... 2200/9248
2019-10-17T20:17:15.5778233Z .................................................................................................... 2300/9248
2019-10-17T20:17:20.3956779Z .................................................................................................... 2400/9248
2019-10-17T20:17:42.4234765Z .................................................................................................... 2500/9248
---
2019-10-17T20:20:34.7581952Z ...........................i...............i........................................................ 4800/9248
2019-10-17T20:20:46.3553369Z .................................................................................................... 4900/9248
2019-10-17T20:20:52.5589855Z .................................................................................................... 5000/9248
2019-10-17T20:21:01.6290380Z .................................................................................................... 5100/9248
2019-10-17T20:21:09.1899472Z ...........................ii.ii.................................................................... 5200/9248
2019-10-17T20:21:18.4750643Z .................................................................................................... 5400/9248
2019-10-17T20:21:29.2142074Z .................................................................................................... 5500/9248
2019-10-17T20:21:36.6742641Z i................................................................................................... 5600/9248
2019-10-17T20:21:41.6736047Z .................................................................................................... 5700/9248
2019-10-17T20:21:41.6736047Z .................................................................................................... 5700/9248
2019-10-17T20:21:53.3075006Z .................................................................................................... 5800/9248
2019-10-17T20:22:05.9069469Z ..ii...i..ii............i........................................................................... 5900/9248
2019-10-17T20:22:27.3035693Z .................................................................................................... 6100/9248
2019-10-17T20:22:33.4415669Z .................................................................................................... 6200/9248
2019-10-17T20:22:33.4415669Z .................................................................................................... 6200/9248
2019-10-17T20:22:47.5109032Z ........................i..ii....................................................................... 6300/9248
2019-10-17T20:23:10.9230968Z ........................................FF........F.........F....................................... 6500/9248
2019-10-17T20:23:12.9660735Z ...................i................................................................................ 6600/9248
2019-10-17T20:23:15.2227829Z .............................................................................................i...... 6700/9248
2019-10-17T20:23:17.9246342Z .................................................................................................... 6800/9248
---
2019-10-17T20:27:22.3987889Z failures:
2019-10-17T20:27:22.4039498Z 
2019-10-17T20:27:22.4040154Z ---- [ui] ui/binop/binop-fail-3.rs stdout ----
2019-10-17T20:27:22.4040216Z normalized stderr:
2019-10-17T20:27:22.4041375Z warning: lint `resolve_trait_on_defaulted_unit` has been removed: `converted into hard error, see ***/issues/48950`
2019-10-17T20:27:22.4041807Z   --> $DIR/binop-fail-3.rs:7:9
2019-10-17T20:27:22.4041879Z    |
2019-10-17T20:27:22.4041923Z LL | #[allow(resolve_trait_on_defaulted_unit)]
2019-10-17T20:27:22.4042029Z    |
2019-10-17T20:27:22.4042074Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2019-10-17T20:27:22.4042106Z 
2019-10-17T20:27:22.4042339Z 
2019-10-17T20:27:22.4042339Z 
2019-10-17T20:27:22.4042374Z 
2019-10-17T20:27:22.4042416Z 
2019-10-17T20:27:22.4042460Z The actual stderr differed from the expected stderr.
2019-10-17T20:27:22.4042945Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-fail-3/binop-fail-3.stderr
2019-10-17T20:27:22.4043211Z To update references, rerun the tests and pass the `--bless` flag
2019-10-17T20:27:22.4043852Z To only update this specific test, also pass `--test-args binop/binop-fail-3.rs`
2019-10-17T20:27:22.4043961Z error: 1 errors occurred comparing output.
2019-10-17T20:27:22.4044026Z status: exit code: 0
2019-10-17T20:27:22.4044026Z status: exit code: 0
2019-10-17T20:27:22.4044742Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/binop-fail-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-fail-3/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/binop-fail-3/auxiliary" "-A" "unused"
2019-10-17T20:27:22.4045076Z ------------------------------------------
2019-10-17T20:27:22.4045110Z 
2019-10-17T20:27:22.4045344Z ------------------------------------------
2019-10-17T20:27:22.4045392Z stderr:
2019-10-17T20:27:22.4045392Z stderr:
2019-10-17T20:27:22.4045614Z ------------------------------------------
2019-10-17T20:27:22.4045983Z warning: lint `resolve_trait_on_defaulted_unit` has been removed: `converted into hard error, see ***/issues/48950`
2019-10-17T20:27:22.4046228Z   --> /checkout/src/test/ui/binop/binop-fail-3.rs:7:9
2019-10-17T20:27:22.4046278Z    |
2019-10-17T20:27:22.4046344Z LL | #[allow(resolve_trait_on_defaulted_unit)]
2019-10-17T20:27:22.4046436Z    |
2019-10-17T20:27:22.4046509Z    = note: `#[warn(renamed_and_removed_lints)]` on by default
2019-10-17T20:27:22.4046540Z 
2019-10-17T20:27:22.4046567Z 
2019-10-17T20:27:22.4046567Z 
2019-10-17T20:27:22.4046783Z ------------------------------------------
2019-10-17T20:27:22.4046834Z 
2019-10-17T20:27:22.4046860Z 
2019-10-17T20:27:22.4047428Z ---- [ui] ui/panics/doublepanic.rs stdout ----
2019-10-17T20:27:22.4047700Z normalized stderr:
2019-10-17T20:27:22.4047902Z error: expected one of `!` or `::`, found `-`
2019-10-17T20:27:22.4048079Z   --> $DIR/doublepanic.rs:1:4
2019-10-17T20:27:22.4048461Z LL | run-fail
2019-10-17T20:27:22.4048461Z LL | run-fail
2019-10-17T20:27:22.4048505Z    |    ^ expected one of `!` or `::` here
2019-10-17T20:27:22.4048570Z error: aborting due to previous error
2019-10-17T20:27:22.4048595Z 
2019-10-17T20:27:22.4048634Z 
2019-10-17T20:27:22.4048657Z 
2019-10-17T20:27:22.4048657Z 
2019-10-17T20:27:22.4048679Z 
2019-10-17T20:27:22.4048718Z The actual stderr differed from the expected stderr.
2019-10-17T20:27:22.4049184Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/doublepanic/doublepanic.stderr
2019-10-17T20:27:22.4049409Z To update references, rerun the tests and pass the `--bless` flag
2019-10-17T20:27:22.4049665Z To only update this specific test, also pass `--test-args panics/doublepanic.rs`
2019-10-17T20:27:22.4049937Z error: 1 errors occurred comparing output.
2019-10-17T20:27:22.4049980Z status: exit code: 1
2019-10-17T20:27:22.4049980Z status: exit code: 1
2019-10-17T20:27:22.4050656Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/doublepanic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/doublepanic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/doublepanic/auxiliary" "-A" "unused"
2019-10-17T20:27:22.4051214Z ------------------------------------------
2019-10-17T20:27:22.4051247Z 
2019-10-17T20:27:22.4051441Z ------------------------------------------
2019-10-17T20:27:22.4051483Z stderr:
2019-10-17T20:27:22.4051483Z stderr:
2019-10-17T20:27:22.4051690Z ------------------------------------------
2019-10-17T20:27:22.4052156Z error: expected one of `!` or `::`, found `-`
2019-10-17T20:27:22.4052676Z   --> /checkout/src/test/ui/panics/doublepanic.rs:1:4
2019-10-17T20:27:22.4053262Z LL | run-fail
2019-10-17T20:27:22.4053262Z LL | run-fail
2019-10-17T20:27:22.4053310Z    |    ^ expected one of `!` or `::` here
2019-10-17T20:27:22.4053401Z error: aborting due to previous error
2019-10-17T20:27:22.4053431Z 
2019-10-17T20:27:22.4053457Z 
2019-10-17T20:27:22.4053670Z ------------------------------------------
2019-10-17T20:27:22.4053670Z ------------------------------------------
2019-10-17T20:27:22.4053701Z 
2019-10-17T20:27:22.4053745Z 
2019-10-17T20:27:22.4053967Z ---- [ui] ui/panics/explicit-panic-msg.rs stdout ----
2019-10-17T20:27:22.4054018Z normalized stderr:
2019-10-17T20:27:22.4054257Z error: expected one of `!` or `::`, found `-`
2019-10-17T20:27:22.4054516Z    |
2019-10-17T20:27:22.4054698Z LL | run-fail
2019-10-17T20:27:22.4054698Z LL | run-fail
2019-10-17T20:27:22.4054765Z    |    ^ expected one of `!` or `::` here
2019-10-17T20:27:22.4054837Z error: aborting due to previous error
2019-10-17T20:27:22.4054866Z 
2019-10-17T20:27:22.4054910Z 
2019-10-17T20:27:22.4054936Z 
2019-10-17T20:27:22.4054936Z 
2019-10-17T20:27:22.4054970Z 
2019-10-17T20:27:22.4055015Z The actual stderr differed from the expected stderr.
2019-10-17T20:27:22.4055347Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/explicit-panic-msg/explicit-panic-msg.stderr
2019-10-17T20:27:22.4055595Z To update references, rerun the tests and pass the `--bless` flag
2019-10-17T20:27:22.4055855Z To only update this specific test, also pass `--test-args panics/explicit-panic-msg.rs`
2019-10-17T20:27:22.4055963Z error: 1 errors occurred comparing output.
2019-10-17T20:27:22.4056007Z status: exit code: 1
2019-10-17T20:27:22.4056007Z status: exit code: 1
2019-10-17T20:27:22.4056814Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/explicit-panic-msg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/explicit-panic-msg" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/explicit-panic-msg/auxiliary" "-A" "unused"
2019-10-17T20:27:22.4057361Z ------------------------------------------
2019-10-17T20:27:22.4057393Z 
2019-10-17T20:27:22.4057586Z ------------------------------------------
2019-10-17T20:27:22.4057627Z stderr:
2019-10-17T20:27:22.4057627Z stderr:
2019-10-17T20:27:22.4057832Z ------------------------------------------
2019-10-17T20:27:22.4058033Z error: expected one of `!` or `::`, found `-`
2019-10-17T20:27:22.4058311Z    |
2019-10-17T20:27:22.4058477Z LL | run-fail
2019-10-17T20:27:22.4058477Z LL | run-fail
2019-10-17T20:27:22.4058520Z    |    ^ expected one of `!` or `::` here
2019-10-17T20:27:22.4058604Z error: aborting due to previous error
2019-10-17T20:27:22.4058630Z 
2019-10-17T20:27:22.4058653Z 
2019-10-17T20:27:22.4058854Z ------------------------------------------
2019-10-17T20:27:22.4058854Z ------------------------------------------
2019-10-17T20:27:22.4058901Z 
2019-10-17T20:27:22.4058924Z 
2019-10-17T20:27:22.4059119Z ---- [ui] ui/panics/if-check-panic.rs stdout ----
2019-10-17T20:27:22.4059163Z normalized stderr:
2019-10-17T20:27:22.4059228Z error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `return`
2019-10-17T20:27:22.4059458Z    |
2019-10-17T20:27:22.4059649Z LL |         run-fail
2019-10-17T20:27:22.4059926Z    |                 - help: a semicolon may be missing here
2019-10-17T20:27:22.4059926Z    |                 - help: a semicolon may be missing here
2019-10-17T20:27:22.4060137Z LL |         return even(x - 2);
2019-10-17T20:27:22.4060225Z 
2019-10-17T20:27:22.4060267Z error[E0425]: cannot find value `run` in this scope
2019-10-17T20:27:22.4060452Z   --> $DIR/if-check-panic.rs:8:9
2019-10-17T20:27:22.4060512Z    |
---
2019-10-17T20:27:22.4061650Z 
2019-10-17T20:27:22.4061673Z 
2019-10-17T20:27:22.4061696Z 
2019-10-17T20:27:22.4061755Z The actual stderr differed from the expected stderr.
2019-10-17T20:27:22.4062022Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/if-check-panic/if-check-panic.stderr
2019-10-17T20:27:22.4062260Z To update references, rerun the tests and pass the `--bless` flag
2019-10-17T20:27:22.4062517Z To only update this specific test, also pass `--test-args panics/if-check-panic.rs`
2019-10-17T20:27:22.4062602Z error: 1 errors occurred comparing output.
2019-10-17T20:27:22.4062664Z status: exit code: 1
2019-10-17T20:27:22.4062664Z status: exit code: 1
2019-10-17T20:27:22.4063632Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/if-check-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/if-check-panic" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/if-check-panic/auxiliary" "-A" "unused"
2019-10-17T20:27:22.4063966Z ------------------------------------------
2019-10-17T20:27:22.4064002Z 
2019-10-17T20:27:22.4064232Z ------------------------------------------
2019-10-17T20:27:22.4064367Z stderr:
2019-10-17T20:27:22.4064367Z stderr:
2019-10-17T20:27:22.4064595Z ------------------------------------------
2019-10-17T20:27:22.4064670Z error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `return`
2019-10-17T20:27:22.4064907Z   --> /checkout/src/test/ui/panics/if-check-panic.rs:9:9
2019-10-17T20:27:22.4065165Z LL |         run-fail
2019-10-17T20:27:22.4065397Z    |                 - help: a semicolon may be missing here
2019-10-17T20:27:22.4065397Z    |                 - help: a semicolon may be missing here
2019-10-17T20:27:22.4065604Z LL |         return even(x - 2);
2019-10-17T20:27:22.4065707Z 
2019-10-17T20:27:22.4065753Z error[E0425]: cannot find value `run` in this scope
2019-10-17T20:27:22.4065986Z   --> /checkout/src/test/ui/panics/if-check-panic.rs:8:9
2019-10-17T20:27:22.4066050Z    |
---
2019-10-17T20:27:22.4068031Z normalized stderr:
2019-10-17T20:27:22.4068091Z warning: trait objects without an explicit `dyn` are deprecated
2019-10-17T20:27:22.4068281Z   --> $DIR/panic-macro-any.rs:7:27
2019-10-17T20:27:22.4068321Z    |
2019-10-17T20:27:22.4068385Z LL |     panic!(box 413 as Box<::std::any::Any + Send>);
2019-10-17T20:27:22.4068444Z    |                           ^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn ::std::any::Any + Send`
2019-10-17T20:27:22.4068528Z    = note: `#[warn(bare_trait_objects)]` on by default
2019-10-17T20:27:22.4068574Z 
2019-10-17T20:27:22.4068597Z 
2019-10-17T20:27:22.4068619Z 
2019-10-17T20:27:22.4068619Z 
2019-10-17T20:27:22.4068642Z 
2019-10-17T20:27:22.4068683Z The actual stderr differed from the expected stderr.
2019-10-17T20:27:22.4068979Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/panic-macro-any.stderr
2019-10-17T20:27:22.4069203Z To update references, rerun the tests and pass the `--bless` flag
2019-10-17T20:27:22.4069468Z To only update this specific test, also pass `--test-args panics/panic-macro-any.rs`
2019-10-17T20:27:22.4069545Z error: 1 errors occurred comparing output.
2019-10-17T20:27:22.4069588Z status: exit code: 0
2019-10-17T20:27:22.4069588Z status: exit code: 0
2019-10-17T20:27:22.4070276Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/panic-macro-any.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/panic-macro-any/auxiliary" "-A" "unused"
2019-10-17T20:27:22.4070588Z ------------------------------------------
2019-10-17T20:27:22.4070620Z 
2019-10-17T20:27:22.4070826Z ------------------------------------------
2019-10-17T20:27:22.4070890Z stderr:
2019-10-17T20:27:22.4070890Z stderr:
2019-10-17T20:27:22.4071093Z ------------------------------------------
2019-10-17T20:27:22.4071143Z warning: trait objects without an explicit `dyn` are deprecated
2019-10-17T20:27:22.4071579Z   --> /checkout/src/test/ui/panics/panic-macro-any.rs:7:27
2019-10-17T20:27:22.4071884Z    |
2019-10-17T20:27:22.4071925Z LL |     panic!(box 413 as Box<::std::any::Any + Send>);
2019-10-17T20:27:22.4072162Z    |                           ^^^^^^^^^^^^^^^^^^^^^^ help: use `dyn`: `dyn ::std::any::Any + Send`
2019-10-17T20:27:22.4072446Z    = note: `#[warn(bare_trait_objects)]` on by default
2019-10-17T20:27:22.4072475Z 
2019-10-17T20:27:22.4072519Z 
2019-10-17T20:27:22.4072913Z ------------------------------------------
---
2019-10-17T20:27:22.4080562Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-10-17T20:27:22.4080664Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-10-17T20:27:22.4093717Z 
2019-10-17T20:27:22.4094669Z 
2019-10-17T20:27:22.4099391Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-10-17T20:27:22.4099698Z 
2019-10-17T20:27:22.4099746Z 
2019-10-17T20:27:22.4105487Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-10-17T20:27:22.4106422Z Build completed unsuccessfully in 1:02:18
2019-10-17T20:27:22.4106422Z Build completed unsuccessfully in 1:02:18
2019-10-17T20:27:22.4165324Z == clock drift check ==
2019-10-17T20:27:22.4185517Z   local time: Thu Oct 17 20:27:22 UTC 2019
2019-10-17T20:27:22.4612453Z   network time: Thu, 17 Oct 2019 20:27:22 GMT
2019-10-17T20:27:22.4617919Z == end clock drift check ==
2019-10-17T20:27:23.2294695Z ##[error]Bash exited with code '1'.
2019-10-17T20:27:23.2332502Z ##[section]Starting: Checkout
2019-10-17T20:27:23.2334731Z ==============================================================================
2019-10-17T20:27:23.2334787Z Task         : Get sources
2019-10-17T20:27:23.2334855Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
