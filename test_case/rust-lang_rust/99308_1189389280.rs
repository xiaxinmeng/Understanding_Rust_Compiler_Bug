plain

---- [ui] src/test/ui/associated-types/associated-types-unconstrained.rs stdout ----
diff of stderr:

- error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
+ error[E0283]: type annotations needed
3    |
- LL |     fn bar() -> isize;
-    |     ------------------ `Foo::bar` defined here
- ...
- ...
7 LL |     let x: isize = Foo::bar();
8    |                    ^^^^^^^^ cannot call trait method as a free function

10 error: aborting due to previous error
11 
- For more information about this error, try `rustc --explain E0790`.
---

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-unconstrained.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-unconstrained/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/associated-types/associated-types-unconstrained.rs:14:20
   |
LL |     let x: isize = Foo::bar();
LL |     let x: isize = Foo::bar();
   |                    ^^^^^^^^ cannot call trait method as a free function
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/error-codes/E0283.rs stdout ----
diff of stderr:

- error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
+ error[E0283]: type annotations needed
2   --> $DIR/E0283.rs:30:21
- LL |     fn create() -> u32;
-    |     ------------------- `Generator::create` defined here
- ...
- ...
7 LL |     let cont: u32 = Generator::create();
8    |                     ^^^^^^^^^^^^^^^^^ cannot call trait method as a free function

15 
16 error: aborting due to 2 previous errors
17 
---
To only update this specific test, also pass `--test-args error-codes/E0283.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0283.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0283" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0283/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/error-codes/E0283.rs:30:21
   |
   |
LL |     let cont: u32 = Generator::create(); //~ ERROR E0790
   |                     ^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/error-codes/E0283.rs:35:24
   |
   |
LL |     let bar = foo_impl.into() * 1u32; //~ ERROR E0283
   |                        ^^^^ cannot call trait method as a free function
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/error-codes/E0790.rs stdout ----
diff of stderr:

- error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
+ error[E0283]: type annotations needed
2   --> $DIR/E0790.rs:17:9
3    |
- LL |         fn my_fn();
-    |         ----------- `MyTrait::my_fn` defined here
- ...
7 LL |         MyTrait::my_fn();
-    |         ^^^^^^^^^^^^^^ cannot call associated function of trait
- help: use the fully-qualified path to the only available implementation
-    |
-    |
- LL |         <::inner::MyStruct as MyTrait>::my_fn();
-    |         +++++++++++++++++++++        +
+    |         ^^^^^^^^^^^^^^ cannot call trait method as a free function
14 
- error[E0790]: cannot refer to the associated constant on trait without specifying the corresponding `impl` type
+ error[E0283]: type annotations needed
16   --> $DIR/E0790.rs:21:17
17    |
- LL |         const MY_ASSOC_CONST: ();
-    |         ------------------------- `MyTrait::MY_ASSOC_CONST` defined here
- ...
21 LL |         let _ = MyTrait::MY_ASSOC_CONST;
-    |
- help: use the fully-qualified path to the only available implementation
-    |
-    |
- LL |         let _ = <::inner::MyStruct as MyTrait>::MY_ASSOC_CONST;
-    |                 +++++++++++++++++++++        +
+    |                 ^^^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
28 
- error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
+ error[E0283]: type annotations needed
30   --> $DIR/E0790.rs:26:5
31    |
- LL |         fn my_fn();
-    |         ----------- `MyTrait::my_fn` defined here
- ...
35 LL |     inner::MyTrait::my_fn();
-    |     ^^^^^^^^^^^^^^^^^^^^^ cannot call associated function of trait
- help: use the fully-qualified path to the only available implementation
-    |
-    |
- LL |     inner::<::inner::MyStruct as MyTrait>::my_fn();
-    |            +++++++++++++++++++++        +
+    |     ^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
42 
- error[E0790]: cannot refer to the associated constant on trait without specifying the corresponding `impl` type
+ error[E0283]: type annotations needed
44   --> $DIR/E0790.rs:30:13
45    |
- LL |         const MY_ASSOC_CONST: ();
-    |         ------------------------- `MyTrait::MY_ASSOC_CONST` defined here
- ...
49 LL |     let _ = inner::MyTrait::MY_ASSOC_CONST;
-    |
- help: use the fully-qualified path to the only available implementation
-    |
-    |
- LL |     let _ = inner::<::inner::MyStruct as MyTrait>::MY_ASSOC_CONST;
-    |                    +++++++++++++++++++++        +
+    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
56 
- error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
+ error[E0283]: type annotations needed
58   --> $DIR/E0790.rs:50:5
59    |
- LL |     fn my_fn();
-    |     ----------- `MyTrait2::my_fn` defined here
- ...
63 LL |     MyTrait2::my_fn();
-    |     ^^^^^^^^^^^^^^^ cannot call associated function of trait
-    |
- help: use a fully-qualified path to a specific available implementation (2 found)
-    |
- LL |     <::Impl1 as MyTrait2>::my_fn();
-    |     +++++++++++         +
+    |     ^^^^^^^^^^^^^^^ cannot call trait method as a free function
71 error: aborting due to 5 previous errors
72 

- For more information about this error, try `rustc --explain E0790`.
---
To only update this specific test, also pass `--test-args error-codes/E0790.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0790.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0790" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0790/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/error-codes/E0790.rs:17:9
   |
   |
LL |         MyTrait::my_fn(); //~ ERROR E0790
   |         ^^^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/error-codes/E0790.rs:21:17
   |
   |
LL |         let _ = MyTrait::MY_ASSOC_CONST; //~ ERROR E0790
   |                 ^^^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/error-codes/E0790.rs:26:5
   |
   |
LL |     inner::MyTrait::my_fn(); //~ ERROR E0790
   |     ^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/error-codes/E0790.rs:30:13
   |
   |
LL |     let _ = inner::MyTrait::MY_ASSOC_CONST; //~ ERROR E0790
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/error-codes/E0790.rs:50:5
   |
   |
LL |     MyTrait2::my_fn(); //~ ERROR E0790
   |     ^^^^^^^^^^^^^^^ cannot call trait method as a free function
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/traits/static-method-generic-inference.rs stdout ----
diff of stderr:

- error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
+ error[E0283]: type annotations needed
3    |
- LL |         fn new() -> T;
- LL |         fn new() -> T;
-    |         -------------- `HasNew::new` defined here
- ...
7 LL |     let _f: base::Foo = base::HasNew::new();
8    |                         ^^^^^^^^^^^^^^^^^ cannot call trait method as a free function

10 error: aborting due to previous error
11 
- For more information about this error, try `rustc --explain E0790`.
---
To only update this specific test, also pass `--test-args traits/static-method-generic-inference.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/static-method-generic-inference.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/static-method-generic-inference" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/static-method-generic-inference/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/traits/static-method-generic-inference.rs:24:25
   |
   |
LL |     let _f: base::Foo = base::HasNew::new();
   |                         ^^^^^^^^^^^^^^^^^ cannot call trait method as a free function
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.
------------------------------------------
