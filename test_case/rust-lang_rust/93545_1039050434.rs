plain
..................................iii............................................................... 12600/12639
.......................................
failures:

---- [ui] ui/rfc-3086-metavar-expr/out-of-bounds-arguments.rs stdout ----

5    |              ^^^^^^^^^^^^^^^^
6 
6 
7 error: index depth can not be less than or equal to 3
-   --> $DIR/out-of-bounds-arguments.rs:20:18
9    |
9    |
10 LL |                 ${index(10)},
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

12 
12 
13 error: length depth can not be less than or equal to 2
-   --> $DIR/out-of-bounds-arguments.rs:33:18
15    |
15    |
16 LL |                 ${length(10)}


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-3086-metavar-expr/out-of-bounds-arguments/out-of-bounds-arguments.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-3086-metavar-expr/out-of-bounds-arguments.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-3086-metavar-expr/out-of-bounds-arguments.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-3086-metavar-expr/out-of-bounds-arguments" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-3086-metavar-expr/out-of-bounds-arguments/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: count depth can not be less than or equal to 4
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/out-of-bounds-arguments.rs:7:14
   |
LL |             ${count(foo, 10)},


error: index depth can not be less than or equal to 3
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/out-of-bounds-arguments.rs:19:18
   |
LL |                 ${index(10)},


error: length depth can not be less than or equal to 2
  --> /checkout/src/test/ui/rfc-3086-metavar-expr/out-of-bounds-arguments.rs:32:18
   |
LL |                 ${length(10)}

error: aborting due to 3 previous errors



------------------------------------------



failures:
    [ui] ui/rfc-3086-metavar-expr/out-of-bounds-arguments.rs
test result: FAILED. 12510 passed; 1 failed; 128 ignored; 0 measured; 0 filtered out; finished in 97.48s

Build completed unsuccessfully in 0:10:15
