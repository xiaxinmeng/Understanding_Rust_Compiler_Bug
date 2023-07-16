plain
........................................................................................ 3344/14093
........................................................................................ 3432/14093
...............................................................iiiii.................... 3520/14093
........................................................................................ 3608/14093
...............F..F....F....FF.F..FF..F................................................. 3696/14093
........................................................................................ 3872/14093
........................................................................................ 3960/14093
........................................................................................ 4048/14093
...................................i..........i..........i.............................. 4136/14093
---

---- [ui] src/test/ui/dyn-star/align.rs#normal stdout ----
diff of stderr:

4 LL | #![feature(dyn_star)]
6    |
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
8    = note: `#[warn(incomplete_features)]` on by default
8    = note: `#[warn(incomplete_features)]` on by default
9 
10 warning: 1 warning emitted

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/align.normal/align.normal.stderr
To only update this specific test, also pass `--test-args dyn-star/align.rs`


error in revision `normal`: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/align.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "normal" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/align.normal" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/align.normal/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(dyn_star)]
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: `#[warn(incomplete_features)]` on by default


warning: 1 warning emitted
------------------------------------------


---- [ui] src/test/ui/dyn-star/align.rs#over_aligned stdout ----
diff of stderr:

4 LL | #![feature(dyn_star)]
6    |
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
8    = note: `#[warn(incomplete_features)]` on by default
8    = note: `#[warn(incomplete_features)]` on by default
9 
10 error[E0277]: `AlignedUsize` needs to be a pointer-sized type

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/align.over_aligned/align.over_aligned.stderr
To only update this specific test, also pass `--test-args dyn-star/align.rs`


error in revision `over_aligned`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/align.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "over_aligned" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/align.over_aligned" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/align.over_aligned/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(dyn_star)]
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: `#[warn(incomplete_features)]` on by default


error[E0277]: `AlignedUsize` needs to be a pointer-sized type
   |
   |
LL |     let x = AlignedUsize(12) as dyn* Debug;
   |             ^^^^^^^^^^^^^^^^ `AlignedUsize` needs to be a pointer-sized type
   |
   = help: the trait `PointerSized` is not implemented for `AlignedUsize`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/dyn-star/feature-gate-dyn_star.rs stdout ----
diff of stderr:

