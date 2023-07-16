plain
---- [ui] src/test/ui/impl-trait/cross-return-site-inference.rs stdout ----
diff of stderr:

17 LL |     Ok(())
18    |     ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
19    |
-    = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `gimli`, `std`:
+    = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `std`:
21            - impl<'a> From<&'a str> for Cow<'a, str>;
22            - impl<'a> From<&str> for Box<(dyn std::error::Error + Send + Sync + 'a)>;
23            - impl<> From<&str> for Arc<str>;

24            - impl<> From<&str> for Box<(dyn std::error::Error + 'static)>;
-            and 37 more
+            and 22 more
26    = note: required for `Result<(), _>` to implement `FromResidual<Result<Infallible, &str>>`
27 help: consider specifying the generic arguments

48    |            |
48    |            |
49    |            cannot infer type of the type parameter `E` declared on the enum `Result`
50    |
-    = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `gimli`, `std`:
+    = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `std`:
52            - impl<'a> From<&'a str> for Cow<'a, str>;
53            - impl<'a> From<&str> for Box<(dyn std::error::Error + Send + Sync + 'a)>;
54            - impl<> From<&str> for Arc<str>;

55            - impl<> From<&str> for Box<(dyn std::error::Error + 'static)>;
-            and 37 more
+            and 22 more
57 help: consider specifying the generic arguments
58    |
59 LL |     return Err::<(), E>(From::from("foo"));
78    |     |
78    |     |
79    |     cannot infer type of the type parameter `E` declared on the enum `Result`
80    |
-    = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `gimli`, `std`:
+    = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `std`:
82            - impl<'a> From<&'a str> for Cow<'a, str>;
83            - impl<'a> From<&str> for Box<(dyn std::error::Error + Send + Sync + 'a)>;
84            - impl<> From<&str> for Arc<str>;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-emscripten
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-emscripten
85            - impl<> From<&str> for Box<(dyn std::error::Error + 'static)>;
-            and 37 more
+            and 22 more
87 help: consider specifying the generic arguments
88    |
89 LL |     Err::<(), E>(From::from("foo"))

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/cross-return-site-inference/cross-return-site-inference.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/cross-return-site-inference.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/cross-return-site-inference.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/cross-return-site-inference" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/cross-return-site-inference/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/impl-trait/cross-return-site-inference.rs:33:5
   |
LL |     Ok(())
LL |     Ok(())
   |     ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
help: consider specifying the generic arguments
   |
   |
LL |     Ok::<(), E>(())

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/impl-trait/cross-return-site-inference.rs:33:5
   |
   |
LL |     Err("whoops")?;
   |                  - type must be known at this point
LL |     Ok(())
   |     ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
   |
   = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `std`:
           - impl<'a> From<&'a str> for Cow<'a, str>;
           - impl<'a> From<&str> for Box<(dyn std::error::Error + Send + Sync + 'a)>;
           - impl<> From<&str> for Arc<str>;
           - impl<> From<&str> for Box<(dyn std::error::Error + 'static)>;
           and 22 more
   = note: required for `Result<(), _>` to implement `FromResidual<Result<Infallible, &str>>`
help: consider specifying the generic arguments
   |
LL |     Ok::<(), E>(())

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/impl-trait/cross-return-site-inference.rs:39:12
   |
   |
LL |     return Err(From::from("foo"));
   |            ^^^ cannot infer type of the type parameter `E` declared on the enum `Result`
help: consider specifying the generic arguments
   |
   |
LL |     return Err::<(), E>(From::from("foo"));

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/impl-trait/cross-return-site-inference.rs:39:12
   |
   |
LL |     return Err(From::from("foo"));
   |            ^^^ ---------- type must be known at this point
   |            |
   |            cannot infer type of the type parameter `E` declared on the enum `Result`
   |
   = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `std`:
           - impl<'a> From<&'a str> for Cow<'a, str>;
           - impl<'a> From<&str> for Box<(dyn std::error::Error + Send + Sync + 'a)>;
           - impl<> From<&str> for Arc<str>;
           - impl<> From<&str> for Box<(dyn std::error::Error + 'static)>;
           and 22 more
help: consider specifying the generic arguments
   |
LL |     return Err::<(), E>(From::from("foo"));

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/impl-trait/cross-return-site-inference.rs:46:5
   |
   |
LL |     Err(From::from("foo"))
   |     ^^^ cannot infer type of the type parameter `E` declared on the enum `Result`
help: consider specifying the generic arguments
   |
   |
LL |     Err::<(), E>(From::from("foo"))

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/impl-trait/cross-return-site-inference.rs:46:5
   |
   |
LL |     Err(From::from("foo"))
   |     ^^^ ---------- type must be known at this point
   |     |
   |     cannot infer type of the type parameter `E` declared on the enum `Result`
   |
   = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `std`:
           - impl<'a> From<&'a str> for Cow<'a, str>;
           - impl<'a> From<&str> for Box<(dyn std::error::Error + Send + Sync + 'a)>;
           - impl<> From<&str> for Arc<str>;
           - impl<> From<&str> for Box<(dyn std::error::Error + 'static)>;
           and 22 more
help: consider specifying the generic arguments
   |
LL |     Err::<(), E>(From::from("foo"))

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0282, E0283.
---
---- [ui] src/test/ui/inference/cannot-infer-async.rs stdout ----
diff of stderr:

18 LL |         Ok(())
19    |         ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
20    |
-    = note: multiple `impl`s satisfying `_: From<std::io::Error>` found in the following crates: `alloc`, `core`, `gimli`, `std`:
+    = note: multiple `impl`s satisfying `_: From<std::io::Error>` found in the following crates: `alloc`, `core`, `std`:
22            - impl<'a, E> From<E> for Box<(dyn std::error::Error + 'a)>
23              where E: 'a, E: std::error::Error;
24            - impl<'a, E> From<E> for Box<(dyn std::error::Error + Send + Sync + 'a)>

