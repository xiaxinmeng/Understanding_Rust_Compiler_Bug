plain
.................................................................................................... 9400/11771
.................................................................................................... 9500/11771
...................................................................................................i 9600/11771
......i............................................................................................. 9700/11771
.............................................iiiiiii..iiiiii.i...................................... 9800/11771
.....................................................................................F.............. 9900/11771
.................................................................................................... 10100/11771
.................................................................................................... 10200/11771
.................................................................................................... 10300/11771
.................................................................................................... 10400/11771
---

---- [ui] ui/hygiene/trait_items.rs stdout ----
diff of stderr:

1 error[E0599]: no method named `f` found for unit type `()` in the current scope
3    |
+ LL |         fn f(&self) {}
+ LL |         fn f(&self) {}
+    |            - the method is available for `()` here
+ ...
4 LL |     fn f() { ::baz::m!(); }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
6 ...



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/trait_items/trait_items.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args hygiene/trait_items.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/hygiene/trait_items.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/trait_items" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/hygiene/trait_items/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `f` found for unit type `()` in the current scope
   |
LL |         fn f(&self) {}
LL |         fn f(&self) {}
   |            - the method is available for `()` here
...
LL |     fn f() { ::baz::m!(); }
...
...
LL |     pub macro m() { ().f() } //~ ERROR no method named `f` found
   |                        ^ method not found in `()`
   = help: items from traits can only be used if the trait is in scope
   = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
   = note: the following trait is implemented but not in scope; perhaps add a `use` for it:
           `use foo::T;`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui/impl-trait/no-method-suggested-traits.rs stdout ----
diff of stderr:

37 error[E0599]: no method named `method` found for type `char` in the current scope
39    |
39    |
+ LL |         fn method(&self) {}
+    |            ------ the method is available for `char` here
+ ...
40 LL |     'a'.method();
41    |         ^^^^^^ method not found in `char`

63    |
63    |
64 LL |     1i32.method();
65    |          ^^^^^^ method not found in `i32`
+    | 
+   ::: $DIR/auxiliary/no_method_suggested_traits.rs:8:12
+    |
+ LL |         fn method(&self) {}
+    |            ------ the method is available for `i32` here
67    = help: items from traits can only be used if the trait is in scope
68 help: the following trait is implemented but not in scope; perhaps add a `use` for it:



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/no-method-suggested-traits.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/no-method-suggested-traits.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/no-method-suggested-traits.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/no-method-suggested-traits/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `method` found for type `u32` in the current scope
   |
   |
LL |     1u32.method();
   |          ^^^^^^ method not found in `u32`
   = help: items from traits can only be used if the trait is in scope
help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
   |
LL | use foo::Bar;
LL | use foo::Bar;
   |
LL | use no_method_suggested_traits::foo::PubPub;
   |
LL | use no_method_suggested_traits::qux::PrivPub;
LL | use no_method_suggested_traits::Reexported;
   |


error[E0599]: no method named `method` found for struct `Rc<&mut Box<&u32>>` in the current scope
   |
   |
LL |     std::rc::Rc::new(&mut Box::new(&1u32)).method();
   |                                            ^^^^^^ method not found in `Rc<&mut Box<&u32>>`
   = help: items from traits can only be used if the trait is in scope
help: the following traits are implemented but not in scope; perhaps add a `use` for one of them:
   |
LL | use foo::Bar;
LL | use foo::Bar;
   |
LL | use no_method_suggested_traits::foo::PubPub;
   |
LL | use no_method_suggested_traits::qux::PrivPub;
LL | use no_method_suggested_traits::Reexported;
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


---- [ui] ui/issues/issue-43189.rs stdout ----
diff of stderr:

3    |
4 LL |     ().a();
5    |        ^ method not found in `()`
+    | 
+   ::: $DIR/auxiliary/xcrate-issue-43189-a.rs:5:8
+    |
+ LL |     fn a(&self) {}
+    |        - the method is available for `()` here
7    = help: items from traits can only be used if the trait is in scope
8 help: the following trait is implemented but not in scope; perhaps add a `use` for it:



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189/issue-43189.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-43189.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-43189.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43189/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `a` found for unit type `()` in the current scope
   |
   |
LL |     ().a();
   |        ^ method not found in `()`
  ::: /checkout/src/test/ui/issues/auxiliary/xcrate-issue-43189-a.rs:5:8
   |
   |
LL |     fn a(&self) {}
   |        - the method is available for `()` here
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
   |
LL | use xcrate_issue_43189_b::xcrate_issue_43189_a::A;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
---
---- [ui] ui/issues/issue-56175.rs stdout ----
diff of stderr:

