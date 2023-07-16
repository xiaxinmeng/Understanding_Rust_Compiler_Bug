plain
2019-07-31T22:52:42.6803875Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-31T22:52:42.6803929Z 
2019-07-31T22:52:42.6804179Z   git checkout -b <new-branch-name>
2019-07-31T22:52:42.6804224Z 
2019-07-31T22:52:42.6804525Z HEAD is now at 95a212a24 Auto merge of #63176 - Centril:rollup-jtbtt4n, r=Centril
2019-07-31T22:52:42.6972177Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-31T22:52:42.6975212Z ==============================================================================
2019-07-31T22:52:42.6975308Z Task         : Bash
2019-07-31T22:52:42.6975538Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-01T00:53:34.7485544Z 
2019-08-01T00:53:34.7485611Z 10 LL |     a + b
2019-08-01T00:53:34.7485667Z 11    |     ^^^^^
2019-08-01T00:53:34.7485737Z 12    |
2019-08-01T00:53:34.7486038Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7486333Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7486621Z 14    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7486688Z 15 
2019-08-01T00:53:34.7486944Z 16 error[E0381]: use of possibly uninitialized variable: `a`
2019-08-01T00:53:34.7487017Z 
2019-08-01T00:53:34.7487092Z The actual stderr differed from the expected stderr.
2019-08-01T00:53:34.7487092Z The actual stderr differed from the expected stderr.
2019-08-01T00:53:34.7487425Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_refutable.nll/const_let_refutable.nll.stderr
2019-08-01T00:53:34.7487709Z To update references, rerun the tests and pass the `--bless` flag
2019-08-01T00:53:34.7487972Z To only update this specific test, also pass `--test-args consts/const_let_refutable.rs`
2019-08-01T00:53:34.7488093Z error: 1 errors occurred comparing output.
2019-08-01T00:53:34.7488170Z status: exit code: 1
2019-08-01T00:53:34.7488170Z status: exit code: 1
2019-08-01T00:53:34.7489140Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const_let_refutable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_refutable.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_let_refutable.nll/auxiliary" "-A" "unused"
2019-08-01T00:53:34.7490064Z ------------------------------------------
2019-08-01T00:53:34.7490119Z 
2019-08-01T00:53:34.7490377Z ------------------------------------------
2019-08-01T00:53:34.7490446Z stderr:
2019-08-01T00:53:34.7490446Z stderr:
2019-08-01T00:53:34.7490699Z ------------------------------------------
2019-08-01T00:53:34.7490782Z error[E0005]: refutable pattern in function argument: `&[]` not covered
2019-08-01T00:53:34.7491160Z    |
2019-08-01T00:53:34.7491160Z    |
2019-08-01T00:53:34.7491489Z LL | const fn slice([a, b]: &[i32]) -> i32 { //~ ERROR refutable pattern in function argument
2019-08-01T00:53:34.7491582Z    |                ^^^^^^ pattern `&[]` not covered
2019-08-01T00:53:34.7491650Z 
2019-08-01T00:53:34.7491738Z error[E0723]: can only call other `const fn` within a `const fn`, but `const <&i32 as std::ops::Add>::add` is not stable as `const fn`
2019-08-01T00:53:34.7492136Z    |
2019-08-01T00:53:34.7492136Z    |
2019-08-01T00:53:34.7492223Z LL |     a + b //~ ERROR can only call other `const fn` within a `const fn`
2019-08-01T00:53:34.7492384Z    |
2019-08-01T00:53:34.7492384Z    |
2019-08-01T00:53:34.7492689Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7492809Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7492861Z 
2019-08-01T00:53:34.7492946Z error[E0381]: use of possibly uninitialized variable: `a`
2019-08-01T00:53:34.7493591Z    |
2019-08-01T00:53:34.7493591Z    |
2019-08-01T00:53:34.7493651Z LL |     a + b //~ ERROR can only call other `const fn` within a `const fn`
2019-08-01T00:53:34.7493736Z    |     ^ use of possibly uninitialized `a`
2019-08-01T00:53:34.7493774Z 
2019-08-01T00:53:34.7493847Z error[E0381]: use of possibly uninitialized variable: `b`
2019-08-01T00:53:34.7494229Z    |
2019-08-01T00:53:34.7494229Z    |
2019-08-01T00:53:34.7494288Z LL |     a + b //~ ERROR can only call other `const fn` within a `const fn`
2019-08-01T00:53:34.7494374Z    |         ^ use of possibly uninitialized `b`
2019-08-01T00:53:34.7494463Z error: aborting due to 4 previous errors
2019-08-01T00:53:34.7494520Z 
2019-08-01T00:53:34.7494578Z Some errors have detailed explanations: E0005, E0381, E0723.
2019-08-01T00:53:34.7494856Z For more information about an error, try `rustc --explain E0005`.
---
2019-08-01T00:53:34.7495535Z 
2019-08-01T00:53:34.7495603Z 4 LL |     x.0.field;
2019-08-01T00:53:34.7495664Z 5    |     ^^^^^^^^^
2019-08-01T00:53:34.7495734Z 6    |
2019-08-01T00:53:34.7495980Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7496273Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7496373Z 8    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7496436Z 9 
2019-08-01T00:53:34.7496517Z 10 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7496572Z 
2019-08-01T00:53:34.7496815Z 13 LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
2019-08-01T00:53:34.7496972Z 15    |
2019-08-01T00:53:34.7497234Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7497234Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7497528Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7497611Z 17    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7497692Z 18 
2019-08-01T00:53:34.7497748Z 19 error[E0716]: temporary value dropped while borrowed
2019-08-01T00:53:34.7497835Z 
2019-08-01T00:53:34.7498067Z The actual stderr differed from the expected stderr.
2019-08-01T00:53:34.7498067Z The actual stderr differed from the expected stderr.
2019-08-01T00:53:34.7498422Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn.nll/min_const_fn_dyn.nll.stderr
2019-08-01T00:53:34.7498700Z To update references, rerun the tests and pass the `--bless` flag
2019-08-01T00:53:34.7506088Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn_dyn.rs`
2019-08-01T00:53:34.7506244Z error: 1 errors occurred comparing output.
2019-08-01T00:53:34.7506300Z status: exit code: 1
2019-08-01T00:53:34.7506300Z status: exit code: 1
2019-08-01T00:53:34.7507206Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn_dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn_dyn.nll/auxiliary" "-A" "unused"
2019-08-01T00:53:34.7507801Z ------------------------------------------
2019-08-01T00:53:34.7507859Z 
2019-08-01T00:53:34.7508060Z ------------------------------------------
2019-08-01T00:53:34.7508133Z stderr:
2019-08-01T00:53:34.7508133Z stderr:
2019-08-01T00:53:34.7508326Z ------------------------------------------
2019-08-01T00:53:34.7508412Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7508820Z    |
2019-08-01T00:53:34.7508869Z LL |     x.0.field;
2019-08-01T00:53:34.7508938Z    |     ^^^^^^^^^
2019-08-01T00:53:34.7508987Z    |
2019-08-01T00:53:34.7508987Z    |
2019-08-01T00:53:34.7510009Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7510123Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7510193Z 
2019-08-01T00:53:34.7510277Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7510715Z    |
2019-08-01T00:53:34.7510715Z    |
2019-08-01T00:53:34.7511006Z LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
2019-08-01T00:53:34.7511187Z    |
2019-08-01T00:53:34.7511187Z    |
2019-08-01T00:53:34.7511495Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7511609Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7511660Z 
2019-08-01T00:53:34.7511745Z error[E0716]: temporary value dropped while borrowed
2019-08-01T00:53:34.7512123Z    |
2019-08-01T00:53:34.7512123Z    |
2019-08-01T00:53:34.7512395Z LL | const fn no_inner_dyn_trait_ret() -> Hide { Hide(HasDyn { field: &0 }) }
2019-08-01T00:53:34.7512775Z    |                                                                  -^    - temporary value is freed at the end of this statement
2019-08-01T00:53:34.7512885Z    |                                                                  ||
2019-08-01T00:53:34.7513009Z    |                                                                  |creates a temporary which is freed while still in use
2019-08-01T00:53:34.7513838Z    |                                                                  cast requires that borrow lasts for `'static`
2019-08-01T00:53:34.7513960Z error: aborting due to 3 previous errors
2019-08-01T00:53:34.7514111Z 
2019-08-01T00:53:34.7514164Z Some errors have detailed explanations: E0716, E0723.
2019-08-01T00:53:34.7514407Z For more information about an error, try `rustc --explain E0716`.
2019-08-01T00:53:34.7514407Z For more information about an error, try `rustc --explain E0716`.
2019-08-01T00:53:34.7514451Z 
2019-08-01T00:53:34.7514641Z ------------------------------------------
2019-08-01T00:53:34.7514702Z 
2019-08-01T00:53:34.7514730Z 
2019-08-01T00:53:34.7514941Z ---- [ui (nll)] ui/consts/min_const_fn/min_const_fn.rs stdout ----
2019-08-01T00:53:34.7515020Z diff of stderr:
2019-08-01T00:53:34.7515052Z 
2019-08-01T00:53:34.7515284Z 10 LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
2019-08-01T00:53:34.7515425Z 12    |
2019-08-01T00:53:34.7515668Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7515668Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7515952Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7516034Z 14    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7516113Z 15 
2019-08-01T00:53:34.7516328Z 16 error[E0493]: destructors cannot be evaluated at compile-time
2019-08-01T00:53:34.7516387Z 
2019-08-01T00:53:34.7516606Z 25 LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
2019-08-01T00:53:34.7516854Z 27    |
2019-08-01T00:53:34.7517132Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7517132Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7541128Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7541485Z 29    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7541585Z 30 
2019-08-01T00:53:34.7541937Z 31 error[E0493]: destructors cannot be evaluated at compile-time
2019-08-01T00:53:34.7541995Z 
2019-08-01T00:53:34.7542280Z 40 LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
2019-08-01T00:53:34.7542456Z 42    |
2019-08-01T00:53:34.7542749Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7542749Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7543094Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7543387Z 44    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7543534Z 46 error[E0723]: mutable references in const fn are unstable
2019-08-01T00:53:34.7543597Z 
2019-08-01T00:53:34.7543597Z 
2019-08-01T00:53:34.7543838Z 49 LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
2019-08-01T00:53:34.7544157Z 51    |
2019-08-01T00:53:34.7544431Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7544431Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7544710Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7544811Z 53    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7544877Z 54 
2019-08-01T00:53:34.7544959Z 55 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7545016Z 
2019-08-01T00:53:34.7545262Z 58 LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
2019-08-01T00:53:34.7545403Z 60    |
2019-08-01T00:53:34.7545652Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7545652Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7545954Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7546199Z 62    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7546281Z 63 
2019-08-01T00:53:34.7546342Z 64 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7546562Z 
2019-08-01T00:53:34.7546769Z 67 LL | const fn foo11_2<T: Send>(t: T) -> T { t }
2019-08-01T00:53:34.7546897Z 69    |
2019-08-01T00:53:34.7547152Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7547152Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7547579Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7547673Z 71    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7547733Z 72 
2019-08-01T00:53:34.7547807Z 73 error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-08-01T00:53:34.7547850Z 
2019-08-01T00:53:34.7548074Z 76 LL | const fn foo19(f: f32) -> f32 { f * 2.0 }
2019-08-01T00:53:34.7548207Z 78    |
2019-08-01T00:53:34.7548438Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7548438Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7548710Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7548786Z 80    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7548961Z 81 
2019-08-01T00:53:34.7549018Z 82 error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-08-01T00:53:34.7549081Z 
2019-08-01T00:53:34.7549308Z 85 LL | const fn foo19_2(f: f32) -> f32 { 2.0 - f }
2019-08-01T00:53:34.7549443Z 87    |
2019-08-01T00:53:34.7549790Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7549790Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7550382Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7550506Z 89    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7550583Z 90 
2019-08-01T00:53:34.7550671Z 91 error[E0723]: only int and `bool` operations are stable in const fn
2019-08-01T00:53:34.7550723Z 
2019-08-01T00:53:34.7550990Z 94 LL | const fn foo19_3(f: f32) -> f32 { -f }
2019-08-01T00:53:34.7551166Z 96    |
2019-08-01T00:53:34.7551479Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7551479Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7551818Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7551914Z 98    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7552006Z 99 
2019-08-01T00:53:34.7552085Z 100 error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-08-01T00:53:34.7552140Z 
2019-08-01T00:53:34.7552421Z 103 LL | const fn foo19_4(f: f32, g: f32) -> f32 { f / g }
2019-08-01T00:53:34.7552591Z 105    |
2019-08-01T00:53:34.7552879Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7552879Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7553216Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7553505Z 107    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7553568Z 108 
2019-08-01T00:53:34.7553819Z 109 error[E0723]: cannot access `static` items in const fn
2019-08-01T00:53:34.7553860Z 
2019-08-01T00:53:34.7554069Z 112 LL | const fn foo25() -> u32 { BAR }
2019-08-01T00:53:34.7554206Z 114    |
2019-08-01T00:53:34.7554486Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7554486Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7554767Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7554869Z 116    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7554952Z 117 
2019-08-01T00:53:34.7555011Z 118 error[E0723]: cannot access `static` items in const fn
2019-08-01T00:53:34.7555051Z 
2019-08-01T00:53:34.7555287Z 121 LL | const fn foo26() -> &'static u32 { &BAR }
2019-08-01T00:53:34.7555438Z 123    |
2019-08-01T00:53:34.7555691Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7555691Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7555989Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7556072Z 125    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7556239Z 127 error[E0723]: casting pointers to ints is unstable in const fn
2019-08-01T00:53:34.7556283Z 
2019-08-01T00:53:34.7556283Z 
2019-08-01T00:53:34.7556513Z 130 LL | const fn foo30(x: *const u32) -> usize { x as usize }
2019-08-01T00:53:34.7556666Z 132    |
2019-08-01T00:53:34.7557098Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7557098Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7557368Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7558033Z 134    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7558179Z 136 error[E0723]: casting pointers to ints is unstable in const fn
2019-08-01T00:53:34.7558222Z 
2019-08-01T00:53:34.7558222Z 
2019-08-01T00:53:34.7558699Z 139 LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
2019-08-01T00:53:34.7559015Z 141    |
2019-08-01T00:53:34.7559330Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7559330Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7560125Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7560233Z 143    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7560414Z 145 error[E0723]: casting pointers to ints is unstable in const fn
2019-08-01T00:53:34.7560485Z 
2019-08-01T00:53:34.7560485Z 
2019-08-01T00:53:34.7560816Z 148 LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
2019-08-01T00:53:34.7560993Z 150    |
2019-08-01T00:53:34.7561308Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7561308Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7561642Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7561758Z 152    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7561923Z 154 error[E0723]: casting pointers to ints is unstable in const fn
2019-08-01T00:53:34.7561974Z 
2019-08-01T00:53:34.7561974Z 
2019-08-01T00:53:34.7562286Z 157 LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
2019-08-01T00:53:34.7562486Z 159    |
2019-08-01T00:53:34.7562775Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7562775Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7563117Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7563368Z 161    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7563445Z 162 
2019-08-01T00:53:34.7563507Z 163 error[E0723]: loops and conditional expressions are not stable in const fn
2019-08-01T00:53:34.7563567Z 
2019-08-01T00:53:34.7563788Z 166 LL | const fn foo30_4(b: bool) -> usize { if b { 1 } else { 42 } }
2019-08-01T00:53:34.7563933Z 168    |
2019-08-01T00:53:34.7564182Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7564182Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7564437Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7564541Z 170    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7564600Z 171 
2019-08-01T00:53:34.7564862Z 172 error[E0723]: loops are not allowed in const fn
2019-08-01T00:53:34.7565079Z 
2019-08-01T00:53:34.7565152Z 175 LL | const fn foo30_5(b: bool) { while b { } }
2019-08-01T00:53:34.7565296Z 177    |
2019-08-01T00:53:34.7565548Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7565548Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7565838Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7565919Z 179    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7565997Z 180 
2019-08-01T00:53:34.7566056Z 181 error[E0723]: loops and conditional expressions are not stable in const fn
2019-08-01T00:53:34.7566230Z 
2019-08-01T00:53:34.7566478Z 184 LL | const fn foo36(a: bool, b: bool) -> bool { a && b }
2019-08-01T00:53:34.7566625Z 186    |
2019-08-01T00:53:34.7566889Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7566889Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7567160Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7567343Z 188    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7567412Z 189 
2019-08-01T00:53:34.7567489Z 190 error[E0723]: loops and conditional expressions are not stable in const fn
2019-08-01T00:53:34.7567532Z 
2019-08-01T00:53:34.7567794Z 193 LL | const fn foo37(a: bool, b: bool) -> bool { a || b }
2019-08-01T00:53:34.7567941Z 195    |
2019-08-01T00:53:34.7568343Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7568343Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7568635Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7568715Z 197    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7569033Z 199 error[E0723]: mutable references in const fn are unstable
2019-08-01T00:53:34.7569088Z 
2019-08-01T00:53:34.7569088Z 
2019-08-01T00:53:34.7569146Z 202 LL | const fn inc(x: &mut i32) { *x += 1 }
2019-08-01T00:53:34.7569269Z 204    |
2019-08-01T00:53:34.7570005Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7570005Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7570339Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7570455Z 206    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7570530Z 207 
2019-08-01T00:53:34.7570634Z 208 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7570690Z 
2019-08-01T00:53:34.7570769Z 211 LL | impl<T: std::fmt::Debug> Foo<T> {
2019-08-01T00:53:34.7570916Z 213    |
2019-08-01T00:53:34.7571211Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7571211Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7571558Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7571658Z 215    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7571750Z 216 
2019-08-01T00:53:34.7571823Z 217 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7571897Z 
2019-08-01T00:53:34.7571963Z 220 LL | impl<T: std::fmt::Debug + Sized> Foo<T> {
2019-08-01T00:53:34.7572111Z 222    |
2019-08-01T00:53:34.7572432Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7572432Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7572754Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7572867Z 224    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7572940Z 225 
2019-08-01T00:53:34.7573033Z 226 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7573269Z 
2019-08-01T00:53:34.7573508Z 229 LL | impl<T: Sync + Sized> Foo<T> {
2019-08-01T00:53:34.7573638Z 231    |
2019-08-01T00:53:34.7573888Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7573888Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7574377Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7574471Z 233    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7574657Z 234 
2019-08-01T00:53:34.7574713Z 235 error[E0723]: `impl Trait` in const fn is unstable
2019-08-01T00:53:34.7574753Z 
2019-08-01T00:53:34.7575227Z 238 LL | const fn no_rpit2() -> AlanTuring<impl std::fmt::Debug> { AlanTuring(0) }
2019-08-01T00:53:34.7575402Z 240    |
2019-08-01T00:53:34.7575796Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7575796Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7576467Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7576575Z 242    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7576642Z 243 
2019-08-01T00:53:34.7576885Z 244 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7576934Z 
2019-08-01T00:53:34.7577015Z 247 LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
2019-08-01T00:53:34.7577410Z 249    |
2019-08-01T00:53:34.7577821Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7577821Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7578158Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7578242Z 251    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7578332Z 252 
2019-08-01T00:53:34.7578393Z 253 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7578457Z 
2019-08-01T00:53:34.7578512Z 256 LL | const fn no_apit(_x: impl std::fmt::Debug) {}
2019-08-01T00:53:34.7578654Z 258    |
2019-08-01T00:53:34.7578927Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7578927Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7579199Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7579305Z 260    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7579366Z 261 
2019-08-01T00:53:34.7579440Z 262 error[E0723]: `impl Trait` in const fn is unstable
2019-08-01T00:53:34.7579480Z 
2019-08-01T00:53:34.7580124Z 265 LL | const fn no_rpit() -> impl std::fmt::Debug {}
2019-08-01T00:53:34.7580314Z 267    |
2019-08-01T00:53:34.7580630Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7580630Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7580968Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7581065Z 269    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7581162Z 270 
2019-08-01T00:53:34.7581235Z 271 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7581299Z 
2019-08-01T00:53:34.7581383Z 274 LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {}
2019-08-01T00:53:34.7581539Z 276    |
2019-08-01T00:53:34.7581834Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7581834Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7582184Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7582302Z 278    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7582376Z 279 
2019-08-01T00:53:34.7582468Z 280 error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7582522Z 
2019-08-01T00:53:34.7582795Z 283 LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-08-01T00:53:34.7582975Z 285    |
2019-08-01T00:53:34.7584020Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7584020Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7584313Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7584391Z 287    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7584521Z 289 error[E0515]: cannot return reference to temporary value
2019-08-01T00:53:34.7584658Z 
2019-08-01T00:53:34.7584658Z 
2019-08-01T00:53:34.7584744Z 301 LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
2019-08-01T00:53:34.7584894Z 303    |
2019-08-01T00:53:34.7585176Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7585176Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7586214Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7586341Z 305    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7586472Z 307 error[E0723]: function pointers in const fn are unstable
2019-08-01T00:53:34.7586511Z 
2019-08-01T00:53:34.7586511Z 
2019-08-01T00:53:34.7586717Z 310 LL | const fn no_fn_ptrs(_x: fn()) {}
2019-08-01T00:53:34.7586843Z 312    |
2019-08-01T00:53:34.7587163Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7587163Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7587423Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7587691Z 314    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7588007Z 316 error[E0723]: function pointers in const fn are unstable
2019-08-01T00:53:34.7588048Z 
2019-08-01T00:53:34.7588048Z 
2019-08-01T00:53:34.7588287Z 319 LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
2019-08-01T00:53:34.7588430Z 321    |
2019-08-01T00:53:34.7588677Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7588677Z -    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-08-01T00:53:34.7588964Z +    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7589062Z 323    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7589203Z 325 error: aborting due to 37 previous errors
2019-08-01T00:53:34.7589242Z 
2019-08-01T00:53:34.7589270Z 
2019-08-01T00:53:34.7589343Z The actual stderr differed from the expected stderr.
2019-08-01T00:53:34.7589343Z The actual stderr differed from the expected stderr.
2019-08-01T00:53:34.7590480Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll/min_const_fn.nll.stderr
2019-08-01T00:53:34.7590808Z To update references, rerun the tests and pass the `--bless` flag
2019-08-01T00:53:34.7591117Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`
2019-08-01T00:53:34.7591265Z error: 1 errors occurred comparing output.
2019-08-01T00:53:34.7591352Z status: exit code: 1
2019-08-01T00:53:34.7591352Z status: exit code: 1
2019-08-01T00:53:34.7592272Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll/auxiliary" "-A" "unused"
2019-08-01T00:53:34.7592793Z ------------------------------------------
2019-08-01T00:53:34.7592843Z 
2019-08-01T00:53:34.7593414Z ------------------------------------------
2019-08-01T00:53:34.7593487Z stderr:
2019-08-01T00:53:34.7593487Z stderr:
2019-08-01T00:53:34.7593673Z ------------------------------------------
2019-08-01T00:53:34.7593908Z error[E0493]: destructors cannot be evaluated at compile-time
2019-08-01T00:53:34.7594213Z    |
2019-08-01T00:53:34.7594213Z    |
2019-08-01T00:53:34.7594524Z LL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-08-01T00:53:34.7594612Z    |                         ^^^^ constant functions cannot evaluate destructors
2019-08-01T00:53:34.7594729Z error[E0723]: mutable references in const fn are unstable
2019-08-01T00:53:34.7594994Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:36
2019-08-01T00:53:34.7595054Z    |
2019-08-01T00:53:34.7595054Z    |
2019-08-01T00:53:34.7595277Z LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
2019-08-01T00:53:34.7595419Z    |
2019-08-01T00:53:34.7595419Z    |
2019-08-01T00:53:34.7595658Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7595753Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7595795Z 
2019-08-01T00:53:34.7595998Z error[E0493]: destructors cannot be evaluated at compile-time
2019-08-01T00:53:34.7596304Z    |
2019-08-01T00:53:34.7596304Z    |
2019-08-01T00:53:34.7596547Z LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-08-01T00:53:34.7596641Z    |                            ^^^^ constant functions cannot evaluate destructors
2019-08-01T00:53:34.7596737Z error[E0723]: mutable references in const fn are unstable
2019-08-01T00:53:34.7596981Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:46:42
2019-08-01T00:53:34.7597040Z    |
2019-08-01T00:53:34.7597040Z    |
2019-08-01T00:53:34.7597274Z LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
2019-08-01T00:53:34.7597414Z    |
2019-08-01T00:53:34.7597414Z    |
2019-08-01T00:53:34.7597651Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7597745Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7597786Z 
2019-08-01T00:53:34.7598014Z error[E0493]: destructors cannot be evaluated at compile-time
2019-08-01T00:53:34.7598714Z    |
2019-08-01T00:53:34.7598714Z    |
2019-08-01T00:53:34.7598942Z LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructors
2019-08-01T00:53:34.7599037Z    |                           ^^^^ constant functions cannot evaluate destructors
2019-08-01T00:53:34.7599155Z error[E0723]: mutable references in const fn are unstable
2019-08-01T00:53:34.7599400Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:53:38
2019-08-01T00:53:34.7599479Z    |
2019-08-01T00:53:34.7599479Z    |
2019-08-01T00:53:34.7599695Z LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
2019-08-01T00:53:34.7600180Z    |
2019-08-01T00:53:34.7600180Z    |
2019-08-01T00:53:34.7600540Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7600648Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7600783Z error[E0723]: mutable references in const fn are unstable
2019-08-01T00:53:34.7601084Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:58:39
2019-08-01T00:53:34.7601158Z    |
2019-08-01T00:53:34.7601158Z    |
2019-08-01T00:53:34.7601436Z LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
2019-08-01T00:53:34.7601604Z    |
2019-08-01T00:53:34.7601604Z    |
2019-08-01T00:53:34.7602020Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7602132Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7602185Z 
2019-08-01T00:53:34.7602276Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7602720Z    |
2019-08-01T00:53:34.7602720Z    |
2019-08-01T00:53:34.7603001Z LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
2019-08-01T00:53:34.7603155Z    |
2019-08-01T00:53:34.7603155Z    |
2019-08-01T00:53:34.7603811Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7603887Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7603946Z 
2019-08-01T00:53:34.7604002Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-08-01T00:53:34.7604314Z    |
2019-08-01T00:53:34.7604314Z    |
2019-08-01T00:53:34.7604524Z LL | const fn foo11_2<T: Send>(t: T) -> T { t }
2019-08-01T00:53:34.7604647Z    |
2019-08-01T00:53:34.7604647Z    |
2019-08-01T00:53:34.7604883Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7604981Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7605022Z 
2019-08-01T00:53:34.7605076Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-08-01T00:53:34.7605395Z    |
2019-08-01T00:53:34.7605395Z    |
2019-08-01T00:53:34.7605588Z LL | const fn foo19(f: f32) -> f32 { f * 2.0 }
2019-08-01T00:53:34.7605716Z    |
2019-08-01T00:53:34.7605716Z    |
2019-08-01T00:53:34.7605969Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7606053Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7606094Z 
2019-08-01T00:53:34.7606175Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-08-01T00:53:34.7606476Z    |
2019-08-01T00:53:34.7606476Z    |
2019-08-01T00:53:34.7606677Z LL | const fn foo19_2(f: f32) -> f32 { 2.0 - f }
2019-08-01T00:53:34.7606806Z    |
2019-08-01T00:53:34.7606806Z    |
2019-08-01T00:53:34.7607058Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7607149Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7607190Z 
2019-08-01T00:53:34.7607244Z error[E0723]: only int and `bool` operations are stable in const fn
2019-08-01T00:53:34.7607555Z    |
2019-08-01T00:53:34.7607555Z    |
2019-08-01T00:53:34.7607763Z LL | const fn foo19_3(f: f32) -> f32 { -f }
2019-08-01T00:53:34.7607887Z    |
2019-08-01T00:53:34.7607887Z    |
2019-08-01T00:53:34.7608121Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7608221Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7608261Z 
2019-08-01T00:53:34.7608335Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-08-01T00:53:34.7608635Z    |
2019-08-01T00:53:34.7608635Z    |
2019-08-01T00:53:34.7608835Z LL | const fn foo19_4(f: f32, g: f32) -> f32 { f / g }
2019-08-01T00:53:34.7608969Z    |
2019-08-01T00:53:34.7608969Z    |
2019-08-01T00:53:34.7609491Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7610022Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-08-01T00:53:34.7610095Z 
2019-08-01T00:53:34.7610160Z error[E0723]: cannot access `static` items in const fn
2019-08-01T00:53:34.7610591Z    |
2019-08-01T00:53:34.7610591Z    |
2019-08-01T00:53:34.7610988Z LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot access `static` items in const fn
2019-08-01T00:53:34.7611165Z    |
2019-08-01T00:53:34.7611165Z    |
2019-08-01T00:53:34.7611486Z    = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
2019-08-01T00:53:34.7611601Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
---
2019-08-01T00:53:34.7645018Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:533:22
2019-08-01T00:53:34.7645102Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-08-01T00:53:34.7645165Z 
2019-08-01T00:53:34.7645195Z 
2019-08-01T00:53:34.7647142Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.38.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-08-01T00:53:34.7647681Z 
2019-08-01T00:53:34.7647711Z 
2019-08-01T00:53:34.7647765Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-01T00:53:34.7647847Z Build completed unsuccessfully in 1:56:25
2019-08-01T00:53:34.7647847Z Build completed unsuccessfully in 1:56:25
2019-08-01T00:53:35.6163272Z ##[error]Bash exited with code '1'.
2019-08-01T00:53:35.6226342Z ##[section]Starting: Upload CPU usage statistics
2019-08-01T00:53:35.6232641Z ==============================================================================
2019-08-01T00:53:35.6232733Z Task         : Bash
2019-08-01T00:53:35.6232819Z Description  : Run a Bash script on macOS, Linux, or Windows
