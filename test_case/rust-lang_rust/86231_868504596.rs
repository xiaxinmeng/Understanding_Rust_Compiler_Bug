plain
    Finished release [optimized] target(s) in 24.09s
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 12021 tests
...................................F.FF............................................................. 100/12021
.................................................................................................... 300/12021
.................................................................................................... 400/12021
.................................................................................................... 500/12021
.................................................................................................... 600/12021
---
.................................................................................................... 3200/12021
.................................................................................................... 3300/12021
.................................................................................................... 3400/12021
.................................................................................................... 3500/12021
..i........i..............F................................FFF.F.................................... 3600/12021
.......F............................................................................i............... 3800/12021
..............................i..................................................................... 3900/12021
.................................................................................................... 4000/12021
.................................................................................................... 4100/12021
---
.................................................................................................... 11500/12021
.................................................................................................... 11600/12021
.................................................................................................... 11700/12021
.................................................................................................... 11800/12021
.................................F...F.............................................................. 11900/12021
.....................
failures:

---- [ui] ui/abi/unsupported.rs#aarch64 stdout ----
---- [ui] ui/abi/unsupported.rs#aarch64 stdout ----
diff of stderr:

1 error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:20:1
3    |
3    |
4 LL | extern "ptx-kernel" fn ptx() {}

6 
6 
7 error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:22:1
9    |
9    |
10 LL | extern "amdgpu-kernel" fn amdgpu() {}

12 
12 
13 error[E0570]: `"wasm"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:24:1
15    |
15    |
16 LL | extern "wasm" fn wasm() {}

18 
18 
19 error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:29:1
21    |
21    |
22 LL | extern "msp430-interrupt" fn msp430() {}

24 
24 
25 error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:31:1
27    |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
28 LL | extern "avr-interrupt" fn avr() {}

30 
30 
31 error[E0570]: `"x86-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:33:1
33    |
33    |
34 LL | extern "x86-interrupt" fn x86() {}

36 
36 
37 error[E0570]: `"stdcall"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:35:1
39    |
39    |
40 LL | extern "stdcall" fn stdcall() {}

42 
42 
43 error[E0570]: `"thiscall"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:38:1
45    |
45    |
46 LL | extern "thiscall" fn thiscall() {}


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64/unsupported.aarch64.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `aarch64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/unsupported.rs" "-Zthreads=1" "--cfg" "aarch64" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=aarch64-unknown-linux-gnu" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.aarch64/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
   |
   |
LL | extern "ptx-kernel" fn ptx() {}


error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
   |
   |
LL | extern "amdgpu-kernel" fn amdgpu() {}


error[E0570]: `"wasm"` is not a supported ABI for the current target
   |
   |
LL | extern "wasm" fn wasm() {}


error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "msp430-interrupt" fn msp430() {}


error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "avr-interrupt" fn avr() {}


error[E0570]: `"x86-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "x86-interrupt" fn x86() {}


error[E0570]: `"stdcall"` is not a supported ABI for the current target
   |
   |
LL | extern "stdcall" fn stdcall() {}


error[E0570]: `"thiscall"` is not a supported ABI for the current target
   |
   |
LL | extern "thiscall" fn thiscall() {}

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0570`.
For more information about this error, try `rustc --explain E0570`.

------------------------------------------


---- [ui] ui/abi/unsupported.rs#i686 stdout ----


1 error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:20:1
3    |
3    |
4 LL | extern "ptx-kernel" fn ptx() {}

6 
6 
7 error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:22:1
9    |
9    |
10 LL | extern "amdgpu-kernel" fn amdgpu() {}

12 
12 
13 error[E0570]: `"wasm"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:24:1
15    |
15    |
16 LL | extern "wasm" fn wasm() {}

18 
18 
19 error[E0570]: `"aapcs"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:26:1
21    |
21    |
22 LL | extern "aapcs" fn aapcs() {}

24 
24 
25 error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:29:1
27    |
27    |
28 LL | extern "msp430-interrupt" fn msp430() {}

30 
30 
31 error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:31:1
33    |
33    |
34 LL | extern "avr-interrupt" fn avr() {}

36 
- error: aborting due to 6 previous errors
- error: aborting due to 6 previous errors
+ error[E0570]: `"stdcall"` is not a supported ABI for the current target
+    |
+    |
+ LL | extern "stdcall" fn stdcall() {}
+ 
+ 
+ error[E0570]: `"thiscall"` is not a supported ABI for the current target
+    |
+    |
+ LL | extern "thiscall" fn thiscall() {}
+ 
+ error: aborting due to 8 previous errors
38 
39 For more information about this error, try `rustc --explain E0570`.
39 For more information about this error, try `rustc --explain E0570`.
40 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.i686/unsupported.i686.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `i686`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/unsupported.rs" "-Zthreads=1" "--cfg" "i686" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.i686" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-unknown-linux-gnu" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.i686/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
   |
   |
LL | extern "ptx-kernel" fn ptx() {}


error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
   |
   |
LL | extern "amdgpu-kernel" fn amdgpu() {}


error[E0570]: `"wasm"` is not a supported ABI for the current target
   |
   |
LL | extern "wasm" fn wasm() {}


error[E0570]: `"aapcs"` is not a supported ABI for the current target
   |
   |
LL | extern "aapcs" fn aapcs() {}


error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "msp430-interrupt" fn msp430() {}


error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "avr-interrupt" fn avr() {}


error[E0570]: `"stdcall"` is not a supported ABI for the current target
   |
   |
LL | extern "stdcall" fn stdcall() {}


error[E0570]: `"thiscall"` is not a supported ABI for the current target
   |
   |
LL | extern "thiscall" fn thiscall() {}

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0570`.
For more information about this error, try `rustc --explain E0570`.