4 LL | pub fn dyn_star_parameter(_: &dyn* Send) {
6    |
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
8    = help: add `#![feature(dyn_star)]` to the crate attributes to enable
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/feature-gate-dyn_star/feature-gate-dyn_star.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dyn-star/feature-gate-dyn_star.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/feature-gate-dyn_star.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/feature-gate-dyn_star" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/feature-gate-dyn_star/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: dyn* trait objects are unstable
   |
   |
LL | pub fn dyn_star_parameter(_: &dyn* Send) {
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = help: add `#![feature(dyn_star)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/dyn-star/no-explicit-dyn-star-cast.rs stdout ----
diff of stderr:

4 LL |     let dyn_i: dyn* Debug = i as dyn* Debug;
6    |
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
8    = help: add `#![feature(dyn_star)]` to the crate attributes to enable
9 
10 error[E0658]: dyn* trait objects are unstable

13 LL |     let dyn_i: dyn* Debug = i as dyn* Debug;
15    |
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
17    = help: add `#![feature(dyn_star)]` to the crate attributes to enable
18 
19 error[E0606]: casting `usize` as `dyn* Debug` is invalid

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/no-explicit-dyn-star-cast/no-explicit-dyn-star-cast.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dyn-star/no-explicit-dyn-star-cast.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/no-explicit-dyn-star-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/no-explicit-dyn-star-cast" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/no-explicit-dyn-star-cast/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: dyn* trait objects are unstable
   |
   |
LL |     let dyn_i: dyn* Debug = i as dyn* Debug;
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = help: add `#![feature(dyn_star)]` to the crate attributes to enable

error[E0658]: dyn* trait objects are unstable
   |
   |
LL |     let dyn_i: dyn* Debug = i as dyn* Debug;
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = help: add `#![feature(dyn_star)]` to the crate attributes to enable

error[E0606]: casting `usize` as `dyn* Debug` is invalid
   |
   |
LL |     let dyn_i: dyn* Debug = i as dyn* Debug;

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0606, E0658.
Some errors have detailed explanations: E0606, E0658.
For more information about an error, try `rustc --explain E0606`.
------------------------------------------


---- [ui] src/test/ui/dyn-star/return.rs stdout ----
diff of stderr:

4 LL | #![feature(dyn_star)]
6    |
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
8    = note: `#[warn(incomplete_features)]` on by default
---
To only update this specific test, also pass `--test-args dyn-star/return.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/return.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/return" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/return/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(dyn_star)]
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: `#[warn(incomplete_features)]` on by default


warning: 1 warning emitted
------------------------------------------


---- [ui] src/test/ui/dyn-star/dont-unsize-coerce-dyn-star.rs stdout ----
diff of stderr:

4 LL | #![feature(dyn_star)]
6    |
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
8    = note: `#[warn(incomplete_features)]` on by default
---
To only update this specific test, also pass `--test-args dyn-star/dont-unsize-coerce-dyn-star.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/dont-unsize-coerce-dyn-star.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dont-unsize-coerce-dyn-star/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dont-unsize-coerce-dyn-star/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(dyn_star)]
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: `#[warn(incomplete_features)]` on by default


warning: 1 warning emitted
------------------------------------------


---- [ui] src/test/ui/dyn-star/no-unsize-coerce-dyn-trait.rs stdout ----
diff of stderr:

4 LL | #![feature(dyn_star, trait_upcasting)]
6    |
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
8    = note: `#[warn(incomplete_features)]` on by default
---
To only update this specific test, also pass `--test-args dyn-star/no-unsize-coerce-dyn-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/no-unsize-coerce-dyn-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/no-unsize-coerce-dyn-trait" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/no-unsize-coerce-dyn-trait/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(dyn_star, trait_upcasting)]
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: `#[warn(incomplete_features)]` on by default


error[E0308]: mismatched types
  --> /checkout/src/test/ui/dyn-star/no-unsize-coerce-dyn-trait.rs:11:26
   |
LL |     let y: Box<dyn* B> = x;
   |            -----------   ^ expected trait `B`, found trait `A`
   |            expected due to this
   |
   |
   = note: expected struct `Box<dyn* B>`
              found struct `Box<dyn* A>`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/dyn-star/dispatch-on-pin-mut.rs stdout ----
diff of stderr:

4 LL | #![feature(dyn_star)]
6    |
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
8    = note: `#[warn(incomplete_features)]` on by default
---
To only update this specific test, also pass `--test-args dyn-star/dispatch-on-pin-mut.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/dispatch-on-pin-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dispatch-on-pin-mut/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/dispatch-on-pin-mut/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
warning: the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(dyn_star)]
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: `#[warn(incomplete_features)]` on by default


warning: 1 warning emitted
------------------------------------------


---- [ui] src/test/ui/dyn-star/upcast.rs stdout ----
diff of stderr:

4 LL | #![feature(dyn_star, trait_upcasting)]
6    |
-    = note: see issue #91611 <https://github.com/rust-lang/rust/issues/91611> for more information
+    = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
8    = note: `#[warn(incomplete_features)]` on by default
8    = note: `#[warn(incomplete_features)]` on by default
9 
10 error[E0277]: `dyn* Foo` needs to be a pointer-sized type

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/upcast/upcast.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args dyn-star/upcast.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-star/upcast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/upcast" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-star/upcast/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `dyn_star` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![feature(dyn_star, trait_upcasting)]
   |
   = note: see issue #102425 <https://github.com/rust-lang/rust/issues/102425> for more information
   = note: `#[warn(incomplete_features)]` on by default


error[E0277]: `dyn* Foo` needs to be a pointer-sized type
   |
   |
LL |     let w: dyn* Bar = w;
   |                       ^ `dyn* Foo` needs to be a pointer-sized type
   |
   = help: the trait `PointerSized` is not implemented for `dyn* Foo`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
