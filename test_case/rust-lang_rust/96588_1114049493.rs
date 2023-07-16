plain
To only update this specific test, also pass `--test-args issues/issue-77218/issue-77218-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-77218/issue-77218-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-77218/issue-77218-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-77218/issue-77218-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0070]: invalid left-hand side of assignment
   |
   |
LL |     while Some(0) = value.get(0) { //~ ERROR invalid left-hand side of assignment
   |                -  ^
   |                cannot assign to this expression
   |
help: you might have meant to use pattern destructuring
   |
   |
LL |     while let Some(0) = value.get(0) { //~ ERROR invalid left-hand side of assignment

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-77218/issue-77218-2.rs:4:11
   |
   |
LL |     while Some(0) = value.get(0) { //~ ERROR invalid left-hand side of assignment
   |           ^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0070, E0308.
For more information about an error, try `rustc --explain E0070`.
---
To only update this specific test, also pass `--test-args issues/issue-77218/issue-77218.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-77218/issue-77218.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-77218/issue-77218" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-77218/issue-77218/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0070]: invalid left-hand side of assignment
   |
   |
LL |     while Some(0) = value.get(0) {} //~ ERROR invalid left-hand side of assignment
   |                -  ^
   |                cannot assign to this expression
   |
help: you might have meant to use pattern destructuring
   |
   |
LL |     while let Some(0) = value.get(0) {} //~ ERROR invalid left-hand side of assignment

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-77218/issue-77218.rs:4:11
   |
   |
LL |     while Some(0) = value.get(0) {} //~ ERROR invalid left-hand side of assignment
   |           ^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `()`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0070, E0308.
For more information about an error, try `rustc --explain E0070`.
---
22 
23 error[E0308]: mismatched types
+   --> $DIR/while-let-typo.rs:4:11
+    |
+ LL |     while Some(x) = foo {}
+    |           ^^^^^^^^^^^^^ expected `bool`, found `()`
+ error[E0308]: mismatched types
+   --> $DIR/while-let-typo.rs:5:11
+    |
+    |
+ LL |     while Some(foo) = bar {}
+    |           ^^^^^^^^^^^^^^^ expected `bool`, found `()`
+ error[E0308]: mismatched types
24   --> $DIR/while-let-typo.rs:6:11
25    |
26 LL |     while 3 = foo {}
---
To only update this specific test, also pass `--test-args suggestions/while-let-typo.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/while-let-typo.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/while-let-typo" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/while-let-typo/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `x` in this scope
   |
   |
LL |     while Some(x) = foo {} //~ ERROR cannot find value `x` in this scope
   |
help: you might have meant to use pattern matching
   |
   |
LL |     while let Some(x) = foo {} //~ ERROR cannot find value `x` in this scope

error[E0425]: cannot find value `x` in this scope
  --> /checkout/src/test/ui/suggestions/while-let-typo.rs:8:11
   |
   |
LL |     while x = 5 {} //~ ERROR cannot find value `x` in this scope
   |
help: you might have meant to use pattern matching
   |
   |
LL |     while let x = 5 {} //~ ERROR cannot find value `x` in this scope
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/while-let-typo.rs:4:11
  --> /checkout/src/test/ui/suggestions/while-let-typo.rs:4:11
   |
LL |     while Some(x) = foo {} //~ ERROR cannot find value `x` in this scope
   |           ^^^^^^^^^^^^^ expected `bool`, found `()`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/while-let-typo.rs:5:11
   |
   |
LL |     while Some(foo) = bar {}
   |           ^^^^^^^^^^^^^^^ expected `bool`, found `()`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/while-let-typo.rs:6:11
   |
   |
LL |     while 3 = foo {} //~ ERROR mismatched types
   |           ^^^^^^^ expected `bool`, found `()`

error[E0070]: invalid left-hand side of assignment
   |
   |
LL |     while Some(3) = foo {} //~ ERROR invalid left-hand side of assignment
   |                -  ^
   |                cannot assign to this expression
   |
help: you might have meant to use pattern destructuring
   |
   |
LL |     while let Some(3) = foo {} //~ ERROR invalid left-hand side of assignment

error[E0308]: mismatched types
  --> /checkout/src/test/ui/suggestions/while-let-typo.rs:7:11
   |
   |
LL |     while Some(3) = foo {} //~ ERROR invalid left-hand side of assignment
   |           ^^^^^^^^^^^^^ expected `bool`, found `()`
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0070, E0308, E0425.
For more information about an error, try `rustc --explain E0070`.
