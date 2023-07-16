plain
.................................................................................................... 100/12115
.............................................ii...........ii........................................ 200/12115
.................................................................................................... 300/12115
.................................................................................................... 400/12115
................................................................................................F... 500/12115
F.......F....F.FFFF.F........F..FF....FF...................F..................F.......F............. 600/12115
....FF..........F.F..........F..............F..i.................................................... 700/12115
.................................................................................................... 900/12115
.................................................................................................... 1000/12115
.................................................................................................... 1100/12115
.....................................i.............................................................. 1200/12115
---
.......................i........i.........i......................................................... 3600/12115
..........................................................................................ii........ 3700/12115
.................................................................................................... 3800/12115
........i............................................i.............................................. 3900/12115
............................F..............................................................F........ 4000/12115
.................................................................................................... 4200/12115
.................................................................................................... 4300/12115
.................................................................................................... 4400/12115
.................................................................................................... 4500/12115
.................................................................................................... 4500/12115
......................................................ii............................................ 4600/12115
....................................................i........F...........F.......................... 4700/12115
.................................................................................................... 4900/12115
.................................................................................................... 5000/12115
.................................................................................................... 5100/12115
.................................................................................................... 5200/12115
---
failures:

---- [ui] ui/async-await/async-fn-send-uses-nonsend.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-send-uses-nonsend.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-send-uses-nonsend/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-send-uses-nonsend.rs:17:16: 17:18]> in MIR, but typeck only knows about {ResumeTy, impl Future, (), impl Debug, impl Future, Option<impl Debug>} and []
   |
   |
LL |   async fn still_send() {
   |  _______________________^
LL | |     fut().await;
LL | |     println!("{:?} {:?}", non_send(), non_sync());
LL | |     fut().await;
...  |
LL | |     fut().await;
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `still_send::{closure#0}`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/async-fn-size-uninit-locals.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-size-uninit-locals.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size-uninit-locals/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size-uninit-locals/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-size-uninit-locals.rs:47:16: 47:18]> in MIR, but typeck only knows about {ResumeTy, Big, impl Future, ()} and []
   |
   |
