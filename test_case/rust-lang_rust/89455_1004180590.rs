plain
.................................................................................................... 900/12532
.................................................................................................... 1000/12532
.................................................................................................... 1100/12532
.................................................................................................... 1200/12532
........i............................................................................F..F....F...... 1300/12532
......................................F............................................................. 1500/12532
.................................................................................F.................. 1600/12532
i................................................................................................... 1700/12532
.................................................................................................... 1800/12532
---
.................................................................................................... 2500/12532
.................................................................................................... 2600/12532
.................................................................................................... 2700/12532
.................................................................................................... 2800/12532
..................................i............................F...FF...FF.......................... 2900/12532
.................................................iiiii.............................................. 3100/12532
.................................................................................................... 3200/12532
.......................................................................................F............ 3300/12532
.......................................................................................F............ 3300/12532
...FF.F............................................................................................. 3400/12532
................................................................................................i... 3600/12532
................................................................................................i... 3600/12532
......i.........i.............................................F........................F............ 3700/12532
.......................................................................ii.........F......F.......... 3800/12532
.................................................................................................... 4000/12532
.................................................................................................... 4100/12532
.................................................................................................... 4200/12532
.................................................................................................... 4300/12532
---
.......................i............................................................................ 6300/12532
........................................................................................i........... 6400/12532
.......................................ii.ii........i...i........................................... 6500/12532
.................................................................................................... 6600/12532
.........................i.....i..................F....................i................i........... 6700/12532
..i.....................................................i.................................F......... 6800/12532
.........................................................i.............F.......F..F.............F... 6900/12532
.i............................................................F..................................... 7100/12532
......ii................................ii..........................F..............................i 7200/12532
.................................................................................................... 7300/12532
.............................................................i...................................... 7400/12532
---
.................................i...............................................F.................. 8500/12532
.......................................................................................i............ 8600/12532
.................................................................................................... 8700/12532
..................................................................i................................. 8800/12532
................................................F................................................... 8900/12532
............................F....F.........................F........................................ 9000/12532
.................................................................................................... 9200/12532
.................................................................................................iii 9300/12532
i.iiiii..................................................................ii...............i......... 9400/12532
.................................................................................................... 9500/12532
.................................................................................................... 9500/12532
.................................................................................................... 9600/12532
.................................................................................................... 9700/12532
.................................................................................................... 9800/12532
.................................................................................................... 9900/12532
.................................................................................................... 10000/12532
....................F..FF..FFF..FF..F.FF...F.F...FF...............................i..ii.i........... 10100/12532
.....i.............................................................................................. 10300/12532
.iiiiii.i..iiiiii.i................................................................................. 10400/12532
......................................................................................F............. 10500/12532
......................................................................................F............. 10500/12532
.........................................F.........................F................................ 10600/12532
.................................................................................................... 10800/12532
.................................................................................................... 10900/12532
.................................................................................................... 11000/12532
.................................................................................................... 11100/12532
---
---- [ui] ui/c-variadic/variadic-ffi-1.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | C-variadic function must have C or cdecl calling convention
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-1.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-pc-windows-msvc" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0045]: C-variadic function must have C or cdecl calling convention
   |
   |
LL |     fn printf(_: *const u8, ...); //~ ERROR: variadic function must have C or cdecl calling
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadics require C or cdecl calling convention
error[E0060]: this function takes at least 2 arguments but 0 arguments were supplied
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:20:9
   |
   |
LL |         foo(); //~ ERROR this function takes at least 2 arguments but 0 arguments were supplied
   |         ^^^-- supplied 0 arguments
   |         expected at least 2 arguments
   |
note: function defined here
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:13:8
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:13:8
   |
LL |     fn foo(f: isize, x: u8, ...);

error[E0060]: this function takes at least 2 arguments but 1 argument was supplied
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:21:9
   |
   |
LL |         foo(1); //~ ERROR this function takes at least 2 arguments but 1 argument was supplied
   |         ^^^ - supplied 1 argument
   |         expected at least 2 arguments
   |
note: function defined here
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:13:8
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:13:8
   |
LL |     fn foo(f: isize, x: u8, ...);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:23:56
   |
   |
LL |         let x: unsafe extern "C" fn(f: isize, x: u8) = foo; //~ ERROR mismatched types
   |                -------------------------------------   ^^^ expected non-variadic fn, found variadic function
   |                expected due to this
   |
   |
   = note: expected fn pointer `unsafe extern "C" fn(_, _)`
                 found fn item `unsafe extern "C" fn(_, _, ...) {foo}`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:24:54
   |
   |
