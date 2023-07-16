plain
12 warning: associated type bounds are unstable
-   --> $DIR/constraints-before-generic-args-syntactic-pass.rs:7:23
+   --> $DIR/constraints-before-generic-args-syntactic-pass.rs:8:23
14    |
15 LL |     foo::<T = u8, 'a, T: Ord>();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/constraints-before-generic-args-syntactic-pass/constraints-before-generic-args-syntactic-pass.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/constraints-before-generic-args-syntactic-pass/constraints-before-generic-args-syntactic-pass.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/constraints-before-generic-args-syntactic-pass.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/constraints-before-generic-args-syntactic-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/constraints-before-generic-args-syntactic-pass" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/constraints-before-generic-args-syntactic-pass/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/constraints-before-generic-args-syntactic-pass.rs:5:19
   |
   |
LL |     foo::<T = u8, T: Ord, String>();
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
   = warning: unstable syntax can change at any point in the future, causing a hard error!

warning: associated type bounds are unstable
  --> /checkout/src/test/ui/parser/constraints-before-generic-args-syntactic-pass.rs:8:23
   |
   |
LL |     foo::<T = u8, 'a, T: Ord>();
   |
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = note: see issue #52662 <https://github.com/rust-lang/rust/issues/52662> for more information
   = help: add `#![feature(associated_type_bounds)]` to the crate attributes to enable
   = warning: unstable syntax can change at any point in the future, causing a hard error!

warning: 2 warnings emitted
------------------------------------------

