plain
........................................................................................ 4664/12916
........................................................................................ 4752/12916
................F....................................................................... 4840/12916
........................................................................................ 4928/12916
.......F...........F.F..i............................................................... 5016/12916
........................................................................................ 5192/12916
........................................................................................ 5280/12916
........................................................................................ 5368/12916
........................................................................................ 5456/12916
---
...............................................................................ii....... 7304/12916
.........................ii...........................................................i. 7392/12916
........................................................................................ 7480/12916
......................................................................................i. 7568/12916
..........................................................FFF.F........F................ 7656/12916
.................F...................................................................... 7744/12916
............i....i..ii.................................................................. 7920/12916
........................................................................................ 8008/12916
........................................................................................ 8096/12916
........................................................................................ 8184/12916
---
---- [ui] src/test/ui/associated-consts/associated-const-ambiguity-report.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-consts/associated-const-ambiguity-report.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/associated-const-ambiguity-report" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-consts/associated-const-ambiguity-report/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/error-codes/E0034.rs stdout ----

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0034.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0034" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0034/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/impl-trait/no-method-suggested-traits.rs stdout ----
diff of stderr:


1 error[E0599]: no method named `method` found for type `u32` in the current scope
3    |
3    |
+ LL |         fn method(&self) {}
+    |            ------ the method is available for `u32` here
+ ...
4 LL |     1u32.method();
5    |          ^^^^^^ method not found in `u32`

7    = help: items from traits can only be used if the trait is in scope
- help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
+ help: the following trait is implemented but not in scope; perhaps add a `use` for it:
+ help: the following trait is implemented but not in scope; perhaps add a `use` for it:
9    |
10 LL | use foo::Bar;
11    |

- LL | use no_method_suggested_traits::Reexported;
-    |
- LL | use no_method_suggested_traits::foo::PubPub;
-    |
- LL | use no_method_suggested_traits::qux::PrivPub;
18 
18 
19 error[E0599]: no method named `method` found for struct `Rc<&mut Box<&u32>>` in the current scope


23    |                                            ^^^^^^ method not found in `Rc<&mut Box<&u32>>`
25    = help: items from traits can only be used if the trait is in scope
- help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
+ help: the following trait is implemented but not in scope; perhaps add a `use` for it:
27    |
27    |
28 LL | use foo::Bar;
-    |
- LL | use no_method_suggested_traits::Reexported;
-    |
- LL | use no_method_suggested_traits::foo::PubPub;
-    |
- LL | use no_method_suggested_traits::qux::PrivPub;
36 
36 
37 error[E0599]: no method named `method` found for type `char` in the current scope

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/no-method-suggested-traits.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/no-method-suggested-traits.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `method` found for type `u32` in the current scope
   |
   |
LL |         fn method(&self) {}
   |            ------ the method is available for `u32` here
...
LL |     1u32.method();
   |          ^^^^^^ method not found in `u32`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use foo::Bar;
LL | use foo::Bar;
   |

error[E0599]: no method named `method` found for struct `Rc<&mut Box<&u32>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&1u32)).method();
   |                                            ^^^^^^ method not found in `Rc<&mut Box<&u32>>`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use foo::Bar;
LL | use foo::Bar;
   |

error[E0599]: no method named `method` found for type `char` in the current scope
   |
   |
LL |         fn method(&self) {}
   |            ------ the method is available for `char` here
...
LL |     'a'.method();
   |         ^^^^^^ method not found in `char`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use foo::Bar;
LL | use foo::Bar;
   |

error[E0599]: no method named `method` found for struct `Rc<&mut Box<&char>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&'a')).method();
   |                                           ^^^^^^ method not found in `Rc<&mut Box<&char>>`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use foo::Bar;
LL | use foo::Bar;
   |

error[E0599]: no method named `method` found for type `i32` in the current scope
   |
   |
LL |     1i32.method();
   |          ^^^^^^ method not found in `i32`
  ::: /checkout/src/test/ui/impl-trait/auxiliary/no_method_suggested_traits.rs:8:12
   |
   |
LL |         fn method(&self) {}
   |            ------ the method is available for `i32` here
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
   |
LL | use no_method_suggested_traits::foo::PubPub;