------------------------------------------


---- [ui] ui/abi/unsupported.rs#x64 stdout ----


1 error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:20:1
3    |
3    |
4 LL | extern "ptx-kernel" fn ptx() {}

6 
6 
7 error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:22:1
9    |
9    |
10 LL | extern "amdgpu-kernel" fn amdgpu() {}

12 
12 
13 error[E0570]: `"wasm"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:24:1
15    |
15    |
16 LL | extern "wasm" fn wasm() {}

18 
18 
19 error[E0570]: `"aapcs"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:26:1
21    |
21    |
22 LL | extern "aapcs" fn aapcs() {}

24 
24 
25 error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:29:1
27    |
27    |
28 LL | extern "msp430-interrupt" fn msp430() {}

30 
30 
31 error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:31:1
33    |
33    |
34 LL | extern "avr-interrupt" fn avr() {}

36 
36 
37 error[E0570]: `"stdcall"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:35:1
39    |
39    |
40 LL | extern "stdcall" fn stdcall() {}

42 
42 
43 error[E0570]: `"thiscall"` is not a supported ABI for the current target
-   --> $DIR/unsupported.rs:38:1
45    |
45    |
46 LL | extern "thiscall" fn thiscall() {}


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.x64/unsupported.x64.stderr
To only update this specific test, also pass `--test-args abi/unsupported.rs`


error in revision `x64`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/abi/unsupported.rs" "-Zthreads=1" "--cfg" "x64" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.x64" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=x86_64-unknown-linux-gnu" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/abi/unsupported.x64/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0570]: `"ptx-kernel"` is not a supported ABI for the current target
   |
   |
LL | extern "ptx-kernel" fn ptx() {}


error[E0570]: `"amdgpu-kernel"` is not a supported ABI for the current target
   |
   |
LL | extern "amdgpu-kernel" fn amdgpu() {}


error[E0570]: `"wasm"` is not a supported ABI for the current target
   |
   |
LL | extern "wasm" fn wasm() {}


error[E0570]: `"aapcs"` is not a supported ABI for the current target
   |
   |
LL | extern "aapcs" fn aapcs() {}


error[E0570]: `"msp430-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "msp430-interrupt" fn msp430() {}


error[E0570]: `"avr-interrupt"` is not a supported ABI for the current target
   |
   |
LL | extern "avr-interrupt" fn avr() {}


error[E0570]: `"stdcall"` is not a supported ABI for the current target
   |
   |
LL | extern "stdcall" fn stdcall() {}


error[E0570]: `"thiscall"` is not a supported ABI for the current target
   |
   |
LL | extern "thiscall" fn thiscall() {}

