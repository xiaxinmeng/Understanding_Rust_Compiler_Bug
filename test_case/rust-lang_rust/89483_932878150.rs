plain
.................................................................................................... 1900/12240
.................................................................................................... 2000/12240
.....i.............................................................................................. 2100/12240
.................................................................................................... 2200/12240
...................F..............................................F................................. 2300/12240
............F....................................................................................... 2400/12240
.................................................................................................... 2500/12240
.F......F...................F.............................................................F......... 2600/12240
......F............................................................................................. 2700/12240
.................................................................................................... 2900/12240
....................................iiiii........................................................... 3000/12240
.................................................................................................... 3100/12240
.................................................................................................... 3200/12240
---
.................................................i..........................F....................... 6000/12240
.................................................................................................... 6100/12240
...............................................i.................................................... 6200/12240
...................i................................................................................ 6300/12240
..........................................................................F...ii.ii....F..i...i..... 6400/12240
........................................F........................................................... 6500/12240
..........i..............................................i.......................................... 6700/12240
..........i..............................................i.......................................... 6700/12240
.........................................................i...............F....................F..... 6800/12240
.................................................................................................... 7000/12240
.......ii.......................................................i................................... 7100/12240
.......................................................F............................................ 7200/12240
.................................................................................................... 7300/12240
.................................................................................................... 7300/12240
.................................................................................................... 7400/12240
...............................................F..F........ii................i.....i.ii............. 7500/12240
.....................................................................................F......F....... 7600/12240
.................................................................................................... 7800/12240
.................................................................................................... 7900/12240
............................i..ii..............................................................ii... 8000/12240
.................................................................................................... 8100/12240
---
.................................................................................................... 9400/12240
.................................................................................................... 9500/12240
.................................................................................................... 9600/12240
.................................................................................................... 9700/12240
.................................F.F................................................................ 9800/12240
..............................................................F...............ii.i.................. 9900/12240
.................................................................................................... 10000/12240
.........................................................................iiiiii.i..iiiiiiF.i........ 10100/12240
.................................................................................................... 10300/12240
.................................................................................................... 10400/12240
.................................................................................................... 10500/12240
.................................................................................................... 10600/12240
.................................................................................................... 10600/12240
.................................................................................................... 10700/12240
.................................................................................................... 10800/12240
.................................................................................................... 10900/12240
......ii...............................i.......F.................................................... 11000/12240
.................................................................................................... 11100/12240
.................................................................................................... 11200/12240
.................................................................................................... 11300/12240
..................F..F.FF.F.................F.F...F.FFF............................................. 11400/12240
.................................................................................................... 11600/12240
.................................................................................................... 11700/12240
.................................................................................................... 11800/12240
.................................................................................................... 11900/12240
---
- warning: anonymous parameters are deprecated and will be removed in the next edition.
+ warning: anonymous parameters are deprecated and will be removed in the next edition
16   --> $DIR/anon-params-deprecated.rs:12:30
17    |
18 LL |     fn bar_with_default_impl(String, String) {}
21    = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
22    = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>
23 
- warning: anonymous parameters are deprecated and will be removed in the next edition.
- warning: anonymous parameters are deprecated and will be removed in the next edition.
+ warning: anonymous parameters are deprecated and will be removed in the next edition
25   --> $DIR/anon-params-deprecated.rs:12:38
26    |
27 LL |     fn bar_with_default_impl(String, String) {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-deprecated/anon-params-deprecated.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args anon-params/anon-params-deprecated.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/anon-params/anon-params-deprecated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-deprecated" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/anon-params/anon-params-deprecated/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: anonymous parameters are deprecated and will be removed in the next edition
  --> /checkout/src/test/ui/anon-params/anon-params-deprecated.rs:9:12
   |
LL |     fn foo(i32); //~ WARNING anonymous parameters are deprecated
   |            ^^^ help: try naming the parameter or explicitly ignoring it: `_: i32`
note: the lint level is defined here
  --> /checkout/src/test/ui/anon-params/anon-params-deprecated.rs:1:9
   |
LL | #![warn(anonymous_parameters)]
LL | #![warn(anonymous_parameters)]
   |         ^^^^^^^^^^^^^^^^^^^^
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>

warning: anonymous parameters are deprecated and will be removed in the next edition
  --> /checkout/src/test/ui/anon-params/anon-params-deprecated.rs:12:30
   |
LL |     fn bar_with_default_impl(String, String) {}
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


warning: anonymous parameters are deprecated and will be removed in the next edition
  --> /checkout/src/test/ui/anon-params/anon-params-deprecated.rs:12:38
   |
LL |     fn bar_with_default_impl(String, String) {}
   |
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>

---
- error: pointers cannot be reliably compared during const eval.
+ error: pointers cannot be reliably compared during const eval
2   --> $DIR/const_raw_ptr_ops.rs:4:26
3    |
4 LL | const X: bool = unsafe { &1 as *const i32 == &2 as *const i32 };
6    |
7    = note: see issue #53020 <https://github.com/rust-lang/rust/issues/53020> for more information
8 
- error: pointers cannot be reliably compared during const eval.
- error: pointers cannot be reliably compared during const eval.
+ error: pointers cannot be reliably compared during const eval
10   --> $DIR/const_raw_ptr_ops.rs:6:27
11    |
12 LL | const X2: bool = unsafe { 42 as *const i32 == 43 as *const i32 };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops/const_raw_ptr_ops.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const_raw_ptr_ops.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_raw_ptr_ops/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: pointers cannot be reliably compared during const eval
  --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:4:26
   |
LL | const X: bool = unsafe { &1 as *const i32 == &2 as *const i32 }; //~ ERROR cannot be reliably
   |
   = note: see issue #53020 <https://github.com/rust-lang/rust/issues/53020> for more information

error: pointers cannot be reliably compared during const eval
error: pointers cannot be reliably compared during const eval
  --> /checkout/src/test/ui/consts/const-eval/const_raw_ptr_ops.rs:6:27
   |
LL | const X2: bool = unsafe { 42 as *const i32 == 43 as *const i32 }; //~ ERROR cannot be reliably
   |
   = note: see issue #53020 <https://github.com/rust-lang/rust/issues/53020> for more information

error: aborting due to 2 previous errors
---
- error: pointers cannot be cast to integers during const eval.
+ error: pointers cannot be cast to integers during const eval
2   --> $DIR/match-test-ptr-null.rs:6:15
3    |
4 LL |         match &1 as *const i32 as usize {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/match-test-ptr-null.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/match-test-ptr-null.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/match-test-ptr-null/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6:15
   |
LL |         match &1 as *const i32 as usize {
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error: aborting due to previous error


------------------------------------------
---
- error: pointers cannot be cast to integers during const eval.
+ error: pointers cannot be cast to integers during const eval
29   --> $DIR/const-extern-fn-min-const-fn.rs:9:48
30    |
31 LL | const extern "C" fn ptr_cast(val: *const u8) { val as usize; }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn/const-extern-fn-min-const-fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-extern-fn/const-extern-fn-min-const-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: function pointers cannot appear in constant functions
  --> /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs:4:41
   |
LL | const unsafe extern "C" fn closure() -> fn() { || {} }
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_fn_ptr_basics)]` to the crate attributes to enable


error[E0658]: function pointer casts are not allowed in constant functions
  --> /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs:4:48
   |
LL | const unsafe extern "C" fn closure() -> fn() { || {} }
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_fn_ptr_basics)]` to the crate attributes to enable


error[E0658]: floating point arithmetic is not allowed in constant functions
  --> /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs:7:42
   |
LL | const unsafe extern "C" fn use_float() { 1.0 + 1.0; }
   |
   = note: see issue #57241 <https://github.com/rust-lang/rust/issues/57241> for more information
   = help: add `#![feature(const_fn_floating_point_arithmetic)]` to the crate attributes to enable


error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/const-extern-fn/const-extern-fn-min-const-fn.rs:9:48
   |
LL | const extern "C" fn ptr_cast(val: *const u8) { val as usize; }
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0658`.

---
- error: pointers cannot be cast to integers during const eval.
+ error: pointers cannot be cast to integers during const eval
2   --> $DIR/issue-17458.rs:1:28
3    |
4 LL | static X: usize = unsafe { core::ptr::null::<usize>() as usize };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-17458/issue-17458.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-17458.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-17458.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-17458" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-17458/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/issue-17458.rs:1:28
   |
LL | static X: usize = unsafe { core::ptr::null::<usize>() as usize };
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error: aborting due to previous error


------------------------------------------
---
- error: pointers cannot be reliably compared during const eval.
+ error: pointers cannot be reliably compared during const eval
2   --> $DIR/issue-25826.rs:3:30
3    |
4 LL |     const A: bool = unsafe { id::<u8> as *const () < id::<u16> as *const () };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-25826/issue-25826.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-25826.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-25826.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-25826" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-25826/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: pointers cannot be reliably compared during const eval
  --> /checkout/src/test/ui/consts/issue-25826.rs:3:30
   |
LL |     const A: bool = unsafe { id::<u8> as *const () < id::<u16> as *const () };
   |
   = note: see issue #53020 <https://github.com/rust-lang/rust/issues/53020> for more information

error: aborting due to previous error
---
- error: pointers cannot be cast to integers during const eval.
+ error: pointers cannot be cast to integers during const eval
2   --> $DIR/issue-52023-array-size-pointer-cast.rs:2:17
3    |
4 LL |     let _ = [0; (&0 as *const i32) as usize];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-52023-array-size-pointer-cast/issue-52023-array-size-pointer-cast.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-52023-array-size-pointer-cast.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-52023-array-size-pointer-cast.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-52023-array-size-pointer-cast" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-52023-array-size-pointer-cast/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/issue-52023-array-size-pointer-cast.rs:2:17
   |
LL |     let _ = [0; (&0 as *const i32) as usize]; //~ ERROR pointers cannot be cast to integers during const eval
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error: aborting due to previous error


------------------------------------------
---
- error: pointers cannot be reliably compared during const eval.
+ error: pointers cannot be reliably compared during const eval
20   --> $DIR/cmp_fn_pointers.rs:4:14
21    |
22 LL |     unsafe { x == y }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cmp_fn_pointers/cmp_fn_pointers.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/min_const_fn/cmp_fn_pointers.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/cmp_fn_pointers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cmp_fn_pointers" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/cmp_fn_pointers/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: function pointers cannot appear in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/cmp_fn_pointers.rs:1:14
   |
LL | const fn cmp(x: fn(), y: fn()) -> bool {
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_fn_ptr_basics)]` to the crate attributes to enable


error[E0658]: function pointers cannot appear in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/cmp_fn_pointers.rs:1:23
   |
LL | const fn cmp(x: fn(), y: fn()) -> bool {
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_fn_ptr_basics)]` to the crate attributes to enable


error: pointers cannot be reliably compared during const eval
  --> /checkout/src/test/ui/consts/min_const_fn/cmp_fn_pointers.rs:4:14
   |
LL |     unsafe { x == y }
   |
   = note: see issue #53020 <https://github.com/rust-lang/rust/issues/53020> for more information

error: aborting due to 3 previous errors
---
---- [ui] ui/consts/min_const_fn/min_const_fn.rs stdout ----
diff of stderr:

164    |
165    = help: consider extracting the value of the `static` to a `const`, and referring to that
- error: pointers cannot be cast to integers during const eval.
+ error: pointers cannot be cast to integers during const eval
168   --> $DIR/min_const_fn.rs:92:42
169    |
169    |
170 LL | const fn foo30(x: *const u32) -> usize { x as usize }
173    = note: at compile-time, pointers do not have an integer value
173    = note: at compile-time, pointers do not have an integer value
174    = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
- error: pointers cannot be cast to integers during const eval.
+ error: pointers cannot be cast to integers during const eval
177   --> $DIR/min_const_fn.rs:94:63
178    |
178    |
179 LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
182    = note: at compile-time, pointers do not have an integer value
182    = note: at compile-time, pointers do not have an integer value
183    = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
- error: pointers cannot be cast to integers during const eval.
+ error: pointers cannot be cast to integers during const eval
186   --> $DIR/min_const_fn.rs:96:42
187    |
187    |
188 LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
191    = note: at compile-time, pointers do not have an integer value
191    = note: at compile-time, pointers do not have an integer value
192    = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
- error: pointers cannot be cast to integers during const eval.
+ error: pointers cannot be cast to integers during const eval
195   --> $DIR/min_const_fn.rs:98:63
196    |
196    |
197 LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/min_const_fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/min_const_fn/min_const_fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/min_const_fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:37:25
   |
LL |     const fn into_inner(self) -> T { self.0 } //~ destructors cannot be evaluated
   |                         ^^^^                - value is dropped here
   |                         constant functions cannot evaluate destructors

error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:22
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:22
   |
LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:36
   |
LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:39:45
   |
LL |     const fn get_mut(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:46:28
   |
LL |     const fn into_inner_lt(self) -> T { self.0 } //~ destructors cannot be evaluated
   |                            ^^^^                - value is dropped here
   |                            constant functions cannot evaluate destructors

error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:48:25
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:48:25
   |
LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:48:42
   |
LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:48:51
   |
LL |     const fn get_mut_lt(&'a mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:55:27
   |
LL |     const fn into_inner_s(self) -> T { self.0 } //~ ERROR destructors
   |                           ^^^^                - value is dropped here
   |                           constant functions cannot evaluate destructors

error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:57:24
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:57:24
   |
LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:57:38
   |
LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:57:47
   |
LL |     const fn get_mut_s(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:64:25
   |
LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:64:39
   |
LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:64:48
   |
LL |     const fn get_mut_sq(&mut self) -> &mut T { &mut self.0 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:84:16
   |
LL | const fn foo11<T: std::fmt::Display>(t: T) -> T { t }
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable


error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:86:18
   |
LL | const fn foo11_2<T: Send>(t: T) -> T { t }
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable


error[E0013]: constant functions cannot refer to statics
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:90:27
   |
LL | const fn foo25() -> u32 { BAR } //~ ERROR cannot refer to statics
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error[E0013]: constant functions cannot refer to statics
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:91:37
   |
   |
LL | const fn foo26() -> &'static u32 { &BAR } //~ ERROR cannot refer to statics
   |
   |
   = help: consider extracting the value of the `static` to a `const`, and referring to that
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:92:42
   |
   |
LL | const fn foo30(x: *const u32) -> usize { x as usize }
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:94:63
   |
   |
LL | const fn foo30_with_unsafe(x: *const u32) -> usize { unsafe { x as usize } }
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:96:42
   |
   |
LL | const fn foo30_2(x: *mut u32) -> usize { x as usize }
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:98:63
   |
   |
LL | const fn foo30_2_with_unsafe(x: *mut u32) -> usize { unsafe { x as usize } }
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error[E0658]: mutable references are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:101:14
   |
   |
LL | const fn inc(x: &mut i32) { *x += 1 }
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable


error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:110:6
   |
LL | impl<T: std::fmt::Debug> Foo<T> {
   |      ^
LL | //~^ ERROR trait bounds other than `Sized` on const fn parameters are unstable
LL |     const fn foo(&self) {}
   |     ------------------- function declared as const here
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable

error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:115:6
   |
LL | impl<T: std::fmt::Debug + Sized> Foo<T> {
   |      ^
LL | //~^ ERROR trait bounds other than `Sized` on const fn parameters are unstable
LL |     const fn foo2(&self) {}
   |     -------------------- function declared as const here
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable

error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:120:6
   |
LL | impl<T: Sync + Sized> Foo<T> {
   |      ^
LL | //~^ ERROR trait bounds other than `Sized` on const fn parameters are unstable
LL |     const fn foo3(&self) {}
   |     -------------------- function declared as const here
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable

error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:126:34
   |
LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable


error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:126:19
   |
LL | const fn no_apit2(_x: AlanTuring<impl std::fmt::Debug>) {}
   |                   ^^                                     - value is dropped here
   |                   constant functions cannot evaluate destructors

error[E0658]: trait bounds other than `Sized` on const fn parameters are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:129:22
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:129:22
   |
LL | const fn no_apit(_x: impl std::fmt::Debug) {}
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable


error[E0493]: destructors cannot be evaluated at compile-time
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:129:18
   |
LL | const fn no_apit(_x: impl std::fmt::Debug) {}
   |                  ^^                         - value is dropped here
   |                  constant functions cannot evaluate destructors

error[E0658]: trait objects in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:132:23
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:132:23
   |
LL | const fn no_dyn_trait(_x: &dyn std::fmt::Debug) {}
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable


error[E0658]: trait objects in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:134:32
   |
LL | const fn no_dyn_trait_ret() -> &'static dyn std::fmt::Debug { &() }
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable


error[E0658]: trait objects in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:139:41
   |
LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
   | -------------------------------------   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   | function declared as const here
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable

error[E0658]: trait objects in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:139:42
   |
LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
   | |
   | function declared as const here
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable

error[E0658]: trait objects in const fn are unstable
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:139:42
   |
LL | const fn really_no_traits_i_mean_it() { (&() as &dyn std::fmt::Debug, ()).1 }
   | |
   | function declared as const here
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_trait_bound)]` to the crate attributes to enable

