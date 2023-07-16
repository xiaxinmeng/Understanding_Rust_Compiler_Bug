plain
2019-12-22T18:40:18.3864612Z ---- [ui (nll)] ui/hrtb/hrtb-perfect-forwarding.rs stdout ----
2019-12-22T18:40:18.3864704Z diff of stderr:
2019-12-22T18:40:18.3864746Z 
2019-12-22T18:40:18.3864822Z 44    |
2019-12-22T18:40:18.3864897Z 45    = help: a `loop` may express intention better if this is on purpose
2019-12-22T18:40:18.3865240Z - error: higher-ranked subtype error
2019-12-22T18:40:18.3865542Z -   --> $DIR/hrtb-perfect-forwarding.rs:46:5
2019-12-22T18:40:18.3865770Z -    |
2019-12-22T18:40:18.3865770Z -    |
2019-12-22T18:40:18.3865996Z - LL |     foo_hrtb_bar_not(&mut t);
2019-12-22T18:40:18.3866445Z - 
2019-12-22T18:40:18.3866527Z 53 error: lifetime may not live long enough
2019-12-22T18:40:18.3866793Z 54   --> $DIR/hrtb-perfect-forwarding.rs:46:5
2019-12-22T18:40:18.3866880Z 55    |
---
2019-12-22T18:40:18.3867635Z + 
2019-12-22T18:40:18.3867874Z + error: higher-ranked subtype error
2019-12-22T18:40:18.3868119Z +   --> $DIR/hrtb-perfect-forwarding.rs:46:5
2019-12-22T18:40:18.3868204Z +    |
2019-12-22T18:40:18.3868262Z + LL |     foo_hrtb_bar_not(&mut t);
2019-12-22T18:40:18.3868421Z 63 
2019-12-22T18:40:18.3868504Z 64 warning: function cannot return without recursing
2019-12-22T18:40:18.3868776Z 65   --> $DIR/hrtb-perfect-forwarding.rs:49:1
2019-12-22T18:40:18.3868825Z 
2019-12-22T18:40:18.3868825Z 
2019-12-22T18:40:18.3868860Z 
2019-12-22T18:40:18.3868943Z The actual stderr differed from the expected stderr.
2019-12-22T18:40:18.3869331Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-perfect-forwarding.nll/hrtb-perfect-forwarding.nll.stderr
2019-12-22T18:40:18.3869656Z To update references, rerun the tests and pass the `--bless` flag
2019-12-22T18:40:18.3869967Z To only update this specific test, also pass `--test-args hrtb/hrtb-perfect-forwarding.rs`
2019-12-22T18:40:18.3870107Z error: 1 errors occurred comparing output.
2019-12-22T18:40:18.3870193Z status: exit code: 1
2019-12-22T18:40:18.3870193Z status: exit code: 1
2019-12-22T18:40:18.3871780Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-perfect-forwarding.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hrtb/hrtb-perfect-forwarding.nll/auxiliary" "-A" "unused"
2019-12-22T18:40:18.3872444Z ------------------------------------------
2019-12-22T18:40:18.3872497Z 
2019-12-22T18:40:18.3872751Z ------------------------------------------
2019-12-22T18:40:18.3872835Z stderr:
2019-12-22T18:40:18.3872835Z stderr:
2019-12-22T18:40:18.3873066Z ------------------------------------------
2019-12-22T18:40:18.3873160Z warning: function cannot return without recursing
2019-12-22T18:40:18.3873433Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:22:1
2019-12-22T18:40:18.3873537Z    |
2019-12-22T18:40:18.3873769Z LL | / fn no_hrtb<'b,T>(mut t: T)
2019-12-22T18:40:18.3874016Z LL | |     where T : Bar<&'b isize>
2019-12-22T18:40:18.3874086Z LL | | {
2019-12-22T18:40:18.3874379Z LL | |     // OK -- `T : Bar<&'b isize>`, and thus the impl above ensures that
2019-12-22T18:40:18.3874773Z LL | |     // `&mut T : Bar<&'b isize>`.
2019-12-22T18:40:18.3874861Z LL | |     no_hrtb(&mut t);
2019-12-22T18:40:18.3875119Z    | |     --------------- recursive call site
2019-12-22T18:40:18.3875192Z LL | | }
2019-12-22T18:40:18.3875269Z    | |_^ cannot return without recursing
2019-12-22T18:40:18.3875419Z    = note: `#[warn(unconditional_recursion)]` on by default
2019-12-22T18:40:18.3875419Z    = note: `#[warn(unconditional_recursion)]` on by default
2019-12-22T18:40:18.3875511Z    = help: a `loop` may express intention better if this is on purpose
2019-12-22T18:40:18.3875647Z warning: function cannot return without recursing
2019-12-22T18:40:18.3875948Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:30:1
2019-12-22T18:40:18.3876026Z    |
2019-12-22T18:40:18.3876026Z    |
2019-12-22T18:40:18.3876102Z LL | / fn bar_hrtb<T>(mut t: T)
2019-12-22T18:40:18.3876343Z LL | |     where T : for<'b> Bar<&'b isize>
2019-12-22T18:40:18.3876430Z LL | | {
2019-12-22T18:40:18.3876696Z LL | |     // OK -- `T : for<'b> Bar<&'b isize>`, and thus the impl above
2019-12-22T18:40:18.3876798Z ...  |
2019-12-22T18:40:18.3876858Z LL | |     bar_hrtb(&mut t);
2019-12-22T18:40:18.3877121Z    | |     ---------------- recursive call site
2019-12-22T18:40:18.3877193Z LL | | }
2019-12-22T18:40:18.3877269Z    | |_^ cannot return without recursing
2019-12-22T18:40:18.3877350Z    |
2019-12-22T18:40:18.3877422Z    = help: a `loop` may express intention better if this is on purpose
2019-12-22T18:40:18.3877557Z warning: function cannot return without recursing
2019-12-22T18:40:18.3877831Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:39:1
2019-12-22T18:40:18.3877921Z    |
2019-12-22T18:40:18.3877921Z    |
2019-12-22T18:40:18.3878171Z LL | / fn foo_hrtb_bar_not<'b,T>(mut t: T)
2019-12-22T18:40:18.3878441Z LL | |     where T : for<'a> Foo<&'a isize> + Bar<&'b isize>
2019-12-22T18:40:18.3878531Z LL | | {
2019-12-22T18:40:18.3878804Z LL | |     // Not OK -- The forwarding impl for `Foo` requires that `Bar` also
2019-12-22T18:40:18.3878909Z ...  |
2019-12-22T18:40:18.3878978Z LL | |     foo_hrtb_bar_not(&mut t); //~ ERROR mismatched types
2019-12-22T18:40:18.3879259Z    | |     ------------------------ recursive call site
2019-12-22T18:40:18.3879332Z LL | | }
2019-12-22T18:40:18.3879408Z    | |_^ cannot return without recursing
2019-12-22T18:40:18.3879474Z    |
2019-12-22T18:40:18.3879562Z    = help: a `loop` may express intention better if this is on purpose
2019-12-22T18:40:18.3879692Z error: lifetime may not live long enough
2019-12-22T18:40:18.3879960Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:46:5
2019-12-22T18:40:18.3880050Z    |
2019-12-22T18:40:18.3880050Z    |
2019-12-22T18:40:18.3880276Z LL | fn foo_hrtb_bar_not<'b,T>(mut t: T)
2019-12-22T18:40:18.3880724Z ...
2019-12-22T18:40:18.3880724Z ...
2019-12-22T18:40:18.3880794Z LL |     foo_hrtb_bar_not(&mut t); //~ ERROR mismatched types
2019-12-22T18:40:18.3881118Z    |     ^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'b` must outlive `'static`
2019-12-22T18:40:18.3881464Z    = help: consider replacing `'b` with `'static`
2019-12-22T18:40:18.3881514Z 
2019-12-22T18:40:18.3881747Z error: higher-ranked subtype error
2019-12-22T18:40:18.3882014Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:46:5
2019-12-22T18:40:18.3882014Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:46:5
2019-12-22T18:40:18.3882104Z    |
2019-12-22T18:40:18.3882172Z LL |     foo_hrtb_bar_not(&mut t); //~ ERROR mismatched types
2019-12-22T18:40:18.3882307Z 
2019-12-22T18:40:18.3882388Z warning: function cannot return without recursing
2019-12-22T18:40:18.3882661Z   --> /checkout/src/test/ui/hrtb/hrtb-perfect-forwarding.rs:49:1
2019-12-22T18:40:18.3882760Z    |
2019-12-22T18:40:18.3882760Z    |
2019-12-22T18:40:18.3882822Z LL | / fn foo_hrtb_bar_hrtb<T>(mut t: T)
2019-12-22T18:40:18.3883109Z LL | |     where T : for<'a> Foo<&'a isize> + for<'b> Bar<&'b isize>
2019-12-22T18:40:18.3883188Z LL | | {
2019-12-22T18:40:18.3883511Z LL | |     // OK -- now we have `T : for<'b> Bar&'b isize>`.
2019-12-22T18:40:18.3883680Z LL | |     foo_hrtb_bar_hrtb(&mut t);
2019-12-22T18:40:18.3883976Z    | |     ------------------------- recursive call site
2019-12-22T18:40:18.3884050Z LL | | }
2019-12-22T18:40:18.3884126Z    | |_^ cannot return without recursing
2019-12-22T18:40:18.3884192Z    |
2019-12-22T18:40:18.3884278Z    = help: a `loop` may express intention better if this is on purpose
2019-12-22T18:40:18.3884409Z error: aborting due to 2 previous errors
2019-12-22T18:40:18.3884454Z 
2019-12-22T18:40:18.3884489Z 
2019-12-22T18:40:18.3884738Z ------------------------------------------
---
2019-12-22T18:40:18.3894936Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:537:22
2019-12-22T18:40:18.3895057Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-12-22T18:40:18.3914631Z 
2019-12-22T18:40:19.3359087Z 
2019-12-22T18:40:19.3364113Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.42.0-nightly\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-12-22T18:40:19.3365679Z 
2019-12-22T18:40:19.3365837Z 
2019-12-22T18:40:19.3366146Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-12-22T18:40:19.3366739Z Build completed unsuccessfully in 1:41:53
2019-12-22T18:40:19.3366739Z Build completed unsuccessfully in 1:41:53
2019-12-22T18:40:19.3367000Z == clock drift check ==
2019-12-22T18:40:19.3367206Z   local time: Sun Dec 22 18:40:18 UTC 2019
2019-12-22T18:40:19.3367395Z   network time: Sun, 22 Dec 2019 18:40:18 GMT
2019-12-22T18:40:19.3367597Z == end clock drift check ==
2019-12-22T18:40:19.4682323Z 
2019-12-22T18:40:19.4818236Z ##[error]Bash exited with code '1'.
2019-12-22T18:40:19.4868995Z ##[section]Starting: Checkout
2019-12-22T18:40:19.4870941Z ==============================================================================
2019-12-22T18:40:19.4871032Z Task         : Get sources
2019-12-22T18:40:19.4871432Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
