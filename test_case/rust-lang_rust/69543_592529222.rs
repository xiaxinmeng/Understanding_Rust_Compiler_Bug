plain
2020-02-28T14:09:47.2637160Z 
2020-02-28T14:09:47.2638033Z ---- [ui] ui/suggestions/mut-borrow-needed-by-trait.rs stdout ----
2020-02-28T14:09:47.2638378Z diff of stderr:
2020-02-28T14:09:47.2638559Z 
2020-02-28T14:09:47.2639034Z 28 error[E0599]: no method named `write_fmt` found for struct `std::io::BufWriter<&dyn std::io::Write>` in the current scope
2020-02-28T14:09:47.2640119Z 30    |
2020-02-28T14:09:47.2640119Z 30    |
2020-02-28T14:09:47.2640578Z - LL |     writeln!(fp, "hello world").unwrap();
2020-02-28T14:09:47.2641309Z -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `std::io::BufWriter<&dyn std::io::Write>`
2020-02-28T14:09:47.2641899Z -    | 
2020-02-28T14:09:47.2642585Z -   ::: $SRC_DIR/libstd/io/buffered.rs:LL:COL
2020-02-28T14:09:47.2643015Z -    |
2020-02-28T14:09:47.2643439Z - LL | pub struct BufWriter<W: Write> {
2020-02-28T14:09:47.2644090Z -    | ------------------------------ doesn't satisfy `_: std::io::Write`
2020-02-28T14:09:47.2644527Z + LL |       writeln!(fp, "hello world").unwrap();
2020-02-28T14:09:47.2645051Z +    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `std::io::BufWriter<&dyn std::io::Write>`
2020-02-28T14:09:47.2645832Z 39    = note: the method `write_fmt` exists but the following trait bounds were not satisfied:
2020-02-28T14:09:47.2646323Z 40            `&dyn std::io::Write: std::io::Write`
2020-02-28T14:09:47.2646572Z 
2020-02-28T14:09:47.2646688Z 
2020-02-28T14:09:47.2646688Z 
2020-02-28T14:09:47.2646955Z The actual stderr differed from the expected stderr.
2020-02-28T14:09:47.2647804Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-borrow-needed-by-trait/mut-borrow-needed-by-trait.stderr
2020-02-28T14:09:47.2648814Z To update references, rerun the tests and pass the `--bless` flag
2020-02-28T14:09:47.2649545Z To only update this specific test, also pass `--test-args suggestions/mut-borrow-needed-by-trait.rs`
2020-02-28T14:09:47.2650115Z error: 1 errors occurred comparing output.
2020-02-28T14:09:47.2650404Z status: exit code: 1
2020-02-28T14:09:47.2650404Z status: exit code: 1
2020-02-28T14:09:47.2652849Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/mut-borrow-needed-by-trait.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-borrow-needed-by-trait" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/mut-borrow-needed-by-trait/auxiliary"
2020-02-28T14:09:47.2654829Z ------------------------------------------
2020-02-28T14:09:47.2655056Z 
2020-02-28T14:09:47.2655466Z ------------------------------------------
2020-02-28T14:09:47.2655729Z stderr:
2020-02-28T14:09:47.2655729Z stderr:
2020-02-28T14:09:47.2656146Z ------------------------------------------
2020-02-28T14:09:47.2656607Z error[E0277]: the trait bound `&dyn std::io::Write: std::io::Write` is not satisfied
2020-02-28T14:09:47.2657368Z   --> /checkout/src/test/ui/suggestions/mut-borrow-needed-by-trait.rs:17:29
2020-02-28T14:09:47.2657700Z    |
2020-02-28T14:09:47.2657956Z LL |     let fp = BufWriter::new(fp);
2020-02-28T14:09:47.2658461Z    |                             ^^ the trait `std::io::Write` is not implemented for `&dyn std::io::Write`
2020-02-28T14:09:47.2659317Z    = note: `std::io::Write` is implemented for `&mut dyn std::io::Write`, but not for `&dyn std::io::Write`
2020-02-28T14:09:47.2659892Z    = note: required by `std::io::BufWriter::<W>::new`
2020-02-28T14:09:47.2660148Z 
2020-02-28T14:09:47.2660512Z error[E0277]: the trait bound `&dyn std::io::Write: std::io::Write` is not satisfied
2020-02-28T14:09:47.2660512Z error[E0277]: the trait bound `&dyn std::io::Write: std::io::Write` is not satisfied
2020-02-28T14:09:47.2661254Z   --> /checkout/src/test/ui/suggestions/mut-borrow-needed-by-trait.rs:17:14
2020-02-28T14:09:47.2661594Z    |
2020-02-28T14:09:47.2661836Z LL |     let fp = BufWriter::new(fp);
2020-02-28T14:09:47.2662350Z    |              ^^^^^^^^^^^^^^ the trait `std::io::Write` is not implemented for `&dyn std::io::Write`
2020-02-28T14:09:47.2663200Z    = note: `std::io::Write` is implemented for `&mut dyn std::io::Write`, but not for `&dyn std::io::Write`
2020-02-28T14:09:47.2663732Z    = note: required by `std::io::BufWriter`
2020-02-28T14:09:47.2663953Z 
2020-02-28T14:09:47.2664317Z error[E0277]: the trait bound `&dyn std::io::Write: std::io::Write` is not satisfied
2020-02-28T14:09:47.2664317Z error[E0277]: the trait bound `&dyn std::io::Write: std::io::Write` is not satisfied
2020-02-28T14:09:47.2665062Z   --> /checkout/src/test/ui/suggestions/mut-borrow-needed-by-trait.rs:17:14
2020-02-28T14:09:47.2665404Z    |
2020-02-28T14:09:47.2665645Z LL |     let fp = BufWriter::new(fp);
2020-02-28T14:09:47.2666169Z    |              ^^^^^^^^^^^^^^^^^^ the trait `std::io::Write` is not implemented for `&dyn std::io::Write`
2020-02-28T14:09:47.2667023Z    = note: `std::io::Write` is implemented for `&mut dyn std::io::Write`, but not for `&dyn std::io::Write`
2020-02-28T14:09:47.2667558Z    = note: required by `std::io::BufWriter`
2020-02-28T14:09:47.2667784Z 
2020-02-28T14:09:47.2667784Z 
2020-02-28T14:09:47.2668231Z error[E0599]: no method named `write_fmt` found for struct `std::io::BufWriter<&dyn std::io::Write>` in the current scope
2020-02-28T14:09:47.2669391Z    |
2020-02-28T14:09:47.2669391Z    |
2020-02-28T14:09:47.2669750Z LL |       writeln!(fp, "hello world").unwrap(); //~ ERROR no method named `write_fmt` found for struct
2020-02-28T14:09:47.2670425Z    |       ^^^^^^^^^^^^^^^^^^^^^^^^^^^ method not found in `std::io::BufWriter<&dyn std::io::Write>`
2020-02-28T14:09:47.2671186Z    = note: the method `write_fmt` exists but the following trait bounds were not satisfied:
2020-02-28T14:09:47.2671655Z            `&dyn std::io::Write: std::io::Write`
2020-02-28T14:09:47.2671655Z            `&dyn std::io::Write: std::io::Write`
2020-02-28T14:09:47.2672146Z            which is required by `std::io::BufWriter<&dyn std::io::Write>: std::io::Write`
2020-02-28T14:09:47.2673393Z 
2020-02-28T14:09:47.2673635Z error: aborting due to 4 previous errors
2020-02-28T14:09:47.2673839Z 
2020-02-28T14:09:47.2674126Z Some errors have detailed explanations: E0277, E0599.
---
2020-02-28T14:09:47.2676197Z ---- [ui] ui/unique-object-noncopyable.rs stdout ----
2020-02-28T14:09:47.2676498Z diff of stderr:
2020-02-28T14:09:47.2679392Z 
2020-02-28T14:09:47.2679574Z 9 ...
2020-02-28T14:09:47.2679816Z 10 LL |     let _z = y.clone();
2020-02-28T14:09:47.2680232Z 11    |                ^^^^^ method not found in `std::boxed::Box<dyn Foo>`
2020-02-28T14:09:47.2680910Z -    | 
2020-02-28T14:09:47.2681364Z -   ::: $SRC_DIR/liballoc/boxed.rs:LL:COL
2020-02-28T14:09:47.2681786Z -    |
2020-02-28T14:09:47.2682470Z - LL | pub struct Box<T: ?Sized>(Unique<T>);
2020-02-28T14:09:47.2683244Z -    | ------------------------------------- doesn't satisfy `std::boxed::Box<dyn Foo>: std::clone::Clone`
2020-02-28T14:09:47.2684040Z 18    = note: the method `clone` exists but the following trait bounds were not satisfied:
2020-02-28T14:09:47.2684498Z 19            `dyn Foo: std::marker::Sized`
2020-02-28T14:09:47.2684731Z 
2020-02-28T14:09:47.2684862Z 
2020-02-28T14:09:47.2684862Z 
2020-02-28T14:09:47.2685115Z The actual stderr differed from the expected stderr.
2020-02-28T14:09:47.2685926Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-object-noncopyable/unique-object-noncopyable.stderr
2020-02-28T14:09:47.2686689Z To update references, rerun the tests and pass the `--bless` flag
2020-02-28T14:09:47.2687383Z To only update this specific test, also pass `--test-args unique-object-noncopyable.rs`
2020-02-28T14:09:47.2687934Z error: 1 errors occurred comparing output.
2020-02-28T14:09:47.2688222Z status: exit code: 1
2020-02-28T14:09:47.2688222Z status: exit code: 1
2020-02-28T14:09:47.2690469Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unique-object-noncopyable.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-object-noncopyable" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-object-noncopyable/auxiliary"
2020-02-28T14:09:47.2692371Z ------------------------------------------
2020-02-28T14:09:47.2692579Z 
2020-02-28T14:09:47.2692992Z ------------------------------------------
2020-02-28T14:09:47.2693260Z stderr:
2020-02-28T14:09:47.2693260Z stderr:
2020-02-28T14:09:47.2693694Z ------------------------------------------
2020-02-28T14:09:47.2694170Z error[E0599]: no method named `clone` found for struct `std::boxed::Box<dyn Foo>` in the current scope
2020-02-28T14:09:47.2694923Z   --> /checkout/src/test/ui/unique-object-noncopyable.rs:24:16
2020-02-28T14:09:47.2701947Z    |
2020-02-28T14:09:47.2702213Z LL | trait Foo {
2020-02-28T14:09:47.2702749Z    | ---------
2020-02-28T14:09:47.2702960Z    | |
2020-02-28T14:09:47.2703604Z    | doesn't satisfy `dyn Foo: std::clone::Clone`
2020-02-28T14:09:47.2704190Z    | doesn't satisfy `dyn Foo: std::marker::Sized`
2020-02-28T14:09:47.2704491Z ...
2020-02-28T14:09:47.2704785Z LL |     let _z = y.clone(); //~ ERROR no method named `clone` found
2020-02-28T14:09:47.2705265Z    |                ^^^^^ method not found in `std::boxed::Box<dyn Foo>`
2020-02-28T14:09:47.2766557Z    = note: the method `clone` exists but the following trait bounds were not satisfied:
2020-02-28T14:09:47.2767378Z            `dyn Foo: std::marker::Sized`
2020-02-28T14:09:47.2767378Z            `dyn Foo: std::marker::Sized`
2020-02-28T14:09:47.2767865Z            which is required by `std::boxed::Box<dyn Foo>: std::clone::Clone`
2020-02-28T14:09:47.2768320Z            `dyn Foo: std::clone::Clone`
2020-02-28T14:09:47.2768758Z            which is required by `std::boxed::Box<dyn Foo>: std::clone::Clone`
2020-02-28T14:09:47.2769306Z error: aborting due to previous error
2020-02-28T14:09:47.2769501Z 
2020-02-28T14:09:47.2770265Z For more information about this error, try `rustc --explain E0599`.
2020-02-28T14:09:47.2770524Z 
2020-02-28T14:09:47.2770524Z 
2020-02-28T14:09:47.2770966Z ------------------------------------------
2020-02-28T14:09:47.2771170Z 
2020-02-28T14:09:47.2771286Z 
2020-02-28T14:09:47.2771735Z ---- [ui] ui/unique-pinned-nocopy.rs stdout ----
2020-02-28T14:09:47.2772013Z diff of stderr:
2020-02-28T14:09:47.2772177Z 
2020-02-28T14:09:47.2772333Z 6 ...
2020-02-28T14:09:47.2772562Z 7 LL |     let _j = i.clone();
2020-02-28T14:09:47.2772947Z 8    |                ^^^^^ method not found in `std::boxed::Box<R>`
2020-02-28T14:09:47.2773442Z -    | 
2020-02-28T14:09:47.2773876Z -   ::: $SRC_DIR/liballoc/boxed.rs:LL:COL
2020-02-28T14:09:47.2774296Z -    |
2020-02-28T14:09:47.2774750Z - LL | pub struct Box<T: ?Sized>(Unique<T>);
2020-02-28T14:09:47.2775491Z -    | ------------------------------------- doesn't satisfy `std::boxed::Box<R>: std::clone::Clone`
2020-02-28T14:09:47.2776271Z 15    = note: the method `clone` exists but the following trait bounds were not satisfied:
2020-02-28T14:09:47.2776715Z 16            `R: std::clone::Clone`
2020-02-28T14:09:47.2776921Z 
2020-02-28T14:09:47.2777039Z 
2020-02-28T14:09:47.2777039Z 
2020-02-28T14:09:47.2777304Z The actual stderr differed from the expected stderr.
2020-02-28T14:09:47.2778090Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-pinned-nocopy/unique-pinned-nocopy.stderr
2020-02-28T14:09:47.2778826Z To update references, rerun the tests and pass the `--bless` flag
2020-02-28T14:09:47.2779517Z To only update this specific test, also pass `--test-args unique-pinned-nocopy.rs`
2020-02-28T14:09:47.2780052Z error: 1 errors occurred comparing output.
2020-02-28T14:09:47.2780336Z status: exit code: 1
2020-02-28T14:09:47.2780336Z status: exit code: 1
2020-02-28T14:09:47.2782539Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unique-pinned-nocopy.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-pinned-nocopy" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-A" "unused" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unique-pinned-nocopy/auxiliary"
2020-02-28T14:09:47.2784405Z ------------------------------------------
2020-02-28T14:09:47.2784629Z 
2020-02-28T14:09:47.2785047Z ------------------------------------------
2020-02-28T14:09:47.2785309Z stderr:
2020-02-28T14:09:47.2785309Z stderr:
2020-02-28T14:09:47.2785725Z ------------------------------------------
2020-02-28T14:09:47.2786204Z error[E0599]: no method named `clone` found for struct `std::boxed::Box<R>` in the current scope
2020-02-28T14:09:47.2786931Z   --> /checkout/src/test/ui/unique-pinned-nocopy.rs:12:16
2020-02-28T14:09:47.2787223Z    |
2020-02-28T14:09:47.2787421Z LL | struct R {
2020-02-28T14:09:47.2788020Z    | -------- doesn't satisfy `R: std::clone::Clone`
2020-02-28T14:09:47.2788324Z ...
2020-02-28T14:09:47.2788613Z LL |     let _j = i.clone(); //~ ERROR no method named `clone` found
2020-02-28T14:09:47.2789076Z    |                ^^^^^ method not found in `std::boxed::Box<R>`
2020-02-28T14:09:47.2789737Z    = note: the method `clone` exists but the following trait bounds were not satisfied:
2020-02-28T14:09:47.2790161Z            `R: std::clone::Clone`
2020-02-28T14:09:47.2790161Z            `R: std::clone::Clone`
2020-02-28T14:09:47.2790622Z            which is required by `std::boxed::Box<R>: std::clone::Clone`
2020-02-28T14:09:47.2791661Z    = note: the following trait defines an item `clone`, perhaps you need to implement it:
2020-02-28T14:09:47.2792112Z            candidate #1: `std::clone::Clone`
2020-02-28T14:09:47.2792327Z 
2020-02-28T14:09:47.2792561Z error: aborting due to previous error
---
2020-02-28T14:09:47.2797794Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:348:22
2020-02-28T14:09:47.2798305Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
2020-02-28T14:09:47.2798579Z 
2020-02-28T14:09:47.2798715Z 
2020-02-28T14:09:47.2804140Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.1-rust-1.43.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
2020-02-28T14:09:47.2807298Z 
2020-02-28T14:09:47.2807435Z 
2020-02-28T14:09:47.2808150Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --target i586-unknown-linux-gnu,i686-unknown-linux-musl
2020-02-28T14:09:47.2808672Z Build completed unsuccessfully in 1:22:00
2020-02-28T14:09:47.2808672Z Build completed unsuccessfully in 1:22:00
2020-02-28T14:09:47.2886495Z == clock drift check ==
2020-02-28T14:09:47.2936097Z   local time: Fri Feb 28 14:09:47 UTC 2020
2020-02-28T14:09:47.5960696Z   network time: Fri, 28 Feb 2020 14:09:47 GMT
2020-02-28T14:09:47.5967566Z == end clock drift check ==
2020-02-28T14:09:48.1733996Z 
2020-02-28T14:09:48.1807124Z ##[error]Bash exited with code '1'.
2020-02-28T14:09:48.1884444Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-02-28T14:09:48.1889511Z ==============================================================================
2020-02-28T14:09:48.1889917Z Task         : Get sources
2020-02-28T14:09:48.1890329Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
