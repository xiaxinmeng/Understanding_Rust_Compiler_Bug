plain
1 error[E0208]: [o]
-   --> $DIR/variance-object-types.rs:11:1
+   --> $DIR/variance-object-types.rs:8:1
3    |
4 LL | / struct Foo<'a> {
5 LL | |     x: Box<dyn Fn(i32) -> &'a i32 + 'static>

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-object-types/variance-object-types.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args variance/variance-object-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-object-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-object-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-object-types/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0208]: [o]
   |
   |
LL | / struct Foo<'a> { //~ ERROR [o]
LL | |     x: Box<dyn Fn(i32) -> &'a i32 + 'static>
LL | | }

error: aborting due to previous error
------------------------------------------

