plain
........................................................................................ 13288/13303
...............
failures:

---- [ui] src/test/ui/lint/let_underscore/let_underscore_lock.rs stdout ----
error: ui test compiled successfully!
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/let_underscore/let_underscore_lock.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/let_underscore/let_underscore_lock" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/let_underscore/let_underscore_lock/auxiliary"
stdout: none
stderr: none


---- [ui] src/test/ui/lint/let_underscore/let_underscore_must_use.rs stdout ----


- warning: non-binding let on a expression marked `must_use`
-   --> $DIR/let_underscore_must_use.rs:11:5
-    |
- LL |     let _ = MustUseType;
-    |
-    |
-    = note: requested on the command line with `-W let-underscore-must-use`
- help: consider binding to an unused variable
-    |
- LL |     let _unused = MustUseType;
- help: consider explicitly droping with `std::mem::drop`
-    |
-    |
- LL |     let _ = drop(...);
- 
- 
- warning: non-binding let on a expression marked `must_use`
-   --> $DIR/let_underscore_must_use.rs:12:5
- LL |     let _ = must_use_function();
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |
- help: consider binding to an unused variable
- help: consider binding to an unused variable
-    |
- LL |     let _unused = must_use_function();
- help: consider explicitly droping with `std::mem::drop`
-    |
-    |
- LL |     let _ = drop(...);
- 
- warning: 2 warnings emitted
- 
- 
---
To only update this specific test, also pass `--test-args lint/let_underscore/let_underscore_must_use.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/let_underscore/let_underscore_must_use.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/let_underscore/let_underscore_must_use/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-W" "let_underscore_must_use" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/let_underscore/let_underscore_must_use/auxiliary"
stdout: none
stderr: none


failures:
    [ui] src/test/ui/lint/let_underscore/let_underscore_lock.rs
