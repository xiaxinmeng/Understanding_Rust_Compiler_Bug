plain

---- [ui] ui/asm/aarch64/duplicate-options.rs stdout ----
diff of stderr:

1 error: the `nomem` option was already provided
-   --> $DIR/duplicate-options.rs:8:33
3    |
3    |
4 LL |         asm!("", options(nomem, nomem));
5    |                                 ^^^^^ this option was already provided
6 
6 
7 error: the `preserves_flags` option was already provided
-   --> $DIR/duplicate-options.rs:10:43
9    |
9    |
10 LL |         asm!("", options(preserves_flags, preserves_flags));
11    |                                           ^^^^^^^^^^^^^^^ this option was already provided
12 
12 
13 error: the `nostack` option was already provided
-   --> $DIR/duplicate-options.rs:12:61
15    |
15    |
16 LL |         asm!("", options(nostack, preserves_flags), options(nostack));
17    |                                                             ^^^^^^^ this option was already provided
Some tests failed in compiletest suite=ui mode=ui host=aarch64-unknown-linux-gnu target=aarch64-unknown-linux-gnu
18 
18 
19 error: the `nostack` option was already provided
-   --> $DIR/duplicate-options.rs:14:35
21    |
21    |
22 LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));
23    |                                   ^^^^^^^ this option was already provided
24 
24 
25 error: the `nostack` option was already provided
-   --> $DIR/duplicate-options.rs:14:53
27    |
27    |
28 LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));
29    |                                                     ^^^^^^^ this option was already provided
30 
30 
31 error: the `nostack` option was already provided
-   --> $DIR/duplicate-options.rs:14:71
33    |
33    |
34 LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));
35    |                                                                       ^^^^^^^ this option was already provided
36 
36 
37 error: the `noreturn` option was already provided
-   --> $DIR/duplicate-options.rs:21:38
39    |
39    |
40 LL |             options(preserves_flags, noreturn),
41    |                                      ^^^^^^^^ this option was already provided
42 
42 
43 error: the `nomem` option was already provided
-   --> $DIR/duplicate-options.rs:22:21
45    |
45    |
46 LL |             options(nomem, nostack),
47    |                     ^^^^^ this option was already provided
48 
48 
49 error: the `noreturn` option was already provided
-   --> $DIR/duplicate-options.rs:23:21
51    |
51    |
52 LL |             options(noreturn),
53    |                     ^^^^^^^^ this option was already provided

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/duplicate-options/duplicate-options.stderr
diff of fixed:
---
3 
4 use std::arch::asm;


The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/duplicate-options/duplicate-options.fixed
To only update this specific test, also pass `--test-args asm/aarch64/duplicate-options.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/duplicate-options.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/duplicate-options" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/duplicate-options/auxiliary"
stdout: none
--- stderr -------------------------------
error: the `nomem` option was already provided
   |
   |
LL |         asm!("", options(nomem, nomem));
   |                                 ^^^^^ this option was already provided

error: the `preserves_flags` option was already provided
   |
   |
LL |         asm!("", options(preserves_flags, preserves_flags));
   |                                           ^^^^^^^^^^^^^^^ this option was already provided

error: the `nostack` option was already provided
   |
   |
LL |         asm!("", options(nostack, preserves_flags), options(nostack));
   |                                                             ^^^^^^^ this option was already provided

error: the `nostack` option was already provided
   |
   |
LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));
   |                                   ^^^^^^^ this option was already provided

error: the `nostack` option was already provided
   |
   |
LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));
   |                                                     ^^^^^^^ this option was already provided

error: the `nostack` option was already provided
   |
   |
LL |         asm!("", options(nostack, nostack), options(nostack), options(nostack));
   |                                                                       ^^^^^^^ this option was already provided

error: the `noreturn` option was already provided
   |
   |
LL |             options(preserves_flags, noreturn), //~ ERROR the `noreturn` option was already provided
   |                                      ^^^^^^^^ this option was already provided

error: the `nomem` option was already provided
   |
   |
LL |             options(nomem, nostack),            //~ ERROR the `nomem` option was already provided
   |                     ^^^^^ this option was already provided

error: the `noreturn` option was already provided
   |
   |
LL |             options(noreturn),                  //~ ERROR the `noreturn` option was already provided
   |                     ^^^^^^^^ this option was already provided
error: aborting due to 9 previous errors
------------------------------------------


---
61 error: unrecognized instruction mnemonic
-   --> $DIR/srcloc.rs:38:14
+   --> $DIR/srcloc.rs:39:14
63    |
64 LL |         asm!(concat!("invalid", "_", "instruction"));

71    |     ^
72 
73 error: unrecognized instruction mnemonic
---
109 error: unrecognized instruction mnemonic
-   --> $DIR/srcloc.rs:62:13
+   --> $DIR/srcloc.rs:63:13
111    |
112 LL |             concat!("invalid", "_", "instruction"),

119    | ^
120 
121 error: unrecognized instruction mnemonic
121 error: unrecognized instruction mnemonic
-   --> $DIR/srcloc.rs:69:13
+   --> $DIR/srcloc.rs:70:13
123    |
124 LL |             concat!("invalid", "_", "instruction"),

131    | ^
132 
133 error: unrecognized instruction mnemonic
---
To only update this specific test, also pass `--test-args asm/aarch64/srcloc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/aarch64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/asm/aarch64/srcloc.rs" "-Zthreads=1" "--target=aarch64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/srcloc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/aarch64-unknown-linux-gnu/native/rust-test-helpers" "-Ccodegen-units=1" "-L" "/checkout/obj/build/aarch64-unknown-linux-gnu/test/ui/asm/aarch64/srcloc/auxiliary"
stdout: none
--- stderr -------------------------------
error: unrecognized instruction mnemonic
   |
LL |         asm!("invalid_instruction");
   |               ^
   |
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:16:13
   |
LL |             invalid_instruction
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:13
LL |             invalid_instruction
   |             ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:21:13
   |
LL |             invalid_instruction
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:13
LL |             invalid_instruction
   |             ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:27:13
   |
LL |             invalid_instruction
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:3:13
LL |             invalid_instruction
   |             ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:34:13
   |
LL |             invalid_instruction
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:3:13
LL |             invalid_instruction
   |             ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:39:14
   |
LL |         asm!(concat!("invalid", "_", "instruction"));
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:43:14
   |
LL |             "invalid_instruction",
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:49:14
   |
LL |             "invalid_instruction",
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:56:14
   |
LL |             "invalid_instruction",
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:3:1
LL | invalid_instruction
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:63:13
   |
LL |             concat!("invalid", "_", "instruction"),
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:70:13
   |
LL |             concat!("invalid", "_", "instruction"),
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:77:14
   |
LL |             "invalid_instruction1",
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction1
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:78:14
   |
LL |             "invalid_instruction2",
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction2
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:84:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction1
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:84:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction2
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:93:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction1
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:93:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction2
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:97:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:3:1
LL | invalid_instruction3
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:97:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:4:1
LL | invalid_instruction4
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:108:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:1:2
LL |     invalid_instruction1
   |     ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:108:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:2:1
LL | invalid_instruction2
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:112:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:4:1
LL | invalid_instruction3
   | ^

error: unrecognized instruction mnemonic
error: unrecognized instruction mnemonic
  --> /checkout/src/test/ui/asm/aarch64/srcloc.rs:112:13
   |
LL |             concat!(
   |             ^
   |
note: instantiated into assembly here
  --> <inline asm>:5:1
LL | invalid_instruction4
   | ^

error: aborting due to 23 previous errors
