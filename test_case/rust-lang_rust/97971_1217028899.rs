plain
........................................................................................ 1056/13392
........................................................................................ 1144/13392
........................................................................................ 1232/13392
..................................................i..................................... 1320/13392
...........................................F......FF.................................... 1408/13392
........................................................................................ 1584/13392
........................................................................................ 1672/13392
...........................................................i......ii.................... 1760/13392
........................................................................................ 1848/13392
---
........................................................................................ 13376/13392
................
failures:

---- [ui] src/test/ui/c-variadic/feature-gate-extended_varargs_abi_support.rs stdout ----


- error[E0658]: using different calling convention than C or cdecl for varargs functions is unstable
+ error[E0658]: using different calling convention than `C` or `cdecl` for varargs functions is unstable
2   --> $DIR/feature-gate-extended_varargs_abi_support.rs:3:14
3    |
4 LL | fn efiapi(f: extern "efiapi" fn(usize, ...)) {
7    = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
7    = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
8    = help: add `#![feature(extended_varargs_abi_support)]` to the crate attributes to enable
9 
- error[E0045]: C-variadic function must have a compatible calling convention, like C or cdecl
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ error[E0045]: C-variadic function must have a compatible calling convention, like `C` or `cdecl`
11   --> $DIR/feature-gate-extended_varargs_abi_support.rs:3:14
12    |
13 LL | fn efiapi(f: extern "efiapi" fn(usize, ...)) {
14    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadic function must have a compatible calling convention
15 
15 
- error[E0658]: using different calling convention than C or cdecl for varargs functions is unstable
+ error[E0658]: using different calling convention than `C` or `cdecl` for varargs functions is unstable
17   --> $DIR/feature-gate-extended_varargs_abi_support.rs:8:12
18    |
19 LL | fn sysv(f: extern "sysv64" fn(usize, ...)) {
22    = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
22    = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
23    = help: add `#![feature(extended_varargs_abi_support)]` to the crate attributes to enable
24 
- error[E0045]: C-variadic function must have a compatible calling convention, like C or cdecl
+ error[E0045]: C-variadic function must have a compatible calling convention, like `C` or `cdecl`
26   --> $DIR/feature-gate-extended_varargs_abi_support.rs:8:12
27    |
28 LL | fn sysv(f: extern "sysv64" fn(usize, ...)) {
29    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadic function must have a compatible calling convention
30 
30 
- error[E0658]: using different calling convention than C or cdecl for varargs functions is unstable
+ error[E0658]: using different calling convention than `C` or `cdecl` for varargs functions is unstable
32   --> $DIR/feature-gate-extended_varargs_abi_support.rs:13:11
33    |
34 LL | fn win(f: extern "win64" fn(usize, ...)) {
37    = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
37    = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
38    = help: add `#![feature(extended_varargs_abi_support)]` to the crate attributes to enable
39 
- error[E0045]: C-variadic function must have a compatible calling convention, like C or cdecl
+ error[E0045]: C-variadic function must have a compatible calling convention, like `C` or `cdecl`
41   --> $DIR/feature-gate-extended_varargs_abi_support.rs:13:11
42    |
43 LL | fn win(f: extern "win64" fn(usize, ...)) {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/feature-gate-extended_varargs_abi_support/feature-gate-extended_varargs_abi_support.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args c-variadic/feature-gate-extended_varargs_abi_support.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/feature-gate-extended_varargs_abi_support.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/feature-gate-extended_varargs_abi_support" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/feature-gate-extended_varargs_abi_support/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0658]: using different calling convention than `C` or `cdecl` for varargs functions is unstable
  --> /checkout/src/test/ui/c-variadic/feature-gate-extended_varargs_abi_support.rs:3:14
   |
LL | fn efiapi(f: extern "efiapi" fn(usize, ...)) {
   |
   = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
   = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
   = help: add `#![feature(extended_varargs_abi_support)]` to the crate attributes to enable

error[E0045]: C-variadic function must have a compatible calling convention, like `C` or `cdecl`
  --> /checkout/src/test/ui/c-variadic/feature-gate-extended_varargs_abi_support.rs:3:14
   |
LL | fn efiapi(f: extern "efiapi" fn(usize, ...)) {
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadic function must have a compatible calling convention

error[E0658]: using different calling convention than `C` or `cdecl` for varargs functions is unstable
  --> /checkout/src/test/ui/c-variadic/feature-gate-extended_varargs_abi_support.rs:8:12
   |
LL | fn sysv(f: extern "sysv64" fn(usize, ...)) {
   |
   = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
   = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
   = help: add `#![feature(extended_varargs_abi_support)]` to the crate attributes to enable

error[E0045]: C-variadic function must have a compatible calling convention, like `C` or `cdecl`
  --> /checkout/src/test/ui/c-variadic/feature-gate-extended_varargs_abi_support.rs:8:12
   |
LL | fn sysv(f: extern "sysv64" fn(usize, ...)) {
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadic function must have a compatible calling convention

error[E0658]: using different calling convention than `C` or `cdecl` for varargs functions is unstable
  --> /checkout/src/test/ui/c-variadic/feature-gate-extended_varargs_abi_support.rs:13:11
   |
LL | fn win(f: extern "win64" fn(usize, ...)) {
   |
   = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
   = note: see issue #100189 <https://github.com/rust-lang/rust/issues/100189> for more information
   = help: add `#![feature(extended_varargs_abi_support)]` to the crate attributes to enable

error[E0045]: C-variadic function must have a compatible calling convention, like `C` or `cdecl`
  --> /checkout/src/test/ui/c-variadic/feature-gate-extended_varargs_abi_support.rs:13:11
   |
LL | fn win(f: extern "win64" fn(usize, ...)) {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadic function must have a compatible calling convention
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0045, E0658.
For more information about an error, try `rustc --explain E0045`.
For more information about an error, try `rustc --explain E0045`.
------------------------------------------


---- [ui] src/test/ui/c-variadic/variadic-ffi-1.rs stdout ----
diff of stderr:

- error[E0045]: C-variadic function must have a compatible calling convention, like C or cdecl
+ error[E0045]: C-variadic function must have a compatible calling convention, like `C` or `cdecl`
3    |
3    |
4 LL |     fn printf(_: *const u8, ...);

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/variadic-ffi-1.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-1.rs" "-Zthreads=1" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-pc-windows-msvc" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0045]: C-variadic function must have a compatible calling convention, like `C` or `cdecl`
   |
   |
LL |     fn printf(_: *const u8, ...);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadic function must have a compatible calling convention
error[E0060]: this function takes at least 2 arguments but 0 arguments were supplied
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:22:9
   |
   |
LL |         foo(); //~ ERROR this function takes at least 2 arguments but 0 arguments were supplied
   |         ^^^-- two arguments of type `isize` and `u8` are missing
note: function defined here
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:15:8
   |
   |
LL |     fn foo(f: isize, x: u8, ...);
help: provide the arguments
   |
   |
LL |         foo(/* isize */, /* u8 */); //~ ERROR this function takes at least 2 arguments but 0 arguments were supplied

error[E0060]: this function takes at least 2 arguments but 1 argument was supplied
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:23:9
   |
   |
LL |         foo(1); //~ ERROR this function takes at least 2 arguments but 1 argument was supplied
   |         ^^^--- an argument of type `u8` is missing
note: function defined here
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:15:8
   |
   |
LL |     fn foo(f: isize, x: u8, ...);
help: provide the argument
   |
   |
LL |         foo(1, /* u8 */); //~ ERROR this function takes at least 2 arguments but 1 argument was supplied

error[E0308]: mismatched types
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:25:56
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
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-1.rs:26:54
   |
   |
LL |         let y: extern "C" fn(f: isize, x: u8, ...) = bar; //~ ERROR mismatched types
   |                -----------------------------------   ^^^ expected variadic fn, found non-variadic function
   |                expected due to this
   |
   = note: expected fn pointer `extern "C" fn(_, _, ...)`
   = note: expected fn pointer `extern "C" fn(_, _, ...)`
                 found fn item `extern "C" fn(_, _) {bar}`

error[E0617]: can't pass `f32` to variadic function
   |
   |
LL |         foo(1, 2, 3f32); //~ ERROR can't pass
   |                   ^^^^ help: cast the value to `c_double`: `3f32 as c_double`

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
For more information about an error, try `rustc --explain E0045`.
------------------------------------------


---- [ui] src/test/ui/c-variadic/variadic-ffi-2.rs stdout ----
diff of stderr:

- error[E0045]: C-variadic function must have a compatible calling convention, like C, cdecl, win64, sysv64 or efiapi
+ error[E0045]: C-variadic function must have a compatible calling convention, like `C`, `cdecl`, `win64`, `sysv64` or `efiapi`
3    |
3    |
4 LL | fn baz(f: extern "stdcall" fn(usize, ...)) {

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-2/variadic-ffi-2.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0045]: C-variadic function must have a compatible calling convention, like `C`, `cdecl`, `win64`, `sysv64` or `efiapi`
   |
   |
LL | fn baz(f: extern "stdcall" fn(usize, ...)) {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadic function must have a compatible calling convention
error: aborting due to previous error

For more information about this error, try `rustc --explain E0045`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/error-codes/E0045.rs stdout ----
diff of stderr:

- error[E0045]: C-variadic function must have a compatible calling convention, like C or cdecl
+ error[E0045]: C-variadic function must have a compatible calling convention, like `C` or `cdecl`
2   --> $DIR/E0045.rs:1:17
3    |
4 LL | extern "Rust" { fn foo(x: u8, ...); }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0045/E0045.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0045.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0045.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0045" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0045/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0045]: C-variadic function must have a compatible calling convention, like `C` or `cdecl`
   |
   |
LL | extern "Rust" { fn foo(x: u8, ...); }   //~ ERROR E0045
   |                 ^^^^^^^^^^^^^^^^^^^ C-variadic function must have a compatible calling convention
error: aborting due to previous error

For more information about this error, try `rustc --explain E0045`.
------------------------------------------
