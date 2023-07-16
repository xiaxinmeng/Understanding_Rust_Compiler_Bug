plain
2019-07-31T08:35:45.9334016Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-31T08:35:45.9542264Z ##[command]git config gc.auto 0
2019-07-31T08:35:45.9608845Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-31T08:35:45.9678991Z ##[command]git config --get-all http.proxy
2019-07-31T08:35:45.9824288Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63008/merge:refs/remotes/pull/63008/merge
---
2019-07-31T08:36:21.2742319Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T08:36:21.2742359Z 
2019-07-31T08:36:21.2742583Z   git checkout -b <new-branch-name>
2019-07-31T08:36:21.2742610Z 
2019-07-31T08:36:21.2742673Z HEAD is now at 937722a64 Merge 955d629abe965f62fa6a44f1de8263a0758c17d7 into 4a18848e05b0957474fdb5be162502742b5eb9fd
2019-07-31T08:36:21.2915244Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T08:36:21.2919393Z ==============================================================================
2019-07-31T08:36:21.2919454Z Task         : Bash
2019-07-31T08:36:21.2919501Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-31T09:37:16.3889867Z .................................................................................................... 1400/8819
2019-07-31T09:37:22.4528314Z .................................................................................................... 1500/8819
2019-07-31T09:37:35.8142898Z .................................................................i...............i.................. 1600/8819
2019-07-31T09:37:43.7375942Z .................................................................................................... 1700/8819
2019-07-31T09:37:59.3907449Z ...................................................iiiii............................................ 1800/8819
2019-07-31T09:38:11.4124689Z .................................................................................................... 2000/8819
2019-07-31T09:38:14.1611316Z .................................................................................................... 2100/8819
2019-07-31T09:38:18.0769549Z .................................................................................................... 2200/8819
2019-07-31T09:38:25.0390746Z .................................................................................................... 2300/8819
---
2019-07-31T09:42:31.0361382Z .................................................................................................... 5300/8819
2019-07-31T09:42:38.8660790Z ..............i..................................................................................... 5400/8819
2019-07-31T09:42:44.6760299Z .................................................................................................... 5500/8819
2019-07-31T09:42:57.0824609Z .................................................................................................... 5600/8819
2019-07-31T09:43:10.7733745Z ........ii...i..ii...........i...................................................................... 5700/8819
2019-07-31T09:43:27.0395727Z .................................................................................................... 5900/8819
2019-07-31T09:43:32.1205050Z .................................................................................................... 6000/8819
2019-07-31T09:43:32.1205050Z .................................................................................................... 6000/8819
2019-07-31T09:43:46.2664220Z ........i..ii....................................................................................... 6100/8819
2019-07-31T09:44:05.5927054Z ...................................................i................................................ 6300/8819
2019-07-31T09:44:07.8777528Z .................................................................................................... 6400/8819
2019-07-31T09:44:10.4096232Z .....................i.............................................................................. 6500/8819
2019-07-31T09:44:15.1330429Z .................................................................................................... 6600/8819
---
2019-07-31T09:48:18.8068826Z 
2019-07-31T09:48:18.8069380Z ---- [ui] ui/async-await/await-keyword/2015-edition-error-various-positions.rs stdout ----
2019-07-31T09:48:18.8069457Z diff of stderr:
2019-07-31T09:48:18.8069487Z 
2019-07-31T09:48:18.8069539Z 73    |     ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
2019-07-31T09:48:18.8069588Z 74    |
2019-07-31T09:48:18.8069664Z 75    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-07-31T09:48:18.8070086Z -    = note: for more information, see issue #49716 <***/issues/49716>
2019-07-31T09:48:18.8070614Z +    = note: see issue #49716 <***/issues/49716> for more information
2019-07-31T09:48:18.8070693Z 77 
2019-07-31T09:48:18.8070740Z 78 error: `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8071044Z 79   --> $DIR/2015-edition-error-various-positions.rs:35:11
2019-07-31T09:48:18.8071105Z 
2019-07-31T09:48:18.8071152Z The actual stderr differed from the expected stderr.
2019-07-31T09:48:18.8071534Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-keyword/2015-edition-error-various-positions/2015-edition-error-various-positions.stderr
2019-07-31T09:48:18.8071534Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-keyword/2015-edition-error-various-positions/2015-edition-error-various-positions.stderr
2019-07-31T09:48:18.8071785Z To update references, rerun the tests and pass the `--bless` flag
2019-07-31T09:48:18.8072086Z To only update this specific test, also pass `--test-args async-await/await-keyword/2015-edition-error-various-positions.rs`
2019-07-31T09:48:18.8072187Z error: 1 errors occurred comparing output.
2019-07-31T09:48:18.8072232Z status: exit code: 1
2019-07-31T09:48:18.8072232Z status: exit code: 1
2019-07-31T09:48:18.8073114Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/await-keyword/2015-edition-error-various-positions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-keyword/2015-edition-error-various-positions" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-keyword/2015-edition-error-various-positions/auxiliary" "-A" "unused"
2019-07-31T09:48:18.8073466Z ------------------------------------------
2019-07-31T09:48:18.8073500Z 
2019-07-31T09:48:18.8073714Z ------------------------------------------
2019-07-31T09:48:18.8073888Z stderr:
2019-07-31T09:48:18.8073888Z stderr:
2019-07-31T09:48:18.8074102Z ------------------------------------------
2019-07-31T09:48:18.8074281Z error: `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8075015Z    |
2019-07-31T09:48:18.8075015Z    |
2019-07-31T09:48:18.8075063Z LL |     pub mod await { //~ ERROR `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8075132Z    |             ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
2019-07-31T09:48:18.8075217Z note: lint level defined here
2019-07-31T09:48:18.8075473Z   --> /checkout/src/test/ui/async-await/await-keyword/2015-edition-error-various-positions.rs:3:9
2019-07-31T09:48:18.8075536Z    |
2019-07-31T09:48:18.8075536Z    |
2019-07-31T09:48:18.8075576Z LL | #![deny(keyword_idents)]
2019-07-31T09:48:18.8075618Z    |         ^^^^^^^^^^^^^^
2019-07-31T09:48:18.8075690Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-07-31T09:48:18.8075993Z    = note: see issue #49716 <***/issues/49716> for more information
2019-07-31T09:48:18.8076054Z 
2019-07-31T09:48:18.8076096Z error: `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8076427Z    |
2019-07-31T09:48:18.8076427Z    |
2019-07-31T09:48:18.8076472Z LL |         pub struct await; //~ ERROR `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8076525Z    |                    ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
2019-07-31T09:48:18.8076569Z    |
2019-07-31T09:48:18.8076639Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-07-31T09:48:18.8076923Z    = note: see issue #49716 <***/issues/49716> for more information
2019-07-31T09:48:18.8076973Z 
2019-07-31T09:48:18.8077014Z error: `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8077598Z    |
2019-07-31T09:48:18.8077598Z    |
2019-07-31T09:48:18.8077662Z LL | use outer_mod::await::await; //~ ERROR `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8077717Z    |                ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
2019-07-31T09:48:18.8077762Z    |
2019-07-31T09:48:18.8078518Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-07-31T09:48:18.8078897Z    = note: see issue #49716 <***/issues/49716> for more information
2019-07-31T09:48:18.8078932Z 
2019-07-31T09:48:18.8078995Z error: `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8079329Z    |
2019-07-31T09:48:18.8079329Z    |
2019-07-31T09:48:18.8079396Z LL | use outer_mod::await::await; //~ ERROR `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8079464Z    |                       ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
2019-07-31T09:48:18.8079518Z    |
2019-07-31T09:48:18.8079592Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-07-31T09:48:18.8079894Z    = note: see issue #49716 <***/issues/49716> for more information
2019-07-31T09:48:18.8079930Z 
2019-07-31T09:48:18.8079991Z error: `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8080321Z    |
2019-07-31T09:48:18.8080321Z    |
2019-07-31T09:48:18.8080381Z LL | struct Foo { await: () }
2019-07-31T09:48:18.8080433Z    |              ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
2019-07-31T09:48:18.8080480Z    |
2019-07-31T09:48:18.8080561Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-07-31T09:48:18.8080999Z    = note: see issue #49716 <***/issues/49716> for more information
2019-07-31T09:48:18.8081034Z 
2019-07-31T09:48:18.8081095Z error: `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8081427Z    |
2019-07-31T09:48:18.8081427Z    |
2019-07-31T09:48:18.8081487Z LL | impl Foo { fn await() {} }
2019-07-31T09:48:18.8081540Z    |               ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
2019-07-31T09:48:18.8081586Z    |
2019-07-31T09:48:18.8081659Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-07-31T09:48:18.8081956Z    = note: see issue #49716 <***/issues/49716> for more information
2019-07-31T09:48:18.8081991Z 
2019-07-31T09:48:18.8082052Z error: `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8082401Z    |
2019-07-31T09:48:18.8082401Z    |
2019-07-31T09:48:18.8082444Z LL | macro_rules! await {
2019-07-31T09:48:18.8082512Z    |              ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
2019-07-31T09:48:18.8082559Z    |
2019-07-31T09:48:18.8082614Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-07-31T09:48:18.8082931Z    = note: see issue #49716 <***/issues/49716> for more information
2019-07-31T09:48:18.8082967Z 
2019-07-31T09:48:18.8083011Z error: `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8083358Z    |
2019-07-31T09:48:18.8083358Z    |
2019-07-31T09:48:18.8083405Z LL |     await!(); //~ ERROR `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8083575Z    |     ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
2019-07-31T09:48:18.8083637Z    |
2019-07-31T09:48:18.8083692Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-07-31T09:48:18.8084042Z    = note: see issue #49716 <***/issues/49716> for more information
2019-07-31T09:48:18.8084078Z 
2019-07-31T09:48:18.8084124Z error: `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8084472Z    |
2019-07-31T09:48:18.8084472Z    |
2019-07-31T09:48:18.8084522Z LL |     match await { await => {} } //~ ERROR `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8084597Z    |           ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
2019-07-31T09:48:18.8084642Z    |
2019-07-31T09:48:18.8084697Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-07-31T09:48:18.8085032Z    = note: see issue #49716 <***/issues/49716> for more information
2019-07-31T09:48:18.8085068Z 
2019-07-31T09:48:18.8085113Z error: `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8085465Z    |
2019-07-31T09:48:18.8085465Z    |
2019-07-31T09:48:18.8085515Z LL |     match await { await => {} } //~ ERROR `await` is a keyword in the 2018 edition
2019-07-31T09:48:18.8085589Z    |                   ^^^^^ help: you can use a raw identifier to stay compatible: `r#await`
2019-07-31T09:48:18.8085635Z    |
2019-07-31T09:48:18.8085690Z    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in the 2018 edition!
2019-07-31T09:48:18.8086005Z    = note: see issue #49716 <***/issues/49716> for more information
2019-07-31T09:48:18.8086085Z error: aborting due to 10 previous errors
2019-07-31T09:48:18.8086114Z 
2019-07-31T09:48:18.8086249Z 
2019-07-31T09:48:18.8086496Z ------------------------------------------
---
2019-07-31T09:48:18.8104340Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-07-31T09:48:18.8104589Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-07-31T09:48:18.8120314Z 
2019-07-31T09:48:18.8120425Z 
2019-07-31T09:48:18.8122837Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-07-31T09:48:18.8123129Z 
2019-07-31T09:48:18.8123159Z 
2019-07-31T09:48:18.8129554Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-31T09:48:18.8129627Z Build completed unsuccessfully in 1:05:22
2019-07-31T09:48:18.8129627Z Build completed unsuccessfully in 1:05:22
2019-07-31T09:48:19.6887281Z ##[error]Bash exited with code '1'.
2019-07-31T09:48:19.6946194Z ##[section]Starting: Checkout
2019-07-31T09:48:19.6948739Z ==============================================================================
2019-07-31T09:48:19.6948800Z Task         : Get sources
2019-07-31T09:48:19.6948849Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
