plain
........................................................................................ 6600/13721
.......................................................................i................ 6688/13721
........................................................................................ 6776/13721
............................................i........................................... 6864/13721
............Fii.iiF.......i....i........................................................ 6952/13721
.............................................................................i....i..... 7128/13721
....................................i..................i.............i.................. 7216/13721
...........................................i............................................ 7304/13721
................................................................i....................... 7392/13721
---
diff of stderr:

22    |                ^ required by this bound in `collect`
23 
24 error[E0277]: a slice of type `[i32]` cannot be built since `[i32]` has no definite size
-   --> $DIR/collect-into-slice.rs:8:30
26    |
26    |
27 LL |     let some_generated_vec = (0..10).collect();
-    |                              |
-    |                              |
-    |                              try explicitly collecting into a `Vec<{integer}>`
+    |                               ^^^^^  ------- required by a bound introduced by this call
+    |                               |
+    |                               try explicitly collecting into a `Vec<{integer}>`
32    = help: the trait `FromIterator<{integer}>` is not implemented for `[i32]`
33 note: required by a bound in `collect`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/collect-into-slice/collect-into-slice.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args iterators/collect-into-slice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/collect-into-slice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/collect-into-slice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/collect-into-slice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
   |
   |
LL |     let some_generated_vec = (0..10).collect();
   |
   = help: the trait `Sized` is not implemented for `[i32]`
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature
   = help: unsized locals are gated as an unstable feature

error[E0277]: the size for values of type `[i32]` cannot be known at compilation time
   |
   |
LL |     let some_generated_vec = (0..10).collect();
   |
   = help: the trait `Sized` is not implemented for `[i32]`
note: required by a bound in `collect`
  --> /checkout/library/core/src/iter/traits/iterator.rs:1832:16
  --> /checkout/library/core/src/iter/traits/iterator.rs:1832:16
   |
LL |     fn collect<B: FromIterator<Self::Item>>(self) -> B


error[E0277]: a slice of type `[i32]` cannot be built since `[i32]` has no definite size
   |
   |
LL |     let some_generated_vec = (0..10).collect();
   |                               ^^^^^  ------- required by a bound introduced by this call
   |                               |
   |                               try explicitly collecting into a `Vec<{integer}>`
   = help: the trait `FromIterator<{integer}>` is not implemented for `[i32]`
note: required by a bound in `collect`
  --> /checkout/library/core/src/iter/traits/iterator.rs:1832:19
   |
   |
LL |     fn collect<B: FromIterator<Self::Item>>(self) -> B

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
For more information about this error, try `rustc --explain E0277`.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------


---- [ui] src/test/ui/iterators/collect-into-array.rs stdout ----
diff of stderr:

1 error[E0277]: an array of type `[u32; 10]` cannot be built directly from an iterator
-   --> $DIR/collect-into-array.rs:3:31
3    |
3    |
4 LL |     let whatever: [u32; 10] = (0..10).collect();
-    |                               |
-    |                               |
-    |                               try collecting into a `Vec<{integer}>`, then using `.try_into()`
+    |                                ^^^^^  ------- required by a bound introduced by this call
+    |                                |
+    |                                try collecting into a `Vec<{integer}>`, then using `.try_into()`
9    = help: the trait `FromIterator<{integer}>` is not implemented for `[u32; 10]`
10 note: required by a bound in `collect`



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/collect-into-array/collect-into-array.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args iterators/collect-into-array.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/collect-into-array.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/collect-into-array" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/collect-into-array/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: an array of type `[u32; 10]` cannot be built directly from an iterator
   |
   |
LL |     let whatever: [u32; 10] = (0..10).collect();
   |                                ^^^^^  ------- required by a bound introduced by this call
   |                                |
   |                                try collecting into a `Vec<{integer}>`, then using `.try_into()`
   = help: the trait `FromIterator<{integer}>` is not implemented for `[u32; 10]`
note: required by a bound in `collect`
  --> /checkout/library/core/src/iter/traits/iterator.rs:1832:19
   |
   |
LL |     fn collect<B: FromIterator<Self::Item>>(self) -> B

error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
