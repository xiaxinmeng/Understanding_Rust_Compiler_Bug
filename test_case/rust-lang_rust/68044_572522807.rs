plain
2020-01-09T09:53:26.1529813Z 1 error[E0716]: temporary value dropped while borrowed
2020-01-09T09:53:26.1530337Z -   --> $DIR/auto-trait-regions.rs:44:24
2020-01-09T09:53:26.1530751Z +   --> $DIR/auto-trait-regions.rs:45:24
2020-01-09T09:53:26.1530933Z 3    |
2020-01-09T09:53:26.1531060Z 4 LL |         let a = A(&mut true, &mut true, No);
2020-01-09T09:53:26.1531436Z 5    |                        ^^^^                - temporary value is freed at the end of this statement
2020-01-09T09:53:26.1531756Z 12    = note: consider using a `let` binding to create a longer lived value
2020-01-09T09:53:26.1531892Z 13 
2020-01-09T09:53:26.1532034Z 14 error[E0716]: temporary value dropped while borrowed
2020-01-09T09:53:26.1532513Z -   --> $DIR/auto-trait-regions.rs:44:35
2020-01-09T09:53:26.1532513Z -   --> $DIR/auto-trait-regions.rs:44:35
2020-01-09T09:53:26.1533216Z +   --> $DIR/auto-trait-regions.rs:45:35
2020-01-09T09:53:26.1533438Z 16    |
2020-01-09T09:53:26.1533568Z 17 LL |         let a = A(&mut true, &mut true, No);
2020-01-09T09:53:26.1533946Z 18    |                                   ^^^^     - temporary value is freed at the end of this statement
2020-01-09T09:53:26.1534253Z 31    |     ^^^^^^^^^^^^^^^
2020-01-09T09:53:26.1534375Z 32 
2020-01-09T09:53:26.1534689Z 33 error: higher-ranked subtype error
2020-01-09T09:53:26.1535025Z -   --> $DIR/auto-trait-regions.rs:48:5
2020-01-09T09:53:26.1535025Z -   --> $DIR/auto-trait-regions.rs:48:5
2020-01-09T09:53:26.1535342Z +   --> $DIR/auto-trait-regions.rs:49:5
2020-01-09T09:53:26.1535518Z 35    |
2020-01-09T09:53:26.1535638Z 36 LL |     assert_foo(gen);
2020-01-09T09:53:26.1535881Z 
2020-01-09T09:53:26.1535992Z 
2020-01-09T09:53:26.1536142Z The actual stderr differed from the expected stderr.
2020-01-09T09:53:26.1536142Z The actual stderr differed from the expected stderr.
2020-01-09T09:53:26.1536568Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions.nll/auto-trait-regions.nll.stderr
2020-01-09T09:53:26.1536957Z To update references, rerun the tests and pass the `--bless` flag
2020-01-09T09:53:26.1537328Z To only update this specific test, also pass `--test-args generator/auto-trait-regions.rs`
2020-01-09T09:53:26.1537622Z error: 1 errors occurred comparing output.
2020-01-09T09:53:26.1537766Z status: exit code: 1
2020-01-09T09:53:26.1537766Z status: exit code: 1
2020-01-09T09:53:26.1538653Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/auto-trait-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/auto-trait-regions.nll/auxiliary" "-A" "unused"
2020-01-09T09:53:26.1539293Z ------------------------------------------
2020-01-09T09:53:26.1539429Z 
2020-01-09T09:53:26.1539741Z ------------------------------------------
2020-01-09T09:53:26.1539914Z stderr:
2020-01-09T09:53:26.1539914Z stderr:
2020-01-09T09:53:26.1540207Z ------------------------------------------
2020-01-09T09:53:26.1540391Z error[E0716]: temporary value dropped while borrowed
2020-01-09T09:53:26.1540712Z   --> /checkout/src/test/ui/generator/auto-trait-regions.rs:45:24
2020-01-09T09:53:26.1540901Z    |
2020-01-09T09:53:26.1541049Z LL |         let a = A(&mut true, &mut true, No);
2020-01-09T09:53:26.1541401Z    |                        ^^^^                - temporary value is freed at the end of this statement
2020-01-09T09:53:26.1541761Z    |                        creates a temporary which is freed while still in use
2020-01-09T09:53:26.1541895Z LL |         yield;
2020-01-09T09:53:26.1542031Z LL |         assert_foo(a);
2020-01-09T09:53:26.1542333Z    |                    - borrow later used here
2020-01-09T09:53:26.1542333Z    |                    - borrow later used here
2020-01-09T09:53:26.1542507Z    |
2020-01-09T09:53:26.1542653Z    = note: consider using a `let` binding to create a longer lived value
2020-01-09T09:53:26.1542764Z 
2020-01-09T09:53:26.1542989Z error[E0716]: temporary value dropped while borrowed
2020-01-09T09:53:26.1543391Z   --> /checkout/src/test/ui/generator/auto-trait-regions.rs:45:35
2020-01-09T09:53:26.1543576Z    |
2020-01-09T09:53:26.1543699Z LL |         let a = A(&mut true, &mut true, No);
2020-01-09T09:53:26.1544062Z    |                                   ^^^^     - temporary value is freed at the end of this statement
2020-01-09T09:53:26.1544397Z    |                                   creates a temporary which is freed while still in use
2020-01-09T09:53:26.1544666Z LL |         yield;
2020-01-09T09:53:26.1544789Z LL |         assert_foo(a);
2020-01-09T09:53:26.1545122Z    |                    - borrow later used here
2020-01-09T09:53:26.1545122Z    |                    - borrow later used here
2020-01-09T09:53:26.1545305Z    |
2020-01-09T09:53:26.1545451Z    = note: consider using a `let` binding to create a longer lived value
2020-01-09T09:53:26.1545560Z 
2020-01-09T09:53:26.1545859Z error: higher-ranked subtype error
2020-01-09T09:53:26.1546221Z   --> /checkout/src/test/ui/generator/auto-trait-regions.rs:30:5
2020-01-09T09:53:26.1546382Z    |
2020-01-09T09:53:26.1546517Z LL |     assert_foo(gen);
2020-01-09T09:53:26.1546933Z 
2020-01-09T09:53:26.1547231Z error: higher-ranked subtype error
2020-01-09T09:53:26.1547785Z   --> /checkout/src/test/ui/generator/auto-trait-regions.rs:49:5
2020-01-09T09:53:26.1547958Z    |
2020-01-09T09:53:26.1547958Z    |
2020-01-09T09:53:26.1548337Z LL |     assert_foo(gen);
2020-01-09T09:53:26.1548611Z 
2020-01-09T09:53:26.1549024Z error: aborting due to 4 previous errors
2020-01-09T09:53:26.1549143Z 
2020-01-09T09:53:26.1549688Z For more information about this error, try `rustc --explain E0716`.
---
2020-01-09T09:53:26.1553673Z ---- [ui (nll)] ui/hrtb/hrtb-perfect-forwarding.rs stdout ----
2020-01-09T09:53:26.1553748Z diff of stderr:
2020-01-09T09:53:26.1553804Z 
2020-01-09T09:53:26.1553858Z 39 ...  |
2020-01-09T09:53:26.1553931Z 40 LL | |     foo_hrtb_bar_not(&mut t);
2020-01-09T09:53:26.1554382Z 41    | |     ------------------------ recursive call site
2020-01-09T09:53:26.1554473Z + LL | |
2020-01-09T09:53:26.1554530Z 42 LL | | }
2020-01-09T09:53:26.1554792Z 43    | |_^ cannot return without recursing
2020-01-09T09:53:26.1554905Z 
2020-01-09T09:53:26.1554960Z 62    |     ^^^^^^^^^^^^^^^^^^^^^^^^
2020-01-09T09:53:26.1555046Z 63 
2020-01-09T09:53:26.1555109Z 64 warning: function cannot return without recursing
2020-01-09T09:53:26.1555109Z 64 warning: function cannot return without recursing
2020-01-09T09:53:26.1555546Z -   --> $DIR/hrtb-perfect-forwarding.rs:49:1
2020-01-09T09:53:26.1556007Z +   --> $DIR/hrtb-perfect-forwarding.rs:50:1
2020-01-09T09:53:26.1556094Z 66    |
2020-01-09T09:53:26.1556465Z 67 LL | / fn foo_hrtb_bar_hrtb<T>(mut t: T)
2020-01-09T09:53:26.1556873Z 68 LL | |     where T : for<'a> Foo<&'a isize> + for<'b> Bar<&'b isize>
2020-01-09T09:53:26.1556985Z 
2020-01-09T09:53:26.1557054Z The actual stderr differed from the expected stderr.
2020-01-09T09:53:26.1557054Z The actual stderr differed from the expected stderr.
2020-01-09T09:53:26.1557465Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-perfect-forwarding.nll/hrtb-perfect-forwarding.nll.stderr
2020-01-09T09:53:26.1557781Z To update references, rerun the tests and pass the `--bless` flag
2020-01-09T09:53:26.1558123Z To only update this specific test, also pass `--test-args hrtb/hrtb-perfect-forwarding.rs`
2020-01-09T09:53:26.1558278Z error: 1 errors occurred comparing output.
2020-01-09T09:53:26.1558349Z status: exit code: 1
2020-01-09T09:53:26.1558349Z status: exit code: 1
2020-01-09T09:53:26.1559525Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-perfect-forwarding.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-perfect-forwarding.nll/auxiliary" "-A" "unused"
2020-01-09T09:53:26.1560134Z ------------------------------------------
2020-01-09T09:53:26.1560185Z 
2020-01-09T09:53:26.1560448Z ------------------------------------------
2020-01-09T09:53:26.1560601Z stderr:
2020-01-09T09:53:26.1560601Z stderr:
2020-01-09T09:53:26.1560952Z ------------------------------------------
2020-01-09T09:53:26.1561035Z warning: function cannot return without recursing
2020-01-09T09:53:26.1561336Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:22:1
2020-01-09T09:53:26.1561420Z    |
2020-01-09T09:53:26.1561855Z LL | / fn no_hrtb<'b,T>(mut t: T)
2020-01-09T09:53:26.1562087Z LL | |     where T : Bar<&'b isize>
2020-01-09T09:53:26.1562183Z LL | | {
2020-01-09T09:53:26.1562654Z LL | |     // OK -- `T : Bar<&'b isize>`, and thus the impl above ensures that
2020-01-09T09:53:26.1562915Z LL | |     // `&mut T : Bar<&'b isize>`.
2020-01-09T09:53:26.1563002Z LL | |     no_hrtb(&mut t);
2020-01-09T09:53:26.1563239Z    | |     --------------- recursive call site
2020-01-09T09:53:26.1563329Z LL | | }
2020-01-09T09:53:26.1563388Z    | |_^ cannot return without recursing
2020-01-09T09:53:26.1563534Z    = note: `#[warn(unconditional_recursion)]` on by default
2020-01-09T09:53:26.1563534Z    = note: `#[warn(unconditional_recursion)]` on by default
2020-01-09T09:53:26.1563823Z    = help: a `loop` may express intention better if this is on purpose
2020-01-09T09:53:26.1563959Z warning: function cannot return without recursing
2020-01-09T09:53:26.1564241Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:30:1
2020-01-09T09:53:26.1564334Z    |
2020-01-09T09:53:26.1564334Z    |
2020-01-09T09:53:26.1564394Z LL | / fn bar_hrtb<T>(mut t: T)
2020-01-09T09:53:26.1565026Z LL | |     where T : for<'b> Bar<&'b isize>
2020-01-09T09:53:26.1565097Z LL | | {
2020-01-09T09:53:26.1565541Z LL | |     // OK -- `T : for<'b> Bar<&'b isize>`, and thus the impl above
2020-01-09T09:53:26.1565614Z ...  |
2020-01-09T09:53:26.1565688Z LL | |     bar_hrtb(&mut t);
2020-01-09T09:53:26.1565917Z    | |     ---------------- recursive call site
2020-01-09T09:53:26.1566001Z LL | | }
2020-01-09T09:53:26.1566076Z    | |_^ cannot return without recursing
2020-01-09T09:53:26.1566317Z    |
2020-01-09T09:53:26.1566401Z    = help: a `loop` may express intention better if this is on purpose
2020-01-09T09:53:26.1566522Z warning: function cannot return without recursing
2020-01-09T09:53:26.1567061Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:39:1
2020-01-09T09:53:26.1567369Z    |
2020-01-09T09:53:26.1567369Z    |
2020-01-09T09:53:26.1567596Z LL | / fn foo_hrtb_bar_not<'b,T>(mut t: T)
2020-01-09T09:53:26.1568049Z LL | |     where T : for<'a> Foo<&'a isize> + Bar<&'b isize>
2020-01-09T09:53:26.1568124Z LL | | {
2020-01-09T09:53:26.1568579Z LL | |     // Not OK -- The forwarding impl for `Foo` requires that `Bar` also
2020-01-09T09:53:26.1568659Z ...  |
2020-01-09T09:53:26.1568746Z LL | |     foo_hrtb_bar_not(&mut t); //~ ERROR mismatched types
2020-01-09T09:53:26.1569164Z    | |     ------------------------ recursive call site
2020-01-09T09:53:26.1569621Z LL | |                               //~| ERROR mismatched types
2020-01-09T09:53:26.1569691Z LL | | }
2020-01-09T09:53:26.1569764Z    | |_^ cannot return without recursing
2020-01-09T09:53:26.1569843Z    |
2020-01-09T09:53:26.1569916Z    = help: a `loop` may express intention better if this is on purpose
2020-01-09T09:53:26.1570216Z error: lifetime may not live long enough
2020-01-09T09:53:26.1570485Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:46:5
2020-01-09T09:53:26.1570576Z    |
2020-01-09T09:53:26.1570576Z    |
2020-01-09T09:53:26.1581701Z LL | fn foo_hrtb_bar_not<'b,T>(mut t: T)
2020-01-09T09:53:26.1582461Z ...
2020-01-09T09:53:26.1582461Z ...
2020-01-09T09:53:26.1582532Z LL |     foo_hrtb_bar_not(&mut t); //~ ERROR mismatched types
2020-01-09T09:53:26.1582833Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'static`
2020-01-09T09:53:26.1583122Z    = help: consider replacing `'b` with `'static`
2020-01-09T09:53:26.1583165Z 
2020-01-09T09:53:26.1583368Z error: higher-ranked subtype error
2020-01-09T09:53:26.1583596Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:46:5
2020-01-09T09:53:26.1583596Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:46:5
2020-01-09T09:53:26.1583756Z    |
2020-01-09T09:53:26.1583813Z LL |     foo_hrtb_bar_not(&mut t); //~ ERROR mismatched types
2020-01-09T09:53:26.1583929Z 
2020-01-09T09:53:26.1584002Z warning: function cannot return without recursing
2020-01-09T09:53:26.1584251Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:50:1
2020-01-09T09:53:26.1584330Z    |
2020-01-09T09:53:26.1584330Z    |
2020-01-09T09:53:26.1584381Z LL | / fn foo_hrtb_bar_hrtb<T>(mut t: T)
2020-01-09T09:53:26.1584634Z LL | |     where T : for<'a> Foo<&'a isize> + for<'b> Bar<&'b isize>
2020-01-09T09:53:26.1584699Z LL | | {
2020-01-09T09:53:26.1584930Z LL | |     // OK -- now we have `T : for<'b> Bar&'b isize>`.
2020-01-09T09:53:26.1584993Z LL | |     foo_hrtb_bar_hrtb(&mut t);
2020-01-09T09:53:26.1585222Z    | |     ------------------------- recursive call site
2020-01-09T09:53:26.1585299Z LL | | }
2020-01-09T09:53:26.1585350Z    | |_^ cannot return without recursing
2020-01-09T09:53:26.1585422Z    |
2020-01-09T09:53:26.1585487Z    = help: a `loop` may express intention better if this is on purpose
2020-01-09T09:53:26.1585603Z error: aborting due to 2 previous errors
2020-01-09T09:53:26.1585638Z 
2020-01-09T09:53:26.1585683Z 
2020-01-09T09:53:26.1585886Z ------------------------------------------
---
2020-01-09T09:53:26.9759183Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:386:22
2020-01-09T09:53:26.9759505Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2020-01-09T09:53:26.9759583Z 
2020-01-09T09:53:26.9759613Z 
2020-01-09T09:53:26.9761961Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2020-01-09T09:53:26.9783836Z 
2020-01-09T09:53:26.9783880Z 
2020-01-09T09:53:26.9783993Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2020-01-09T09:53:26.9784078Z Build completed unsuccessfully in 1:25:56
2020-01-09T09:53:26.9784078Z Build completed unsuccessfully in 1:25:56
2020-01-09T09:53:26.9784172Z == clock drift check ==
2020-01-09T09:53:26.9784261Z   local time: Thu Jan  9 09:53:26 UTC 2020
2020-01-09T09:53:26.9784499Z   network time: Thu, 09 Jan 2020 09:53:26 GMT
2020-01-09T09:53:26.9784698Z == end clock drift check ==
2020-01-09T09:53:27.1763965Z 
2020-01-09T09:53:27.1851448Z ##[error]Bash exited with code '1'.
2020-01-09T09:53:27.1895752Z ##[section]Starting: Checkout
2020-01-09T09:53:27.1897473Z ==============================================================================
2020-01-09T09:53:27.1897562Z Task         : Get sources
2020-01-09T09:53:27.1897626Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
