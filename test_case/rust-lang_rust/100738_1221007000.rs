plain
........................................................................................ 2112/13397
........................................................................................ 2200/13397
........................................................................................ 2288/13397
........................................................................................ 2376/13397
.........................................................F.....F........................ 2464/13397
........................................................................................ 2640/13397
............................................F........................................... 2728/13397
........................................................................................ 2816/13397
..........F............................................................................. 2904/13397
..........F............................................................................. 2904/13397
...............................F.....................................F.................. 2992/13397
.....FF...............i.....................................................i........... 3080/13397
........................................................................................ 3256/13397
....................................................iiiii............................... 3344/13397
........................................................................................ 3432/13397
........................................................................................ 3520/13397
---
........................................................................................ 12760/13397
........................................................................................ 12848/13397
........................................................................................ 12936/13397
........................................................................................ 13024/13397
...............................................................................F.F...... 13112/13397
........................................................................................ 13288/13397
..iii................................................................................... 13376/13397
.....................
failures:
failures:

---- [ui] src/test/ui/check-static-values-constraints.rs stdout ----
diff of stderr:

13   --> $DIR/check-static-values-constraints.rs:79:33
14    |
15 LL | static STATIC11: Box<MyOwned> = box MyOwned;
-    |                                 ^^^^^^^^^^^ allocation not allowed in statics
17 
17 
18 error[E0015]: cannot call non-const fn `<str as ToString>::to_string` in statics

27   --> $DIR/check-static-values-constraints.rs:94:5
28    |
29 LL |     box MyOwned,
---
39   --> $DIR/check-static-values-constraints.rs:99:6
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

40    |
41 LL |     &box MyOwned,
-    |      ^^^^^^^^^^^ allocation not allowed in statics
43 
44 error[E0010]: allocations are not allowed in statics
45   --> $DIR/check-static-values-constraints.rs:100:6


46    |
47 LL |     &box MyOwned,
-    |      ^^^^^^^^^^^ allocation not allowed in statics
49 
50 error[E0010]: allocations are not allowed in statics
51   --> $DIR/check-static-values-constraints.rs:106:5

---
57   --> $DIR/check-static-values-constraints.rs:110:45

66   --> $DIR/check-static-values-constraints.rs:110:38
67    |
68 LL |     let y = { static x: Box<isize> = box 3; x };
-    |                                      ^^^^^ allocation not allowed in statics
70 
71 error: aborting due to 10 previous errors
72 