LL |         let y: extern "C" fn(f: isize, x: u8, ...) = bar; //~ ERROR mismatched types
   |                -----------------------------------   ^^^ expected variadic fn, found non-variadic function
   |                expected due to this
   |
   |
   = note: expected fn pointer `extern "C" fn(_, _, ...)`
                 found fn item `extern "C" fn(_, _) {bar}`

error[E0617]: can't pass `f32` to variadic function
   |
   |
LL |         foo(1, 2, 3f32); //~ ERROR can't pass
   |                   ^^^^ help: cast the value to `c_double`: `3f32 as c_double`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error[E0617]: can't pass `bool` to variadic function
   |
   |
LL |         foo(1, 2, true); //~ ERROR can't pass
   |                   ^^^^ help: cast the value to `c_int`: `true as c_int`

error[E0617]: can't pass `i8` to variadic function
   |
   |
LL |         foo(1, 2, 1i8); //~ ERROR can't pass
   |                   ^^^ help: cast the value to `c_int`: `1i8 as c_int`

error[E0617]: can't pass `u8` to variadic function
   |
   |
LL |         foo(1, 2, 1u8); //~ ERROR can't pass
   |                   ^^^ help: cast the value to `c_uint`: `1u8 as c_uint`

error[E0617]: can't pass `i16` to variadic function
   |
   |
LL |         foo(1, 2, 1i16); //~ ERROR can't pass
   |                   ^^^^ help: cast the value to `c_int`: `1i16 as c_int`

error[E0617]: can't pass `u16` to variadic function
   |
   |
LL |         foo(1, 2, 1u16); //~ ERROR can't pass
   |                   ^^^^ help: cast the value to `c_uint`: `1u16 as c_uint`
error: aborting due to 11 previous errors

Some errors have detailed explanations: E0045, E0060, E0308, E0617.
For more information about an error, try `rustc --explain E0045`.
---
---- [ui] ui/c-variadic/variadic-ffi-2.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | C-variadic function must have C or cdecl calling convention
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0045]: C-variadic function must have C or cdecl calling convention
   |
   |
