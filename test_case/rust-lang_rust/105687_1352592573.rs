plain
---- [ui] src/test/ui/typeck/issue-92481.rs stdout ----
diff of stderr:

32    |
33 LL |         d..||_=m
34    |         ^ field does not exist
+   --> $SRC_DIR/core/src/result.rs:LL:COL
-   ::: $SRC_DIR/core/src/result.rs:LL:COL
-    |
- LL |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
- LL |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
-    |     -- `Result<_, _>::Ok` defined here
+    = note: `Result<_, _>::Ok` defined here
40    |
41 help: `Result<_, _>::Ok` is a tuple variant, use the appropriate syntax


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-92481/issue-92481.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-92481/issue-92481.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-92481.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-92481.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-92481" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-92481/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected parameter name, found `{`
   |
   |
LL | fn r({) {
   |      ^ expected parameter name

error: expected one of `,`, `:`, or `}`, found `..`
   |
   |
LL | fn r({) {
   |      ^ unclosed delimiter
LL |     Ok {             //~ ERROR mismatched types [E0308]
LL |         d..||_=m
   |          -^
   |          |
   |          help: `}` may belong here
error[E0425]: cannot find value `d` in this scope
  --> /checkout/src/test/ui/typeck/issue-92481.rs:7:9
   |
   |
LL |         d..||_=m
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


error[E0425]: cannot find value `m` in this scope
   |
   |
LL |         d..||_=m


error[E0559]: variant `Result<_, _>::Ok` has no field named `d`
   |
   |
LL |         d..||_=m
   |         ^ field does not exist
  --> /rustc/FAKE_PREFIX/library/core/src/result.rs:508:5
   |
   = note: `Result<_, _>::Ok` defined here
   |
help: `Result<_, _>::Ok` is a tuple variant, use the appropriate syntax
   |
LL |     Result<_, _>::Ok(/* fields */)

error[E0308]: mismatched types
  --> /checkout/src/test/ui/typeck/issue-92481.rs:6:5
   |
   |
LL |   fn r({) {
   |           - help: a return type might be missing here: `-> _`
LL | /     Ok {             //~ ERROR mismatched types [E0308]
LL | |         d..||_=m
LL | |     }
   |
   = note: expected unit type `()`
                   found enum `Result<_, _>`