error[E0599]: no method named `method` found for struct `Rc<&mut Box<&i32>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&1i32)).method();
   |                                            ^^^^^^ method not found in `Rc<&mut Box<&i32>>`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
   |
LL | use no_method_suggested_traits::foo::PubPub;


error[E0599]: no method named `method` found for struct `Foo` in the current scope
   |
LL | struct Foo;
LL | struct Foo;
   | ----------- method `method` not found for this
...
LL |     Foo.method();
   |         ^^^^^^ method not found in `Foo`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following traits define an item `method`, perhaps you need to implement one of them:
           candidate #1: `foo::Bar`
           candidate #2: `PubPub`
           candidate #3: `no_method_suggested_traits::qux::PrivPub`
           candidate #4: `Reexported`

error[E0599]: no method named `method` found for struct `Rc<&mut Box<&Foo>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&Foo)).method();
   |                                           ^^^^^^ method not found in `Rc<&mut Box<&Foo>>`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following traits define an item `method`, perhaps you need to implement one of them:
           candidate #1: `foo::Bar`
           candidate #2: `PubPub`
           candidate #3: `no_method_suggested_traits::qux::PrivPub`
           candidate #4: `Reexported`

error[E0599]: no method named `method2` found for type `u64` in the current scope
   |
   |
LL |     1u64.method2();
   |          ^^^^^^^ method not found in `u64`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
   |
LL |     pub trait Bar {
   |     ^^^^^^^^^^^^^


error[E0599]: no method named `method2` found for struct `Rc<&mut Box<&u64>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&1u64)).method2();
   |                                            ^^^^^^^ method not found in `Rc<&mut Box<&u64>>`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
   |
LL |     pub trait Bar {
   |     ^^^^^^^^^^^^^


error[E0599]: no method named `method2` found for struct `no_method_suggested_traits::Foo` in the current scope
   |
   |
LL |     no_method_suggested_traits::Foo.method2();
   |                                     ^^^^^^^ method not found in `no_method_suggested_traits::Foo`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
   |
LL |     pub trait Bar {
   |     ^^^^^^^^^^^^^


error[E0599]: no method named `method2` found for struct `Rc<&mut Box<&no_method_suggested_traits::Foo>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Foo)).method2();
   |                                                                       ^^^^^^^ method not found in `Rc<&mut Box<&no_method_suggested_traits::Foo>>`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
   |
LL |     pub trait Bar {
   |     ^^^^^^^^^^^^^


error[E0599]: no method named `method2` found for enum `no_method_suggested_traits::Bar` in the current scope
   |
   |
LL |     no_method_suggested_traits::Bar::X.method2();
   |                                        ^^^^^^^ method not found in `no_method_suggested_traits::Bar`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
   |
LL |     pub trait Bar {
   |     ^^^^^^^^^^^^^


error[E0599]: no method named `method2` found for struct `Rc<&mut Box<&no_method_suggested_traits::Bar>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method2();
   |                                                                          ^^^^^^^ method not found in `Rc<&mut Box<&no_method_suggested_traits::Bar>>`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `foo::Bar` defines an item `method2`, perhaps you need to implement it
   |
LL |     pub trait Bar {
   |     ^^^^^^^^^^^^^


error[E0599]: no method named `method3` found for struct `Foo` in the current scope
   |
LL | struct Foo;
LL | struct Foo;
   | ----------- method `method3` not found for this
...
LL |     Foo.method3();
   |         ^^^^^^^ method not found in `Foo`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `method3`, perhaps you need to implement it:
           candidate #1: `PubPub`

error[E0599]: no method named `method3` found for struct `Rc<&mut Box<&Foo>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&Foo)).method3();
   |                                           ^^^^^^^ method not found in `Rc<&mut Box<&Foo>>`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `method3`, perhaps you need to implement it:
           candidate #1: `PubPub`

error[E0599]: no method named `method3` found for enum `Bar` in the current scope
   |
   |
LL | enum Bar { X }
   | -------- method `method3` not found for this
...
LL |     Bar::X.method3();
   |            ^^^^^^^ method not found in `Bar`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `method3`, perhaps you need to implement it:
           candidate #1: `PubPub`

