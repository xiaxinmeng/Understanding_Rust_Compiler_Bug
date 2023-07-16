plain

---- [ui] src/test/ui/consts/min_const_fn/min_const_fn.rs stdout ----
diff of stderr:

1 error[E0493]: destructor of `Foo<T>` cannot be evaluated at compile-time
-   --> $DIR/min_const_fn.rs:37:25
3    |
4 LL |     const fn into_inner(self) -> T { self.0 }
5    |                         ^^^^                - value is dropped here


7    |                         the destructor for this type cannot be evaluated in constant functions
8 
9 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:39:22
+   --> $DIR/min_const_fn.rs:41:22
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
11    |
12 LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }

16    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
17 
18 error[E0658]: mutable references are not allowed in constant functions
18 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:39:36
+   --> $DIR/min_const_fn.rs:41:36
20    |
21 LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }

25    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
26 
27 error[E0658]: mutable references are not allowed in constant functions
27 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:39:45
+   --> $DIR/min_const_fn.rs:41:45
29    |
30 LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }

34    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
35 
35 
36 error[E0493]: destructor of `Foo<T>` cannot be evaluated at compile-time
-   --> $DIR/min_const_fn.rs:46:28
38    |
38    |
39 LL |     const fn into_inner_lt(self) -> T { self.0 }
40    |                            ^^^^                - value is dropped here
42    |                            the destructor for this type cannot be evaluated in constant functions
43 
44 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:48:25
-   --> $DIR/min_const_fn.rs:48:25
+   --> $DIR/min_const_fn.rs:50:25
46    |
47 LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }

51    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
52 
53 error[E0658]: mutable references are not allowed in constant functions
53 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:48:42
+   --> $DIR/min_const_fn.rs:50:42
55    |
56 LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }

60    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
61 
62 error[E0658]: mutable references are not allowed in constant functions
62 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:48:51
+   --> $DIR/min_const_fn.rs:50:51
64    |
65 LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }

69    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
70 
70 
71 error[E0493]: destructor of `Foo<T>` cannot be evaluated at compile-time
-   --> $DIR/min_const_fn.rs:55:27
73    |
73    |
74 LL |     const fn into_inner_s(self) -> T { self.0 }
75    |                           ^^^^                - value is dropped here
77    |                           the destructor for this type cannot be evaluated in constant functions
78 
79 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:57:24
-   --> $DIR/min_const_fn.rs:57:24
+   --> $DIR/min_const_fn.rs:59:24
81    |
82 LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }

86    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
87 
88 error[E0658]: mutable references are not allowed in constant functions
88 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:57:38
+   --> $DIR/min_const_fn.rs:59:38
90    |
91 LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }

95    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
96 
97 error[E0658]: mutable references are not allowed in constant functions
97 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:57:47
+   --> $DIR/min_const_fn.rs:59:47
99    |
100 LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }

104    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
105 
106 error[E0658]: mutable references are not allowed in constant functions
106 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:64:25
+   --> $DIR/min_const_fn.rs:66:25
108    |
109 LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }

113    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
114 
115 error[E0658]: mutable references are not allowed in constant functions
115 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:64:39
+   --> $DIR/min_const_fn.rs:66:39
117    |
118 LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }

122    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
123 
124 error[E0658]: mutable references are not allowed in constant functions
124 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:64:48
+   --> $DIR/min_const_fn.rs:66:48
126    |
127 LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }

131    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
132 
133 error[E0013]: constant functions cannot refer to statics
133 error[E0013]: constant functions cannot refer to statics
-   --> $DIR/min_const_fn.rs:89:27
+   --> $DIR/min_const_fn.rs:91:27
135    |
136 LL | const fn foo25() -> u32 { BAR }


139    = help: consider extracting the value of the `static` to a `const`, and referring to that
141 error[E0013]: constant functions cannot refer to statics
-   --> $DIR/min_const_fn.rs:90:37
+   --> $DIR/min_const_fn.rs:92:37
143    |
143    |
144 LL | const fn foo26() -> &'static u32 { &BAR }


