plain
---- [ui] src/test/ui/error-codes/E0516.rs stdout ----
diff of stderr:

3    |
4 LL |     let x: typeof(92) = 92;
5    |            ^^^^^^^^^^ reserved keyword
+    |
+ help: consider replacing `typeof(...)` with an actual type
+ LL |     let x: i32 = 92;
+    |            ~~~
6 
7 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args error-codes/E0516.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0516.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0516" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0516/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0516]: `typeof` is a reserved keyword but unimplemented
   |
   |
LL |     let x: typeof(92) = 92; //~ ERROR E0516
   |            ^^^^^^^^^^ reserved keyword
   |
help: consider replacing `typeof(...)` with an actual type
   |
LL |     let x: i32 = 92; //~ ERROR E0516

error: aborting due to previous error

For more information about this error, try `rustc --explain E0516`.
For more information about this error, try `rustc --explain E0516`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-29184.rs stdout ----
diff of stderr:

3    |
4 LL |     let x: typeof(92) = 92;
5    |            ^^^^^^^^^^ reserved keyword
+    |
+ help: consider replacing `typeof(...)` with an actual type
+ LL |     let x: i32 = 92;
+    |            ~~~
6 
7 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args issues/issue-29184.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-29184.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29184" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29184/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0516]: `typeof` is a reserved keyword but unimplemented
   |
   |
LL |     let x: typeof(92) = 92; //~ ERROR `typeof` is a reserved keyword
   |            ^^^^^^^^^^ reserved keyword
   |
help: consider replacing `typeof(...)` with an actual type
   |
LL |     let x: i32 = 92; //~ ERROR `typeof` is a reserved keyword

error: aborting due to previous error

For more information about this error, try `rustc --explain E0516`.
For more information about this error, try `rustc --explain E0516`.
------------------------------------------


---- [ui] src/test/ui/typeof/type_mismatch.rs stdout ----
diff of stderr:

3    |
4 LL |     let b: typeof(a) = 1i8;
5    |            ^^^^^^^^^ reserved keyword
+    |
+ help: consider replacing `typeof(...)` with an actual type
+    |
+ LL |     let b: u8 = 1i8;
6 
7 error[E0308]: mismatched types
8   --> $DIR/type_mismatch.rs:5:24

---
To only update this specific test, also pass `--test-args typeof/type_mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeof/type_mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeof/type_mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeof/type_mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0516]: `typeof` is a reserved keyword but unimplemented
   |
   |
LL |     let b: typeof(a) = 1i8;
   |            ^^^^^^^^^ reserved keyword
   |
help: consider replacing `typeof(...)` with an actual type
   |
LL |     let b: u8 = 1i8;

error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeof/type_mismatch.rs:5:24
   |
   |
LL |     let b: typeof(a) = 1i8;
   |            ---------   ^^^ expected `u8`, found `i8`
   |            expected due to this
   |
   |
help: change the type of the numeric literal from `i8` to `u8`
   |
LL |     let b: typeof(a) = 1u8;

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0516.
