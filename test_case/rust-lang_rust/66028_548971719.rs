plain
2019-11-01T21:15:14.1838931Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-01T21:15:14.2041426Z ##[command]git config gc.auto 0
2019-11-01T21:15:14.2125403Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-01T21:15:14.2175331Z ##[command]git config --get-all http.proxy
2019-11-01T21:15:14.2326116Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66028/merge:refs/remotes/pull/66028/merge
---
2019-11-01T22:16:11.2350262Z .................................................................................................... 1600/9262
2019-11-01T22:16:17.2036766Z .................................................................................................... 1700/9262
2019-11-01T22:16:29.9251292Z ..........................................................i...............i......................... 1800/9262
2019-11-01T22:16:38.1164630Z .................................................................................................... 1900/9262
2019-11-01T22:16:53.0732327Z ................................................iiiii............................................... 2000/9262
2019-11-01T22:17:04.0388546Z ................................................................................F................... 2200/9262
2019-11-01T22:17:06.6907543Z .................................................................................................... 2300/9262
2019-11-01T22:17:10.5670095Z .................................................................................................... 2400/9262
2019-11-01T22:17:34.1357076Z .................................................................................................... 2500/9262
---
2019-11-01T22:20:30.5799867Z ................................................i...............i................................... 4800/9262
2019-11-01T22:20:39.7556346Z .................................................................................................... 4900/9262
2019-11-01T22:20:48.5265267Z .................................................................................................... 5000/9262
2019-11-01T22:20:55.1269162Z .................................................................................................... 5100/9262
2019-11-01T22:21:05.5214536Z .................................................ii.ii...........i.................................. 5200/9262
2019-11-01T22:21:16.4711479Z .................................................................................................... 5400/9262
2019-11-01T22:21:26.0247861Z .................................................................................................... 5500/9262
2019-11-01T22:21:33.5872223Z ......................i............................................................................. 5600/9262
2019-11-01T22:21:40.2495171Z .................................................................................................... 5700/9262
2019-11-01T22:21:40.2495171Z .................................................................................................... 5700/9262
2019-11-01T22:21:52.2080588Z .................................................................................................... 5800/9262
2019-11-01T22:22:04.6368562Z .......ii...i..ii...........i....................................................................... 5900/9262
2019-11-01T22:22:27.3019501Z .................................................................................................... 6100/9262
2019-11-01T22:22:31.3554662Z .................................................................................................... 6200/9262
2019-11-01T22:22:31.3554662Z .................................................................................................... 6200/9262
2019-11-01T22:22:45.7138464Z ..........................i..ii..................................................................... 6300/9262
2019-11-01T22:23:06.3337519Z ............................................................................................i....... 6500/9262
2019-11-01T22:23:08.6141395Z .................................................................................................... 6600/9262
2019-11-01T22:23:10.9826894Z ...................................................................i................................ 6700/9262
2019-11-01T22:23:13.9735167Z .................................................................................................... 6800/9262
---
2019-11-01T22:28:10.7374314Z 
2019-11-01T22:28:10.7374482Z 4 LL | struct Foo;
2019-11-01T22:28:10.7374598Z 5    | ^^^^^^^^^^^
2019-11-01T22:28:10.7374703Z 6    |
2019-11-01T22:28:10.7375543Z -    = note: first defined in crate `alloc` (which `std` depends on).
2019-11-01T22:28:10.7375815Z +    = note: first defined in crate `alloc`.
2019-11-01T22:28:10.7376142Z 9 error: aborting due to previous error
2019-11-01T22:28:10.7376284Z 10 
2019-11-01T22:28:10.7376404Z 
2019-11-01T22:28:10.7376529Z 
2019-11-01T22:28:10.7376529Z 
2019-11-01T22:28:10.7377028Z The actual stderr differed from the expected stderr.
2019-11-01T22:28:10.7377559Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152/E0152.stderr
2019-11-01T22:28:10.7378034Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T22:28:10.7378677Z To only update this specific test, also pass `--test-args error-codes/E0152.rs`
2019-11-01T22:28:10.7378969Z error: 1 errors occurred comparing output.
2019-11-01T22:28:10.7379081Z status: exit code: 1
2019-11-01T22:28:10.7379081Z status: exit code: 1
2019-11-01T22:28:10.7379743Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0152.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0152/auxiliary" "-A" "unused"
2019-11-01T22:28:10.7380262Z ------------------------------------------
2019-11-01T22:28:10.7380432Z 
2019-11-01T22:28:10.7380744Z ------------------------------------------
2019-11-01T22:28:10.7380900Z stderr:
2019-11-01T22:28:10.7380900Z stderr:
2019-11-01T22:28:10.7381397Z ------------------------------------------
2019-11-01T22:28:10.7381785Z error[E0152]: duplicate lang item found: `arc`.
2019-11-01T22:28:10.7382567Z    |
2019-11-01T22:28:10.7382719Z LL | struct Foo; //~ ERROR E0152
2019-11-01T22:28:10.7382841Z    | ^^^^^^^^^^^
2019-11-01T22:28:10.7382970Z    |
---
2019-11-01T22:28:10.7384501Z 
2019-11-01T22:28:10.7384842Z ---- [ui] ui/issues/issue-21763.rs stdout ----
2019-11-01T22:28:10.7385790Z diff of stderr:
2019-11-01T22:28:10.7385926Z 
2019-11-01T22:28:10.7386106Z 10    = help: within `(std::rc::Rc<()>, std::rc::Rc<()>)`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-11-01T22:28:10.7386302Z 11    = note: required because it appears within the type `(std::rc::Rc<()>, std::rc::Rc<()>)`
2019-11-01T22:28:10.7386496Z 12    = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(std::rc::Rc<()>, std::rc::Rc<()>)>`
2019-11-01T22:28:10.7387266Z -    = note: required because it appears within the type `hashbrown::map::HashMap<std::rc::Rc<()>, std::rc::Rc<()>, std::collections::hash_map::RandomState>`
2019-11-01T22:28:10.7387510Z +    = note: required because it appears within the type `hashbrown::HashMap<std::rc::Rc<()>, std::rc::Rc<()>, std::collections::hash_map::RandomState>`
2019-11-01T22:28:10.7387685Z 14    = note: required because it appears within the type `std::collections::HashMap<std::rc::Rc<()>, std::rc::Rc<()>>`
2019-11-01T22:28:10.7388011Z 16 error: aborting due to previous error
2019-11-01T22:28:10.7388147Z 
2019-11-01T22:28:10.7388284Z 
2019-11-01T22:28:10.7388454Z The actual stderr differed from the expected stderr.
2019-11-01T22:28:10.7388454Z The actual stderr differed from the expected stderr.
2019-11-01T22:28:10.7389025Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/issue-21763.stderr
2019-11-01T22:28:10.7389367Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T22:28:10.7389732Z To only update this specific test, also pass `--test-args issues/issue-21763.rs`
2019-11-01T22:28:10.7389981Z error: 1 errors occurred comparing output.
2019-11-01T22:28:10.7390109Z status: exit code: 1
2019-11-01T22:28:10.7390109Z status: exit code: 1
2019-11-01T22:28:10.7390785Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21763.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21763/auxiliary" "-A" "unused"
2019-11-01T22:28:10.7391258Z ------------------------------------------
2019-11-01T22:28:10.7391392Z 
2019-11-01T22:28:10.7391697Z ------------------------------------------
2019-11-01T22:28:10.7391864Z stderr:
2019-11-01T22:28:10.7391864Z stderr:
2019-11-01T22:28:10.7392148Z ------------------------------------------
2019-11-01T22:28:10.7392488Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-11-01T22:28:10.7393000Z   --> /checkout/src/test/ui/issues/issue-21763.rs:9:5
2019-11-01T22:28:10.7393334Z    |
2019-11-01T22:28:10.7393485Z LL | fn foo<T: Send>() {}
2019-11-01T22:28:10.7394142Z ...
2019-11-01T22:28:10.7394142Z ...
2019-11-01T22:28:10.7394268Z LL |     foo::<HashMap<Rc<()>, Rc<()>>>();
2019-11-01T22:28:10.7394512Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-11-01T22:28:10.7394679Z    |
2019-11-01T22:28:10.7395329Z    = help: within `(std::rc::Rc<()>, std::rc::Rc<()>)`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-11-01T22:28:10.7395518Z    = note: required because it appears within the type `(std::rc::Rc<()>, std::rc::Rc<()>)`
2019-11-01T22:28:10.7395730Z    = note: required because of the requirements on the impl of `std::marker::Send` for `hashbrown::raw::RawTable<(std::rc::Rc<()>, std::rc::Rc<()>)>`
2019-11-01T22:28:10.7395932Z    = note: required because it appears within the type `hashbrown::HashMap<std::rc::Rc<()>, std::rc::Rc<()>, std::collections::hash_map::RandomState>`
2019-11-01T22:28:10.7399202Z    = note: required because it appears within the type `std::collections::HashMap<std::rc::Rc<()>, std::rc::Rc<()>>`
2019-11-01T22:28:10.7399712Z error: aborting due to previous error
2019-11-01T22:28:10.7399815Z 
2019-11-01T22:28:10.7400479Z For more information about this error, try `rustc --explain E0277`.
2019-11-01T22:28:10.7400646Z 
2019-11-01T22:28:10.7400646Z 
2019-11-01T22:28:10.7400961Z ------------------------------------------
2019-11-01T22:28:10.7401120Z 
2019-11-01T22:28:10.7401223Z 
2019-11-01T22:28:10.7401526Z ---- [ui] ui/issues/issue-43189.rs stdout ----
2019-11-01T22:28:10.7401843Z diff of stderr:
2019-11-01T22:28:10.7401948Z 
2019-11-01T22:28:10.7402093Z 7    = help: items from traits can only be used if the trait is in scope
2019-11-01T22:28:10.7402214Z 8 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-11-01T22:28:10.7402334Z 9    |
2019-11-01T22:28:10.7402897Z - LL | use xcrate_issue_43189_b::xcrate_issue_43189_a::A;
2019-11-01T22:28:10.7403058Z + LL | use xcrate_issue_43189_a::A;
2019-11-01T22:28:10.7403275Z 12 
2019-11-01T22:28:10.7403400Z 13 error: aborting due to previous error
2019-11-01T22:28:10.7403493Z 
2019-11-01T22:28:10.7403587Z 
2019-11-01T22:28:10.7403587Z 
2019-11-01T22:28:10.7403705Z The actual stderr differed from the expected stderr.
2019-11-01T22:28:10.7404079Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189/issue-43189.stderr
2019-11-01T22:28:10.7404605Z To update references, rerun the tests and pass the `--bless` flag
2019-11-01T22:28:10.7405731Z To only update this specific test, also pass `--test-args issues/issue-43189.rs`
2019-11-01T22:28:10.7406136Z error: 1 errors occurred comparing output.
2019-11-01T22:28:10.7406281Z status: exit code: 1
2019-11-01T22:28:10.7406281Z status: exit code: 1
2019-11-01T22:28:10.7407210Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-43189.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189/auxiliary" "-A" "unused"
2019-11-01T22:28:10.7407865Z ------------------------------------------
2019-11-01T22:28:10.7408035Z 
2019-11-01T22:28:10.7408404Z ------------------------------------------
2019-11-01T22:28:10.7408792Z stderr:
2019-11-01T22:28:10.7408792Z stderr:
2019-11-01T22:28:10.7409086Z ------------------------------------------
2019-11-01T22:28:10.7409242Z error[E0599]: no method named `a` found for type `()` in the current scope
2019-11-01T22:28:10.7409561Z   --> /checkout/src/test/ui/issues/issue-43189.rs:10:8
2019-11-01T22:28:10.7409739Z    |
2019-11-01T22:28:10.7409873Z LL |     ().a();
2019-11-01T22:28:10.7409998Z    |        ^ method not found in `()`
2019-11-01T22:28:10.7410221Z    = help: items from traits can only be used if the trait is in scope
2019-11-01T22:28:10.7410221Z    = help: items from traits can only be used if the trait is in scope
2019-11-01T22:28:10.7410337Z help: the following trait is implemented but not in scope; perhaps add a `use` for it:
2019-11-01T22:28:10.7410570Z    |
2019-11-01T22:28:10.7413487Z LL | use xcrate_issue_43189_a::A;
2019-11-01T22:28:10.7414012Z 
2019-11-01T22:28:10.7414129Z error: aborting due to previous error
2019-11-01T22:28:10.7414225Z 
2019-11-01T22:28:10.7414679Z For more information about this error, try `rustc --explain E0599`.
---
2019-11-01T22:28:10.7460974Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-01T22:28:10.7461155Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-01T22:28:10.7461259Z 
2019-11-01T22:28:10.7461349Z 
2019-11-01T22:28:10.7462581Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-11-01T22:28:10.7463144Z 
2019-11-01T22:28:10.7463234Z 
2019-11-01T22:28:10.7463347Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-01T22:28:10.7463471Z Build completed unsuccessfully in 1:06:00
2019-11-01T22:28:10.7463471Z Build completed unsuccessfully in 1:06:00
2019-11-01T22:28:10.7472489Z == clock drift check ==
2019-11-01T22:28:10.7489526Z   local time: Fri Nov  1 22:28:10 UTC 2019
2019-11-01T22:28:10.8213714Z   network time: Fri, 01 Nov 2019 22:28:10 GMT
2019-11-01T22:28:10.8218366Z == end clock drift check ==
2019-11-01T22:28:12.0076936Z 
2019-11-01T22:28:12.0202960Z ##[error]Bash exited with code '1'.
2019-11-01T22:28:12.0242250Z ##[section]Starting: Checkout
2019-11-01T22:28:12.0244584Z ==============================================================================
2019-11-01T22:28:12.0244650Z Task         : Get sources
2019-11-01T22:28:12.0244706Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