LL | fn baz(f: extern "stdcall" fn(usize, ...)) {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadics require C or cdecl calling convention
error: aborting due to previous error

For more information about this error, try `rustc --explain E0045`.


------------------------------------------


---- [ui] ui/c-variadic/variadic-ffi-no-fixed-args.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | C-variadic function must be declared with at least one named argument
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-no-fixed-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-no-fixed-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-no-fixed-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
---- [ui] ui/closures/2229_closure_analysis/filter-on-struct-member.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | First Pass analysis includes:
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/2229_closure_analysis/filter-on-struct-member.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/filter-on-struct-member" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/2229_closure_analysis/filter-on-struct-member/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: First Pass analysis includes:
   |
   |
LL |             |v| self.filter.allowed(*v),
   |
   |
note: Capturing self[Deref,(0, 0)] -> ImmBorrow
   |
   |
LL |             |v| self.filter.allowed(*v),


error: Min Capture analysis includes:
   |
   |
LL |             |v| self.filter.allowed(*v),
   |
   |
note: Min Capture self[Deref,(0, 0)] -> ImmBorrow
   |
   |
LL |             |v| self.filter.allowed(*v),

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/cmse-nonsecure/cmse-nonsecure-call/gate_test.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | C-cmse-nonsecure-call ABI is experimental and subject to change
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cmse-nonsecure/cmse-nonsecure-call/gate_test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/gate_test" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cmse-nonsecure/cmse-nonsecure-call/gate_test/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: C-cmse-nonsecure-call ABI is experimental and subject to change
   |
   |
LL |         core::mem::transmute::<usize, extern "C-cmse-nonsecure-call" fn(i32, i32, i32, i32) -> i32>(
   |
   = note: see issue #81391 <https://github.com/rust-lang/rust/issues/81391> for more information
   = note: see issue #81391 <https://github.com/rust-lang/rust/issues/81391> for more information
   = help: add `#![feature(abi_c_cmse_nonsecure_call)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/dep-graph/dep-graph-assoc-type-codegen.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | OK
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/dep-graph-assoc-type-codegen.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-assoc-type-codegen/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-assoc-type-codegen.rs:29:5
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR OK

error: aborting due to previous error



------------------------------------------


---- [ui] ui/dep-graph/dep-graph-caller-callee.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | OK
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/dep-graph-caller-callee.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-caller-callee/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs:21:5
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR OK


error: no path from `x` to `typeck`
  --> /checkout/src/test/ui/dep-graph/dep-graph-caller-callee.rs:32:5
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR no path

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | OK
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/dep-graph-trait-impl-two-traits-same-method.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs:33:5
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR OK


error: no path from `x::<impl Foo for u32>` to `typeck`
  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl-two-traits-same-method.rs:42:5
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR no path

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/dep-graph/dep-graph-trait-impl.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | OK
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/dep-graph-trait-impl.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-trait-impl/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs:28:5
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs:33:5
   |
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs:38:5
   |
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR OK

error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs:43:5
   |
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR OK


error: no path from `x::<impl Foo for char>` to `typeck`
  --> /checkout/src/test/ui/dep-graph/dep-graph-trait-impl.rs:56:5
   |
LL |     #[rustc_then_this_would_need(typeck)] //~ ERROR no path

error: aborting due to 5 previous errors



------------------------------------------


---- [ui] ui/dep-graph/dep-graph-variance-alias.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | OK
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/dep-graph-variance-alias.inc" "-Z" "incremental-verify-ich" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dep-graph/dep-graph-variance-alias/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: OK
  --> /checkout/src/test/ui/dep-graph/dep-graph-variance-alias.rs:19:1
   |
LL | #[rustc_then_this_would_need(variances_of)] //~ ERROR OK

error: aborting due to previous error



------------------------------------------


---- [ui] ui/error-codes/E0045.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | C-variadic function must have C or cdecl calling convention
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0045.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0045" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0045/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0045]: C-variadic function must have C or cdecl calling convention
   |
   |
LL | extern "Rust" { fn foo(x: u8, ...); }   //~ ERROR E0045
   |                 ^^^^^^^^^^^^^^^^^^^ C-variadics require C or cdecl calling convention
error: aborting due to previous error

For more information about this error, try `rustc --explain E0045`.


------------------------------------------


---- [ui] ui/error-codes/E0075.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | SIMD vector cannot be empty
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0075.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0075" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0075/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0075]: SIMD vector cannot be empty
  --> /checkout/src/test/ui/error-codes/E0075.rs:4:1
   |
LL | struct Bad; //~ ERROR E0075

error: aborting due to previous error

For more information about this error, try `rustc --explain E0075`.
---
---- [ui] ui/error-codes/E0076.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | SIMD vector should be homogeneous
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0076.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0076" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0076/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0076]: SIMD vector should be homogeneous
  --> /checkout/src/test/ui/error-codes/E0076.rs:4:1
   |
LL | struct Bad(u16, u32, u32);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^ SIMD elements must have the same type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0076`.


------------------------------------------


---- [ui] ui/error-codes/E0077.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | SIMD vector element type should be a primitive scalar (integer/float/pointer) type
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0077.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0077" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0077/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0077]: SIMD vector element type should be a primitive scalar (integer/float/pointer) type
   |
   |
LL | struct Bad(String); //~ ERROR E0077

error: aborting due to previous error

For more information about this error, try `rustc --explain E0077`.
---
---- [ui] ui/feature-gates/feature-gate-abi_ptx.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | PTX ABIs are experimental and subject to change
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi_ptx.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi_ptx" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=nvptx64-nvidia-cuda" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi_ptx/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL | extern "ptx-kernel" fn fu() {} //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL |     extern "ptx-kernel" fn mu(); //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL |     extern "ptx-kernel" fn dmu() {} //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL |     extern "ptx-kernel" fn mu() {} //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL |     extern "ptx-kernel" fn imu() {} //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL | type TAU = extern "ptx-kernel" fn(); //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable

error[E0658]: PTX ABIs are experimental and subject to change
   |
   |
LL | extern "ptx-kernel" {} //~ ERROR PTX ABIs are experimental
   |
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = note: see issue #38788 <https://github.com/rust-lang/rust/issues/38788> for more information
   = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/feature-gates/feature-gate-c_variadic.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | C-variadic functions are unstable
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-c_variadic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-c_variadic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-c_variadic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: C-variadic functions are unstable
  --> /checkout/src/test/ui/feature-gates/feature-gate-c_variadic.rs:3:1
   |
LL | pub unsafe extern "C" fn test(_: i32, ap: ...) { }
   |
   = note: see issue #44930 <https://github.com/rust-lang/rust/issues/44930> for more information
   = help: add `#![feature(c_variadic)]` to the crate attributes to enable

---
---- [ui] ui/feature-gates/feature-gate-repr-simd.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | SIMD types are experimental and possibly buggy
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-repr-simd.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-repr-simd" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-repr-simd/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: SIMD types are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-repr-simd.rs:1:1
   |
LL | #[repr(simd)] //~ error: SIMD types are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(repr_simd)]` to the crate attributes to enable


error[E0658]: SIMD types are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-repr-simd.rs:6:1
   |
LL | #[repr(simd)] //~ error: SIMD types are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(repr_simd)]` to the crate attributes to enable


error[E0566]: conflicting representation hints
  --> /checkout/src/test/ui/feature-gates/feature-gate-repr-simd.rs:4:8
   |
LL | #[repr(C)] //~ ERROR conflicting representation hints
   |        ^
LL | //~^ WARN this was previously accepted
LL | #[repr(simd)] //~ error: SIMD types are experimental
   |
   = note: `#[deny(conflicting_repr_hints)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #68585 <https://github.com/rust-lang/rust/issues/68585>
---
---- [ui] ui/feature-gates/feature-gate-simd.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | SIMD types are experimental and possibly buggy
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-simd.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-simd" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-simd/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: SIMD types are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-simd.rs:3:1
   |
LL | #[repr(simd)] //~ ERROR SIMD types are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(repr_simd)]` to the crate attributes to enable

---
---- [ui] ui/linkage-attr/issue-10755.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | No such file or directory (os error 2)
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/linkage-attr/issue-10755.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/issue-10755" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "linker=llllll" "-C" "linker-flavor=ld" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/linkage-attr/issue-10755/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: linker `llllll` not found
   |
   = note: No such file or directory (os error 2)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/lint/lint-enum-intrinsics-non-enums.rs stdout ----

error: warning: diagnostic messages should not end with punctuations
  |
1 | the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `()`, which is not an enum.
  |                                                                                                                               - this is a punctuation
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: punctuated-endings` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-enum-intrinsics-non-enums" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-enum-intrinsics-non-enums/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:26:5
   |
LL |     discriminant(&());
   |
   |
   = note: `#[deny(enum_intrinsics_non_enums)]` on by default
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `()`, which is not an enum.
   |
   |
LL |     discriminant(&());

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:29:5
   |
   |
LL |     discriminant(&&SomeEnum::B);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `&SomeEnum`, which is not an enum.
   |
   |
LL |     discriminant(&&SomeEnum::B);

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:32:5
   |
   |
LL |     discriminant(&SomeStruct);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `SomeStruct`, which is not an enum.
   |
   |
LL |     discriminant(&SomeStruct);

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:35:5
   |
   |
LL |     discriminant(&123u32);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `u32`, which is not an enum.
   |
   |
LL |     discriminant(&123u32);

error: the return value of `mem::discriminant` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:38:5
   |
   |
LL |     discriminant(&&123i8);
   |
   |
note: the argument to `discriminant` should be a reference to an enum, but it was passed a reference to a `&i8`, which is not an enum.
   |
   |
LL |     discriminant(&&123i8);

error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:46:5
   |
   |
LL |     variant_count::<&str>();
   |
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `&str`, which is not an enum.
error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:49:5
   |
   |
LL |     variant_count::<*const u8>();
   |
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `*const u8`, which is not an enum.
error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:52:5
   |
LL |     variant_count::<()>();
LL |     variant_count::<()>();
   |     ^^^^^^^^^^^^^^^^^^^^^
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `()`, which is not an enum.
error: the return value of `mem::variant_count` is unspecified when called with a non-enum type
  --> /checkout/src/test/ui/lint/lint-enum-intrinsics-non-enums.rs:55:5
   |
   |
LL |     variant_count::<&SomeEnum>();
   |
   |
   = note: the type parameter of `variant_count` should be an enum, but it was instantiated with the type `&SomeEnum`, which is not an enum.
error: aborting due to 9 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/lint/must_not_suspend/boxed.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | You gotta use Umm's, ya know?
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_not_suspend/boxed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/boxed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/boxed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: boxed `Umm` held across a suspend point, but should not be
   |
   |
LL |     let _guard = bar(); //~ ERROR boxed `Umm` held across
   |         ^^^^^^
LL |     other().await;
   |            ------ the value is held across this suspend point
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/must_not_suspend/boxed.rs:3:9
   |
   |
LL | #![deny(must_not_suspend)]
   |         ^^^^^^^^^^^^^^^^
note: You gotta use Umm's, ya know?
   |
   |
LL |     let _guard = bar(); //~ ERROR boxed `Umm` held across
   |         ^^^^^^
help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
   |
   |
LL |     let _guard = bar(); //~ ERROR boxed `Umm` held across

error: aborting due to previous error



------------------------------------------


---- [ui] ui/lint/must_not_suspend/ref.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | You gotta use Umm's, ya know?
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_not_suspend/ref.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/ref" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/ref/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `Umm` held across a suspend point, but should not be
   |
   |
LL |         let guard = &mut self.u; //~ ERROR `Umm` held across
LL | 
LL | 
LL |         other().await;
   |                ------ the value is held across this suspend point
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/must_not_suspend/ref.rs:3:9
   |
   |
LL | #![deny(must_not_suspend)]
   |         ^^^^^^^^^^^^^^^^
note: You gotta use Umm's, ya know?
   |
   |
LL |         let guard = &mut self.u; //~ ERROR `Umm` held across
   |                          ^^^^^^
help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
   |
   |
LL |         let guard = &mut self.u; //~ ERROR `Umm` held across

error: aborting due to previous error



------------------------------------------


---- [ui] ui/lint/must_not_suspend/unit.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | You gotta use Umm's, ya know?
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_not_suspend/unit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/unit" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/unit/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `Umm` held across a suspend point, but should not be
   |
   |
LL |     let _guard = bar(); //~ ERROR `Umm` held across
   |         ^^^^^^
LL |     other().await;
   |            ------ the value is held across this suspend point
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/must_not_suspend/unit.rs:3:9
   |
   |
LL | #![deny(must_not_suspend)]
   |         ^^^^^^^^^^^^^^^^
note: You gotta use Umm's, ya know?
   |
   |
LL |     let _guard = bar(); //~ ERROR `Umm` held across
   |         ^^^^^^
help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
   |
   |
LL |     let _guard = bar(); //~ ERROR `Umm` held across

error: aborting due to previous error



------------------------------------------


---- [ui] ui/lint/must_not_suspend/warn.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | You gotta use Umm's, ya know?
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/must_not_suspend/warn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/warn/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/must_not_suspend/warn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `Umm` held across a suspend point, but should not be
   |
   |
LL |     let _guard = bar(); //~ WARNING `Umm` held across
   |         ^^^^^^
LL |     other().await;
   |            ------ the value is held across this suspend point
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/must_not_suspend/warn.rs:4:9
   |
   |
LL | #![warn(must_not_suspend)]
   |         ^^^^^^^^^^^^^^^^
note: You gotta use Umm's, ya know?
   |
   |
LL |     let _guard = bar(); //~ WARNING `Umm` held across
   |         ^^^^^^
help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
   |
   |
LL |     let _guard = bar(); //~ WARNING `Umm` held across

warning: 1 warning emitted



------------------------------------------


---- [ui] ui/lto/lto-duplicate-symbols.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | Linking globals named 'foo': symbol multiply defined!
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lto/lto-duplicate-symbols.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-duplicate-symbols" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "lto" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lto/lto-duplicate-symbols/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: Linking globals named 'foo': symbol multiply defined!

error: failed to load bc of "lto-duplicate-symbols2.lto_duplicate_symbols2.71086114-cgu.0.rcgu.o": 
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/macros/local-ambiguity-multiple-parsing-options.rs stdout ----

error: warning: diagnostic messages should not end with punctuations
  |
1 | local ambiguity when calling macro `ambiguity`: multiple parsing options: built-in NTs ident ('i') or ident ('j').
  |                                                                                                                  - this is a punctuation
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: punctuated-endings` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/local-ambiguity-multiple-parsing-options.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/local-ambiguity-multiple-parsing-options" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/local-ambiguity-multiple-parsing-options/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: local ambiguity when calling macro `ambiguity`: multiple parsing options: built-in NTs ident ('i') or ident ('j').
   |
   |
LL | ambiguity!(error); //~ ERROR local ambiguity


error: local ambiguity when calling macro `ambiguity`: multiple parsing options: built-in NTs ident ('i') or ident ('j').
   |
   |
LL | ambiguity!(error); //~ ERROR local ambiguity

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/object-lifetime/object-lifetime-default.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | BaseDefault
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: BaseDefault
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default.rs:6:1
   |
LL | struct A<T>(T); //~ ERROR BaseDefault

error: BaseDefault
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default.rs:9:1
   |
   |
LL | struct B<'a,T>(&'a (), T); //~ ERROR BaseDefault

error: 'a
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default.rs:12:1
   |
   |
LL | struct C<'a,T:'a>(&'a T); //~ ERROR 'a

error: Ambiguous
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default.rs:15:1
   |
   |
LL | struct D<'a,'b,T:'a+'b>(&'a T, &'b T); //~ ERROR Ambiguous

error: 'b
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default.rs:18:1
   |
   |
LL | struct E<'a,'b:'a,T:'b>(&'a T, &'b T); //~ ERROR 'b

error: 'a,'b
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default.rs:21:1
   |
   |
LL | struct F<'a,'b,T:'a,U:'b>(&'a T, &'b U); //~ ERROR 'a,'b


error: 'a,Ambiguous
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default.rs:24:1
   |
LL | struct G<'a,'b,T:'a,U:'a+'b>(&'a T, &'b U); //~ ERROR 'a,Ambiguous

error: aborting due to 7 previous errors



------------------------------------------


---- [ui] ui/parser/emoji-identifiers.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | Unicode character '➖' (Heavy Minus Sign) looks like '-' (Minus/Hyphen), but it is not
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/emoji-identifiers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{2796}
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:13:33
   |
LL |     let _ = i_like_to_😄_a_lot() ➖ 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope
   |
   |
help: Unicode character '➖' (Heavy Minus Sign) looks like '-' (Minus/Hyphen), but it is not
   |
LL |     let _ = i_like_to_😄_a_lot() - 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope


error[E0425]: cannot find function `i_like_to_😄_a_lot` in this scope
   |
   |
LL | fn i_like_to_😅_a_lot() -> 👀 { //~ ERROR identifiers cannot contain emoji
   | ----------------------------- similarly named function `i_like_to_😅_a_lot` defined here
...
LL |     let _ = i_like_to_😄_a_lot() ➖ 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope
   |             ^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `i_like_to_😅_a_lot`
error: Ferris cannot be used as an identifier
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:17:9
   |
   |
LL |     let 🦀 = 1;//~ ERROR Ferris cannot be used as an identifier
---
---- [ui] ui/parser/recover-from-homoglyph.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/recover-from-homoglyph.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-homoglyph" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/recover-from-homoglyph/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{37e}
   |
   |
LL |     println!(""); //~ ERROR unknown start of token: \u{37e}
   |
   |
help: Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
   |
LL |     println!(""); //~ ERROR unknown start of token: \u{37e}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/recover-from-homoglyph.rs:3:20
   |
   |
LL |     let x: usize = (); //~ ERROR mismatched types
   |            -----   ^^ expected `usize`, found `()`
   |            expected due to this

error: aborting due to 2 previous errors

---
---- [ui] ui/parser/unicode-chars.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unicode-chars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-chars" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-chars/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{37e}
   |
LL |     let y = 0;
   |              ^
   |
   |
help: Unicode character ';' (Greek Question Mark) looks like ';' (Semicolon), but it is not
LL |     let y = 0;
   |              ~

error: aborting due to previous error
---
---- [ui] ui/parser/unicode-quote-chars.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) look like '"' (Quotation Mark), but are not
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/unicode-quote-chars.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/unicode-quote-chars/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{201c}
   |
   |
LL |     println!(“hello world”);
   |
   |
help: Unicode characters '“' (Left Double Quotation Mark) and '”' (Right Double Quotation Mark) look like '"' (Quotation Mark), but are not
LL |     println!("hello world");
   |              ~~~~~~~~~~~~~


error: unknown start of token: \u{201d}
   |
   |
LL |     println!(“hello world”);
   |
   |
help: Unicode character '”' (Right Double Quotation Mark) looks like '"' (Quotation Mark), but it is not
   |
LL |     println!(“hello world");


error: expected `,`, found `world`
   |
   |
LL |     println!(“hello world”);
   |                     ^^^^^ expected `,`
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/parser/variadic-ffi-nested-syntactic-fail.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | C-variadic type `...` may not be nested inside another type
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/variadic-ffi-nested-syntactic-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-nested-syntactic-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/variadic-ffi-nested-syntactic-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0743]: C-variadic type `...` may not be nested inside another type
   |
   |
LL | fn f1<'a>(x: u8, y: &'a ...) {}


error[E0743]: C-variadic type `...` may not be nested inside another type
   |
   |
LL | fn f2<'a>(x: u8, y: Vec<&'a ...>) {}

error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/variadic-ffi-nested-syntactic-fail.rs:8:33
   |
   |
LL |     let _recovery_witness: () = 0; //~ ERROR mismatched types
   |                            --   ^ expected `()`, found integer
   |                            expected due to this

error: aborting due to 3 previous errors

---
---- [ui] ui/rfc-2093-infer-outlives/cross-crate.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | T: 'a
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/cross-crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/cross-crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/cross-crate.rs:4:1
   |
LL | / struct Foo<'a, T> { //~ ERROR rustc_outlives
LL | |     bar: std::slice::IterMut<'a, T>
LL | | }
   |
   = note: T: 'a

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/enum.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | T: 'a
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/enum.rs:7:1
   |
LL | / enum Foo<'a, T> { //~ ERROR rustc_outlives
LL | |     One(Bar<'a, T>)
LL | | }
   |
   = note: T: 'a

error: rustc_outlives
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/enum.rs:13:1
   |
LL | / struct Bar<'b, U> { //~ ERROR rustc_outlives
LL | |     field2: &'b U
LL | | }
   |
   = note: U: 'b

error: rustc_outlives
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/enum.rs:19:1
   |
LL | / enum Ying<'c, K> { //~ ERROR rustc_outlives
LL | |     One(&'c Yang<K>)
LL | | }
   |
   |
   = note: K: 'c
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/rfc-2093-infer-outlives/explicit-dyn.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | A: 'a
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/explicit-dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/explicit-dyn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/explicit-dyn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/explicit-dyn.rs:8:1
   |
LL | / struct Foo<'a, A> //~ ERROR rustc_outlives
LL | | {
LL | |     foo: Box<dyn Trait<'a, A>>
LL | | }
   |
   = note: A: 'a

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/explicit-enum.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | U: 'a
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/explicit-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/explicit-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/explicit-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/explicit-enum.rs:4:1
   |
LL | / enum Foo<'a, U> { //~ ERROR rustc_outlives
LL | |     One(Bar<'a, U>)
LL | | }
   |
   = note: U: 'a

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/explicit-struct.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | U: 'b
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/explicit-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/explicit-struct" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/explicit-struct/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/explicit-struct.rs:4:1
   |
LL | / struct Foo<'b, U> { //~ ERROR rustc_outlives
LL | |     bar: Bar<'b, U>
LL | | }
   |
   = note: U: 'b

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/explicit-projection.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | B: 'a
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/explicit-projection.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/explicit-projection" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/explicit-projection/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/explicit-projection.rs:8:1
   |
LL | / struct Foo<'a, A, B> where A: Trait<'a, B> //~ ERROR rustc_outlives
LL | | {
LL | |     foo: <A as Trait<'a, B>>::Type
LL | | }
   |
   = note: B: 'a

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/explicit-union.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | U: 'b
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/explicit-union.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/explicit-union" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/explicit-union/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/explicit-union.rs:5:1
   |
LL | / union Foo<'b, U: Copy> { //~ ERROR rustc_outlives
LL | |     bar: Bar<'b, U>
LL | | }
   |
   = note: U: 'b

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/infer-static.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | U: 'static
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/infer-static.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/infer-static" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/infer-static/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/infer-static.rs:5:1
   |
LL | / struct Foo<U> { //~ ERROR rustc_outlives
LL | |     bar: Bar<U>
LL | | }
   |
   = note: U: 'static

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/nested-regions.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | T: 'b
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/nested-regions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/nested-regions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/nested-regions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/nested-regions.rs:4:1
   |
LL | / struct Foo<'a, 'b, T> { //~ ERROR rustc_outlives
LL | |     x: &'a &'b T
LL | | }
   |
   |
   = note: 'b: 'a
   = note: T: 'a
   = note: T: 'b
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/rfc-2093-infer-outlives/nested-enum.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | T: 'a
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/nested-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/nested-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/nested-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/nested-enum.rs:4:1
   |
LL | / enum Foo<'a, T> { //~ ERROR rustc_outlives
LL | |
LL | |     One(Bar<'a, T>)
LL | | }
   |
   = note: T: 'a

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/nested-union.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | T: 'a
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/nested-union.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/nested-union" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/nested-union/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/nested-union.rs:5:1
   |
LL | / union Foo<'a, T: Copy> { //~ ERROR rustc_outlives
LL | |     field1: Bar<'a, T>
LL | | }
   |
   = note: T: 'a

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/nested-structs.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | T: 'a
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/nested-structs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/nested-structs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/nested-structs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/nested-structs.rs:4:1
   |
LL | / struct Foo<'a, T> { //~ ERROR rustc_outlives
LL | |     field1: Bar<'a, T>
LL | | }
   |
   = note: T: 'a

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/reference.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | T: 'a
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/reference.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/reference" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/reference/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/reference.rs:4:1
   |
LL | / struct Foo<'a, T> { //~ ERROR rustc_outlives
LL | |     bar: &'a T,
LL | | }
   |
   = note: T: 'a

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/self-structs.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | T: 'a
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/self-structs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/self-structs" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/self-structs/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/self-structs.rs:4:1
   |
LL | / struct Foo<'a, 'b, T> { //~ ERROR rustc_outlives
LL | |     field1: dyn Bar<'a, 'b, T>
LL | | }
   |
   = note: T: 'a

error: aborting due to previous error
---
---- [ui] ui/rfc-2093-infer-outlives/self-dyn.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | A: 'a
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2093-infer-outlives/self-dyn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/self-dyn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2093-infer-outlives/self-dyn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: rustc_outlives
  --> /checkout/src/test/ui/rfc-2093-infer-outlives/self-dyn.rs:9:1
   |
LL | / struct Foo<'a, 'b, A> //~ ERROR rustc_outlives
LL | | {
LL | |     foo: Box<dyn Trait<'a, 'b, A>>
LL | | }
   |
   = note: A: 'a

error: aborting due to previous error
---
---- [ui] ui/simd/type-len.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | SIMD vector cannot be empty
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/type-len.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-len" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/type-len/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0075]: SIMD vector cannot be empty
  --> /checkout/src/test/ui/simd/type-len.rs:6:1
   |
LL | struct empty; //~ ERROR SIMD vector cannot be empty

error[E0075]: SIMD vector cannot be empty
  --> /checkout/src/test/ui/simd/type-len.rs:9:1
   |
   |
LL | struct empty2([f32; 0]); //~ ERROR SIMD vector cannot be empty

error[E0076]: SIMD vector should be homogeneous
  --> /checkout/src/test/ui/simd/type-len.rs:15:1
   |
   |
LL | struct i64f64(i64, f64); //~ ERROR SIMD vector should be homogeneous
   | ^^^^^^^^^^^^^^^^^^^^^^^^ SIMD elements must have the same type

error[E0077]: SIMD vector element type should be a primitive scalar (integer/float/pointer) type
   |
   |
LL | struct FooV(Foo, Foo); //~ ERROR SIMD vector element type should be a primitive scalar (integer/float/pointer) type


error[E0077]: SIMD vector element type should be a primitive scalar (integer/float/pointer) type
   |
   |
LL | struct FooV2([Foo; 2]); //~ ERROR SIMD vector element type should be a primitive scalar (integer/float/pointer) type

error[E0075]: SIMD vector cannot have more than 32768 elements
  --> /checkout/src/test/ui/simd/type-len.rs:26:1
   |
   |
LL | struct TooBig([f32; 65536]); //~ ERROR SIMD vector cannot have more than 32768 elements

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0075, E0076, E0077.
---
---- [ui] ui/span/gated-features-attr-spans.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | SIMD types are experimental and possibly buggy
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/gated-features-attr-spans.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/gated-features-attr-spans" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/gated-features-attr-spans/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: SIMD types are experimental and possibly buggy
  --> /checkout/src/test/ui/span/gated-features-attr-spans.rs:1:1
   |
LL | #[repr(simd)] //~ ERROR are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(repr_simd)]` to the crate attributes to enable

---
---- [ui] ui/span/issue-81800.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | Unicode character '˂' (Modifier Letter Left Arrowhead) looks like '<' (Less-Than Sign), but it is not
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-81800.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-81800" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-81800/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{2c2}
   |
   |
LL | fn x˂- //~ ERROR: unknown start of token
   |
   |
help: Unicode character '˂' (Modifier Letter Left Arrowhead) looks like '<' (Less-Than Sign), but it is not
   |
LL | fn x<- //~ ERROR: unknown start of token


error: expected one of `#`, `>`, `const`, identifier, or lifetime, found `-`
   |
   |
LL | fn x˂- //~ ERROR: unknown start of token
   |      ^ expected one of `#`, `>`, `const`, identifier, or lifetime
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/unwind-abis/feature-gate-c-unwind.rs stdout ----

error: warning: diagnostic messages should start with lowercase letters
  |
1 | C-unwind ABI is experimental and subject to change
  | - this is an uppercase letter
  |
  = note: for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>
  = help: to disable this lint, add `ignored-diaglints: capitalized-messages` to top of the test file
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unwind-abis/feature-gate-c-unwind.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/feature-gate-c-unwind" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/feature-gate-c-unwind/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: C-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-c-unwind.rs:4:8
   |
LL | extern "C-unwind" fn f() {}
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable

---
test result: FAILED. 12362 passed; 51 failed; 119 ignored; 0 measured; 0 filtered out; finished in 144.39s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:40