error[E0599]: no method named `method3` found for struct `Rc<&mut Box<&Bar>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&Bar::X)).method3();
   |                                              ^^^^^^^ method not found in `Rc<&mut Box<&Bar>>`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
   = note: the following trait defines an item `method3`, perhaps you need to implement it:
           candidate #1: `PubPub`

error[E0599]: no method named `method3` found for type `usize` in the current scope
   |
   |
LL |     1_usize.method3(); //~ ERROR no method named
   |             ^^^^^^^ method not found in `usize`

error[E0599]: no method named `method3` found for struct `Rc<&mut Box<&usize>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&1_usize)).method3(); //~ ERROR no method named
   |                                               ^^^^^^^ method not found in `Rc<&mut Box<&usize>>`

error[E0599]: no method named `method3` found for struct `no_method_suggested_traits::Foo` in the current scope
   |
   |
LL |     no_method_suggested_traits::Foo.method3();  //~ ERROR no method named
   |                                     ^^^^^^^ method not found in `no_method_suggested_traits::Foo`

error[E0599]: no method named `method3` found for struct `Rc<&mut Box<&no_method_suggested_traits::Foo>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Foo)).method3();
   |                                                                       ^^^^^^^ method not found in `Rc<&mut Box<&no_method_suggested_traits::Foo>>`

error[E0599]: no method named `method3` found for enum `no_method_suggested_traits::Bar` in the current scope
   |
   |
LL |     no_method_suggested_traits::Bar::X.method3();  //~ ERROR no method named
   |                                        ^^^^^^^ method not found in `no_method_suggested_traits::Bar`

error[E0599]: no method named `method3` found for struct `Rc<&mut Box<&no_method_suggested_traits::Bar>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method3();
   |                                                                          ^^^^^^^ method not found in `Rc<&mut Box<&no_method_suggested_traits::Bar>>`
error: aborting due to 24 previous errors

For more information about this error, try `rustc --explain E0599`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/inference/inference_unstable_featured.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/inference_unstable_featured.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable_featured" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable_featured/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/inference/issue-81522.rs stdout ----

error: test compilation failed although it shouldn't!
error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/issue-81522.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-81522/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-81522/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: use of unstable library feature 'ipu_flatten'
   |
   |
LL |     'x'.ipu_flatten();
   |
   = note: see issue #99999 <https://github.com/rust-lang/rust/issues/99999> for more information
   = note: see issue #99999 <https://github.com/rust-lang/rust/issues/99999> for more information
   = help: add `#![feature(ipu_flatten)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'ipu_flatten'
   |
   |
LL |     let _ = 'x'.ipu_flatten();
   |
   = note: see issue #99999 <https://github.com/rust-lang/rust/issues/99999> for more information
   = note: see issue #99999 <https://github.com/rust-lang/rust/issues/99999> for more information
   = help: add `#![feature(ipu_flatten)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'ipu_flatten'
   |
   |
LL |         'x'.ipu_flatten();
   |
   = note: see issue #99999 <https://github.com/rust-lang/rust/issues/99999> for more information
   = note: see issue #99999 <https://github.com/rust-lang/rust/issues/99999> for more information
   = help: add `#![feature(ipu_flatten)]` to the crate attributes to enable

warning: unused import: `inference_unstable_itertools::IpuItertools`
   |
   |
LL | use inference_unstable_itertools::IpuItertools;
   |
   = note: `#[warn(unused_imports)]` on by default

error: aborting due to 3 previous errors; 1 warning emitted
---
---- [ui] src/test/ui/inference/inference_unstable.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/inference_unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: use of unstable library feature 'ipu_flatten'
   |
   |
LL |     assert_eq!('x'.ipu_flatten(), 1);
   |
   = note: see issue #99999 <https://github.com/rust-lang/rust/issues/99999> for more information
   = note: see issue #99999 <https://github.com/rust-lang/rust/issues/99999> for more information
   = help: add `#![feature(ipu_flatten)]` to the crate attributes to enable
warning: an associated function with this name may be added to the standard library in the future
  --> /checkout/src/test/ui/inference/inference_unstable.rs:19:20
   |
   |
LL |     assert_eq!('x'.ipu_by_value_vs_by_ref(), 1);
   |
   = note: `#[warn(unstable_name_collisions)]` on by default
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `inference_unstable_itertools::IpuItertools::ipu_by_value_vs_by_ref(...)` to keep using the current method
   = help: add `#![feature(ipu_flatten)]` to the crate attributes to enable `inference_unstable_iterator::IpuIterator::ipu_by_value_vs_by_ref`
