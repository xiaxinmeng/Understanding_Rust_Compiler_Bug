plain
2019-09-13T15:34:52.4124300Z 329 
2019-09-13T15:34:52.4124375Z 
2019-09-13T15:34:52.4124432Z 
2019-09-13T15:34:52.4124499Z The actual stderr differed from the expected stderr.
2019-09-13T15:34:52.4125062Z Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll/min_const_fn.nll.stderr
2019-09-13T15:34:52.4125900Z To update references, rerun the tests and pass the `--bless` flag
2019-09-13T15:34:52.4127322Z To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`
2019-09-13T15:34:52.4127738Z error: 1 errors occurred comparing output.
2019-09-13T15:34:52.4127984Z status: exit code: 1
2019-09-13T15:34:52.4127984Z status: exit code: 1
2019-09-13T15:34:52.4128978Z command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll" "-Zborrowck=mir" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn.nll/auxiliary" "-A" "unused"
2019-09-13T15:34:52.4130242Z ------------------------------------------
2019-09-13T15:34:52.4130312Z 
2019-09-13T15:34:52.4130794Z ------------------------------------------
2019-09-13T15:34:52.4131006Z stderr:
2019-09-13T15:34:52.4131006Z stderr:
2019-09-13T15:34:52.4131382Z ------------------------------------------
2019-09-13T15:34:52.4131668Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-13T15:34:52.4131987Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:37:25
2019-09-13T15:34:52.4132070Z    |
2019-09-13T15:34:52.4134019Z LL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-09-13T15:34:52.4134173Z    |                         ^^^^ constant functions cannot evaluate destructors
2019-09-13T15:34:52.4134306Z error[E0723]: mutable references in const fn are unstable
2019-09-13T15:34:52.4134672Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:36
2019-09-13T15:34:52.4134750Z    |
2019-09-13T15:34:52.4134750Z    |
2019-09-13T15:34:52.4135042Z LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
2019-09-13T15:34:52.4135224Z    |
2019-09-13T15:34:52.4135530Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4135647Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4135700Z 
2019-09-13T15:34:52.4135700Z 
2019-09-13T15:34:52.4135986Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-13T15:34:52.4136283Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:44:28
2019-09-13T15:34:52.4136422Z    |
2019-09-13T15:34:52.4136731Z LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructors cannot be evaluated
2019-09-13T15:34:52.4136850Z    |                            ^^^^ constant functions cannot evaluate destructors
2019-09-13T15:34:52.4136993Z error[E0723]: mutable references in const fn are unstable
2019-09-13T15:34:52.4137282Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:46:42
2019-09-13T15:34:52.4137386Z    |
2019-09-13T15:34:52.4137386Z    |
2019-09-13T15:34:52.4138009Z LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
2019-09-13T15:34:52.4138374Z    |
2019-09-13T15:34:52.4138739Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4139002Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4139097Z 
2019-09-13T15:34:52.4139097Z 
2019-09-13T15:34:52.4139990Z error[E0493]: destructors cannot be evaluated at compile-time
2019-09-13T15:34:52.4140322Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:51:27
2019-09-13T15:34:52.4140613Z    |
2019-09-13T15:34:52.4143565Z LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructors
2019-09-13T15:34:52.4144022Z    |                           ^^^^ constant functions cannot evaluate destructors
2019-09-13T15:34:52.4144235Z error[E0723]: mutable references in const fn are unstable
2019-09-13T15:34:52.4144941Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:53:38
2019-09-13T15:34:52.4145192Z    |
2019-09-13T15:34:52.4145192Z    |
2019-09-13T15:34:52.4145540Z LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
2019-09-13T15:34:52.4145854Z    |
2019-09-13T15:34:52.4146317Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4148212Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4148315Z 
2019-09-13T15:34:52.4148315Z 
2019-09-13T15:34:52.4148448Z error[E0723]: mutable references in const fn are unstable
2019-09-13T15:34:52.4148836Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:58:39
2019-09-13T15:34:52.4148933Z    |
2019-09-13T15:34:52.4150107Z LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
2019-09-13T15:34:52.4150305Z    |
2019-09-13T15:34:52.4150629Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4150927Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4151031Z 
2019-09-13T15:34:52.4151031Z 
2019-09-13T15:34:52.4151133Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-13T15:34:52.4151927Z    |
2019-09-13T15:34:52.4151927Z    |
2019-09-13T15:34:52.4152322Z LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
2019-09-13T15:34:52.4152849Z    |
2019-09-13T15:34:52.4153458Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4153705Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4153983Z 
2019-09-13T15:34:52.4153983Z 
2019-09-13T15:34:52.4154082Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-13T15:34:52.4154943Z    |
2019-09-13T15:34:52.4154943Z    |
2019-09-13T15:34:52.4155288Z LL | const fn foo11_2<T: Send>(t: T) -> T { t }
2019-09-13T15:34:52.4155442Z    |
2019-09-13T15:34:52.4155734Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4156003Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4156087Z 
2019-09-13T15:34:52.4156087Z 
2019-09-13T15:34:52.4156176Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-09-13T15:34:52.4156856Z    |
2019-09-13T15:34:52.4156856Z    |
2019-09-13T15:34:52.4157182Z LL | const fn foo19(f: f32) -> f32 { f * 2.0 }
2019-09-13T15:34:52.4157363Z    |
2019-09-13T15:34:52.4157665Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4157924Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4158003Z 
2019-09-13T15:34:52.4158003Z 
2019-09-13T15:34:52.4158113Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-09-13T15:34:52.4158670Z    |
2019-09-13T15:34:52.4158670Z    |
2019-09-13T15:34:52.4159004Z LL | const fn foo19_2(f: f32) -> f32 { 2.0 - f }
2019-09-13T15:34:52.4159851Z    |
2019-09-13T15:34:52.4160265Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4160365Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4160442Z 
2019-09-13T15:34:52.4160442Z 
2019-09-13T15:34:52.4160511Z error[E0723]: only int and `bool` operations are stable in const fn
2019-09-13T15:34:52.4161026Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:84:35
2019-09-13T15:34:52.4161104Z    |
2019-09-13T15:34:52.4161375Z LL | const fn foo19_3(f: f32) -> f32 { -f }
2019-09-13T15:34:52.4161535Z    |
2019-09-13T15:34:52.4161836Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4161964Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4162019Z 
2019-09-13T15:34:52.4162019Z 
2019-09-13T15:34:52.4162107Z error[E0723]: only int, `bool` and `char` operations are stable in const fn
2019-09-13T15:34:52.4162500Z    |
2019-09-13T15:34:52.4162500Z    |
2019-09-13T15:34:52.4162766Z LL | const fn foo19_4(f: f32, g: f32) -> f32 { f / g }
2019-09-13T15:34:52.4162934Z    |
2019-09-13T15:34:52.4163265Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4163360Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4163430Z 
2019-09-13T15:34:52.4163430Z 
2019-09-13T15:34:52.4163498Z error[E0723]: cannot access `static` items in const fn
2019-09-13T15:34:52.4163881Z    |
2019-09-13T15:34:52.4163881Z    |
2019-09-13T15:34:52.4164295Z LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot access `static` items in const fn
2019-09-13T15:34:52.4164479Z    |
2019-09-13T15:34:52.4164806Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4164921Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4164974Z 
2019-09-13T15:34:52.4164974Z 
2019-09-13T15:34:52.4165059Z error[E0723]: cannot access `static` items in const fn
2019-09-13T15:34:52.4165455Z    |
2019-09-13T15:34:52.4165455Z    |
2019-09-13T15:34:52.4165757Z LL | const fn foo26() -> &'static u32 { &BAR } //~ ERROR cannot access `static` items
2019-09-13T15:34:52.4165934Z    |
2019-09-13T15:34:52.4166254Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4166357Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4166429Z 
2019-09-13T15:34:52.4166429Z 
2019-09-13T15:34:52.4166661Z error[E0723]: casting pointers to ints is unstable in const fn
2019-09-13T15:34:52.4167176Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:92:42
2019-09-13T15:34:52.4167254Z    |
2019-09-13T15:34:52.4167538Z LL | const fn foo30(x: *const u32) -> usize { x as usize }
2019-09-13T15:34:52.4167835Z    |
2019-09-13T15:34:52.4168142Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4168254Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4168306Z 
2019-09-13T15:34:52.4168306Z 
2019-09-13T15:34:52.4168374Z error[E0723]: casting pointers to ints is unstable in const fn
2019-09-13T15:34:52.4168683Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:94:63
2019-09-13T15:34:52.4168777Z    |
2019-09-13T15:34:52.4169078Z LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
2019-09-13T15:34:52.4170036Z    |
2019-09-13T15:34:52.4170459Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4170707Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4170811Z 
2019-09-13T15:34:52.4170811Z 
2019-09-13T15:34:52.4170903Z error[E0723]: casting pointers to ints is unstable in const fn
2019-09-13T15:34:52.4171570Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:96:42
2019-09-13T15:34:52.4171790Z    |
2019-09-13T15:34:52.4172150Z LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
2019-09-13T15:34:52.4172488Z    |
2019-09-13T15:34:52.4172846Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4173092Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4173197Z 
2019-09-13T15:34:52.4173197Z 
2019-09-13T15:34:52.4173301Z error[E0723]: casting pointers to ints is unstable in const fn
2019-09-13T15:34:52.4173655Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:98:63
2019-09-13T15:34:52.4173734Z    |
2019-09-13T15:34:52.4174047Z LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
2019-09-13T15:34:52.4174432Z    |
2019-09-13T15:34:52.4174771Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4174887Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4175076Z 
2019-09-13T15:34:52.4175076Z 
2019-09-13T15:34:52.4175203Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-09-13T15:34:52.4175678Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:100:38
2019-09-13T15:34:52.4175789Z    |
2019-09-13T15:34:52.4176310Z LL | const fn foo30_4(b: bool) -> usize { if b { 1 } else { 42 } }
2019-09-13T15:34:52.4176672Z    |
2019-09-13T15:34:52.4177034Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4177282Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4177378Z 
2019-09-13T15:34:52.4177378Z 
2019-09-13T15:34:52.4177484Z error[E0723]: loops are not allowed in const fn
2019-09-13T15:34:52.4177842Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:102:29
2019-09-13T15:34:52.4177939Z    |
2019-09-13T15:34:52.4178313Z LL | const fn foo30_5(b: bool) { while b { } }
2019-09-13T15:34:52.4178503Z    |
2019-09-13T15:34:52.4178864Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4178958Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4179157Z 
2019-09-13T15:34:52.4179157Z 
2019-09-13T15:34:52.4180072Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-09-13T15:34:52.4180534Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:105:44
2019-09-13T15:34:52.4180634Z    |
2019-09-13T15:34:52.4180904Z LL | const fn foo36(a: bool, b: bool) -> bool { a && b }
2019-09-13T15:34:52.4181241Z    |
2019-09-13T15:34:52.4181710Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4181810Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4182019Z 
2019-09-13T15:34:52.4182019Z 
2019-09-13T15:34:52.4182219Z error[E0723]: loops and conditional expressions are not stable in const fn
2019-09-13T15:34:52.4182606Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:107:44
2019-09-13T15:34:52.4182840Z    |
2019-09-13T15:34:52.4183534Z LL | const fn foo37(a: bool, b: bool) -> bool { a || b }
2019-09-13T15:34:52.4183904Z    |
2019-09-13T15:34:52.4184315Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4184573Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4185029Z 
2019-09-13T15:34:52.4185029Z 
2019-09-13T15:34:52.4185243Z error[E0723]: mutable references in const fn are unstable
2019-09-13T15:34:52.4185665Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:109:14
2019-09-13T15:34:52.4185745Z    |
2019-09-13T15:34:52.4185825Z LL | const fn inc(x: &mut i32) { *x += 1 }
2019-09-13T15:34:52.4186129Z    |
2019-09-13T15:34:52.4186507Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4186637Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4186823Z 
2019-09-13T15:34:52.4186823Z 
2019-09-13T15:34:52.4186991Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-13T15:34:52.4187577Z    |
2019-09-13T15:34:52.4187577Z    |
2019-09-13T15:34:52.4187755Z LL | impl<T: std::fmt::Debug> Foo<T> {
2019-09-13T15:34:52.4188090Z    |
2019-09-13T15:34:52.4188591Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4188846Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4188927Z 
2019-09-13T15:34:52.4188927Z 
2019-09-13T15:34:52.4189002Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-13T15:34:52.4190396Z    |
2019-09-13T15:34:52.4190396Z    |
2019-09-13T15:34:52.4190633Z LL | impl<T: std::fmt::Debug + Sized> Foo<T> {
2019-09-13T15:34:52.4190959Z    |
2019-09-13T15:34:52.4191377Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:52.4191611Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:52.4191715Z 
2019-09-13T15:34:52.4191715Z 
2019-09-13T15:34:52.4191815Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-13T15:34:52.4192898Z    |
2019-09-13T15:34:52.4192898Z    |
2019-09-13T15:34:53.2864561Z LL | impl<T: Sync + Sized> Foo<T> {
2019-09-13T15:34:53.2876195Z    |
2019-09-13T15:34:53.2877129Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:53.2877454Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:53.2877580Z 
2019-09-13T15:34:53.2877580Z 
2019-09-13T15:34:53.2877693Z error[E0723]: `impl Trait` in const fn is unstable
2019-09-13T15:34:53.2878346Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:130:24
2019-09-13T15:34:53.2878582Z    |
2019-09-13T15:34:53.2878939Z LL | const fn no_rpit2() -> AlanTuring<impl std::fmt::Debug> { AlanTuring(0) }
2019-09-13T15:34:53.2879289Z    |
2019-09-13T15:34:53.2879739Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:53.2880009Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:53.2880308Z 
2019-09-13T15:34:53.2880308Z 
2019-09-13T15:34:53.2880419Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-13T15:34:53.2881106Z    |
2019-09-13T15:34:53.2881106Z    |
2019-09-13T15:34:53.2881212Z LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
2019-09-13T15:34:53.2881701Z    |
2019-09-13T15:34:53.2882252Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:53.2882362Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:53.2882411Z 
2019-09-13T15:34:53.2882411Z 
2019-09-13T15:34:53.2882478Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-13T15:34:53.2884298Z    |
2019-09-13T15:34:53.2884298Z    |
2019-09-13T15:34:53.2884613Z LL | const fn no_apit(_x: impl std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
2019-09-13T15:34:53.2884783Z    |
2019-09-13T15:34:53.2885360Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:53.2885607Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:53.2885804Z 
2019-09-13T15:34:53.2885804Z 
2019-09-13T15:34:53.2885869Z error[E0723]: `impl Trait` in const fn is unstable
2019-09-13T15:34:53.2886342Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:135:23
2019-09-13T15:34:53.2886548Z    |
2019-09-13T15:34:53.2886925Z LL | const fn no_rpit() -> impl std::fmt::Debug {} //~ ERROR `impl Trait` in const fn is unstable
2019-09-13T15:34:53.2887484Z    |
2019-09-13T15:34:53.2887841Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:53.2888075Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:53.2888249Z 
2019-09-13T15:34:53.2888249Z 
2019-09-13T15:34:53.2888321Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-13T15:34:53.2889200Z    |
2019-09-13T15:34:53.2889200Z    |
2019-09-13T15:34:53.2889447Z LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {} //~ ERROR trait bounds other than `Sized`
2019-09-13T15:34:53.2889635Z    |
2019-09-13T15:34:53.2890117Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:53.2892712Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:53.2892844Z 
2019-09-13T15:34:53.2892844Z 
2019-09-13T15:34:53.2892948Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-13T15:34:53.2893925Z    |
2019-09-13T15:34:53.2893925Z    |
2019-09-13T15:34:53.2894385Z LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-09-13T15:34:53.2894987Z    |
2019-09-13T15:34:53.2895360Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:53.2895472Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:53.2895522Z 
2019-09-13T15:34:53.2895522Z 
2019-09-13T15:34:53.2895760Z error[E0515]: cannot return reference to temporary value
2019-09-13T15:34:53.2896142Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:137:63
2019-09-13T15:34:53.2896236Z    |
2019-09-13T15:34:53.2896685Z LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
2019-09-13T15:34:53.2897419Z    |                                                               ||
2019-09-13T15:34:53.2897537Z    |                                                               |temporary value created here
2019-09-13T15:34:53.2897658Z    |                                                               returns a reference to data owned by the current function
2019-09-13T15:34:53.2897830Z 
2019-09-13T15:34:53.2897830Z 
2019-09-13T15:34:53.2898045Z error[E0723]: trait bounds other than `Sized` on const fn parameters are unstable
2019-09-13T15:34:53.2898735Z    |
2019-09-13T15:34:53.2898735Z    |
2019-09-13T15:34:53.2899119Z LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
2019-09-13T15:34:53.2899615Z    |
2019-09-13T15:34:53.2899954Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:53.2900428Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:53.2900507Z 
2019-09-13T15:34:53.2900507Z 
2019-09-13T15:34:53.2900590Z error[E0723]: function pointers in const fn are unstable
2019-09-13T15:34:53.2901049Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:148:21
2019-09-13T15:34:53.2901282Z    |
2019-09-13T15:34:53.2901533Z LL | const fn no_fn_ptrs(_x: fn()) {}
2019-09-13T15:34:53.2901879Z    |
2019-09-13T15:34:53.2902217Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:53.2902472Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:53.2902552Z 
2019-09-13T15:34:53.2902552Z 
2019-09-13T15:34:53.2902634Z error[E0723]: function pointers in const fn are unstable
2019-09-13T15:34:53.2903056Z   --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:150:27
2019-09-13T15:34:53.2903298Z    |
2019-09-13T15:34:53.2903658Z LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
2019-09-13T15:34:53.2903955Z    |
2019-09-13T15:34:53.2904530Z    = note: for more information, see issue https://github.com/rust-lang/rust/issues/57563
2019-09-13T15:34:53.2904769Z    = help: add `#![feature(const_fn)]` to the crate attributes to enable
2019-09-13T15:34:53.2904870Z 
---
2019-09-13T15:34:53.2908290Z thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:536:22
2019-09-13T15:34:53.2908523Z note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
2019-09-13T15:34:53.2908621Z 
2019-09-13T15:34:53.2908686Z 
2019-09-13T15:34:53.2910533Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--llvm-version" "9.0.0-rust-1.39.0-dev\n" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"
2019-09-13T15:34:53.2911373Z 
2019-09-13T15:34:53.2911413Z 
2019-09-13T15:34:53.2911743Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-09-13T15:34:53.2911871Z Build completed unsuccessfully in 2:08:08
2019-09-13T15:34:53.2911871Z Build completed unsuccessfully in 2:08:08
2019-09-13T15:34:53.2911982Z == clock drift check ==
2019-09-13T15:34:53.2912067Z   local time: Fri Sep 13 15:34:52 UTC 2019
2019-09-13T15:34:53.2912174Z   network time: Fri, 13 Sep 2019 15:34:52 GMT
2019-09-13T15:34:53.2912239Z == end clock drift check ==
2019-09-13T15:34:53.4864787Z ##[error]Bash exited with code '1'.
2019-09-13T15:34:53.4902317Z ##[section]Starting: Upload CPU usage statistics
2019-09-13T15:34:53.4907476Z ==============================================================================
2019-09-13T15:34:53.4907573Z Task         : Bash
2019-09-13T15:34:53.4907660Z Description  : Run a Bash script on macOS, Linux, or Windows