147    = help: consider extracting the value of the `static` to a `const`, and referring to that
149 error: pointers cannot be cast to integers during const eval
-   --> $DIR/min_const_fn.rs:91:42
+   --> $DIR/min_const_fn.rs:93:42
151    |
151    |
152 LL | const fn foo30(x: *const u32) -> usize { x as usize }


156    = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
158 error: pointers cannot be cast to integers during const eval
-   --> $DIR/min_const_fn.rs:93:63
+   --> $DIR/min_const_fn.rs:95:63
160    |
160    |
161 LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }


165    = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
167 error: pointers cannot be cast to integers during const eval
-   --> $DIR/min_const_fn.rs:95:42
+   --> $DIR/min_const_fn.rs:97:42
169    |
169    |
170 LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }


174    = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
176 error: pointers cannot be cast to integers during const eval
-   --> $DIR/min_const_fn.rs:97:63
+   --> $DIR/min_const_fn.rs:99:63
178    |
178    |
179 LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }


183    = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
185 error[E0658]: mutable references are not allowed in constant functions
-   --> $DIR/min_const_fn.rs:100:14
+   --> $DIR/min_const_fn.rs:102:14
187    |
187    |
188 LL | const fn inc(x: &mut i32) { *x += 1 }

192    = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable
193 
193 
194 error[E0493]: destructor of `AlanTuring<impl std::fmt::Debug>` cannot be evaluated at compile-time
-   --> $DIR/min_const_fn.rs:122:19
196    |
196    |
197 LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
198    |                   ^^                                     - value is dropped here
200    |                   the destructor for this type cannot be evaluated in constant functions
201 
201 
202 error[E0493]: destructor of `impl std::fmt::Debug` cannot be evaluated at compile-time
-   --> $DIR/min_const_fn.rs:124:18
204    |
204    |
205 LL | const fn no_apit(_x: impl std::fmt::Debug) {}
206    |                  ^^                         - value is dropped here

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/min_const_fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0493]: destructor of `Foo<T>` cannot be evaluated at compile-time
   |
   |
LL |     const fn into_inner(self) -> T { self.0 } //~ destructor of
   |                         ^^^^                - value is dropped here
   |                         the destructor for this type cannot be evaluated in constant functions

error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:41:22
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:41:22
   |
LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:41:36
   |
LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:41:45
   |
LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0493]: destructor of `Foo<T>` cannot be evaluated at compile-time
   |
   |
LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructor of
   |                            ^^^^                - value is dropped here
   |                            the destructor for this type cannot be evaluated in constant functions

error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:50:25
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:50:25
   |
LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:50:42
   |
LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:50:51
   |
LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0493]: destructor of `Foo<T>` cannot be evaluated at compile-time
   |
   |
LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructor
   |                           ^^^^                - value is dropped here
   |                           the destructor for this type cannot be evaluated in constant functions

error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:59:24
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:59:24
   |
LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:59:38
   |
LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:59:47
   |
LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:66:25
   |
LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:66:39
   |
LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:66:48
   |
LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0013]: constant functions cannot refer to statics
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:91:27
   |
LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot refer to statics
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error[E0013]: constant functions cannot refer to statics
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:92:37
   |
   |
LL | const fn foo26() -> &'static u32 { &BAR } //~ ERROR cannot refer to statics
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:93:42
   |
   |
LL | const fn foo30(x: *const u32) -> usize { x as usize }
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:95:63
   |
   |
LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:97:42
   |
   |
LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:99:63
   |
   |
LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:102:14
   |
   |
LL | const fn inc(x: &mut i32) { *x += 1 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0493]: destructor of `AlanTuring<impl std::fmt::Debug>` cannot be evaluated at compile-time
   |
   |
LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
   |                   ^^                                     - value is dropped here
   |                   the destructor for this type cannot be evaluated in constant functions


error[E0493]: destructor of `impl std::fmt::Debug` cannot be evaluated at compile-time
   |
   |
LL | const fn no_apit(_x: impl std::fmt::Debug) {}
   |                  ^^                         - value is dropped here
   |                  the destructor for this type cannot be evaluated in constant functions

error: aborting due to 24 previous errors

