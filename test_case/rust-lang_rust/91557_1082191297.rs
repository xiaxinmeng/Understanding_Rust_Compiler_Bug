plain
.................................................................................................... 4500/12762
..............................................F..................................................... 4600/12762
.................................................................................................... 4700/12762
.................................................................................................... 4800/12762
..........................................................................................FiF...F... 4900/12762
.................................................................................................... 5100/12762
.................................................................................................... 5200/12762
.................................................................................................... 5300/12762
.................................................................................................... 5400/12762
---

---- [ui] ui/const-generics/const-arg-in-const-arg.rs#min stdout ----
diff of stderr:

16    = help: const parameters may only be used as standalone arguments, i.e. `N`
17    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
18 
- error: generic parameters may not be used in const operations
-   --> $DIR/const-arg-in-const-arg.rs:24:23
-    |
- LL |     let _ = [0; bar::<N>()];
-    |                       ^ cannot perform const operation using `N`
-    |
-    = help: const parameters may only be used as standalone arguments, i.e. `N`
-    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
- 
- error: generic parameters may not be used in const operations
-   --> $DIR/const-arg-in-const-arg.rs:29:24
-    |
- LL |     let _: Foo<{ foo::<T>() }>;
-    |                        ^ cannot perform const operation using `T`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    = note: type parameters may not be used in const expressions
-    = note: type parameters may not be used in const expressions
-    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
- 
- error: generic parameters may not be used in const operations
-   --> $DIR/const-arg-in-const-arg.rs:30:24
-    |
- LL |     let _: Foo<{ bar::<N>() }>;
-    |                        ^ cannot perform const operation using `N`
-    |
-    = help: const parameters may only be used as standalone arguments, i.e. `N`
-    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
- 
- error: generic parameters may not be used in const operations
-   --> $DIR/const-arg-in-const-arg.rs:35:27
-    |
- LL |     let _ = Foo::<{ foo::<T>() }>;
-    |                           ^ cannot perform const operation using `T`
-    = note: type parameters may not be used in const expressions
-    = note: type parameters may not be used in const expressions
-    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
- 
- error: generic parameters may not be used in const operations
-   --> $DIR/const-arg-in-const-arg.rs:36:27
-    |
- LL |     let _ = Foo::<{ bar::<N>() }>;
-    |                           ^ cannot perform const operation using `N`
-    |
-    = help: const parameters may only be used as standalone arguments, i.e. `N`
-    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
- 
64 error[E0658]: a non-static lifetime is not allowed in a `const`
66    |

97    = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
98    = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable
98    = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable
99 
+ error: generic parameters may not be used in const operations
+    |
+    |
+ LL |     let _ = [0; bar::<N>()];
+    |                       ^ cannot perform const operation using `N`
+    |
+    = help: const parameters may only be used as standalone arguments, i.e. `N`
+    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
+ 
100 error[E0658]: a non-static lifetime is not allowed in a `const`
102    |

133    = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
134    = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable
134    = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable
135 
+ error: generic parameters may not be used in const operations
+    |
+    |
+ LL |     let _: Foo<{ foo::<T>() }>;
+    |                        ^ cannot perform const operation using `T`
+    = note: type parameters may not be used in const expressions
+    = note: type parameters may not be used in const expressions
+    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
+ 
+ error: generic parameters may not be used in const operations
+    |
+    |
+ LL |     let _: Foo<{ bar::<N>() }>;
+    |                        ^ cannot perform const operation using `N`
+    |
+    = help: const parameters may only be used as standalone arguments, i.e. `N`
+    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
+ 
136 error[E0658]: a non-static lifetime is not allowed in a `const`
138    |

169    = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
170    = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable
170    = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable
171 
+ error: generic parameters may not be used in const operations
+    |
+    |
+ LL |     let _ = Foo::<{ foo::<T>() }>;
+    |                           ^ cannot perform const operation using `T`
+    = note: type parameters may not be used in const expressions
+    = note: type parameters may not be used in const expressions
+    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
+ 
+ error: generic parameters may not be used in const operations
+    |
+    |
+ LL |     let _ = Foo::<{ bar::<N>() }>;
+    |                           ^ cannot perform const operation using `N`
+    |
+    = help: const parameters may only be used as standalone arguments, i.e. `N`
+    = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
+ 
172 error[E0658]: a non-static lifetime is not allowed in a `const`
174    |

