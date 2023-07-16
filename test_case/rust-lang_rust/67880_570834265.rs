plain
2020-01-05T00:08:43.5229305Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-05T00:08:43.5436946Z ##[command]git config gc.auto 0
2020-01-05T00:08:43.5499319Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-05T00:08:43.5559740Z ##[command]git config --get-all http.proxy
2020-01-05T00:08:43.5701281Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67880/merge:refs/remotes/pull/67880/merge
---
2020-01-05T00:55:36.4186734Z .................................................................................................... 1500/9475
2020-01-05T00:55:41.4445846Z .................................................................................................... 1600/9475
2020-01-05T00:55:45.5775255Z .................................................................................................... 1700/9475
2020-01-05T00:55:53.5989210Z .................................................................................................... 1800/9475
2020-01-05T00:56:00.5512901Z i................................................................................................... 1900/9475
2020-01-05T00:56:06.1800574Z ........................................................................................iiiii....... 2000/9475
2020-01-05T00:56:24.6880323Z .................................................................................................... 2200/9475
2020-01-05T00:56:26.6550005Z .................................................................................................... 2300/9475
2020-01-05T00:56:28.5893636Z .................................................................................................... 2400/9475
2020-01-05T00:56:33.6507342Z .................................................................................................... 2500/9475
---
2020-01-05T00:59:05.1011910Z ....................i...............i............................................................... 4900/9475
2020-01-05T00:59:13.6803318Z .................................................................................................... 5000/9475
2020-01-05T00:59:18.7042965Z .................................................................i.................................. 5100/9475
2020-01-05T00:59:25.4615196Z .................................................................................................... 5200/9475
2020-01-05T00:59:31.8593550Z ................................ii.ii...........i................................................... 5300/9475
2020-01-05T00:59:39.7731466Z .................................................................................................... 5500/9475
2020-01-05T00:59:47.9852903Z .................................................................................................... 5600/9475
2020-01-05T00:59:53.9529387Z ................i................................................................................... 5700/9475
2020-01-05T00:59:59.1895495Z .................................................................................................... 5800/9475
2020-01-05T00:59:59.1895495Z .................................................................................................... 5800/9475
2020-01-05T01:00:08.7148940Z .................................................................................................... 5900/9475
2020-01-05T01:00:18.6789959Z .....ii...i..ii...........i......................................................................... 6000/9475
2020-01-05T01:00:33.3744599Z .................................................................................................... 6200/9475
2020-01-05T01:00:39.7840252Z .................................................................................................... 6300/9475
2020-01-05T01:00:39.7840252Z .................................................................................................... 6300/9475
2020-01-05T01:00:54.3052683Z ................................i..ii............................................................... 6400/9475
2020-01-05T01:01:11.6930166Z .................................................................................................... 6600/9475
2020-01-05T01:01:13.4442230Z .......i............................................................................................ 6700/9475
2020-01-05T01:01:15.3703604Z .................................................................................................... 6800/9475
2020-01-05T01:01:17.6152972Z ........i........................................................................................... 6900/9475
---
2020-01-05T01:02:41.2333065Z .................................................................................................... 7500/9475
2020-01-05T01:02:44.6428230Z .................................................................................................... 7600/9475
2020-01-05T01:02:49.1174982Z .................................................................................................... 7700/9475
2020-01-05T01:02:58.2072631Z .................................................................................................... 7800/9475
2020-01-05T01:03:05.0241595Z ...........................................iiii..................................................... 7900/9475
2020-01-05T01:03:17.6941222Z .................................................................................................... 8100/9475
2020-01-05T01:03:24.7242453Z .................................................................................................... 8200/9475
2020-01-05T01:03:36.4952454Z .................................................................................................... 8300/9475
2020-01-05T01:03:42.9272132Z .................................................................................................... 8400/9475
---
2020-01-05T01:05:22.4260558Z diff of stderr:
2020-01-05T01:05:22.4260668Z 
2020-01-05T01:05:22.4260781Z 8 help: the bound will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4260912Z 9    |
2020-01-05T01:05:22.4261024Z 10 LL | type SVec<T> = Vec<T>;
2020-01-05T01:05:22.4262134Z +    |           --
2020-01-05T01:05:22.4262330Z 12 
2020-01-05T01:05:22.4262483Z 13 warning: where clauses are not enforced in type aliases
2020-01-05T01:05:22.4262885Z 14   --> $DIR/type-alias-bounds.rs:10:21
2020-01-05T01:05:22.4262885Z 14   --> $DIR/type-alias-bounds.rs:10:21
2020-01-05T01:05:22.4263051Z 
2020-01-05T01:05:22.4263207Z 30 help: the bound will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4263380Z 31    |
2020-01-05T01:05:22.4263760Z 32 LL | type VVec<'b, 'a> = (&'b u32, Vec<&'a i32>);
2020-01-05T01:05:22.4264823Z +    |                --
2020-01-05T01:05:22.4265010Z 34 
2020-01-05T01:05:22.4265581Z 35 warning: bounds on generic parameters are not enforced in type aliases
2020-01-05T01:05:22.4266088Z 36   --> $DIR/type-alias-bounds.rs:14:18
2020-01-05T01:05:22.4266088Z 36   --> $DIR/type-alias-bounds.rs:14:18
2020-01-05T01:05:22.4266240Z 
2020-01-05T01:05:22.4266998Z 41 help: the bound will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4267312Z 42    |
2020-01-05T01:05:22.4268137Z 43 LL | type WVec<'b, T> = (&'b u32, Vec<T>);
2020-01-05T01:05:22.4268959Z +    |               --
2020-01-05T01:05:22.4269111Z 45 
2020-01-05T01:05:22.4269246Z 46 warning: where clauses are not enforced in type aliases
2020-01-05T01:05:22.4269550Z 47   --> $DIR/type-alias-bounds.rs:16:25
2020-01-05T01:05:22.4269550Z 47   --> $DIR/type-alias-bounds.rs:16:25
2020-01-05T01:05:22.4269688Z 
2020-01-05T01:05:22.4269792Z 
2020-01-05T01:05:22.4269931Z The actual stderr differed from the expected stderr.
2020-01-05T01:05:22.4270320Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-alias-bounds/type-alias-bounds.stderr
2020-01-05T01:05:22.4271202Z To update references, rerun the tests and pass the `--bless` flag
2020-01-05T01:05:22.4271574Z To only update this specific test, also pass `--test-args type/type-alias-bounds.rs`
2020-01-05T01:05:22.4271841Z error: 1 errors occurred comparing output.
2020-01-05T01:05:22.4271955Z status: exit code: 0
2020-01-05T01:05:22.4271955Z status: exit code: 0
2020-01-05T01:05:22.4272740Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type/type-alias-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-alias-bounds" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type/type-alias-bounds/auxiliary" "-A" "unused"
2020-01-05T01:05:22.4273243Z ------------------------------------------
2020-01-05T01:05:22.4273377Z 
2020-01-05T01:05:22.4273668Z ------------------------------------------
2020-01-05T01:05:22.4273831Z stderr:
2020-01-05T01:05:22.4273831Z stderr:
2020-01-05T01:05:22.4274140Z ------------------------------------------
2020-01-05T01:05:22.4274294Z warning: bounds on generic parameters are not enforced in type aliases
2020-01-05T01:05:22.4274624Z   --> /checkout/src/test/ui/type/type-alias-bounds.rs:8:14
2020-01-05T01:05:22.4274776Z    |
2020-01-05T01:05:22.4278877Z LL | type SVec<T: Send + Send> = Vec<T>;
2020-01-05T01:05:22.4280677Z    |
2020-01-05T01:05:22.4280737Z    = note: `#[warn(type_alias_bounds)]` on by default
2020-01-05T01:05:22.4280953Z help: the bound will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4280988Z    |
2020-01-05T01:05:22.4280988Z    |
2020-01-05T01:05:22.4281048Z LL | type SVec<T> = Vec<T>;
2020-01-05T01:05:22.4281358Z 
2020-01-05T01:05:22.4281394Z warning: where clauses are not enforced in type aliases
2020-01-05T01:05:22.4281779Z   --> /checkout/src/test/ui/type/type-alias-bounds.rs:10:21
2020-01-05T01:05:22.4281817Z    |
2020-01-05T01:05:22.4281817Z    |
2020-01-05T01:05:22.4281851Z LL | type S2Vec<T> where T: Send = Vec<T>;
2020-01-05T01:05:22.4281979Z    |
2020-01-05T01:05:22.4282017Z help: the clause will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4282072Z    |
2020-01-05T01:05:22.4282072Z    |
2020-01-05T01:05:22.4282106Z LL | type S2Vec<T>  = Vec<T>;
2020-01-05T01:05:22.4282302Z 
2020-01-05T01:05:22.4282354Z warning: bounds on generic parameters are not enforced in type aliases
2020-01-05T01:05:22.4282775Z   --> /checkout/src/test/ui/type/type-alias-bounds.rs:12:19
2020-01-05T01:05:22.4282814Z    |
2020-01-05T01:05:22.4282814Z    |
2020-01-05T01:05:22.4283150Z LL | type VVec<'b, 'a: 'b + 'b> = (&'b u32, Vec<&'a i32>);
2020-01-05T01:05:22.4283209Z    |                   ^^   ^^
2020-01-05T01:05:22.4283281Z help: the bound will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4283334Z    |
2020-01-05T01:05:22.4283334Z    |
2020-01-05T01:05:22.4283560Z LL | type VVec<'b, 'a> = (&'b u32, Vec<&'a i32>);
2020-01-05T01:05:22.4283774Z 
2020-01-05T01:05:22.4283811Z warning: bounds on generic parameters are not enforced in type aliases
2020-01-05T01:05:22.4284015Z   --> /checkout/src/test/ui/type/type-alias-bounds.rs:14:18
2020-01-05T01:05:22.4284069Z    |
2020-01-05T01:05:22.4284069Z    |
2020-01-05T01:05:22.4284259Z LL | type WVec<'b, T: 'b + 'b> = (&'b u32, Vec<T>);
2020-01-05T01:05:22.4284299Z    |                  ^^   ^^
2020-01-05T01:05:22.4284387Z help: the bound will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4284431Z    |
2020-01-05T01:05:22.4284431Z    |
2020-01-05T01:05:22.4284619Z LL | type WVec<'b, T> = (&'b u32, Vec<T>);
2020-01-05T01:05:22.4284941Z 
2020-01-05T01:05:22.4284977Z warning: where clauses are not enforced in type aliases
2020-01-05T01:05:22.4285181Z   --> /checkout/src/test/ui/type/type-alias-bounds.rs:16:25
2020-01-05T01:05:22.4285236Z    |
2020-01-05T01:05:22.4285236Z    |
2020-01-05T01:05:22.4285758Z LL | type W2Vec<'b, T> where T: 'b, T: 'b = (&'b u32, Vec<T>);
2020-01-05T01:05:22.4285848Z    |
2020-01-05T01:05:22.4285887Z help: the clause will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4285923Z    |
2020-01-05T01:05:22.4285923Z    |
2020-01-05T01:05:22.4286304Z LL | type W2Vec<'b, T>  = (&'b u32, Vec<T>);
2020-01-05T01:05:22.4286666Z 
2020-01-05T01:05:22.4286703Z warning: bounds on generic parameters are not enforced in type aliases
2020-01-05T01:05:22.4286932Z   --> /checkout/src/test/ui/type/type-alias-bounds.rs:47:12
2020-01-05T01:05:22.4287146Z    |
2020-01-05T01:05:22.4287146Z    |
2020-01-05T01:05:22.4287199Z LL | type T1<U: Bound> = U::Assoc; //~ WARN not enforced in type aliases
2020-01-05T01:05:22.4287295Z    |
2020-01-05T01:05:22.4287295Z    |
2020-01-05T01:05:22.4287335Z help: use fully disambiguated paths (i.e., `<T as Trait>::Assoc`) to refer to associated types in type aliases
2020-01-05T01:05:22.4290641Z    |
2020-01-05T01:05:22.4290641Z    |
2020-01-05T01:05:22.4290680Z LL | type T1<U: Bound> = U::Assoc; //~ WARN not enforced in type aliases
2020-01-05T01:05:22.4290782Z help: the bound will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4290820Z    |
2020-01-05T01:05:22.4290820Z    |
2020-01-05T01:05:22.4290873Z LL | type T1<U> = U::Assoc; //~ WARN not enforced in type aliases
2020-01-05T01:05:22.4291188Z 
2020-01-05T01:05:22.4291238Z warning: where clauses are not enforced in type aliases
2020-01-05T01:05:22.4291468Z   --> /checkout/src/test/ui/type/type-alias-bounds.rs:48:18
2020-01-05T01:05:22.4291515Z    |
2020-01-05T01:05:22.4291515Z    |
2020-01-05T01:05:22.4291554Z LL | type T2<U> where U: Bound = U::Assoc;  //~ WARN not enforced in type aliases
2020-01-05T01:05:22.4291642Z    |
2020-01-05T01:05:22.4291642Z    |
2020-01-05T01:05:22.4291682Z help: use fully disambiguated paths (i.e., `<T as Trait>::Assoc`) to refer to associated types in type aliases
2020-01-05T01:05:22.4291948Z    |
2020-01-05T01:05:22.4291948Z    |
2020-01-05T01:05:22.4291986Z LL | type T2<U> where U: Bound = U::Assoc;  //~ WARN not enforced in type aliases
2020-01-05T01:05:22.4292085Z help: the clause will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4292122Z    |
2020-01-05T01:05:22.4292122Z    |
2020-01-05T01:05:22.4292276Z LL | type T2<U>  = U::Assoc;  //~ WARN not enforced in type aliases
2020-01-05T01:05:22.4292541Z 
2020-01-05T01:05:22.4292579Z warning: bounds on generic parameters are not enforced in type aliases
2020-01-05T01:05:22.4292798Z   --> /checkout/src/test/ui/type/type-alias-bounds.rs:56:12
2020-01-05T01:05:22.4292836Z    |
2020-01-05T01:05:22.4292836Z    |
2020-01-05T01:05:22.4292874Z LL | type T5<U: Bound> = <U as Bound>::Assoc;  //~ WARN not enforced in type aliases
2020-01-05T01:05:22.4293022Z    |
2020-01-05T01:05:22.4293061Z help: the bound will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4293097Z    |
2020-01-05T01:05:22.4293097Z    |
2020-01-05T01:05:22.4293151Z LL | type T5<U> = <U as Bound>::Assoc;  //~ WARN not enforced in type aliases
2020-01-05T01:05:22.4293347Z 
2020-01-05T01:05:22.4293399Z warning: bounds on generic parameters are not enforced in type aliases
2020-01-05T01:05:22.4293611Z   --> /checkout/src/test/ui/type/type-alias-bounds.rs:57:12
2020-01-05T01:05:22.4293650Z    |
2020-01-05T01:05:22.4293650Z    |
2020-01-05T01:05:22.4293767Z LL | type T6<U: Bound> = ::std::vec::Vec<U>;  //~ WARN not enforced in type aliases
2020-01-05T01:05:22.4293853Z    |
2020-01-05T01:05:22.4293891Z help: the bound will not be checked when the type alias is used, and should be removed
2020-01-05T01:05:22.4293944Z    |
2020-01-05T01:05:22.4293944Z    |
2020-01-05T01:05:22.4293981Z LL | type T6<U> = ::std::vec::Vec<U>;  //~ WARN not enforced in type aliases
2020-01-05T01:05:22.4294201Z 
2020-01-05T01:05:22.4294237Z 
2020-01-05T01:05:22.4294984Z ------------------------------------------
2020-01-05T01:05:22.4295019Z 
---
2020-01-05T01:05:22.4295989Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:385:22
2020-01-05T01:05:22.4296207Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-05T01:05:22.4296895Z 
2020-01-05T01:05:22.4296927Z 
2020-01-05T01:05:22.4300011Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-7/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "7.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-01-05T01:05:22.4318293Z 
2020-01-05T01:05:22.4318318Z 
2020-01-05T01:05:22.4318702Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-05T01:05:22.4318865Z Build completed unsuccessfully in 0:50:18
2020-01-05T01:05:22.4318865Z Build completed unsuccessfully in 0:50:18
2020-01-05T01:05:22.4342801Z == clock drift check ==
2020-01-05T01:05:22.4358947Z   local time: Sun Jan  5 01:05:22 UTC 2020
2020-01-05T01:05:22.9902661Z   network time: Sun, 05 Jan 2020 01:05:22 GMT
2020-01-05T01:05:22.9904483Z == end clock drift check ==
2020-01-05T01:05:24.0764220Z 
2020-01-05T01:05:24.0843381Z ##[error]Bash exited with code '1'.
2020-01-05T01:05:24.0878020Z ##[section]Starting: Checkout
2020-01-05T01:05:24.0879468Z ==============================================================================
2020-01-05T01:05:24.0879511Z Task         : Get sources
2020-01-05T01:05:24.0879546Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