error[E0658]: function pointers cannot appear in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:144:21
   |
LL | const fn no_fn_ptrs(_x: fn()) {}
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_fn_ptr_basics)]` to the crate attributes to enable


error[E0658]: function pointers cannot appear in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:146:27
   |
LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_fn_ptr_basics)]` to the crate attributes to enable


error[E0658]: function pointer casts are not allowed in constant functions
  --> /checkout/src/test/ui/consts/min_const_fn/min_const_fn.rs:146:46
   |
LL | const fn no_fn_ptrs2() -> fn() { fn foo() {} foo }
   |
   = note: see issue #57563 <https://github.com/rust-lang/rust/issues/57563> for more information
   = help: add `#![feature(const_fn_fn_ptr_basics)]` to the crate attributes to enable

---
- error: pointers cannot be reliably compared during const eval.
+ error: pointers cannot be reliably compared during const eval
2   --> $DIR/E0395.rs:4:29
3    |
4 LL | static BAZ: bool = unsafe { (&FOO as *const i32) == (&BAR as *const i32) };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0395/E0395.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0395.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0395.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0395" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0395/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: pointers cannot be reliably compared during const eval
  --> /checkout/src/test/ui/error-codes/E0395.rs:4:29
   |
LL | static BAZ: bool = unsafe { (&FOO as *const i32) == (&BAR as *const i32) };
   |
   = note: see issue #53020 <https://github.com/rust-lang/rust/issues/53020> for more information

