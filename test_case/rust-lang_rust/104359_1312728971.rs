plain
......................................i....i.........................................i.. 7216/13831
................i..............i........................................................ 7304/13831
......i................................................................................. 7392/13831
...........................i............................................................ 7480/13831
...................................................F...............F.................... 7568/13831
...........................ii.......................................ii.................. 7744/13831
...............................................i........................................ 7832/13831
........................................................................................ 7920/13831
............................................................ii.......................... 8008/13831
---

34 LL |     impl_array();
35    |     ^^^^^^^^^^^^
36 
- error: unused array of arrays of arrays of `S` that must be used
+ error: unused array of array of arrays of `S` that must be used
39    |
39    |
40 LL |     array_of_arrays_of_arrays();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/must_use-array/must_use-array.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/unused/must_use-array.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/must_use-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/must_use-array" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/must_use-array/auxiliary"
stdout: none
--- stderr -------------------------------
error: unused array of `S` that must be used
   |
   |
LL |     singleton(); //~ ERROR unused array of `S` that must be used
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused/must_use-array.rs:1:9
   |
   |
LL | #![deny(unused_must_use)]
   |         ^^^^^^^^^^^^^^^

error: unused array of `S` that must be used
   |
   |
LL |     many(); //~ ERROR unused array of `S` that must be used


error: unused array of `S` in tuple element 0 that must be used
   |
   |
LL |     ([S], 0, ()); //~ ERROR unused array of `S` in tuple element 0 that must be used


error: unused array of implementers of `T` that must be used
   |
   |
LL |     array_of_impl_trait(); //~ ERROR unused array of implementers of `T` that must be used


error: unused array of boxed `T` trait objects in tuple element 1 that must be used
   |
LL |     impl_array();
   |     ^^^^^^^^^^^^


error: unused array of array of arrays of `S` that must be used
   |
   |
LL |     array_of_arrays_of_arrays();


error: unused array of `S` that must be used
   |
LL |     usize_max();
   |     ^^^^^^^^^^^

---
---- [ui] src/test/ui/lint/unused/unused-closure.rs stdout ----
diff of stderr:

37    |
38    = note: closures are lazy and do nothing unless called
- error: unused array of boxed arrays of closures that must be used
+ error: unused array of boxed array of closures that must be used
41   --> $DIR/unused-closure.rs:18:5
42    |
42    |
43 LL |     [Box::new([|| {}; 10]); 1];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-closure/unused-closure.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/unused/unused-closure.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused/unused-closure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-closure" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused/unused-closure/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:9:5
   |
   |
LL | /     || { //~ ERROR unused closure that must be used
LL | |         println!("Hello!");
LL | |     };
   |
   |
   = note: closures are lazy and do nothing unless called
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:6:9
   |
LL | #![deny(unused_must_use)]
   |         ^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^

error: unused implementer of `Future` that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:13:5
   |
LL |     async {};    //~ ERROR unused implementer of `Future` that must be used
   |
   |
   = note: futures do nothing unless you `.await` or poll them
error: unused closure that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:14:5
   |
   |
LL |     || async {}; //~ ERROR unused closure that must be used
   |
   |
   = note: closures are lazy and do nothing unless called
error: unused closure that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:15:5
   |
   |
LL |     async || {}; //~ ERROR unused closure that must be used
   |
   |
   = note: closures are lazy and do nothing unless called
error: unused array of boxed array of closures that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:18:5
   |
   |
LL |     [Box::new([|| {}; 10]); 1]; //~ ERROR unused array of boxed arrays of closures that must be used
   |
   |
   = note: closures are lazy and do nothing unless called
error: unused closure that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:20:5
   |
   |
LL |     vec![|| "a"].pop().unwrap(); //~ ERROR unused closure that must be used
   |
   |
   = note: closures are lazy and do nothing unless called
error: unused closure that must be used
  --> /checkout/src/test/ui/lint/unused/unused-closure.rs:23:9
   |
   |
LL |         || true; //~ ERROR unused closure that must be used
   |
   |
   = note: closures are lazy and do nothing unless called
error: aborting due to 7 previous errors
------------------------------------------


