plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:8f4b7f84864484a7bf31766abe9204da3cbe65b3)
Download action repository 'rust-lang/simpleinfra@master' (SHA:b0f40ff9e272bf83125cc4e89ea27f05d0d23823)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-16core-64gb)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

6    |
7    = help: the trait `std::fmt::Display` is not implemented for `()`
8    = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
- note: required by a bound in `Foo::bar::{opaque#0}`
+ note: required by a bound in `Foo::{opaque#0}`
10   --> $DIR/doesnt-satisfy.rs:8:22
12 LL |     fn bar() -> impl std::fmt::Display;


-    |                      ^^^^^^^^^^^^^^^^^ required by this bound in `Foo::bar::{opaque#0}`
+    |                      ^^^^^^^^^^^^^^^^^ required by this bound in `Foo::{opaque#0}`
15 error: aborting due to previous error
16 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/doesnt-satisfy.next/doesnt-satisfy.next.stderr
To only update this specific test, also pass `--test-args impl-trait/in-trait/doesnt-satisfy.rs`

error in revision `next`: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-trait/in-trait/doesnt-satisfy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/doesnt-satisfy.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/in-trait/doesnt-satisfy.next/auxiliary" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
error[E0277]: `()` doesn't implement `std::fmt::Display`
  --> fake-test-src-base/impl-trait/in-trait/doesnt-satisfy.rs:12:17
LL |     fn bar() -> () {}
LL |     fn bar() -> () {}
   |                 ^^ `()` cannot be formatted with the default formatter
   = help: the trait `std::fmt::Display` is not implemented for `()`
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `Foo::{opaque#0}`
  --> fake-test-src-base/impl-trait/in-trait/doesnt-satisfy.rs:8:22
LL |     fn bar() -> impl std::fmt::Display;
LL |     fn bar() -> impl std::fmt::Display;
   |                      ^^^^^^^^^^^^^^^^^ required by this bound in `Foo::{opaque#0}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
---

4 LL |         fn main() -> Something {
5    |                      ^^^^^^^^^ the trait `Termination` is not implemented for `Something`
6    |
- note: required by a bound in `Main::main::{opaque#0}`
+ note: required by a bound in `Main::{opaque#0}`
9    |
10 LL |         fn main() -> impl std::process::Termination;


-    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Main::main::{opaque#0}`
+    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Main::{opaque#0}`
13 error: aborting due to previous error
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/issue-103052-2.next/issue-103052-2.next.stderr
To only update this specific test, also pass `--test-args rfc-1937-termination-trait/issue-103052-2.rs`

error in revision `next`: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/rfc-1937-termination-trait/issue-103052-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "next" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/issue-103052-2.next" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/issue-103052-2.next/auxiliary" "-Zlower-impl-trait-in-trait-to-assoc-ty"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Something: Termination` is not satisfied
  --> fake-test-src-base/rfc-1937-termination-trait/issue-103052-2.rs:15:22
LL |         fn main() -> Something {
   |                      ^^^^^^^^^ the trait `Termination` is not implemented for `Something`
   |
   |
note: required by a bound in `Main::{opaque#0}`
  --> fake-test-src-base/rfc-1937-termination-trait/issue-103052-2.rs:9:27
LL |         fn main() -> impl std::process::Termination;
LL |         fn main() -> impl std::process::Termination;
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `Main::{opaque#0}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
