plain
diff of stderr:

14   --> $DIR/fn-help-with-err.rs:12:10
15    |
16 LL |     arc2.blablabla();
-    |          ^^^^^^^^^ method not found in `Arc<[closure@$DIR/fn-help-with-err.rs:10:36: 10:40]>`
-    = note: `arc2` is a function, perhaps you wish to call it
-    = note: `arc2` is a function, perhaps you wish to call it
+    |     ---- ^^^^^^^^^ method not found in `Arc<[closure@$DIR/fn-help-with-err.rs:10:36: 10:40]>`
+    |     this is a function, perhaps you wish to call it
20 
21 error: aborting due to 3 previous errors
22 
---
To only update this specific test, also pass `--test-args functions-closures/fn-help-with-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/functions-closures/fn-help-with-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/fn-help-with-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/functions-closures/fn-help-with-err/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `oops` in this scope
   |
   |
LL |     let arc = std::sync::Arc::new(oops);


error[E0599]: no method named `blablabla` found for struct `Arc<_>` in the current scope
   |
   |
LL |     arc.blablabla();
   |         ^^^^^^^^^ method not found in `Arc<_>`

error[E0599]: no method named `blablabla` found for struct `Arc<[closure@/checkout/src/test/ui/functions-closures/fn-help-with-err.rs:10:36: 10:40]>` in the current scope
   |
   |
LL |     arc2.blablabla();
   |     ---- ^^^^^^^^^ method not found in `Arc<[closure@/checkout/src/test/ui/functions-closures/fn-help-with-err.rs:10:36: 10:40]>`
   |     this is a function, perhaps you wish to call it

error: aborting due to 3 previous errors

---
diff of stderr:

2   --> $DIR/issue-29124.rs:15:15
3    |
4 LL |     Obj::func.x();
-    |               ^ method not found in `fn() -> Ret {Obj::func}`
-    |
-    = note: `Obj::func` is a function, perhaps you wish to call it
+    |     --------- ^ method not found in `fn() -> Ret {Obj::func}`
+    |     this is a function, perhaps you wish to call it
8 
8 
9 error[E0599]: no method named `x` found for fn item `fn() -> Ret {func}` in the current scope

11    |
12 LL |     func.x();
12 LL |     func.x();
-    |          ^ method not found in `fn() -> Ret {func}`
-    |
-    = note: `func` is a function, perhaps you wish to call it
+    |     ---- ^ method not found in `fn() -> Ret {func}`
+    |     this is a function, perhaps you wish to call it
16 
17 error: aborting due to 2 previous errors
18 
---
To only update this specific test, also pass `--test-args issues/issue-29124.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-29124.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29124" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-29124/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `x` found for fn item `fn() -> Ret {Obj::func}` in the current scope
   |
   |
LL |     Obj::func.x();
   |     --------- ^ method not found in `fn() -> Ret {Obj::func}`
   |     this is a function, perhaps you wish to call it


error[E0599]: no method named `x` found for fn item `fn() -> Ret {func}` in the current scope
   |
LL |     func.x();
LL |     func.x();
   |     ---- ^ method not found in `fn() -> Ret {func}`
   |     this is a function, perhaps you wish to call it

error: aborting due to 2 previous errors

---
diff of stderr:

2   --> $DIR/issue-57362-1.rs:20:7
3    |
4 LL |     a.f();
-    |       ^ method not found in `fn(&u8)`
+    |     - ^ method not found in `fn(&u8)`
+    |     this is a function, perhaps you wish to call it
6    |
6    |
-    = note: `a` is a function, perhaps you wish to call it
8    = help: items from traits can only be used if the trait is implemented and in scope
9 note: `Trait` defines an item `f`, perhaps you need to implement it


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57362-1/issue-57362-1.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57362-1/issue-57362-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-57362-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-57362-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57362-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57362-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `f` found for fn pointer `fn(&u8)` in the current scope
   |
   |
LL |     a.f(); //~ ERROR no method named `f`
   |     - ^ method not found in `fn(&u8)`
   |     this is a function, perhaps you wish to call it
   |
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `Trait` defines an item `f`, perhaps you need to implement it
   |
LL | trait Trait {
   | ^^^^^^^^^^^

---
diff of stderr:

2   --> $DIR/unboxed-closures-static-call-wrong-trait.rs:7:10
3    |
4 LL |     mut_.call((0, ));
-    |          ^^^^ method not found in `[closure@$DIR/unboxed-closures-static-call-wrong-trait.rs:6:26: 6:31]`
-    |
-    = note: `mut_` is a function, perhaps you wish to call it
+    |     ---- ^^^^ method not found in `[closure@$DIR/unboxed-closures-static-call-wrong-trait.rs:6:26: 6:31]`
+    |     this is a function, perhaps you wish to call it
8 
9 error: aborting due to previous error
10 
---
To only update this specific test, also pass `--test-args unboxed-closures/unboxed-closures-static-call-wrong-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unboxed-closures/unboxed-closures-static-call-wrong-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-static-call-wrong-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unboxed-closures/unboxed-closures-static-call-wrong-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `call` found for closure `[closure@/checkout/src/test/ui/unboxed-closures/unboxed-closures-static-call-wrong-trait.rs:6:26: 6:31]` in the current scope
   |
   |
LL |     mut_.call((0, )); //~ ERROR no method named `call` found
   |     ---- ^^^^ method not found in `[closure@/checkout/src/test/ui/unboxed-closures/unboxed-closures-static-call-wrong-trait.rs:6:26: 6:31]`
   |     this is a function, perhaps you wish to call it

error: aborting due to previous error