warning: an associated function with this name may be added to the standard library in the future
  --> /checkout/src/test/ui/inference/inference_unstable.rs:22:20
   |
   |
LL |     assert_eq!('x'.ipu_by_ref_vs_by_ref_mut(), 1);
   |
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `inference_unstable_itertools::IpuItertools::ipu_by_ref_vs_by_ref_mut(...)` to keep using the current method
   = help: add `#![feature(ipu_flatten)]` to the crate attributes to enable `inference_unstable_iterator::IpuIterator::ipu_by_ref_vs_by_ref_mut`
warning: an associated function with this name may be added to the standard library in the future
  --> /checkout/src/test/ui/inference/inference_unstable.rs:25:40
   |
   |
LL |     assert_eq!((&mut 'x' as *mut char).ipu_by_mut_ptr_vs_by_const_ptr(), 1);
   |
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `inference_unstable_itertools::IpuItertools::ipu_by_mut_ptr_vs_by_const_ptr(...)` to keep using the current method
   = help: add `#![feature(ipu_flatten)]` to the crate attributes to enable `inference_unstable_iterator::IpuIterator::ipu_by_mut_ptr_vs_by_const_ptr`

error[E0658]: use of unstable library feature 'assoc_const_ipu_iter'
   |
   |
LL |     assert_eq!(char::C, 1);
   |
   = note: see issue #99999 <https://github.com/rust-lang/rust/issues/99999> for more information
   = note: see issue #99999 <https://github.com/rust-lang/rust/issues/99999> for more information
   = help: add `#![feature(assoc_const_ipu_iter)]` to the crate attributes to enable
error: aborting due to 2 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0658`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-3702-2.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3702-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3702-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3702-2/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/issues/issue-65634-raw-ident-suggestion.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-65634-raw-ident-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65634-raw-ident-suggestion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-65634-raw-ident-suggestion/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/methods/method-ambig-two-traits-from-bounds.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-from-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-bounds" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-bounds/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/methods/method-ambig-two-traits-from-impls.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-from-impls.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-impls" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-impls/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/methods/method-ambig-two-traits-from-impls2.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-from-impls2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-impls2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-from-impls2/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/methods/method-ambig-two-traits-with-default-method.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-with-default-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-with-default-method" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-with-default-method/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/methods/method-ambig-two-traits-cross-crate.rs stdout ----

error: ui test compiled successfully!
error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-ambig-two-traits-cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-cross-crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-ambig-two-traits-cross-crate/auxiliary"
stdout: none
stderr: none

---- [ui] src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs stdout ----
diff of stderr:


23    |                   |
24    |                   expected due to this
25 
- error[E0034]: multiple applicable items in scope
-   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:122:15
-    |
- LL |     let z = x.foo();
-    |               ^^^ multiple `foo` found
-    |
- note: candidate #1 is defined in an impl of the trait `X` for the type `T`
-   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:45:9
-    |
- LL |         fn foo(self: Smaht<Self, u64>) -> u64 {
-    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- note: candidate #2 is defined in an impl of the trait `NuisanceFoo` for the type `T`
-   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:72:9
- LL |         fn foo(self) {}
-    |         ^^^^^^^^^^^^
-    |         ^^^^^^^^^^^^
- note: candidate #3 is defined in the trait `FinalFoo`
-   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:59:5
-    |
- LL |     fn foo(&self) -> u8;
- help: disambiguate the associated function for candidate #1
-    |
-    |
- LL |     let z = X::foo(x);
- help: disambiguate the associated function for candidate #2
-    |
-    |
- LL |     let z = NuisanceFoo::foo(x);
- help: disambiguate the associated function for candidate #3
-    |
-    |
- LL |     let z = FinalFoo::foo(x);
- 
60 error[E0308]: mismatched types
61   --> $DIR/method-deref-to-same-trait-object-with-separate-params.rs:139:24
62    |
---
To only update this specific test, also pass `--test-args methods/method-deref-to-same-trait-object-with-separate-params.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-deref-to-same-trait-object-with-separate-params/auxiliary"
stdout: none
--- stderr -------------------------------
warning: the feature `unsized_locals` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(unsized_locals, unsized_fn_params)]
   |            ^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #48055 <https://github.com/rust-lang/rust/issues/48055> for more information

