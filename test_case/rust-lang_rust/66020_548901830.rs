plain
2019-11-01T17:22:04.3148461Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-01T17:22:04.3339721Z ##[command]git config gc.auto 0
2019-11-01T17:22:04.3434588Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-01T17:22:04.3481486Z ##[command]git config --get-all http.proxy
2019-11-01T17:22:04.3620578Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66020/merge:refs/remotes/pull/66020/merge
---
2019-11-01T18:20:54.3610397Z .................................................................................................... 1600/9262
2019-11-01T18:21:00.0816222Z .................................................................................................... 1700/9262
2019-11-01T18:21:12.3335664Z ..........................................................i...............i......................... 1800/9262
2019-11-01T18:21:20.2337088Z .................................................................................................... 1900/9262
2019-11-01T18:21:34.6202034Z ................................................iiiii............................................... 2000/9262
2019-11-01T18:21:45.1421778Z .................................................................................................... 2200/9262
2019-11-01T18:21:47.6654783Z .................................................................................................... 2300/9262
2019-11-01T18:21:51.3375889Z .................................................................................................... 2400/9262
2019-11-01T18:22:14.2370388Z .................................................................................................... 2500/9262
---
2019-11-01T18:25:01.8775522Z ................................................i...............i................................... 4800/9262
2019-11-01T18:25:10.4736570Z .................................................................................................... 4900/9262
2019-11-01T18:25:18.9190352Z .................................................................................................... 5000/9262
2019-11-01T18:25:25.3119139Z .................................................................................................... 5100/9262
2019-11-01T18:25:35.1257568Z .................................................ii.ii...........i.................................. 5200/9262
2019-11-01T18:25:44.9536362Z .................................................................................................... 5400/9262
2019-11-01T18:25:54.8174269Z .................................................................................................... 5500/9262
2019-11-01T18:26:02.2097556Z ......................i............................................................................. 5600/9262
2019-11-01T18:26:08.6335854Z .................................................................................................... 5700/9262
2019-11-01T18:26:08.6335854Z .................................................................................................... 5700/9262
2019-11-01T18:26:20.3016906Z .................................................................................................... 5800/9262
2019-11-01T18:26:32.1932685Z .......ii...i..ii...........i....................................................................... 5900/9262
2019-11-01T18:26:53.5033212Z .................................................................................................... 6100/9262
2019-11-01T18:27:00.2319176Z .................................................................................................... 6200/9262
2019-11-01T18:27:00.2319176Z .................................................................................................... 6200/9262
2019-11-01T18:27:13.8633628Z ..........................i..ii..................................................................... 6300/9262
2019-11-01T18:27:33.7038350Z ............................................................................................i....... 6500/9262
2019-11-01T18:27:35.9582786Z .................................................................................................... 6600/9262
2019-11-01T18:27:38.1936894Z ...................................................................i................................ 6700/9262
2019-11-01T18:27:41.0867405Z .................................................................................................... 6800/9262
---
2019-11-01T18:32:25.4774100Z error: /checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs:7: expected error not found: type annotations needed
2019-11-01T18:32:25.4774323Z 
2019-11-01T18:32:25.4774494Z error: 0 unexpected errors found, 1 expected errors not found
2019-11-01T18:32:25.4774670Z status: exit code: 1
2019-11-01T18:32:25.4776007Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-overridden-binding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-overridden-binding/auxiliary" "-A" "unused"
2019-11-01T18:32:25.4776374Z not found errors (from test file): [
2019-11-01T18:32:25.4776712Z         line_num: 7,
2019-11-01T18:32:25.4776865Z         kind: Some(
2019-11-01T18:32:25.4777011Z             Error,
2019-11-01T18:32:25.4777173Z         ),
2019-11-01T18:32:25.4777173Z         ),
2019-11-01T18:32:25.4777326Z         msg: "type annotations needed",
2019-11-01T18:32:25.4777631Z ]
2019-11-01T18:32:25.4777760Z 
2019-11-01T18:32:25.4778340Z thread '[ui] ui/associated-types/associated-types-overridden-binding.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-01T18:32:25.4778775Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-01T18:32:25.4778775Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-01T18:32:25.4778926Z 
2019-11-01T18:32:25.4779375Z ---- [ui] ui/issues/issue-20831-debruijn.rs stdout ----
2019-11-01T18:32:25.4779588Z 
2019-11-01T18:32:25.4780065Z error: /checkout/src/test/ui/issues/issue-20831-debruijn.rs:28: expected error not found: mismatched types
2019-11-01T18:32:25.4780247Z 
2019-11-01T18:32:25.4780729Z error: /checkout/src/test/ui/issues/issue-20831-debruijn.rs:28: expected error not found: mismatched types
2019-11-01T18:32:25.4781078Z error: 0 unexpected errors found, 2 expected errors not found
2019-11-01T18:32:25.4781254Z status: exit code: 1
2019-11-01T18:32:25.4781254Z status: exit code: 1
2019-11-01T18:32:25.4782243Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-20831-debruijn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-20831-debruijn/auxiliary" "-A" "unused"
2019-11-01T18:32:25.4782535Z not found errors (from test file): [
2019-11-01T18:32:25.4782862Z         line_num: 28,
2019-11-01T18:32:25.4783012Z         kind: Some(
2019-11-01T18:32:25.4783160Z             Error,
2019-11-01T18:32:25.4783322Z         ),
2019-11-01T18:32:25.4783322Z         ),
2019-11-01T18:32:25.4783472Z         msg: "mismatched types",
2019-11-01T18:32:25.4783781Z     Error {
2019-11-01T18:32:25.4783949Z         line_num: 28,
2019-11-01T18:32:25.4784097Z         kind: Some(
2019-11-01T18:32:25.4784272Z             Error,
2019-11-01T18:32:25.4784272Z             Error,
2019-11-01T18:32:25.4784421Z         ),
2019-11-01T18:32:25.4784569Z         msg: "mismatched types",
2019-11-01T18:32:25.4785358Z ]
2019-11-01T18:32:25.4785489Z 
2019-11-01T18:32:25.4786061Z thread '[ui] ui/issues/issue-20831-debruijn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1520:13
2019-11-01T18:32:25.4786262Z 
2019-11-01T18:32:25.4786262Z 
2019-11-01T18:32:25.4786705Z ---- [ui] ui/suggestions/missing-assoc-type-bound-restriction.rs stdout ----
2019-11-01T18:32:25.4786933Z diff of stderr:
2019-11-01T18:32:25.4787065Z 
2019-11-01T18:32:25.4787209Z 32    |
2019-11-01T18:32:25.4787388Z 33    = note: required because of the requirements on the impl of `Child<A>` for `ChildWrapper<<T as Parent>::Assoc>`
2019-11-01T18:32:25.4787547Z 34 
2019-11-01T18:32:25.4787989Z - error[E0277]: the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-01T18:32:25.4789023Z -    |
2019-11-01T18:32:25.4789023Z -    |
2019-11-01T18:32:25.4789513Z - LL | trait Parent {
2019-11-01T18:32:25.4790556Z -    | ------------ required by `Parent`
2019-11-01T18:32:25.4791917Z - ...
2019-11-01T18:32:25.4792217Z - LL | impl<A, T: Parent<Ty = A>> Parent for ParentWrapper<T> {
2019-11-01T18:32:25.4792591Z -    |                                                       - help: consider further restricting the associated type: `where <T as Parent>::Assoc: Child<A>`
2019-11-01T18:32:25.4792809Z - ...
2019-11-01T18:32:25.4793076Z - LL |     type Assoc = ChildWrapper<T::Assoc>;
2019-11-01T18:32:25.4793400Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Child<A>` is not implemented for `<T as Parent>::Assoc`
2019-11-01T18:32:25.4793863Z - error: aborting due to 3 previous errors
2019-11-01T18:32:25.4793919Z + error: aborting due to 2 previous errors
2019-11-01T18:32:25.4793966Z 48 
2019-11-01T18:32:25.4794252Z 49 For more information about this error, try `rustc --explain E0277`.
2019-11-01T18:32:25.4794252Z 49 For more information about this error, try `rustc --explain E0277`.
2019-11-01T18:32:25.4794317Z 50 
2019-11-01T18:32:25.4794349Z 
2019-11-01T18:32:25.4794378Z 
2019-11-01T18:32:25.4794447Z The actual stderr differed from the expected stderr.
2019-11-01T18:32:25.4795016Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-type-bound-restriction/missing-assoc-type-bound-restriction.stderr
2019-11-01T18:32:25.4795344Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T18:32:25.4795738Z To only update this specific test, also pass `--test-args suggestions/missing-assoc-type-bound-restriction.rs`
2019-11-01T18:32:25.4795836Z error: 1 errors occurred comparing output.
2019-11-01T18:32:25.4795890Z status: exit code: 1
2019-11-01T18:32:25.4795890Z status: exit code: 1
2019-11-01T18:32:25.4796821Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/missing-assoc-type-bound-restriction.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-type-bound-restriction" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/missing-assoc-type-bound-restriction/auxiliary" "-A" "unused"
2019-11-01T18:32:25.4797229Z ------------------------------------------
2019-11-01T18:32:25.4797269Z 
2019-11-01T18:32:25.4797532Z ------------------------------------------
2019-11-01T18:32:25.4797605Z stderr:
2019-11-01T18:32:25.4797605Z stderr:
2019-11-01T18:32:25.4797864Z ------------------------------------------
2019-11-01T18:32:25.4797926Z error[E0277]: the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-01T18:32:25.4798323Z    |
2019-11-01T18:32:25.4798381Z LL |   trait Parent {
2019-11-01T18:32:25.4798659Z    |   ------------ required by `Parent`
2019-11-01T18:32:25.4798711Z ...
2019-11-01T18:32:25.4798711Z ...
2019-11-01T18:32:25.4798767Z LL |   impl<A, T: Parent<Ty = A>> Parent for ParentWrapper<T> {
2019-11-01T18:32:25.4799185Z    |   ^                                                     - help: consider further restricting the associated type: `where <T as Parent>::Assoc: Child<A>`
2019-11-01T18:32:25.4799253Z    |  _|
2019-11-01T18:32:25.4799302Z    | |
2019-11-01T18:32:25.4799379Z LL | |     //~^ ERROR the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-01T18:32:25.4799437Z LL | |     type Ty = A;
2019-11-01T18:32:25.4799490Z LL | |     type Assoc = ChildWrapper<T::Assoc>;
2019-11-01T18:32:25.4799569Z LL | |     //~^ ERROR the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-01T18:32:25.4799634Z LL | |     //~| ERROR the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-01T18:32:25.4799688Z LL | | }
2019-11-01T18:32:25.4799863Z    | |_^ the trait `Child<A>` is not implemented for `<T as Parent>::Assoc`
2019-11-01T18:32:25.4799910Z 
2019-11-01T18:32:25.4799966Z error[E0277]: the trait bound `<T as Parent>::Assoc: Child<A>` is not satisfied
2019-11-01T18:32:25.4800401Z    |
2019-11-01T18:32:25.4800401Z    |
2019-11-01T18:32:25.4800452Z LL |     type Assoc: Child<Self::Ty>;
2019-11-01T18:32:25.4800728Z    |          ----- associated type defined here
2019-11-01T18:32:25.4800797Z ...
2019-11-01T18:32:25.4800852Z LL | impl<A, T: Parent<Ty = A>> Parent for ParentWrapper<T> {
2019-11-01T18:32:25.4803076Z    | ------------------------------------------------------- help: consider further restricting the associated type: `where <T as Parent>::Assoc: Child<A>`
2019-11-01T18:32:25.4803540Z    | in this `impl` item
2019-11-01T18:32:25.4803587Z ...
2019-11-01T18:32:25.4803587Z ...
2019-11-01T18:32:25.4803656Z LL |     type Assoc = ChildWrapper<T::Assoc>;
2019-11-01T18:32:25.4803736Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `Child<A>` is not implemented for `<T as Parent>::Assoc`
2019-11-01T18:32:25.4803950Z    |
2019-11-01T18:32:25.4804012Z    = note: required because of the requirements on the impl of `Child<A>` for `ChildWrapper<<T as Parent>::Assoc>`
2019-11-01T18:32:25.4804125Z error: aborting due to 2 previous errors
2019-11-01T18:32:25.4804161Z 
2019-11-01T18:32:25.4804600Z For more information about this error, try `rustc --explain E0277`.
2019-11-01T18:32:25.4804662Z 
---
2019-11-01T18:32:25.4806365Z 
2019-11-01T18:32:25.4809994Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-01T18:32:25.4825864Z 
2019-11-01T18:32:25.4825965Z 
2019-11-01T18:32:25.4827690Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-01T18:32:25.4827941Z 
2019-11-01T18:32:25.4827987Z 
2019-11-01T18:32:25.4830807Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-01T18:32:25.4830882Z Build completed unsuccessfully in 1:03:45
2019-11-01T18:32:25.4830882Z Build completed unsuccessfully in 1:03:45
2019-11-01T18:32:25.4881141Z == clock drift check ==
2019-11-01T18:32:25.4906293Z   local time: Fri Nov  1 18:32:25 UTC 2019
2019-11-01T18:32:25.5804130Z   network time: Fri, 01 Nov 2019 18:32:25 GMT
2019-11-01T18:32:25.5804881Z == end clock drift check ==
2019-11-01T18:32:26.5462738Z 
2019-11-01T18:32:26.5607922Z ##[error]Bash exited with code '1'.
2019-11-01T18:32:26.5641185Z ##[section]Starting: Checkout
2019-11-01T18:32:26.5643218Z ==============================================================================
2019-11-01T18:32:26.5643302Z Task         : Get sources
2019-11-01T18:32:26.5643355Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