---
To only update this specific test, also pass `--test-args check-static-values-constraints.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/check-static-values-constraints.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/check-static-values-constraints/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0493]: destructors cannot be evaluated at compile-time
   |
   |
LL |                                           ..SafeStruct{field1: SafeEnum::Variant3(WithDtor),
   |  ___________________________________________^
LL | | //~^ ERROR destructors cannot be evaluated at compile-time
LL | |                                                      field2: SafeEnum::Variant1}};
   | |                                                                                ^- value is dropped here
   |                                                                                  statics cannot evaluate destructors

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:79:33
  --> /checkout/src/test/ui/check-static-values-constraints.rs:79:33
   |
LL | static STATIC11: Box<MyOwned> = box MyOwned;


error[E0015]: cannot call non-const fn `<str as ToString>::to_string` in statics
   |
   |
LL |     field2: SafeEnum::Variant4("str".to_string())
   |
   = note: calls in statics are limited to constant functions, tuple structs and tuple variants

error[E0010]: allocations are not allowed in statics
error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:94:5
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:95:5
   |
   |
LL |     box MyOwned, //~ ERROR allocations are not allowed in statics

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:99:6
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:100:6
   |
   |
LL |     &box MyOwned, //~ ERROR allocations are not allowed in statics

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:106:5
   |
   |
LL |     box 3;
   |     ^^^^^

error[E0507]: cannot move out of static item `x`
  --> /checkout/src/test/ui/check-static-values-constraints.rs:110:45
   |
LL |     let y = { static x: Box<isize> = box 3; x };
   |                                             |
   |                                             |
   |                                             move occurs because `x` has type `Box<isize>`, which does not implement the `Copy` trait

error[E0010]: allocations are not allowed in statics
  --> /checkout/src/test/ui/check-static-values-constraints.rs:110:38
   |
   |
LL |     let y = { static x: Box<isize> = box 3; x };

error: aborting due to 10 previous errors

Some errors have detailed explanations: E0010, E0015, E0493, E0507.
Some errors have detailed explanations: E0010, E0015, E0493, E0507.
For more information about an error, try `rustc --explain E0010`.
------------------------------------------


---- [ui] src/test/ui/consts/async-block.rs#without_feature stdout ----
diff of stderr:

- error[E0658]: `async` blocks are not allowed in constants
+ error: `async` blocks are not allowed in constants
3    |
3    |
4 LL | const _: i32 = { core::mem::ManuallyDrop::new(async { 0 }); 4 };
7    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
8    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
9 
9 
- error[E0658]: `async` blocks are not allowed in statics
+ error: `async` blocks are not allowed in statics
12    |
12    |
13 LL | static _FUT: &(dyn Future<Output = ()> + Sync) = &async {};
18 
19 error: aborting due to 2 previous errors
20 
- For more information about this error, try `rustc --explain E0658`.
- For more information about this error, try `rustc --explain E0658`.
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature/async-block.without_feature.stderr
To only update this specific test, also pass `--test-args consts/async-block.rs`


error in revision `without_feature`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/async-block.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "without_feature" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature/auxiliary"
stdout: none
--- stderr -------------------------------
error: `async` blocks are not allowed in constants
   |
   |
LL | const _: i32 = { core::mem::ManuallyDrop::new(async { 0 }); 4 };
   |
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable


error: `async` blocks are not allowed in statics
   |
   |
LL | static _FUT: &(dyn Future<Output = ()> + Sync) = &async {};
   |
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable


error: aborting due to 2 previous errors
------------------------------------------


---- [ui] src/test/ui/consts/const-address-of-interior-mut.rs stdout ----
diff of stderr:

- error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
+ error: cannot borrow here, since the borrowed element may contain interior mutability
3    |
3    |
4 LL | const A: () = { let x = Cell::new(2); &raw const x; };
7    = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
8    = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable
9 
9 
- error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
+ error: cannot borrow here, since the borrowed element may contain interior mutability
12    |
12    |
13 LL | static B: () = { let x = Cell::new(2); &raw const x; };
16    = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
17    = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable
18 
18 
- error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
+ error: cannot borrow here, since the borrowed element may contain interior mutability
21    |
21    |
22 LL | static mut C: () = { let x = Cell::new(2); &raw const x; };
25    = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
26    = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable
27 
27 
- error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
+ error: cannot borrow here, since the borrowed element may contain interior mutability
30    |
30    |
31 LL |     let y = &raw const x;
36 
37 error: aborting due to 4 previous errors
38 
- For more information about this error, try `rustc --explain E0658`.
---
To only update this specific test, also pass `--test-args consts/const-address-of-interior-mut.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-address-of-interior-mut.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-address-of-interior-mut" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-address-of-interior-mut/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot borrow here, since the borrowed element may contain interior mutability
   |
   |
LL | const A: () = { let x = Cell::new(2); &raw const x; };      //~ ERROR interior mutability
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable


error: cannot borrow here, since the borrowed element may contain interior mutability
   |
   |
LL | static B: () = { let x = Cell::new(2); &raw const x; };     //~ ERROR interior mutability
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable


error: cannot borrow here, since the borrowed element may contain interior mutability
   |
   |
LL | static mut C: () = { let x = Cell::new(2); &raw const x; }; //~ ERROR interior mutability
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable


error: cannot borrow here, since the borrowed element may contain interior mutability
   |
   |
LL |     let y = &raw const x;                                   //~ ERROR interior mutability
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable

---

7    = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
8    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
9 
- error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
+ error: cannot borrow here, since the borrowed element may contain interior mutability
12    |
13 LL |     let p = &a;



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-multi-ref/const-multi-ref.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-multi-ref.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-multi-ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-multi-ref" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-multi-ref/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: mutable references are not allowed in constants
   |
   |
LL |     let p = &mut a; //~ ERROR mutable references are not allowed in constants
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error: cannot borrow here, since the borrowed element may contain interior mutability
   |
   |