205    = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
206    = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable
206    = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable
207 
- error: aborting due to 23 previous errors
+ error[E0747]: unresolved item provided when a constant was expected
+   --> $DIR/const-arg-in-const-arg.rs:14:23
+    |
+ LL |     let _: [u8; bar::<N>()];
+    |
+    |
+ help: if this generic argument was intended as a const parameter, surround it with braces
+    |
+ LL |     let _: [u8; bar::<{ N }>()];
+    |                       +   +
- For more information about this error, try `rustc --explain E0658`.
- For more information about this error, try `rustc --explain E0658`.
+ error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+    |
+    |
+ LL |     let _: [u8; faz::<'a>(&())];
+    |
+ note: the late bound lifetime parameter is introduced here
+   --> $DIR/const-arg-in-const-arg.rs:8:14
+    |
+    |
+ LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
+ 
+ 
+ error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+    |
+    |
+ LL |     let _: [u8; faz::<'b>(&())];
+    |
+ note: the late bound lifetime parameter is introduced here
+   --> $DIR/const-arg-in-const-arg.rs:8:14
+    |
+    |
+ LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
+ 
+ error[E0747]: unresolved item provided when a constant was expected
+   --> $DIR/const-arg-in-const-arg.rs:24:23
+    |
+    |
+ LL |     let _ = [0; bar::<N>()];
+    |
+    |
+ help: if this generic argument was intended as a const parameter, surround it with braces
+    |
+ LL |     let _ = [0; bar::<{ N }>()];
+    |                       +   +
+ 
+ error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+    |
+    |
+ LL |     let _ = [0; faz::<'a>(&())];
+    |
+ note: the late bound lifetime parameter is introduced here
+   --> $DIR/const-arg-in-const-arg.rs:8:14
+    |
+    |
+ LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
+ 
+ 
+ error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+    |
+    |
+ LL |     let _ = [0; faz::<'b>(&())];
+    |
+ note: the late bound lifetime parameter is introduced here
+   --> $DIR/const-arg-in-const-arg.rs:8:14
+    |
+    |
+ LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
+ 
+ error[E0747]: unresolved item provided when a constant was expected
+   --> $DIR/const-arg-in-const-arg.rs:30:24
+    |
+    |
+ LL |     let _: Foo<{ bar::<N>() }>;
+    |
+    |
+ help: if this generic argument was intended as a const parameter, surround it with braces
+    |
+ LL |     let _: Foo<{ bar::<{ N }>() }>;
+    |                        +   +
+ 
+ error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+    |
+    |
+ LL |     let _: Foo<{ faz::<'a>(&()) }>;
+    |
+ note: the late bound lifetime parameter is introduced here
+   --> $DIR/const-arg-in-const-arg.rs:8:14
+    |
+    |
+ LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
+ 
+ 
+ error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+    |
+    |
+ LL |     let _: Foo<{ faz::<'b>(&()) }>;
+    |
+ note: the late bound lifetime parameter is introduced here
+   --> $DIR/const-arg-in-const-arg.rs:8:14
+    |
+    |
+ LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
+ 
+ error: constant expression depends on a generic parameter
+   --> $DIR/const-arg-in-const-arg.rs:22:17
+    |
+    |
+ LL |     let _ = [0; foo::<T>()];
+    |
+    = note: this may fail depending on what value the parameter takes
+ 
+ error[E0747]: unresolved item provided when a constant was expected
+ error[E0747]: unresolved item provided when a constant was expected
+   --> $DIR/const-arg-in-const-arg.rs:36:27
+    |
+ LL |     let _ = Foo::<{ bar::<N>() }>;
+    |
+    |
+ help: if this generic argument was intended as a const parameter, surround it with braces
+    |
+ LL |     let _ = Foo::<{ bar::<{ N }>() }>;
+    |                           +   +
+ 
+ error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+    |
+    |
+ LL |     let _ = Foo::<{ faz::<'a>(&()) }>;
+    |
+ note: the late bound lifetime parameter is introduced here
+   --> $DIR/const-arg-in-const-arg.rs:8:14
+    |
+    |
+ LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
+ 
+ 
+ error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+    |
+    |
+ LL |     let _ = Foo::<{ faz::<'b>(&()) }>;
+    |
+ note: the late bound lifetime parameter is introduced here
+   --> $DIR/const-arg-in-const-arg.rs:8:14
+    |
+    |
+ LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
+ 
+ error: aborting due to 36 previous errors
+ 
+ Some errors have detailed explanations: E0658, E0747.
+ Some errors have detailed explanations: E0658, E0747.
+ For more information about an error, try `rustc --explain E0658`.
211 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.min/const-arg-in-const-arg.min.stderr
To only update this specific test, also pass `--test-args const-generics/const-arg-in-const-arg.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-arg-in-const-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.min/auxiliary"
stdout: none
--- stderr -------------------------------
error: generic parameters may not be used in const operations
   |
   |
LL |     let _: [u8; foo::<T>()]; //~ ERROR generic parameters may not
   |                       ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |     let _: [u8; bar::<N>()]; //~ ERROR generic parameters may not
   |                       ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _: [u8; faz::<'a>(&())]; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _: [u8; baz::<'a>(&())]; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _: [u8; faz::<'b>(&())]; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _: [u8; baz::<'b>(&())]; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error: generic parameters may not be used in const operations
   |
   |
LL |     let _ = [0; bar::<N>()]; //~ ERROR generic parameters may not
   |                       ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _ = [0; faz::<'a>(&())]; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _ = [0; baz::<'a>(&())]; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _ = [0; faz::<'b>(&())]; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _ = [0; baz::<'b>(&())]; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error: generic parameters may not be used in const operations
   |
   |
LL |     let _: Foo<{ foo::<T>() }>; //~ ERROR generic parameters may not
   |                        ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |     let _: Foo<{ bar::<N>() }>; //~ ERROR generic parameters may not
   |                        ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _: Foo<{ faz::<'a>(&()) }>; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _: Foo<{ baz::<'a>(&()) }>; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _: Foo<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _: Foo<{ baz::<'b>(&()) }>; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error: generic parameters may not be used in const operations
   |
   |
LL |     let _ = Foo::<{ foo::<T>() }>; //~ ERROR generic parameters may not
   |                           ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error: generic parameters may not be used in const operations
   |
   |
LL |     let _ = Foo::<{ bar::<N>() }>; //~ ERROR generic parameters may not
   |                           ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _ = Foo::<{ faz::<'a>(&()) }>; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _ = Foo::<{ baz::<'a>(&()) }>; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _ = Foo::<{ faz::<'b>(&()) }>; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     let _ = Foo::<{ baz::<'b>(&()) }>; //~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


---
To only update this specific test, also pass `--test-args const-generics/outer-lifetime-in-const-generic-default.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/outer-lifetime-in-const-generic-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/outer-lifetime-in-const-generic-default" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/outer-lifetime-in-const-generic-default/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |         let x: &'a ();
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable

---

4 LL |     type Output = &i32;
5    |                   ^ expected named lifetime parameter
6    |
- help: consider using the `'_` lifetime
+ help: consider introducing a named lifetime parameter
8    |
- LL |     type Output = &'_ i32;
-    |                   ~~~
+ LL |     type Output<'a> = &'a i32;
+    |                ++++    ++
12 error[E0106]: missing lifetime specifier
13   --> $DIR/assoc-type.rs:16:20


15 LL |     type Output = &'_ i32;
16    |                    ^^ expected named lifetime parameter
17    |
- help: consider using the `'_` lifetime
+ help: consider introducing a named lifetime parameter
19    |
- LL |     type Output = &'_ i32;
-    |                    ~~
+ LL |     type Output<'a> = &'a i32;
+    |                ++++    ~~
23 error: aborting due to 2 previous errors
24 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/assoc-type/assoc-type.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-header-lifetime-elision/assoc-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-header-lifetime-elision/assoc-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/assoc-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/assoc-type/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
LL |     type Output = &i32;
   |                   ^ expected named lifetime parameter
   |
   |
help: consider introducing a named lifetime parameter
   |
LL |     type Output<'a> = &'a i32;
   |                ++++    ++
error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/impl-header-lifetime-elision/assoc-type.rs:16:20
   |
   |
LL |     type Output = &'_ i32;
   |                    ^^ expected named lifetime parameter
help: consider introducing a named lifetime parameter
   |
   |
LL |     type Output<'a> = &'a i32;
   |                ++++    ~~
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0106`.
------------------------------------------
------------------------------------------


---- [ui] ui/inline-const/const-expr-lifetime.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inline-const/const-expr-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-lifetime/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-lifetime/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     const { InvariantRef::<'a, ()>::new(&()) }
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable

---

---- [ui] ui/inline-const/const-expr-lifetime-err.rs stdout ----
diff of stderr:

- error[E0597]: `y` does not live long enough
-   --> $DIR/const-expr-lifetime-err.rs:23:30
+ error[E0658]: a non-static lifetime is not allowed in a `const`
3    |
3    |
- LL | fn foo<'a>() {
-    |        -- lifetime `'a` defined here
- LL |     let y = ();
7 LL |     equate(InvariantRef::new(&y), const { InvariantRef::<'a>::NEW });
-    |            ------------------^^-
-    |            |                 borrowed value does not live long enough
-    |            |                 borrowed value does not live long enough
-    |            argument requires that `y` is borrowed for `'a`
- LL |
- LL | }
-    | - `y` dropped here while still borrowed
+    |
+    = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
+    = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable
15 
---
To only update this specific test, also pass `--test-args inline-const/const-expr-lifetime-err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inline-const/const-expr-lifetime-err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-lifetime-err" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-expr-lifetime-err/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     equate(InvariantRef::new(&y), const { InvariantRef::<'a>::NEW });
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable

---
---- [ui] ui/inline-const/const-match-pat-lifetime.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inline-const/const-match-pat-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-match-pat-lifetime/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inline-const/const-match-pat-lifetime/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |     match const { InvariantRef::<'a, _>::new(&()) } {
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
   |
   |
LL |         const { InvariantRef::<'a, ()>::new(&()) } => {
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable

