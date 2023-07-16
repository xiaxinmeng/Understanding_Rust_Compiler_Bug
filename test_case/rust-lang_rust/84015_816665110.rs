plain
.................................................................................................... 9400/11747
.................................................................................................... 9500/11747
...................................................................................i......i......... 9600/11747
.................................................................................................... 9700/11747
.............................iiiiiii..iiiiii.i...................................................... 9800/11747
.................................................................................................... 10000/11747
.................................................................................................... 10100/11747
.................................................................................................... 10200/11747
.................................................................................................... 10300/11747
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.105 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii...... 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.36s

 finished in 2.436 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
doc tests for: /checkout/src/doc/unstable-book/src/language-features/unsized-tuple-coercion.md
doc tests for: /checkout/src/doc/unstable-book/src/library-features/asm.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/src/doc/unstable-book/src/library-features/asm.md" "--test-args" ""

stdout ----

running 16 tests
---
---- /checkout/src/doc/unstable-book/src/library-features/asm.md - Guide_level_explanation::Register_template_modifiers (line 338) stdout ----
error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:343:11
  |
7 |     asm!("mov {0:h}, {0:l}", inout(reg_abcd) x);
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:2:6
  |
2 |     mov %ah, %al

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/library-features/asm.md - Guide_level_explanation::Inputs_and_outputs (line 130) stdout ----
error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:135:11
  |
7 |     asm!("add {0}, {number}", inout(reg) x => y, number = const 5);
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:2:6
  |
2 |     add %rax, 5

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/library-features/asm.md - Guide_level_explanation::Options (line 418) stdout ----
error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:424:10
  |
8 |         "add {0}, {1}",
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:2:6
  |
2 |     add %rax, %rcx

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/library-features/asm.md - Guide_level_explanation::Inputs_and_outputs (line 54) stdout ----
error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:58:11
  |
6 |     asm!("mov {}, 5", out(reg) x);
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:2:6
  |
2 |     mov %rax, 5

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/library-features/asm.md - Guide_level_explanation::Late_output_operands (line 173) stdout ----
error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:178:11
  |
7 |     asm!("add {0}, {1}", inlateout(reg) a, in(reg) b);
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:2:6
  |
2 |     add %rax, %rcx

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/library-features/asm.md - Guide_level_explanation::Late_output_operands (line 152) stdout ----
error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:159:10
  |
9 |         "add {0}, {1}",
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:2:6
  |
2 |     add %rax, %rdx

error: unknown token in expression
  --> /checkout/src/doc/unstable-book/src/library-features/asm.md:160:10
   |
   |
10 |         "add {0}, {2}",
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:3:5
   |
3  | add %rax, %rcx

error: aborting due to 2 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/library-features/asm.md - Guide_level_explanation::Clobbered_registers (line 274) stdout ----
error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:280:10
  |
8 |         "mov {tmp}, {x}",
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:2:6
  |
2 |     mov %rcx, %rax

error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:281:10
  |
  |
9 |         "shl {tmp}, 1",
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:3:5
  |
3 | shl %rcx, 1

error: unknown token in expression
  --> /checkout/src/doc/unstable-book/src/library-features/asm.md:282:10
   |
   |
10 |         "shl {x}, 2",
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:4:5
   |
4  | shl %rax, 2

error: unknown token in expression
  --> /checkout/src/doc/unstable-book/src/library-features/asm.md:283:10
   |
   |
11 |         "add {x}, {tmp}",
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:5:5
   |
5  | add %rax, %rcx

error: aborting due to 4 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/library-features/asm.md - Guide_level_explanation::Inputs_and_outputs (line 76) stdout ----
error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:82:10
  |
8 |         "mov {0}, {1}",
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:2:6
  |
2 |     mov %rcx, %rax

error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:83:10
  |
  |
9 |         "add {0}, {number}",
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:3:5
  |
3 | add %rcx, 5

error: aborting due to 2 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/library-features/asm.md - Guide_level_explanation::Inputs_and_outputs (line 116) stdout ----
error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:120:11
  |
6 |     asm!("add {0}, {number}", inout(reg) x, number = const 5);
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:2:6
  |
2 |     add %rax, 5

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/library-features/asm.md - Guide_level_explanation::Labels (line 381) stdout ----
warning: formatting may not be suitable for sub-register argument
  --> /checkout/src/doc/unstable-book/src/library-features/asm.md:387:14
   |
8  |         "mov {0}, 10",
9  |         "2:",
9  |         "2:",
10 |         "sub {0}, 1",
   |              ^^^
11 |         "cmp {0}, 3",
...
...
15 |         "add {0}, 2",
16 |         out(reg) a
   |                  - for this argument
   |
   |
   = note: `#[warn(asm_sub_register)]` on by default
   = help: use the `e` modifier to have the register formatted as `eax`
   = help: or use the `r` modifier to keep the default formatting of `rax`
error: unknown token in expression
 --> /checkout/src/doc/unstable-book/src/library-features/asm.md:387:10
  |
  |
8 |         "mov {0}, 10",
  |
note: instantiated into assembly here
note: instantiated into assembly here
 --> <inline asm>:2:6
  |
2 |     mov %rax, 10

error: unknown token in expression
  --> /checkout/src/doc/unstable-book/src/library-features/asm.md:389:10
   |
   |
10 |         "sub {0}, 1",
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:4:5
   |
4  | sub %rax, 1

error: unknown token in expression
  --> /checkout/src/doc/unstable-book/src/library-features/asm.md:390:10
   |
   |
11 |         "cmp {0}, 3",
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:5:5
   |
5  | cmp %rax, 3

error: unknown token in expression
  --> /checkout/src/doc/unstable-book/src/library-features/asm.md:394:10
   |
   |
15 |         "add {0}, 2",
   |
note: instantiated into assembly here
note: instantiated into assembly here
  --> <inline asm>:9:5
   |
9  | add %rax, 2

error: aborting due to 4 previous errors; 1 warning emitted

Couldn't compile the test.