25              where E: 'a, E: std::error::Error, E: Send, E: Sync;
26            - impl<E> From<E> for Report<E>
27              where E: std::error::Error;
-            - impl<R> From<R> for gimli::read::abbrev::DebugAbbrev<R>;
-            and 29 more
+            - impl<T> From<T> for Arc<T>;
+            and 14 more
30    = note: required for `Result<(), _>` to implement `FromResidual<Result<Infallible, std::io::Error>>`
31 help: consider specifying the generic arguments


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-async/cannot-infer-async.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-async/cannot-infer-async.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/cannot-infer-async.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-async.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-async" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-async/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/cannot-infer-async.rs:13:9
   |
LL |         Ok(())
LL |         Ok(())
   |         ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
help: consider specifying the generic arguments
   |
   |
LL |         Ok::<(), E>(())

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/cannot-infer-async.rs:13:9
   |
   |
LL |         make_unit()?;
   |                    - type must be known at this point
LL |
LL |         Ok(())
   |         ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
   |
   = note: multiple `impl`s satisfying `_: From<std::io::Error>` found in the following crates: `alloc`, `core`, `std`:
           - impl<'a, E> From<E> for Box<(dyn std::error::Error + 'a)>
             where E: 'a, E: std::error::Error;
           - impl<'a, E> From<E> for Box<(dyn std::error::Error + Send + Sync + 'a)>
             where E: 'a, E: std::error::Error, E: Send, E: Sync;
           - impl<E> From<E> for Report<E>
             where E: std::error::Error;
           - impl<T> From<T> for Arc<T>;
           and 14 more
   = note: required for `Result<(), _>` to implement `FromResidual<Result<Infallible, std::io::Error>>`
help: consider specifying the generic arguments
   |
LL |         Ok::<(), E>(())

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0282, E0283.
Some errors have detailed explanations: E0282, E0283.
For more information about an error, try `rustc --explain E0282`.
------------------------------------------


---- [ui] src/test/ui/inference/cannot-infer-closure.rs stdout ----
diff of stderr:

