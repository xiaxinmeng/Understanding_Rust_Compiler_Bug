plain
######################################################################## 100.0%
extracting /checkout/obj/build/cache/2022-04-05/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
Building rustbuild
    Updating crates.io index
    Updating git repository `https://github.com/Zoxc/rustc-hash/`
---
   Compiling tinystr v0.3.4
   Compiling remove_dir_all v0.5.3
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
   Compiling cpufeatures v0.2.1
   Compiling rustc-hash v1.0.1 (https://github.com/Zoxc/rustc-hash/?branch=new-hash#a5a1e17e)
   Compiling unic-common v0.9.0
   Compiling unic-char-range v0.9.0
   Compiling scoped-tls v1.0.0
   Compiling self_cell v0.10.2
---
   Compiling tinystr v0.3.4
   Compiling remove_dir_all v0.5.3
   Compiling arrayvec v0.7.0
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
   Compiling rustc-hash v1.0.1 (https://github.com/Zoxc/rustc-hash/?branch=new-hash#a5a1e17e)
   Compiling unicode-width v0.1.8
   Compiling unic-char-range v0.9.0
   Compiling scoped-tls v1.0.0
   Compiling unic-common v0.9.0
---

---- [ui] src/test/ui/generic-associated-types/self-outlives-lint.rs stdout ----
diff of stderr:

108    = note: this bound is currently required to ensure that impls have maximum flexibility
109    = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
- error: missing required bound on `Item`
-   --> $DIR/self-outlives-lint.rs:142:5
-    |
-    |
- LL |     type Item<'a>;
-    |                  |
-    |                  |
-    |                  help: add the required where clause: `where Self: 'a`
-    |
-    = note: this bound is currently required to ensure that impls have maximum flexibility
-    = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
122 error: missing required bound on `Iterator`
123   --> $DIR/self-outlives-lint.rs:144:5
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
124    |
124    |

126    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^-
127    |                                                       |
128    |                                                       help: add the required where clause: `where Self: 'a`
+    |
+    = note: this bound is currently required to ensure that impls have maximum flexibility
+    = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
+ error: missing required bound on `Item`
+   --> $DIR/self-outlives-lint.rs:142:5
+    |
+    |
+ LL |     type Item<'a>;
+    |                  |
+    |                  |
+    |                  help: add the required where clause: `where Self: 'a`
129    |
130    = note: this bound is currently required to ensure that impls have maximum flexibility
131    = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/self-outlives-lint/self-outlives-lint.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/self-outlives-lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/self-outlives-lint" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/self-outlives-lint/auxiliary"
stdout: none
--- stderr -------------------------------
error: missing required bound on `Item`
   |
   |
LL |     type Item<'x>;
   |                  |
   |                  |
   |                  help: add the required where clause: `where Self: 'x`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Out`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:25:5
   |
   |
LL |     type Out<'x>;
   |                 |
   |                 |
   |                 help: add the required where clause: `where T: 'x`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Out`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:39:5
   |
   |
LL |     type Out<'x>;
   |                 |
   |                 |
   |                 help: add the required where clause: `where T: 'x`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bounds on `Out`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:46:5
   |
   |
LL |     type Out<'x, 'y>;
   |                     |
   |                     |
   |                     help: add the required where clauses: `where T: 'x, U: 'y`
   |
   = note: these bounds are currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Out`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:61:5
   |
   |
LL |     type Out<'x, D>;
   |                    |
   |                    |
   |                    help: add the required where clause: `where D: 'x`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Out`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:77:5
   |
   |
LL |     type Out<'x, D>;
   |                    |
   |                    |
   |                    help: add the required where clause: `where D: 'x`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Out`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:92:5
   |
   |
LL |     type Out<'x, D>;
   |                    |
   |                    |
   |                    help: add the required where clause: `where D: 'x`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bounds on `Bar`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:114:5
   |
   |
LL |     type Bar<'b>;
   |                 |
   |                 |
   |                 help: add the required where clauses: `where Self: 'a, Self: 'b`
   |
   = note: these bounds are currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Bar`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:122:5
   |
   |
LL |     type Bar<'b>;
   |                 |
   |                 |
   |                 help: add the required where clause: `where Self: 'b`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Bar`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:129:5
   |
   |
LL |     type Bar<'b>;
   |                 |
   |                 |
   |                 help: add the required where clause: `where Self: 'b`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Iterator`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:144:5
   |
   |
LL |     type Iterator<'a>: Iterator<Item = Self::Item<'a>>;
   |                                                       |
   |                                                       |
   |                                                       help: add the required where clause: `where Self: 'a`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Item`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:142:5
   |
   |
LL |     type Item<'a>;
   |                  |
   |                  |
   |                  help: add the required where clause: `where Self: 'a`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Item`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:150:5
   |
   |
LL |     type Item<'a>;
   |                  |
   |                  |
   |                  help: add the required where clause: `where Self: 'a`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Bar`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:159:5
   |
   |
LL |     type Bar<'a, 'b>;
   |                     |
   |                     |
   |                     help: add the required where clause: `where 'b: 'a`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: missing required bound on `Fut`
  --> /checkout/src/test/ui/generic-associated-types/self-outlives-lint.rs:175:5
   |
   |
LL |     type Fut<'out>;
   |                   |
   |                   |
   |                   help: add the required where clause: `where 'ctx: 'out`
   |
   = note: this bound is currently required to ensure that impls have maximum flexibility
   = note: we are soliciting feedback, see issue #87479 <https://github.com/rust-lang/rust/issues/87479> for more information
error: aborting due to 15 previous errors
------------------------------------------