error: aborting due to previous error
---
4 LL |     fn f(u8) {}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/future-incompatible-lint-group/future-incompatible-lint-group.stderr
To only update this specific test, also pass `--test-args future-incompatible-lint-group.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/future-incompatible-lint-group.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/future-incompatible-lint-group" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/future-incompatible-lint-group/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: anonymous parameters are deprecated and will be removed in the next edition
  --> /checkout/src/test/ui/future-incompatible-lint-group.rs:7:10
   |
LL |     fn f(u8) {} //~ WARN anonymous parameters are deprecated
   |          ^^ help: try naming the parameter or explicitly ignoring it: `_: u8`
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>


error: this attribute can only be applied at the crate level
  --> /checkout/src/test/ui/future-incompatible-lint-group.rs:13:12
   |
LL |     #![doc(test(some_test))]
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/future-incompatible-lint-group.rs:3:9
   |
   |
LL | #![deny(future_incompatible)]
   |         ^^^^^^^^^^^^^^^^^^^
   = note: `#[deny(invalid_doc_attributes)]` implied by `#[deny(future_incompatible)]`
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: read https://doc.rust-lang.org/nightly/rustdoc/the-doc-attribute.html#at-the-crate-level for more information
error: aborting due to previous error; 1 warning emitted


------------------------------------------
---
- error: items in traits are not importable.
+ error: items in traits are not importable
2   --> $DIR/issue-30560.rs:7:5
3    |
4 LL | use T::*;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-30560/issue-30560.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args imports/issue-30560.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/imports/issue-30560.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-30560" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/imports/issue-30560/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: items in traits are not importable
  --> /checkout/src/test/ui/imports/issue-30560.rs:7:5
   |
LL | use T::*; //~ ERROR items in traits are not importable

error[E0432]: unresolved import `Alias`
  --> /checkout/src/test/ui/imports/issue-30560.rs:2:5
   |
   |
LL | use Alias::*; //~ ERROR unresolved import `Alias` [E0432]
   |     ^^^^^ `Alias` is a type alias, not a module
error[E0432]: unresolved import `std::io::Result`
  --> /checkout/src/test/ui/imports/issue-30560.rs:4:14
   |
   |
