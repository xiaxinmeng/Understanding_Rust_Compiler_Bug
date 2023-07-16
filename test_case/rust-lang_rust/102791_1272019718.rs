plain

6    |                         |
7    |                         required by a bound introduced by this call
8    |
+    = note: required for `Option<&str>` to implement `PathLike`
9 note: required by a bound in `File::open`
10   --> $SRC_DIR/std/src/fs.rs:LL:COL


- LL |     pub fn open<P: AsRef<Path>>(path: P) -> io::Result<File> {
-    |                    ^^^^^^^^^^^ required by this bound in `File::open`
+ LL |     pub fn open<P: PathLike>(path: P) -> io::Result<File> {
+    |                    ^^^^^^^^ required by this bound in `File::open`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
15 error: aborting due to previous error
16 

---
To only update this specific test, also pass `--test-args async-await/issue-72442.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-72442.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72442/issue-72442.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72442" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-72442/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Option<&str>: AsRef<Path>` is not satisfied
   |
LL |             let mut f = File::open(path.to_str())?;
LL |             let mut f = File::open(path.to_str())?;
   |                         ---------- ^^^^^^^^^^^^^ the trait `AsRef<Path>` is not implemented for `Option<&str>`
   |                         required by a bound introduced by this call
   |
   |
   = note: required for `Option<&str>` to implement `PathLike`
note: required by a bound in `File::open`
   |
   |
LL |     pub fn open<P: PathLike>(path: P) -> io::Result<File> {
   |                    ^^^^^^^^ required by this bound in `File::open`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
