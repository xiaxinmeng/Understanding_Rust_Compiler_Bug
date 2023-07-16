plain
diff of stderr:

5    |          ^ expected `:`
6    |
7    = note: this error originates in the derive macro `A` (in Nightly builds, run with -Z macro-backtrace for more info)
+ help: consider using `enum` instead of `struct`
+    |
+ LL | #[derive(enum)]
8 
9 error: proc-macro derive produced unparseable tokens
10   --> $DIR/derive-bad.rs:6:10

---
To only update this specific test, also pass `--test-args proc-macro/derive-bad.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/derive-bad.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-bad" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-bad/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected `:`, found `}`
   |
LL | #[derive(A)]
   |          ^ expected `:`
   |
   |
   = note: this error originates in the derive macro `A` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider using `enum` instead of `struct`
   |
LL | #[derive(enum)]

error: proc-macro derive produced unparseable tokens
  --> /checkout/src/test/ui/proc-macro/derive-bad.rs:6:10
   |
   |
LL | #[derive(A)]
   |          ^

error[E0428]: the name `A` is defined multiple times
   |
LL | #[derive(A)]
LL | #[derive(A)]
   |          - previous definition of the type `A` here
...
LL | struct A; //~ ERROR the name `A` is defined multiple times
   | ^^^^^^^^^ `A` redefined here
   |
   = note: `A` must be defined only once in the type namespace of this module
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0428`.
------------------------------------------
