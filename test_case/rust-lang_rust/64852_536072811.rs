plain
2019-09-27T18:30:56.7239845Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-27T18:30:56.7438574Z ##[command]git config gc.auto 0
2019-09-27T18:30:56.7514371Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-27T18:30:56.7564984Z ##[command]git config --get-all http.proxy
2019-09-27T18:30:56.7704398Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64852/merge:refs/remotes/pull/64852/merge
---
2019-09-27T19:33:07.7084173Z .................................................................................................... 1500/9047
2019-09-27T19:33:13.7538064Z .................................................................................................... 1600/9047
2019-09-27T19:33:25.7903074Z .........................................................................i...............i.......... 1700/9047
2019-09-27T19:33:32.7128327Z ........................................F........................................................... 1800/9047
2019-09-27T19:33:41.0658912Z ................................................................iiiii............................... 1900/9047
2019-09-27T19:34:00.1442470Z .................................................................................................... 2100/9047
2019-09-27T19:34:02.7205899Z .................................................................................................... 2200/9047
2019-09-27T19:34:06.0433894Z .................................................................................................... 2300/9047
2019-09-27T19:34:14.3148829Z .................................................................................................... 2400/9047
---
2019-09-27T19:37:11.9423715Z ....................................................i...............i............................... 4700/9047
2019-09-27T19:37:21.2631435Z .................................................................................................... 4800/9047
2019-09-27T19:37:29.4474347Z .................................................................................................... 4900/9047
2019-09-27T19:37:36.9138509Z .................................................................................................... 5000/9047
2019-09-27T19:37:46.6484106Z ........................................ii.ii....................................................... 5100/9047
2019-09-27T19:37:56.8782954Z .................................................................................................... 5300/9047
2019-09-27T19:38:07.5460056Z .................................................................................................... 5400/9047
2019-09-27T19:38:15.0159254Z .....i.............................................................................................. 5500/9047
2019-09-27T19:38:20.2461236Z .................................................................................................... 5600/9047
2019-09-27T19:38:20.2461236Z .................................................................................................... 5600/9047
2019-09-27T19:38:31.8128530Z .................................................................................................... 5700/9047
2019-09-27T19:38:44.9222874Z ii...i..ii...........i.............................................................................. 5800/9047
2019-09-27T19:39:07.0010937Z .................................................................................................... 6000/9047
2019-09-27T19:39:13.8133285Z .................................................................................................... 6100/9047
2019-09-27T19:39:13.8133285Z .................................................................................................... 6100/9047
2019-09-27T19:39:27.8561717Z ...i..ii............................................................................................ 6200/9047
2019-09-27T19:39:46.7029275Z ...............................................................i.................................... 6400/9047
2019-09-27T19:39:48.9306138Z .................................................................................................... 6500/9047
2019-09-27T19:39:51.9488636Z ...................................i................................................................ 6600/9047
2019-09-27T19:39:55.4849601Z .................................................................................................... 6700/9047
---
2019-09-27T19:43:56.5612221Z 
2019-09-27T19:43:56.5612263Z 34 LL |     x.await;
2019-09-27T19:43:56.5612327Z 35    |       ^^^^^
2019-09-27T19:43:56.5612370Z 36    |
2019-09-27T19:43:56.5612644Z + note: Type parameter 'impl Future<Output = ()>' was declared here
2019-09-27T19:43:56.5613483Z +   --> $DIR/suggest-switching-edition-on-await.rs:39:21
2019-09-27T19:43:56.5613526Z +    |
2019-09-27T19:43:56.5613564Z + LL | fn await_on_apit(x: impl Future<Output = ()>) {
2019-09-27T19:43:56.5613604Z +    |                     ^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-27T19:43:56.5613662Z 37    = note: to `.await` a `Future`, switch to Rust 2018
2019-09-27T19:43:56.5613703Z 38    = help: set `edition = "2018"` in `Cargo.toml`
2019-09-27T19:43:56.5613944Z 39    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
2019-09-27T19:43:56.5614022Z 
2019-09-27T19:43:56.5614061Z The actual stderr differed from the expected stderr.
2019-09-27T19:43:56.5614606Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-switching-edition-on-await/suggest-switching-edition-on-await.stderr
2019-09-27T19:43:56.5614606Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-switching-edition-on-await/suggest-switching-edition-on-await.stderr
2019-09-27T19:43:56.5614965Z To update references, rerun the tests and pass the `--bless` flag
2019-09-27T19:43:56.5615250Z To only update this specific test, also pass `--test-args async-await/suggest-switching-edition-on-await.rs`
2019-09-27T19:43:56.5615341Z error: 1 errors occurred comparing output.
2019-09-27T19:43:56.5615379Z status: exit code: 1
2019-09-27T19:43:56.5615379Z status: exit code: 1
2019-09-27T19:43:56.5616112Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/suggest-switching-edition-on-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-switching-edition-on-await" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/suggest-switching-edition-on-await/auxiliary" "-A" "unused"
2019-09-27T19:43:56.5616431Z ------------------------------------------
2019-09-27T19:43:56.5616459Z 
2019-09-27T19:43:56.5616650Z ------------------------------------------
2019-09-27T19:43:56.5616687Z stderr:
2019-09-27T19:43:56.5616687Z stderr:
2019-09-27T19:43:56.5616891Z ------------------------------------------
2019-09-27T19:43:56.5616935Z error[E0609]: no field `await` on type `await_on_struct_missing::S`
2019-09-27T19:43:56.5617156Z   --> /checkout/src/test/ui/async-await/suggest-switching-edition-on-await.rs:9:7
2019-09-27T19:43:56.5617214Z    |
2019-09-27T19:43:56.5617250Z LL |     x.await;
2019-09-27T19:43:56.5617287Z    |       ^^^^^ unknown field
2019-09-27T19:43:56.5617339Z    |
2019-09-27T19:43:56.5617379Z    = note: to `.await` a `Future`, switch to Rust 2018
2019-09-27T19:43:56.5617421Z    = help: set `edition = "2018"` in `Cargo.toml`
2019-09-27T19:43:56.5617668Z    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
2019-09-27T19:43:56.5617698Z 
2019-09-27T19:43:56.5617744Z error[E0609]: no field `await` on type `await_on_struct_similar::S`
2019-09-27T19:43:56.5618031Z    |
2019-09-27T19:43:56.5618067Z LL |     x.await;
2019-09-27T19:43:56.5618107Z    |       ^^^^^ help: a field with a similar name exists: `awai`
2019-09-27T19:43:56.5618160Z    |
2019-09-27T19:43:56.5618160Z    |
2019-09-27T19:43:56.5618198Z    = note: to `.await` a `Future`, switch to Rust 2018
2019-09-27T19:43:56.5618239Z    = help: set `edition = "2018"` in `Cargo.toml`
2019-09-27T19:43:56.5618480Z    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
2019-09-27T19:43:56.5618510Z 
2019-09-27T19:43:56.5618554Z error[E0609]: no field `await` on type `std::pin::Pin<&mut dyn std::future::Future<Output = ()>>`
2019-09-27T19:43:56.5618850Z    |
2019-09-27T19:43:56.5618886Z LL |     x.await;
2019-09-27T19:43:56.5618931Z    |       ^^^^^ unknown field
2019-09-27T19:43:56.5618984Z    |
2019-09-27T19:43:56.5618984Z    |
2019-09-27T19:43:56.5619023Z    = note: to `.await` a `Future`, switch to Rust 2018
2019-09-27T19:43:56.5619464Z    = help: set `edition = "2018"` in `Cargo.toml`
2019-09-27T19:43:56.5619843Z    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
2019-09-27T19:43:56.5619929Z error[E0609]: no field `await` on type `impl Future<Output = ()>`
2019-09-27T19:43:56.5620205Z   --> /checkout/src/test/ui/async-await/suggest-switching-edition-on-await.rs:40:7
2019-09-27T19:43:56.5620277Z    |
2019-09-27T19:43:56.5620320Z LL |     x.await;
2019-09-27T19:43:56.5620320Z LL |     x.await;
2019-09-27T19:43:56.5620364Z    |       ^^^^^
2019-09-27T19:43:56.5620425Z    |
2019-09-27T19:43:56.5620677Z note: Type parameter 'impl Future<Output = ()>' was declared here
2019-09-27T19:43:56.5621198Z    |
2019-09-27T19:43:56.5621198Z    |
2019-09-27T19:43:56.5621333Z LL | fn await_on_apit(x: impl Future<Output = ()>) {
2019-09-27T19:43:56.5621392Z    |                     ^^^^^^^^^^^^^^^^^^^^^^^^
2019-09-27T19:43:56.5621462Z    = note: to `.await` a `Future`, switch to Rust 2018
2019-09-27T19:43:56.5621516Z    = help: set `edition = "2018"` in `Cargo.toml`
2019-09-27T19:43:56.5621813Z    = note: for more on editions, read https://doc.rust-lang.org/edition-guide
2019-09-27T19:43:56.5621914Z error: aborting due to 4 previous errors
2019-09-27T19:43:56.5621945Z 
2019-09-27T19:43:56.5622204Z For more information about this error, try `rustc --explain E0609`.
2019-09-27T19:43:56.5622238Z 
---
2019-09-27T19:43:56.5623794Z 
2019-09-27T19:43:56.5623831Z 
2019-09-27T19:43:56.5623868Z The actual stderr differed from the expected stderr.
2019-09-27T19:43:56.5624121Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-30580/issue-30580.stderr
2019-09-27T19:43:56.5624337Z To update references, rerun the tests and pass the `--bless` flag
2019-09-27T19:43:56.5624566Z To only update this specific test, also pass `--test-args derived-errors/issue-30580.rs`
2019-09-27T19:43:56.5624640Z error: 1 errors occurred comparing output.
2019-09-27T19:43:56.5624695Z status: exit code: 1
2019-09-27T19:43:56.5624695Z status: exit code: 1
2019-09-27T19:43:56.5625318Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derived-errors/issue-30580.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-30580" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derived-errors/issue-30580/auxiliary" "-A" "unused"
2019-09-27T19:43:56.5625603Z ------------------------------------------
2019-09-27T19:43:56.5625631Z 
2019-09-27T19:43:56.5625835Z ------------------------------------------
2019-09-27T19:43:56.5625881Z stderr:
2019-09-27T19:43:56.5625881Z stderr:
2019-09-27T19:43:56.5626071Z ------------------------------------------
2019-09-27T19:43:56.5626133Z error[E0609]: no field `c` on type `&Foo`
2019-09-27T19:43:56.5626350Z   --> /checkout/src/test/ui/derived-errors/issue-30580.rs:12:11
2019-09-27T19:43:56.5626391Z    |
2019-09-27T19:43:56.5626432Z LL |         b.c; //~ ERROR no field `c` on type `&Foo`
2019-09-27T19:43:56.5626492Z    |           ^ help: a field with a similar name exists: `a`
2019-09-27T19:43:56.5626555Z error: aborting due to previous error
2019-09-27T19:43:56.5626579Z 
2019-09-27T19:43:56.5626807Z For more information about this error, try `rustc --explain E0609`.
2019-09-27T19:43:56.5626835Z 
2019-09-27T19:43:56.5626835Z 
2019-09-27T19:43:56.5627011Z ------------------------------------------
2019-09-27T19:43:56.5627038Z 
2019-09-27T19:43:56.5627078Z 
2019-09-27T19:43:56.5627271Z ---- [ui] ui/issues/issue-31011.rs stdout ----
2019-09-27T19:43:56.5627311Z diff of stderr:
2019-09-27T19:43:56.5627335Z 
2019-09-27T19:43:56.5627470Z 6 ...
2019-09-27T19:43:56.5627509Z 7 LL |     log!(context, "entered wrapper");
2019-09-27T19:43:56.5627870Z +    |
2019-09-27T19:43:56.5627870Z +    |
2019-09-27T19:43:56.5628088Z + note: Type parameter 'T' was declared here
2019-09-27T19:43:56.5628272Z +   --> $DIR/issue-31011.rs:16:9
2019-09-27T19:43:56.5628326Z +    |
2019-09-27T19:43:56.5628508Z + LL | fn wrap<T>(context: &T) -> ()
2019-09-27T19:43:56.5628583Z 9 
2019-09-27T19:43:56.5628638Z 10 error: aborting due to previous error
2019-09-27T19:43:56.5628674Z 11 
2019-09-27T19:43:56.5628698Z 
2019-09-27T19:43:56.5628698Z 
2019-09-27T19:43:56.5628720Z 
2019-09-27T19:43:56.5628773Z The actual stderr differed from the expected stderr.
2019-09-27T19:43:56.5629027Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31011/issue-31011.stderr
2019-09-27T19:43:56.5629693Z To update references, rerun the tests and pass the `--bless` flag
2019-09-27T19:43:56.5630001Z To only update this specific test, also pass `--test-args issues/issue-31011.rs`
2019-09-27T19:43:56.5630082Z error: 1 errors occurred comparing output.
2019-09-27T19:43:56.5630153Z status: exit code: 1
2019-09-27T19:43:56.5630153Z status: exit code: 1
2019-09-27T19:43:56.5630874Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-31011.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31011" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-31011/auxiliary" "-A" "unused"
2019-09-27T19:43:56.5631194Z ------------------------------------------
2019-09-27T19:43:56.5631226Z 
2019-09-27T19:43:56.5631436Z ------------------------------------------
2019-09-27T19:43:56.5631508Z stderr:
2019-09-27T19:43:56.5631508Z stderr:
2019-09-27T19:43:56.5631720Z ------------------------------------------
2019-09-27T19:43:56.5631769Z error[E0609]: no field `trace` on type `&T`
2019-09-27T19:43:56.5632025Z   --> /checkout/src/test/ui/issues/issue-31011.rs:3:17
2019-09-27T19:43:56.5632071Z    |
2019-09-27T19:43:56.5632114Z LL |         if $ctx.trace {
2019-09-27T19:43:56.5632218Z ...
2019-09-27T19:43:56.5632218Z ...
2019-09-27T19:43:56.5632262Z LL |     log!(context, "entered wrapper");
2019-09-27T19:43:56.5632570Z    |
2019-09-27T19:43:56.5633090Z note: Type parameter 'T' was declared here
2019-09-27T19:43:56.5633273Z   --> /checkout/src/test/ui/issues/issue-31011.rs:16:9
2019-09-27T19:43:56.5633326Z    |
2019-09-27T19:43:56.5633326Z    |
2019-09-27T19:43:56.5633493Z LL | fn wrap<T>(context: &T) -> ()
2019-09-27T19:43:56.5633571Z 
2019-09-27T19:43:56.5633613Z error: aborting due to previous error
2019-09-27T19:43:56.5633637Z 
2019-09-27T19:43:56.5633834Z For more information about this error, try `rustc --explain E0609`.
---
2019-09-27T19:43:56.5634341Z diff of stderr:
2019-09-27T19:43:56.5634364Z 
2019-09-27T19:43:56.5634542Z 2   --> $DIR/struct-pat-derived-error.rs:8:31
2019-09-27T19:43:56.5634595Z 3    |
2019-09-27T19:43:56.5634631Z 4 LL |         let A { x, y } = self.d;
2019-09-27T19:43:56.5634847Z +    |                               ^ help: a field with a similar name exists: `b`
2019-09-27T19:43:56.5634903Z 6 
2019-09-27T19:43:56.5634903Z 6 
2019-09-27T19:43:56.5634940Z 7 error[E0026]: struct `A` does not have fields named `x`, `y`
2019-09-27T19:43:56.5635280Z 
2019-09-27T19:43:56.5635301Z 
2019-09-27T19:43:56.5635338Z The actual stderr differed from the expected stderr.
2019-09-27T19:43:56.5635693Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-pat-derived-error/struct-pat-derived-error.stderr
2019-09-27T19:43:56.5635693Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-pat-derived-error/struct-pat-derived-error.stderr
2019-09-27T19:43:56.5635949Z To update references, rerun the tests and pass the `--bless` flag
2019-09-27T19:43:56.5636182Z To only update this specific test, also pass `--test-args structs/struct-pat-derived-error.rs`
2019-09-27T19:43:56.5636267Z error: 1 errors occurred comparing output.
2019-09-27T19:43:56.5636305Z status: exit code: 1
2019-09-27T19:43:56.5636305Z status: exit code: 1
2019-09-27T19:43:56.5636934Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs/struct-pat-derived-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-pat-derived-error" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/struct-pat-derived-error/auxiliary" "-A" "unused"
2019-09-27T19:43:56.5637226Z ------------------------------------------
2019-09-27T19:43:56.5637272Z 
2019-09-27T19:43:56.5637463Z ------------------------------------------
2019-09-27T19:43:56.5637500Z stderr:
2019-09-27T19:43:56.5637500Z stderr:
2019-09-27T19:43:56.5637701Z ------------------------------------------
2019-09-27T19:43:56.5637741Z error[E0609]: no field `d` on type `&A`
2019-09-27T19:43:56.5638008Z    |
2019-09-27T19:43:56.5638008Z    |
2019-09-27T19:43:56.5638052Z LL |         let A { x, y } = self.d; //~ ERROR no field `d` on type `&A`
2019-09-27T19:43:56.5638098Z    |                               ^ help: a field with a similar name exists: `b`
2019-09-27T19:43:56.5638133Z 
2019-09-27T19:43:56.5638188Z error[E0026]: struct `A` does not have fields named `x`, `y`
2019-09-27T19:43:56.5638450Z    |
2019-09-27T19:43:56.5638450Z    |
2019-09-27T19:43:56.5638507Z LL |         let A { x, y } = self.d; //~ ERROR no field `d` on type `&A`
2019-09-27T19:43:56.5638550Z    |                 ^  ^ struct `A` does not have these fields
2019-09-27T19:43:56.5638577Z 
2019-09-27T19:43:56.5638614Z error[E0027]: pattern does not mention fields `b`, `c`
2019-09-27T19:43:56.5638885Z    |
2019-09-27T19:43:56.5638885Z    |
2019-09-27T19:43:56.5638927Z LL |         let A { x, y } = self.d; //~ ERROR no field `d` on type `&A`
2019-09-27T19:43:56.5638985Z    |             ^^^^^^^^^^ missing fields `b`, `c`
2019-09-27T19:43:56.5639047Z error: aborting due to 3 previous errors
2019-09-27T19:43:56.5639080Z 
2019-09-27T19:43:56.5639137Z Some errors have detailed explanations: E0026, E0027, E0609.
2019-09-27T19:43:56.5639777Z For more information about an error, try `rustc --explain E0026`.
---
2019-09-27T19:43:56.5647148Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-09-27T19:43:56.5647381Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-27T19:43:56.5659813Z 
2019-09-27T19:43:56.5660924Z 
2019-09-27T19:43:56.5663029Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-09-27T19:43:56.5663454Z 
2019-09-27T19:43:56.5663480Z 
2019-09-27T19:43:56.5668333Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-27T19:43:56.5668426Z Build completed unsuccessfully in 1:05:52
2019-09-27T19:43:56.5668426Z Build completed unsuccessfully in 1:05:52
2019-09-27T19:43:56.5724367Z == clock drift check ==
2019-09-27T19:43:56.5740003Z   local time: Fri Sep 27 19:43:56 UTC 2019
2019-09-27T19:43:56.8562464Z   network time: Fri, 27 Sep 2019 19:43:56 GMT
2019-09-27T19:43:56.8563377Z == end clock drift check ==
2019-09-27T19:43:57.8978969Z ##[error]Bash exited with code '1'.
2019-09-27T19:43:57.9028703Z ##[section]Starting: Checkout
2019-09-27T19:43:57.9031048Z ==============================================================================
2019-09-27T19:43:57.9031127Z Task         : Get sources
2019-09-27T19:43:57.9031174Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
