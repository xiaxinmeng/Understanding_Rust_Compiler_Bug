plain
2019-11-27T19:16:57.5620558Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-27T19:16:57.5797277Z ##[command]git config gc.auto 0
2019-11-27T19:16:57.5855765Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-27T19:16:57.5906604Z ##[command]git config --get-all http.proxy
2019-11-27T19:16:57.6036405Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66821/merge:refs/remotes/pull/66821/merge
---
2019-11-27T20:09:41.8378627Z .................................................................................................... 1600/9298
2019-11-27T20:09:46.1767452Z .................................................................................................... 1700/9298
2019-11-27T20:09:57.1328916Z ...................................i................................................................ 1800/9298
2019-11-27T20:10:04.0094609Z .................................................................................................... 1900/9298
2019-11-27T20:10:16.4591341Z ....................iiiii........................................................................... 2000/9298
2019-11-27T20:10:25.6684346Z .................................................................................................... 2200/9298
2019-11-27T20:10:27.9808461Z .................................................................................................... 2300/9298
2019-11-27T20:10:32.4230824Z .................................................................................................... 2400/9298
2019-11-27T20:10:51.4645773Z .................................................................................................... 2500/9298
---
2019-11-27T20:13:14.2613217Z .....................i...............i.............................................................. 4800/9298
2019-11-27T20:13:24.2207056Z .................................................................................................... 4900/9298
2019-11-27T20:13:29.8591912Z .................................................................................................... 5000/9298
2019-11-27T20:13:37.8927626Z .................................................................................................... 5100/9298
2019-11-27T20:13:45.1380410Z ..........................ii.ii...........i......................................................... 5200/9298
2019-11-27T20:13:54.4343164Z .................................................................................................... 5400/9298
2019-11-27T20:14:04.2742458Z .................................................................................................... 5500/9298
2019-11-27T20:14:11.0760376Z ........i........................................................................................... 5600/9298
2019-11-27T20:14:16.9176314Z .................................................................................................... 5700/9298
2019-11-27T20:14:16.9176314Z .................................................................................................... 5700/9298
2019-11-27T20:14:26.3878128Z ..............................................................................................ii...i 5800/9298
2019-11-27T20:14:37.8552604Z ..ii...........i.................................................................................... 5900/9298
2019-11-27T20:14:54.2377325Z .................................................................................................... 6100/9298
2019-11-27T20:14:57.7946161Z .................................................................................................... 6200/9298
2019-11-27T20:14:57.7946161Z .................................................................................................... 6200/9298
2019-11-27T20:15:10.0717056Z .................i..ii.............................................................................. 6300/9298
2019-11-27T20:15:27.5426686Z .....................................................................................i.............. 6500/9298
2019-11-27T20:15:29.7319664Z .................................................................................................... 6600/9298
2019-11-27T20:15:31.7395685Z ............................................................................i....................... 6700/9298
2019-11-27T20:15:34.3967575Z .................................................................................................... 6800/9298
---
2019-11-27T20:19:51.1188138Z ---- [ui] ui/issues/issue-38091.rs stdout ----
2019-11-27T20:19:51.1188336Z 
2019-11-27T20:19:51.1188699Z error: test compilation failed although it shouldn't!
2019-11-27T20:19:51.1188884Z status: exit code: 101
2019-11-27T20:19:51.1189668Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-38091.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38091/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38091/auxiliary"
2019-11-27T20:19:51.1190213Z ------------------------------------------
2019-11-27T20:19:51.1190356Z 
2019-11-27T20:19:51.1190686Z ------------------------------------------
2019-11-27T20:19:51.1190837Z stderr:
2019-11-27T20:19:51.1190837Z stderr:
2019-11-27T20:19:51.1191141Z ------------------------------------------
2019-11-27T20:19:51.1191666Z error: internal compiler error: src/librustc/traits/codegen/mod.rs:127: Encountered errors `[FulfillmentError(Obligation(predicate=Binder(TraitPredicate(<() as Valid>)), depth=2),Unimplemented)]` resolving bounds after type-checking
2019-11-27T20:19:51.1192175Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:892:9
2019-11-27T20:19:51.1192346Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-27T20:19:51.1192454Z 
2019-11-27T20:19:51.1192587Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-27T20:19:51.1192587Z note: the compiler unexpectedly panicked. this is a bug.
2019-11-27T20:19:51.1192688Z 
2019-11-27T20:19:51.1193199Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2019-11-27T20:19:51.1193710Z note: rustc 1.41.0-dev running on x86_64-unknown-linux-gnu
2019-11-27T20:19:51.1193851Z 
2019-11-27T20:19:51.1193851Z 
2019-11-27T20:19:51.1194230Z note: compiler flags: -Z threads=1 -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
2019-11-27T20:19:51.1194510Z error: aborting due to previous error
2019-11-27T20:19:51.1194609Z 
2019-11-27T20:19:51.1194721Z 
2019-11-27T20:19:51.1195026Z ------------------------------------------
2019-11-27T20:19:51.1195026Z ------------------------------------------
2019-11-27T20:19:51.1195161Z 
2019-11-27T20:19:51.1195259Z 
2019-11-27T20:19:51.1195597Z ---- [ui] ui/type-alias-impl-trait/bound_reduction2.rs stdout ----
2019-11-27T20:19:51.1195764Z diff of stderr:
2019-11-27T20:19:51.1195866Z 
2019-11-27T20:19:51.1196004Z + error[E0277]: the trait bound `T: TraitWithAssoc` is not satisfied
2019-11-27T20:19:51.1196312Z +   --> $DIR/bound_reduction2.rs:10:1
2019-11-27T20:19:51.1196478Z +    |
2019-11-27T20:19:51.1196615Z + LL | type Foo<V> = impl Trait<V>;
2019-11-27T20:19:51.1196737Z +    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `TraitWithAssoc` is not implemented for `T`
2019-11-27T20:19:51.1196872Z + ...
2019-11-27T20:19:51.1197202Z + LL | fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> {
2019-11-27T20:19:51.1197593Z +    |                  -- help: consider further restricting this bound: `T: TraitWithAssoc +`
2019-11-27T20:19:51.1197763Z + 
2019-11-27T20:19:51.1197895Z 1 error: defining opaque type use does not fully define opaque type: generic parameter `V` is specified as concrete type `<T as TraitWithAssoc>::Assoc`
2019-11-27T20:19:51.1198367Z 3    |
2019-11-27T20:19:51.1198602Z 
2019-11-27T20:19:51.1198602Z 
2019-11-27T20:19:51.1198768Z 12 LL | type Foo<V> = impl Trait<V>;
2019-11-27T20:19:51.1199020Z 14 
2019-11-27T20:19:51.1199502Z - error: aborting due to 2 previous errors
2019-11-27T20:19:51.1199956Z + error: aborting due to 3 previous errors
2019-11-27T20:19:51.1200083Z 16 
2019-11-27T20:19:51.1200083Z 16 
2019-11-27T20:19:51.1200473Z + For more information about this error, try `rustc --explain E0277`.
2019-11-27T20:19:51.1200994Z 17 
2019-11-27T20:19:51.1201127Z 
2019-11-27T20:19:51.1201280Z 
2019-11-27T20:19:51.1201443Z The actual stderr differed from the expected stderr.
2019-11-27T20:19:51.1201935Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bound_reduction2/bound_reduction2.stderr
2019-11-27T20:19:51.1202352Z To update references, rerun the tests and pass the `--bless` flag
2019-11-27T20:19:51.1202814Z To only update this specific test, also pass `--test-args type-alias-impl-trait/bound_reduction2.rs`
2019-11-27T20:19:51.1203322Z error: 1 errors occurred comparing output.
2019-11-27T20:19:51.1203455Z status: exit code: 1
2019-11-27T20:19:51.1203455Z status: exit code: 1
2019-11-27T20:19:51.1204258Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/bound_reduction2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bound_reduction2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bound_reduction2/auxiliary" "-A" "unused"
2019-11-27T20:19:51.1204803Z ------------------------------------------
2019-11-27T20:19:51.1204963Z 
2019-11-27T20:19:51.1205497Z ------------------------------------------
2019-11-27T20:19:51.1208982Z stderr:
2019-11-27T20:19:51.1208982Z stderr:
2019-11-27T20:19:51.1209440Z ------------------------------------------
2019-11-27T20:19:51.1209620Z error[E0277]: the trait bound `T: TraitWithAssoc` is not satisfied
2019-11-27T20:19:51.1210169Z    |
2019-11-27T20:19:51.1210169Z    |
2019-11-27T20:19:51.1210288Z LL | type Foo<V> = impl Trait<V>;
2019-11-27T20:19:51.1210424Z    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `TraitWithAssoc` is not implemented for `T`
2019-11-27T20:19:51.1210545Z ...
2019-11-27T20:19:51.1211001Z LL | fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> { //~ ERROR does not fully define
2019-11-27T20:19:51.1211416Z    |                  -- help: consider further restricting this bound: `T: TraitWithAssoc +`
2019-11-27T20:19:51.1211677Z 
2019-11-27T20:19:51.1211823Z error: defining opaque type use does not fully define opaque type: generic parameter `V` is specified as concrete type `<T as TraitWithAssoc>::Assoc`
2019-11-27T20:19:51.1212389Z    |
2019-11-27T20:19:51.1212389Z    |
2019-11-27T20:19:51.1212759Z LL | / fn foo_desugared<T: TraitWithAssoc>(_: T) -> Foo<T::Assoc> { //~ ERROR does not fully define
2019-11-27T20:19:51.1213181Z LL | | }
2019-11-27T20:19:51.1213369Z    | |_^
2019-11-27T20:19:51.1213542Z 
2019-11-27T20:19:51.1213708Z error: could not find defining uses
2019-11-27T20:19:51.1213708Z error: could not find defining uses
2019-11-27T20:19:51.1214125Z   --> /checkout/src/test/ui/type-alias-impl-trait/bound_reduction2.rs:10:1
2019-11-27T20:19:51.1214789Z    |
2019-11-27T20:19:51.1214833Z LL | type Foo<V> = impl Trait<V>;
2019-11-27T20:19:51.1214914Z 
2019-11-27T20:19:51.1214951Z error: aborting due to 3 previous errors
2019-11-27T20:19:51.1214974Z 
2019-11-27T20:19:51.1215276Z For more information about this error, try `rustc --explain E0277`.
---
2019-11-27T20:19:51.1227119Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-27T20:19:51.1227187Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-27T20:19:51.1241992Z 
2019-11-27T20:19:51.1242064Z 
2019-11-27T20:19:51.1243586Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-27T20:19:51.1243810Z 
2019-11-27T20:19:51.1243852Z 
2019-11-27T20:19:51.8053599Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-27T20:19:51.8054502Z Build completed unsuccessfully in 0:57:15
2019-11-27T20:19:51.8054502Z Build completed unsuccessfully in 0:57:15
2019-11-27T20:19:51.8054782Z == clock drift check ==
2019-11-27T20:19:51.8054933Z   local time: Wed Nov 27 20:19:51 UTC 2019
2019-11-27T20:19:51.8055095Z   network time: Wed, 27 Nov 2019 20:19:51 GMT
2019-11-27T20:19:51.8055230Z == end clock drift check ==
2019-11-27T20:19:52.0091745Z 
2019-11-27T20:19:52.0211529Z ##[error]Bash exited with code '1'.
2019-11-27T20:19:52.0245442Z ##[section]Starting: Checkout
2019-11-27T20:19:52.0247240Z ==============================================================================
2019-11-27T20:19:52.0247314Z Task         : Get sources
2019-11-27T20:19:52.0247363Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
