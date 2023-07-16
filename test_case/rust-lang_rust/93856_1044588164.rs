plain
.......i............................................................................................ 6400/12649
........................................................................i........................... 6500/12649
........................ii.ii........i...i.......................................................... 6600/12649
.................................................................................................... 6700/12649
...........i....i..F.....................................i................i.............i........... 6800/12649
............................................i....................................................... 7000/12649
.................................................................................................... 7100/12649
..................................................................ii................................ 7200/12649
ii.........................................................i........................................ 7300/12649
---
............................................iii..................................................... 12600/12649
.................................................
failures:

---- [ui] ui/lifetimes/re-empty-in-error.rs stdout ----


1 error[E0477]: the type `&'b ()` does not fulfill the required lifetime
2   --> $DIR/re-empty-in-error.rs:8:5
3    |
- LL |     foo(&10); 
+ LL |     foo(&10);
6    |
7 note: type must outlive the empty lifetime as required by this binding



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error/re-empty-in-error.stderr
To only update this specific test, also pass `--test-args lifetimes/re-empty-in-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/re-empty-in-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/re-empty-in-error/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0477]: the type `&'b ()` does not fulfill the required lifetime
  --> /checkout/src/test/ui/lifetimes/re-empty-in-error.rs:8:5
LL |     foo(&10);
   |     ^^^
   |
note: type must outlive the empty lifetime as required by this binding
note: type must outlive the empty lifetime as required by this binding
  --> /checkout/src/test/ui/lifetimes/re-empty-in-error.rs:3:47
   |
LL | fn foo<'a>(_a: &'a u32) where for<'b> &'b (): 'a {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0477`.
