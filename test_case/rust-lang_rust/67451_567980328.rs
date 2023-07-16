plain
2019-12-20T16:15:02.2724375Z ---- [ui] ui/async-await/issues/issue-62009-1.rs stdout ----
2019-12-20T16:15:02.2724462Z diff of stderr:
2019-12-20T16:15:02.2724503Z 
2019-12-20T16:15:02.2724572Z 32    |
2019-12-20T16:15:02.2724640Z 33 LL |     (|_| 2333).await;
2019-12-20T16:15:02.2724997Z 34    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@$DIR/issue-62009-1.rs:15:5: 15:15]`
2019-12-20T16:15:02.2726034Z -    | 
2019-12-20T16:15:02.2726257Z -   ::: $SRC_DIR/libstd/future.rs:LL:COL
2019-12-20T16:15:02.2726900Z - LL |     F: Future
2019-12-20T16:15:02.2726900Z - LL |     F: Future
2019-12-20T16:15:02.2727650Z -    |        ------ required by this bound in `std::future::poll_with_tls_context`
2019-12-20T16:15:02.2728087Z 41 error: aborting due to 4 previous errors
2019-12-20T16:15:02.2728163Z 42 
2019-12-20T16:15:02.2728222Z 
2019-12-20T16:15:02.2728261Z 
2019-12-20T16:15:02.2728261Z 
2019-12-20T16:15:02.2728353Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2728806Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/issue-62009-1.stderr
2019-12-20T16:15:02.2729163Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2729532Z To only update this specific test, also pass `--test-args async-await/issues/issue-62009-1.rs`
2019-12-20T16:15:02.2729696Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2729792Z status: exit code: 1
2019-12-20T16:15:02.2729792Z status: exit code: 1
2019-12-20T16:15:02.2731368Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62009-1.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62009-1/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2731876Z ------------------------------------------
2019-12-20T16:15:02.2731921Z 
2019-12-20T16:15:02.2732151Z ------------------------------------------
2019-12-20T16:15:02.2732212Z stderr:
2019-12-20T16:15:02.2732212Z stderr:
2019-12-20T16:15:02.2732432Z ------------------------------------------
2019-12-20T16:15:02.2732513Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-12-20T16:15:02.2732791Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:9:5
2019-12-20T16:15:02.2733141Z LL | fn main() {
2019-12-20T16:15:02.2736138Z    |    ---- this is not `async`
2019-12-20T16:15:02.2736138Z    |    ---- this is not `async`
2019-12-20T16:15:02.2736466Z LL |     async { let (); }.await;
2019-12-20T16:15:02.2736574Z    |     ^^^^^^^^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-12-20T16:15:02.2736629Z 
2019-12-20T16:15:02.2737118Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-12-20T16:15:02.2738805Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:11:5
2019-12-20T16:15:02.2739202Z LL |   fn main() {
2019-12-20T16:15:02.2739542Z    |      ---- this is not `async`
2019-12-20T16:15:02.2739625Z ...
2019-12-20T16:15:02.2739712Z LL | /     async {
2019-12-20T16:15:02.2739712Z LL | /     async {
2019-12-20T16:15:02.2739797Z LL | |     //~^ ERROR `await` is only allowed inside `async` functions and blocks
2019-12-20T16:15:02.2740024Z LL | |         let task1 = print_dur().await;
2019-12-20T16:15:02.2740120Z LL | |     }.await;
2019-12-20T16:15:02.2740364Z    | |___________^ only allowed inside `async` functions and blocks
2019-12-20T16:15:02.2740587Z 
2019-12-20T16:15:02.2740670Z error[E0728]: `await` is only allowed inside `async` functions and blocks
2019-12-20T16:15:02.2740969Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:15:5
2019-12-20T16:15:02.2750092Z LL | fn main() {
2019-12-20T16:15:02.2750585Z    |    ---- this is not `async`
2019-12-20T16:15:02.2750683Z ...
2019-12-20T16:15:02.2750683Z ...
2019-12-20T16:15:02.2750746Z LL |     (|_| 2333).await;
2019-12-20T16:15:02.2751037Z    |     ^^^^^^^^^^^^^^^^ only allowed inside `async` functions and blocks
2019-12-20T16:15:02.2751086Z 
2019-12-20T16:15:02.2751422Z error[E0277]: the trait bound `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:15:5: 15:15]: std::future::Future` is not satisfied
2019-12-20T16:15:02.2751712Z   --> /checkout/src/test/ui/async-await/issues/issue-62009-1.rs:15:5
2019-12-20T16:15:02.2751802Z    |
2019-12-20T16:15:02.2751853Z LL |     (|_| 2333).await;
2019-12-20T16:15:02.2752337Z    |     ^^^^^^^^^^^^^^^^ the trait `std::future::Future` is not implemented for `[closure@/checkout/src/test/ui/async-await/issues/issue-62009-1.rs:15:5: 15:15]`
2019-12-20T16:15:02.2752465Z error: aborting due to 4 previous errors
2019-12-20T16:15:02.2752498Z 
2019-12-20T16:15:02.2752568Z Some errors have detailed explanations: E0277, E0728.
2019-12-20T16:15:02.2752774Z For more information about an error, try `rustc --explain E0277`.
---
2019-12-20T16:15:02.2753269Z ---- [ui] ui/closures/closure-move-sync.rs stdout ----
2019-12-20T16:15:02.2753341Z diff of stderr:
2019-12-20T16:15:02.2753372Z 
2019-12-20T16:15:02.2753438Z 3    |
2019-12-20T16:15:02.2753484Z 4 LL |     let t = thread::spawn(|| {
2019-12-20T16:15:02.2753573Z 5    |             ^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-12-20T16:15:02.2753741Z -    | 
2019-12-20T16:15:02.2753928Z -   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-12-20T16:15:02.2753983Z 8    |
2019-12-20T16:15:02.2754188Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-12-20T16:15:02.2754404Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-12-20T16:15:02.2754643Z 12    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
2019-12-20T16:15:02.2754756Z 13    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-12-20T16:15:02.2754756Z 13    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-12-20T16:15:02.2755062Z 14    = note: required because it appears within the type `[closure@$DIR/closure-move-sync.rs:9:27: 12:6 recv:&std::sync::mpsc::Receiver<()>]`
2019-12-20T16:15:02.2755179Z 18    |
2019-12-20T16:15:02.2755179Z 18    |
2019-12-20T16:15:02.2755252Z 19 LL |     thread::spawn(|| tx.send(()).unwrap());
2019-12-20T16:15:02.2755341Z 20    |     ^^^^^^^^^^^^^ `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-12-20T16:15:02.2755528Z -    | 
2019-12-20T16:15:02.2755729Z -   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-12-20T16:15:02.2755892Z -    |
2019-12-20T16:15:02.2756112Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-12-20T16:15:02.2756346Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-12-20T16:15:02.2756627Z 27    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
2019-12-20T16:15:02.2757157Z 28    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-12-20T16:15:02.2757344Z 
2019-12-20T16:15:02.2757400Z 
2019-12-20T16:15:02.2757400Z 
2019-12-20T16:15:02.2757473Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2757933Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync/closure-move-sync.stderr
2019-12-20T16:15:02.2758234Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2758554Z To only update this specific test, also pass `--test-args closures/closure-move-sync.rs`
2019-12-20T16:15:02.2758701Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2758774Z status: exit code: 1
2019-12-20T16:15:02.2758774Z status: exit code: 1
2019-12-20T16:15:02.2759850Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-move-sync.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-move-sync/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2760569Z ------------------------------------------
2019-12-20T16:15:02.2760617Z 
2019-12-20T16:15:02.2760986Z ------------------------------------------
2019-12-20T16:15:02.2761226Z stderr:
2019-12-20T16:15:02.2761226Z stderr:
2019-12-20T16:15:02.2761417Z ------------------------------------------
2019-12-20T16:15:02.2761512Z error[E0277]: `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-12-20T16:15:02.2762005Z    |
2019-12-20T16:15:02.2762005Z    |
2019-12-20T16:15:02.2762073Z LL |     let t = thread::spawn(|| {
2019-12-20T16:15:02.2762151Z    |             ^^^^^^^^^^^^^ `std::sync::mpsc::Receiver<()>` cannot be shared between threads safely
2019-12-20T16:15:02.2762314Z    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Receiver<()>`
2019-12-20T16:15:02.2762428Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-12-20T16:15:02.2762428Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Receiver<()>`
2019-12-20T16:15:02.2762803Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/closures/closure-move-sync.rs:9:27: 12:6 recv:&std::sync::mpsc::Receiver<()>]`
2019-12-20T16:15:02.2762881Z 
2019-12-20T16:15:02.2762965Z error[E0277]: `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-12-20T16:15:02.2763289Z    |
2019-12-20T16:15:02.2763289Z    |
2019-12-20T16:15:02.2763345Z LL |     thread::spawn(|| tx.send(()).unwrap());
2019-12-20T16:15:02.2763438Z    |     ^^^^^^^^^^^^^ `std::sync::mpsc::Sender<()>` cannot be shared between threads safely
2019-12-20T16:15:02.2763601Z    = help: the trait `std::marker::Sync` is not implemented for `std::sync::mpsc::Sender<()>`
2019-12-20T16:15:02.2763713Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-12-20T16:15:02.2763713Z    = note: required because of the requirements on the impl of `std::marker::Send` for `&std::sync::mpsc::Sender<()>`
2019-12-20T16:15:02.2764251Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/closures/closure-move-sync.rs:21:19: 21:42 tx:&std::sync::mpsc::Sender<()>]`
2019-12-20T16:15:02.2764398Z error: aborting due to 2 previous errors
2019-12-20T16:15:02.2764456Z 
2019-12-20T16:15:02.2764850Z For more information about this error, try `rustc --explain E0277`.
2019-12-20T16:15:02.2764912Z 
2019-12-20T16:15:02.2764912Z 
2019-12-20T16:15:02.2765709Z ------------------------------------------
2019-12-20T16:15:02.2765767Z 
2019-12-20T16:15:02.2765816Z 
2019-12-20T16:15:02.2766059Z ---- [ui] ui/consts/const-size_of-cycle.rs stdout ----
2019-12-20T16:15:02.2766144Z diff of stderr:
2019-12-20T16:15:02.2766256Z 
2019-12-20T16:15:02.2766330Z 15 LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-12-20T16:15:02.2767036Z 17 note: ...which requires const-evaluating `std::mem::size_of`...
2019-12-20T16:15:02.2767036Z 17 note: ...which requires const-evaluating `std::mem::size_of`...
2019-12-20T16:15:02.2767335Z -   --> $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-12-20T16:15:02.2767559Z -    |
2019-12-20T16:15:02.2767785Z - LL |     intrinsics::size_of::<T>()
2019-12-20T16:15:02.2768034Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-20T16:15:02.2768325Z 22 note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2019-12-20T16:15:02.2768602Z -   --> $SRC_DIR/libcore/intrinsics.rs:LL:COL
2019-12-20T16:15:02.2768806Z -    |
2019-12-20T16:15:02.2769064Z - LL |     pub fn size_of<T>() -> usize;
2019-12-20T16:15:02.2769404Z 27    = note: ...which requires computing layout of `Foo`...
2019-12-20T16:15:02.2769404Z 27    = note: ...which requires computing layout of `Foo`...
2019-12-20T16:15:02.2769544Z 28    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
2019-12-20T16:15:02.2769956Z 29    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`, completing the cycle
2019-12-20T16:15:02.2770264Z 
2019-12-20T16:15:02.2770325Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2770957Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
2019-12-20T16:15:02.2770957Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/const-size_of-cycle.stderr
2019-12-20T16:15:02.2771207Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2771477Z To only update this specific test, also pass `--test-args consts/const-size_of-cycle.rs`
2019-12-20T16:15:02.2771597Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2771672Z status: exit code: 1
2019-12-20T16:15:02.2771672Z status: exit code: 1
2019-12-20T16:15:02.2772503Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of-cycle.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of-cycle/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2772959Z ------------------------------------------
2019-12-20T16:15:02.2773001Z 
2019-12-20T16:15:02.2773214Z ------------------------------------------
2019-12-20T16:15:02.2773272Z stderr:
2019-12-20T16:15:02.2773272Z stderr:
2019-12-20T16:15:02.2773470Z ------------------------------------------
2019-12-20T16:15:02.2773712Z error[E0391]: cycle detected when const-evaluating + checking `Foo::bytes::{{constant}}#0`
2019-12-20T16:15:02.2774203Z    |
2019-12-20T16:15:02.2774203Z    |
2019-12-20T16:15:02.2774275Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-12-20T16:15:02.2774415Z    |
2019-12-20T16:15:02.2774415Z    |
2019-12-20T16:15:02.2775044Z note: ...which requires const-evaluating + checking `Foo::bytes::{{constant}}#0`...
2019-12-20T16:15:02.2775549Z    |
2019-12-20T16:15:02.2775549Z    |
2019-12-20T16:15:02.2775609Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-12-20T16:15:02.2775699Z    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-20T16:15:02.2776046Z note: ...which requires const-evaluating `Foo::bytes::{{constant}}#0`...
2019-12-20T16:15:02.2776425Z    |
2019-12-20T16:15:02.2776425Z    |
2019-12-20T16:15:02.2776503Z LL |     bytes: [u8; std::mem::size_of::<Foo>()]
2019-12-20T16:15:02.2777348Z note: ...which requires const-evaluating `std::mem::size_of`...
2019-12-20T16:15:02.2777348Z note: ...which requires const-evaluating `std::mem::size_of`...
2019-12-20T16:15:02.2777663Z note: ...which requires const-evaluating + checking `std::intrinsics::size_of`...
2019-12-20T16:15:02.2777759Z    = note: ...which requires computing layout of `Foo`...
2019-12-20T16:15:02.2777898Z    = note: ...which requires normalizing `ParamEnvAnd { param_env: ParamEnv { caller_bounds: [], reveal: All, def_id: None }, value: [u8; _] }`...
2019-12-20T16:15:02.2778293Z    = note: ...which again requires const-evaluating + checking `Foo::bytes::{{constant}}#0`, completing the cycle
2019-12-20T16:15:02.2778428Z note: cycle used when processing `Foo`
2019-12-20T16:15:02.2778820Z    |
2019-12-20T16:15:02.2778903Z LL | struct Foo {
2019-12-20T16:15:02.2778973Z    | ^^^^^^^^^^
2019-12-20T16:15:02.2779036Z 
---
2019-12-20T16:15:02.2779923Z 
2019-12-20T16:15:02.2780361Z ---- [ui] ui/consts/offset_from_ub.rs stdout ----
2019-12-20T16:15:02.2780429Z diff of stderr:
2019-12-20T16:15:02.2780481Z 
2019-12-20T16:15:02.2780714Z 1 error: any use of this value will cause an error
2019-12-20T16:15:02.2781113Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-20T16:15:02.2781285Z -    |
2019-12-20T16:15:02.2781520Z - LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-20T16:15:02.2782106Z -    |           |
2019-12-20T16:15:02.2782365Z -    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-12-20T16:15:02.2782365Z -    |           ptr_offset_from cannot compute offset of pointers into different allocations.
2019-12-20T16:15:02.2782837Z -    |           inside call to `std::ptr::<impl *const Struct>::offset_from` at $DIR/offset_from_ub.rs:21:27
2019-12-20T16:15:02.2782934Z 9    | 
2019-12-20T16:15:02.2782989Z 10   ::: $DIR/offset_from_ub.rs:15:1
2019-12-20T16:15:02.2783100Z 
2019-12-20T16:15:02.2783179Z 21    = note: `#[deny(const_err)]` on by default
2019-12-20T16:15:02.2783243Z 22 
2019-12-20T16:15:02.2783321Z 23 error: any use of this value will cause an error
2019-12-20T16:15:02.2783321Z 23 error: any use of this value will cause an error
2019-12-20T16:15:02.2783549Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-20T16:15:02.2783753Z -    |
2019-12-20T16:15:02.2783980Z - LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-20T16:15:02.2784581Z -    |           |
2019-12-20T16:15:02.2784822Z -    |           a memory access tried to interpret some bytes as a pointer
2019-12-20T16:15:02.2784822Z -    |           a memory access tried to interpret some bytes as a pointer
2019-12-20T16:15:02.2785093Z -    |           inside call to `std::ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:27:14
2019-12-20T16:15:02.2785194Z 31    | 
2019-12-20T16:15:02.2785264Z 32   ::: $DIR/offset_from_ub.rs:25:1
2019-12-20T16:15:02.2785351Z 
2019-12-20T16:15:02.2785536Z 38    | |__-
2019-12-20T16:15:02.2785591Z 39 
2019-12-20T16:15:02.2785666Z 40 error: any use of this value will cause an error
2019-12-20T16:15:02.2785666Z 40 error: any use of this value will cause an error
2019-12-20T16:15:02.2785881Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-20T16:15:02.2786206Z -    |
2019-12-20T16:15:02.2786416Z - LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-20T16:15:02.2787216Z -    |           |
2019-12-20T16:15:02.2787216Z -    |           |
2019-12-20T16:15:02.2787607Z -    |           exact_div: 1 cannot be divided by 2 without remainder
2019-12-20T16:15:02.2788024Z -    |           inside call to `std::ptr::<impl *const u16>::offset_from` at $DIR/offset_from_ub.rs:35:14
2019-12-20T16:15:02.2788124Z 48    | 
2019-12-20T16:15:02.2788295Z 49   ::: $DIR/offset_from_ub.rs:30:1
2019-12-20T16:15:02.2788426Z 
2019-12-20T16:15:02.2788671Z 58    | |__-
2019-12-20T16:15:02.2788756Z 59 
2019-12-20T16:15:02.2788828Z 60 error: any use of this value will cause an error
2019-12-20T16:15:02.2788828Z 60 error: any use of this value will cause an error
2019-12-20T16:15:02.2789109Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-20T16:15:02.2789328Z -    |
2019-12-20T16:15:02.2789612Z - LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-20T16:15:02.2790306Z -    |           |
2019-12-20T16:15:02.2790513Z -    |           invalid use of NULL pointer
2019-12-20T16:15:02.2790513Z -    |           invalid use of NULL pointer
2019-12-20T16:15:02.2790778Z -    |           inside call to `std::ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:41:14
2019-12-20T16:15:02.2790870Z 68    | 
2019-12-20T16:15:02.2790921Z 69   ::: $DIR/offset_from_ub.rs:38:1
2019-12-20T16:15:02.2791136Z 
2019-12-20T16:15:02.2791314Z 76    | |__-
2019-12-20T16:15:02.2791374Z 77 
2019-12-20T16:15:02.2791445Z 78 error: any use of this value will cause an error
2019-12-20T16:15:02.2791445Z 78 error: any use of this value will cause an error
2019-12-20T16:15:02.2791644Z -   --> $SRC_DIR/libcore/ptr/mod.rs:LL:COL
2019-12-20T16:15:02.2791819Z -    |
2019-12-20T16:15:02.2792018Z - LL |           intrinsics::ptr_offset_from(self, origin)
2019-12-20T16:15:02.2792596Z -    |           |
2019-12-20T16:15:02.2792840Z -    |           a memory access tried to interpret some bytes as a pointer
2019-12-20T16:15:02.2792840Z -    |           a memory access tried to interpret some bytes as a pointer
2019-12-20T16:15:02.2793111Z -    |           inside call to `std::ptr::<impl *const u8>::offset_from` at $DIR/offset_from_ub.rs:48:14
2019-12-20T16:15:02.2793200Z 86    | 
2019-12-20T16:15:02.2793259Z 87   ::: $DIR/offset_from_ub.rs:44:1
2019-12-20T16:15:02.2793366Z 
2019-12-20T16:15:02.2793416Z 
2019-12-20T16:15:02.2793474Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2793474Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2793775Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/offset_from_ub.stderr
2019-12-20T16:15:02.2794026Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2794287Z To only update this specific test, also pass `--test-args consts/offset_from_ub.rs`
2019-12-20T16:15:02.2794409Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2794655Z status: exit code: 1
2019-12-20T16:15:02.2794655Z status: exit code: 1
2019-12-20T16:15:02.2795567Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/offset_from_ub.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/offset_from_ub/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2796326Z ------------------------------------------
2019-12-20T16:15:02.2796392Z 
2019-12-20T16:15:02.2796581Z ------------------------------------------
2019-12-20T16:15:02.2797049Z stderr:
2019-12-20T16:15:02.2797049Z stderr:
2019-12-20T16:15:02.2797323Z ------------------------------------------
2019-12-20T16:15:02.2797423Z error: any use of this value will cause an error
2019-12-20T16:15:02.2797495Z    | 
2019-12-20T16:15:02.2797584Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:15:1
2019-12-20T16:15:02.2797680Z    |
2019-12-20T16:15:02.2797749Z LL | / pub const DIFFERENT_ALLOC: usize = {
2019-12-20T16:15:02.2797958Z LL | |     //~^ NOTE
2019-12-20T16:15:02.2798048Z LL | |     let uninit = std::mem::MaybeUninit::<Struct>::uninit();
2019-12-20T16:15:02.2798161Z LL | |     let base_ptr: *const Struct = &uninit as *const _ as *const Struct;
2019-12-20T16:15:02.2798388Z LL | |     offset as usize
2019-12-20T16:15:02.2798457Z LL | | };
2019-12-20T16:15:02.2798708Z    | |__-
2019-12-20T16:15:02.2798789Z    |
2019-12-20T16:15:02.2798789Z    |
2019-12-20T16:15:02.2798858Z    = note: `#[deny(const_err)]` on by default
2019-12-20T16:15:02.2798906Z 
2019-12-20T16:15:02.2798994Z error: any use of this value will cause an error
2019-12-20T16:15:02.2799068Z    | 
2019-12-20T16:15:02.2799161Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:25:1
2019-12-20T16:15:02.2799240Z    |
2019-12-20T16:15:02.2799323Z LL | / pub const NOT_PTR: usize = {
2019-12-20T16:15:02.2799415Z LL | |     //~^ NOTE
2019-12-20T16:15:02.2799498Z LL | |     unsafe { (42 as *const u8).offset_from(&5u8) as usize }
2019-12-20T16:15:02.2800075Z    | |__-
2019-12-20T16:15:02.2800283Z 
2019-12-20T16:15:02.2800518Z error: any use of this value will cause an error
2019-12-20T16:15:02.2800595Z    | 
2019-12-20T16:15:02.2800595Z    | 
2019-12-20T16:15:02.2800812Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:30:1
2019-12-20T16:15:02.2800896Z    |
2019-12-20T16:15:02.2800948Z LL | / pub const NOT_MULTIPLE_OF_SIZE: isize = {
2019-12-20T16:15:02.2801023Z LL | |     //~^ NOTE
2019-12-20T16:15:02.2801077Z LL | |     let data = [5u8, 6, 7];
2019-12-20T16:15:02.2801152Z LL | |     let base_ptr = data.as_ptr();
2019-12-20T16:15:02.2801216Z LL | |     let field_ptr = &data[1] as *const u8 as *const u16;
2019-12-20T16:15:02.2801307Z LL | |     unsafe { field_ptr.offset_from(base_ptr as *const u16) }
2019-12-20T16:15:02.2801554Z    | |__-
2019-12-20T16:15:02.2801586Z 
2019-12-20T16:15:02.2801654Z error: any use of this value will cause an error
2019-12-20T16:15:02.2801711Z    | 
2019-12-20T16:15:02.2801711Z    | 
2019-12-20T16:15:02.2801793Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:38:1
2019-12-20T16:15:02.2801856Z    |
2019-12-20T16:15:02.2801927Z LL | / pub const OFFSET_FROM_NULL: isize = {
2019-12-20T16:15:02.2801986Z LL | |     //~^ NOTE
2019-12-20T16:15:02.2802059Z LL | |     let ptr = 0 as *const u8;
2019-12-20T16:15:02.2802127Z LL | |     unsafe { ptr.offset_from(ptr) }
2019-12-20T16:15:02.2802398Z    | |__-
2019-12-20T16:15:02.2802433Z 
2019-12-20T16:15:02.2802490Z error: any use of this value will cause an error
2019-12-20T16:15:02.2802568Z    | 
2019-12-20T16:15:02.2802568Z    | 
2019-12-20T16:15:02.2802628Z   ::: /checkout/src/test/ui/consts/offset_from_ub.rs:44:1
2019-12-20T16:15:02.2802707Z    |
2019-12-20T16:15:02.2802794Z LL | / pub const DIFFERENT_INT: isize = { // offset_from with two different integers: like DIFFERENT_ALLOC
2019-12-20T16:15:02.2802873Z LL | |     //~^ NOTE
2019-12-20T16:15:02.2802950Z LL | |     let ptr1 = 8 as *const u8;
2019-12-20T16:15:02.2803011Z LL | |     let ptr2 = 16 as *const u8;
2019-12-20T16:15:02.2803097Z LL | |     unsafe { ptr2.offset_from(ptr1) }
2019-12-20T16:15:02.2803523Z    | |__-
2019-12-20T16:15:02.2803556Z 
2019-12-20T16:15:02.2803624Z error: aborting due to 5 previous errors
2019-12-20T16:15:02.2803662Z 
---
2019-12-20T16:15:02.2805440Z 3    |
2019-12-20T16:15:02.2805546Z 4 LL |      x: Error
2019-12-20T16:15:02.2805614Z 5    |      ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-12-20T16:15:02.2806124Z -    | 
2019-12-20T16:15:02.2807802Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-12-20T16:15:02.2808468Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-12-20T16:15:02.2808805Z -    |             - required by this bound in `std::hash::Hash::hash`
2019-12-20T16:15:02.2808907Z 11 
2019-12-20T16:15:02.2808972Z 12 error: aborting due to previous error
2019-12-20T16:15:02.2808972Z 12 error: aborting due to previous error
2019-12-20T16:15:02.2809060Z 13 
2019-12-20T16:15:02.2809190Z 
2019-12-20T16:15:02.2809226Z 
2019-12-20T16:15:02.2809313Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2809752Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum-struct-variant/derives-span-Hash-enum-struct-variant.stderr
2019-12-20T16:15:02.2810087Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2810590Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-enum-struct-variant.rs`
2019-12-20T16:15:02.2810727Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2810793Z status: exit code: 1
2019-12-20T16:15:02.2810793Z status: exit code: 1
2019-12-20T16:15:02.2813441Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-enum-struct-variant.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum-struct-variant" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum-struct-variant/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2814285Z ------------------------------------------
2019-12-20T16:15:02.2814345Z 
2019-12-20T16:15:02.2814714Z ------------------------------------------
2019-12-20T16:15:02.2815009Z stderr:
2019-12-20T16:15:02.2815009Z stderr:
2019-12-20T16:15:02.2815384Z ------------------------------------------
2019-12-20T16:15:02.2815488Z error[E0277]: the trait bound `Error: std::hash::Hash` is not satisfied
2019-12-20T16:15:02.2815763Z   --> /checkout/src/test/ui/derives/derives-span-Hash-enum-struct-variant.rs:12:6
2019-12-20T16:15:02.2815860Z    |
2019-12-20T16:15:02.2815941Z LL |      x: Error //~ ERROR
2019-12-20T16:15:02.2816017Z    |      ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-12-20T16:15:02.2816142Z error: aborting due to previous error
2019-12-20T16:15:02.2816182Z 
2019-12-20T16:15:02.2816444Z For more information about this error, try `rustc --explain E0277`.
2019-12-20T16:15:02.2816499Z 
---
2019-12-20T16:15:02.2817917Z 3    |
2019-12-20T16:15:02.2818008Z 4 LL |     x: Error
2019-12-20T16:15:02.2818090Z 5    |     ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-12-20T16:15:02.2818332Z -    | 
2019-12-20T16:15:02.2818765Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-12-20T16:15:02.2819260Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-12-20T16:15:02.2819560Z -    |             - required by this bound in `std::hash::Hash::hash`
2019-12-20T16:15:02.2819785Z 11 
2019-12-20T16:15:02.2819879Z 12 error: aborting due to previous error
2019-12-20T16:15:02.2819879Z 12 error: aborting due to previous error
2019-12-20T16:15:02.2819950Z 13 
2019-12-20T16:15:02.2820005Z 
2019-12-20T16:15:02.2820040Z 
2019-12-20T16:15:02.2820127Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2820844Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-struct/derives-span-Hash-struct.stderr
2019-12-20T16:15:02.2821104Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2821632Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-struct.rs`
2019-12-20T16:15:02.2821774Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2821851Z status: exit code: 1
2019-12-20T16:15:02.2821851Z status: exit code: 1
2019-12-20T16:15:02.2822815Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-struct.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-struct/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2823262Z ------------------------------------------
2019-12-20T16:15:02.2823304Z 
2019-12-20T16:15:02.2823509Z ------------------------------------------
2019-12-20T16:15:02.2823567Z stderr:
2019-12-20T16:15:02.2823567Z stderr:
2019-12-20T16:15:02.2823763Z ------------------------------------------
2019-12-20T16:15:02.2823854Z error[E0277]: the trait bound `Error: std::hash::Hash` is not satisfied
2019-12-20T16:15:02.2824082Z   --> /checkout/src/test/ui/derives/derives-span-Hash-struct.rs:11:5
2019-12-20T16:15:02.2824164Z    |
2019-12-20T16:15:02.2824214Z LL |     x: Error //~ ERROR
2019-12-20T16:15:02.2824299Z    |     ^^^^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-12-20T16:15:02.2824582Z error: aborting due to previous error
2019-12-20T16:15:02.2824617Z 
2019-12-20T16:15:02.2824823Z For more information about this error, try `rustc --explain E0277`.
2019-12-20T16:15:02.2824882Z 
---
2019-12-20T16:15:02.2825517Z 3    |
2019-12-20T16:15:02.2825564Z 4 LL |      Error
2019-12-20T16:15:02.2825651Z 5    |      ^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-12-20T16:15:02.2825821Z -    | 
2019-12-20T16:15:02.2826017Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-12-20T16:15:02.2826555Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-12-20T16:15:02.2827493Z -    |             - required by this bound in `std::hash::Hash::hash`
2019-12-20T16:15:02.2827579Z 11 
2019-12-20T16:15:02.2827660Z 12 error: aborting due to previous error
2019-12-20T16:15:02.2827660Z 12 error: aborting due to previous error
2019-12-20T16:15:02.2827730Z 13 
2019-12-20T16:15:02.2827766Z 
2019-12-20T16:15:02.2827820Z 
2019-12-20T16:15:02.2827890Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2828287Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum/derives-span-Hash-enum.stderr
2019-12-20T16:15:02.2828610Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2828924Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-enum.rs`
2019-12-20T16:15:02.2829081Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2829170Z status: exit code: 1
2019-12-20T16:15:02.2829170Z status: exit code: 1
2019-12-20T16:15:02.2831444Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-enum.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-enum/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2832662Z ------------------------------------------
2019-12-20T16:15:02.2832826Z 
2019-12-20T16:15:02.2833079Z ------------------------------------------
2019-12-20T16:15:02.2833141Z stderr:
---
2019-12-20T16:15:02.2835910Z 3    |
2019-12-20T16:15:02.2835966Z 4 LL |     Error
2019-12-20T16:15:02.2836053Z 5    |     ^^^^^ the trait `std::hash::Hash` is not implemented for `Error`
2019-12-20T16:15:02.2836245Z -    | 
2019-12-20T16:15:02.2836467Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-12-20T16:15:02.2837331Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-12-20T16:15:02.2837616Z -    |             - required by this bound in `std::hash::Hash::hash`
2019-12-20T16:15:02.2837714Z 11 
2019-12-20T16:15:02.2837789Z 12 error: aborting due to previous error
2019-12-20T16:15:02.2837789Z 12 error: aborting due to previous error
2019-12-20T16:15:02.2837878Z 13 
2019-12-20T16:15:02.2837916Z 
2019-12-20T16:15:02.2837951Z 
2019-12-20T16:15:02.2838038Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2838449Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-tuple-struct/derives-span-Hash-tuple-struct.stderr
2019-12-20T16:15:02.2843021Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2843534Z To only update this specific test, also pass `--test-args derives/derives-span-Hash-tuple-struct.rs`
2019-12-20T16:15:02.2843672Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2843917Z status: exit code: 1
2019-12-20T16:15:02.2843917Z status: exit code: 1
2019-12-20T16:15:02.2845597Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/derives-span-Hash-tuple-struct.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-tuple-struct" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/derives-span-Hash-tuple-struct/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2846121Z ------------------------------------------
2019-12-20T16:15:02.2846166Z 
2019-12-20T16:15:02.2846544Z ------------------------------------------
2019-12-20T16:15:02.2846625Z stderr:
---
2019-12-20T16:15:02.2849556Z 31    |
2019-12-20T16:15:02.2849645Z 32 LL |     fn hash(&self, hasher: &mut impl Hasher) {}
2019-12-20T16:15:02.2850519Z 33    |                                 ^^^^^^^^^^^ expected generic parameter, found `impl Trait`
2019-12-20T16:15:02.2850971Z -    | 
2019-12-20T16:15:02.2851191Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-12-20T16:15:02.2851566Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-12-20T16:15:02.2851768Z -    |             - declaration in trait here
2019-12-20T16:15:02.2851849Z 39 
2019-12-20T16:15:02.2851902Z 40 error: aborting due to 3 previous errors
2019-12-20T16:15:02.2851902Z 40 error: aborting due to 3 previous errors
2019-12-20T16:15:02.2852147Z 41 
2019-12-20T16:15:02.2852177Z 
2019-12-20T16:15:02.2852206Z 
2019-12-20T16:15:02.2852280Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2852585Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/impl-generic-mismatch.stderr
2019-12-20T16:15:02.2852850Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2853125Z To only update this specific test, also pass `--test-args impl-trait/impl-generic-mismatch.rs`
2019-12-20T16:15:02.2853248Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2853313Z status: exit code: 1
2019-12-20T16:15:02.2853313Z status: exit code: 1
2019-12-20T16:15:02.2854200Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-generic-mismatch/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2854651Z ------------------------------------------
2019-12-20T16:15:02.2854709Z 
2019-12-20T16:15:02.2854898Z ------------------------------------------
2019-12-20T16:15:02.2855202Z stderr:
2019-12-20T16:15:02.2855202Z stderr:
2019-12-20T16:15:02.2855389Z ------------------------------------------
2019-12-20T16:15:02.2855473Z error[E0643]: method `foo` has incompatible signature for trait
2019-12-20T16:15:02.2855702Z   --> /checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs:12:12
2019-12-20T16:15:02.2855792Z    |
2019-12-20T16:15:02.2855858Z LL |     fn foo(&self, _: &impl Debug);
2019-12-20T16:15:02.2856071Z    |                       ---------- declaration in trait here
2019-12-20T16:15:02.2856150Z ...
2019-12-20T16:15:02.2856201Z LL |     fn foo<U: Debug>(&self, _: &U) { }
2019-12-20T16:15:02.2856284Z    |            ^ expected `impl Trait`, found generic parameter
2019-12-20T16:15:02.2856421Z help: try removing the generic parameter and using `impl Trait` instead
2019-12-20T16:15:02.2856483Z    |
2019-12-20T16:15:02.2856483Z    |
2019-12-20T16:15:02.2856550Z LL |     fn foo(&self, _: &impl Debug) { }
2019-12-20T16:15:02.2857149Z 
2019-12-20T16:15:02.2857351Z error[E0643]: method `bar` has incompatible signature for trait
2019-12-20T16:15:02.2857716Z   --> /checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs:21:23
2019-12-20T16:15:02.2857795Z    |
2019-12-20T16:15:02.2857795Z    |
2019-12-20T16:15:02.2857877Z LL |     fn bar<U: Debug>(&self, _: &U);
2019-12-20T16:15:02.2858264Z    |            - declaration in trait here
2019-12-20T16:15:02.2858353Z ...
2019-12-20T16:15:02.2858417Z LL |     fn bar(&self, _: &impl Debug) { }
2019-12-20T16:15:02.2858525Z    |                       ^^^^^^^^^^ expected generic parameter, found `impl Trait`
2019-12-20T16:15:02.2858708Z help: try changing the `impl Trait` argument to a generic parameter
2019-12-20T16:15:02.2858803Z    |
2019-12-20T16:15:02.2858803Z    |
2019-12-20T16:15:02.2858868Z LL |     fn bar<U: Debug>(&self, _: &U) { }
2019-12-20T16:15:02.2859009Z 
2019-12-20T16:15:02.2859098Z error[E0643]: method `hash` has incompatible signature for trait
2019-12-20T16:15:02.2859425Z   --> /checkout/src/test/ui/impl-trait/impl-generic-mismatch.rs:32:33
2019-12-20T16:15:02.2859534Z    |
---
2019-12-20T16:15:02.2860714Z 
2019-12-20T16:15:02.2860962Z ---- [ui] ui/imports/extern-prelude-extern-crate-restricted-shadowing.rs stdout ----
2019-12-20T16:15:02.2861035Z diff of stderr:
2019-12-20T16:15:02.2861069Z 
2019-12-20T16:15:02.2861137Z 22 LL | define_vec!();
2019-12-20T16:15:02.2861439Z 24 note: `Vec` could also refer to the struct defined here
2019-12-20T16:15:02.2861439Z 24 note: `Vec` could also refer to the struct defined here
2019-12-20T16:15:02.2861842Z -   --> $SRC_DIR/libstd/prelude/v1.rs:LL:COL
2019-12-20T16:15:02.2862370Z - LL | pub use crate::vec::Vec;
2019-12-20T16:15:02.2862556Z -    |         ^^^^^^^^^^^^^^^
2019-12-20T16:15:02.2862637Z 29 
2019-12-20T16:15:02.2862691Z 30 error: aborting due to 2 previous errors
2019-12-20T16:15:02.2862691Z 30 error: aborting due to 2 previous errors
2019-12-20T16:15:02.2862765Z 31 
2019-12-20T16:15:02.2862794Z 
2019-12-20T16:15:02.2862824Z 
2019-12-20T16:15:02.2862897Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2863253Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing/extern-prelude-extern-crate-restricted-shadowing.stderr
2019-12-20T16:15:02.2863530Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2863835Z To only update this specific test, also pass `--test-args imports/extern-prelude-extern-crate-restricted-shadowing.rs`
2019-12-20T16:15:02.2863966Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2864024Z status: exit code: 1
2019-12-20T16:15:02.2864024Z status: exit code: 1
2019-12-20T16:15:02.2865137Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2865658Z ------------------------------------------
2019-12-20T16:15:02.2865728Z 
2019-12-20T16:15:02.2865934Z ------------------------------------------
2019-12-20T16:15:02.2866011Z stderr:
2019-12-20T16:15:02.2866011Z stderr:
2019-12-20T16:15:02.2866192Z ------------------------------------------
2019-12-20T16:15:02.2866533Z error: macro-expanded `extern crate` items cannot shadow names passed with `--extern`
2019-12-20T16:15:02.2867196Z   --> /checkout/src/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing.rs:22:9
2019-12-20T16:15:02.2867293Z    |
2019-12-20T16:15:02.2867375Z LL |         extern crate std as core;
2019-12-20T16:15:02.2867447Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-20T16:15:02.2867533Z ...
2019-12-20T16:15:02.2867595Z LL | define_other_core!();
2019-12-20T16:15:02.2867925Z 
2019-12-20T16:15:02.2867925Z 
2019-12-20T16:15:02.2868285Z error[E0659]: `Vec` is ambiguous (macro-expanded name vs less macro-expanded name from outer scope during import/macro resolution)
2019-12-20T16:15:02.2868741Z    |
2019-12-20T16:15:02.2868812Z LL |         Vec::panic!(); //~ ERROR `Vec` is ambiguous
2019-12-20T16:15:02.2868907Z    |         ^^^ ambiguous name
2019-12-20T16:15:02.2869001Z    |
2019-12-20T16:15:02.2869001Z    |
2019-12-20T16:15:02.2869069Z note: `Vec` could refer to the crate imported here
2019-12-20T16:15:02.2869396Z   --> /checkout/src/test/ui/imports/extern-prelude-extern-crate-restricted-shadowing.rs:8:9
2019-12-20T16:15:02.2869485Z    |
2019-12-20T16:15:02.2869564Z LL |         extern crate std as Vec;
2019-12-20T16:15:02.2869637Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^
2019-12-20T16:15:02.2869724Z ...
2019-12-20T16:15:02.2869786Z LL | define_vec!();
2019-12-20T16:15:02.2870290Z note: `Vec` could also refer to the struct defined here
2019-12-20T16:15:02.2870351Z 
2019-12-20T16:15:02.2870601Z error: aborting due to 2 previous errors
2019-12-20T16:15:02.2870655Z 
---
2019-12-20T16:15:02.2871443Z ---- [ui] ui/in-band-lifetimes/mismatched_trait_impl-2.rs stdout ----
2019-12-20T16:15:02.2871527Z diff of stderr:
2019-12-20T16:15:02.2871561Z 
2019-12-20T16:15:02.2871611Z 3    |
2019-12-20T16:15:02.2871818Z 4 LL |     fn deref(&self) -> &dyn Trait {
2019-12-20T16:15:02.2872058Z 5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found fn(&Struct) -> &dyn Trait
2019-12-20T16:15:02.2872232Z -    | 
2019-12-20T16:15:02.2872609Z -   ::: $SRC_DIR/libcore/ops/deref.rs:LL:COL
2019-12-20T16:15:02.2872941Z -    |
2019-12-20T16:15:02.2873314Z - LL |     fn deref(&self) -> &Self::Target;
2019-12-20T16:15:02.2873758Z -    |     --------------------------------- expected fn(&Struct) -> &(dyn Trait + 'static)
2019-12-20T16:15:02.2873848Z 11    |
2019-12-20T16:15:02.2874059Z 12    = note: expected `fn(&Struct) -> &(dyn Trait + 'static)`
2019-12-20T16:15:02.2874289Z 13               found `fn(&Struct) -> &dyn Trait`
2019-12-20T16:15:02.2874367Z 
2019-12-20T16:15:02.2874441Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2874441Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2874777Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl-2/mismatched_trait_impl-2.stderr
2019-12-20T16:15:02.2875026Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2875306Z To only update this specific test, also pass `--test-args in-band-lifetimes/mismatched_trait_impl-2.rs`
2019-12-20T16:15:02.2875431Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2875491Z status: exit code: 1
2019-12-20T16:15:02.2875491Z status: exit code: 1
2019-12-20T16:15:02.2876959Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/in-band-lifetimes/mismatched_trait_impl-2.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl-2" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/in-band-lifetimes/mismatched_trait_impl-2/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2877806Z ------------------------------------------
2019-12-20T16:15:02.2877876Z 
2019-12-20T16:15:02.2878109Z ------------------------------------------
2019-12-20T16:15:02.2878198Z stderr:
2019-12-20T16:15:02.2878198Z stderr:
2019-12-20T16:15:02.2878424Z ------------------------------------------
2019-12-20T16:15:02.2878721Z error: `impl` item signature doesn't match `trait` item signature
2019-12-20T16:15:02.2879022Z   --> /checkout/src/test/ui/in-band-lifetimes/mismatched_trait_impl-2.rs:11:5
2019-12-20T16:15:02.2879124Z    |
2019-12-20T16:15:02.2879370Z LL |     fn deref(&self) -> &dyn Trait {
2019-12-20T16:15:02.2879643Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ found fn(&Struct) -> &dyn Trait
2019-12-20T16:15:02.2879749Z    |
2019-12-20T16:15:02.2880000Z    = note: expected `fn(&Struct) -> &(dyn Trait + 'static)`
2019-12-20T16:15:02.2880271Z               found `fn(&Struct) -> &dyn Trait`
2019-12-20T16:15:02.2880563Z error: aborting due to previous error
2019-12-20T16:15:02.2880602Z 
2019-12-20T16:15:02.2880632Z 
2019-12-20T16:15:02.2880856Z ------------------------------------------
2019-12-20T16:15:02.2880856Z ------------------------------------------
2019-12-20T16:15:02.2880898Z 
2019-12-20T16:15:02.2880928Z 
2019-12-20T16:15:02.2881165Z ---- [ui] ui/interior-mutability/interior-mutability.rs stdout ----
2019-12-20T16:15:02.2881234Z diff of stderr:
2019-12-20T16:15:02.2881286Z 
2019-12-20T16:15:02.2881343Z 3    |
2019-12-20T16:15:02.2881577Z 4 LL |     catch_unwind(|| { x.set(23); });
2019-12-20T16:15:02.2881672Z 5    |     ^^^^^^^^^^^^ `std::cell::UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
2019-12-20T16:15:02.2881907Z -    | 
2019-12-20T16:15:02.2882109Z -   ::: $SRC_DIR/libstd/panic.rs:LL:COL
2019-12-20T16:15:02.2882275Z -    |
2019-12-20T16:15:02.2882518Z - LL | pub fn catch_unwind<F: FnOnce() -> R + UnwindSafe, R>(f: F) -> Result<R> {
2019-12-20T16:15:02.2883072Z 11    |
2019-12-20T16:15:02.2883072Z 11    |
2019-12-20T16:15:02.2883151Z 12    = help: within `std::cell::Cell<i32>`, the trait `std::panic::RefUnwindSafe` is not implemented for `std::cell::UnsafeCell<i32>`
2019-12-20T16:15:02.2883267Z 13    = note: required because it appears within the type `std::cell::Cell<i32>`
2019-12-20T16:15:02.2883372Z 
2019-12-20T16:15:02.2883432Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2884134Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/interior-mutability.stderr
2019-12-20T16:15:02.2884134Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/interior-mutability.stderr
2019-12-20T16:15:02.2884413Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2884887Z To only update this specific test, also pass `--test-args interior-mutability/interior-mutability.rs`
2019-12-20T16:15:02.2885872Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2885941Z status: exit code: 1
2019-12-20T16:15:02.2885941Z status: exit code: 1
2019-12-20T16:15:02.2887788Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/interior-mutability/interior-mutability.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/interior-mutability/interior-mutability/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2888479Z ------------------------------------------
2019-12-20T16:15:02.2888550Z 
2019-12-20T16:15:02.2888789Z ------------------------------------------
2019-12-20T16:15:02.2888878Z stderr:
2019-12-20T16:15:02.2888878Z stderr:
2019-12-20T16:15:02.2889106Z ------------------------------------------
2019-12-20T16:15:02.2889243Z error[E0277]: the type `std::cell::UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
2019-12-20T16:15:02.2889670Z    |
2019-12-20T16:15:02.2889670Z    |
2019-12-20T16:15:02.2889760Z LL |     catch_unwind(|| { x.set(23); });
2019-12-20T16:15:02.2889877Z    |     ^^^^^^^^^^^^ `std::cell::UnsafeCell<i32>` may contain interior mutability and a reference may not be safely transferrable across a catch_unwind boundary
2019-12-20T16:15:02.2890009Z    |
2019-12-20T16:15:02.2890105Z    = help: within `std::cell::Cell<i32>`, the trait `std::panic::RefUnwindSafe` is not implemented for `std::cell::UnsafeCell<i32>`
2019-12-20T16:15:02.2890394Z    = note: required because it appears within the type `std::cell::Cell<i32>`
2019-12-20T16:15:02.2890509Z    = note: required because of the requirements on the impl of `std::panic::UnwindSafe` for `&std::cell::Cell<i32>`
2019-12-20T16:15:02.2891046Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/interior-mutability/interior-mutability.rs:8:18: 8:35 x:&std::cell::Cell<i32>]`
2019-12-20T16:15:02.2891194Z error: aborting due to previous error
2019-12-20T16:15:02.2891251Z 
2019-12-20T16:15:02.2891485Z For more information about this error, try `rustc --explain E0277`.
2019-12-20T16:15:02.2891535Z 
---
2019-12-20T16:15:02.2892538Z 3    |
2019-12-20T16:15:02.2892589Z 4 LL | struct Foo(Bar);
2019-12-20T16:15:02.2892827Z 5    |            ^^^ the trait `std::hash::Hash` is not implemented for `Bar`
2019-12-20T16:15:02.2892996Z -    | 
2019-12-20T16:15:02.2893190Z -   ::: $SRC_DIR/libcore/hash/mod.rs:LL:COL
2019-12-20T16:15:02.2893550Z - LL |     fn hash<H: Hasher>(&self, state: &mut H);
2019-12-20T16:15:02.2893765Z -    |             - required by this bound in `std::hash::Hash::hash`
2019-12-20T16:15:02.2894018Z 11 
2019-12-20T16:15:02.2894075Z 12 error: aborting due to previous error
2019-12-20T16:15:02.2894075Z 12 error: aborting due to previous error
2019-12-20T16:15:02.2894147Z 13 
2019-12-20T16:15:02.2894176Z 
2019-12-20T16:15:02.2894204Z 
2019-12-20T16:15:02.2894277Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2894565Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21160/issue-21160.stderr
2019-12-20T16:15:02.2894807Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2895059Z To only update this specific test, also pass `--test-args issues/issue-21160.rs`
2019-12-20T16:15:02.2895174Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2895231Z status: exit code: 1
2019-12-20T16:15:02.2895231Z status: exit code: 1
2019-12-20T16:15:02.2896128Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-21160.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21160" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-21160/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2896616Z ------------------------------------------
2019-12-20T16:15:02.2896674Z 
2019-12-20T16:15:02.2897235Z ------------------------------------------
2019-12-20T16:15:02.2897331Z stderr:
---
2019-12-20T16:15:02.2899535Z 3    |
2019-12-20T16:15:02.2899615Z 4 LL |         None @ _ => {}
2019-12-20T16:15:02.2899698Z 5    |         ^^^^ cannot be named the same as a unit variant
2019-12-20T16:15:02.2899951Z -    | 
2019-12-20T16:15:02.2900207Z -   ::: $SRC_DIR/libstd/prelude/v1.rs:LL:COL
2019-12-20T16:15:02.2900562Z -    |
2019-12-20T16:15:02.2900939Z - LL | pub use crate::option::Option::{self, None, Some};
2019-12-20T16:15:02.2901212Z -    |                                       ---- the unit variant `None` is defined here
2019-12-20T16:15:02.2901358Z 12 error[E0530]: match bindings cannot shadow constants
2019-12-20T16:15:02.2901577Z 13   --> $DIR/issue-27033.rs:10:9
2019-12-20T16:15:02.2901786Z 
2019-12-20T16:15:02.2901814Z 
2019-12-20T16:15:02.2901814Z 
2019-12-20T16:15:02.2901885Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2902153Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27033/issue-27033.stderr
2019-12-20T16:15:02.2902402Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2902638Z To only update this specific test, also pass `--test-args issues/issue-27033.rs`
2019-12-20T16:15:02.2902752Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2902827Z status: exit code: 1
2019-12-20T16:15:02.2902827Z status: exit code: 1
2019-12-20T16:15:02.2903660Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-27033.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27033" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-27033/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2904091Z ------------------------------------------
2019-12-20T16:15:02.2904131Z 
2019-12-20T16:15:02.2904331Z ------------------------------------------
2019-12-20T16:15:02.2904388Z stderr:
2019-12-20T16:15:02.2904388Z stderr:
2019-12-20T16:15:02.2904584Z ------------------------------------------
2019-12-20T16:15:02.2904664Z error[E0530]: match bindings cannot shadow unit variants
2019-12-20T16:15:02.2905125Z   --> /checkout/src/test/ui/issues/issue-27033.rs:6:9
2019-12-20T16:15:02.2905211Z    |
2019-12-20T16:15:02.2905270Z LL |         None @ _ => {} //~ ERROR match bindings cannot shadow unit variants
2019-12-20T16:15:02.2905364Z    |         ^^^^ cannot be named the same as a unit variant
2019-12-20T16:15:02.2905679Z error[E0530]: match bindings cannot shadow constants
2019-12-20T16:15:02.2905932Z   --> /checkout/src/test/ui/issues/issue-27033.rs:10:9
2019-12-20T16:15:02.2906011Z    |
2019-12-20T16:15:02.2906062Z LL |     const C: u8 = 1;
2019-12-20T16:15:02.2906062Z LL |     const C: u8 = 1;
2019-12-20T16:15:02.2906312Z    |     ---------------- the constant `C` is defined here
2019-12-20T16:15:02.2906377Z LL |     match 1 {
2019-12-20T16:15:02.2906458Z LL |         C @ 2 => { //~ ERROR match bindings cannot shadow constant
2019-12-20T16:15:02.2906547Z    |         ^ cannot be named the same as a constant
2019-12-20T16:15:02.2906638Z error: aborting due to 2 previous errors
2019-12-20T16:15:02.2906692Z 
2019-12-20T16:15:02.2907283Z For more information about this error, try `rustc --explain E0530`.
2019-12-20T16:15:02.2907364Z 
2019-12-20T16:15:02.2907364Z 
2019-12-20T16:15:02.2907600Z ------------------------------------------
2019-12-20T16:15:02.2907649Z 
2019-12-20T16:15:02.2907703Z 
2019-12-20T16:15:02.2907937Z ---- [ui] ui/no-send-res-ports.rs stdout ----
2019-12-20T16:15:02.2908038Z diff of stderr:
2019-12-20T16:15:02.2908079Z 
2019-12-20T16:15:02.2908160Z 3    |
2019-12-20T16:15:02.2908223Z 4 LL |     thread::spawn(move|| {
2019-12-20T16:15:02.2908326Z 5    |     ^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-12-20T16:15:02.2908546Z -    | 
2019-12-20T16:15:02.2908796Z -   ::: $SRC_DIR/libstd/thread/mod.rs:LL:COL
2019-12-20T16:15:02.2909000Z -    |
2019-12-20T16:15:02.2909278Z - LL |     F: FnOnce() -> T, F: Send + 'static, T: Send + 'static
2019-12-20T16:15:02.2909577Z -    |                          ---- required by this bound in `std::thread::spawn`
2019-12-20T16:15:02.2909682Z 11    |
2019-12-20T16:15:02.2910069Z 12    = help: within `[closure@$DIR/no-send-res-ports.rs:28:19: 32:6 x:main::Foo]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-12-20T16:15:02.2910212Z 13    = note: required because it appears within the type `Port<()>`
2019-12-20T16:15:02.2910672Z 
2019-12-20T16:15:02.2910734Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2911406Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/no-send-res-ports.stderr
2019-12-20T16:15:02.2911406Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/no-send-res-ports.stderr
2019-12-20T16:15:02.2911820Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2912247Z To only update this specific test, also pass `--test-args no-send-res-ports.rs`
2019-12-20T16:15:02.2912364Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2912421Z status: exit code: 1
2019-12-20T16:15:02.2912421Z status: exit code: 1
2019-12-20T16:15:02.2913245Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/no-send-res-ports.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/no-send-res-ports/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2913684Z ------------------------------------------
2019-12-20T16:15:02.2913724Z 
2019-12-20T16:15:02.2913907Z ------------------------------------------
2019-12-20T16:15:02.2913983Z stderr:
2019-12-20T16:15:02.2913983Z stderr:
2019-12-20T16:15:02.2914182Z ------------------------------------------
2019-12-20T16:15:02.2914251Z error[E0277]: `std::rc::Rc<()>` cannot be sent between threads safely
2019-12-20T16:15:02.2914807Z    |
2019-12-20T16:15:02.2914807Z    |
2019-12-20T16:15:02.2914879Z LL |     thread::spawn(move|| {
2019-12-20T16:15:02.2914944Z    |     ^^^^^^^^^^^^^ `std::rc::Rc<()>` cannot be sent between threads safely
2019-12-20T16:15:02.2915024Z    |
2019-12-20T16:15:02.2915532Z    = help: within `[closure@/checkout/src/test/ui/no-send-res-ports.rs:28:19: 32:6 x:main::Foo]`, the trait `std::marker::Send` is not implemented for `std::rc::Rc<()>`
2019-12-20T16:15:02.2915732Z    = note: required because it appears within the type `Port<()>`
2019-12-20T16:15:02.2915824Z    = note: required because it appears within the type `main::Foo`
2019-12-20T16:15:02.2916137Z    = note: required because it appears within the type `[closure@/checkout/src/test/ui/no-send-res-ports.rs:28:19: 32:6 x:main::Foo]`
2019-12-20T16:15:02.2916267Z error: aborting due to previous error
2019-12-20T16:15:02.2916303Z 
2019-12-20T16:15:02.2916533Z For more information about this error, try `rustc --explain E0277`.
2019-12-20T16:15:02.2916585Z 
---
2019-12-20T16:15:02.2917700Z diff of stderr:
2019-12-20T16:15:02.2917741Z 
2019-12-20T16:15:02.2917804Z 5 LL | |     "0".parse()
2019-12-20T16:15:02.2917891Z 6 LL | | }
2019-12-20T16:15:02.2917971Z 7    | |_^ `main` can only return types that implement `std::process::Termination`
2019-12-20T16:15:02.2918213Z -    | 
2019-12-20T16:15:02.2918455Z -   ::: $SRC_DIR/libtest/lib.rs:LL:COL
2019-12-20T16:15:02.2918661Z -    |
2019-12-20T16:15:02.2918933Z - LL |   pub fn assert_test_result<T: Termination>(result: T) {
2019-12-20T16:15:02.2919365Z 13    |
2019-12-20T16:15:02.2919365Z 13    |
2019-12-20T16:15:02.2919468Z 14    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-12-20T16:15:02.2919627Z 
2019-12-20T16:15:02.2919663Z 
2019-12-20T16:15:02.2919750Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2920195Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
2019-12-20T16:15:02.2920195Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/termination-trait-test-wrong-type.stderr
2019-12-20T16:15:02.2920640Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2920958Z To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-test-wrong-type.rs`
2019-12-20T16:15:02.2921098Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2921160Z status: exit code: 1
2019-12-20T16:15:02.2921160Z status: exit code: 1
2019-12-20T16:15:02.2922428Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2922948Z ------------------------------------------
2019-12-20T16:15:02.2923009Z 
2019-12-20T16:15:02.2923216Z ------------------------------------------
2019-12-20T16:15:02.2923299Z stderr:
2019-12-20T16:15:02.2923299Z stderr:
2019-12-20T16:15:02.2923503Z ------------------------------------------
2019-12-20T16:15:02.2923699Z error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseFloatError>`
2019-12-20T16:15:02.2924130Z    |
2019-12-20T16:15:02.2924130Z    |
2019-12-20T16:15:02.2924397Z LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> { //~ ERROR
2019-12-20T16:15:02.2924569Z LL | |     "0".parse()
2019-12-20T16:15:02.2924733Z LL | | }
2019-12-20T16:15:02.2924803Z    | |_^ `main` can only return types that implement `std::process::Termination`
2019-12-20T16:15:02.2924895Z    |
2019-12-20T16:15:02.2925314Z    = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseFloatError>`
2019-12-20T16:15:02.2925459Z error: aborting due to previous error
2019-12-20T16:15:02.2925517Z 
2019-12-20T16:15:02.2925963Z For more information about this error, try `rustc --explain E0277`.
2019-12-20T16:15:02.2926029Z 
---
2019-12-20T16:15:02.2927517Z 6 LL |     // suggest a where-clause, if needed
2019-12-20T16:15:02.2927595Z 7 LL |     mem::size_of::<U>();
2019-12-20T16:15:02.2927886Z 8    |                    ^ doesn't have a size known at compile-time
2019-12-20T16:15:02.2928098Z -    | 
2019-12-20T16:15:02.2928343Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-12-20T16:15:02.2928414Z 11    |
2019-12-20T16:15:02.2928661Z - LL | pub const fn size_of<T>() -> usize {
2019-12-20T16:15:02.2929170Z -    |
2019-12-20T16:15:02.2929250Z 15    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-12-20T16:15:02.2929657Z 16    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-20T16:15:02.2929781Z 17 
2019-12-20T16:15:02.2929781Z 17 
2019-12-20T16:15:02.2929819Z 
2019-12-20T16:15:02.2929879Z 23 ...
2019-12-20T16:15:02.2929960Z 24 LL |     mem::size_of::<Misc<U>>();
2019-12-20T16:15:02.2930793Z -    | 
2019-12-20T16:15:02.2930793Z -    | 
2019-12-20T16:15:02.2930995Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-12-20T16:15:02.2931055Z 28    |
2019-12-20T16:15:02.2931259Z - LL | pub const fn size_of<T>() -> usize {
2019-12-20T16:15:02.2932042Z -    |
2019-12-20T16:15:02.2932042Z -    |
2019-12-20T16:15:02.2932112Z 32    = help: within `Misc<U>`, the trait `std::marker::Sized` is not implemented for `U`
2019-12-20T16:15:02.2932566Z 34    = note: required because it appears within the type `Misc<U>`
2019-12-20T16:15:02.2932630Z 
2019-12-20T16:15:02.2932682Z 62    |
2019-12-20T16:15:02.2932682Z 62    |
2019-12-20T16:15:02.2932753Z 63 LL |     mem::size_of::<[T]>();
2019-12-20T16:15:02.2933208Z -    | 
2019-12-20T16:15:02.2933208Z -    | 
2019-12-20T16:15:02.2933406Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-12-20T16:15:02.2933486Z 67    |
2019-12-20T16:15:02.2933705Z - LL | pub const fn size_of<T>() -> usize {
2019-12-20T16:15:02.2934152Z -    |
2019-12-20T16:15:02.2934220Z 71    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-12-20T16:15:02.2934740Z 72    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-20T16:15:02.2934996Z 73 
2019-12-20T16:15:02.2934996Z 73 
2019-12-20T16:15:02.2935045Z 
2019-12-20T16:15:02.2935194Z 76    |
2019-12-20T16:15:02.2935273Z 77 LL |     mem::size_of::<[&U]>();
2019-12-20T16:15:02.2935918Z -    | 
2019-12-20T16:15:02.2935918Z -    | 
2019-12-20T16:15:02.2936218Z -   ::: $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-12-20T16:15:02.2936416Z -    |
2019-12-20T16:15:02.2936616Z - LL | pub const fn size_of<T>() -> usize {
2019-12-20T16:15:02.2937379Z 84    |
2019-12-20T16:15:02.2937473Z 85    = help: the trait `std::marker::Sized` is not implemented for `[&U]`
2019-12-20T16:15:02.2937868Z 86    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-20T16:15:02.2937948Z 
2019-12-20T16:15:02.2937948Z 
2019-12-20T16:15:02.2937984Z 
2019-12-20T16:15:02.2938071Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2938459Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/trait-suggest-where-clause.stderr
2019-12-20T16:15:02.2938781Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2939108Z To only update this specific test, also pass `--test-args traits/trait-suggest-where-clause.rs`
2019-12-20T16:15:02.2939248Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2939339Z status: exit code: 1
2019-12-20T16:15:02.2939339Z status: exit code: 1
2019-12-20T16:15:02.2940703Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/trait-suggest-where-clause.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/trait-suggest-where-clause/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2941144Z ------------------------------------------
2019-12-20T16:15:02.2941184Z 
2019-12-20T16:15:02.2941387Z ------------------------------------------
2019-12-20T16:15:02.2941460Z stderr:
2019-12-20T16:15:02.2941460Z stderr:
2019-12-20T16:15:02.2941641Z ------------------------------------------
2019-12-20T16:15:02.2941720Z error[E0277]: the size for values of type `U` cannot be known at compilation time
2019-12-20T16:15:02.2942079Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:10:20
2019-12-20T16:15:02.2967607Z    |
2019-12-20T16:15:02.2967728Z LL | fn check<T: Iterator, U: ?Sized>() {
2019-12-20T16:15:02.2968247Z    |                       -- help: consider further restricting this bound: `U: std::marker::Sized +`
2019-12-20T16:15:02.2968638Z LL |     mem::size_of::<U>();
2019-12-20T16:15:02.2968914Z    |                    ^ doesn't have a size known at compile-time
2019-12-20T16:15:02.2969009Z    |
2019-12-20T16:15:02.2969084Z    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-12-20T16:15:02.2969084Z    = help: the trait `std::marker::Sized` is not implemented for `U`
2019-12-20T16:15:02.2969482Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-20T16:15:02.2969568Z 
2019-12-20T16:15:02.2969668Z error[E0277]: the size for values of type `U` cannot be known at compilation time
2019-12-20T16:15:02.2969977Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:13:5
2019-12-20T16:15:02.2970059Z    |
2019-12-20T16:15:02.2970137Z LL | fn check<T: Iterator, U: ?Sized>() {
2019-12-20T16:15:02.2970451Z    |                       -- help: consider further restricting this bound: `U: std::marker::Sized +`
2019-12-20T16:15:02.2970549Z ...
2019-12-20T16:15:02.2970907Z LL |     mem::size_of::<Misc<U>>();
2019-12-20T16:15:02.2971263Z    |
2019-12-20T16:15:02.2971263Z    |
2019-12-20T16:15:02.2971338Z    = help: within `Misc<U>`, the trait `std::marker::Sized` is not implemented for `U`
2019-12-20T16:15:02.2971853Z    = note: required because it appears within the type `Misc<U>`
2019-12-20T16:15:02.2971902Z 
2019-12-20T16:15:02.2971902Z 
2019-12-20T16:15:02.2971974Z error[E0277]: the trait bound `u64: std::convert::From<T>` is not satisfied
2019-12-20T16:15:02.2972319Z    |
2019-12-20T16:15:02.2972319Z    |
2019-12-20T16:15:02.2972378Z LL |     <u64 as From<T>>::from;
2019-12-20T16:15:02.2972456Z    |     ^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<T>` is not implemented for `u64`
2019-12-20T16:15:02.2972606Z    = note: required by `std::convert::From::from`
2019-12-20T16:15:02.2972661Z 
2019-12-20T16:15:02.2972661Z 
2019-12-20T16:15:02.2972735Z error[E0277]: the trait bound `u64: std::convert::From<<T as std::iter::Iterator>::Item>` is not satisfied
2019-12-20T16:15:02.2973092Z    |
2019-12-20T16:15:02.2973092Z    |
2019-12-20T16:15:02.2973163Z LL |     <u64 as From<<T as Iterator>::Item>>::from;
2019-12-20T16:15:02.2973259Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<<T as std::iter::Iterator>::Item>` is not implemented for `u64`
2019-12-20T16:15:02.2973416Z    = note: required by `std::convert::From::from`
2019-12-20T16:15:02.2973470Z 
2019-12-20T16:15:02.2973470Z 
2019-12-20T16:15:02.2973536Z error[E0277]: the trait bound `Misc<_>: std::convert::From<T>` is not satisfied
2019-12-20T16:15:02.2973873Z    |
2019-12-20T16:15:02.2973873Z    |
2019-12-20T16:15:02.2973941Z LL |     <Misc<_> as From<T>>::from;
2019-12-20T16:15:02.2974023Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::convert::From<T>` is not implemented for `Misc<_>`
2019-12-20T16:15:02.2974170Z    = note: required by `std::convert::From::from`
2019-12-20T16:15:02.2974219Z 
2019-12-20T16:15:02.2974286Z error[E0277]: the size for values of type `[T]` cannot be known at compilation time
2019-12-20T16:15:02.2974732Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:31:20
2019-12-20T16:15:02.2974732Z   --> /checkout/src/test/ui/traits/trait-suggest-where-clause.rs:31:20
2019-12-20T16:15:02.2974810Z    |
2019-12-20T16:15:02.2974863Z LL |     mem::size_of::<[T]>();
2019-12-20T16:15:02.2975166Z    |
2019-12-20T16:15:02.2975238Z    = help: the trait `std::marker::Sized` is not implemented for `[T]`
2019-12-20T16:15:02.2975558Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-20T16:15:02.2975640Z 
2019-12-20T16:15:02.2975640Z 
2019-12-20T16:15:02.2975707Z error[E0277]: the size for values of type `[&U]` cannot be known at compilation time
2019-12-20T16:15:02.2976417Z    |
2019-12-20T16:15:02.2976417Z    |
2019-12-20T16:15:02.2976481Z LL |     mem::size_of::<[&U]>();
2019-12-20T16:15:02.2977238Z    |
2019-12-20T16:15:02.2977311Z    = help: the trait `std::marker::Sized` is not implemented for `[&U]`
2019-12-20T16:15:02.2977729Z    = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
2019-12-20T16:15:02.2977807Z 
---
2019-12-20T16:15:02.2978697Z 
2019-12-20T16:15:02.2978970Z ---- [ui] ui/type_length_limit.rs stdout ----
2019-12-20T16:15:02.2979046Z diff of stderr:
2019-12-20T16:15:02.2979174Z 
2019-12-20T16:15:02.2979539Z 1 error: reached the type-length limit while instantiating `std::mem::drop::<std::option::Op... G), (G, G, G), (G, G, G))))))>>`
2019-12-20T16:15:02.2979813Z -   --> $SRC_DIR/libcore/mem/mod.rs:LL:COL
2019-12-20T16:15:02.2980016Z -    |
2019-12-20T16:15:02.2980404Z - LL | pub fn drop<T>(_x: T) { }
2019-12-20T16:15:02.2980835Z 6    |
2019-12-20T16:15:02.2980835Z 6    |
2019-12-20T16:15:02.2981070Z 7    = note: consider adding a `#![type_length_limit="1094"]` attribute to your crate
2019-12-20T16:15:02.2981182Z 
2019-12-20T16:15:02.2981222Z 
2019-12-20T16:15:02.2981450Z The actual stderr differed from the expected stderr.
2019-12-20T16:15:02.2981763Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
2019-12-20T16:15:02.2981763Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
2019-12-20T16:15:02.2982034Z To update references, rerun the tests and pass the `--bless` flag
2019-12-20T16:15:02.2982302Z To only update this specific test, also pass `--test-args type_length_limit.rs`
2019-12-20T16:15:02.2982432Z error: 1 errors occurred comparing output.
2019-12-20T16:15:02.2982497Z status: exit code: 1
2019-12-20T16:15:02.2982497Z status: exit code: 1
2019-12-20T16:15:02.2983397Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=i686-unknown-linux-musl" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "-Clinker=/musl-i686/bin/musl-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary" "-A" "unused"
2019-12-20T16:15:02.2983896Z ------------------------------------------
2019-12-20T16:15:02.2983949Z 
2019-12-20T16:15:02.2984170Z ------------------------------------------
2019-12-20T16:15:02.2984244Z stderr:
2019-12-20T16:15:02.2984244Z stderr:
2019-12-20T16:15:02.2984451Z ------------------------------------------
2019-12-20T16:15:02.2984769Z error: reached the type-length limit while instantiating `std::mem::drop::<std::option::Op... G), (G, G, G), (G, G, G))))))>>`
2019-12-20T16:15:02.2984861Z    |
2019-12-20T16:15:02.2984946Z    = note: consider adding a `#![type_length_limit="1094"]` attribute to your crate
2019-12-20T16:15:02.2985240Z error: aborting due to previous error
2019-12-20T16:15:02.2985281Z 
2019-12-20T16:15:02.2985311Z 
2019-12-20T16:15:02.2985713Z ------------------------------------------
---
2019-12-20T16:15:02.2992388Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-20T16:15:02.2992488Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-20T16:15:02.2992607Z 
2019-12-20T16:15:02.2992639Z 
2019-12-20T16:15:02.2994801Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-musl/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-musl" "--mode" "ui" "--target" "i686-unknown-linux-musl" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "/musl-i686/bin/musl-gcc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-musl/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2019-12-20T16:15:02.2995444Z 
2019-12-20T16:15:02.2995479Z 
2019-12-20T16:15:02.2995959Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2019-12-20T16:15:02.2996061Z Build completed unsuccessfully in 1:33:42
2019-12-20T16:15:02.2996061Z Build completed unsuccessfully in 1:33:42
2019-12-20T16:15:02.2996127Z == clock drift check ==
2019-12-20T16:15:02.2996197Z   local time: Fri Dec 20 16:15:02 UTC 2019
2019-12-20T16:15:02.5734612Z   network time: Fri, 20 Dec 2019 16:15:02 GMT
2019-12-20T16:15:02.5735267Z == end clock drift check ==
2019-12-20T16:15:03.6045726Z 
2019-12-20T16:15:03.6187240Z ##[error]Bash exited with code '1'.
2019-12-20T16:15:03.6231425Z ##[section]Starting: Checkout
2019-12-20T16:15:03.6233057Z ==============================================================================
2019-12-20T16:15:03.6233166Z Task         : Get sources
2019-12-20T16:15:03.6233231Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