3    |
4 LL |     reexported_trait::FooStruct.trait_method();
5    |                                 ^^^^^^^^^^^^ method not found in `FooStruct`
+    | 
+   ::: $DIR/auxiliary/reexported-trait.rs:3:12
6    |
+ LL |         fn trait_method(&self) {
+    |            ------------ the method is available for `FooStruct` here
7    = help: items from traits can only be used if the trait is in scope
8 help: the following trait is implemented but not in scope; perhaps add a `use` for it:
9    |


15    |
16 LL |     reexported_trait::FooStruct.trait_method_b();
17    |                                 ^^^^^^^^^^^^^^ method not found in `FooStruct`
+    | 
+   ::: $DIR/auxiliary/reexported-trait.rs:7:12
+    |
+ LL |         fn trait_method_b(&self) {
+    |            -------------- the method is available for `FooStruct` here
19    = help: items from traits can only be used if the trait is in scope
20 help: the following trait is implemented but not in scope; perhaps add a `use` for it:



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56175/issue-56175.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-56175.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-56175.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56175" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56175/auxiliary" "--extern" "reexported_trait=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-56175/auxiliary/libreexported_trait.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `trait_method` found for struct `FooStruct` in the current scope
   |
   |
LL |     reexported_trait::FooStruct.trait_method();
   |                                 ^^^^^^^^^^^^ method not found in `FooStruct`
  ::: /checkout/src/test/ui/issues/auxiliary/reexported-trait.rs:3:12
   |
   |
LL |         fn trait_method(&self) {
   |            ------------ the method is available for `FooStruct` here
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
   |
LL | use reexported_trait::Trait;


error[E0599]: no method named `trait_method_b` found for struct `FooStruct` in the current scope
   |
   |
LL |     reexported_trait::FooStruct.trait_method_b();
   |                                 ^^^^^^^^^^^^^^ method not found in `FooStruct`
  ::: /checkout/src/test/ui/issues/auxiliary/reexported-trait.rs:7:12
   |
   |
LL |         fn trait_method_b(&self) {
   |            -------------- the method is available for `FooStruct` here
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
   |
LL | use reexported_trait::TraitBRename;

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0599`.
For more information about this error, try `rustc --explain E0599`.

------------------------------------------


---- [ui] ui/shadowed/shadowed-trait-methods.rs stdout ----
diff of stderr:

1 error[E0599]: no method named `f` found for unit type `()` in the current scope
3    |
3    |
+ LL |     pub trait T { fn f(&self) {} }
+    |                      - the method is available for `()` here
4 LL |     ().f()
5    |        ^ method not found in `()`
6    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/shadowed/shadowed-trait-methods/shadowed-trait-methods.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args shadowed/shadowed-trait-methods.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/shadowed/shadowed-trait-methods.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/shadowed/shadowed-trait-methods" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/shadowed/shadowed-trait-methods/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `f` found for unit type `()` in the current scope
   |
   |
LL |     pub trait T { fn f(&self) {} }
   |                      - the method is available for `()` here
...
LL |     ().f() //~ ERROR no method
   |        ^ method not found in `()`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use foo::T;
---

---- [ui] ui/traits/item-privacy.rs stdout ----
diff of stderr:

20 LL | struct S;
21    | --------- method `b` not found for this
22 ...
+ LL |         fn b(&self) { }
+    |            - the method is available for `S` here
23 LL |     S.b();
24    |       ^ method not found in `S`
25    |

---
To only update this specific test, also pass `--test-args traits/item-privacy.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/item-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/item-privacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/item-privacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `a` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- method `a` not found for this
...
LL |     S.a(); //~ ERROR no method named `a` found
   |       ^ method not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `method::A` defines an item `a`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no method named `b` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- method `b` not found for this
...
LL |         fn b(&self) { }
   |            - the method is available for `S` here
...
LL |     S.b(); //~ ERROR no method named `b` found
   |       ^ method not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use method::B;
LL | use method::B;
   |

error[E0624]: associated function `a` is private
   |
   |
LL |     c.a(); //~ ERROR associated function `a` is private
   |       ^ private associated function

error[E0599]: no function or associated item named `a` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- function or associated item `a` not found for this
...
LL |     S::a(&S);
   |        ^ function or associated item not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `method::A` defines an item `a`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no function or associated item named `b` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- function or associated item `b` not found for this
...
LL |     S::b(&S);
   |        ^ function or associated item not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use method::B;
LL | use method::B;
   |

error[E0624]: associated function `a` is private
   |
   |
LL |     <dyn C>::a(&S); //~ ERROR associated function `a` is private
   |              ^ private associated function

error[E0599]: no associated item named `A` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- associated item `A` not found for this
...
LL |     S::A; //~ ERROR no associated item named `A` found
   |        ^ associated item not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `assoc_const::A` defines an item `A`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no associated item named `B` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- associated item `B` not found for this
...
LL |     S::B; //~ ERROR no associated item named `B` found
   |        ^ associated item not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use assoc_const::B;
LL | use assoc_const::B;
   |

error[E0624]: associated constant `A` is private
   |
   |
LL |     <dyn C>::A; //~ ERROR associated constant `A` is private
   |              ^ private associated constant

error[E0038]: the trait `assoc_const::C` cannot be made into an object
   |
   |
LL |     <dyn C>::A; //~ ERROR associated constant `A` is private
   |      ^^^^^ `assoc_const::C` cannot be made into an object
   |
   = help: consider moving `C` to another trait
   = help: consider moving `B` to another trait
   = help: consider moving `A` to another trait
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
   |
   |
LL |         const A: u8 = 0;
   |               ^ ...because it contains this associated `const`
...
LL |         const B: u8 = 0;
   |               ^ ...because it contains this associated `const`
...
LL |     pub trait C: A + B {
   |               - this trait cannot be made into an object...
LL |         const C: u8 = 0;
   |               ^ ...because it contains this associated `const`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/item-privacy.rs:115:12
   |
   |
LL |     let _: S::A; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/item-privacy.rs:116:12
   |
   |
LL |     let _: S::B; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::B`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/item-privacy.rs:117:12
   |
   |
LL |     let _: S::C; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::C`

error: associated type `A` is private
   |
   |
LL |     let _: T::A; //~ ERROR associated type `A` is private
   |            ^^^^ private associated type

error: associated type `A` is private
   |
   |
LL |         A = u8, //~ ERROR associated type `A` is private
   |         ^^^^^^ private associated type
error: aborting due to 15 previous errors

Some errors have detailed explanations: E0038, E0223, E0599, E0624.
For more information about an error, try `rustc --explain E0038`.
---
test result: FAILED. 11668 passed; 6 failed; 97 ignored; 0 measured; 0 filtered out; finished in 122.31s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:07
