\n"},"level":"warning","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs","byte_start":289,"byte_end":290,"line_start":12,"line_end":12,"column_start":67,"column_end":68,"is_primary":true,"text":[{"text":"const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }","highlight_start":67,"highlight_end":68}],"label":"creates a temporary which is freed while still in use","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs","byte_start":294,"byte_end":295,"line_start":12,"line_end":12,"column_start":72,"column_end":73,"is_primary":false,"text":[{"text":"const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }","highlight_start":72,"highlight_end":73}],"label":"temporary value is freed at the end of this statement","suggested_replacement":null,"suggestion_applicability":null,"expansion":null},{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs","byte_start":288,"byte_end":290,"line_start":12,"line_end":12,"column_start":66,"column_end":68,"is_primary":false,"text":[{"text":"const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }","highlight_start":66,"highlight_end":68}],"label":"cast requires that borrow lasts for `'static`","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"this error has been downgraded to a warning for backwards compatibility with previous releases","code":null,"level":"warning","spans":[],"children":[],"rendered":null},{"message":"this represents potential undefined behavior in your code and this warning will become a hard error in the future","code":null,"level":"warning","spans":[],"children":[],"rendered":null}],"rendered":"warning[E0716]: temporary value dropped while borrowed\n  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs:12:67\n   |\nLL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }\n   |                                                                  -^    - temporary value is freed at the end of this statement\n   |                                                                  ||\n   |                                                                  |creates a temporary which is freed while still in use\n   |                                                                  cast requires that borrow lasts for `'static`\n   |\n   = warning: this error has been downgraded to a warning for backwards compatibility with previous releases\n   = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future\n\n"}
[01:20:51] {"message":"aborting due to 2 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 2 previous errors\n\n"}
[01:20:51] {"message":"Some errors occurred: E0716, E0723.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0716, E0723.\n"}
[01:20:51] 
[01:20:51] ------------------------------------------
[01:20:51] 
[01:20:51] thread '[ui (nll)] ui/consts/min_const_fn/min_const_fn_dyn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:20:51] thread '[ui (nll)] ui/consts/min_const_fn/min_const_fn_dyn.rs' panicked at 'explicit panic', src/tools/compiletest/src/runtest.rs:3295:9
[01:20:51] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:20:51] 
[01:20:51] ---- [ui (nll)] ui/consts/min_const_fn/min_const_fn.rs stdout ----
[01:20:51] diff of stderr:
[01:20:51] 
[01:20:51] 4 LL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated
[01:20:51] 5    |                         ^^^^ constant functions cannot evaluate destructors
[01:20:51] - error: mutable references in const fn are unstable
[01:20:51] + error[E0723]: mutable references in const fn are unstable (see issue #57563)
[01:20:51] 8   --> $DIR/min_const_fn.rs:39:36
[01:20:51] 9    |
[01:20:51] 9    |
[01:20:51] 10 LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
[01:20:51] 11    |                                    ^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 12 
[01:20:51] 12 
[01:20:51] 13 error[E0493]: destructors cannot be evaluated at compile-time
[01:20:51] 
[01:20:51] 
[01:20:51] 16 LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructors cannot be evaluated
[01:20:51] 17    |                            ^^^^ constant functions cannot evaluate destructors
[01:20:51] - error: mutable references in const fn are unstable
[01:20:51] + error[E0723]: mutable references in const fn are unstable (see issue #57563)
[01:20:51] 20   --> $DIR/min_const_fn.rs:46:42
[01:20:51] 21    |
[01:20:51] 21    |
[01:20:51] 22 LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
[01:20:51] 23    |                                          ^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 24 
[01:20:51] 24 
[01:20:51] 25 error[E0493]: destructors cannot be evaluated at compile-time
[01:20:51] 
[01:20:51] 
[01:20:51] 28 LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructors
[01:20:51] 29    |                           ^^^^ constant functions cannot evaluate destructors
[01:20:51] - error: mutable references in const fn are unstable
[01:20:51] + error[E0723]: mutable references in const fn are unstable (see issue #57563)
[01:20:51] 32   --> $DIR/min_const_fn.rs:53:38
[01:20:51] 33    |
[01:20:51] 33    |
[01:20:51] 34 LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
[01:20:51] 35    |                                      ^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 36 
[01:20:51] 36 
[01:20:51] - error: mutable references in const fn are unstable
[01:20:51] + error[E0723]: mutable references in const fn are unstable (see issue #57563)
[01:20:51] 38   --> $DIR/min_const_fn.rs:58:39
[01:20:51] 39    |
[01:20:51] 40 LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
[01:20:51] 41    |                                       ^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 42 
[01:20:51] 42 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 45    |
[01:20:51] 45    |
[01:20:51] 46 LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
[01:20:51] 47    |                ^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 48 
[01:20:51] 48 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 51    |
[01:20:51] 51    |
[01:20:51] 52 LL | const fn foo11_2<T: Send>(t: T) -> T { t }
[01:20:51] 53    |                  ^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 54 
[01:20:51] 54 
[01:20:51] - error: only int, `bool` and `char` operations are stable in const fn
[01:20:51] + error[E0723]: only int, `bool` and `char` operations are stable in const fn (see issue #57563)
[01:20:51] 57    |
[01:20:51] 57    |
[01:20:51] 58 LL | const fn foo19(f: f32) -> f32 { f * 2.0 }
[01:20:51] 59    |                                 ^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 60 
[01:20:51] 60 
[01:20:51] - error: only int, `bool` and `char` operations are stable in const fn
[01:20:51] + error[E0723]: only int, `bool` and `char` operations are stable in const fn (see issue #57563)
[01:20:51] 63    |
[01:20:51] 63    |
[01:20:51] 64 LL | const fn foo19_2(f: f32) -> f32 { 2.0 - f }
[01:20:51] 65    |                                   ^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 66 
[01:20:51] 66 
[01:20:51] - error: only int and `bool` operations are stable in const fn
[01:20:51] + error[E0723]: only int and `bool` operations are stable in const fn (see issue #57563)
[01:20:51] 69    |
[01:20:51] 69    |
[01:20:51] 70 LL | const fn foo19_3(f: f32) -> f32 { -f }
[01:20:51] 71    |                                   ^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 72 
[01:20:51] 72 
[01:20:51] - error: only int, `bool` and `char` operations are stable in const fn
[01:20:51] + error[E0723]: only int, `bool` and `char` operations are stable in const fn (see issue #57563)
[01:20:51] 75    |
[01:20:51] 75    |
[01:20:51] 76 LL | const fn foo19_4(f: f32, g: f32) -> f32 { f / g }
[01:20:51] 77    |                                           ^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 78 
[01:20:51] 78 
[01:20:51] - error: cannot access `static` items in const fn
[01:20:51] + error[E0723]: cannot access `static` items in const fn (see issue #57563)
[01:20:51] 81    |
[01:20:51] 81    |
[01:20:51] 82 LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot access `static` items in const fn
[01:20:51] 83    |                           ^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 84 
[01:20:51] 84 
[01:20:51] - error: cannot access `static` items in const fn
[01:20:51] + error[E0723]: cannot access `static` items in const fn (see issue #57563)
[01:20:51] 87    |
[01:20:51] 87    |
[01:20:51] 88 LL | const fn foo26() -> &'static u32 { &BAR } //~ ERROR cannot access `static` items
[01:20:51] 89    |                                    ^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 90 
[01:20:51] 90 
[01:20:51] - error: casting pointers to ints is unstable in const fn
[01:20:51] + error[E0723]: casting pointers to ints is unstable in const fn (see issue #57563)
[01:20:51] 93    |
[01:20:51] 93    |
[01:20:51] 94 LL | const fn foo30(x: *const u32) -> usize { x as usize }
[01:20:51] 95    |                                          ^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 96 
[01:20:51] 96 
[01:20:51] - error: casting pointers to ints is unstable in const fn
[01:20:51] + error[E0723]: casting pointers to ints is unstable in const fn (see issue #57563)
[01:20:51] 99    |
[01:20:51] 99    |
[01:20:51] 100 LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
[01:20:51] 101    |                                                               ^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 102 
[01:20:51] 102 
[01:20:51] - error: casting pointers to ints is unstable in const fn
[01:20:51] + error[E0723]: casting pointers to ints is unstable in const fn (see issue #57563)
[01:20:51] 105    |
[01:20:51] 105    |
[01:20:51] 106 LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
[01:20:51] 107    |                                          ^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 108 
[01:20:51] 108 
[01:20:51] - error: casting pointers to ints is unstable in const fn
[01:20:51] + error[E0723]: casting pointers to ints is unstable in const fn (see issue #57563)
[01:20:51] 111    |
[01:20:51] 111    |
[01:20:51] 112 LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
[01:20:51] 113    |                                                               ^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 114 
[01:20:51] 114 
[01:20:51] - error: `if`, `match`, `&&` and `||` are not stable in const fn
[01:20:51] + error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn (see issue #57563)
[01:20:51] 117    |
[01:20:51] 117    |
[01:20:51] 118 LL | const fn foo30_4(b: bool) -> usize { if b { 1 } else { 42 } }
[01:20:51] 119    |                                      ^^^^^^^^^^^^^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 120 
[01:20:51] 120 
[01:20:51] - error: `if`, `match`, `&&` and `||` are not stable in const fn
[01:20:51] + error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn (see issue #57563)
[01:20:51] 123    |
[01:20:51] 123    |
[01:20:51] 124 LL | const fn foo30_5(b: bool) { while b { } } //~ ERROR not stable in const fn
[01:20:51] 125    |                             ^^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 126 
[01:20:51] 126 
[01:20:51] - error: `if`, `match`, `&&` and `||` are not stable in const fn
[01:20:51] + error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn (see issue #57563)
[01:20:51] 129    |
[01:20:51] 129    |
[01:20:51] 130 LL | const fn foo36(a: bool, b: bool) -> bool { a && b }
[01:20:51] 131    |                                            ^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 132 
[01:20:51] 132 
[01:20:51] - error: `if`, `match`, `&&` and `||` are not stable in const fn
[01:20:51] + error[E0723]: `if`, `match`, `&&` and `||` are not stable in const fn (see issue #57563)
[01:20:51] 135    |
[01:20:51] 135    |
[01:20:51] 136 LL | const fn foo37(a: bool, b: bool) -> bool { a || b }
[01:20:51] 137    |                                            ^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 138 
[01:20:51] 138 
[01:20:51] - error: mutable references in const fn are unstable
[01:20:51] + error[E0723]: mutable references in const fn are unstable (see issue #57563)
[01:20:51] 140   --> $DIR/min_const_fn.rs:108:14
[01:20:51] 141    |
[01:20:51] 142 LL | const fn inc(x: &mut i32) { *x += 1 }
[01:20:51] 143    |              ^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 144 
[01:20:51] 144 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 147    |
[01:20:51] 147    |
[01:20:51] 148 LL | impl<T: std::fmt::Debug> Foo<T> {
[01:20:51] 149    |      ^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 150 
[01:20:51] 150 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 153    |
[01:20:51] 153    |
[01:20:51] 154 LL | impl<T: std::fmt::Debug + Sized> Foo<T> {
[01:20:51] 155    |      ^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 156 
[01:20:51] 156 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 159    |
[01:20:51] 159    |
[01:20:51] 160 LL | impl<T: Sync + Sized> Foo<T> {
[01:20:51] 161    |      ^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 162 
[01:20:51] 162 
[01:20:51] - error: `impl Trait` in const fn is unstable
[01:20:51] + error[E0723]: `impl Trait` in const fn is unstable (see issue #57563)
[01:20:51] 165    |
[01:20:51] 165    |
[01:20:51] 166 LL | const fn no_rpit2() -> AlanTuring<impl std::fmt::Debug> { AlanTuring(0) }
[01:20:51] 167    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 168 
[01:20:51] 168 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 171    |
[01:20:51] 171    |
[01:20:51] 172 LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
[01:20:51] 173    |                                  ^^^^^^^^^^^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 174 
[01:20:51] 174 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 177    |
[01:20:51] 177    |
[01:20:51] 178 LL | const fn no_apit(_x: impl std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
[01:20:51] 179    |                      ^^^^^^^^^^^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 180 
[01:20:51] 180 
[01:20:51] - error: `impl Trait` in const fn is unstable
[01:20:51] + error[E0723]: `impl Trait` in const fn is unstable (see issue #57563)
[01:20:51] 183    |
[01:20:51] 183    |
[01:20:51] 184 LL | const fn no_rpit() -> impl std::fmt::Debug {} //~ ERROR `impl Trait` in const fn is unstable
[01:20:51] 185    |                       ^^^^^^^^^^^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 186 
[01:20:51] 186 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 189    |
[01:20:51] 189    |
[01:20:51] 190 LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
[01:20:51] 191    |                       ^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 192 
[01:20:51] 192 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 195    |
[01:20:51] 195    |
[01:20:51] 196 LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
[01:20:51] 197    |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 198 
[01:20:51] 198 
[01:20:51] 199 warning[E0515]: cannot return reference to temporary value
[01:20:51] 200   --> $DIR/min_const_fn.rs:136:63
[01:20:51] 
[01:20:51] 208    = warning: this error has been downgraded to a warning for backwards compatibility with previous releases
[01:20:51] 209    = warning: this represents potential undefined behavior in your code and this warning will become a hard error in the future
[01:20:51] 210 
[01:20:51] - error: trait bounds other than `Sized` on const fn parameters are unstable
[01:20:51] + error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable (see issue #57563)
[01:20:51] 213    |
[01:20:51] 213    |
[01:20:51] 214 LL | const fn really_no_traits_i_mean_it() { (&() as &std::fmt::Debug, ()).1 }
[01:20:51] 215    |                                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 216 
[01:20:51] 216 
[01:20:51] - error: function pointers in const fn are unstable
[01:20:51] + error[E0723]: function pointers in const fn are unstable (see issue #57563)
[01:20:51] 219    |
[01:20:51] 219    |
[01:20:51] 220 LL | const fn no_fn_ptrs(_x: fn()) {}
[01:20:51] 221    |                     ^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 222 
[01:20:51] 222 
[01:20:51] - error: function pointers in const fn are unstable
[01:20:51] + error[E0723]: function pointers in const fn are unstable (see issue #57563)
[01:20:51] 225    |
[01:20:51] 225    |
[01:20:51] 226 LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
[01:20:51] 227    |                           ^^^^
[01:20:51] +    |
[01:20:51] +    = help: add #![feature(const_fn)] to the crate attributes to enable
[01:20:51] 228 
---
[01:20:51] 233 
[01:20:51] 
[01:20:51] 
[01:20:51] The actual stderr differed from the expected stderr.
[01:20:51] Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll/min_const_fn.nll.stderr
[01:20:51] To update references, rerun the tests and pass the `--bless` flag
[01:20:51] To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`
[01:20:51] error: 1 errors occurred comparing output.
[01:20:51] status: exit code: 1
[01:20:51] status: exit code: 1
[01:20:51] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll/a" "-Zborrowck=migrate" "-Ztwo-phase-borrows" "-Crpath" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll/auxiliary" "-A" "unused"
[01:20:51] ------------------------------------------
[01:20:51] 
[01:20:51] ------------------------------------------
[01:20:51] stderr:
[01:20:51] stderr:
[01:20:51] ------------------------------------------
[01:20:51] {"message":"destructors cannot be evaluated at compile-time","code":{"code":"E0493","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs","byte_start":1264,"byte_end":1268,"line_start":37,"line_end":37,"column_start":25,"column_end":29,"is_primary":true,"text":[{"text":"    const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated","highlight_start":25,"highlight_end":29}],"label":"constant functions cannot evaluate destructors","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0493]: destructors cannot be evaluated at compile-time\n  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:37:25\n   |\nLL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated\n   |                         ^^^^ constant functions cannot evaluate destructors\n\n"}
[01:20:51] {"message":"mutable references in const fn are unstable (see issue #57563)","code":{"code":"E0723","explanation":"\nAn feature unstable in `const` contexts was used.\n\nErroneous code example:\n\n