error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0570`.
For more information about this error, try `rustc --explain E0570`.

------------------------------------------


---- [ui] ui/c-variadic/variadic-ffi-1.rs stdout ----
diff of stderr:

+ error[E0570]: `"stdcall"` is not a supported ABI for the current target
+    |
+    |
+ LL | / extern "stdcall" {
+ LL | |     fn printf(_: *const u8, ...);
+ LL | | }
+ 
+ 
1 error[E0045]: C-variadic function must have C or cdecl calling convention
-   --> $DIR/variadic-ffi-1.rs:8:5
3    |
3    |
4 LL |     fn printf(_: *const u8, ...);
5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ C-variadics require C or cdecl calling convention
6 
7 error[E0060]: this function takes at least 2 arguments but 0 arguments were supplied
-   --> $DIR/variadic-ffi-1.rs:19:9
+   --> $DIR/variadic-ffi-1.rs:20:9
---
15 note: function defined here
-   --> $DIR/variadic-ffi-1.rs:12:8
+   --> $DIR/variadic-ffi-1.rs:13:8
17    |
18 LL |     fn foo(f: isize, x: u8, ...);

20 
21 error[E0060]: this function takes at least 2 arguments but 1 argument was supplied
-   --> $DIR/variadic-ffi-1.rs:20:9
---
29 note: function defined here
-   --> $DIR/variadic-ffi-1.rs:12:8
+   --> $DIR/variadic-ffi-1.rs:13:8
31    |
32 LL |     fn foo(f: isize, x: u8, ...);

34 
35 error[E0308]: mismatched types
-   --> $DIR/variadic-ffi-1.rs:22:56
-   --> $DIR/variadic-ffi-1.rs:22:56
+   --> $DIR/variadic-ffi-1.rs:23:56
37    |
38 LL |         let x: unsafe extern "C" fn(f: isize, x: u8) = foo;
39    |                -------------------------------------   ^^^ expected non-variadic fn, found variadic function

44                  found fn item `unsafe extern "C" fn(_, _, ...) {foo}`
46 error[E0308]: mismatched types
-   --> $DIR/variadic-ffi-1.rs:23:54
+   --> $DIR/variadic-ffi-1.rs:24:54
48    |
48    |
49 LL |         let y: extern "C" fn(f: isize, x: u8, ...) = bar;
50    |                -----------------------------------   ^^^ expected variadic fn, found non-variadic function

55                  found fn item `extern "C" fn(_, _) {bar}`
56 
57 error[E0617]: can't pass `f32` to variadic function
-   --> $DIR/variadic-ffi-1.rs:25:19
59    |
59    |
60 LL |         foo(1, 2, 3f32);
61    |                   ^^^^ help: cast the value to `c_double`: `3f32 as c_double`
62 
62 
63 error[E0617]: can't pass `bool` to variadic function
-   --> $DIR/variadic-ffi-1.rs:26:19
65    |
66 LL |         foo(1, 2, true);
66 LL |         foo(1, 2, true);
67    |                   ^^^^ help: cast the value to `c_int`: `true as c_int`
68 
68 
69 error[E0617]: can't pass `i8` to variadic function
-   --> $DIR/variadic-ffi-1.rs:27:19
71    |
71    |
72 LL |         foo(1, 2, 1i8);
73    |                   ^^^ help: cast the value to `c_int`: `1i8 as c_int`
74 
74 
75 error[E0617]: can't pass `u8` to variadic function
-   --> $DIR/variadic-ffi-1.rs:28:19
77    |
77    |
78 LL |         foo(1, 2, 1u8);
79    |                   ^^^ help: cast the value to `c_uint`: `1u8 as c_uint`
80 
80 
81 error[E0617]: can't pass `i16` to variadic function
-   --> $DIR/variadic-ffi-1.rs:29:19
83    |
83    |
84 LL |         foo(1, 2, 1i16);
85    |                   ^^^^ help: cast the value to `c_int`: `1i16 as c_int`
86 
86 
87 error[E0617]: can't pass `u16` to variadic function
-   --> $DIR/variadic-ffi-1.rs:30:19
89    |
89    |
90 LL |         foo(1, 2, 1u16);
91    |                   ^^^^ help: cast the value to `c_uint`: `1u16 as c_uint`
92 
- error: aborting due to 11 previous errors
+ error: aborting due to 12 previous errors
94 
94 
- Some errors have detailed explanations: E0045, E0060, E0308, E0617.
+ Some errors have detailed explanations: E0045, E0060, E0308, E0570, E0617.
96 For more information about an error, try `rustc --explain E0045`.
97 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/variadic-ffi-1.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-1.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-unknown-linux-gnu" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-1/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0570]: `"stdcall"` is not a supported ABI for the current target
   |
   |
LL | / extern "stdcall" {
LL | |     fn printf(_: *const u8, ...); //~ ERROR: variadic function must have C or cdecl calling
LL | | }


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
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0045, E0060, E0308, E0570, E0617.
For more information about an error, try `rustc --explain E0045`.
For more information about an error, try `rustc --explain E0045`.

------------------------------------------


---- [ui] ui/extern/extern-vectorcall.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/extern-vectorcall.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-vectorcall/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-vectorcall/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0570]: `"vectorcall"` is not a supported ABI for the current target
   |
   |
LL |     extern "vectorcall" fn test1(i: i32) {


error[E0570]: `"vectorcall"` is not a supported ABI for the current target
   |
   |
LL | extern "vectorcall" fn test2(i: i32) {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0570`.
For more information about this error, try `rustc --explain E0570`.

------------------------------------------


---- [ui] ui/feature-gates/feature-gate-abi-avr-interrupt.rs stdout ----
diff of stderr:

1 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:10:8
3    |
3    |
4 LL | extern "avr-non-blocking-interrupt" fn fu() {}


8    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
9 
10 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:12:8
12    |
12    |
13 LL | extern "avr-interrupt" fn f() {}


17    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
18 
19 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:16:12
21    |
21    |
22 LL |     extern "avr-interrupt" fn m();


26    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
27 
28 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:18:12
30    |
30    |
31 LL |     extern "avr-non-blocking-interrupt" fn mu();


35    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
36 
37 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:21:12
39    |
39    |
40 LL |     extern "avr-interrupt" fn dm() {}


44    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
45 
46 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:23:12
48    |
48    |
49 LL |     extern "avr-non-blocking-interrupt" fn dmu() {}


53    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
54 
55 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:29:12
57    |
57    |
58 LL |     extern "avr-interrupt" fn m() {}


62    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
63 
64 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:31:12
66    |
66    |
67 LL |     extern "avr-non-blocking-interrupt" fn mu() {}


