plain
.................................................................................................... 11700/12648
.................................................................................................... 11800/12648
....................................................................i............................... 11900/12648
.................................................................................................... 12000/12648
..................F..FF............................................................................. 12100/12648
.................................................................................................... 12300/12648
.................................................................................................... 12400/12648
.................................................................................................... 12500/12648
............................................iii..................................................... 12600/12648
............................................iii..................................................... 12600/12648
................................................
failures:

---- [ui] ui/typeck/issue-87181/tuple-fields.rs stdout ----

2   --> $DIR/tuple-fields.rs:12:15
3    |
3    |
4 LL |     thing.bar.0;
-    |     --------- ^
-    |     |
-    |     help: call the constructor: `(thing.bar)(_, _)`
8    |
8    |
9    = help: `thing.bar` is the constructor of a struct or enum variant
+ help: call the constructor
+    |
+ LL |     (thing.bar)(_, _).0;
10 
11 error: aborting due to previous error
12 

---
To only update this specific test, also pass `--test-args typeck/issue-87181/tuple-fields.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-87181/tuple-fields.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-87181/tuple-fields" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-87181/tuple-fields/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0609]: no field `0` on type `fn(char, u16) -> Foo {Foo}`
  --> /checkout/src/test/ui/typeck/issue-87181/tuple-fields.rs:12:15
   |
LL |     thing.bar.0;
   |
   |
   = help: `thing.bar` is the constructor of a struct or enum variant
   |
   |
LL |     (thing.bar)(_, _).0;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0609`.
For more information about this error, try `rustc --explain E0609`.

------------------------------------------


---- [ui] ui/typeck/issue-87181/tuple-method.rs stdout ----

2   --> $DIR/tuple-method.rs:12:15
3    |
3    |
4 LL |     thing.bar.foo();
-    |     --------- ^^^ method not found in `fn(u8, i32) -> Foo {Foo}`
-    |     |
-    |     help: call the constructor: `(thing.bar)(_, _)`
+    |               ^^^ method not found in `fn(u8, i32) -> Foo {Foo}`
8    |
9    = help: `thing.bar` is the constructor of a struct or enum variant
+ help: call the constructor
+    |
+ LL |     (thing.bar)(_, _).foo();
10 
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-87181/tuple-method/tuple-method.stderr
To only update this specific test, also pass `--test-args typeck/issue-87181/tuple-method.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-87181/tuple-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-87181/tuple-method" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-87181/tuple-method/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `foo` found for fn item `fn(u8, i32) -> Foo {Foo}` in the current scope
  --> /checkout/src/test/ui/typeck/issue-87181/tuple-method.rs:12:15
   |
LL |     thing.bar.foo();
   |               ^^^ method not found in `fn(u8, i32) -> Foo {Foo}`
   |
   = help: `thing.bar` is the constructor of a struct or enum variant
   |
   |
LL |     (thing.bar)(_, _).foo();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui/typeck/issue-87181/empty-tuple-method.rs stdout ----

2   --> $DIR/empty-tuple-method.rs:12:15
3    |
3    |
4 LL |     thing.bar.foo();
-    |     --------- ^^^ method not found in `fn() -> Foo {Foo}`
-    |     |
-    |     help: call the constructor: `(thing.bar)()`
+    |               ^^^ method not found in `fn() -> Foo {Foo}`
8    |
9    = help: `thing.bar` is the constructor of a struct or enum variant
+ help: call the constructor
+    |
+ LL |     (thing.bar)().foo();
10 
11 error: aborting due to previous error
12 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-87181/empty-tuple-method/empty-tuple-method.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-87181/empty-tuple-method.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-87181/empty-tuple-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-87181/empty-tuple-method" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-87181/empty-tuple-method/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `foo` found for fn item `fn() -> Foo {Foo}` in the current scope
  --> /checkout/src/test/ui/typeck/issue-87181/empty-tuple-method.rs:12:15
   |
LL |     thing.bar.foo();
   |               ^^^ method not found in `fn() -> Foo {Foo}`
   |
   = help: `thing.bar` is the constructor of a struct or enum variant
   |
   |
LL |     (thing.bar)().foo();

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------



failures:
    [ui] ui/typeck/issue-87181/empty-tuple-method.rs
    [ui] ui/typeck/issue-87181/tuple-fields.rs
    [ui] ui/typeck/issue-87181/tuple-method.rs
test result: FAILED. 12517 passed; 3 failed; 128 ignored; 0 measured; 0 filtered out; finished in 126.72s

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Build completed unsuccessfully in 0:13:20
