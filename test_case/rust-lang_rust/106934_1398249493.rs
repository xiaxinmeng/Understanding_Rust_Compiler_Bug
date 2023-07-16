plain
........................................................................................ 8800/14213
........................................................................................ 8888/14213
..............................................................................i..ii..... 8976/14213
.........................................................ii............................. 9064/14213
............................................................F..F........iiii............ 9152/14213
..........................i........................................i.................... 9328/14213
................................................i....................................... 9416/14213
........................................................................................ 9504/14213
........................................................................i............... 9592/14213
---
......................iii............................................................... 14168/14213
.............................................
failures:

---- [ui] tests/ui/offset-of/offset-of-private.rs stdout ----

1 error[E0616]: field `private` of struct `Foo` is private
-   --> $DIR/offset_of_private.rs:15:24
+   --> $DIR/offset-of-private.rs:15:24
+   --> $DIR/offset-of-private.rs:15:24
3    |
4 LL |     offset_of!(m::Foo, private);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-private/offset-of-private.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-private/offset-of-private.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args offset-of/offset-of-private.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/offset-of/offset-of-private.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-private" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-private/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error[E0616]: field `private` of struct `Foo` is private
  --> /checkout/tests/ui/offset-of/offset-of-private.rs:15:24
   |
LL |     offset_of!(m::Foo, private); //~ ERROR field `private` of struct `Foo` is private

error: aborting due to previous error

For more information about this error, try `rustc --explain E0616`.
For more information about this error, try `rustc --explain E0616`.
------------------------------------------


---- [ui] tests/ui/offset-of/offset-of-dst-field.rs stdout ----

1 error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
-   --> $DIR/offset_of_dst_field.rs:13:5
+   --> $DIR/offset-of-dst-field.rs:13:5
+   --> $DIR/offset-of-dst-field.rs:13:5
3    |
4 LL |     offset_of!(Foo, slice);


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-dst-field/offset-of-dst-field.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args offset-of/offset-of-dst-field.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/offset-of/offset-of-dst-field.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-dst-field" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/offset-of/offset-of-dst-field/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the size for values of type `[u8]` cannot be known at compilation time
  --> /checkout/tests/ui/offset-of/offset-of-dst-field.rs:13:5
   |
LL |     offset_of!(Foo, slice); //~ ERROR the size for values of type
   |
   = help: the trait `Sized` is not implemented for `[u8]`

error: aborting due to previous error
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------



failures:
    [ui] tests/ui/offset-of/offset-of-dst-field.rs
    [ui] tests/ui/offset-of/offset-of-private.rs
test result: FAILED. 14075 passed; 2 failed; 136 ignored; 0 measured; 0 filtered out; finished in 113.10s

Build completed unsuccessfully in 0:11:05
