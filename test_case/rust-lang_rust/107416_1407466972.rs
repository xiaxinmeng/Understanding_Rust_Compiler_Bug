plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:ece894d15649f0f9d7884f915dc821f00bd0418b)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
............................................................i.................i......... 1936/14368
..ii.................................................................................... 2024/14368
........................................................................................ 2112/14368
................................................................i....................... 2200/14368
...................F....F............................................................... 2288/14368
........................................................................................ 2464/14368
........................................................................................ 2552/14368
........................................................................................ 2640/14368
........................................................................................ 2728/14368
---

---- [ui] tests/ui/const-generics/const-arg-in-const-arg.rs#full stdout ----
diff of stderr:

- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
3    |
3    |
4 LL |     let _: [u8; faz::<'a>(&())];

10 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
12 
12 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
15    |
15    |
16 LL |     let _: [u8; faz::<'b>(&())];

22 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
24 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
27    |
27    |
28 LL |     let _: Foo<{ faz::<'a>(&()) }>;

34 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
36 
36 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
39    |
39    |
40 LL |     let _: Foo<{ faz::<'b>(&()) }>;
94    |
94    |
95    = help: try adding a `where` bound using this expression: `where [(); bar::<N>()]:`
96 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
99    |
99    |
100 LL |     let _ = [0; faz::<'a>(&())];

106 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
108 
108 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
111    |
111    |
112 LL |     let _ = [0; faz::<'b>(&())];
134    |
134    |
135    = help: try adding a `where` bound using this expression: `where [(); { bar::<N>() }]:`
136 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
139    |
139    |
140 LL |     let _ = Foo::<{ faz::<'a>(&()) }>;

146 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
148 
148 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
151    |
151    |
152 LL |     let _ = Foo::<{ faz::<'b>(&()) }>;
160 
161 error: aborting due to 16 previous errors
162 
+ For more information about this error, try `rustc --explain E0793`.
---
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.full/const-arg-in-const-arg.full.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const-arg-in-const-arg.rs`

error in revision `full`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/const-arg-in-const-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.full" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.full/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:18:23
   |
LL |     let _: [u8; faz::<'a>(&())]; //[min]~ ERROR a non-static lifetime
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:8:14
   |
   |
LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:21:23
   |
LL |     let _: [u8; faz::<'b>(&())]; //[min]~ ERROR a non-static lifetime
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:8:14
   |
   |
LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:41:24
   |
LL |     let _: Foo<{ faz::<'a>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:8:14
   |
   |
LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:44:24
   |
LL |     let _: Foo<{ faz::<'b>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:8:14
   |
   |
LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }

error: unconstrained generic constant
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:13:12
   |
   |
LL |     let _: [u8; foo::<T>()]; //[min]~ ERROR generic parameters may not
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); foo::<T>()]:`
error: unconstrained generic constant
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:15:12
   |
   |
LL |     let _: [u8; bar::<N>()]; //[min]~ ERROR generic parameters may not
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); bar::<N>()]:`
error: unconstrained generic constant
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:36:12
   |
   |
LL |     let _: Foo<{ foo::<T>() }>; //[min]~ ERROR generic parameters may not
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { foo::<T>() }]:`
error: unconstrained generic constant
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:38:12
   |
   |
LL |     let _: Foo<{ bar::<N>() }>; //[min]~ ERROR generic parameters may not
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { bar::<N>() }]:`
error: unconstrained generic constant
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:25:17
   |
   |
LL |     let _ = [0; foo::<T>()]; //[min]~ ERROR constant expression depends on a generic parameter
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); foo::<T>()]:`
error: unconstrained generic constant
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:27:17
   |
   |
LL |     let _ = [0; bar::<N>()]; //[min]~ ERROR generic parameters may not
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); bar::<N>()]:`

error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:30:23
   |
LL |     let _ = [0; faz::<'a>(&())]; //[min]~ ERROR a non-static lifetime
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:8:14
   |
   |
LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:33:23
   |
LL |     let _ = [0; faz::<'b>(&())]; //[min]~ ERROR a non-static lifetime
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:8:14
   |
   |
LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }

error: unconstrained generic constant
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:47:19
   |
   |
LL |     let _ = Foo::<{ foo::<T>() }>; //[min]~ ERROR generic parameters may not
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { foo::<T>() }]:`
error: unconstrained generic constant
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:49:19
   |
   |
LL |     let _ = Foo::<{ bar::<N>() }>; //[min]~ ERROR generic parameters may not
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { bar::<N>() }]:`

error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:52:27
   |
LL |     let _ = Foo::<{ faz::<'a>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:8:14
   |
   |
LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:55:27
   |
LL |     let _ = Foo::<{ faz::<'b>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:8:14
   |
   |
LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }

error: aborting due to 16 previous errors

For more information about this error, try `rustc --explain E0793`.
For more information about this error, try `rustc --explain E0793`.
------------------------------------------


---- [ui] tests/ui/const-generics/const-arg-in-const-arg.rs#min stdout ----
diff of stderr:

216 LL |     let _: [u8; bar::<{ N }>()];
217    |                       +   +
218 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
221    |
221    |
222 LL |     let _: [u8; faz::<'a>(&())];

228 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
230 
230 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
233    |
233    |
234 LL |     let _: [u8; faz::<'b>(&())];

251 LL |     let _: Foo<{ bar::<{ N }>() }>;
252    |                        +   +
253 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
256    |
256    |
257 LL |     let _: Foo<{ faz::<'a>(&()) }>;

263 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
265 
265 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
268    |
268    |
269 LL |     let _: Foo<{ faz::<'b>(&()) }>;

294 LL |     let _ = [0; bar::<{ N }>()];
295    |                       +   +
296 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
299    |
299    |
300 LL |     let _ = [0; faz::<'a>(&())];

306 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
308 
308 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
311    |
311    |
312 LL |     let _ = [0; faz::<'b>(&())];

329 LL |     let _ = Foo::<{ bar::<{ N }>() }>;
330    |                           +   +
331 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
334    |
334    |
335 LL |     let _ = Foo::<{ faz::<'a>(&()) }>;

341 LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }
343 
343 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
346    |
346    |
347 LL |     let _ = Foo::<{ faz::<'b>(&()) }>;
355 
356 error: aborting due to 36 previous errors
357 
- Some errors have detailed explanations: E0658, E0747.
- Some errors have detailed explanations: E0658, E0747.
+ Some errors have detailed explanations: E0658, E0747, E0793.
359 For more information about an error, try `rustc --explain E0658`.
360 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.min/const-arg-in-const-arg.min.stderr
To only update this specific test, also pass `--test-args const-generics/const-arg-in-const-arg.rs`


error in revision `min`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/const-generics/const-arg-in-const-arg.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.min" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-arg-in-const-arg.min/auxiliary"
stdout: none
--- stderr -------------------------------
error: generic parameters may not be used in const operations
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:13:23
   |
LL |     let _: [u8; foo::<T>()]; //[min]~ ERROR generic parameters may not
   |                       ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
error: generic parameters may not be used in const operations
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:15:23
   |
   |
LL |     let _: [u8; bar::<N>()]; //[min]~ ERROR generic parameters may not
   |                       ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:18:23
   |
LL |     let _: [u8; faz::<'a>(&())]; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:20:23
   |
LL |     let _: [u8; baz::<'a>(&())]; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:21:23
   |
LL |     let _: [u8; faz::<'b>(&())]; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:23:23
   |
LL |     let _: [u8; baz::<'b>(&())]; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error: generic parameters may not be used in const operations
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:27:23
   |
LL |     let _ = [0; bar::<N>()]; //[min]~ ERROR generic parameters may not
   |                       ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:30:23
   |
LL |     let _ = [0; faz::<'a>(&())]; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:32:23
   |
LL |     let _ = [0; baz::<'a>(&())]; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:33:23
   |
LL |     let _ = [0; faz::<'b>(&())]; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:35:23
   |
LL |     let _ = [0; baz::<'b>(&())]; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error: generic parameters may not be used in const operations
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:36:24
   |
LL |     let _: Foo<{ foo::<T>() }>; //[min]~ ERROR generic parameters may not
   |                        ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
error: generic parameters may not be used in const operations
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:38:24
   |
   |
LL |     let _: Foo<{ bar::<N>() }>; //[min]~ ERROR generic parameters may not
   |                        ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:41:24
   |
LL |     let _: Foo<{ faz::<'a>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:43:24
   |
LL |     let _: Foo<{ baz::<'a>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:44:24
   |
LL |     let _: Foo<{ faz::<'b>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:46:24
   |
LL |     let _: Foo<{ baz::<'b>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error: generic parameters may not be used in const operations
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:47:27
   |
LL |     let _ = Foo::<{ foo::<T>() }>; //[min]~ ERROR generic parameters may not
   |                           ^ cannot perform const operation using `T`
   = note: type parameters may not be used in const expressions
   = note: type parameters may not be used in const expressions
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions
error: generic parameters may not be used in const operations
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:49:27
   |
   |
LL |     let _ = Foo::<{ bar::<N>() }>; //[min]~ ERROR generic parameters may not
   |                           ^ cannot perform const operation using `N`
   |
   = help: const parameters may only be used as standalone arguments, i.e. `N`
   = help: use `#![feature(generic_const_exprs)]` to allow generic const expressions

error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:52:27
   |
LL |     let _ = Foo::<{ faz::<'a>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:54:27
   |
LL |     let _ = Foo::<{ baz::<'a>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:55:27
   |
LL |     let _ = Foo::<{ faz::<'b>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0658]: a non-static lifetime is not allowed in a `const`
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:57:27
   |
LL |     let _ = Foo::<{ baz::<'b>(&()) }>; //[min]~ ERROR a non-static lifetime
   |
   = note: see issue #76560 <https://github.com/rust-lang/rust/issues/76560> for more information
   = help: add `#![feature(generic_const_exprs)]` to the crate attributes to enable


error[E0747]: unresolved item provided when a constant was expected
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:15:23
   |
LL |     let _: [u8; bar::<N>()]; //[min]~ ERROR generic parameters may not
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
LL |     let _: [u8; bar::<{ N }>()]; //[min]~ ERROR generic parameters may not
   |                       +   +

error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:18:23
   |
LL |     let _: [u8; faz::<'a>(&())]; //[min]~ ERROR a non-static lifetime
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/const-generics/const-arg-in-const-arg.rs:8:14
   |
   |
LL | const fn faz<'a>(_: &'a ()) -> usize { 13 }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
---
To only update this specific test, also pass `--test-args methods/method-call-lifetime-args-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/methods/method-call-lifetime-args-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-lifetime-args-fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-lifetime-args-fail/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0107]: this associated function takes 2 lifetime arguments but 1 lifetime argument was supplied
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:16:7
   |
LL |     S.early::<'static>();
   |       ^^^^^   ------- supplied 1 lifetime argument
   |       expected 2 lifetime arguments
   |
   |
note: associated function defined here, with 2 lifetime parameters: `'a`, `'b`
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:6:8
   |
LL |     fn early<'a, 'b>(self) -> (&'a u8, &'b u8) { loop {} }
help: add missing lifetime argument
   |
   |
LL |     S.early::<'static, 'static>();


error[E0107]: this associated function takes 2 lifetime arguments but 3 lifetime arguments were supplied
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:18:7
   |
LL |     S.early::<'static, 'static, 'static>();
   |       |
   |       expected 2 lifetime arguments
   |
   |
note: associated function defined here, with 2 lifetime parameters: `'a`, `'b`
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:6:8
   |
LL |     fn early<'a, 'b>(self) -> (&'a u8, &'b u8) { loop {} }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:27:15
   |
LL |     S::late::<'static>(S, &0, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:4:13
   |
   |
LL |     fn late<'a, 'b>(self, _: &'a u8, _: &'b u8) {}


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:29:15
   |
LL |     S::late::<'static, 'static>(S, &0, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:4:13
   |
   |
LL |     fn late<'a, 'b>(self, _: &'a u8, _: &'b u8) {}


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:31:15
   |
LL |     S::late::<'static, 'static, 'static>(S, &0, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:4:13
   |
   |
LL |     fn late<'a, 'b>(self, _: &'a u8, _: &'b u8) {}


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:34:21
   |
LL |     S::late_early::<'static, 'static>(S, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:7:19
   |
   |
LL |     fn late_early<'a, 'b>(self, _: &'a u8) -> &'b u8 { loop {} }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:36:21
   |
LL |     S::late_early::<'static, 'static, 'static>(S, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:7:19
   |
   |
LL |     fn late_early<'a, 'b>(self, _: &'a u8) -> &'b u8 { loop {} }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:40:24
   |
LL |     S::late_implicit::<'static>(S, &0, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:5:31
   |
   |
LL |     fn late_implicit(self, _: &u8, _: &u8) {}


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:42:24
   |
LL |     S::late_implicit::<'static, 'static>(S, &0, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:5:31
   |
   |
LL |     fn late_implicit(self, _: &u8, _: &u8) {}


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:44:24
   |
LL |     S::late_implicit::<'static, 'static, 'static>(S, &0, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:5:31
   |
   |
LL |     fn late_implicit(self, _: &u8, _: &u8) {}


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:47:30
   |
LL |     S::late_implicit_early::<'static, 'static>(S, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:8:41
   |
   |
LL |     fn late_implicit_early<'b>(self, _: &u8) -> &'b u8 { loop {} }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:49:30
   |
LL |     S::late_implicit_early::<'static, 'static, 'static>(S, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:8:41
   |
   |
LL |     fn late_implicit_early<'b>(self, _: &u8) -> &'b u8 { loop {} }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:52:35
   |
LL |     S::late_implicit_self_early::<'static, 'static>(&S);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:9:37
   |
   |
LL |     fn late_implicit_self_early<'b>(&self) -> &'b u8 { loop {} }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:54:35
   |
LL |     S::late_implicit_self_early::<'static, 'static, 'static>(&S);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:9:37
   |
   |
LL |     fn late_implicit_self_early<'b>(&self) -> &'b u8 { loop {} }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:57:28
   |
LL |     S::late_unused_early::<'static, 'static>(S);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:10:26
   |
   |
LL |     fn late_unused_early<'a, 'b>(self) -> &'b u8 { loop {} }


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:59:28
   |
LL |     S::late_unused_early::<'static, 'static, 'static>(S);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:10:26
   |
   |
LL |     fn late_unused_early<'a, 'b>(self) -> &'b u8 { loop {} }


error[E0107]: this associated function takes 2 lifetime arguments but 1 lifetime argument was supplied
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:63:8
   |
LL |     S::early::<'static>(S);
   |        ^^^^^   ------- supplied 1 lifetime argument
   |        expected 2 lifetime arguments
   |
   |
note: associated function defined here, with 2 lifetime parameters: `'a`, `'b`
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:6:8
   |
LL |     fn early<'a, 'b>(self) -> (&'a u8, &'b u8) { loop {} }
help: add missing lifetime argument
   |
   |
LL |     S::early::<'static, 'static>(S);


error[E0107]: this associated function takes 2 lifetime arguments but 3 lifetime arguments were supplied
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:65:8
   |
LL |     S::early::<'static, 'static, 'static>(S);
   |        |
   |        expected 2 lifetime arguments
   |
   |
note: associated function defined here, with 2 lifetime parameters: `'a`, `'b`
  --> fake-test-src-base/methods/method-call-lifetime-args-fail.rs:6:8
   |
LL |     fn early<'a, 'b>(self) -> (&'a u8, &'b u8) { loop {} }

error: aborting due to 18 previous errors

Some errors have detailed explanations: E0107, E0793.
Some errors have detailed explanations: E0107, E0793.
For more information about an error, try `rustc --explain E0107`.
------------------------------------------


---- [ui] tests/ui/methods/method-call-lifetime-args.rs stdout ----
diff of stderr:

- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
3    |
3    |
4 LL |     S::late::<'static>(S, &0, &0);

10 LL |     fn late<'a, 'b>(self, _: &'a u8, _: &'b u8) {}
12 
12 
- error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
+ error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
15    |
15    |
16 LL |     S::late_implicit::<'static>(S, &0, &0);
24 
25 error: aborting due to 2 previous errors
26 
+ For more information about this error, try `rustc --explain E0793`.
---
To only update this specific test, also pass `--test-args methods/method-call-lifetime-args.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/methods/method-call-lifetime-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-lifetime-args" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/methods/method-call-lifetime-args/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args.rs:9:15
   |
LL |     S::late::<'static>(S, &0, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args.rs:4:13
   |
   |
LL |     fn late<'a, 'b>(self, _: &'a u8, _: &'b u8) {}


error[E0793]: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
  --> fake-test-src-base/methods/method-call-lifetime-args.rs:11:24
   |
LL |     S::late_implicit::<'static>(S, &0, &0);
   |
note: the late bound lifetime parameter is introduced here
  --> fake-test-src-base/methods/method-call-lifetime-args.rs:5:31
   |
   |
LL |     fn late_implicit(self, _: &u8, _: &u8) {}

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0793`.
