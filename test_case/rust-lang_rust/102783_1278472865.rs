plain
---- [ui] src/test/ui/threads-sendsync/issue-43733-2.rs stdout ----

error: /checkout/src/test/ui/threads-sendsync/issue-43733-2.rs:24: unexpected error: '24:5: 24:44: unresolved import `std::thread::__FastLocalKeyInner` [E0432]'

error: /checkout/src/test/ui/threads-sendsync/issue-43733-2.rs:26: expected error not found: `UnsafeCell<Option<()>>` cannot be shared between threads

error: /checkout/src/test/ui/threads-sendsync/issue-43733-2.rs:26: expected error not found: cannot be shared between threads safely [E0277]
error: 1 unexpected errors found, 2 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/issue-43733-2.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733-2/auxiliary"
    Error {
        line_num: 24,
        kind: Some(
            Error,
---
        line_num: 26,
        kind: Some(
            Error,
        ),
        msg: "`UnsafeCell<Option<()>>` cannot be shared between threads",
    Error {
        line_num: 26,
        kind: Some(
            Error,
            Error,
        ),
        msg: "cannot be shared between threads safely [E0277]",
]

thread '[ui] src/test/ui/threads-sendsync/issue-43733-2.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:1434:13


---- [ui] src/test/ui/threads-sendsync/issue-43733.rs#mir stdout ----
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-emscripten
diff of stderr:

- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
-   --> $DIR/issue-43733.rs:21:5
+ error[E0433]: failed to resolve: could not find `__FastLocalKeyInner` in `thread`
3    |
3    |
- LL |     __KEY.get(Default::default)
-    |
-    = note: consult the function's documentation for information on how to avoid undefined behavior
-    = note: consult the function's documentation for information on how to avoid undefined behavior
+ LL | static __KEY: std::thread::__FastLocalKeyInner<Foo> = std::thread::__FastLocalKeyInner::new();
+    |                                                                    ^^^^^^^^^^^^^^^^^^^ could not find `__FastLocalKeyInner` in `thread`
- error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
-   --> $DIR/issue-43733.rs:26:42
+ error[E0412]: cannot find type `__FastLocalKeyInner` in module `std::thread`
+   --> $DIR/issue-43733.rs:15:28
+   --> $DIR/issue-43733.rs:15:28
11    |
- LL | static FOO: std::thread::LocalKey<Foo> = std::thread::LocalKey::new(__getit);
-    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
+ LL | static __KEY: std::thread::__FastLocalKeyInner<Foo> = std::thread::__FastLocalKeyInner::new();
+    |                            ^^^^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `__StaticLocalKeyInner`
-    = note: consult the function's documentation for information on how to avoid undefined behavior
+   ::: $SRC_DIR/std/src/thread/local.rs:LL:COL
+    |
+    |
+ LL |     pub struct Key<T> {
+    |     ----------------- similarly named struct `__StaticLocalKeyInner` defined here
17 error: aborting due to 2 previous errors
18 

- For more information about this error, try `rustc --explain E0133`.
- For more information about this error, try `rustc --explain E0133`.
+ Some errors have detailed explanations: E0412, E0433.
+ For more information about an error, try `rustc --explain E0412`.
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir/issue-43733.mir.stderr
To only update this specific test, also pass `--test-args threads-sendsync/issue-43733.rs`


error in revision `mir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/issue-43733.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "mir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.mir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: could not find `__FastLocalKeyInner` in `thread`
   |
   |
LL | static __KEY: std::thread::__FastLocalKeyInner<Foo> = std::thread::__FastLocalKeyInner::new();
   |                                                                    ^^^^^^^^^^^^^^^^^^^ could not find `__FastLocalKeyInner` in `thread`
error[E0412]: cannot find type `__FastLocalKeyInner` in module `std::thread`
  --> /checkout/src/test/ui/threads-sendsync/issue-43733.rs:15:28
   |
   |
LL | static __KEY: std::thread::__FastLocalKeyInner<Foo> = std::thread::__FastLocalKeyInner::new();
   |                            ^^^^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `__StaticLocalKeyInner`
  ::: /checkout/library/std/src/thread/local.rs:869:5
   |
LL |     pub struct Key<T> {
LL |     pub struct Key<T> {
   |     ----------------- similarly named struct `__StaticLocalKeyInner` defined here
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.
------------------------------------------


---- [ui] src/test/ui/threads-sendsync/issue-43733.rs#thir stdout ----
diff of stderr:

- error[E0133]: call to unsafe function `$LOCALKEYINNER::<T>::get` is unsafe and requires unsafe function or block
-   --> $DIR/issue-43733.rs:21:5
+ error[E0433]: failed to resolve: could not find `__FastLocalKeyInner` in `thread`
3    |
3    |
- LL |     __KEY.get(Default::default)
-    |
-    = note: consult the function's documentation for information on how to avoid undefined behavior
-    = note: consult the function's documentation for information on how to avoid undefined behavior
+ LL | static __KEY: std::thread::__FastLocalKeyInner<Foo> = std::thread::__FastLocalKeyInner::new();
+    |                                                                    ^^^^^^^^^^^^^^^^^^^ could not find `__FastLocalKeyInner` in `thread`
8 
- error[E0133]: call to unsafe function `LocalKey::<T>::new` is unsafe and requires unsafe function or block
-   --> $DIR/issue-43733.rs:26:42
+ error[E0412]: cannot find type `__FastLocalKeyInner` in module `std::thread`
11    |
11    |
- LL | static FOO: std::thread::LocalKey<Foo> = std::thread::LocalKey::new(__getit);
-    |                                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
+ LL | static __KEY: std::thread::__FastLocalKeyInner<Foo> = std::thread::__FastLocalKeyInner::new();
+    |                            ^^^^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `__StaticLocalKeyInner`
-    = note: consult the function's documentation for information on how to avoid undefined behavior
+   ::: $SRC_DIR/std/src/thread/local.rs:LL:COL
+    |
+    |
+ LL |     pub struct Key<T> {
+    |     ----------------- similarly named struct `__StaticLocalKeyInner` defined here
17 error: aborting due to 2 previous errors
18 

- For more information about this error, try `rustc --explain E0133`.
- For more information about this error, try `rustc --explain E0133`.
+ Some errors have detailed explanations: E0412, E0433.
+ For more information about an error, try `rustc --explain E0412`.
20 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir/issue-43733.thir.stderr
To only update this specific test, also pass `--test-args threads-sendsync/issue-43733.rs`


error in revision `thir`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/issue-43733.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--cfg" "thir" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/issue-43733.thir/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: could not find `__FastLocalKeyInner` in `thread`
   |
   |
LL | static __KEY: std::thread::__FastLocalKeyInner<Foo> = std::thread::__FastLocalKeyInner::new();
   |                                                                    ^^^^^^^^^^^^^^^^^^^ could not find `__FastLocalKeyInner` in `thread`
error[E0412]: cannot find type `__FastLocalKeyInner` in module `std::thread`
  --> /checkout/src/test/ui/threads-sendsync/issue-43733.rs:15:28
   |
   |
LL | static __KEY: std::thread::__FastLocalKeyInner<Foo> = std::thread::__FastLocalKeyInner::new();
   |                            ^^^^^^^^^^^^^^^^^^^ help: a struct with a similar name exists: `__StaticLocalKeyInner`
  ::: /checkout/library/std/src/thread/local.rs:869:5
   |
LL |     pub struct Key<T> {
LL |     pub struct Key<T> {
   |     ----------------- similarly named struct `__StaticLocalKeyInner` defined here
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