17 LL |         Ok(b)
18    |         ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
19    |
-    = note: multiple `impl`s satisfying `_: From<()>` found in the following crates: `alloc`, `core`, `gimli`, `std`:
-            - impl<R> From<R> for gimli::read::abbrev::DebugAbbrev<R>;
-            - impl<R> From<R> for gimli::read::addr::DebugAddr<R>;
-            - impl<R> From<R> for gimli::read::aranges::DebugAranges<R>;
-            - impl<R> From<R> for gimli::read::line::DebugLine<R>;
-            and 26 more
+    = note: multiple `impl`s satisfying `_: From<()>` found in the following crates: `alloc`, `core`, `std`:
+            - impl<T> From<T> for Arc<T>;
+            - impl<T> From<T> for Box<T>;
+            - impl<T> From<T> for Cell<T>;
+            - impl<T> From<T> for Exclusive<T>;
+            and 11 more
26    = note: required for `Result<(), _>` to implement `FromResidual<Result<Infallible, ()>>`
27 help: consider specifying the generic arguments


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure/cannot-infer-closure.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure/cannot-infer-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/cannot-infer-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-closure.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-closure/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/cannot-infer-closure.rs:4:9
   |
LL |         Ok(b)
LL |         Ok(b)
   |         ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
help: consider specifying the generic arguments
   |
   |
LL |         Ok::<(), E>(b)

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/cannot-infer-closure.rs:4:9
   |
   |
LL |         Err(a)?;
   |               - type must be known at this point
LL |         Ok(b)
   |         ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
   |
   = note: multiple `impl`s satisfying `_: From<()>` found in the following crates: `alloc`, `core`, `std`:
           - impl<T> From<T> for Arc<T>;
           - impl<T> From<T> for Box<T>;
           - impl<T> From<T> for Cell<T>;
           - impl<T> From<T> for Exclusive<T>;
           and 11 more
   = note: required for `Result<(), _>` to implement `FromResidual<Result<Infallible, ()>>`
help: consider specifying the generic arguments
   |
LL |         Ok::<(), E>(b)

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0282, E0283.
---
---- [ui] src/test/ui/inference/cannot-infer-partial-try-return.rs stdout ----
diff of stderr:

24    |
25 LL | impl<E, T> From<E> for QualifiedError<T>
26    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: and more `impl`s found in the following crates: `alloc`, `core`, `gimli`, `std`:
+    = note: and more `impl`s found in the following crates: `alloc`, `core`, `std`:
28            - impl From<Infallible> for TryFromIntError;
29            - impl From<Infallible> for TryFromSliceError;
30            - impl<'a, E> From<E> for Box<(dyn std::error::Error + 'a)>

31              where E: 'a, E: std::error::Error;
32            - impl<'a, E> From<E> for Box<(dyn std::error::Error + Send + Sync + 'a)>
33              where E: 'a, E: std::error::Error, E: Send, E: Sync;
-            and 31 more
+            and 16 more
35 note: required for `QualifiedError<_>` to implement `From<Infallible>`
37    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-partial-try-return/cannot-infer-partial-try-return.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/cannot-infer-partial-try-return.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/cannot-infer-partial-try-return.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-partial-try-return" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/cannot-infer-partial-try-return/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/cannot-infer-partial-try-return.rs:20:9
   |
LL |         infallible()?;
   |         ------------- type must be known at this point
   |         ------------- type must be known at this point
LL |         Ok(())
   |         ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
help: consider specifying the generic arguments
   |
   |
LL |         Ok::<(), QualifiedError<_>>(())

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/cannot-infer-partial-try-return.rs:20:9
   |
   |
LL |         infallible()?;
   |                     - type must be known at this point
LL |         Ok(())
   |         ^^ cannot infer type of the type parameter `E` declared on the enum `Result`
   |
note: multiple `impl`s satisfying `_: From<Infallible>` found
   |
   |
LL | impl<E, T> From<E> for QualifiedError<T>
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: and more `impl`s found in the following crates: `alloc`, `core`, `std`:
           - impl From<Infallible> for TryFromIntError;
           - impl From<Infallible> for TryFromSliceError;
           - impl<'a, E> From<E> for Box<(dyn std::error::Error + 'a)>
             where E: 'a, E: std::error::Error;
           - impl<'a, E> From<E> for Box<(dyn std::error::Error + Send + Sync + 'a)>
             where E: 'a, E: std::error::Error, E: Send, E: Sync;
           and 16 more