71    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
72 
73 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:36:12
75    |
75    |
76 LL |     extern "avr-interrupt" fn im() {}


80    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
81 
82 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:38:12
84    |
84    |
85 LL |     extern "avr-non-blocking-interrupt" fn imu() {}


89    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
90 
91 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:42:18
93    |
93    |
94 LL | type TA = extern "avr-interrupt" fn();


98    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
99 
100 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:44:19
102    |
102    |
103 LL | type TAU = extern "avr-non-blocking-interrupt" fn();


107    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
108 
109 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:47:8
111    |
111    |
112 LL | extern "avr-interrupt" {}


116    = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable
117 
118 error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi-avr-interrupt.rs:49:8
120    |
120    |
121 LL | extern "avr-non-blocking-interrupt" {}


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt/feature-gate-abi-avr-interrupt.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt/feature-gate-abi-avr-interrupt.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi-avr-interrupt.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi-avr-interrupt.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=avr-unknown-gnu-atmega328" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi-avr-interrupt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | extern "avr-non-blocking-interrupt" fn fu() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | extern "avr-interrupt" fn f() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-interrupt" fn m();
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-non-blocking-interrupt" fn mu();
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-interrupt" fn dm() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-non-blocking-interrupt" fn dmu() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-interrupt" fn m() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-non-blocking-interrupt" fn mu() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-interrupt" fn im() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL |     extern "avr-non-blocking-interrupt" fn imu() {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | type TA = extern "avr-interrupt" fn();
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | type TAU = extern "avr-non-blocking-interrupt" fn();
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | extern "avr-interrupt" {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
   = help: add `#![feature(abi_avr_interrupt)]` to the crate attributes to enable

error[E0658]: avr-interrupt and avr-non-blocking-interrupt ABIs are experimental and subject to change
   |
   |
LL | extern "avr-non-blocking-interrupt" {}
   |
   = note: see issue #69664 <https://github.com/rust-lang/rust/issues/69664> for more information
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-abi.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-abi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-abi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:15:8
   |
LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:17:8
   |
LL | extern "platform-intrinsic" fn f2() {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:19:8
   |
   |
LL | extern "vectorcall" fn f3() {} //~ ERROR vectorcall is experimental and subject to change
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable

error[E0658]: rust-call ABI is subject to change
   |
   |
LL | extern "rust-call" fn f4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL | extern "x86-interrupt" fn f7() {} //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL | extern "efiapi" fn f10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:26:12
   |
   |
LL |     extern "rust-intrinsic" fn m1(); //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:28:12
   |
LL |     extern "platform-intrinsic" fn m2(); //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:30:12
   |
   |
LL |     extern "vectorcall" fn m3(); //~ ERROR vectorcall is experimental and subject to change
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable

error[E0658]: rust-call ABI is subject to change
   |
   |
LL |     extern "rust-call" fn m4(_: ()); //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "x86-interrupt" fn m7(); //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn m10(); //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:35:12
   |
   |
LL |     extern "vectorcall" fn dm3() {} //~ ERROR vectorcall is experimental and subject to change
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable

error[E0658]: rust-call ABI is subject to change
   |
   |
LL |     extern "rust-call" fn dm4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "x86-interrupt" fn dm7() {} //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn dm10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:45:12
   |
   |
LL |     extern "rust-intrinsic" fn m1() {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:47:12
   |
LL |     extern "platform-intrinsic" fn m2() {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:49:12
   |
   |
LL |     extern "vectorcall" fn m3() {} //~ ERROR vectorcall is experimental and subject to change
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable

error[E0658]: rust-call ABI is subject to change
   |
   |
LL |     extern "rust-call" fn m4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "x86-interrupt" fn m7() {} //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn m10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:57:12
   |
   |
LL |     extern "rust-intrinsic" fn im1() {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:59:12
   |
LL |     extern "platform-intrinsic" fn im2() {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:61:12
   |
   |
LL |     extern "vectorcall" fn im3() {} //~ ERROR vectorcall is experimental and subject to change
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable

error[E0658]: rust-call ABI is subject to change
   |
   |
LL |     extern "rust-call" fn im4(_: ()) {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL |     extern "x86-interrupt" fn im7() {} //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL |     extern "efiapi" fn im10() {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:68:18
   |
   |
LL | type A1 = extern "rust-intrinsic" fn(); //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:69:18
   |
LL | type A2 = extern "platform-intrinsic" fn(); //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:70:18
   |
   |
LL | type A3 = extern "vectorcall" fn(); //~ ERROR vectorcall is experimental and subject to change
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable

error[E0658]: rust-call ABI is subject to change
   |
   |
LL | type A4 = extern "rust-call" fn(_: ()); //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL | type A7 = extern "x86-interrupt" fn(); //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL | type A10 = extern "efiapi" fn(); //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable
error[E0658]: intrinsics are subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:76:8
   |
   |
LL | extern "rust-intrinsic" {} //~ ERROR intrinsics are subject to change
   |
   = help: add `#![feature(intrinsics)]` to the crate attributes to enable

error[E0658]: platform intrinsics are experimental and possibly buggy
error[E0658]: platform intrinsics are experimental and possibly buggy
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:77:8
   |
LL | extern "platform-intrinsic" {} //~ ERROR platform intrinsics are experimental
   |
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = note: see issue #27731 <https://github.com/rust-lang/rust/issues/27731> for more information
   = help: add `#![feature(platform_intrinsics)]` to the crate attributes to enable
error[E0658]: vectorcall is experimental and subject to change
  --> /checkout/src/test/ui/feature-gates/feature-gate-abi.rs:78:8
   |
   |
LL | extern "vectorcall" {} //~ ERROR vectorcall is experimental and subject to change
   |
   |
   = help: add `#![feature(abi_vectorcall)]` to the crate attributes to enable

error[E0658]: rust-call ABI is subject to change
   |
   |
LL | extern "rust-call" {} //~ ERROR rust-call ABI is subject to change
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable


error[E0658]: x86-interrupt ABI is experimental and subject to change
   |
   |
LL | extern "x86-interrupt" {} //~ ERROR x86-interrupt ABI is experimental
   |
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = note: see issue #40180 <https://github.com/rust-lang/rust/issues/40180> for more information
   = help: add `#![feature(abi_x86_interrupt)]` to the crate attributes to enable

error[E0658]: efiapi ABI is experimental and subject to change
   |
   |
LL | extern "efiapi" {} //~ ERROR efiapi ABI is experimental and subject to change
   |
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = note: see issue #65815 <https://github.com/rust-lang/rust/issues/65815> for more information
   = help: add `#![feature(abi_efiapi)]` to the crate attributes to enable

error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "rust-intrinsic" fn m1(); //~ ERROR intrinsics are subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "platform-intrinsic" fn m2(); //~ ERROR platform intrinsics are experimental


error[E0570]: `"vectorcall"` is not a supported ABI for the current target
   |
   |
LL | extern "vectorcall" {} //~ ERROR vectorcall is experimental and subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL | extern "rust-intrinsic" fn f1() {} //~ ERROR intrinsics are subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL | extern "platform-intrinsic" fn f2() {} //~ ERROR platform intrinsics are experimental


error[E0570]: `"vectorcall"` is not a supported ABI for the current target
   |
   |
LL | extern "vectorcall" fn f3() {} //~ ERROR vectorcall is experimental and subject to change


error[E0570]: `"vectorcall"` is not a supported ABI for the current target
   |
   |
LL |     extern "vectorcall" fn dm3() {} //~ ERROR vectorcall is experimental and subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "rust-intrinsic" fn m1() {} //~ ERROR intrinsics are subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "platform-intrinsic" fn m2() {} //~ ERROR platform intrinsics are experimental


error[E0570]: `"vectorcall"` is not a supported ABI for the current target
   |
   |
LL |     extern "vectorcall" fn m3() {} //~ ERROR vectorcall is experimental and subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "rust-intrinsic" fn im1() {} //~ ERROR intrinsics are subject to change


error: intrinsic must be in `extern "rust-intrinsic" { ... }` block
   |
   |
LL |     extern "platform-intrinsic" fn im2() {} //~ ERROR platform intrinsics are experimental


error[E0570]: `"vectorcall"` is not a supported ABI for the current target
   |
   |
LL |     extern "vectorcall" fn im3() {} //~ ERROR vectorcall is experimental and subject to change

error: aborting due to 53 previous errors

Some errors have detailed explanations: E0570, E0658.
Some errors have detailed explanations: E0570, E0658.
For more information about an error, try `rustc --explain E0570`.

------------------------------------------


---- [ui] ui/feature-gates/feature-gate-abi_ptx.rs stdout ----


1 error[E0658]: PTX ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi_ptx.rs:7:8
+   --> $DIR/feature-gate-abi_ptx.rs:8:8
3    |
4 LL | extern "ptx-kernel" fn fu() {}


8    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
9 
10 error[E0658]: PTX ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi_ptx.rs:10:12
+   --> $DIR/feature-gate-abi_ptx.rs:11:12
12    |
13 LL |     extern "ptx-kernel" fn mu();


17    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
18 
19 error[E0658]: PTX ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi_ptx.rs:11:12
+   --> $DIR/feature-gate-abi_ptx.rs:12:12
21    |
22 LL |     extern "ptx-kernel" fn dmu() {}


26    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
27 
28 error[E0658]: PTX ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi_ptx.rs:16:12
+   --> $DIR/feature-gate-abi_ptx.rs:17:12
30    |
31 LL |     extern "ptx-kernel" fn mu() {}


35    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
36 
37 error[E0658]: PTX ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi_ptx.rs:20:12
+   --> $DIR/feature-gate-abi_ptx.rs:21:12
39    |
40 LL |     extern "ptx-kernel" fn imu() {}


44    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
45 
46 error[E0658]: PTX ABIs are experimental and subject to change
-   --> $DIR/feature-gate-abi_ptx.rs:23:19
+   --> $DIR/feature-gate-abi_ptx.rs:24:19
48    |
49 LL | type TAU = extern "ptx-kernel" fn();


53    = help: add `#![feature(abi_ptx)]` to the crate attributes to enable
---
1 error[E0658]: stdcall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-stdcall-unwind.rs:11:8
+   --> $DIR/feature-gate-stdcall-unwind.rs:12:8
3    |
4 LL | extern "stdcall-unwind" fn fu() {}

8    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
9 
10 error[E0658]: stdcall-unwind ABI is experimental and subject to change
10 error[E0658]: stdcall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-stdcall-unwind.rs:14:12
+   --> $DIR/feature-gate-stdcall-unwind.rs:15:12
12    |
13 LL |     extern "stdcall-unwind" fn mu();

17    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
18 
19 error[E0658]: stdcall-unwind ABI is experimental and subject to change
19 error[E0658]: stdcall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-stdcall-unwind.rs:15:12
+   --> $DIR/feature-gate-stdcall-unwind.rs:16:12
21    |
22 LL |     extern "stdcall-unwind" fn dmu() {}

26    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
27 
28 error[E0658]: stdcall-unwind ABI is experimental and subject to change
28 error[E0658]: stdcall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-stdcall-unwind.rs:20:12
+   --> $DIR/feature-gate-stdcall-unwind.rs:21:12
30    |
31 LL |     extern "stdcall-unwind" fn mu() {}

35    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
36 
37 error[E0658]: stdcall-unwind ABI is experimental and subject to change
37 error[E0658]: stdcall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-stdcall-unwind.rs:24:12
+   --> $DIR/feature-gate-stdcall-unwind.rs:25:12
39    |
40 LL |     extern "stdcall-unwind" fn imu() {}

44    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
45 
46 error[E0658]: stdcall-unwind ABI is experimental and subject to change
46 error[E0658]: stdcall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-stdcall-unwind.rs:27:19
+   --> $DIR/feature-gate-stdcall-unwind.rs:28:19
48    |
49 LL | type TAU = extern "stdcall-unwind" fn();

53    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
54 
55 error[E0658]: stdcall-unwind ABI is experimental and subject to change
55 error[E0658]: stdcall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-stdcall-unwind.rs:29:8
+   --> $DIR/feature-gate-stdcall-unwind.rs:30:8
57    |
58 LL | extern "stdcall-unwind" {}

61    = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
62    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
63 
63 
- error: aborting due to 7 previous errors
+ error[E0570]: `"stdcall-unwind"` is not a supported ABI for the current target
+    |
+    |
+ LL | extern "stdcall-unwind" {}
65 
- For more information about this error, try `rustc --explain E0658`.
- For more information about this error, try `rustc --explain E0658`.
+ error[E0570]: `"stdcall-unwind"` is not a supported ABI for the current target
+    |
+    |
+ LL | extern "stdcall-unwind" fn fu() {}
+ 
+ 
+ error[E0570]: `"stdcall-unwind"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "stdcall-unwind" fn dmu() {}
+ 
+ 
+ error[E0570]: `"stdcall-unwind"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "stdcall-unwind" fn mu() {}
+ 
+ 
+ error[E0570]: `"stdcall-unwind"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "stdcall-unwind" fn imu() {}
+ 
+ error: aborting due to 12 previous errors
+ 
+ Some errors have detailed explanations: E0570, E0658.
---
To only update this specific test, also pass `--test-args unwind-abis/feature-gate-stdcall-unwind.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/feature-gate-stdcall-unwind" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-unknown-linux-gnu" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/feature-gate-stdcall-unwind/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:12:8
   |
LL | extern "stdcall-unwind" fn fu() {} //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:15:12
   |
LL |     extern "stdcall-unwind" fn mu(); //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:16:12
   |
LL |     extern "stdcall-unwind" fn dmu() {} //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:21:12
   |
LL |     extern "stdcall-unwind" fn mu() {} //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:25:12
   |
LL |     extern "stdcall-unwind" fn imu() {} //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:28:19
   |
LL | type TAU = extern "stdcall-unwind" fn(); //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: stdcall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-stdcall-unwind.rs:30:8
   |
LL | extern "stdcall-unwind" {} //~ ERROR stdcall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0570]: `"stdcall-unwind"` is not a supported ABI for the current target
   |
   |
LL | extern "stdcall-unwind" {} //~ ERROR stdcall-unwind ABI is experimental


error[E0570]: `"stdcall-unwind"` is not a supported ABI for the current target
   |
   |
LL | extern "stdcall-unwind" fn fu() {} //~ ERROR stdcall-unwind ABI is experimental


error[E0570]: `"stdcall-unwind"` is not a supported ABI for the current target
   |
   |
LL |     extern "stdcall-unwind" fn dmu() {} //~ ERROR stdcall-unwind ABI is experimental


error[E0570]: `"stdcall-unwind"` is not a supported ABI for the current target
   |
   |
LL |     extern "stdcall-unwind" fn mu() {} //~ ERROR stdcall-unwind ABI is experimental


error[E0570]: `"stdcall-unwind"` is not a supported ABI for the current target
   |
   |
LL |     extern "stdcall-unwind" fn imu() {} //~ ERROR stdcall-unwind ABI is experimental

error: aborting due to 12 previous errors

Some errors have detailed explanations: E0570, E0658.
---
1 error[E0658]: thiscall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:12:8
+   --> $DIR/feature-gate-thiscall-unwind.rs:13:8
3    |
4 LL | extern "thiscall-unwind" fn fu() {}

8    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
9 
10 error[E0658]: thiscall is experimental and subject to change
10 error[E0658]: thiscall is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:13:8
+   --> $DIR/feature-gate-thiscall-unwind.rs:14:8
12    |
13 LL | extern "thiscall" fn f() {}


16    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
18 error[E0658]: thiscall is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:16:12
+   --> $DIR/feature-gate-thiscall-unwind.rs:17:12
20    |
20    |
21 LL |     extern "thiscall" fn m();


24    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
26 error[E0658]: thiscall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:17:12
+   --> $DIR/feature-gate-thiscall-unwind.rs:18:12
28    |
28    |
29 LL |     extern "thiscall-unwind" fn mu();

33    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
34 
35 error[E0658]: thiscall is experimental and subject to change
35 error[E0658]: thiscall is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:19:12
+   --> $DIR/feature-gate-thiscall-unwind.rs:20:12
37    |
38 LL |     extern "thiscall" fn dm() {}


41    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
43 error[E0658]: thiscall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:20:12
+   --> $DIR/feature-gate-thiscall-unwind.rs:21:12
45    |
45    |
46 LL |     extern "thiscall-unwind" fn dmu() {}

50    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
51 
52 error[E0658]: thiscall is experimental and subject to change
52 error[E0658]: thiscall is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:25:12
+   --> $DIR/feature-gate-thiscall-unwind.rs:26:12
54    |
55 LL |     extern "thiscall" fn m() {}


58    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
60 error[E0658]: thiscall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:26:12
+   --> $DIR/feature-gate-thiscall-unwind.rs:27:12
62    |
62    |
63 LL |     extern "thiscall-unwind" fn mu() {}

67    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
68 
69 error[E0658]: thiscall is experimental and subject to change
69 error[E0658]: thiscall is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:30:12
+   --> $DIR/feature-gate-thiscall-unwind.rs:31:12
71    |
72 LL |     extern "thiscall" fn im() {}


75    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
77 error[E0658]: thiscall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:31:12
+   --> $DIR/feature-gate-thiscall-unwind.rs:32:12
79    |
79    |
80 LL |     extern "thiscall-unwind" fn imu() {}

84    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
85 
86 error[E0658]: thiscall is experimental and subject to change
86 error[E0658]: thiscall is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:34:18
+   --> $DIR/feature-gate-thiscall-unwind.rs:35:18
88    |
89 LL | type TA = extern "thiscall" fn();


92    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
94 error[E0658]: thiscall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:35:19
+   --> $DIR/feature-gate-thiscall-unwind.rs:36:19
96    |
96    |
97 LL | type TAU = extern "thiscall-unwind" fn();

101    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
102 
103 error[E0658]: thiscall is experimental and subject to change
103 error[E0658]: thiscall is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:37:8
+   --> $DIR/feature-gate-thiscall-unwind.rs:38:8
105    |
106 LL | extern "thiscall" {}


109    = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
111 error[E0658]: thiscall-unwind ABI is experimental and subject to change
-   --> $DIR/feature-gate-thiscall-unwind.rs:38:8
+   --> $DIR/feature-gate-thiscall-unwind.rs:39:8
113    |
113    |
114 LL | extern "thiscall-unwind" {}

117    = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
118    = help: add `#![feature(c_unwind)]` to the crate attributes to enable
119 
119 
- error: aborting due to 14 previous errors
+ error[E0570]: `"thiscall"` is not a supported ABI for the current target
+    |
+    |
+ LL | extern "thiscall" {}
121 
- For more information about this error, try `rustc --explain E0658`.
- For more information about this error, try `rustc --explain E0658`.
+ error[E0570]: `"thiscall-unwind"` is not a supported ABI for the current target
+    |
+    |
+ LL | extern "thiscall-unwind" {}
+ 
+ 
+ error[E0570]: `"thiscall-unwind"` is not a supported ABI for the current target
+    |
+    |
+ LL | extern "thiscall-unwind" fn fu() {}
+ 
+ 
+ error[E0570]: `"thiscall"` is not a supported ABI for the current target
+    |
+    |
+ LL | extern "thiscall" fn f() {}
+ 
+ 
+ error[E0570]: `"thiscall"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "thiscall" fn dm() {}
+ 
+ 
+ error[E0570]: `"thiscall-unwind"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "thiscall-unwind" fn dmu() {}
+ 
+ 
+ error[E0570]: `"thiscall"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "thiscall" fn m() {}
+ 
+ 
+ error[E0570]: `"thiscall-unwind"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "thiscall-unwind" fn mu() {}
+ 
+ 
+ error[E0570]: `"thiscall"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "thiscall" fn im() {}
+ 
+ 
+ error[E0570]: `"thiscall-unwind"` is not a supported ABI for the current target
+    |
+    |
+ LL |     extern "thiscall-unwind" fn imu() {}
+ 
+ error: aborting due to 24 previous errors
+ 
+ Some errors have detailed explanations: E0570, E0658.
---
To only update this specific test, also pass `--test-args unwind-abis/feature-gate-thiscall-unwind.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/feature-gate-thiscall-unwind" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=i686-unknown-linux-gnu" "--crate-type=rlib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unwind-abis/feature-gate-thiscall-unwind/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:13:8
   |
LL | extern "thiscall-unwind" fn fu() {} //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:14:8
   |
LL | extern "thiscall" fn f() {} //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:17:12
   |
   |
LL |     extern "thiscall" fn m(); //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:18:12
   |
   |
LL |     extern "thiscall-unwind" fn mu(); //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:20:12
   |
LL |     extern "thiscall" fn dm() {} //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:21:12
   |
   |
LL |     extern "thiscall-unwind" fn dmu() {} //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:26:12
   |
LL |     extern "thiscall" fn m() {} //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:27:12
   |
   |
LL |     extern "thiscall-unwind" fn mu() {} //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:31:12
   |
LL |     extern "thiscall" fn im() {} //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:32:12
   |
   |
LL |     extern "thiscall-unwind" fn imu() {} //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:35:18
   |
LL | type TA = extern "thiscall" fn(); //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:36:19
   |
   |
LL | type TAU = extern "thiscall-unwind" fn(); //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0658]: thiscall is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:38:8
   |
LL | extern "thiscall" {} //~ ERROR thiscall is experimental
   |
   |
   = help: add `#![feature(abi_thiscall)]` to the crate attributes to enable
error[E0658]: thiscall-unwind ABI is experimental and subject to change
  --> /checkout/src/test/ui/unwind-abis/feature-gate-thiscall-unwind.rs:39:8
   |
   |
LL | extern "thiscall-unwind" {} //~ ERROR thiscall-unwind ABI is experimental
   |
   = note: see issue #74990 <https://github.com/rust-lang/rust/issues/74990> for more information
   = help: add `#![feature(c_unwind)]` to the crate attributes to enable


error[E0570]: `"thiscall"` is not a supported ABI for the current target
   |
   |
LL | extern "thiscall" {} //~ ERROR thiscall is experimental


error[E0570]: `"thiscall-unwind"` is not a supported ABI for the current target
   |
   |
LL | extern "thiscall-unwind" {} //~ ERROR thiscall-unwind ABI is experimental


error[E0570]: `"thiscall-unwind"` is not a supported ABI for the current target
   |
   |
LL | extern "thiscall-unwind" fn fu() {} //~ ERROR thiscall-unwind ABI is experimental


error[E0570]: `"thiscall"` is not a supported ABI for the current target
   |
   |
LL | extern "thiscall" fn f() {} //~ ERROR thiscall is experimental


error[E0570]: `"thiscall"` is not a supported ABI for the current target
   |
   |
LL |     extern "thiscall" fn dm() {} //~ ERROR thiscall is experimental


error[E0570]: `"thiscall-unwind"` is not a supported ABI for the current target
   |
   |
LL |     extern "thiscall-unwind" fn dmu() {} //~ ERROR thiscall-unwind ABI is experimental


error[E0570]: `"thiscall"` is not a supported ABI for the current target
   |
   |
LL |     extern "thiscall" fn m() {} //~ ERROR thiscall is experimental


error[E0570]: `"thiscall-unwind"` is not a supported ABI for the current target
   |
   |
LL |     extern "thiscall-unwind" fn mu() {} //~ ERROR thiscall-unwind ABI is experimental


error[E0570]: `"thiscall"` is not a supported ABI for the current target
   |
   |
LL |     extern "thiscall" fn im() {} //~ ERROR thiscall is experimental


error[E0570]: `"thiscall-unwind"` is not a supported ABI for the current target
   |
   |
LL |     extern "thiscall-unwind" fn imu() {} //~ ERROR thiscall-unwind ABI is experimental

error: aborting due to 24 previous errors

Some errors have detailed explanations: E0570, E0658.
---
    [ui] ui/extern/extern-vectorcall.rs
    [ui] ui/feature-gates/feature-gate-abi-avr-interrupt.rs
    [ui] ui/feature-gates/feature-gate-abi-msp430-interrupt.rs
    [ui] ui/feature-gates/feature-gate-abi.rs
    [ui] ui/feature-gates/feature-gate-abi_ptx.rs
    [ui] ui/unwind-abis/feature-gate-stdcall-unwind.rs
    [ui] ui/unwind-abis/feature-gate-thiscall-unwind.rs

test result: FAILED. 11910 passed; 12 failed; 99 ignored; 0 measured; 0 filtered out; finished in 122.06s
test result: FAILED. 11910 passed; 12 failed; 99 ignored; 0 measured; 0 filtered out; finished in 122.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:25
