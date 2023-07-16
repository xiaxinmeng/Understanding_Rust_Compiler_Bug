plain
....i.............i................................................................i.... 7392/13973
........................................................................................ 7480/13973
.................i...................................................................... 7568/13973
........................................................................................ 7656/13973
..............................................F....F.................................... 7744/13973
..........................................i............................................. 7920/13973
........................................................................................ 8008/13973
.........................................................ii............................. 8096/13973
........................................................................................ 8184/13973
---
---- [ui] src/test/ui/closures/closure-no-fn-4.rs stdout ----
diff of stderr:

18    |
19 LL |         false => |a| a - b,
20    |                          ^ `b` captured here
+    = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
22 error: aborting due to previous error
23 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-no-fn-4/closure-no-fn-4.stderr
To only update this specific test, also pass `--test-args closures/closure-no-fn-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-no-fn-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-no-fn-4" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-no-fn-4/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/closures/closure-no-fn-4.rs:5:18
   |
   |
LL |       let _: fn(usize) -> usize = match true {
   |  _________________________________-
LL | |         true => |a| a + 1,
   | |                 --------- this is found to be of type `fn(usize) -> usize`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL | |         false => |a| a - b,
   | |                  ^^^^^^^^^ expected fn pointer, found closure
LL | |         //~^ ERROR `match` arms have incompatible types
LL | |     };
   |
   = note: expected fn pointer `fn(usize) -> usize`
   = note: expected fn pointer `fn(usize) -> usize`
                 found closure `[closure@/checkout/src/test/ui/closures/closure-no-fn-4.rs:5:18: 5:21]`
note: closures can only be coerced to `fn` types if they do not capture any variables
   |
   |
LL |         false => |a| a - b,
   |                          ^ `b` captured here
   = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs#leak stdout ----
diff of stderr:

14    |
15    = note: expected fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
16               found fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
+    = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
18 error: aborting due to previous error
19 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1.leak/old-lub-glb-hr-noteq1.leak.stderr
To only update this specific test, also pass `--test-args lub-glb/old-lub-glb-hr-noteq1.rs`


error in revision `leak`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "leak" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1.leak" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq1.leak/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq1.rs:14:14
   |
   |
LL |       let z = match 22 {
LL | |         0 => x,
LL | |         0 => x,
   | |              - this is found to be of type `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
LL | |         _ => y,
   | |              ^ one type is more general than the other
LL | |         //[leak]~^ ERROR `match` arms have incompatible types
LL | |         //[noleak]~^^ ERROR mismatched types
LL | |     };
   |
   |
   = note: expected fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
              found fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
   = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs#leak stdout ----
diff of stderr:

13    |
14    = note: expected fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
15               found fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
+    = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
17 error: aborting due to previous error
18 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq2.leak/old-lub-glb-hr-noteq2.leak.stderr
To only update this specific test, also pass `--test-args lub-glb/old-lub-glb-hr-noteq2.rs`


error in revision `leak`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "leak" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq2.leak" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lub-glb/old-lub-glb-hr-noteq2.leak/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lub-glb/old-lub-glb-hr-noteq2.rs:25:14
   |
   |