LL | use std::io::Result::*; //~ ERROR unresolved import `std::io::Result` [E0432]
   |              ^^^^^^ `Result` is a type alias, not a module
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0432`.

---
- error: pointers cannot be cast to integers during const eval.
+ error: pointers cannot be cast to integers during const eval
2   --> $DIR/issue-18294.rs:3:31
3    |
4 LL |     const Y: usize = unsafe { &X as *const u32 as usize };

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18294/issue-18294.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-18294.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18294.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18294" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18294/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: pointers cannot be cast to integers during const eval
  --> /checkout/src/test/ui/issues/issue-18294.rs:3:31
   |
LL |     const Y: usize = unsafe { &X as *const u32 as usize }; //~ ERROR pointers cannot be cast to integers
   |
   = note: at compile-time, pointers do not have an integer value
   = note: at compile-time, pointers do not have an integer value
   = note: avoiding this restriction via `transmute`, `union`, or raw pointers leads to compile-time undefined behavior
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/issues/issue-50403.rs stdout ----
diff of stderr:

- error: concat_idents! takes 1 or more arguments.
+ error: concat_idents! takes 1 or more arguments
3    |
3    |
4 LL |     let x = concat_idents!();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50403/issue-50403.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-50403.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50403.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50403" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50403/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: concat_idents! takes 1 or more arguments
   |
   |
LL |     let x = concat_idents!(); //~ ERROR concat_idents! takes 1 or more arguments

error: aborting due to previous error



------------------------------------------


---- [ui] ui/iterators/into-iter-on-arrays-2018.rs stdout ----
diff of stderr:

- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
3    |
3    |
4 LL |     let _: Iter<'_, i32> = array.into_iter();

16 LL |     let _: Iter<'_, i32> = IntoIterator::into_iter(array);
17    |                            ++++++++++++++++++++++++     ~
18 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
21    |
21    |
22 LL |     let _: Iter<'_, i32> = Box::new(array).into_iter();
25    = warning: this changes meaning in Rust 2021
26    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
27 
27 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
30    |
30    |
31 LL |     let _: Iter<'_, i32> = Rc::new(array).into_iter();
34    = warning: this changes meaning in Rust 2021
35    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
36 
36 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
39    |
39    |
40 LL |     let _: Iter<'_, i32> = Array(array).into_iter();
43    = warning: this changes meaning in Rust 2021
44    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
45 
45 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
48    |
48    |
49 LL |     for _ in [1, 2, 3].into_iter() {}

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-2018/into-iter-on-arrays-2018.stderr
To only update this specific test, also pass `--test-args iterators/into-iter-on-arrays-2018.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/into-iter-on-arrays-2018.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-2018" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-2018/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     let _: Iter<'_, i32> = array.into_iter();
   |
   = note: `#[warn(array_into_iter)]` on by default
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     let _: Iter<'_, i32> = array.iter();
   |                                  ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
   |
LL |     let _: Iter<'_, i32> = IntoIterator::into_iter(array);
   |                            ++++++++++++++++++++++++     ~

warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     let _: Iter<'_, i32> = Box::new(array).into_iter();
   |                                            ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     let _: Iter<'_, i32> = Rc::new(array).into_iter();
   |                                           ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     let _: Iter<'_, i32> = Array(array).into_iter();
   |                                         ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     for _ in [1, 2, 3].into_iter() {}
   |
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     for _ in [1, 2, 3].iter() {}
   |                        ~~~~
help: or remove `.into_iter()` to iterate by value
   |
LL -     for _ in [1, 2, 3].into_iter() {}
LL +     for _ in [1, 2, 3] {}

warning: 5 warnings emitted



------------------------------------------


---- [ui] ui/iterators/into-iter-on-arrays-lint.rs stdout ----
diff of stderr:

- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
3    |
4 LL |     small.into_iter();

16 LL |     IntoIterator::into_iter(small);
16 LL |     IntoIterator::into_iter(small);
17    |     ++++++++++++++++++++++++     ~
18 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
21    |
21    |
22 LL |     [1, 2].into_iter();
33 LL |     IntoIterator::into_iter([1, 2]);
34    |     ++++++++++++++++++++++++      ~
35 
35 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
38    |
39 LL |     big.into_iter();

50 LL |     IntoIterator::into_iter(big);
50 LL |     IntoIterator::into_iter(big);
51    |     ++++++++++++++++++++++++   ~
52 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
55    |
55    |
56 LL |     [0u8; 33].into_iter();

67 LL |     IntoIterator::into_iter([0u8; 33]);
68    |     ++++++++++++++++++++++++         ~
69 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
72    |
72    |
73 LL |     Box::new(small).into_iter();
76    = warning: this changes meaning in Rust 2021
77    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
78 
78 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
81    |
81    |
82 LL |     Box::new([1, 2]).into_iter();
85    = warning: this changes meaning in Rust 2021
86    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
87 
87 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
90    |
90    |
91 LL |     Box::new(big).into_iter();
94    = warning: this changes meaning in Rust 2021
95    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
96 
96 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
99    |
99    |
100 LL |     Box::new([0u8; 33]).into_iter();
103    = warning: this changes meaning in Rust 2021
104    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
105 
105 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
108    |
108    |
109 LL |     Box::new(Box::new(small)).into_iter();
112    = warning: this changes meaning in Rust 2021
113    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
114 
114 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
117    |
117    |
118 LL |     Box::new(Box::new([1, 2])).into_iter();
121    = warning: this changes meaning in Rust 2021
122    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
123 
123 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
126    |
126    |
127 LL |     Box::new(Box::new(big)).into_iter();
130    = warning: this changes meaning in Rust 2021
131    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
132 
132 
- warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021.
+ warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
135    |
135    |
136 LL |     Box::new(Box::new([0u8; 33])).into_iter();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/into-iter-on-arrays-lint.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args iterators/into-iter-on-arrays-lint.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
LL |     small.into_iter();
   |           ^^^^^^^^^
   |
   |
   = note: `#[warn(array_into_iter)]` on by default
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
LL |     small.iter();
   |           ~~~~
   |           ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
LL |     IntoIterator::into_iter(small);
   |     ++++++++++++++++++++++++     ~


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     [1, 2].into_iter();
   |
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     [1, 2].iter();
   |            ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
LL |     IntoIterator::into_iter([1, 2]);
   |     ++++++++++++++++++++++++      ~


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
LL |     big.into_iter();
   |         ^^^^^^^^^
   |
   |
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     big.iter();
   |         ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
LL |     IntoIterator::into_iter(big);
   |     ++++++++++++++++++++++++   ~


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     [0u8; 33].into_iter();
   |
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     [0u8; 33].iter();
   |               ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
   |
LL |     IntoIterator::into_iter([0u8; 33]);
   |     ++++++++++++++++++++++++         ~

warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(small).into_iter();
   |                     ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new([1, 2]).into_iter();
   |                      ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(big).into_iter();
   |                   ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new([0u8; 33]).into_iter();
   |                         ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new(small)).into_iter();
   |                               ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new([1, 2])).into_iter();
   |                                ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new(big)).into_iter();
   |                             ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new([0u8; 33])).into_iter();
   |                                   ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>

warning: 12 warnings emitted
---
To only update this specific test, also pass `--test-args lang-items/issue-83471.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-items/issue-83471.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-83471" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/issue-83471/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---

error[E0658]: language items are subject to change
  --> /checkout/src/test/ui/lang-items/issue-83471.rs:7:1
   |
LL | #[lang = "sized"]
   |
   = help: add `#![feature(lang_items)]` to the crate attributes to enable

error[E0658]: language items are subject to change
error[E0658]: language items are subject to change
  --> /checkout/src/test/ui/lang-items/issue-83471.rs:11:1
   |