error[E0308]: mismatched types
  --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:87:24
   |
LL |     let _seetype: () = z; //~ ERROR mismatched types
   |                   --   ^ expected `()`, found `u32`
   |                   expected due to this

error[E0308]: mismatched types
  --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:104:24
  --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:104:24
   |
LL |     let _seetype: () = z; //~ ERROR mismatched types
   |                   --   ^ expected `()`, found `u64`
   |                   expected due to this

error[E0308]: mismatched types
  --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:139:24
  --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:139:24
   |
LL |     let _seetype: () = z; //~ ERROR mismatched types
   |                   --   ^ expected `()`, found `u8`
   |                   expected due to this

error[E0308]: mismatched types
  --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:157:24
  --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:157:24
   |
LL |     let _seetype: () = z; //~ ERROR mismatched types
   |                   --   ^ expected `()`, found `u32`
   |                   expected due to this

error[E0308]: mismatched types
  --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:174:24
  --> /checkout/src/test/ui/methods/method-deref-to-same-trait-object-with-separate-params.rs:174:24
   |
LL |     let _seetype: () = z; //~ ERROR mismatched types
   |                   --   ^ expected `()`, found `u32`
   |                   expected due to this

error: aborting due to 5 previous errors; 1 warning emitted


For more information about this error, try `rustc --explain E0308`.
------------------------------------------


---- [ui] src/test/ui/rust-2021/future-prelude-collision-shadow.rs stdout ----
diff of stderr:

1 error[E0599]: no method named `try_into` found for type `u8` in the current scope
3    |
3    |
+ LL |         fn try_into(self) -> Result<u32, ()>;
+    |            -------- the method is available for `u8` here
+ ...
4 LL |         let _: u32 = 3u8.try_into().unwrap();
5    |                          ^^^^^^^^ method not found in `u8`

7    = help: items from traits can only be used if the trait is in scope
7    = help: items from traits can only be used if the trait is in scope
-    = note: 'std::convert::TryInto' is included in the prelude starting in Edition 2021
- help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
+ help: the following trait is implemented but not in scope; perhaps add a `use` for it:
10    |
11 LL |     use crate::m::TryIntoU32;
- LL |     use std::convert::TryInto;
14    |
15 
16 error: aborting due to previous error
---
To only update this specific test, also pass `--test-args rust-2021/future-prelude-collision-shadow.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2021/future-prelude-collision-shadow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2021/future-prelude-collision-shadow" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2021/future-prelude-collision-shadow/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `try_into` found for type `u8` in the current scope
   |
   |
LL |         fn try_into(self) -> Result<u32, ()>;
   |            -------- the method is available for `u8` here
...
LL |         let _: u32 = 3u8.try_into().unwrap();
   |                          ^^^^^^^^ method not found in `u8`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
   |
LL |     use crate::m::TryIntoU32;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
------------------------------------------


---- [ui] src/test/ui/span/issue-37767.rs stdout ----
diff of stderr:

- error[E0034]: multiple applicable items in scope
-   --> $DIR/issue-37767.rs:10:7
+ error[E0596]: cannot borrow `*a` as mutable, as it is behind a `&` reference
3    |
3    |
+ LL | fn bar<T: B>(a: &T) {
+    |                 -- help: consider changing this to be a mutable reference: `&mut T`
4 LL |     a.foo()
-    |       ^^^ multiple `foo` found
-    |
- note: candidate #1 is defined in the trait `A`
-   --> $DIR/issue-37767.rs:2:5
- LL |     fn foo(&mut self) {}
-    |     ^^^^^^^^^^^^^^^^^
-    |     ^^^^^^^^^^^^^^^^^
- note: candidate #2 is defined in the trait `B`
-   --> $DIR/issue-37767.rs:6:5
- LL |     fn foo(&mut self) {}
-    |     ^^^^^^^^^^^^^^^^^
- help: disambiguate the associated function for candidate #1
-    |
-    |
- LL |     A::foo(&a)
- help: disambiguate the associated function for candidate #2
-    |
-    |
- LL |     B::foo(&a)
-    |     ~~~~~~~~~~
+    |     ^^^^^^^ `a` is a `&` reference, so the data it refers to cannot be borrowed as mutable
- error[E0034]: multiple applicable items in scope
-   --> $DIR/issue-37767.rs:22:7
-    |
-    |
- LL |     a.foo()
-    |       ^^^ multiple `foo` found
-    |
- note: candidate #1 is defined in the trait `C`
-   --> $DIR/issue-37767.rs:14:5
- LL |     fn foo(&self) {}
-    |     ^^^^^^^^^^^^^
-    |     ^^^^^^^^^^^^^
- note: candidate #2 is defined in the trait `D`
-   --> $DIR/issue-37767.rs:18:5
- LL |     fn foo(&self) {}
-    |     ^^^^^^^^^^^^^
- help: disambiguate the associated function for candidate #1
-    |
-    |
- LL |     C::foo(&a)
- help: disambiguate the associated function for candidate #2
-    |
-    |
- LL |     D::foo(&a)
+ error: aborting due to previous error
50 
- error[E0034]: multiple applicable items in scope
-   --> $DIR/issue-37767.rs:34:7
-   --> $DIR/issue-37767.rs:34:7
-    |
- LL |     a.foo()
-    |       ^^^ multiple `foo` found
-    |
- note: candidate #1 is defined in the trait `E`
-   --> $DIR/issue-37767.rs:26:5
- LL |     fn foo(self) {}
-    |     ^^^^^^^^^^^^
-    |     ^^^^^^^^^^^^
- note: candidate #2 is defined in the trait `F`
-   --> $DIR/issue-37767.rs:30:5
- LL |     fn foo(self) {}
-    |     ^^^^^^^^^^^^
- help: disambiguate the associated function for candidate #1
-    |
-    |
- LL |     E::foo(a)
- help: disambiguate the associated function for candidate #2
-    |
-    |
- LL |     F::foo(a)
- 
- error: aborting due to 3 previous errors
- 
- For more information about this error, try `rustc --explain E0034`.
---
To only update this specific test, also pass `--test-args span/issue-37767.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-37767.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-37767" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-37767/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0596]: cannot borrow `*a` as mutable, as it is behind a `&` reference
   |
   |
