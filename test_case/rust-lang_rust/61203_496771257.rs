plain
[01:36:17] 
[01:36:17] 298 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17] 299   --> $DIR/min_const_fn.rs:144:41
[01:36:17] 300    |
[01:36:17] - LL | const fn really_no_traits_i_mean_it() { (&() as &std::fmt::Debug, ()).1 }
[01:36:17] -    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:36:17] + LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
[01:36:17] 303    |
[01:36:17] 304    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17] 305    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll/min_const_fn.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error[E0493]: destructors cannot be evaluated at compile-time
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:37:25
[01:36:17]    |
[01:36:17] LL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated
[01:36:17]    |                         ^^^^ constant functions cannot evaluate destructors
[01:36:17] error[E0723]: mutable references in const fn are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:36
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0493]: destructors cannot be evaluated at compile-time
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:44:28
[01:36:17]    |
[01:36:17] LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructors cannot be evaluated
[01:36:17]    |                            ^^^^ constant functions cannot evaluate destructors
[01:36:17] error[E0723]: mutable references in const fn are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:46:42
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0493]: destructors cannot be evaluated at compile-time
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:51:27
[01:36:17]    |
[01:36:17] LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructors
[01:36:17]    |                           ^^^^ constant functions cannot evaluate destructors
[01:36:17] error[E0723]: mutable references in const fn are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:53:38
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: mutable references in const fn are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:58:39
[01:36:17]    |
[01:36:17] LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:76:16
[01:36:17]    |
[01:36:17] LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:78:18
[01:36:17]    |
[01:36:17] LL | const fn foo11_2<T: Send>(t: T) -> T { t }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: only int, `bool` and `char` operations are stable in const fn
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | const fn foo19(f: f32) -> f32 { f * 2.0 }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: only int, `bool` and `char` operations are stable in const fn
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | const fn foo19_2(f: f32) -> f32 { 2.0 - f }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: only int and `bool` operations are stable in const fn
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:84:35
[01:36:17]    |
[01:36:17] LL | const fn foo19_3(f: f32) -> f32 { -f }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: only int, `bool` and `char` operations are stable in const fn
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | const fn foo19_4(f: f32, g: f32) -> f32 { f / g }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: cannot access `static` items in const fn
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:90:27
[01:36:17]    |
[01:36:17] LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot access `static` items in const fn
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: cannot access `static` items in const fn
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:91:36
[01:36:17]    |
[01:36:17] LL | const fn foo26() -> &'static u32 { &BAR } //~ ERROR cannot access `static` items
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: casting pointers to ints is unstable in const fn
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:92:42
[01:36:17]    |
[01:36:17] LL | const fn foo30(x: *const u32) -> usize { x as usize }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: casting pointers to ints is unstable in const fn
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:94:63
[01:36:17]    |
[01:36:17] LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: casting pointers to ints is unstable in const fn
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:96:42
[01:36:17]    |
[01:36:17] LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: casting pointers to ints is unstable in const fn
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:98:63
[01:36:17]    |
[01:36:17] LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | const fn foo30_4(b: bool) -> usize { if b { 1 } else { 42 } }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | const fn foo30_5(b: bool) { while b { } } //~ ERROR not stable in const fn
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | const fn foo36(a: bool, b: bool) -> bool { a && b }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | const fn foo37(a: bool, b: bool) -> bool { a || b }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: mutable references in const fn are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:108:14
[01:36:17]    |
[01:36:17] LL | const fn inc(x: &mut i32) { *x += 1 }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:113:6
[01:36:17]    |
[01:36:17] LL | impl<T: std::fmt::Debug> Foo<T> {
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:118:6
[01:36:17]    |
[01:36:17] LL | impl<T: std::fmt::Debug + Sized> Foo<T> {
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:123:6
[01:36:17]    |
[01:36:17] LL | impl<T: Sync + Sized> Foo<T> {
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: `impl Trait` in const fn is unstable
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | const fn no_rpit2() -> AlanTuring<impl std::fmt::Debug> { AlanTuring(0) }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:131:34
[01:36:17]    |
[01:36:17] LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:133:22
[01:36:17]    |
[01:36:17] LL | const fn no_apit(_x: impl std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: `impl Trait` in const fn is unstable
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | const fn no_rpit() -> impl std::fmt::Debug {} //~ ERROR `impl Trait` in const fn is unstable
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:135:23
[01:36:17]    |
[01:36:17] LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:136:32
[01:36:17]    |
[01:36:17] LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0515]: cannot return reference to temporary value
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:136:63
[01:36:17]    |
[01:36:17] LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
[01:36:17]    |                                                               ^--
[01:36:17]    |                                                               |temporary value created here
[01:36:17]    |                                                               returns a reference to data owned by the current function
[01:36:17] 
[01:36:17] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17] error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:144:41
[01:36:17]    |
[01:36:17] LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: function pointers in const fn are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:147:21
[01:36:17]    |
[01:36:17] LL | const fn no_fn_ptrs(_x: fn()) {}
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0723]: function pointers in const fn are unstable
[01:36:17]   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:149:27
[01:36:17]    |
[01:36:17] LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
[01:36:17]    |
[01:36:17]    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
[01:36:17]    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:36:17] 
---
[01:36:17] ---- [ui (nll)] ui/issues/issue-10291.rs stdout ----
[01:36:17] diff of stderr:
[01:36:17] 
[01:36:17] 3    |
[01:36:17] 4 LL | fn test<'x>(x: &'x isize) {
[01:36:17] 5    |         -- lifetime `'x` defined here
[01:36:17] - LL |     drop::<Box<for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
[01:36:17] + LL |     drop::<Box<dyn for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
[01:36:17] 7 LL |         x
[01:36:17] 8    |         ^ returning this value requires that `'x` must outlive `'static`
[01:36:17] 
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10291.nll/issue-10291.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args issues/issue-10291.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-10291.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10291.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-10291.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/issues/issue-10291.rs:3:9
[01:36:17]    |
[01:36:17] LL | fn test<'x>(x: &'x isize) {
[01:36:17]    |         -- lifetime `'x` defined here
[01:36:17] LL |     drop::<Box<dyn for<'z> FnMut(&'z isize) -> &'z isize>>(Box::new(|z| {
[01:36:17] LL |         x //~ ERROR E0312
[01:36:17]    |         ^ returning this value requires that `'x` must outlive `'static`
[01:36:17] error: aborting due to previous error
[01:36:17] 
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] 
[01:36:17] ---- [ui (nll)] ui/kindck/kindck-impl-type-params.rs stdout ----
[01:36:17] diff of stderr:
[01:36:17] 
[01:36:17] 1 error[E0277]: `T` cannot be sent between threads safely
[01:36:17] 2   --> $DIR/kindck-impl-type-params.rs:18:13
[01:36:17] 3    |
[01:36:17] - LL |     let a = &t as &Gettable<T>;
[01:36:17] + LL |     let a = &t as &dyn Gettable<T>;
[01:36:17] 5    |             ^^ `T` cannot be sent between threads safely
[01:36:17] 7    = help: the trait `std::marker::Send` is not implemented for `T`
[01:36:17] 
[01:36:17] 12 error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
[01:36:17] 13   --> $DIR/kindck-impl-type-params.rs:18:13
[01:36:17] 13   --> $DIR/kindck-impl-type-params.rs:18:13
[01:36:17] 14    |
[01:36:17] - LL |     let a = &t as &Gettable<T>;
[01:36:17] + LL |     let a = &t as &dyn Gettable<T>;
[01:36:17] 16    |             ^^ the trait `std::marker::Copy` is not implemented for `T`
[01:36:17] 18    = help: consider adding a `where T: std::marker::Copy` bound
[01:36:17] 
[01:36:17] 
[01:36:17] 20    = note: required for the cast to the object type `dyn Gettable<T>`
[01:36:17] 21 
[01:36:17] 22 error[E0277]: `T` cannot be sent between threads safely
[01:36:17] -   --> $DIR/kindck-impl-type-params.rs:25:27
[01:36:17] +   --> $DIR/kindck-impl-type-params.rs:25:31
[01:36:17] 24    |
[01:36:17] - LL |     let a: &Gettable<T> = &t;
[01:36:17] -    |                           ^^ `T` cannot be sent between threads safely
[01:36:17] + LL |     let a: &dyn Gettable<T> = &t;
[01:36:17] +    |                               ^^ `T` cannot be sent between threads safely
[01:36:17] 28    = help: the trait `std::marker::Send` is not implemented for `T`
[01:36:17] 29    = help: consider adding a `where T: std::marker::Send` bound
[01:36:17] 
[01:36:17] 
[01:36:17] 31    = note: required for the cast to the object type `dyn Gettable<T>`
[01:36:17] 33 error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
[01:36:17] -   --> $DIR/kindck-impl-type-params.rs:25:27
[01:36:17] +   --> $DIR/kindck-impl-type-params.rs:25:31
[01:36:17] 35    |
[01:36:17] 35    |
[01:36:17] - LL |     let a: &Gettable<T> = &t;
[01:36:17] -    |                           ^^ the trait `std::marker::Copy` is not implemented for `T`
[01:36:17] + LL |     let a: &dyn Gettable<T> = &t;
[01:36:17] +    |                               ^^ the trait `std::marker::Copy` is not implemented for `T`
[01:36:17] 39    = help: consider adding a `where T: std::marker::Copy` bound
[01:36:17] 39    = help: consider adding a `where T: std::marker::Copy` bound
[01:36:17] 40    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
[01:36:17] 43 error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
[01:36:17] 44   --> $DIR/kindck-impl-type-params.rs:38:13
[01:36:17] 45    |
[01:36:17] 45    |
[01:36:17] - LL |     let a = t as Box<Gettable<String>>;
[01:36:17] + LL |     let a = t as Box<dyn Gettable<String>>;
[01:36:17] 47    |             ^ the trait `std::marker::Copy` is not implemented for `std::string::String`
[01:36:17] 48    |
[01:36:17] 49    = note: required because of the requirements on the impl of `Gettable<std::string::String>` for `S<std::string::String>`
[01:36:17] 
[01:36:17] 50    = note: required for the cast to the object type `dyn Gettable<std::string::String>`
[01:36:17] 51 
[01:36:17] 52 error[E0277]: the trait bound `foo3::Foo: std::marker::Copy` is not satisfied
[01:36:17] -   --> $DIR/kindck-impl-type-params.rs:46:33
[01:36:17] +   --> $DIR/kindck-impl-type-params.rs:46:37
[01:36:17] 54    |
[01:36:17] - LL |     let a: Box<Gettable<Foo>> = t;
[01:36:17] -    |                                 ^ the trait `std::marker::Copy` is not implemented for `foo3::Foo`
[01:36:17] + LL |     let a: Box<dyn Gettable<Foo>> = t;
[01:36:17] +    |                                     ^ the trait `std::marker::Copy` is not implemented for `foo3::Foo`
[01:36:17] 57    |
[01:36:17] 58    = note: required because of the requirements on the impl of `Gettable<foo3::Foo>` for `S<foo3::Foo>`
[01:36:17] 59    = note: required for the cast to the object type `dyn Gettable<foo3::Foo>`
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/kindck-impl-type-params.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args kindck/kindck-impl-type-params.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-impl-type-params.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-impl-type-params.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error[E0277]: `T` cannot be sent between threads safely
[01:36:17]   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:18:13
[01:36:17]    |
[01:36:17] LL |     let a = &t as &dyn Gettable<T>;
[01:36:17]    |             ^^ `T` cannot be sent between threads safely
[01:36:17]    = help: the trait `std::marker::Send` is not implemented for `T`
[01:36:17]    = help: consider adding a `where T: std::marker::Send` bound
[01:36:17]    = help: consider adding a `where T: std::marker::Send` bound
[01:36:17]    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
[01:36:17]    = note: required for the cast to the object type `dyn Gettable<T>`
[01:36:17] error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
[01:36:17]   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:18:13
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL |     let a = &t as &dyn Gettable<T>;
[01:36:17]    |             ^^ the trait `std::marker::Copy` is not implemented for `T`
[01:36:17]    = help: consider adding a `where T: std::marker::Copy` bound
[01:36:17]    = help: consider adding a `where T: std::marker::Copy` bound
[01:36:17]    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
[01:36:17]    = note: required for the cast to the object type `dyn Gettable<T>`
[01:36:17] 
[01:36:17] error[E0277]: `T` cannot be sent between threads safely
[01:36:17]   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:25:31
[01:36:17]    |
[01:36:17] LL |     let a: &dyn Gettable<T> = &t;
[01:36:17]    |                               ^^ `T` cannot be sent between threads safely
[01:36:17]    = help: the trait `std::marker::Send` is not implemented for `T`
[01:36:17]    = help: consider adding a `where T: std::marker::Send` bound
[01:36:17]    = help: consider adding a `where T: std::marker::Send` bound
[01:36:17]    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
[01:36:17]    = note: required for the cast to the object type `dyn Gettable<T>`
[01:36:17] error[E0277]: the trait bound `T: std::marker::Copy` is not satisfied
[01:36:17]   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:25:31
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL |     let a: &dyn Gettable<T> = &t;
[01:36:17]    |                               ^^ the trait `std::marker::Copy` is not implemented for `T`
[01:36:17]    = help: consider adding a `where T: std::marker::Copy` bound
[01:36:17]    = help: consider adding a `where T: std::marker::Copy` bound
[01:36:17]    = note: required because of the requirements on the impl of `Gettable<T>` for `S<T>`
[01:36:17]    = note: required for the cast to the object type `dyn Gettable<T>`
[01:36:17] error[E0277]: the trait bound `std::string::String: std::marker::Copy` is not satisfied
[01:36:17]   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:38:13
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL |     let a = t as Box<dyn Gettable<String>>;
[01:36:17]    |             ^ the trait `std::marker::Copy` is not implemented for `std::string::String`
[01:36:17]    |
[01:36:17]    = note: required because of the requirements on the impl of `Gettable<std::string::String>` for `S<std::string::String>`
[01:36:17]    = note: required for the cast to the object type `dyn Gettable<std::string::String>`
[01:36:17] 
[01:36:17] error[E0277]: the trait bound `foo3::Foo: std::marker::Copy` is not satisfied
[01:36:17]   --> /checkout/src/test/ui/kindck/kindck-impl-type-params.rs:46:37
[01:36:17]    |
[01:36:17] LL |     let a: Box<dyn Gettable<Foo>> = t;
[01:36:17]    |                                     ^ the trait `std::marker::Copy` is not implemented for `foo3::Foo`
[01:36:17]    |
[01:36:17]    = note: required because of the requirements on the impl of `Gettable<foo3::Foo>` for `S<foo3::Foo>`
[01:36:17]    = note: required for the cast to the object type `dyn Gettable<foo3::Foo>`
[01:36:17] error: aborting due to 6 previous errors
[01:36:17] 
[01:36:17] For more information about this error, try `rustc --explain E0277`.
[01:36:17] 
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] 
[01:36:17] ---- [ui (nll)] ui/kindck/kindck-send-object1.rs stdout ----
[01:36:17] diff of stderr:
[01:36:17] 
[01:36:17] 1 error[E0277]: `(dyn Dummy + 'a)` cannot be shared between threads safely
[01:36:17] 2   --> $DIR/kindck-send-object1.rs:10:5
[01:36:17] 3    |
[01:36:17] - LL |     assert_send::<&'a Dummy>();
[01:36:17] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be shared between threads safely
[01:36:17] + LL |     assert_send::<&'a dyn Dummy>();
[01:36:17] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be shared between threads safely
[01:36:17] 6    |
[01:36:17] 7    = help: the trait `std::marker::Sync` is not implemented for `(dyn Dummy + 'a)`
[01:36:17] 8    = note: required because of the requirements on the impl of `std::marker::Send` for `&'a (dyn Dummy + 'a)`
[01:36:17] 
[01:36:17] 15 error[E0277]: `(dyn Dummy + 'a)` cannot be sent between threads safely
[01:36:17] 16   --> $DIR/kindck-send-object1.rs:29:5
[01:36:17] 17    |
[01:36:17] - LL |     assert_send::<Box<Dummy+'a>>();
[01:36:17] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be sent between threads safely
[01:36:17] + LL |     assert_send::<Box<dyn Dummy + 'a>>();
[01:36:17] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be sent between threads safely
[01:36:17] 20    |
[01:36:17] 21    = help: the trait `std::marker::Send` is not implemented for `(dyn Dummy + 'a)`
[01:36:17] 22    = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn Dummy + 'a)>`
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.nll/kindck-send-object1.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args kindck/kindck-send-object1.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/kindck/kindck-send-object1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/kindck/kindck-send-object1.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error[E0277]: `(dyn Dummy + 'a)` cannot be shared between threads safely
[01:36:17]   --> /checkout/src/test/ui/kindck/kindck-send-object1.rs:10:5
[01:36:17]    |
[01:36:17] LL |     assert_send::<&'a dyn Dummy>();
[01:36:17]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be shared between threads safely
[01:36:17]    |
[01:36:17]    = help: the trait `std::marker::Sync` is not implemented for `(dyn Dummy + 'a)`
[01:36:17]    = note: required because of the requirements on the impl of `std::marker::Send` for `&'a (dyn Dummy + 'a)`
[01:36:17] note: required by `assert_send`
[01:36:17]   --> /checkout/src/test/ui/kindck/kindck-send-object1.rs:5:1
[01:36:17]    |
[01:36:17] LL | fn assert_send<T:Send+'static>() { }
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0277]: `(dyn Dummy + 'a)` cannot be sent between threads safely
[01:36:17]   --> /checkout/src/test/ui/kindck/kindck-send-object1.rs:29:5
[01:36:17]    |
[01:36:17] LL |     assert_send::<Box<dyn Dummy + 'a>>();
[01:36:17]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `(dyn Dummy + 'a)` cannot be sent between threads safely
[01:36:17]    |
[01:36:17]    = help: the trait `std::marker::Send` is not implemented for `(dyn Dummy + 'a)`
[01:36:17]    = note: required because of the requirements on the impl of `std::marker::Send` for `std::ptr::Unique<(dyn Dummy + 'a)>`
[01:36:17]    = note: required because it appears within the type `std::boxed::Box<(dyn Dummy + 'a)>`
[01:36:17] note: required by `assert_send`
[01:36:17]   --> /checkout/src/test/ui/kindck/kindck-send-object1.rs:5:1
[01:36:17]    |
[01:36:17] LL | fn assert_send<T:Send+'static>() { }
[01:36:17] 
[01:36:17] error: aborting due to 2 previous errors
[01:36:17] 
[01:36:17] For more information about this error, try `rustc --explain E0277`.
---
[01:36:17] 
[01:36:17] 1 error[E0521]: borrowed data escapes outside of function
[01:36:17] 2   --> $DIR/lifetime-bound-will-change-warning.rs:34:5
[01:36:17] 3    |
[01:36:17] - LL | fn test2<'a>(x: &'a Box<Fn()+'a>) {
[01:36:17] + LL | fn test2<'a>(x: &'a Box<dyn Fn() + 'a>) {
[01:36:17] 5    |              - `x` is a reference that is only valid in the function body
[01:36:17] 6 LL |     // but ref_obj will not, so warn.
[01:36:17] 7 LL |     ref_obj(x)
[01:36:17] 10 error[E0521]: borrowed data escapes outside of function
[01:36:17] 11   --> $DIR/lifetime-bound-will-change-warning.rs:39:5
[01:36:17] 12    |
[01:36:17] 12    |
[01:36:17] - LL | fn test2cc<'a>(x: &'a Box<Fn()+'a>) {
[01:36:17] + LL | fn test2cc<'a>(x: &'a Box<dyn Fn() + 'a>) {
[01:36:17] 14    |                - `x` is a reference that is only valid in the function body
[01:36:17] 15 LL |     // same as test2, but cross crate
[01:36:17] 16 LL |     lib::ref_obj(x)
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-bound-will-change-warning.nll/lifetime-bound-will-change-warning.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args lifetimes/lifetime-bound-will-change-warning.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-bound-will-change-warning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-bound-will-change-warning.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-bound-will-change-warning.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error[E0521]: borrowed data escapes outside of function
[01:36:17]   --> /checkout/src/test/ui/lifetimes/lifetime-bound-will-change-warning.rs:34:5
[01:36:17]    |
[01:36:17] LL | fn test2<'a>(x: &'a Box<dyn Fn() + 'a>) {
[01:36:17]    |              - `x` is a reference that is only valid in the function body
[01:36:17] LL |     // but ref_obj will not, so warn.
[01:36:17] LL |     ref_obj(x) //~ ERROR mismatched types
[01:36:17]    |     ^^^^^^^^^^ `x` escapes the function body here
[01:36:17] error[E0521]: borrowed data escapes outside of function
[01:36:17]   --> /checkout/src/test/ui/lifetimes/lifetime-bound-will-change-warning.rs:39:5
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | fn test2cc<'a>(x: &'a Box<dyn Fn() + 'a>) {
[01:36:17]    |                - `x` is a reference that is only valid in the function body
[01:36:17] LL |     // same as test2, but cross crate
[01:36:17] LL |     lib::ref_obj(x) //~ ERROR mismatched types
[01:36:17]    |     ^^^^^^^^^^^^^^^ `x` escapes the function body here
[01:36:17] error: aborting due to 2 previous errors
[01:36:17] 
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] 
[01:36:17] ---- [ui (nll)] ui/lifetimes/lifetime-errors/ex3-both-anon-regions-using-trait-objects.rs stdout ----
[01:36:17] diff of stderr:
[01:36:17] 
[01:36:17] 1 error[E0596]: cannot borrow `y` as mutable, as it is not declared as mutable
[01:36:17] 2   --> $DIR/ex3-both-anon-regions-using-trait-objects.rs:2:3
[01:36:17] 3    |
[01:36:17] - LL | fn foo(x:Box<Fn(&u8, &u8)> , y: Vec<&u8>, z: &u8) {
[01:36:17] -    |                              - help: consider changing this to be mutable: `mut y`
[01:36:17] + LL | fn foo(x:Box<dyn Fn(&u8, &u8)> , y: Vec<&u8>, z: &u8) {
[01:36:17] +    |                                  - help: consider changing this to be mutable: `mut y`
[01:36:17] 6 LL |   y.push(z);
[01:36:17] 7    |   ^ cannot borrow as mutable
[01:36:17] 
[01:36:17] 9 error: lifetime may not live long enough
[01:36:17] 10   --> $DIR/ex3-both-anon-regions-using-trait-objects.rs:2:3
[01:36:17] 11    |
[01:36:17] 11    |
[01:36:17] - LL | fn foo(x:Box<Fn(&u8, &u8)> , y: Vec<&u8>, z: &u8) {
[01:36:17] -    |                                     |
[01:36:17] -    |                                     let's call the lifetime of this reference `'2`
[01:36:17] -    |                                     let's call the lifetime of this reference `'2`
[01:36:17] + LL | fn foo(x:Box<dyn Fn(&u8, &u8)> , y: Vec<&u8>, z: &u8) {
[01:36:17] +    |                                         |
[01:36:17] +    |                                         let's call the lifetime of this reference `'2`
[01:36:17] +    |                                         let's call the lifetime of this reference `'2`
[01:36:17] 16 LL |   y.push(z);
[01:36:17] 17    |   ^^^^^^^^^ argument requires that `'1` must outlive `'2`
[01:36:17] 
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-using-trait-objects.nll/ex3-both-anon-regions-using-trait-objects.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args lifetimes/lifetime-errors/ex3-both-anon-regions-using-trait-objects.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-using-trait-objects.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-using-trait-objects.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-using-trait-objects.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error[E0596]: cannot borrow `y` as mutable, as it is not declared as mutable
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | fn foo(x:Box<dyn Fn(&u8, &u8)> , y: Vec<&u8>, z: &u8) {
[01:36:17]    |                                  - help: consider changing this to be mutable: `mut y`
[01:36:17] LL |   y.push(z); //~ ERROR lifetime mismatch
[01:36:17]    |   ^ cannot borrow as mutable
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/lifetimes/lifetime-errors/ex3-both-anon-regions-using-trait-objects.rs:2:3
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | fn foo(x:Box<dyn Fn(&u8, &u8)> , y: Vec<&u8>, z: &u8) {
[01:36:17]    |                                         |
[01:36:17]    |                                         let's call the lifetime of this reference `'2`
[01:36:17]    |                                         let's call the lifetime of this reference `'2`
[01:36:17] LL |   y.push(z); //~ ERROR lifetime mismatch
[01:36:17]    |   ^^^^^^^^^ argument requires that `'1` must outlive `'2`
[01:36:17] error: aborting due to 2 previous errors
[01:36:17] 
[01:36:17] For more information about this error, try `rustc --explain E0596`.
[01:36:17] 
---
[01:36:17] 
[01:36:17] 1 error: lifetime may not live long enough
[01:36:17] 2   --> $DIR/object-lifetime-default-elision.rs:71:5
[01:36:17] 3    |
[01:36:17] - LL | fn load3<'a,'b>(ss: &'a SomeTrait) -> &'b SomeTrait {
[01:36:17] + LL | fn load3<'a,'b>(ss: &'a dyn SomeTrait) -> &'b dyn SomeTrait {
[01:36:17] 6    |          |
[01:36:17] 7    |          lifetime `'a` defined here
[01:36:17] 
[01:36:17] 
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-elision.nll/object-lifetime-default-elision.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-elision.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-elision.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-elision.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-elision.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-elision.rs:71:5
[01:36:17]    |
[01:36:17] LL | fn load3<'a,'b>(ss: &'a dyn SomeTrait) -> &'b dyn SomeTrait {
[01:36:17]    |          |
[01:36:17]    |          lifetime `'a` defined here
[01:36:17] ...
[01:36:17] LL |     ss
[01:36:17] LL |     ss
[01:36:17]    |     ^^ returning this value requires that `'a` must outlive `'b`
[01:36:17] error: aborting due to previous error
[01:36:17] 
[01:36:17] 
[01:36:17] ------------------------------------------
---
[01:36:17] 
[01:36:17] 1 error: lifetime may not live long enough
[01:36:17] 2   --> $DIR/object-lifetime-default-from-rptr-box-error.rs:15:5
[01:36:17] 3    |
[01:36:17] - LL | fn c<'a>(t: &'a Box<Test+'a>, mut ss: SomeStruct<'a>) {
[01:36:17] + LL | fn c<'a>(t: &'a Box<dyn Test+'a>, mut ss: SomeStruct<'a>) {
[01:36:17] 5    |      -- lifetime `'a` defined here
[01:36:17] 6 LL |     ss.t = t;
[01:36:17] 7    |     ^^^^^^^^ assignment requires that `'a` must outlive `'static`
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-rptr-box-error.nll/object-lifetime-default-from-rptr-box-error.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-from-rptr-box-error.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-from-rptr-box-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-rptr-box-error.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-rptr-box-error.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-from-rptr-box-error.rs:15:5
[01:36:17]    |
[01:36:17] LL | fn c<'a>(t: &'a Box<dyn Test+'a>, mut ss: SomeStruct<'a>) {
[01:36:17]    |      -- lifetime `'a` defined here
[01:36:17] LL |     ss.t = t; //~ ERROR mismatched types
[01:36:17]    |     ^^^^^^^^ assignment requires that `'a` must outlive `'static`
[01:36:17] error: aborting due to previous error
[01:36:17] 
[01:36:17] 
[01:36:17] ------------------------------------------
---
[01:36:17] 
[01:36:17] 1 error[E0621]: explicit lifetime required in the type of `ss`
[01:36:17] 2   --> $DIR/object-lifetime-default-from-box-error.rs:18:5
[01:36:17] 3    |
[01:36:17] - LL | fn load(ss: &mut SomeStruct) -> Box<SomeTrait> {
[01:36:17] + LL | fn load(ss: &mut SomeStruct) -> Box<dyn SomeTrait> {
[01:36:17] 5    |             --------------- help: add explicit lifetime `'static` to the type of `ss`: `&mut SomeStruct<'static>`
[01:36:17] 6 ...
[01:36:17] 7 LL |     ss.r
[01:36:17] 16 error[E0621]: explicit lifetime required in the type of `ss`
[01:36:17] 17   --> $DIR/object-lifetime-default-from-box-error.rs:31:5
[01:36:17] 18    |
[01:36:17] 18    |
[01:36:17] - LL | fn store1<'b>(ss: &mut SomeStruct, b: Box<SomeTrait+'b>) {
[01:36:17] + LL | fn store1<'b>(ss: &mut SomeStruct, b: Box<dyn SomeTrait+'b>) {
[01:36:17] 20    |                   --------------- help: add explicit lifetime `'b` to the type of `ss`: `&mut SomeStruct<'b>`
[01:36:17] 21 ...
[01:36:17] 22 LL |     ss.r = b;
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error.nll/object-lifetime-default-from-box-error.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-from-box-error.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-from-box-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-box-error.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error[E0621]: explicit lifetime required in the type of `ss`
[01:36:17]   --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-from-box-error.rs:18:5
[01:36:17]    |
[01:36:17] LL | fn load(ss: &mut SomeStruct) -> Box<dyn SomeTrait> {
[01:36:17]    |             --------------- help: add explicit lifetime `'static` to the type of `ss`: `&mut SomeStruct<'static>`
[01:36:17] ...
[01:36:17] LL |     ss.r //~ ERROR explicit lifetime required in the type of `ss` [E0621]
[01:36:17] 
[01:36:17] error[E0507]: cannot move out of borrowed content
[01:36:17]   --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-from-box-error.rs:18:5
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL |     ss.r //~ ERROR explicit lifetime required in the type of `ss` [E0621]
[01:36:17]    |     ^^^^ cannot move out of borrowed content
[01:36:17] error[E0621]: explicit lifetime required in the type of `ss`
[01:36:17]   --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-from-box-error.rs:31:5
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | fn store1<'b>(ss: &mut SomeStruct, b: Box<dyn SomeTrait+'b>) {
[01:36:17]    |                   --------------- help: add explicit lifetime `'b` to the type of `ss`: `&mut SomeStruct<'b>`
[01:36:17] ...
[01:36:17] LL |     ss.r = b; //~ ERROR explicit lifetime required in the type of `ss` [E0621]
[01:36:17]    |     ^^^^ lifetime `'b` required
[01:36:17] error: aborting due to 3 previous errors
[01:36:17] 
[01:36:17] Some errors have detailed explanations: E0507, E0621.
[01:36:17] For more information about an error, try `rustc --explain E0507`.
---
[01:36:17] 
[01:36:17] 1 error: lifetime may not live long enough
[01:36:17] 2   --> $DIR/object-lifetime-default-from-rptr-struct-error.rs:21:5
[01:36:17] 3    |
[01:36:17] - LL | fn c<'a>(t: &'a MyBox<Test+'a>, mut ss: SomeStruct<'a>) {
[01:36:17] + LL | fn c<'a>(t: &'a MyBox<dyn Test+'a>, mut ss: SomeStruct<'a>) {
[01:36:17] 5    |      -- lifetime `'a` defined here
[01:36:17] 6 LL |     ss.t = t;
[01:36:17] 7    |     ^^^^^^^^ assignment requires that `'a` must outlive `'static`
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-rptr-struct-error.nll/object-lifetime-default-from-rptr-struct-error.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-from-rptr-struct-error.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-from-rptr-struct-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-rptr-struct-error.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-from-rptr-struct-error.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-from-rptr-struct-error.rs:21:5
[01:36:17]    |
[01:36:17] LL | fn c<'a>(t: &'a MyBox<dyn Test+'a>, mut ss: SomeStruct<'a>) {
[01:36:17]    |      -- lifetime `'a` defined here
[01:36:17] LL |     ss.t = t; //~ ERROR mismatched types
[01:36:17]    |     ^^^^^^^^ assignment requires that `'a` must outlive `'static`
[01:36:17] error: aborting due to previous error
[01:36:17] 
[01:36:17] 
[01:36:17] ------------------------------------------
---
[01:36:17] 
[01:36:17] 1 error: lifetime may not live long enough
[01:36:17] 2   --> $DIR/object-lifetime-default-mybox.rs:27:5
[01:36:17] 3    |
[01:36:17] - LL | fn load1<'a,'b>(a: &'a MyBox<SomeTrait>,
[01:36:17] + LL | fn load1<'a,'b>(a: &'a MyBox<dyn SomeTrait>,
[01:36:17] 6    |          |
[01:36:17] 7    |          lifetime `'a` defined here
[01:36:17] 
[01:36:17] 12 error[E0521]: borrowed data escapes outside of function
[01:36:17] 12 error[E0521]: borrowed data escapes outside of function
[01:36:17] 13   --> $DIR/object-lifetime-default-mybox.rs:31:5
[01:36:17] 14    |
[01:36:17] - LL | fn load2<'a>(ss: &MyBox<SomeTrait+'a>) -> MyBox<SomeTrait+'a> {
[01:36:17] + LL | fn load2<'a>(ss: &MyBox<dyn SomeTrait + 'a>) -> MyBox<dyn SomeTrait + 'a> {
[01:36:17] 16    |              -- `ss` is a reference that is only valid in the function body
[01:36:17] 17 LL |     load0(ss)
[01:36:17] 18    |     ^^^^^^^^^ `ss` escapes the function body here
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-mybox.nll/object-lifetime-default-mybox.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-mybox.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-mybox.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-mybox.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-mybox.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-mybox.rs:27:5
[01:36:17]    |
[01:36:17] LL | fn load1<'a,'b>(a: &'a MyBox<dyn SomeTrait>,
[01:36:17]    |          |
[01:36:17]    |          lifetime `'a` defined here
[01:36:17] ...
[01:36:17] LL |     a //~ ERROR lifetime mismatch
[01:36:17] LL |     a //~ ERROR lifetime mismatch
[01:36:17]    |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
[01:36:17] error[E0521]: borrowed data escapes outside of function
[01:36:17]   --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-mybox.rs:31:5
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | fn load2<'a>(ss: &MyBox<dyn SomeTrait + 'a>) -> MyBox<dyn SomeTrait + 'a> {
[01:36:17]    |              -- `ss` is a reference that is only valid in the function body
[01:36:17] LL |     load0(ss) //~ ERROR mismatched types
[01:36:17]    |     ^^^^^^^^^ `ss` escapes the function body here
[01:36:17] error: aborting due to 2 previous errors
[01:36:17] 
[01:36:17] 
[01:36:17] ------------------------------------------
---
[01:36:17] 
[01:36:17] 1 error: lifetime may not live long enough
[01:36:17] 2   --> $DIR/region-object-lifetime-2.rs:10:5
[01:36:17] 3    |
[01:36:17] - LL | fn borrowed_receiver_different_lifetimes<'a,'b>(x: &'a Foo) -> &'b () {
[01:36:17] + LL | fn borrowed_receiver_different_lifetimes<'a,'b>(x: &'a dyn Foo) -> &'b () {
[01:36:17] 6    |                                          |
[01:36:17] 7    |                                          lifetime `'a` defined here
[01:36:17] 
[01:36:17] 
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-2.nll/region-object-lifetime-2.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args regions/region-object-lifetime-2.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-object-lifetime-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-2.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-2.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/regions/region-object-lifetime-2.rs:10:5
[01:36:17]    |
[01:36:17] LL | fn borrowed_receiver_different_lifetimes<'a,'b>(x: &'a dyn Foo) -> &'b () {
[01:36:17]    |                                          |
[01:36:17]    |                                          lifetime `'a` defined here
[01:36:17]    |                                          lifetime `'a` defined here
[01:36:17] LL |     x.borrowed() //~ ERROR cannot infer
[01:36:17]    |     ^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'b`
[01:36:17] error: aborting due to previous error
[01:36:17] 
[01:36:17] 
[01:36:17] ------------------------------------------
---
[01:36:17] 
[01:36:17] 1 error: lifetime may not live long enough
[01:36:17] 2   --> $DIR/region-object-lifetime-4.rs:12:5
[01:36:17] 3    |
[01:36:17] - LL | fn borrowed_receiver_related_lifetimes2<'a,'b>(x: &'a (Foo+'b)) -> &'b () {
[01:36:17] + LL | fn borrowed_receiver_related_lifetimes2<'a,'b>(x: &'a (dyn Foo + 'b)) -> &'b () {
[01:36:17] 6    |                                         |
[01:36:17] 7    |                                         lifetime `'a` defined here
[01:36:17] 
[01:36:17] 
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-4.nll/region-object-lifetime-4.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args regions/region-object-lifetime-4.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-object-lifetime-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-4.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-4.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/regions/region-object-lifetime-4.rs:12:5
[01:36:17]    |
[01:36:17] LL | fn borrowed_receiver_related_lifetimes2<'a,'b>(x: &'a (dyn Foo + 'b)) -> &'b () {
[01:36:17] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:36:17] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:36:17] make: *** [check] Error 1
[01:36:17]    |                                         |
[01:36:17]    |                                         lifetime `'a` defined here
[01:36:17]    |                                         lifetime `'a` defined here
[01:36:17] LL |     x.borrowed() //~ ERROR cannot infer
[01:36:17]    |     ^^^^^^^^^^^^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
[01:36:17] error: aborting due to previous error
[01:36:17] 
[01:36:17] 
[01:36:17] ------------------------------------------
---
[01:36:17] 
[01:36:17] 1 error[E0621]: explicit lifetime required in the type of `v`
[01:36:17] 2   --> $DIR/region-object-lifetime-in-coercion.rs:8:12
[01:36:17] 3    |
[01:36:17] - LL | fn a(v: &[u8]) -> Box<Foo + 'static> {
[01:36:17] + LL | fn a(v: &[u8]) -> Box<dyn Foo + 'static> {
[01:36:17] 5    |         ----- help: add explicit lifetime `'static` to the type of `v`: `&'static [u8]`
[01:36:17] - LL |     let x: Box<Foo + 'static> = Box::new(v);
[01:36:17] -    |            ^^^^^^^^^^^^^^^^^^ lifetime `'static` required
[01:36:17] + LL |     let x: Box<dyn Foo + 'static> = Box::new(v);
[01:36:17] 8 
[01:36:17] 9 error[E0621]: explicit lifetime required in the type of `v`
[01:36:17] 10   --> $DIR/region-object-lifetime-in-coercion.rs:14:5
[01:36:17] 
[01:36:17] 
[01:36:17] 11    |
[01:36:17] - LL | fn b(v: &[u8]) -> Box<Foo + 'static> {
[01:36:17] + LL | fn b(v: &[u8]) -> Box<dyn Foo + 'static> {
[01:36:17] 13    |         ----- help: add explicit lifetime `'static` to the type of `v`: `&'static [u8]`
[01:36:17] 14 LL |     Box::new(v)
[01:36:17] 
[01:36:17] 17 error[E0621]: explicit lifetime required in the type of `v`
[01:36:17] 18   --> $DIR/region-object-lifetime-in-coercion.rs:21:5
[01:36:17] 19    |
[01:36:17] 19    |
[01:36:17] - LL | fn c(v: &[u8]) -> Box<Foo> {
[01:36:17] + LL | fn c(v: &[u8]) -> Box<dyn Foo> {
[01:36:17] 21    |         ----- help: add explicit lifetime `'static` to the type of `v`: `&'static [u8]`
[01:36:17] 22 ...
[01:36:17] 23 LL |     Box::new(v)
[01:36:17] 26 error: lifetime may not live long enough
[01:36:17] 27   --> $DIR/region-object-lifetime-in-coercion.rs:26:5
[01:36:17] 28    |
[01:36:17] 28    |
[01:36:17] - LL | fn d<'a,'b>(v: &'a [u8]) -> Box<Foo+'b> {
[01:36:17] + LL | fn d<'a,'b>(v: &'a [u8]) -> Box<dyn Foo+'b> {
[01:36:17] 31    |      |
[01:36:17] 32    |      lifetime `'a` defined here
[01:36:17] 
[01:36:17] 
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion.nll/region-object-lifetime-in-coercion.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args regions/region-object-lifetime-in-coercion.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-object-lifetime-in-coercion.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error[E0621]: explicit lifetime required in the type of `v`
[01:36:17]   --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:8:12
[01:36:17]    |
[01:36:17] LL | fn a(v: &[u8]) -> Box<dyn Foo + 'static> {
[01:36:17]    |         ----- help: add explicit lifetime `'static` to the type of `v`: `&'static [u8]`
[01:36:17] LL |     let x: Box<dyn Foo + 'static> = Box::new(v);
[01:36:17] 
[01:36:17] error[E0621]: explicit lifetime required in the type of `v`
[01:36:17]   --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:14:5
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | fn b(v: &[u8]) -> Box<dyn Foo + 'static> {
[01:36:17]    |         ----- help: add explicit lifetime `'static` to the type of `v`: `&'static [u8]`
[01:36:17] LL |     Box::new(v)
[01:36:17] 
[01:36:17] error[E0621]: explicit lifetime required in the type of `v`
[01:36:17]   --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:21:5
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | fn c(v: &[u8]) -> Box<dyn Foo> {
[01:36:17]    |         ----- help: add explicit lifetime `'static` to the type of `v`: `&'static [u8]`
[01:36:17] LL |     Box::new(v)
[01:36:17]    |     ^^^^^^^^^^^ lifetime `'static` required
[01:36:17] 
[01:36:17] error: lifetime may not live long enough
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/regions/region-object-lifetime-in-coercion.rs:26:5
[01:36:17]    |
[01:36:17] LL | fn d<'a,'b>(v: &'a [u8]) -> Box<dyn Foo+'b> {
[01:36:17]    |      |
[01:36:17]    |      lifetime `'a` defined here
[01:36:17] LL |     Box::new(v)
[01:36:17] LL |     Box::new(v)
[01:36:17]    |     ^^^^^^^^^^^ returning this value requires that `'a` must outlive `'b`
[01:36:17] error: aborting due to 4 previous errors
[01:36:17] 
[01:36:17] For more information about this error, try `rustc --explain E0621`.
[01:36:17] 
---
[01:36:17] 
[01:36:17] 1 error: lifetime may not live long enough
[01:36:17] 2   --> $DIR/regions-close-object-into-object-2.rs:10:5
[01:36:17] 3    |
[01:36:17] - LL | fn g<'a, T: 'static>(v: Box<A<T>+'a>) -> Box<X+'static> {
[01:36:17] + LL | fn g<'a, T: 'static>(v: Box<dyn A<T> + 'a>) -> Box<dyn X + 'static> {
[01:36:17] 5    |      -- lifetime `'a` defined here
[01:36:17] - LL |     box B(&*v) as Box<X>
[01:36:17] -    |     ^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
[01:36:17] + LL |     box B(&*v) as Box<dyn X>
[01:36:17] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
[01:36:17] 8 
[01:36:17] 9 error[E0515]: cannot return value referencing local data `*v`
[01:36:17] 
[01:36:17] 11    |
[01:36:17] 11    |
[01:36:17] - LL |     box B(&*v) as Box<X>
[01:36:17] -    |     ^^^^^^---^^^^^^^^^^^
[01:36:17] + LL |     box B(&*v) as Box<dyn X>
[01:36:17] +    |     ^^^^^^---^^^^^^^^^^^^^^^
[01:36:17] 14    |     |     |
[01:36:17] 15    |     |     `*v` is borrowed here
[01:36:17] 
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-2.nll/regions-close-object-into-object-2.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args regions/regions-close-object-into-object-2.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-object-into-object-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-2.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-2.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/regions/regions-close-object-into-object-2.rs:10:5
[01:36:17]    |
[01:36:17] LL | fn g<'a, T: 'static>(v: Box<dyn A<T> + 'a>) -> Box<dyn X + 'static> {
[01:36:17]    |      -- lifetime `'a` defined here
[01:36:17] LL |     box B(&*v) as Box<dyn X> //~ ERROR cannot infer
[01:36:17]    |     ^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
[01:36:17] 
[01:36:17] error[E0515]: cannot return value referencing local data `*v`
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL |     box B(&*v) as Box<dyn X> //~ ERROR cannot infer
[01:36:17]    |     ^^^^^^---^^^^^^^^^^^^^^^
[01:36:17]    |     |     |
[01:36:17]    |     |     `*v` is borrowed here
[01:36:17] 
[01:36:17] error: aborting due to 2 previous errors
[01:36:17] 
[01:36:17] For more information about this error, try `rustc --explain E0515`.
[01:36:17] For more information about this error, try `rustc --explain E0515`.
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] 
[01:36:17] ---- [ui (nll)] ui/regions/regions-close-object-into-object-4.rs stdout ----
[01:36:17] diff of stderr:
[01:36:17] 
[01:36:17] 1 error[E0310]: the parameter type `U` may not live long enough
[01:36:17] 3    |
[01:36:17] 3    |
[01:36:17] - LL |     box B(&*v) as Box<X>
[01:36:17] + LL |     box B(&*v) as Box<dyn X>
[01:36:17] 6    |
[01:36:17] 6    |
[01:36:17] 7    = help: consider adding an explicit lifetime bound `U: 'static`...
[01:36:17] 9 error: lifetime may not live long enough
[01:36:17] 10   --> $DIR/regions-close-object-into-object-4.rs:10:5
[01:36:17] 11    |
[01:36:17] 11    |
[01:36:17] - LL | fn i<'a, T, U>(v: Box<A<U>+'a>) -> Box<X+'static> {
[01:36:17] + LL | fn i<'a, T, U>(v: Box<dyn A<U>+'a>) -> Box<dyn X + 'static> {
[01:36:17] 13    |      -- lifetime `'a` defined here
[01:36:17] - LL |     box B(&*v) as Box<X>
[01:36:17] -    |     ^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
[01:36:17] + LL |     box B(&*v) as Box<dyn X>
[01:36:17] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
[01:36:17] 16 
[01:36:17] 17 error[E0515]: cannot return value referencing local data `*v`
[01:36:17] 
[01:36:17] 19    |
[01:36:17] 19    |
[01:36:17] - LL |     box B(&*v) as Box<X>
[01:36:17] -    |     ^^^^^^---^^^^^^^^^^^
[01:36:17] + LL |     box B(&*v) as Box<dyn X>
[01:36:17] +    |     ^^^^^^---^^^^^^^^^^^^^^^
[01:36:17] 22    |     |     |
[01:36:17] 23    |     |     `*v` is borrowed here
[01:36:17] 
[01:36:17] 
[01:36:17] 26 error[E0310]: the parameter type `U` may not live long enough
[01:36:17] 28    |
[01:36:17] 28    |
[01:36:17] - LL |     box B(&*v) as Box<X>
[01:36:17] + LL |     box B(&*v) as Box<dyn X>
[01:36:17] 31    |
[01:36:17] 31    |
[01:36:17] 32    = help: consider adding an explicit lifetime bound `U: 'static`...
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-4.nll/regions-close-object-into-object-4.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args regions/regions-close-object-into-object-4.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-object-into-object-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-4.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-object-into-object-4.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error[E0310]: the parameter type `U` may not live long enough
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL |     box B(&*v) as Box<dyn X> //~ ERROR cannot infer
[01:36:17]    |
[01:36:17]    |
[01:36:17]    = help: consider adding an explicit lifetime bound `U: 'static`...
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/regions/regions-close-object-into-object-4.rs:10:5
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL | fn i<'a, T, U>(v: Box<dyn A<U>+'a>) -> Box<dyn X + 'static> {
[01:36:17]    |      -- lifetime `'a` defined here
[01:36:17] LL |     box B(&*v) as Box<dyn X> //~ ERROR cannot infer
[01:36:17]    |     ^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
[01:36:17] 
[01:36:17] error[E0515]: cannot return value referencing local data `*v`
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL |     box B(&*v) as Box<dyn X> //~ ERROR cannot infer
[01:36:17]    |     ^^^^^^---^^^^^^^^^^^^^^^
[01:36:17]    |     |     |
[01:36:17]    |     |     `*v` is borrowed here
[01:36:17] 
[01:36:17] 
[01:36:17] error[E0310]: the parameter type `U` may not live long enough
[01:36:17]    |
[01:36:17]    |
[01:36:17] LL |     box B(&*v) as Box<dyn X> //~ ERROR cannot infer
[01:36:17]    |
[01:36:17]    |
[01:36:17]    = help: consider adding an explicit lifetime bound `U: 'static`...
[01:36:17] error: aborting due to 4 previous errors
[01:36:17] 
[01:36:17] Some errors have detailed explanations: E0310, E0515.
[01:36:17] For more information about an error, try `rustc --explain E0310`.
---
[01:36:17] 
[01:36:17] 1 error: lifetime may not live long enough
[01:36:17] 2   --> $DIR/regions-close-over-type-parameter-multiple.rs:20:5
[01:36:17] 3    |
[01:36:17] - LL | fn make_object_bad<'a,'b,'c,A:SomeTrait+'a+'b>(v: A) -> Box<SomeTrait+'c> {
[01:36:17] + LL | fn make_object_bad<'a,'b,'c,A:SomeTrait+'a+'b>(v: A) -> Box<dyn SomeTrait + 'c> {
[01:36:17] 5    |                    --    -- lifetime `'c` defined here
[01:36:17] 7    |                    lifetime `'a` defined here
[01:36:17] 
[01:36:17] 
[01:36:17] 8 LL |     // A outlives 'a AND 'b...but not 'c.
[01:36:17] - LL |     box v as Box<SomeTrait+'a>
[01:36:17] -    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'c`
[01:36:17] + LL |     box v as Box<dyn SomeTrait + 'a>
[01:36:17] +    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'c`
[01:36:17] 12 error: aborting due to previous error
[01:36:17] 13 
[01:36:17] 
[01:36:17] 
[01:36:17] 
[01:36:17] The actual stderr differed from the expected stderr.
[01:36:17] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-multiple.nll/regions-close-over-type-parameter-multiple.nll.stderr
[01:36:17] To update references, rerun the tests and pass the `--bless` flag
[01:36:17] To only update this specific test, also pass `--test-args regions/regions-close-over-type-parameter-multiple.rs`
[01:36:17] error: 1 errors occurred comparing output.
[01:36:17] status: exit code: 1
[01:36:17] status: exit code: 1
[01:36:17] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-over-type-parameter-multiple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-multiple.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-multiple.nll/auxiliary" "-A" "unused"
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] stderr:
[01:36:17] stderr:
[01:36:17] ------------------------------------------
[01:36:17] error: lifetime may not live long enough
[01:36:17]   --> /checkout/src/test/ui/regions/regions-close-over-type-parameter-multiple.rs:20:5
[01:36:17]    |
[01:36:17] LL | fn make_object_bad<'a,'b,'c,A:SomeTrait+'a+'b>(v: A) -> Box<dyn SomeTrait + 'c> {
[01:36:17]    |                    --    -- lifetime `'c` defined here
[01:36:17]    |                    lifetime `'a` defined here
[01:36:17]    |                    lifetime `'a` defined here
[01:36:17] LL |     // A outlives 'a AND 'b...but not 'c.
[01:36:17] LL |     box v as Box<dyn SomeTrait + 'a> //~ ERROR cannot infer an appropriate lifetime
[01:36:17]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'c`
[01:36:17] error: aborting due to previous error
[01:36:17] 
[01:36:17] 
[01:36:17] ------------------------------------------
[01:36:17] ------------------------------------------
[01:36:17] 
[01:36:17] 
[01:36:17] ---- [ui (nll)] ui/regions/regions-close-over-type-parameter-1.rs stdout ----
[01:36:17] diff of stderr:
[01:36:17] 
[01:36:17] 1 error[E0310]: the parameter type `A` may not live long enough
[01:36:17] 3    |
[01:36:17] 3    |
[01:36:17] - LL |     box v as Box<SomeTrait+'static>
[01:36:17] + LL |     box v as Box<dyn SomeTrait + 'static>
[01:36:18] 6    |
[01:36:18] 6    |
[01:36:18] 7    = help: consider adding an explicit lifetime bound `A: 'static`...
[01:36:18] 
[01:36:18] 9 error[E0309]: the parameter type `A` may not live long enough
[01:36:18] 11    |
[01:36:18] 11    |
[01:36:18] - LL |     box v as Box<SomeTrait+'b>
[01:36:18] + LL |     box v as Box<dyn SomeTrait + 'b>
[01:36:18] 14    |
[01:36:18] 14    |
[01:36:18] 15    = help: consider adding an explicit lifetime bound `A: 'b`...
[01:36:18] 
[01:36:18] The actual stderr differed from the expected stderr.
[01:36:18] The actual stderr differed from the expected stderr.
[01:36:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-1.nll/regions-close-over-type-parameter-1.nll.stderr
[01:36:18] To update references, rerun the tests and pass the `--bless` flag
[01:36:18] To only update this specific test, also pass `--test-args regions/regions-close-over-type-parameter-1.rs`
[01:36:18] error: 1 errors occurred comparing output.
[01:36:18] status: exit code: 1
[01:36:18] status: exit code: 1
[01:36:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-close-over-type-parameter-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-1.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-close-over-type-parameter-1.nll/auxiliary" "-A" "unused"
[01:36:18] ------------------------------------------
[01:36:18] 
[01:36:18] ------------------------------------------
[01:36:18] stderr:
[01:36:18] stderr:
[01:36:18] ------------------------------------------
[01:36:18] error[E0310]: the parameter type `A` may not live long enough
[01:36:18]    |
[01:36:18]    |
[01:36:18] LL |     box v as Box<dyn SomeTrait + 'static>
[01:36:18]    |
[01:36:18]    |
[01:36:18]    = help: consider adding an explicit lifetime bound `A: 'static`...
[01:36:18] 
[01:36:18] error[E0309]: the parameter type `A` may not live long enough
[01:36:18]    |
[01:36:18]    |
[01:36:18] LL |     box v as Box<dyn SomeTrait + 'b>
[01:36:18]    |
[01:36:18]    |
[01:36:18]    = help: consider adding an explicit lifetime bound `A: 'b`...
[01:36:18] error: aborting due to 2 previous errors
[01:36:18] 
[01:36:18] Some errors have detailed explanations: E0309, E0310.
[01:36:18] For more information about an error, try `rustc --explain E0309`.
[01:36:18] For more information about an error, try `rustc --explain E0309`.
[01:36:18] 
[01:36:18] ------------------------------------------
[01:36:18] 
[01:36:18] 
[01:36:18] ---- [ui (nll)] ui/regions/regions-nested-fns.rs stdout ----
[01:36:18] diff of stderr:
[01:36:18] 
[01:36:18] 4 LL |     let mut ay = &y;
[01:36:18] 5    |         ------ `ay` is declared here, outside of the closure body
[01:36:18] 6 LL | 
[01:36:18] - LL |     ignore::<Box<for<'z> FnMut(&'z isize)>>(Box::new(|z| {
[01:36:18] -    |                                                       - `z` is a reference that is only valid in the closure body
[01:36:18] + LL |     ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
[01:36:18] +    |                                                           - `z` is a reference that is only valid in the closure body
[01:36:18] 9 ...
[01:36:18] 10 LL |         ay = z;
[01:36:18] 11    |         ^^^^^^ `z` escapes the closure body here
[01:36:18] 
[01:36:18] 25 error[E0597]: `y` does not live long enough
[01:36:18] 27    |
[01:36:18] 27    |
[01:36:18] - LL |     ignore::<Box<for<'z> FnMut(&'z isize)>>(Box::new(|z| {
[01:36:18] -    |                                                      --- value captured here
[01:36:18] + LL |     ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
[01:36:18] +    |                                                          --- value captured here
[01:36:18] 30 LL |         ay = x;
[01:36:18] 31 LL |         ay = &y;
[01:36:18] 32    |               ^ borrowed value does not live long enough
[01:36:18] 
[01:36:18] The actual stderr differed from the expected stderr.
[01:36:18] The actual stderr differed from the expected stderr.
[01:36:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns.nll/regions-nested-fns.nll.stderr
[01:36:18] To update references, rerun the tests and pass the `--bless` flag
[01:36:18] To only update this specific test, also pass `--test-args regions/regions-nested-fns.rs`
[01:36:18] error: 1 errors occurred comparing output.
[01:36:18] status: exit code: 1
[01:36:18] status: exit code: 1
[01:36:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-nested-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns.nll/auxiliary" "-A" "unused"
[01:36:18] ------------------------------------------
[01:36:18] 
[01:36:18] ------------------------------------------
[01:36:18] stderr:
[01:36:18] stderr:
[01:36:18] ------------------------------------------
[01:36:18] error[E0521]: borrowed data escapes outside of closure
[01:36:18]   --> /checkout/src/test/ui/regions/regions-nested-fns.rs:10:9
[01:36:18]    |
[01:36:18] LL |     let mut ay = &y; //~ ERROR E0495
[01:36:18]    |         ------ `ay` is declared here, outside of the closure body
[01:36:18] LL | 
[01:36:18] LL |     ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
[01:36:18]    |                                                           - `z` is a reference that is only valid in the closure body
[01:36:18] ...
[01:36:18] LL |         ay = z;
[01:36:18]    |         ^^^^^^ `z` escapes the closure body here
[01:36:18] 
[01:36:18] error[E0597]: `y` does not live long enough
[01:36:18]    |
[01:36:18]    |
[01:36:18] LL |     let mut ay = &y; //~ ERROR E0495
[01:36:18]    |                  ^^ borrowed value does not live long enough
[01:36:18] ...
[01:36:18] LL |         if false { return ay; }
[01:36:18]    |                           -- returning this value requires that `y` is borrowed for `'static`
[01:36:18] LL | }
[01:36:18] LL | }
[01:36:18]    | - `y` dropped here while still borrowed
[01:36:18] 
[01:36:18] error[E0597]: `y` does not live long enough
[01:36:18]    |
[01:36:18]    |
[01:36:18] LL |     ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
[01:36:18]    |                                                          --- value captured here
[01:36:18] LL |         ay = x;
[01:36:18] LL |         ay = &y;
[01:36:18]    |               ^ borrowed value does not live long enough
[01:36:18] ...
[01:36:18] LL |         if false { return ay; }
[01:36:18]    |                           -- returning this value requires that `y` is borrowed for `'static`
[01:36:18] LL | }
[01:36:18] LL | }
[01:36:18]    | - `y` dropped here while still borrowed
[01:36:18] error: lifetime may not live long enough
[01:36:18]   --> /checkout/src/test/ui/regions/regions-nested-fns.rs:14:27
[01:36:18]    |
[01:36:18]    |
[01:36:18] LL | fn nested<'x>(x: &'x isize) {
[01:36:18]    |           -- lifetime `'x` defined here
[01:36:18] ...
[01:36:18] LL |         if false { return x; } //~ ERROR E0312
[01:36:18]    |                           ^ returning this value requires that `'x` must outlive `'static`
[01:36:18] error: aborting due to 4 previous errors
[01:36:18] 
[01:36:18] For more information about this error, try `rustc --explain E0597`.
[01:36:18] 
---
[01:36:18] 
[01:36:18] 1 error: lifetime may not live long enough
[01:36:18] 2   --> $DIR/regions-trait-object-subtyping.rs:15:5
[01:36:18] 3    |
[01:36:18] - LL | fn foo3<'a,'b>(x: &'a mut Dummy) -> &'b mut Dummy {
[01:36:18] + LL | fn foo3<'a,'b>(x: &'a mut dyn Dummy) -> &'b mut dyn Dummy {
[01:36:18] 6    |         |
[01:36:18] 7    |         lifetime `'a` defined here
[01:36:18] 
[01:36:18] 12 error: lifetime may not live long enough
[01:36:18] 12 error: lifetime may not live long enough
[01:36:18] 13   --> $DIR/regions-trait-object-subtyping.rs:22:5
[01:36:18] 14    |
[01:36:18] - LL | fn foo4<'a:'b,'b>(x: Wrapper<&'a mut Dummy>) -> Wrapper<&'b mut Dummy> {
[01:36:18] + LL | fn foo4<'a:'b,'b>(x: Wrapper<&'a mut dyn Dummy>) -> Wrapper<&'b mut dyn Dummy> {
[01:36:18] 17    |         |
[01:36:18] 18    |         lifetime `'a` defined here
[01:36:18] 
[01:36:18] 
[01:36:18] 
[01:36:18] The actual stderr differed from the expected stderr.
[01:36:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-trait-object-subtyping.nll/regions-trait-object-subtyping.nll.stderr
[01:36:18] To update references, rerun the tests and pass the `--bless` flag
[01:36:18] To only update this specific test, also pass `--test-args regions/regions-trait-object-subtyping.rs`
[01:36:18] error: 1 errors occurred comparing output.
[01:36:18] status: exit code: 1
[01:36:18] status: exit code: 1
[01:36:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-trait-object-subtyping.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-trait-object-subtyping.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-trait-object-subtyping.nll/auxiliary" "-A" "unused"
[01:36:18] ------------------------------------------
[01:36:18] 
[01:36:18] ------------------------------------------
[01:36:18] stderr:
[01:36:18] stderr:
[01:36:18] ------------------------------------------
[01:36:18] error: lifetime may not live long enough
[01:36:18]   --> /checkout/src/test/ui/regions/regions-trait-object-subtyping.rs:15:5
[01:36:18]    |
[01:36:18] LL | fn foo3<'a,'b>(x: &'a mut dyn Dummy) -> &'b mut dyn Dummy {
[01:36:18]    |         |
[01:36:18]    |         lifetime `'a` defined here
[01:36:18]    |         lifetime `'a` defined here
[01:36:18] LL |     // Without knowing 'a:'b, we can't coerce
[01:36:18] LL |     x //~ ERROR lifetime bound not satisfied
[01:36:18]    |     ^ returning this value requires that `'a` must outlive `'b`
[01:36:18] error: lifetime may not live long enough
[01:36:18]   --> /checkout/src/test/ui/regions/regions-trait-object-subtyping.rs:22:5
[01:36:18]    |
[01:36:18]    |
[01:36:18] LL | fn foo4<'a:'b,'b>(x: Wrapper<&'a mut dyn Dummy>) -> Wrapper<&'b mut dyn Dummy> {
[01:36:18]    |         |
[01:36:18]    |         lifetime `'a` defined here
[01:36:18] LL |     // We can't coerce because it is packed in `Wrapper`
[01:36:18] LL |     x //~ ERROR mismatched types
[01:36:18] LL |     x //~ ERROR mismatched types
[01:36:18]    |     ^ returning this value requires that `'b` must outlive `'a`
[01:36:18] error: aborting due to 2 previous errors
[01:36:18] 
[01:36:18] 
[01:36:18] ------------------------------------------
---
[01:36:18] 
[01:36:18] 1 error: lifetime may not live long enough
[01:36:18] 2   --> $DIR/variance-contravariant-arg-object.rs:14:5
[01:36:18] 3    |
[01:36:18] - LL | fn get_min_from_max<'min, 'max>(v: Box<Get<&'max i32>>)
[01:36:18] + LL | fn get_min_from_max<'min, 'max>(v: Box<dyn Get<&'max i32>>)
[01:36:18] 5    |                     ----  ---- lifetime `'max` defined here
[01:36:18] 6    |                     |
[01:36:18] 7    |                     lifetime `'min` defined here
[01:36:18] 12 error: lifetime may not live long enough
[01:36:18] 13   --> $DIR/variance-contravariant-arg-object.rs:22:5
[01:36:18] 14    |
[01:36:18] 14    |
[01:36:18] - LL | fn get_max_from_min<'min, 'max, G>(v: Box<Get<&'min i32>>)
[01:36:18] + LL | fn get_max_from_min<'min, 'max, G>(v: Box<dyn Get<&'min i32>>)
[01:36:18] 16    |                     ----  ---- lifetime `'max` defined here
[01:36:18] 17    |                     |
[01:36:18] 18    |                     lifetime `'min` defined here
[01:36:18] 
[01:36:18] The actual stderr differed from the expected stderr.
[01:36:18] The actual stderr differed from the expected stderr.
[01:36:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-arg-object.nll/variance-contravariant-arg-object.nll.stderr
[01:36:18] To update references, rerun the tests and pass the `--bless` flag
[01:36:18] To only update this specific test, also pass `--test-args variance/variance-contravariant-arg-object.rs`
[01:36:18] error: 1 errors occurred comparing output.
[01:36:18] status: exit code: 1
[01:36:18] status: exit code: 1
[01:36:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-contravariant-arg-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-arg-object.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-contravariant-arg-object.nll/auxiliary" "-A" "unused"
[01:36:18] ------------------------------------------
[01:36:18] 
[01:36:18] ------------------------------------------
[01:36:18] stderr:
[01:36:18] stderr:
[01:36:18] ------------------------------------------
[01:36:18] error: lifetime may not live long enough
[01:36:18]   --> /checkout/src/test/ui/variance/variance-contravariant-arg-object.rs:14:5
[01:36:18]    |
[01:36:18] LL | fn get_min_from_max<'min, 'max>(v: Box<dyn Get<&'max i32>>)
[01:36:18]    |                     ----  ---- lifetime `'max` defined here
[01:36:18]    |                     |
[01:36:18]    |                     lifetime `'min` defined here
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18]    |     ^ returning this value requires that `'min` must outlive `'max`
[01:36:18] error: lifetime may not live long enough
[01:36:18]   --> /checkout/src/test/ui/variance/variance-contravariant-arg-object.rs:22:5
[01:36:18]    |
[01:36:18]    |
[01:36:18] LL | fn get_max_from_min<'min, 'max, G>(v: Box<dyn Get<&'min i32>>)
[01:36:18]    |                     ----  ---- lifetime `'max` defined here
[01:36:18]    |                     |
[01:36:18]    |                     lifetime `'min` defined here
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18]    |     ^ returning this value requires that `'min` must outlive `'max`
[01:36:18] error: aborting due to 2 previous errors
[01:36:18] 
[01:36:18] 
[01:36:18] ------------------------------------------
---
[01:36:18] 
[01:36:18] 1 error: lifetime may not live long enough
[01:36:18] 2   --> $DIR/variance-covariant-arg-object.rs:15:5
[01:36:18] 3    |
[01:36:18] - LL | fn get_min_from_max<'min, 'max>(v: Box<Get<&'max i32>>)
[01:36:18] + LL | fn get_min_from_max<'min, 'max>(v: Box<dyn Get<&'max i32>>)
[01:36:18] 5    |                     ----  ---- lifetime `'max` defined here
[01:36:18] 6    |                     |
[01:36:18] 7    |                     lifetime `'min` defined here
[01:36:18] 12 error: lifetime may not live long enough
[01:36:18] 13   --> $DIR/variance-covariant-arg-object.rs:22:5
[01:36:18] 14    |
[01:36:18] 14    |
[01:36:18] - LL | fn get_max_from_min<'min, 'max, G>(v: Box<Get<&'min i32>>)
[01:36:18] + LL | fn get_max_from_min<'min, 'max, G>(v: Box<dyn Get<&'min i32>>)
[01:36:18] 16    |                     ----  ---- lifetime `'max` defined here
[01:36:18] 17    |                     |
[01:36:18] 18    |                     lifetime `'min` defined here
[01:36:18] 
[01:36:18] The actual stderr differed from the expected stderr.
[01:36:18] The actual stderr differed from the expected stderr.
[01:36:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-covariant-arg-object.nll/variance-covariant-arg-object.nll.stderr
[01:36:18] To update references, rerun the tests and pass the `--bless` flag
[01:36:18] To only update this specific test, also pass `--test-args variance/variance-covariant-arg-object.rs`
[01:36:18] error: 1 errors occurred comparing output.
[01:36:18] status: exit code: 1
[01:36:18] status: exit code: 1
[01:36:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-covariant-arg-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-covariant-arg-object.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-covariant-arg-object.nll/auxiliary" "-A" "unused"
[01:36:18] ------------------------------------------
[01:36:18] 
[01:36:18] ------------------------------------------
[01:36:18] stderr:
[01:36:18] stderr:
[01:36:18] ------------------------------------------
[01:36:18] error: lifetime may not live long enough
[01:36:18]   --> /checkout/src/test/ui/variance/variance-covariant-arg-object.rs:15:5
[01:36:18]    |
[01:36:18] LL | fn get_min_from_max<'min, 'max>(v: Box<dyn Get<&'max i32>>)
[01:36:18]    |                     ----  ---- lifetime `'max` defined here
[01:36:18]    |                     |
[01:36:18]    |                     lifetime `'min` defined here
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18]    |     ^ returning this value requires that `'min` must outlive `'max`
[01:36:18] error: lifetime may not live long enough
[01:36:18]   --> /checkout/src/test/ui/variance/variance-covariant-arg-object.rs:22:5
[01:36:18]    |
[01:36:18]    |
[01:36:18] LL | fn get_max_from_min<'min, 'max, G>(v: Box<dyn Get<&'min i32>>)
[01:36:18]    |                     ----  ---- lifetime `'max` defined here
[01:36:18]    |                     |
[01:36:18]    |                     lifetime `'min` defined here
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18]    |     ^ returning this value requires that `'min` must outlive `'max`
[01:36:18] error: aborting due to 2 previous errors
[01:36:18] 
[01:36:18] 
[01:36:18] ------------------------------------------
---
[01:36:18] 
[01:36:18] 1 error: lifetime may not live long enough
[01:36:18] 2   --> $DIR/variance-invariant-arg-object.rs:11:5
[01:36:18] 3    |
[01:36:18] - LL | fn get_min_from_max<'min, 'max>(v: Box<Get<&'max i32>>)
[01:36:18] + LL | fn get_min_from_max<'min, 'max>(v: Box<dyn Get<&'max i32>>)
[01:36:18] 5    |                     ----  ---- lifetime `'max` defined here
[01:36:18] 6    |                     |
[01:36:18] 7    |                     lifetime `'min` defined here
[01:36:18] 12 error: lifetime may not live long enough
[01:36:18] 13   --> $DIR/variance-invariant-arg-object.rs:18:5
[01:36:18] 14    |
[01:36:18] 14    |
[01:36:18] - LL | fn get_max_from_min<'min, 'max, G>(v: Box<Get<&'min i32>>)
[01:36:18] + LL | fn get_max_from_min<'min, 'max, G>(v: Box<dyn Get<&'min i32>>)
[01:36:18] 16    |                     ----  ---- lifetime `'max` defined here
[01:36:18] 17    |                     |
[01:36:18] 18    |                     lifetime `'min` defined here
[01:36:18] 
[01:36:18] The actual stderr differed from the expected stderr.
[01:36:18] The actual stderr differed from the expected stderr.
[01:36:18] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-invariant-arg-object.nll/variance-invariant-arg-object.nll.stderr
[01:36:18] To update references, rerun the tests and pass the `--bless` flag
[01:36:18] To only update this specific test, also pass `--test-args variance/variance-invariant-arg-object.rs`
[01:36:18] error: 1 errors occurred comparing output.
[01:36:18] status: exit code: 1
[01:36:18] status: exit code: 1
[01:36:18] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-invariant-arg-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-invariant-arg-object.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-invariant-arg-object.nll/auxiliary" "-A" "unused"
[01:36:18] ------------------------------------------
[01:36:18] 
[01:36:18] ------------------------------------------
[01:36:18] stderr:
[01:36:18] stderr:
[01:36:18] ------------------------------------------
[01:36:18] error: lifetime may not live long enough
[01:36:18]   --> /checkout/src/test/ui/variance/variance-invariant-arg-object.rs:11:5
[01:36:18]    |
[01:36:18] LL | fn get_min_from_max<'min, 'max>(v: Box<dyn Get<&'max i32>>)
[01:36:18]    |                     ----  ---- lifetime `'max` defined here
[01:36:18]    |                     |
[01:36:18]    |                     lifetime `'min` defined here
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18]    |     ^ returning this value requires that `'min` must outlive `'max`
[01:36:18] error: lifetime may not live long enough
[01:36:18]   --> /checkout/src/test/ui/variance/variance-invariant-arg-object.rs:18:5
[01:36:18]    |
[01:36:18]    |
[01:36:18] LL | fn get_max_from_min<'min, 'max, G>(v: Box<dyn Get<&'min i32>>)
[01:36:18]    |                     ----  ---- lifetime `'max` defined here
[01:36:18]    |                     |
[01:36:18]    |                     lifetime `'min` defined here
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18] LL |     v //~ ERROR mismatched types
[01:36:18]    |     ^ returning this value requires that `'min` must outlive `'max`
[01:36:18] error: aborting due to 2 previous errors
[01:36:18] 
[01:36:18] 
[01:36:18] ------------------------------------------
---
[01:36:18] test result: FAILED. 5527 passed; 23 failed; 49 ignored; 0 measured; 0 filtered out
[01:36:18] 
[01:36:18] 
[01:36:18] 
[01:36:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "8.0.0-rust-1.37.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
[01:36:18] 
[01:36:18] 
[01:36:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:18] Build completed unsuccessfully in 0:09:11
---
travis_time:end:18ad9200:start=1559101020538755870,finish=1559101020549949217,duration=11193347
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:089fbfbc
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:071de7e8
travis_time:start:071de7e8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0aa0a447
$ dmesg | grep -i kill
