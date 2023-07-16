plain
.................................................................................................... 9400/11745
.................................................................................................... 9500/11745
..................................................................................i......i.......... 9600/11745
.................................................................................................... 9700/11745
............................iiiiiii..iiiiii.i....................................................... 9800/11745
.................................................................................................... 10000/11745
.................................................................................................... 10100/11745
.................................................................................................... 10200/11745
.................................................................................................... 10300/11745
---
 finished in 0.407 seconds
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 33 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii..

 finished in 0.116 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
 finished in 10.655 seconds
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.30s

 finished in 2.363 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
doc tests for: /checkout/src/doc/unstable-book/src/language-features/unsized-tuple-coercion.md
doc tests for: /checkout/src/doc/unstable-book/src/library-features/asm.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "--test" "/checkout/src/doc/unstable-book/src/library-features/asm.md" "--test-args" ""

stdout ----

running 16 tests
---
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
