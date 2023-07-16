plain
Successfully built 366ffb47291a
Successfully tagged rust-ci:latest
Built container sha256:366ffb47291abefbfd97e963109e12a6cd5bf86e93e8479d845101d392cc81f6
Uploading finished image to https://ci-caches.rust-lang.org/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775
upload failed: - to s3://rust-lang-ci-sccache2/docker/7a933025cb171c4b5f5b6011a3583795b8dcc545c75914dd6be8b89a1706a0990abf8b940dcd0d758184ddd671b7cd225927f49feaa72b37c79d66fed681b775 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-12]
---

54 LL | | }
55    | |_^
56 
- error: `extern` block uses type `Option<Unique<u8>>`, which is not FFI-safe
-   --> $DIR/lint-ctypes-enum.rs:69:17
-    |
- LL |    fn unique(x: Option<std::ptr::Unique<u8>>);
-    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
-    |
-    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    = note: enum has no representation hint
- 
66 error: `extern` block uses type `u128`, which is not FFI-safe
68    |


106    = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
107    = note: enum has no representation hint
- error: aborting due to 9 previous errors
+ error: aborting due to 8 previous errors
110 
111 
---
To only update this specific test, also pass `--test-args lint/lint-ctypes-enum.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-ctypes-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-ctypes-enum/auxiliary"
stdout: none
--- stderr -------------------------------
error: `extern` block uses type `U`, which is not FFI-safe
   |
   |
LL |    fn uf(x: U); //~ ERROR `extern` block uses type `U`
   |             ^ not FFI-safe
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:3:9
   |
   |
LL | #![deny(improper_ctypes)]
   |         ^^^^^^^^^^^^^^^
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint
  --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:9:1
   |
   |
LL | / enum U {
LL | |     A,
LL | | }


error: `extern` block uses type `B`, which is not FFI-safe
   |
   |
LL |    fn bf(x: B); //~ ERROR `extern` block uses type `B`
   |             ^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint
  --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:12:1
   |
   |
LL | / enum B {
LL | |     C,
LL | |     D,
LL | | }


error: `extern` block uses type `T`, which is not FFI-safe
   |
   |
LL |    fn tf(x: T); //~ ERROR `extern` block uses type `T`
   |             ^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint
  --> /checkout/src/test/ui/lint/lint-ctypes-enum.rs:16:1
   |
   |
LL | / enum T {
LL | |     E,
LL | |     F,
LL | |     G,
LL | | }


error: `extern` block uses type `u128`, which is not FFI-safe
   |
   |
LL |    fn nonzero_u128(x: Option<num::NonZeroU128>);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = note: 128-bit integers don't currently have a known stable ABI

error: `extern` block uses type `i128`, which is not FFI-safe
   |
   |
LL |    fn nonzero_i128(x: Option<num::NonZeroI128>);
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = note: 128-bit integers don't currently have a known stable ABI

error: `extern` block uses type `Option<TransparentUnion<NonZeroU8>>`, which is not FFI-safe
   |
   |
LL |    fn transparent_union(x: Option<TransparentUnion<num::NonZeroU8>>);
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint

error: `extern` block uses type `Option<Rust<NonZeroU8>>`, which is not FFI-safe
   |
   |
LL |    fn repr_rust(x: Option<Rust<num::NonZeroU8>>); //~ ERROR `extern` block uses type
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint

error: `extern` block uses type `Result<(), NonZeroI32>`, which is not FFI-safe
   |
   |
LL |    fn no_result(x: Result<(), num::NonZeroI32>); //~ ERROR `extern` block uses type
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not FFI-safe
   |
   = help: consider adding a `#[repr(C)]`, `#[repr(transparent)]`, or integer `#[repr(...)]` attribute to this enum
   = note: enum has no representation hint
error: aborting due to 8 previous errors
------------------------------------------



---- [ui] src/test/ui/traits/cycle-cache-err-60010.rs stdout ----
diff of stderr:

4 LL |     _parse: <ParseQuery as Query<RootDatabase>>::Data,
6    |
6    |
-    = note: required because it appears within the type `*const SalsaStorage`
+    = note: required because it appears within the type `PhantomData<SalsaStorage>`
8    = note: required because it appears within the type `Unique<SalsaStorage>`
9    = note: required because it appears within the type `Box<SalsaStorage>`
10 note: required because it appears within the type `Runtime<RootDatabase>`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/cycle-cache-err-60010.stderr
To only update this specific test, also pass `--test-args traits/cycle-cache-err-60010.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `SalsaStorage: RefUnwindSafe`
   |
   |
LL |     _parse: <ParseQuery as Query<RootDatabase>>::Data,
   |
   |
   = note: required because it appears within the type `PhantomData<SalsaStorage>`
   = note: required because it appears within the type `Unique<SalsaStorage>`
   = note: required because it appears within the type `Box<SalsaStorage>`
note: required because it appears within the type `Runtime<RootDatabase>`
   |
   |
LL | struct Runtime<DB: Database> {
   |        ^^^^^^^
note: required because it appears within the type `RootDatabase`
   |
   |
LL | struct RootDatabase {
   |        ^^^^^^^^^^^^
note: required because of the requirements on the impl of `SourceDatabase` for `RootDatabase`
   |
   |
LL | impl<T> SourceDatabase for T
   |         ^^^^^^^^^^^^^^     ^
note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
   |
   |
LL | impl<DB> Query<DB> for ParseQuery

error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