note: required for `QualifiedError<_>` to implement `From<Infallible>`
   |
   |
LL | impl<E, T> From<E> for QualifiedError<T>
   |            ^^^^^^^     ^^^^^^^^^^^^^^^^^
   = note: required for `Result<(), QualifiedError<_>>` to implement `FromResidual<Result<Infallible, Infallible>>`
help: consider specifying the generic arguments
   |
LL |         Ok::<(), QualifiedError<_>>(())

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0282, E0283.
---
---- [ui] src/test/ui/inference/issue-71732.rs stdout ----
diff of stderr:

28    |          |
29    |          cannot infer type of the type parameter `Q` declared on the associated function `get`
30    |
-    = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `gimli`, `std`:
+    = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `std`:
32            - impl<'a> From<&'a str> for Cow<'a, str>;
33            - impl<'a> From<&str> for Box<(dyn std::error::Error + Send + Sync + 'a)>;
34            - impl<> From<&str> for Arc<str>;

35            - impl<> From<&str> for Box<(dyn std::error::Error + 'static)>;
-            and 37 more
+            and 22 more
37    = note: required for `&str` to implement `Into<_>`
38 help: consider specifying the generic argument


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-71732/issue-71732.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-71732/issue-71732.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/issue-71732.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/issue-71732.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-71732" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-71732/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/inference/issue-71732.rs:18:10
   |
   |
LL |         .get(&"key".into())
   |          ^^^ ------------- type must be known at this point
   |          |
   |          cannot infer type of the type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: Borrow<_>` found in the following crates: `alloc`, `core`:
           - impl Borrow<str> for String;
           - impl<T> Borrow<T> for T
             where T: ?Sized;
note: required by a bound in `HashMap::<K, V, S>::get`
   |
   |
LL |         K: Borrow<Q>,
   |            ^^^^^^^^^ required by this bound in `HashMap::<K, V, S>::get`
help: consider specifying the generic argument
   |
LL |         .get::<Q>(&"key".into())

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-71732.rs:18:10
   |
   |
LL |         .get(&"key".into())
   |          ^^^        ---- type must be known at this point
   |          |
   |          cannot infer type of the type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `_: From<&str>` found in the following crates: `alloc`, `core`, `std`:
           - impl<'a> From<&'a str> for Cow<'a, str>;
           - impl<'a> From<&str> for Box<(dyn std::error::Error + Send + Sync + 'a)>;
           - impl<> From<&str> for Arc<str>;
           - impl<> From<&str> for Box<(dyn std::error::Error + 'static)>;
           and 22 more
   = note: required for `&str` to implement `Into<_>`
help: consider specifying the generic argument
   |
LL |         .get::<Q>(&"key".into())

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0283`.
For more information about this error, try `rustc --explain E0283`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-71584.rs stdout ----
diff of stderr:

18 LL |     d = d % n.into();
20    |
20    |
-    = note: multiple `impl`s satisfying `_: From<u32>` found in the following crates: `alloc`, `core`, `gimli`, `std`:
+    = note: multiple `impl`s satisfying `_: From<u32>` found in the following crates: `alloc`, `core`, `std`:
22            - impl From<u32> for AtomicU32;
23            - impl From<u32> for Ipv4Addr;
24            - impl From<u32> for f64;

25            - impl From<u32> for i128;
-            and 33 more
+            and 18 more
27    = note: required for `u32` to implement `Into<_>`
29    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-71584/issue-71584.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-71584.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-71584.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-71584" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-71584/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-71584.rs:4:15
   |
   |
LL |     d = d % n.into();
   |           |
   |           type must be known at this point
   |
   |
   = note: cannot satisfy `<u64 as Rem<_>>::Output == u64`
   |
   |
LL |     d = d % <u32 as Into<T>>::into(n);

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-71584.rs:4:15
   |
   |
LL |     d = d % n.into();
   |
   |
   = note: multiple `impl`s satisfying `_: From<u32>` found in the following crates: `alloc`, `core`, `std`:
           - impl From<u32> for AtomicU32;
           - impl From<u32> for Ipv4Addr;
           - impl From<u32> for f64;
           - impl From<u32> for i128;
           and 18 more
   = note: required for `u32` to implement `Into<_>`
   |
   |
LL |     d = d % <u32 as Into<T>>::into(n);

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0283, E0284.
Some errors have detailed explanations: E0283, E0284.
For more information about an error, try `rustc --explain E0283`.
------------------------------------------


---- [ui] src/test/ui/traits/issue-77982.rs stdout ----
diff of stderr:

63 LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
65    |
65    |
-    = note: multiple `impl`s satisfying `_: From<u32>` found in the following crates: `alloc`, `core`, `gimli`, `std`:
+    = note: multiple `impl`s satisfying `_: From<u32>` found in the following crates: `alloc`, `core`, `std`:
67            - impl From<u32> for AtomicU32;
68            - impl From<u32> for Ipv4Addr;
69            - impl From<u32> for f64;

70            - impl From<u32> for i128;
-            and 33 more
+            and 18 more
72    = note: required for `u32` to implement `Into<_>`
74    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-77982.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-77982.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^ ------------ type must be known at this point
   |          |
   |          cannot infer type of the type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: Borrow<_>` found in the following crates: `alloc`, `core`:
           - impl Borrow<str> for String;
           - impl<T> Borrow<T> for T
             where T: ?Sized;
note: required by a bound in `HashMap::<K, V, S>::get`
   |
   |
LL |         K: Borrow<Q>,
   |            ^^^^^^^^^ required by this bound in `HashMap::<K, V, S>::get`
help: consider specifying the generic argument
   |
LL |     opts.get::<Q>(opt.as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^     ------ type must be known at this point
   |          |
   |          cannot infer type of the type parameter `Q` declared on the associated function `get`
   |
   = note: multiple `impl`s satisfying `String: AsRef<_>` found in the following crates: `alloc`, `std`:
           - impl AsRef<OsStr> for String;
           - impl AsRef<Path> for String;
           - impl AsRef<[u8]> for String;
           - impl AsRef<str> for String;
help: consider specifying the generic argument
   |
LL |     opts.get::<Q>(opt.as_ref()); //~ ERROR type annotations needed

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:13:59
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
   |                                            |
   |                                            required by a bound introduced by this call
   |
   |
   = note: multiple `impl`s satisfying `u32: From<_>` found in the following crates: `core`, `std`:
           - impl From<Ipv4Addr> for u32;
           - impl From<NonZeroU32> for u32;
           - impl From<bool> for u32;
           - impl From<char> for u32;
           and 4 more
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(<u32 as Into<T>>::into(0u32))).collect();

error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:13:59
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
   |
   |
   = note: multiple `impl`s satisfying `_: From<u32>` found in the following crates: `alloc`, `core`, `std`:
           - impl From<u32> for AtomicU32;
           - impl From<u32> for Ipv4Addr;
           - impl From<u32> for f64;
           - impl From<u32> for i128;
           and 18 more
   = note: required for `u32` to implement `Into<_>`
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(<u32 as Into<T>>::into(0u32))).collect();

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:37:9
   |
   |
LL |     let _ = ().foo(); //~ ERROR type annotations needed
   |         ^      --- type must be known at this point
   |
note: multiple `impl`s satisfying `(): Foo<'_, _>` found
   |
   |
LL | impl Foo<'static, u32> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Foo<'a, i16> for () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider giving this pattern a type, where the type for type parameter `T` is specified
   |
LL |     let _: Box<T> = ().foo(); //~ ERROR type annotations needed

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:41:9
   |
   |
LL |     let _ = (&()).bar(); //~ ERROR type annotations needed
   |         ^         --- type must be known at this point
   |
note: multiple `impl`s satisfying `&(): Bar<'_, _>` found
   |
   |
LL | impl<'a> Bar<'static, u32> for &'a () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
LL | impl<'a> Bar<'a, i16> for &'a () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: consider giving this pattern a type, where the type for type parameter `T` is specified
   |
LL |     let _: Box<T> = (&()).bar(); //~ ERROR type annotations needed

error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0283`.