LL |     let p = &a; //~ ERROR borrowed element may contain interior mutability
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable

---
diff of stderr:

2   --> $DIR/issue-17718-const-borrow.rs:4:39
3    |
4 LL | const B: &'static UnsafeCell<usize> = &A;
-    |                                       ^^ this borrow of an interior mutable value may end up in the final value
6 
7 error[E0492]: constants cannot refer to interior mutable data
8   --> $DIR/issue-17718-const-borrow.rs:9:39


9    |
10 LL | const E: &'static UnsafeCell<usize> = &D.a;
-    |                                       ^^^^ this borrow of an interior mutable value may end up in the final value
12 
13 error[E0492]: constants cannot refer to interior mutable data
14   --> $DIR/issue-17718-const-borrow.rs:11:23


15    |
16 LL | const F: &'static C = &D;
-    |                       ^^ this borrow of an interior mutable value may end up in the final value
18 
19 error: aborting due to 3 previous errors
20 

---
To only update this specific test, also pass `--test-args consts/issue-17718-const-borrow.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-17718-const-borrow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-17718-const-borrow" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-17718-const-borrow/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0492]: constants cannot refer to interior mutable data
   |
   |
LL | const B: &'static UnsafeCell<usize> = &A;

error[E0492]: constants cannot refer to interior mutable data
  --> /checkout/src/test/ui/consts/issue-17718-const-borrow.rs:9:39
   |
   |
LL | const E: &'static UnsafeCell<usize> = &D.a;

error[E0492]: constants cannot refer to interior mutable data
  --> /checkout/src/test/ui/consts/issue-17718-const-borrow.rs:11:23
   |
   |
LL | const F: &'static C = &D;

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0492`.
---
To only update this specific test, also pass `--test-args consts/min_const_fn/bad_const_fn_body_ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/bad_const_fn_body_ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/bad_const_fn_body_ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/bad_const_fn_body_ice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0010]: allocations are not allowed in constant functions
   |
LL |     vec![1, 2, 3]
   |     ^^^^^^^^^^^^^
   |
   |
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0015]: cannot call non-const fn `slice::<impl [i32]>::into_vec::<std::alloc::Global>` in constant functions
   |
LL |     vec![1, 2, 3]
   |     ^^^^^^^^^^^^^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0010, E0015.
For more information about an error, try `rustc --explain E0010`.
---
diff of stderr:

2   --> $DIR/partial_qualif.rs:6:5
3    |
4 LL |     &{a}
-    |     ^^^^ this borrow of an interior mutable value may end up in the final value
6 
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/partial_qualif/partial_qualif.stderr
To only update this specific test, also pass `--test-args consts/partial_qualif.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/partial_qualif.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/partial_qualif" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/partial_qualif/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0492]: constants cannot refer to interior mutable data
   |
   |
LL |     &{a} //~ ERROR cannot refer to interior mutable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0492`.
---
diff of stderr:

2   --> $DIR/qualif_overwrite.rs:10:5
3    |
4 LL |     &{a}
-    |     ^^^^ this borrow of an interior mutable value may end up in the final value
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args consts/qualif_overwrite.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/qualif_overwrite.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/qualif_overwrite" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/qualif_overwrite/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0492]: constants cannot refer to interior mutable data
   |
   |
LL |     &{a} //~ ERROR cannot refer to interior mutable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0492`.
---
diff of stderr:

