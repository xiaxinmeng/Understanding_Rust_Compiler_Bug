`

failures:

---- /checkout/src/doc/unstable-book/src/compiler-flags/sanitizer.md - ControlFlowIntegrity::Example (line 216) stdout ----
error: register %rax is only available in 64-bit mode
  --> /checkout/src/doc/unstable-book/src/compiler-flags/sanitizer.md:241:14
   |
26 |              lea rax, [rdi+2]
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:12:18
   |
12 |              lea rax, [rdi+2]
   |                  ^^^

error: register %rdi is only available in 64-bit mode
  --> /checkout/src/doc/unstable-book/src/compiler-flags/sanitizer.md:241:14
   |
26 |              lea rax, [rdi+2]
   |              ^
   |
note: instantiated into assembly here
  --> <inline asm>:12:24
   |
12 |              lea rax, [rdi+2]
   |                        ^^^

error: aborting due to 2 previous errors

Couldn't compile the test.

failures:
    /checkout/src/doc/unstable-book/src/compiler-flags/sanitizer.md - 