LL |       let z = match 22 {
LL | |         0 => y,
LL | |         0 => y,
   | |              - this is found to be of type `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
LL | |         _ => x,
   | |              ^ one type is more general than the other
LL | |         //[leak]~^ ERROR `match` arms have incompatible types
LL | |     };
   |
   |
   = note: expected fn pointer `for<'a> fn(&'a u8, &'a u8) -> &'a u8`
              found fn pointer `for<'a, 'b> fn(&'a u8, &'b u8) -> &'a u8`
   = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/regions/issue-101280.rs stdout ----
diff of stderr:

8    |
9    = note: expected fn pointer `for<'r> fn(Cell<(&'r i32, &'r i32)>)`
10               found fn pointer `for<'a> fn(Cell<(&'r i32, &'a i32)>)`
+    = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-101280/issue-101280.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/issue-101280.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/issue-101280.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-101280" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/issue-101280/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/regions/issue-101280.rs:6:5
   |
   |
LL | fn f<'r>(f: fn(Cell<(&'r i32, &i32)>)) -> Ty {
   |                                           -- expected `for<'r> fn(Cell<(&'r i32, &'r i32)>)` because of return type
LL |     f
   |     ^ one type is more general than the other
   |
   = note: expected fn pointer `for<'r> fn(Cell<(&'r i32, &'r i32)>)`
              found fn pointer `for<'a> fn(Cell<(&'r i32, &'a i32)>)`
   = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/reify-intrinsic.rs stdout ----
diff of stderr:

26    = note: different `fn` items always have unique types, even if their signatures are the same
27    = help: change the expected type to be function pointer `extern "rust-intrinsic" fn(bool) -> bool`
28    = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `likely as extern "rust-intrinsic" fn(bool) -> bool`
+    = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
30 error: aborting due to 3 previous errors
31 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/reify-intrinsic.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args reify-intrinsic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/reify-intrinsic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/reify-intrinsic/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: cannot coerce intrinsics to function pointers
   |
   |
LL |     let _: unsafe extern "rust-intrinsic" fn(isize) -> usize = std::mem::transmute;
   |            -------------------------------------------------   ^^^^^^^^^^^^^^^^^^^ cannot coerce intrinsics to function pointers
   |            expected due to this
   |
   |
   = note: expected fn pointer `unsafe extern "rust-intrinsic" fn(isize) -> usize`
                 found fn item `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}`

error[E0606]: casting `unsafe extern "rust-intrinsic" fn(_) -> _ {transmute::<_, _>}` as `unsafe extern "rust-intrinsic" fn(isize) -> usize` is invalid
   |
   |
LL |     let _ = std::mem::transmute as unsafe extern "rust-intrinsic" fn(isize) -> usize;

error[E0308]: cannot coerce intrinsics to function pointers
  --> /checkout/src/test/ui/reify-intrinsic.rs:18:9
   |
   |
LL |         std::intrinsics::unlikely,
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^ cannot coerce intrinsics to function pointers
   |
   = note: expected fn item `extern "rust-intrinsic" fn(_) -> _ {likely}`
              found fn item `extern "rust-intrinsic" fn(_) -> _ {unlikely}`
   = note: different `fn` items always have unique types, even if their signatures are the same
   = help: change the expected type to be function pointer `extern "rust-intrinsic" fn(bool) -> bool`
   = help: if the expected type is due to type inference, cast the expected `fn` to a function pointer: `likely as extern "rust-intrinsic" fn(bool) -> bool`
   = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0308, E0606.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/unsafe/unsafe-subtyping.rs stdout ----
diff of stderr:

8    |
9    = note: expected enum `Option<unsafe fn(_)>`
10               found enum `Option<fn(_)>`
+    = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-subtyping/unsafe-subtyping.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsafe/unsafe-subtyping.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/unsafe-subtyping.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-subtyping" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/unsafe-subtyping/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/unsafe/unsafe-subtyping.rs:4:5
   |
   |
LL | fn foo(x: Option<fn(i32)>) -> Option<unsafe fn(i32)> {
   |                               ---------------------- expected `Option<unsafe fn(i32)>` because of return type
LL |     x //~ ERROR mismatched types
   |     ^ expected unsafe fn, found normal fn
   |
   = note: expected enum `Option<unsafe fn(_)>`
              found enum `Option<fn(_)>`
   = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/unsized/box-instead-of-dyn-fn.rs stdout ----
diff of stderr:

16    |
17    = note: expected closure `[closure@$DIR/box-instead-of-dyn-fn.rs:8:9: 8:16]`
18                found struct `Box<[closure@$DIR/box-instead-of-dyn-fn.rs:10:18: 10:25]>`
+    = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
20 error[E0746]: return type cannot have an unboxed trait object
21   --> $DIR/box-instead-of-dyn-fn.rs:5:56



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/box-instead-of-dyn-fn/box-instead-of-dyn-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args unsized/box-instead-of-dyn-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsized/box-instead-of-dyn-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/box-instead-of-dyn-fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsized/box-instead-of-dyn-fn/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0308]: `if` and `else` have incompatible types
   |
   |
LL | /     if a % 2 == 0 {
LL | |         move || println!("{a}")
   | |         |
   | |         the expected closure
   | |         expected because of this
LL | |     } else {
LL | |     } else {
LL | |         Box::new(move || println!("{}", b))
   | |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected closure, found struct `Box`
LL | |         //~^ ERROR `if` and `else` have incompatible types
LL | |     }
   | |_____- `if` and `else` have incompatible types
   |
   = note: expected closure `[closure@/checkout/src/test/ui/unsized/box-instead-of-dyn-fn.rs:8:9: 8:16]`
               found struct `Box<[closure@/checkout/src/test/ui/unsized/box-instead-of-dyn-fn.rs:10:18: 10:25]>`
   = help: The item types of different functions - different items, or the same item with different generics - are distinct, and mixing them will create a type error. See https://doc.rust-lang.org/reference/types/function-item.html for details
error[E0746]: return type cannot have an unboxed trait object
  --> /checkout/src/test/ui/unsized/box-instead-of-dyn-fn.rs:5:56
   |
   |
LL | fn print_on_or_the_other<'a>(a: i32, b: &'a String) -> dyn Fn() + 'a {
   |
   |
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = note: if all the returned values were of the same type you could use `impl Fn() + 'a` as the return type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: you can create a new `enum` with a variant for each returned type
help: return a boxed trait object instead
   |
LL | fn print_on_or_the_other<'a>(a: i32, b: &'a String) -> Box<dyn Fn() + 'a> {
help: ... and box this value
   |
   |
LL |         Box::new(move || println!("{a}"))

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0746.
