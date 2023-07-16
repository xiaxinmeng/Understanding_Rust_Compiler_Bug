plain
..........i............................................................................. 6424/13305
........................................................................................ 6512/13305
.............i.......................................................................... 6600/13305
...................................................................................i.... 6688/13305
..................................................ii.ii........i..Fi.................... 6776/13305
........................................................................................ 6952/13305
..................................i....i.........................................i...... 7040/13305
...........i.............i.......................................................i...... 7128/13305
........................................................................................ 7216/13305
---
139    = warning: this changes meaning in Rust 2021
140    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
141 
- warning: 12 warnings emitted
+ warning: unnecessary allocation, use `&` instead
+    |
+    |
+ LL |     Box::new(small).into_iter();
+    |
+    = note: `#[warn(unused_allocation)]` on by default
+ 
+ 
+ warning: unnecessary allocation, use `&` instead
+    |
+    |
+ LL |     Box::new([1, 2]).into_iter();
+ 
+ 
+ warning: unnecessary allocation, use `&` instead
+    |
+    |
+ LL |     Box::new(big).into_iter();
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ 
+ 
+ warning: unnecessary allocation, use `&` instead
+    |
+    |
+ LL |     Box::new([0u8; 33]).into_iter();
+ 
+ 
+ warning: unnecessary allocation, use `&` instead
+    |
+    |
+ LL |     Box::new(Box::new(small)).into_iter();
+ 
+ 
+ warning: unnecessary allocation, use `&` instead
+    |
+    |
+ LL |     Box::new(Box::new([1, 2])).into_iter();
+ 
+ 
+ warning: unnecessary allocation, use `&` instead
+    |
+    |
+ LL |     Box::new(Box::new(big)).into_iter();
+ 
+ 
+ warning: unnecessary allocation, use `&` instead
+    |
+    |
+ LL |     Box::new(Box::new([0u8; 33])).into_iter();
+ 
+ warning: 20 warnings emitted
143 
144 
---
To only update this specific test, also pass `--test-args iterators/into-iter-on-arrays-lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/auxiliary"
stdout: none
--- stderr -------------------------------
warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
LL |     small.into_iter();
   |           ^^^^^^^^^
   |
   |
   = note: `#[warn(array_into_iter)]` on by default
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
LL |     small.iter();
   |           ~~~~
   |           ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
LL |     IntoIterator::into_iter(small);
   |     ++++++++++++++++++++++++     ~


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     [1, 2].into_iter();
   |
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     [1, 2].iter();
   |            ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
LL |     IntoIterator::into_iter([1, 2]);
   |     ++++++++++++++++++++++++      ~


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
LL |     big.into_iter();
   |         ^^^^^^^^^
   |
   |
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
LL |     big.iter();
   |         ~~~~
   |         ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
LL |     IntoIterator::into_iter(big);
   |     ++++++++++++++++++++++++   ~


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     [0u8; 33].into_iter();
   |
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     [0u8; 33].iter();
   |               ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
   |
LL |     IntoIterator::into_iter([0u8; 33]);
   |     ++++++++++++++++++++++++         ~

warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(small).into_iter();
   |                     ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new([1, 2]).into_iter();
   |                      ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(big).into_iter();
   |                   ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new([0u8; 33]).into_iter();
   |                         ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new(small)).into_iter();
   |                               ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new([1, 2])).into_iter();
   |                                ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new(big)).into_iter();
   |                             ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new([0u8; 33])).into_iter();
   |                                   ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: unnecessary allocation, use `&` instead
   |
   |
LL |     Box::new(small).into_iter();
   |
   = note: `#[warn(unused_allocation)]` on by default


warning: unnecessary allocation, use `&` instead
   |
   |
LL |     Box::new([1, 2]).into_iter();


warning: unnecessary allocation, use `&` instead
   |
   |
LL |     Box::new(big).into_iter();


warning: unnecessary allocation, use `&` instead
   |
   |
LL |     Box::new([0u8; 33]).into_iter();


warning: unnecessary allocation, use `&` instead
   |
   |
LL |     Box::new(Box::new(small)).into_iter();


warning: unnecessary allocation, use `&` instead
   |
   |
LL |     Box::new(Box::new([1, 2])).into_iter();


warning: unnecessary allocation, use `&` instead
   |
   |
LL |     Box::new(Box::new(big)).into_iter();


warning: unnecessary allocation, use `&` instead
   |
   |
LL |     Box::new(Box::new([0u8; 33])).into_iter();

warning: 20 warnings emitted
------------------------------------------



---- [ui] src/test/ui/self/arbitrary_self_types_trait.rs stdout ----
normalized stderr:
warning: unnecessary allocation, use `&` instead
   |
   |
LL |     assert_eq!(&[1,2,3], Box::new(Rc::new(v)).trait_method());
   |
   = note: `#[warn(unused_allocation)]` on by default

warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args self/arbitrary_self_types_trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/arbitrary_self_types_trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_trait/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/arbitrary_self_types_trait/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary allocation, use `&` instead
   |
   |
LL |     assert_eq!(&[1,2,3], Box::new(Rc::new(v)).trait_method());
   |
   = note: `#[warn(unused_allocation)]` on by default

warning: 1 warning emitted
warning: 1 warning emitted
------------------------------------------


---- [ui] src/test/ui/structs-enums/align-struct.rs stdout ----
normalized stderr:
warning: unnecessary allocation, use `&` instead
   |
   |
LL |     assert_eq!(mem::align_of_val(Box::new(Align16(0)).as_ref()), 16);
   |
   = note: `#[warn(unused_allocation)]` on by default

warning: 1 warning emitted
---
To only update this specific test, also pass `--test-args structs-enums/align-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs-enums/align-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/align-struct/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/align-struct/auxiliary"
stdout: none
--- stderr -------------------------------
warning: unnecessary allocation, use `&` instead
   |
   |
LL |     assert_eq!(mem::align_of_val(Box::new(Align16(0)).as_ref()), 16);
   |
   = note: `#[warn(unused_allocation)]` on by default

warning: 1 warning emitted