LL |   async fn single() {
LL | |     let x;
LL | |     let x;
LL | |     fut().await;
LL | |     x = Big::new();
LL | | }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `single::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/async-fn-size-uninit-locals.rs:49:19: 53:2]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/async-closure.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure.default/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-closure.rs:65:46: 68:2]> in MIR, but typeck only knows about {ResumeTy, u8, impl Future, ()} and [u8]
   |
   |
LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `async_closure_in_unsafe_block::{closure#1}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/async-closure.rs:61:28: 61:63]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/async-await.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.default/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-await.rs:69:28: 72:10]> in MIR, but typeck only knows about {ResumeTy, impl Future, ()} and [u8]
   |
   |
LL |       async move {
LL | |         let future = async {
LL | |         let future = async {
LL | |             wake_and_yield_once().await;
LL | |             x
LL | |         };
LL | |         future.await
LL | |     }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `async_nonmove_block::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/async-await.rs:68:16: 74:6]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/async-closure.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure.nomiropt/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=0" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-closure.nomiropt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-closure.rs:65:46: 68:2]> in MIR, but typeck only knows about {ResumeTy, u8, impl Future, ()} and [u8]
   |
   |
LL |         async move |x: u8| unsafe_fn(unsafe_async_fn(x).await)

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z mir-opt-level=0 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `async_closure_in_unsafe_block::{closure#1}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/async-closure.rs:61:28: 61:63]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/conditional-and-guaranteed-initialization.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/conditional-and-guaranteed-initialization.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/conditional-and-guaranteed-initialization" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/conditional-and-guaranteed-initialization/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/conditional-and-guaranteed-initialization.rs:15:34: 15:39]> in MIR, but typeck only knows about {ResumeTy, usize, impl Future, (), impl Future} and [usize]
   |
   |
LL |   async fn conditional_and_guaranteed_initialization(x: usize) -> usize {
LL | |     let y;
LL | |     let y;
LL | |     if x > 5 {
LL | |         y = echo(10).await;
LL | |     y
LL | | }
   | |_^

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `conditional_and_guaranteed_initialization::{closure#0}`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/async-await.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.nomiropt/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=0" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.nomiropt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-await.rs:69:28: 72:10]> in MIR, but typeck only knows about {ResumeTy, impl Future, ()} and [u8]
   |
   |
LL |       async move {
LL | |         let future = async {
LL | |         let future = async {
LL | |             wake_and_yield_once().await;
LL | |             x
LL | |         };
LL | |         future.await
LL | |     }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z mir-opt-level=0 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `async_nonmove_block::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/async-await.rs:68:16: 74:6]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/async-await.rs#thirunsafeck stdout ----

error in revision `thirunsafeck`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-await.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zthir-unsafeck" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-await.thirunsafeck/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-await.rs:69:28: 72:10]> in MIR, but typeck only knows about {ResumeTy, impl Future, ()} and [u8]
   |
   |
LL |       async move {
LL | |         let future = async {
LL | |         let future = async {
LL | |             wake_and_yield_once().await;
LL | |             x
LL | |         };
LL | |         future.await
LL | |     }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z thir-unsafeck -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `async_nonmove_block::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/async-await.rs:68:16: 74:6]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/async-fn-size.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/async-fn-size.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/async-fn-size/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/async-fn-size.rs:67:32: 69:2]> in MIR, but typeck only knows about {ResumeTy, impl Future, (), u8} and []
   |
   |
LL |   async fn await3_level2() -> u8 {
   |  ________________________________^
LL | |     await3_level1().await + await3_level1().await + await3_level1().await
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `await3_level2::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/async-fn-size.rs:71:32: 73:2]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/issue-61793.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-61793.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61793" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-61793/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issue-61793.rs:9:31: 9:33]> in MIR, but typeck only knows about {ResumeTy, (), &(), [closure@/checkout/src/test/ui/async-await/issue-61793.rs:14:18: 14:23], impl Future} and []
   |
LL |       async {
   |  ___________^
   |  ___________^
LL | |         foo(&(), || {}).await;
LL | |     };

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main::{closure#1}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/issue-61793.rs:13:11: 15:6]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/issue-62658.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-62658.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-62658" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-62658/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issue-62658.rs:7:17: 7:19]> in MIR, but typeck only knows about {ResumeTy, [u8; 17], impl Future, (), u64} and []
   |
   |
LL |   async fn foo() {
   |  ________________^
LL | |     // This suspend should be the largest variant.
LL | |     {
LL | |         let x = [0u8; 17];
LL | |     }
LL | | }
   | |_^

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `foo::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/issue-62658.rs:9:16: 23:2]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.default/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs:60:30: 60:33]> in MIR, but typeck only knows about {ResumeTy, D, Rc<RefCell<Vec<DropOrder>>>, &str, &D, impl Future, ()} and [D, D]
   |
   |
LL |   async fn foo_async(x: D, _y: D) {
   |  _________________________________^
LL | |     let l = x.1.clone();
LL | |     let z = D("z", l.clone());
LL | |     l.borrow_mut().push(DropOrder::Function);
LL | |     helper_async(&D("temp", l)).await
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `foo_async::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs:53:33: 58:2]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.nomiropt/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-Z" "mir-opt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.nomiropt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs:60:30: 60:33]> in MIR, but typeck only knows about {ResumeTy, D, Rc<RefCell<Vec<DropOrder>>>, &str, &D, impl Future, ()} and [D, D]
   |
   |
LL |   async fn foo_async(x: D, _y: D) {
   |  _________________________________^
LL | |     let l = x.1.clone();
LL | |     let z = D("z", l.clone());
LL | |     l.borrow_mut().push(DropOrder::Function);
LL | |     helper_async(&D("temp", l)).await
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z mir-opt-level=0 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `foo_async::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/drop-order/drop-order-for-temporary-in-tail-return-expr.rs:53:33: 58:2]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs:67:11: 69:6]> in MIR, but typeck only knows about {ResumeTy, Rc<RefCell<Vec<DropOrder>>>, [static generator@/checkout/src/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs:67:11: 69:6], impl Future, (), D, NeverReady} and [std::rc::Rc<std::cell::RefCell<std::vec::Vec<DropOrder>>>]
   |
   |
LL |   async fn varable_completely_contained_within_block_async(l: DropOrderListPtr) {
   |  _______________________________________________________________________________^
LL | |     l.borrow_mut().push(DropOrder::Function);
LL | |     async {
LL | |         let x = D("x", l.clone());
...  |
LL | |     NeverReady.await;
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `varable_completely_contained_within_block_async::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/drop-order/drop-order-for-locals-when-cancelled.rs:65:79: 73:2]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/issue-73137.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-73137.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-73137/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-73137/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issue-73137.rs:28:22: 28:27]> in MIR, but typeck only knows about {ResumeTy, u32, &u32, [static generator@/checkout/src/test/ui/async-await/issue-73137.rs:28:22: 28:27], impl Future, (), Foo, [static generator@/checkout/src/test/ui/async-await/issue-73137.rs:36:15: 36:17], impl Future} and []
   |
   |
LL |       let mut fut = Box::pin(async {
LL | |         let action = Foo {
LL | |             b: &42,
LL | |             b: &42,
LL | |             a: async { 0 }.await,
...  |
LL | |         async {}.await;
LL | |     });

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/issue-73137.rs:25:34: 37:6]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/issues/issue-55809.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-55809.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-55809/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-55809/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issues/issue-55809.rs:14:42: 16:2]> in MIR, but typeck only knows about {ResumeTy, (), &mut (), impl Future, impl Future, impl Future} and []
   |
   |
LL |   async fn async_main() {
   |  _______________________^
LL | |     let mut v = ();
LL | |
LL | |     let _ = bad(&mut v).await;
LL | |     let _ = foo_async(&mut v).await;
LL | |     let _ = bad(v).await;
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `async_main::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/issues/issue-55809.rs:18:23: 24:2]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/issues/issue-61986.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-61986.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-61986" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-61986/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issues/issue-61986.rs:7:30: 9:2]> in MIR, but typeck only knows about {ResumeTy, impl Future, ()} and []
   |
   |
LL |   async fn listen() {
   |  ___________________^
LL | |     while let Some(_) = bar().await {
LL | |         String::new();
LL | |     }
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `listen::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/issues/issue-61986.rs:11:19: 15:2]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/issues/issue-66695-static-refs.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-66695-static-refs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-66695-static-refs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-66695-static-refs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issues/issue-66695-static-refs.rs:16:25: 16:30]> in MIR, but typeck only knows about {ResumeTy, &[i32; 5], [i32; 5], [static generator@/checkout/src/test/ui/async-await/issues/issue-66695-static-refs.rs:16:25: 16:30], impl Future, ()} and []
   |
LL |       async {
   |  ___________^
   |  ___________^
LL | |         let u = A[async { 1 }.await];
LL | |     };

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/issues/issue-66695-static-refs.rs:15:11: 17:6]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/issues/issue-67611-static-mut-refs.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-67611-static-mut-refs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-67611-static-mut-refs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-67611-static-mut-refs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/issues/issue-67611-static-mut-refs.rs:20:34: 20:39]> in MIR, but typeck only knows about {ResumeTy, &[i32; 5], [i32; 5], [static generator@/checkout/src/test/ui/async-await/issues/issue-67611-static-mut-refs.rs:20:34: 20:39], impl Future, ()} and []
   |
   |
LL |       let index_block = async {
   |  _____________________________^
LL | |         let u = unsafe { A[async { 1 }.await] };
LL | |     };

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/issues/issue-67611-static-mut-refs.rs:19:29: 21:6]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/move-part-await-return-rest-struct.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/move-part-await-return-rest-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/move-part-await-return-rest-struct.rs:18:38: 18:40]> in MIR, but typeck only knows about {ResumeTy, Small, Vec<usize>, impl Future, ()} and []
   |
   |
LL |   async fn move_part_await_return_rest_struct() -> Vec<usize> {
   |  _____________________________________________________________^
LL | |     let s = Small { x: vec![31], y: vec![19, 1441] };
LL | |     needs_vec(s.x).await;
LL | |     s.y
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `move_part_await_return_rest_struct::{closure#0}`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/move-part-await-return-rest-tuple.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/move-part-await-return-rest-tuple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-tuple" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type" "lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/move-part-await-return-rest-tuple/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/move-part-await-return-rest-tuple.rs:12:34: 12:39]> in MIR, but typeck only knows about {ResumeTy, (Vec<usize>, Vec<i32>), &Vec<usize>, Vec<usize>, usize, impl Future, ()} and []
   |
   |
LL |   async fn move_part_await_return_rest_tuple() -> Vec<usize> {
   |  ____________________________________________________________^
LL | |     let x = (vec![3], vec![4, 4]);
LL | |     drop(x.1);
LL | |     echo(x.0[0]).await;
LL | |     x.0
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 --crate-type lib
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `move_part_await_return_rest_tuple::{closure#0}`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/large_moves.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/large_moves.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/large_moves/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/async-await/large_moves.rs:22:26: 24:2]> in MIR, but typeck only knows about {ResumeTy, [u8; 9999], &[u8], &[u8; 9999], impl Future, ()} and []
   |
   |
LL |       let x = async { //~ ERROR large_assignments
   |  ___________________^
LL | |         let y = [0; 9999];
LL | |         dbg!(y);
LL | |         thing(&y).await;
LL | |         dbg!(y);
LL | |     };

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/async-await/large_moves.rs:10:19: 15:6]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/repeat_count_const_in_async_fn.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/repeat_count_const_in_async_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/repeat_count_const_in_async_fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/repeat_count_const_in_async_fn/auxiliary"
------------------------------------------

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `<impl at /checkout/src/test/ui/generator/issue-58888.rs:11:1: 24:2>::check_connection::{closure#0}`
#1 [layout_raw] computing layout of `[generator@/checkout/src/test/ui/generator/issue-58888.rs:17:9: 22:10]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/generator/yielding-in-match-guards.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/yielding-in-match-guards.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yielding-in-match-guards" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/yielding-in-match-guards/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/generator/yielding-in-match-guards.rs:16:20: 16:25]> in MIR, but typeck only knows about {ResumeTy, u8, impl Future, (), &u8} and [u8]
   |
   |
LL |   pub async fn g(x: u8) {
LL | |     match x {
LL | |     match x {
LL | |         y if f().await == y => (),
LL | |         _ => (),
LL | |     }
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `g::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/generator/yielding-in-match-guards.rs:19:23: 24:2]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issue-73914.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-73914.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-73914" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Copt-level=0" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-73914/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/issue-73914.rs:20:26: 22:2]> in MIR, but typeck only knows about {ResumeTy, u64, impl Future, ()} and []
   |
   |
LL |   async fn crash() {
   |  __________________^
LL | |     *new().await = 1 + 1;
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 -C opt-level=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `crash::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/issue-73914.rs:24:18: 26:2]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issue-72470-llvm-dominate.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issue-72470-llvm-dominate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-72470-llvm-dominate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "opt-level=3" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issue-72470-llvm-dominate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@UnboundedReceiver<Msg>::recv::{closure#0}]> in MIR, but typeck only knows about {ResumeTy, impl Future, [closure@/checkout/src/test/ui/issue-72470-llvm-dominate.rs:36:42: 53:18], issue_72470_lib::PollFn<[closure@/checkout/src/test/ui/issue-72470-llvm-dominate.rs:36:42: 53:18]>, ()} and [issue_72470_lib::UnboundedReceiver<Msg>, std::sync::Mutex<()>]
   |
   |
LL |       issue_72470_lib::run(async move {
LL | |         {
LL | |             let output = {
LL | |             let output = {
LL | |                 let mut fut = rx.recv();
LL | |         entity.lock();
LL | |     });
   | |_____^

---
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0 -C opt-level=3
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `main::{closure#0}`
#1 [layout_raw] computing layout of `[static generator@/checkout/src/test/ui/issue-72470-llvm-dominate.rs:32:37: 65:6]`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/mir/issue-71793-inline-args-storage.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/issue-71793-inline-args-storage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-71793-inline-args-storage" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-71793-inline-args-storage/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_mir/src/transform/generator.rs:751:13: Broken MIR: generator contains type from_generator::GenFuture<[static generator@/checkout/src/test/ui/mir/issue-71793-inline-args-storage.rs:11:24: 11:26]> in MIR, but typeck only knows about {ResumeTy, impl Future, ()} and []
   |
   |
LL |   pub async fn connect_many() {
   |  _____________________________^
LL | |     Vec::<String>::new().first().ok_or("").unwrap();
LL | |     connect().await;
LL | | }

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:981:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (167d21d44 2021-07-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `connect_many::{closure#0}`
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 11984 passed; 29 failed; 102 ignored; 0 measured; 0 filtered out; finished in 140.79s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:20