LL | fn bar<T: B>(a: &T) {
   |                 -- help: consider changing this to be a mutable reference: `&mut T`
LL |     a.foo() //~ ERROR multiple applicable items
   |     ^^^^^^^ `a` is a `&` reference, so the data it refers to cannot be borrowed as mutable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0596`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/suggestions/dont-wrap-ambiguous-receivers.rs stdout ----
diff of stderr:

4 LL |     pub struct Chaenomeles;
5    |     ----------------------- method `pick` not found for this
6 ...
+ LL |         fn pick(&self, a: &mut ()) {}
+    |            ---- the method is available for `Chaenomeles` here
+ ...
7 LL |     banana::Chaenomeles.pick()
8    |                         ^^^^ method not found in `Chaenomeles`

10    = help: items from traits can only be used if the trait is in scope
- help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
-    |
-    |
- LL | use banana::Apple;
+ help: the following trait is implemented but not in scope; perhaps add a `use` for it:
14    |
15 LL | use banana::Peach;


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-wrap-ambiguous-receivers/dont-wrap-ambiguous-receivers.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-wrap-ambiguous-receivers/dont-wrap-ambiguous-receivers.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/dont-wrap-ambiguous-receivers.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/dont-wrap-ambiguous-receivers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-wrap-ambiguous-receivers" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/dont-wrap-ambiguous-receivers/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no method named `pick` found for struct `Chaenomeles` in the current scope
   |
   |
LL |     pub struct Chaenomeles;
   |     ----------------------- method `pick` not found for this
...
LL |         fn pick(&self, a: &mut ()) {}
   |            ---- the method is available for `Chaenomeles` here
...
LL |     banana::Chaenomeles.pick()
   |                         ^^^^ method not found in `Chaenomeles`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
   |
LL | use banana::Peach;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.
------------------------------------------


---- [ui] src/test/ui/traits/alias/ambiguous.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/alias/ambiguous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/ambiguous" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/alias/ambiguous/auxiliary"
stdout: none
stderr: none


failures:
    [ui] src/test/ui/associated-consts/associated-const-ambiguity-report.rs
