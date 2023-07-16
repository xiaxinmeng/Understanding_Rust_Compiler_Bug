plain
Successfully built 45bd5abf6cd7
Successfully tagged rust-ci:latest
Built container sha256:45bd5abf6cd7c61ee194f3299b1313b935ee12a0b8ecc1e79c5abe47a02e8635
Uploading finished image to https://ci-caches.rust-lang.org/docker/3ba9d538a45014cf9c069a7f7b39b17975213bb4ad3cb92953bcf42e2feac7246274891423ce6ec10459580375996bc5323b9b2e1eaa3f9fe83d7d3a2f2335cf
upload failed: - to s3://rust-lang-ci-sccache2/docker/3ba9d538a45014cf9c069a7f7b39b17975213bb4ad3cb92953bcf42e2feac7246274891423ce6ec10459580375996bc5323b9b2e1eaa3f9fe83d7d3a2f2335cf Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---

36    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
37    = note: `#[warn(anonymous_parameters)]` on by default
38 
- error[E0718]: `fn` language item must be applied to a trait with 1 generic argument
+ error[E0718]: `r#fn` language item must be applied to a trait with 1 generic argument
41    |
41    |
42 LL | #[lang = "fn"]
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-83471/issue-83471.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-83471/issue-83471.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lang-items/issue-83471.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-items/issue-83471.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-83471" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-83471/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0573]: expected type, found built-in attribute `export_name`
   |
LL |     fn call(export_name);
   |             ^^^^^^^^^^^ not a type

---

error[E0658]: language items are subject to change
  --> /checkout/src/test/ui/lang-items/issue-83471.rs:7:1
   |
LL | #[lang = "sized"]
   |
   = help: add `#![feature(lang_items)]` to the crate attributes to enable

error[E0658]: language items are subject to change
error[E0658]: language items are subject to change
  --> /checkout/src/test/ui/lang-items/issue-83471.rs:11:1
   |
LL | #[lang = "fn"]
   |
   = help: add `#![feature(lang_items)]` to the crate attributes to enable

warning: anonymous parameters are deprecated and will be removed in the next edition
---
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
   = note: `#[warn(anonymous_parameters)]` on by default

error[E0718]: `r#fn` language item must be applied to a trait with 1 generic argument
   |
   |
LL | #[lang = "fn"]
...
LL | trait Fn {
   |         - this trait has 0 generic arguments

---

---- [ui] src/test/ui/rfc-2632-const-trait-impl/const-impl-requires-const-trait.rs stdout ----
diff of stderr:

+ error: const `impl`s must be for traits marked with `#[const_trait]`
+    |
+ LL | impl const A for () {}
+    | ^^^^^^^^^^^^^^^^^^^
+    |
+    |
+ note: this trait must be annotated with `#[const_trait]`
+    |
+    |
+ LL | pub trait A {}
+ 
+ 
1 error: const `impl` for trait `A` which is not marked with `#[const_trait]`
3    |


10    = note: marking a trait with `#[const_trait]` ensures all default method bodies are `const`
11    = note: adding a non-const method body in the future would be a breaking change
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
14 
15 
---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/const-impl-requires-const-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/const-impl-requires-const-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-impl-requires-const-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/const-impl-requires-const-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error: const `impl`s must be for traits marked with `#[const_trait]`
   |
LL | impl const A for () {}
   | ^^^^^^^^^^^^^^^^^^^
   |
   |
note: this trait must be annotated with `#[const_trait]`
   |
LL | pub trait A {}
   | ^^^^^^^^^^^


error: const `impl` for trait `A` which is not marked with `#[const_trait]`
   |
LL | pub trait A {}
LL | pub trait A {}
   | - help: mark `A` as const: `#[const_trait]`
LL | impl const A for () {}
   |            ^
   |
   |
   = note: marking a trait with `#[const_trait]` ensures all default method bodies are `const`
   = note: adding a non-const method body in the future would be a breaking change
error: aborting due to 2 previous errors
------------------------------------------


