plain

---- [ui] src/test/ui/consts/issue-94675.rs stdout ----
diff of stderr:

- error[E0015]: cannot call non-const fn `Vec::<u32>::len` in constant functions
-   --> $DIR/issue-94675.rs:9:27
-    |
- LL |         self.bar[0] = baz.len();
-    |
-    |
-    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
- 
9 error[E0277]: the trait bound `Vec<usize>: ~const IndexMut<usize>` is not satisfied
11    |

32    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
32    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
33    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
- error: aborting due to 3 previous errors
+ error: aborting due to 2 previous errors
36 
37 Some errors have detailed explanations: E0015, E0277.
---
To only update this specific test, also pass `--test-args consts/issue-94675.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-94675.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-94675" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-94675/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `Vec<usize>: ~const IndexMut<usize>` is not satisfied
   |
   |
LL |         self.bar[0] = baz.len();
   |         ^^^^^^^^^^^ vector indices are of type `usize` or ranges of `usize`
   |
   = help: the trait `~const IndexMut<usize>` is not implemented for `Vec<usize>`
note: the trait `IndexMut<usize>` is implemented for `Vec<usize>`, but that implementation is not `const`
   |
   |
LL |         self.bar[0] = baz.len();

error[E0015]: cannot call non-const operator in constant functions
  --> /checkout/src/test/ui/consts/issue-94675.rs:9:9
   |
   |
LL |         self.bar[0] = baz.len();
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
note: impl defined here, but it is not `const`
   |
   |
LL | impl<T, I: SliceIndex<[T]>, A: Allocator> IndexMut<I> for Vec<T, A> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0015, E0277.
For more information about an error, try `rustc --explain E0015`.
