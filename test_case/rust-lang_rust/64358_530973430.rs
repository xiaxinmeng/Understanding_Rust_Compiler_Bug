
---- [ui] ui/traits/cycle-cache-err-60010.rs stdout ----
diff of stderr:

6          |
7          = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
8
-       error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
-         --> $DIR/cycle-cache-err-60010.rs:30:6
-          |
-       LL | impl Database for RootDatabase {
-          |      ^^^^^^^^
-          |
-          = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`
-          = note: required because it appears within the type `SalsaStorage`
-
-       error: aborting due to 2 previous errors
+       error: aborting due to previous error
19
20      For more information about this error, try `rustc --explain E0275`.
21


The actual stderr differed from the expected stderr.
Actual stderr saved to /home/jistone/rust/rust/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/cycle-cache-err-60010.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/cycle-cache-err-60010.rs`

error: 1 errors occurred comparing output.
status: exit code: 1
command: "/home/jistone/rust/rust/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/home/jistone/rust/rust/src/test/ui/traits/cycle-cache-err-60010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/home/jistone/rust/rust/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/home/jistone/rust/rust/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/home/jistone/rust/rust/build/x86_64-unknown-linux-gnu/test/ui/traits/cycle-cache-err-60010/auxiliary" "-A" "unused"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
error[E0275]: overflow evaluating the requirement `RootDatabase: SourceDatabase`
  --> /home/jistone/rust/rust/src/test/ui/traits/cycle-cache-err-60010.rs:27:5
   |
LL |     _parse: <ParseQuery as Query<RootDatabase>>::Data, //~ ERROR overflow
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: required because of the requirements on the impl of `Query<RootDatabase>` for `ParseQuery`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.

------------------------------------------