2   --> $DIR/qualif_overwrite_2.rs:8:5
3    |
4 LL |     &{a.0}
-    |     ^^^^^^ this borrow of an interior mutable value may end up in the final value
6 
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/qualif_overwrite_2/qualif_overwrite_2.stderr
To only update this specific test, also pass `--test-args consts/qualif_overwrite_2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/qualif_overwrite_2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/qualif_overwrite_2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/qualif_overwrite_2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0492]: constants cannot refer to interior mutable data
   |
   |
LL |     &{a.0} //~ ERROR cannot refer to interior mutable

error: aborting due to previous error

For more information about this error, try `rustc --explain E0492`.
---
diff of stderr:

2   --> $DIR/E0010-teach.rs:6:24
3    |
4 LL | const CON : Box<i32> = box 0;
-    |                        ^^^^^ allocation not allowed in constants
6    |
6    |
-    = note: The value of statics and constants must be known at compile time, and they live for the entire lifetime of a program. Creating a boxed value allocates memory on the heap at runtime, and therefore cannot be done at compile time.
+    = note: The value of statics and constants must be known at compile time, and they live for the entire 
+            lifetime of a program. Creating a boxed value allocates memory on the heap at runtime, and 
+            therefore cannot be done at compile time.
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0010-teach/E0010-teach.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0010-teach.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0010-teach.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0010-teach" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "teach" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0010-teach/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0010]: allocations are not allowed in constants
   |
   |
LL | const CON : Box<i32> = box 0; //~ ERROR E0010
   |
   |
   = note: The value of statics and constants must be known at compile time, and they live for the entire 
           lifetime of a program. Creating a boxed value allocates memory on the heap at runtime, and 
           therefore cannot be done at compile time.
error: aborting due to previous error

For more information about this error, try `rustc --explain E0010`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/error-codes/E0010.rs stdout ----
diff of stderr:

2   --> $DIR/E0010.rs:4:24
3    |
4 LL | const CON : Box<i32> = box 0;
-    |                        ^^^^^ allocation not allowed in constants
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args error-codes/E0010.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0010.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0010" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0010/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0010]: allocations are not allowed in constants
   |
   |
LL | const CON : Box<i32> = box 0; //~ ERROR E0010

error: aborting due to previous error

For more information about this error, try `rustc --explain E0010`.
---
diff of stderr:

2   --> $DIR/E0492.rs:4:33
3    |
4 LL | const B: &'static AtomicUsize = &A;
-    |                                 ^^ this borrow of an interior mutable value may end up in the final value
6 
7 error[E0492]: statics cannot refer to interior mutable data
8   --> $DIR/E0492.rs:5:34


9    |
10 LL | static C: &'static AtomicUsize = &A;
-    |                                  ^^ this borrow of an interior mutable value may end up in the final value
12    |
12    |
13    = help: to fix this, the value can be extracted to a separate `static` item and then referenced


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0492/E0492.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0492/E0492.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0492.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0492.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0492" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0492/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0492]: constants cannot refer to interior mutable data
   |
   |
LL | const B: &'static AtomicUsize = &A; //~ ERROR E0492

error[E0492]: statics cannot refer to interior mutable data
  --> /checkout/src/test/ui/error-codes/E0492.rs:5:34
   |
   |
LL | static C: &'static AtomicUsize = &A; //~ ERROR E0492
   |
   |
   = help: to fix this, the value can be extracted to a separate `static` item and then referenced
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0492`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/impl-trait/issues/issue-78722.rs stdout ----
diff of stderr:

- error[E0658]: `async` blocks are not allowed in constants
+ error: `async` blocks are not allowed in constants
3    |
3    |
4 LL |         let f: F = async { 1 };
24 
25 error: aborting due to 3 previous errors
26 
- Some errors have detailed explanations: E0271, E0493, E0658.
---
To only update this specific test, also pass `--test-args impl-trait/issues/issue-78722.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-78722.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722/auxiliary"
stdout: none
--- stderr -------------------------------
error: `async` blocks are not allowed in constants
   |
   |
LL |         let f: F = async { 1 };
   |
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable


error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/impl-trait/issues/issue-78722.rs:13:13
   |
LL |         let f: F = async { 1 };
   |             ^ constants cannot evaluate destructors
LL |     }],
   |     - value is dropped here


error[E0271]: expected `impl Future<Output = ()>` to be a future that resolves to `u8`, but it resolves to `()`
   |
LL |         fn concrete_use() -> F {
   |                              ^ expected `()`, found `u8`

---
diff of stderr:

2   --> $DIR/static-mut-not-constant.rs:3:28
3    |
4 LL | static mut a: Box<isize> = box 3;
-    |                            ^^^^^ allocation not allowed in statics
6 
7 error: aborting due to previous error
8 

---
To only update this specific test, also pass `--test-args static/static-mut-not-constant.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-mut-not-constant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-not-constant" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-mut-not-constant/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0010]: allocations are not allowed in statics
   |
   |