LL | #[lang = "fn"]
   |
   = help: add `#![feature(lang_items)]` to the crate attributes to enable

warning: anonymous parameters are deprecated and will be removed in the next edition
---
   = note: `#[warn(anonymous_parameters)]` on by default
   = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2018!
   = note: for more information, see issue #41686 <https://github.com/rust-lang/rust/issues/41686>

error[E0718]: `fn` language item must be applied to a trait with 1 generic argument
   |
   |
LL | #[lang = "fn"]
...
LL | trait Fn {
LL | trait Fn {
   |         - this trait has 0 generic arguments
error: aborting due to 5 previous errors; 1 warning emitted

Some errors have detailed explanations: E0425, E0573, E0658, E0718.
For more information about an error, try `rustc --explain E0425`.
---
---- [ui] ui/lint/must_not_suspend/mutex.rs stdout ----
diff of stderr:

11    |
12 LL | #![deny(must_not_suspend)]
13    |         ^^^^^^^^^^^^^^^^
- note: Holding a MutexGuard across suspend points can cause deadlocks, delays, and cause Futures to not implement `Send`
+ note: holding a MutexGuard across suspend points can cause deadlocks, delays, and cause Futures to not implement `Send`
16    |
16    |
17 LL |     let _guard = m.lock().unwrap();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/mutex/mutex.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/must_not_suspend/mutex.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_not_suspend/mutex.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/mutex" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/mutex/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `MutexGuard` held across a suspend point, but should not be
   |
   |
LL |     let _guard = m.lock().unwrap(); //~ ERROR `MutexGuard` held across
   |         ^^^^^^
LL |     other().await;
   |     ------------- the value is held across this suspend point
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/must_not_suspend/mutex.rs:2:9
   |
   |
LL | #![deny(must_not_suspend)]
   |         ^^^^^^^^^^^^^^^^
note: holding a MutexGuard across suspend points can cause deadlocks, delays, and cause Futures to not implement `Send`
   |
   |
LL |     let _guard = m.lock().unwrap(); //~ ERROR `MutexGuard` held across
   |         ^^^^^^
help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
   |
   |
LL |     let _guard = m.lock().unwrap(); //~ ERROR `MutexGuard` held across

error: aborting due to previous error



------------------------------------------


---- [ui] ui/lint/rfc-2457-non-ascii-idents/lint-mixed-script-confusables.rs stdout ----
diff of stderr:

- error: The usage of Script Group `Greek` in this crate consists solely of mixed script confusables
+ error: the usage of Script Group `Greek` in this crate consists solely of mixed script confusables
3    |
3    |
4 LL | struct ΑctuallyNotLatin;
9    |
9    |
10 LL | #![deny(mixed_script_confusables)]
11    |         ^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: The usage includes 'Α' (U+0391).
-    = note: Please recheck to make sure their usages are indeed what you want.
+    = note: the usage includes 'Α' (U+0391)
+    = note: please recheck to make sure their usages are indeed what you want
14 
- error: The usage of Script Group `Cyrillic` in this crate consists solely of mixed script confusables
+ error: the usage of Script Group `Cyrillic` in this crate consists solely of mixed script confusables
17    |
18 LL | mod роре {

19    |     ^^^^
19    |     ^^^^
20    |
-    = note: The usage includes 'е' (U+0435), 'о' (U+043E), 'р' (U+0440).
-    = note: Please recheck to make sure their usages are indeed what you want.
+    = note: the usage includes 'е' (U+0435), 'о' (U+043E), 'р' (U+0440)
+    = note: please recheck to make sure their usages are indeed what you want
23 
- error: The usage of Script Group `Japanese, Katakana` in this crate consists solely of mixed script confusables
+ error: the usage of Script Group `Japanese, Katakana` in this crate consists solely of mixed script confusables
26    |
26    |
27 LL |     const エ: &'static str = "アイウ";
28    |           ^^
29    |
29    |
-    = note: The usage includes 'エ' (U+30A8).
-    = note: Please recheck to make sure their usages are indeed what you want.
+    = note: the usage includes 'エ' (U+30A8)
+    = note: please recheck to make sure their usages are indeed what you want
33 error: aborting due to 3 previous errors
34 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2457-non-ascii-idents/lint-mixed-script-confusables/lint-mixed-script-confusables.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/rfc-2457-non-ascii-idents/lint-mixed-script-confusables.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-mixed-script-confusables.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2457-non-ascii-idents/lint-mixed-script-confusables" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/rfc-2457-non-ascii-idents/lint-mixed-script-confusables/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the usage of Script Group `Greek` in this crate consists solely of mixed script confusables
   |
   |
LL | struct ΑctuallyNotLatin;
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/rfc-2457-non-ascii-idents/lint-mixed-script-confusables.rs:1:9
   |
   |
LL | #![deny(mixed_script_confusables)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^
   = note: the usage includes 'Α' (U+0391)
   = note: please recheck to make sure their usages are indeed what you want

error: the usage of Script Group `Cyrillic` in this crate consists solely of mixed script confusables
   |
LL | mod роре {
   |     ^^^^
   |
   |
   = note: the usage includes 'е' (U+0435), 'о' (U+043E), 'р' (U+0440)
   = note: please recheck to make sure their usages are indeed what you want

error: the usage of Script Group `Japanese, Katakana` in this crate consists solely of mixed script confusables
   |
   |
LL |     const エ: &'static str = "アイウ";
   |
   |
   = note: the usage includes 'エ' (U+30A8)
   = note: please recheck to make sure their usages are indeed what you want
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/macros/macros-nonfatal-errors.rs stdout ----
diff of stderr:

138 LL |     llvm_asm!(invalid);
140 
140 
- error: concat_idents! requires ident args.
+ error: concat_idents! requires ident args
143    |
143    |
144 LL |     concat_idents!("not", "idents");

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/macros-nonfatal-errors.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args macros/macros-nonfatal-errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macros-nonfatal-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     #[default] //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL | struct DefaultInnerAttrTupleStruct(#[default] ());


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL | #[default] //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL | #[default] //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     Foo = #[default] 0, //~ ERROR the `#[default]` attribute may only be used on unit enum variants


error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     Bar([u8; #[default] 1]), //~ ERROR the `#[default]` attribute may only be used on unit enum variants

error: no default declared
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:43:10
   |
   |
LL | #[derive(Default)] //~ ERROR no default declared
   |
   |
   = help: make a unit variant default by placing `#[default]` above it
   = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)
error: multiple declared defaults
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:49:10
   |
   |
LL | #[derive(Default)] //~ ERROR multiple declared defaults
...
LL |     Foo,
   |     --- first default
LL |     #[default]
LL |     #[default]
LL |     Bar,
   |     --- additional default
LL |     #[default]
LL |     Baz,
   |     --- additional default
   |
   = note: only one variant can be default
   = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)

error: `#[default]` attribute does not accept a value
   |
   |
LL |     #[default = 1] //~ ERROR `#[default]` attribute does not accept a value
   |
   |
   = help: try using `#[default]`

error: multiple `#[default]` attributes
   |
LL |     #[default]
LL |     #[default]
   |     ---------- `#[default]` used here
LL |     #[default]
   |     ---------- `#[default]` used again here
LL |     Foo, //~ERROR multiple `#[default]` attributes
   |
   |
   = note: only one `#[default]` attribute is needed
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:68:5
   |
LL |     #[default]
   |     ^^^^^^^^^^
   |     ^^^^^^^^^^

error: multiple `#[default]` attributes
   |
LL |     #[default]
LL |     #[default]
   |     ---------- `#[default]` used here
LL |     #[default]
   |     ---------- `#[default]` used again here
...
LL |     Foo, //~ERROR multiple `#[default]` attributes
   |
   |
   = note: only one `#[default]` attribute is needed
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:76:5
   |
LL |     #[default]
   |     ^^^^^^^^^^
   |     ^^^^^^^^^^
LL |     #[default]
   |     ^^^^^^^^^^
LL |     #[default]
   |     ^^^^^^^^^^

error: the `#[default]` attribute may only be used on unit enum variants
   |
   |
LL |     Foo {}, //~ ERROR the `#[default]` attribute may only be used on unit enum variants
   |
   |
   = help: consider a manual implementation of `Default`

error: default variant must be exhaustive
   |
   |
LL |     #[non_exhaustive]
   |     ----------------- declared `#[non_exhaustive]` here
LL |     Foo, //~ ERROR default variant must be exhaustive
   |
   |
   = help: consider a manual implementation of `Default`
error: asm template must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:99:10
   |
   |
LL |     asm!(invalid); //~ ERROR

error: inline assembly must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:100:15
   |
   |
LL |     llvm_asm!(invalid); //~ ERROR


error: concat_idents! requires ident args
   |
   |
LL |     concat_idents!("not", "idents"); //~ ERROR

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:104:17
   |
   |
LL |     option_env!(invalid); //~ ERROR

error: expected string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:105:10
   |
   |
LL |     env!(invalid); //~ ERROR

error: expected string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:106:10
   |
   |
LL |     env!(foo, abr, baz); //~ ERROR


error: environment variable `RUST_HOPEFULLY_THIS_DOESNT_EXIST` not defined
   |
   |
LL |     env!("RUST_HOPEFULLY_THIS_DOESNT_EXIST"); //~ ERROR
   |
   = note: this error originates in the macro `env` (in Nightly builds, run with -Z macro-backtrace for more info)

error: format argument must be a string literal
error: format argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:109:13
   |
LL |     format!(invalid); //~ ERROR
   |
help: you might be missing a string literal to format with
   |
   |
LL |     format!("{}", invalid); //~ ERROR

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:111:14
   |
   |
LL |     include!(invalid); //~ ERROR

error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:113:18
   |
   |
LL |     include_str!(invalid); //~ ERROR


error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
   |
   |
LL |     include_str!("i'd be quite surprised if a file with this name existed"); //~ ERROR
   |
   |
   = note: this error originates in the macro `include_str` (in Nightly builds, run with -Z macro-backtrace for more info)
error: argument must be a string literal
  --> /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:115:20
   |
   |
LL |     include_bytes!(invalid); //~ ERROR


error: couldn't read /checkout/src/test/ui/macros/i'd be quite surprised if a file with this name existed: No such file or directory (os error 2)
   |
   |
LL |     include_bytes!("i'd be quite surprised if a file with this name existed"); //~ ERROR
   |
   = note: this error originates in the macro `include_bytes` (in Nightly builds, run with -Z macro-backtrace for more info)


error: trace_macros! accepts only `true` or `false`
   |
   |
LL |     trace_macros!(invalid); //~ ERROR

error: aborting due to 27 previous errors



------------------------------------------


---- [ui] ui/missing/missing-alloc_error_handler.rs stdout ----
diff of stderr:

- error: `#[alloc_error_handler]` function required, but not found.
+ error: `#[alloc_error_handler]` function required, but not found
2 
- note: Use `#![feature(default_alloc_error_handler)]` for a default error handler.
+ note: Use `#![feature(default_alloc_error_handler)]` for a default error handler
5 error: aborting due to previous error
6 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-alloc_error_handler/missing-alloc_error_handler.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args missing/missing-alloc_error_handler.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing/missing-alloc_error_handler.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-alloc_error_handler" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-alloc_error_handler/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `#[alloc_error_handler]` function required, but not found

note: Use `#![feature(default_alloc_error_handler)]` for a default error handler
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/missing/missing-allocator.rs stdout ----
diff of stderr:

- error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait.
+ error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-allocator/missing-allocator.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args missing/missing-allocator.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/missing/missing-allocator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-allocator" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "panic=abort" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/missing/missing-allocator/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: no global memory allocator found but one is required; link to std or add `#[global_allocator]` to a static item that implements the GlobalAlloc trait
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/never_type/defaulted-never-note.rs#fallback stdout ----
diff of stderr:

4 LL |     foo(_x);
5    |     ^^^ the trait `ImplementedForUnitButNotNever` is not implemented for `!`
-    = note: this trait is implemented for `()`.
+    = note: this trait is implemented for `()`
+    = note: this trait is implemented for `()`
8    = note: this error might have been caused by changes to Rust's type-inference algorithm (see issue #48950 <https://github.com/rust-lang/rust/issues/48950> for more information).
9    = help: did you intend to use the type `()` here instead?
10 note: required by a bound in `foo`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/never_type/defaulted-never-note.fallback/defaulted-never-note.fallback.stderr
To only update this specific test, also pass `--test-args never_type/defaulted-never-note.rs`


error in revision `fallback`: 1 errors occurred comparing output.
---

6 LL |     type 一;
7    |          ^^
8    |
-    = note: This limitation may be lifted in the future; see issue #83942 <https://github.com/rust-lang/rust/issues/83942> for more information
+    = note: this limitation may be lifted in the future; see issue #83942 <https://github.com/rust-lang/rust/issues/83942> for more information
10 
11 error: items in `extern` blocks cannot use non-ascii identifiers

17 LL |     fn 二();
18    |        ^^
19    |
19    |
-    = note: This limitation may be lifted in the future; see issue #83942 <https://github.com/rust-lang/rust/issues/83942> for more information
+    = note: this limitation may be lifted in the future; see issue #83942 <https://github.com/rust-lang/rust/issues/83942> for more information
21 
22 error: items in `extern` blocks cannot use non-ascii identifiers

28 LL |     static 三: usize;
29    |            ^^
30    |
30    |
-    = note: This limitation may be lifted in the future; see issue #83942 <https://github.com/rust-lang/rust/issues/83942> for more information
+    = note: this limitation may be lifted in the future; see issue #83942 <https://github.com/rust-lang/rust/issues/83942> for more information
33 error: aborting due to 3 previous errors
34 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2457/extern_block_nonascii_forbidden/extern_block_nonascii_forbidden.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2457/extern_block_nonascii_forbidden.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2457/extern_block_nonascii_forbidden.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2457/extern_block_nonascii_forbidden" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2457/extern_block_nonascii_forbidden/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: items in `extern` blocks cannot use non-ascii identifiers
   |
LL | extern "C" {
LL | extern "C" {
   | ---------- in this `extern` block
LL |     type 一; //~ items in `extern` blocks cannot use non-ascii identifiers
   |
   |
   = note: this limitation may be lifted in the future; see issue #83942 <https://github.com/rust-lang/rust/issues/83942> for more information

error: items in `extern` blocks cannot use non-ascii identifiers
   |
LL | extern "C" {
LL | extern "C" {
   | ---------- in this `extern` block
LL |     type 一; //~ items in `extern` blocks cannot use non-ascii identifiers
LL |     fn 二(); //~ items in `extern` blocks cannot use non-ascii identifiers
   |
   |
   = note: this limitation may be lifted in the future; see issue #83942 <https://github.com/rust-lang/rust/issues/83942> for more information

error: items in `extern` blocks cannot use non-ascii identifiers
   |
LL | extern "C" {
LL | extern "C" {
   | ---------- in this `extern` block
...
LL |     static 三: usize; //~ items in `extern` blocks cannot use non-ascii identifiers
   |
   |
   = note: this limitation may be lifted in the future; see issue #83942 <https://github.com/rust-lang/rust/issues/83942> for more information
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/sanitize/crt-static.rs stdout ----
diff of stderr:

- error: Sanitizer is incompatible with statically linked libc, disable it using `-C target-feature=-crt-static`
+ error: sanitizer is incompatible with statically linked libc, disable it using `-C target-feature=-crt-static`
3 error: aborting due to previous error
4 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/crt-static/crt-static.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args sanitize/crt-static.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/sanitize/crt-static.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/crt-static" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "sanitizer=address" "-C" "target-feature=+crt-static" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/sanitize/crt-static/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: sanitizer is incompatible with statically linked libc, disable it using `-C target-feature=-crt-static`
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/test-attrs/test-should-panic-attr.rs stdout ----
diff of stderr:

4 LL | #[should_panic(expected)]
6    |
-    = note: errors in this attribute were erroneously allowed and will become a hard error in a future release.
+    = note: errors in this attribute were erroneously allowed and will become a hard error in a future release
8 
8 
9 warning: argument must be of the form: `expected = "error message"`


12 LL | #[should_panic(expect)]
14    |
-    = note: errors in this attribute were erroneously allowed and will become a hard error in a future release.
+    = note: errors in this attribute were erroneously allowed and will become a hard error in a future release
16 
16 
17 warning: argument must be of the form: `expected = "error message"`


20 LL | #[should_panic(expected(foo, bar))]
22    |
-    = note: errors in this attribute were erroneously allowed and will become a hard error in a future release.
+    = note: errors in this attribute were erroneously allowed and will become a hard error in a future release
24 
24 
25 warning: argument must be of the form: `expected = "error message"`


28 LL | #[should_panic(expected = "foo", bar)]
30    |
-    = note: errors in this attribute were erroneously allowed and will become a hard error in a future release.
+    = note: errors in this attribute were erroneously allowed and will become a hard error in a future release
32 
---
To only update this specific test, also pass `--test-args test-attrs/test-should-panic-attr.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-should-panic-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-panic-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-panic-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: argument must be of the form: `expected = "error message"`
   |
   |
LL | #[should_panic(expected)]
   |
   = note: errors in this attribute were erroneously allowed and will become a hard error in a future release


warning: argument must be of the form: `expected = "error message"`
   |
   |
LL | #[should_panic(expect)]
   |
   = note: errors in this attribute were erroneously allowed and will become a hard error in a future release


warning: argument must be of the form: `expected = "error message"`
   |
   |
LL | #[should_panic(expected(foo, bar))]
   |
   = note: errors in this attribute were erroneously allowed and will become a hard error in a future release


warning: argument must be of the form: `expected = "error message"`
   |
   |
LL | #[should_panic(expected = "foo", bar)]
   |
   = note: errors in this attribute were erroneously allowed and will become a hard error in a future release

warning: 4 warnings emitted
---
---- [ui] ui/traits/vtable/vtable-diamond.rs stdout ----
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
diff of stderr:

- error: Vtable entries for `<S as D>`: [
+ error: vtable entries for `<S as D>`: [
2     MetadataDropInPlace,
4     MetadataAlign,

16 LL | | }
17    | |_^
17    | |_^
18 
- error: Vtable entries for `<S as C>`: [
+ error: vtable entries for `<S as C>`: [
20     MetadataDropInPlace,
22     MetadataAlign,


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-diamond/vtable-diamond.stderr
To only update this specific test, also pass `--test-args traits/vtable/vtable-diamond.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/vtable/vtable-diamond.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-diamond" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-diamond/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: vtable entries for `<S as D>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as A>::foo_a),
    Method(<S as B>::foo_b),
    Method(<S as C>::foo_c),
    TraitVPtr(<S as C>),
    Method(<S as D>::foo_d),
  --> /checkout/src/test/ui/traits/vtable/vtable-diamond.rs:21:1
   |
   |
LL | / trait D: B + C {
LL | |     //~^ error Vtable
LL | |     fn foo_d(&self) {}
LL | | }


error: vtable entries for `<S as C>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as A>::foo_a),
    Method(<S as C>::foo_c),
  --> /checkout/src/test/ui/traits/vtable/vtable-diamond.rs:15:1
   |
   |
LL | / trait C: A {
LL | |     //~^ error Vtable
LL | |     fn foo_c(&self) {}
LL | | }

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/traits/vtable/vtable-multiple.rs stdout ----
diff of stderr:

- error: Vtable entries for `<S as C>`: [
+ error: vtable entries for `<S as C>`: [
2     MetadataDropInPlace,
4     MetadataAlign,

15 LL | | }
16    | |_^
16    | |_^
17 
- error: Vtable entries for `<S as B>`: [
+ error: vtable entries for `<S as B>`: [
19     MetadataDropInPlace,
21     MetadataAlign,


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-multiple/vtable-multiple.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/vtable/vtable-multiple.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/vtable/vtable-multiple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-multiple" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-multiple/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: vtable entries for `<S as C>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as A>::foo_a),
    Method(<S as B>::foo_b),
    TraitVPtr(<S as B>),
    Method(<S as C>::foo_c),
  --> /checkout/src/test/ui/traits/vtable/vtable-multiple.rs:16:1
   |
   |
LL | / trait C: A + B {
LL | |     //~^ error Vtable
LL | |     fn foo_c(&self) {}
LL | | }


error: vtable entries for `<S as B>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as B>::foo_b),
  --> /checkout/src/test/ui/traits/vtable/vtable-multiple.rs:10:1
   |
   |
LL | / trait B {
LL | |     //~^ error Vtable
LL | |     fn foo_b(&self) {}
LL | | }

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/traits/vtable/vtable-vacant.rs stdout ----
diff of stderr:

- error: Vtable entries for `<S as B>`: [
+ error: vtable entries for `<S as B>`: [
2     MetadataDropInPlace,
4     MetadataAlign,


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-vacant/vtable-vacant.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/vtable/vtable-vacant.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/vtable/vtable-vacant.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-vacant" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-vacant/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: vtable entries for `<S as B>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as A>::foo_a1),
    Vacant,
    Method(<S as B>::foo_b1),
    Vacant,
  --> /checkout/src/test/ui/traits/vtable/vtable-vacant.rs:15:1
   |
   |
LL | / trait B: A {
LL | |     //~^ error Vtable
LL | |     fn foo_b1(&self) {}
LL | |     fn foo_b2(&self) where Self: Send {}
LL | | }

error: aborting due to previous error



------------------------------------------


---- [ui] ui/traits/vtable/vtable-multi-level.rs stdout ----
diff of stderr:

- error: Vtable entries for `<S as O>`: [
+ error: vtable entries for `<S as O>`: [
2     MetadataDropInPlace,
4     MetadataAlign,

37 LL | | }
38    | |_^
38    | |_^
39 
- error: Vtable entries for `<S as B>`: [
+ error: vtable entries for `<S as B>`: [
41     MetadataDropInPlace,
43     MetadataAlign,

51 LL | | }
52    | |_^
52    | |_^
53 
- error: Vtable entries for `<S as D>`: [
+ error: vtable entries for `<S as D>`: [
55     MetadataDropInPlace,
57     MetadataAlign,

65 LL | | }
66    | |_^
66    | |_^
67 
- error: Vtable entries for `<S as E>`: [
+ error: vtable entries for `<S as E>`: [
69     MetadataDropInPlace,
71     MetadataAlign,

79 LL | | }
80    | |_^
80    | |_^
81 
- error: Vtable entries for `<S as F>`: [
+ error: vtable entries for `<S as F>`: [
83     MetadataDropInPlace,
85     MetadataAlign,

96 LL | | }
97    | |_^
97    | |_^
98 
- error: Vtable entries for `<S as H>`: [
+ error: vtable entries for `<S as H>`: [
100     MetadataDropInPlace,
102     MetadataAlign,

110 LL | | }
111    | |_^
111    | |_^
112 
- error: Vtable entries for `<S as I>`: [
+ error: vtable entries for `<S as I>`: [
114     MetadataDropInPlace,
116     MetadataAlign,

124 LL | | }
125    | |_^
125    | |_^
126 
- error: Vtable entries for `<S as J>`: [
+ error: vtable entries for `<S as J>`: [
128     MetadataDropInPlace,
130     MetadataAlign,

141 LL | | }
142    | |_^
142    | |_^
143 
- error: Vtable entries for `<S as K>`: [
+ error: vtable entries for `<S as K>`: [
145     MetadataDropInPlace,
147     MetadataAlign,

155 LL | | }
156    | |_^
156    | |_^
157 
- error: Vtable entries for `<S as L>`: [
+ error: vtable entries for `<S as L>`: [
159     MetadataDropInPlace,
161     MetadataAlign,

169 LL | | }
170    | |_^
170    | |_^
171 
- error: Vtable entries for `<S as M>`: [
+ error: vtable entries for `<S as M>`: [
173     MetadataDropInPlace,
175     MetadataAlign,

186 LL | | }
187    | |_^
187    | |_^
188 
- error: Vtable entries for `<S as N>`: [
+ error: vtable entries for `<S as N>`: [
190     MetadataDropInPlace,
192     MetadataAlign,


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-multi-level/vtable-multi-level.stderr
To only update this specific test, also pass `--test-args traits/vtable/vtable-multi-level.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/vtable/vtable-multi-level.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-multi-level" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/vtable/vtable-multi-level/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: vtable entries for `<S as O>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as A>::foo_a),
    Method(<S as B>::foo_b),
    TraitVPtr(<S as B>),
    Method(<S as C>::foo_c),
    Method(<S as D>::foo_d),
    TraitVPtr(<S as D>),
    Method(<S as E>::foo_e),
    TraitVPtr(<S as E>),
    Method(<S as F>::foo_f),
    TraitVPtr(<S as F>),
    Method(<S as G>::foo_g),
    Method(<S as H>::foo_h),
    TraitVPtr(<S as H>),
    Method(<S as I>::foo_i),
    TraitVPtr(<S as I>),
    Method(<S as J>::foo_j),
    TraitVPtr(<S as J>),
    Method(<S as K>::foo_k),
    TraitVPtr(<S as K>),
    Method(<S as L>::foo_l),
    TraitVPtr(<S as L>),
    Method(<S as M>::foo_m),
    TraitVPtr(<S as M>),
    Method(<S as N>::foo_n),
    TraitVPtr(<S as N>),
    Method(<S as O>::foo_o),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:95:1
   |
   |
LL | / trait O: G + N {
LL | |     //~^ error Vtable
LL | |     fn foo_o(&self) {}
LL | | }


error: vtable entries for `<S as B>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as B>::foo_b),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:19:1
   |
   |
LL | / trait B {
LL | |     //~^ error Vtable
LL | |     fn foo_b(&self) {}
LL | | }


error: vtable entries for `<S as D>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as D>::foo_d),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:30:1
   |
   |
LL | / trait D {
LL | |     //~^ error Vtable
LL | |     fn foo_d(&self) {}
LL | | }


error: vtable entries for `<S as E>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as E>::foo_e),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:36:1
   |
   |
LL | / trait E {
LL | |     //~^ error Vtable
LL | |     fn foo_e(&self) {}
LL | | }


error: vtable entries for `<S as F>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as D>::foo_d),
    Method(<S as E>::foo_e),
    TraitVPtr(<S as E>),
    Method(<S as F>::foo_f),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:42:1
   |
   |
LL | / trait F: D + E {
LL | |     //~^ error Vtable
LL | |     fn foo_f(&self) {}
LL | | }


error: vtable entries for `<S as H>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as H>::foo_h),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:53:1
   |
   |
LL | / trait H {
LL | |     //~^ error Vtable
LL | |     fn foo_h(&self) {}
LL | | }


error: vtable entries for `<S as I>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as I>::foo_i),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:59:1
   |
   |
LL | / trait I {
LL | |     //~^ error Vtable
LL | |     fn foo_i(&self) {}
LL | | }


error: vtable entries for `<S as J>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as H>::foo_h),
    Method(<S as I>::foo_i),
    TraitVPtr(<S as I>),
    Method(<S as J>::foo_j),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:65:1
   |
   |
LL | / trait J: H + I {
LL | |     //~^ error Vtable
LL | |     fn foo_j(&self) {}
LL | | }


error: vtable entries for `<S as K>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as K>::foo_k),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:71:1
   |
   |
LL | / trait K {
LL | |     //~^ error Vtable
LL | |     fn foo_k(&self) {}
LL | | }


error: vtable entries for `<S as L>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as L>::foo_l),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:77:1
   |
   |
LL | / trait L {
LL | |     //~^ error Vtable
LL | |     fn foo_l(&self) {}
LL | | }


error: vtable entries for `<S as M>`: [
    MetadataDropInPlace,
    MetadataAlign,
    MetadataAlign,
    Method(<S as K>::foo_k),
    Method(<S as L>::foo_l),
    TraitVPtr(<S as L>),
    Method(<S as M>::foo_m),
  --> /checkout/src/test/ui/traits/vtable/vtable-multi-level.rs:83:1
   |
   |
LL | / trait M: K + L {
LL | |     //~^ error Vtable
LL | |     fn foo_m(&self) {}
LL | | }
---
test result: FAILED. 12085 passed; 40 failed; 115 ignored; 0 measured; 0 filtered out; finished in 133.68s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:40
