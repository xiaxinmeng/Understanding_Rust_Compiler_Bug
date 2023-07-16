plain
2019-11-21T11:49:57.8679982Z diff of stderr:
2019-11-21T11:49:57.8680021Z 
2019-11-21T11:49:57.8680359Z 2   --> $DIR/region-lifetime-bounds-on-fns-where-clause.rs:20:43
2019-11-21T11:49:57.8680433Z 3    |
2019-11-21T11:49:57.8680510Z 4 LL |     let _: fn(&mut &isize, &mut &isize) = a;
2019-11-21T11:49:57.8680849Z -    |                                           ^ expected concrete lifetime, found bound lifetime parameter
2019-11-21T11:49:57.8681185Z +    |            ----------------------------   ^ expected concrete lifetime, found bound lifetime parameter
2019-11-21T11:49:57.8681346Z +    |            expected due to this
2019-11-21T11:49:57.8681422Z 6    |
2019-11-21T11:49:57.8681422Z 6    |
2019-11-21T11:49:57.8681732Z 7    = note: expected type `for<'r, 's, 't0, 't1> fn(&'r mut &'s isize, &'t0 mut &'t1 isize)`
2019-11-21T11:49:57.8682049Z 8               found type `for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>}`
2019-11-21T11:49:57.8682162Z 
2019-11-21T11:49:57.8682225Z The actual stderr differed from the expected stderr.
2019-11-21T11:49:57.8682225Z The actual stderr differed from the expected stderr.
2019-11-21T11:49:57.8682649Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.nll/region-lifetime-bounds-on-fns-where-clause.nll.stderr
2019-11-21T11:49:57.8682975Z To update references, rerun the tests and pass the `--bless` flag
2019-11-21T11:49:57.8683302Z To only update this specific test, also pass `--test-args regions/region-lifetime-bounds-on-fns-where-clause.rs`
2019-11-21T11:49:57.8683438Z error: 1 errors occurred comparing output.
2019-11-21T11:49:57.8683515Z status: exit code: 1
2019-11-21T11:49:57.8683515Z status: exit code: 1
2019-11-21T11:49:57.8684471Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.nll/auxiliary" "-A" "unused"
2019-11-21T11:49:57.8685144Z ------------------------------------------
2019-11-21T11:49:57.8685191Z 
2019-11-21T11:49:57.8685441Z ------------------------------------------
2019-11-21T11:49:57.8685506Z stderr:
2019-11-21T11:49:57.8685506Z stderr:
2019-11-21T11:49:57.8685746Z ------------------------------------------
2019-11-21T11:49:57.8685812Z error[E0308]: mismatched types
2019-11-21T11:49:57.8686116Z   --> /checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs:20:43
2019-11-21T11:49:57.8686210Z    |
2019-11-21T11:49:57.8686288Z LL |     let _: fn(&mut &isize, &mut &isize) = a; //~ ERROR mismatched types
2019-11-21T11:49:57.8686629Z    |            ----------------------------   ^ expected concrete lifetime, found bound lifetime parameter
2019-11-21T11:49:57.8686791Z    |            expected due to this
2019-11-21T11:49:57.8686866Z    |
2019-11-21T11:49:57.8686866Z    |
2019-11-21T11:49:57.8687155Z    = note: expected type `for<'r, 's, 't0, 't1> fn(&'r mut &'s isize, &'t0 mut &'t1 isize)`
2019-11-21T11:49:57.8687470Z               found type `for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>}`
2019-11-21T11:49:57.8687595Z error: aborting due to previous error
2019-11-21T11:49:57.8687635Z 
2019-11-21T11:49:57.8687893Z For more information about this error, try `rustc --explain E0308`.
2019-11-21T11:49:57.8687958Z 
---
2019-11-21T11:49:57.8688750Z diff of stderr:
2019-11-21T11:49:57.8688786Z 
2019-11-21T11:49:57.8689092Z 2   --> $DIR/region-multiple-lifetime-bounds-on-fns-where-clause.rs:22:56
2019-11-21T11:49:57.8689166Z 3    |
2019-11-21T11:49:57.8689244Z 4 LL |     let _: fn(&mut &isize, &mut &isize, &mut &isize) = a;
2019-11-21T11:49:57.8689596Z -    |                                                        ^ expected concrete lifetime, found bound lifetime parameter
2019-11-21T11:49:57.8689944Z +    |            -----------------------------------------   ^ expected concrete lifetime, found bound lifetime parameter
2019-11-21T11:49:57.8690110Z +    |            expected due to this
2019-11-21T11:49:57.8690185Z 6    |
2019-11-21T11:49:57.8690185Z 6    |
2019-11-21T11:49:57.8690521Z 7    = note: expected type `for<'r, 's, 't0, 't1, 't2, 't3> fn(&'r mut &'s isize, &'t0 mut &'t1 isize, &'t2 mut &'t3 isize)`
2019-11-21T11:49:57.8690872Z 8               found type `for<'r, 's, 't0> fn(&'r mut &isize, &'s mut &isize, &'t0 mut &isize) {a::<'_, '_, '_>}`
2019-11-21T11:49:57.8690986Z 
2019-11-21T11:49:57.8691047Z The actual stderr differed from the expected stderr.
2019-11-21T11:49:57.8691047Z The actual stderr differed from the expected stderr.
2019-11-21T11:49:57.8691482Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.nll/region-multiple-lifetime-bounds-on-fns-where-clause.nll.stderr
2019-11-21T11:49:57.8691808Z To update references, rerun the tests and pass the `--bless` flag
2019-11-21T11:49:57.8692142Z To only update this specific test, also pass `--test-args regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs`
2019-11-21T11:49:57.8692284Z error: 1 errors occurred comparing output.
2019-11-21T11:49:57.8692365Z status: exit code: 1
2019-11-21T11:49:57.8692365Z status: exit code: 1
2019-11-21T11:49:57.8693336Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.nll/auxiliary" "-A" "unused"
2019-11-21T11:49:57.8693957Z ------------------------------------------
2019-11-21T11:49:57.8694002Z 
2019-11-21T11:49:57.8694248Z ------------------------------------------
2019-11-21T11:49:57.8694326Z stderr:
2019-11-21T11:49:57.8694326Z stderr:
2019-11-21T11:49:57.8694553Z ------------------------------------------
2019-11-21T11:49:57.8694635Z error[E0308]: mismatched types
2019-11-21T11:49:57.8694939Z   --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:22:56
2019-11-21T11:49:57.8695041Z    |
2019-11-21T11:49:57.8695111Z LL |     let _: fn(&mut &isize, &mut &isize, &mut &isize) = a; //~ ERROR E0308
2019-11-21T11:49:57.8695463Z    |            -----------------------------------------   ^ expected concrete lifetime, found bound lifetime parameter
2019-11-21T11:49:57.8695626Z    |            expected due to this
2019-11-21T11:49:57.8695702Z    |
2019-11-21T11:49:57.8695702Z    |
2019-11-21T11:49:57.8696024Z    = note: expected type `for<'r, 's, 't0, 't1, 't2, 't3> fn(&'r mut &'s isize, &'t0 mut &'t1 isize, &'t2 mut &'t3 isize)`
2019-11-21T11:49:57.8696379Z               found type `for<'r, 's, 't0> fn(&'r mut &isize, &'s mut &isize, &'t0 mut &isize) {a::<'_, '_, '_>}`
2019-11-21T11:49:57.8696515Z error: aborting due to previous error
2019-11-21T11:49:57.8696554Z 
2019-11-21T11:49:57.8696829Z For more information about this error, try `rustc --explain E0308`.
2019-11-21T11:49:57.8696953Z 
---
2019-11-21T11:49:57.8697661Z diff of stderr:
2019-11-21T11:49:57.8697711Z 
2019-11-21T11:49:57.8697956Z 2   --> $DIR/regions-lifetime-bounds-on-fns.rs:20:43
2019-11-21T11:49:57.8698039Z 3    |
2019-11-21T11:49:57.8698102Z 4 LL |     let _: fn(&mut &isize, &mut &isize) = a;
2019-11-21T11:49:57.8698430Z -    |                                           ^ expected concrete lifetime, found bound lifetime parameter
2019-11-21T11:49:57.8698772Z +    |            ----------------------------   ^ expected concrete lifetime, found bound lifetime parameter
2019-11-21T11:49:57.8699050Z +    |            expected due to this
2019-11-21T11:49:57.8699117Z 6    |
2019-11-21T11:49:57.8699117Z 6    |
2019-11-21T11:49:57.8699450Z 7    = note: expected type `for<'r, 's, 't0, 't1> fn(&'r mut &'s isize, &'t0 mut &'t1 isize)`
2019-11-21T11:49:57.8699786Z 8               found type `for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>}`
2019-11-21T11:49:57.8699894Z 
2019-11-21T11:49:57.8699974Z The actual stderr differed from the expected stderr.
2019-11-21T11:49:57.8699974Z The actual stderr differed from the expected stderr.
2019-11-21T11:49:57.8700369Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-lifetime-bounds-on-fns.nll/regions-lifetime-bounds-on-fns.nll.stderr
2019-11-21T11:49:57.8700704Z To update references, rerun the tests and pass the `--bless` flag
2019-11-21T11:49:57.8701037Z To only update this specific test, also pass `--test-args regions/regions-lifetime-bounds-on-fns.rs`
2019-11-21T11:49:57.8701182Z error: 1 errors occurred comparing output.
2019-11-21T11:49:57.8701265Z status: exit code: 1
2019-11-21T11:49:57.8701265Z status: exit code: 1
2019-11-21T11:49:57.8702647Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-lifetime-bounds-on-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-lifetime-bounds-on-fns.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-lifetime-bounds-on-fns.nll/auxiliary" "-A" "unused"
2019-11-21T11:49:57.8703317Z ------------------------------------------
2019-11-21T11:49:57.8703381Z 
2019-11-21T11:49:57.8703618Z ------------------------------------------
2019-11-21T11:49:57.8703700Z stderr:
2019-11-21T11:49:57.8703700Z stderr:
2019-11-21T11:49:57.8703926Z ------------------------------------------
2019-11-21T11:49:57.8704008Z error[E0308]: mismatched types
2019-11-21T11:49:57.8704281Z   --> /checkout/src/test/ui/regions/regions-lifetime-bounds-on-fns.rs:20:43
2019-11-21T11:49:57.8704382Z    |
2019-11-21T11:49:57.8704463Z LL |     let _: fn(&mut &isize, &mut &isize) = a; //~ ERROR E0308
2019-11-21T11:49:57.8704792Z    |            ----------------------------   ^ expected concrete lifetime, found bound lifetime parameter
2019-11-21T11:49:57.8704954Z    |            expected due to this
2019-11-21T11:49:57.8705029Z    |
2019-11-21T11:49:57.8705029Z    |
2019-11-21T11:49:57.8705317Z    = note: expected type `for<'r, 's, 't0, 't1> fn(&'r mut &'s isize, &'t0 mut &'t1 isize)`
2019-11-21T11:49:57.8705633Z               found type `for<'r, 's> fn(&'r mut &isize, &'s mut &isize) {a::<'_, '_>}`
2019-11-21T11:49:57.8705758Z error: aborting due to previous error
2019-11-21T11:49:57.8705798Z 
2019-11-21T11:49:57.8706072Z For more information about this error, try `rustc --explain E0308`.
2019-11-21T11:49:57.8706123Z 
---
2019-11-21T11:49:57.8711932Z +    |            |
2019-11-21T11:49:57.8712062Z +    |            expected due to this
2019-11-21T11:49:57.8712874Z 12    |
2019-11-21T11:49:57.8712980Z 13    = note: expected type `i32`
2019-11-21T11:49:57.8713220Z 14               found type `WrongGeneric::<&{integer}>`
2019-11-21T11:49:57.8714104Z 
2019-11-21T11:49:57.8714176Z The actual stderr differed from the expected stderr.
2019-11-21T11:49:57.8714176Z The actual stderr differed from the expected stderr.
2019-11-21T11:49:57.8714914Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll/generic_type_does_not_live_long_enough.nll.stderr
2019-11-21T11:49:57.8717178Z To update references, rerun the tests and pass the `--bless` flag
2019-11-21T11:49:57.8722366Z To only update this specific test, also pass `--test-args type-alias-impl-trait/generic_type_does_not_live_long_enough.rs`
2019-11-21T11:49:57.8722541Z error: 1 errors occurred comparing output.
2019-11-21T11:49:57.8722626Z status: exit code: 1
2019-11-21T11:49:57.8722626Z status: exit code: 1
2019-11-21T11:49:57.8723990Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.nll/auxiliary" "-A" "unused"
2019-11-21T11:49:57.8724824Z ------------------------------------------
2019-11-21T11:49:57.8724878Z 
2019-11-21T11:49:57.8725165Z ------------------------------------------
2019-11-21T11:49:57.8725255Z stderr:
2019-11-21T11:49:57.8725255Z stderr:
2019-11-21T11:49:57.8725513Z ------------------------------------------
2019-11-21T11:49:57.8725608Z error: at least one trait must be specified
2019-11-21T11:49:57.8725949Z   --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:9:24
2019-11-21T11:49:57.8726059Z    |
2019-11-21T11:49:57.8726325Z LL | type WrongGeneric<T> = impl 'static;
2019-11-21T11:49:57.8726463Z 
2019-11-21T11:49:57.8726550Z error[E0308]: mismatched types
2019-11-21T11:49:57.8727022Z   --> /checkout/src/test/ui/type-alias-impl-trait/generic_type_does_not_live_long_enough.rs:6:18
2019-11-21T11:49:57.8727125Z    |
2019-11-21T11:49:57.8727125Z    |
2019-11-21T11:49:57.8727318Z LL |     let z: i32 = x; //~ ERROR mismatched types
2019-11-21T11:49:57.8727609Z    |            ---   ^ expected i32, found opaque type
2019-11-21T11:49:57.8727761Z    |            expected due to this
2019-11-21T11:49:57.8727840Z    |
2019-11-21T11:49:57.8727897Z    = note: expected type `i32`
2019-11-21T11:49:57.8727897Z    = note: expected type `i32`
2019-11-21T11:49:57.8727987Z               found type `WrongGeneric::<&{integer}>`
2019-11-21T11:49:57.8728106Z error: aborting due to 2 previous errors
2019-11-21T11:49:57.8728149Z 
2019-11-21T11:49:57.8728442Z For more information about this error, try `rustc --explain E0308`.
2019-11-21T11:49:57.8728497Z 
---
2019-11-21T11:49:57.8731082Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-11-21T11:49:57.8731195Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-11-21T11:49:57.8739733Z 
2019-11-21T11:49:57.8740876Z 
2019-11-21T11:49:57.8746010Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.41.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-11-21T11:49:57.8746877Z 
2019-11-21T11:49:57.8746915Z 
2019-11-21T11:49:57.8747611Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-11-21T11:49:57.8748583Z Build completed unsuccessfully in 1:37:07
2019-11-21T11:49:57.8748583Z Build completed unsuccessfully in 1:37:07
2019-11-21T11:49:57.8796882Z == clock drift check ==
2019-11-21T11:49:57.8819102Z   local time: Thu Nov 21 11:49:57 UTC 2019
2019-11-21T11:49:58.1528534Z   network time: Thu, 21 Nov 2019 11:49:58 GMT
2019-11-21T11:49:58.1528661Z == end clock drift check ==
2019-11-21T11:49:59.5192442Z 
2019-11-21T11:49:59.5292139Z ##[error]Bash exited with code '1'.
2019-11-21T11:49:59.5328156Z ##[section]Starting: Checkout
2019-11-21T11:49:59.5330342Z ==============================================================================
2019-11-21T11:49:59.5330449Z Task         : Get sources
2019-11-21T11:49:59.5330524Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