LL | static mut a: Box<isize> = box 3;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0010`.
For more information about this error, try `rustc --explain E0010`.
------------------------------------------


---- [ui] src/test/ui/unsafe/ranged_ints3_const.rs#mirunsafeck stdout ----
diff of stderr:

- error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
+ error: cannot borrow here, since the borrowed element may contain interior mutability
3    |
3    |
4 LL |     let y = &x.0;
7    = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
8    = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable
9 
9 
- error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
+ error: cannot borrow here, since the borrowed element may contain interior mutability
12    |
12    |
13 LL |     let y = unsafe { &x.0 };
26 
27 error: aborting due to 3 previous errors
28 
- Some errors have detailed explanations: E0133, E0658.
- Some errors have detailed explanations: E0133, E0658.
- For more information about an error, try `rustc --explain E0133`.
+ For more information about this error, try `rustc --explain E0133`.
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const.mirunsafeck/ranged_ints3_const.mirunsafeck.stderr
To only update this specific test, also pass `--test-args unsafe/ranged_ints3_const.rs`


error in revision `mirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints3_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "mirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const.mirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const.mirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot borrow here, since the borrowed element may contain interior mutability
   |
   |
LL |     let y = &x.0; //~ ERROR the borrowed element may contain interior mutability
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable


error: cannot borrow here, since the borrowed element may contain interior mutability
   |
   |
LL |     let y = unsafe { &x.0 }; //~ ERROR the borrowed element may contain interior mutability
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable


error[E0133]: borrow of layout constrained field with interior mutability is unsafe and requires unsafe function or block
   |
   |
LL |     let y = &x.0; //~ ERROR the borrowed element may contain interior mutability
   |             ^^^^ borrow of layout constrained field with interior mutability
   |
   = note: references to fields of layout constrained fields lose the constraints. Coupled with interior mutability, the field can be changed to invalid values
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0133`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/unsafe/ranged_ints3_const.rs#thirunsafeck stdout ----
diff of stderr:

6    |
7    = note: references to fields of layout constrained fields lose the constraints. Coupled with interior mutability, the field can be changed to invalid values
8 
- error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
+ error: cannot borrow here, since the borrowed element may contain interior mutability
11    |
11    |
12 LL |     let y = &x.0;
15    = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
16    = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable
17 
17 
- error[E0658]: cannot borrow here, since the borrowed element may contain interior mutability
+ error: cannot borrow here, since the borrowed element may contain interior mutability
20    |
20    |
21 LL |     let y = unsafe { &x.0 };
26 
27 error: aborting due to 3 previous errors
28 
- Some errors have detailed explanations: E0133, E0658.
- Some errors have detailed explanations: E0133, E0658.
- For more information about an error, try `rustc --explain E0133`.
+ For more information about this error, try `rustc --explain E0133`.
31 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const.thirunsafeck/ranged_ints3_const.thirunsafeck.stderr
To only update this specific test, also pass `--test-args unsafe/ranged_ints3_const.rs`


error in revision `thirunsafeck`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unsafe/ranged_ints3_const.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "thirunsafeck" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const.thirunsafeck" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "thir-unsafeck" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unsafe/ranged_ints3_const.thirunsafeck/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0133]: borrow of layout constrained field with interior mutability is unsafe and requires unsafe function or block
   |
   |
LL |     let y = &x.0; //~ ERROR the borrowed element may contain interior mutability
   |             ^^^^ borrow of layout constrained field with interior mutability
   |
   = note: references to fields of layout constrained fields lose the constraints. Coupled with interior mutability, the field can be changed to invalid values

error: cannot borrow here, since the borrowed element may contain interior mutability
   |
   |
LL |     let y = &x.0; //~ ERROR the borrowed element may contain interior mutability
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable


error: cannot borrow here, since the borrowed element may contain interior mutability
   |
   |
LL |     let y = unsafe { &x.0 }; //~ ERROR the borrowed element may contain interior mutability
   |
   = note: see issue #80384 <https://github.com/rust-lang/rust/issues/80384> for more information
   = help: add `#![feature(const_refs_to_cell)]` to the crate attributes to enable

