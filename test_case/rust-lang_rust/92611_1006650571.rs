plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f1ce0e6a00593493a12e0e3662119786c761f375 and e0ee154c1bc086846a49a4f8f4d06ca4d1849ea2
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
    
    --- stdout
    
    running 3 tests
    test /tmp/mdbook-dFsDva/fn/closures/closure_examples/iter_find.md - Searching_through_iterators (line 7) ... ignored
    test /tmp/mdbook-dFsDva/fn/closures/closure_examples/iter_find.md - Searching_through_iterators (line 22) ... FAILED
    test /tmp/mdbook-dFsDva/fn/closures/closure_examples/iter_find.md - Searching_through_iterators (line 52) ... ok
    failures:
    
    
    ---- /tmp/mdbook-dFsDva/fn/closures/closure_examples/iter_find.md - Searching_through_iterators (line 22) stdout ----
    error[E0277]: can't compare `&{integer}` with `{integer}`
      --> /tmp/mdbook-dFsDva/fn/closures/closure_examples/iter_find.md:45:71
       |
    24 |     println!("Find 2 in array2: {:?}", array2.into_iter().find(|&x| x == 2));
       |                                                                       ^^ no implementation for `&{integer} == {integer}`
       |
       = help: the trait `PartialEq<{integer}>` is not implemented for `&{integer}`
    error: aborting due to previous error
    
    For more information about this error, try `rustc --explain E0277`.
    Couldn't compile the test.
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-dFsDva/fn/closures/closure_examples/iter_find.md - Searching_through_iterators (line 22)
    test result: FAILED. 1 passed; 1 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.34s
    
    
    --- stderr
    --- stderr
    
[2022-01-06T14:42:00Z ERROR mdbook::book] rustdoc returned an error:
    
    --- stdout
    
    running 16 tests
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Explicit_register_operands (line 162) - compile ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Memory_address_operands (line 325) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Basic_usage (line 20) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Register_template_modifiers (line 302) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 57) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 36) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Late_output_operands (line 125) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 104) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Symbol_operands_and_ABI_clobbers (line 268) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 91) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Clobbered_registers (line 248) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Late_output_operands (line 145) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Explicit_register_operands (line 175) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Options (line 383) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Labels (line 345) ... FAILED
    test /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Clobbered_registers (line 208) ... FAILED
    failures:
    
    
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Explicit_register_operands (line 162) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:165:5
      |
    5 |     asm!("out 0x64, eax", in("eax") cmd);
      |
      = note: consider importing this macro:
              std::arch::asm
    
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Memory_address_operands (line 325) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:328:9
      |
    5 |         asm!("fldcw [{}]", in(reg) &control, options(nostack));
      |
      = note: consider importing this macro:
              std::arch::asm
    
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Basic_usage (line 20) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:22:5
      |
    4 |     asm!("nop");
      |
      = note: consider importing this macro:
              std::arch::asm
    
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Register_template_modifiers (line 302) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:306:5
      |
    6 |     asm!("mov {0:h}, {0:l}", inout(reg_abcd) x);
      |
      = note: consider importing this macro:
              std::arch::asm
    
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 57) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:61:5
    6 |     asm!(
      |     ^^^
      |
      = note: consider importing this macro:
      = note: consider importing this macro:
              std::arch::asm
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 36) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:39:5
      |
    5 |     asm!("mov {}, 5", out(reg) x);
      |
      = note: consider importing this macro:
              std::arch::asm
    
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Late_output_operands (line 125) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:130:5
    7 |     asm!(
      |     ^^^
      |
      = note: consider importing this macro:
      = note: consider importing this macro:
              std::arch::asm
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 104) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:108:5
      |
    6 |     asm!("add {0}, 5", inout(reg) x => y);
      |
      = note: consider importing this macro:
              std::arch::asm
    
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Symbol_operands_and_ABI_clobbers (line 268) stdout ----
    error: cannot find macro `asm` in this scope
      --> /tmp/mdbook-dFsDva/unsafe/asm.md:277:9
    11 |         asm!(
       |         ^^^
       |
       = note: consider importing this macro:
       = note: consider importing this macro:
               std::arch::asm
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 91) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:94:5
      |
    5 |     asm!("add {0}, 5", inout(reg) x);
      |
      = note: consider importing this macro:
              std::arch::asm
    
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Clobbered_registers (line 248) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:252:5
    6 |     asm!(
      |     ^^^
      |
      = note: consider importing this macro:
      = note: consider importing this macro:
              std::arch::asm
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Late_output_operands (line 145) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:149:5
      |
    6 |     asm!("add {0}, {1}", inlateout(reg) a, in(reg) b);
      |
      = note: consider importing this macro:
              std::arch::asm
    
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Explicit_register_operands (line 175) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:181:9
    8 |         asm!(
      |         ^^^
      |
      = note: consider importing this macro:
      = note: consider importing this macro:
              std::arch::asm
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Options (line 383) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:387:5
    6 |     asm!(
      |     ^^^
      |
      = note: consider importing this macro:
      = note: consider importing this macro:
              std::arch::asm
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Labels (line 345) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:348:5
    5 |     asm!(
      |     ^^^
      |
      = note: consider importing this macro:
      = note: consider importing this macro:
              std::arch::asm
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    ---- /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Clobbered_registers (line 208) stdout ----
    error: cannot find macro `asm` in this scope
     --> /tmp/mdbook-dFsDva/unsafe/asm.md:213:5
    7 |     asm!(
      |     ^^^
      |
      = note: consider importing this macro:
      = note: consider importing this macro:
              std::arch::asm
    
    error: aborting due to previous error
    
    Couldn't compile the test.
    
    failures:
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Basic_usage (line 20)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Clobbered_registers (line 208)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Clobbered_registers (line 248)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Explicit_register_operands (line 162)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Explicit_register_operands (line 175)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 104)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 36)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 57)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Inputs_and_outputs (line 91)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Labels (line 345)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Late_output_operands (line 125)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Late_output_operands (line 145)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Memory_address_operands (line 325)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Options (line 383)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Register_template_modifiers (line 302)
        /tmp/mdbook-dFsDva/unsafe/asm.md - Inline_assembly::Symbol_operands_and_ABI_clobbers (line 268)
    test result: FAILED. 0 passed; 16 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s
    
    
    --- stderr
---
This PR updated 'src/doc/rust-by-example', verifying if status is 'test-pass'...

We detected that this PR updated 'rust-by-example', but its tests failed.

If you do intend to update 'rust-by-example', please check the error messages above and
commit another update.

If you do NOT intend to update 'rust-by-example', please ensure you did not accidentally
change the submodule at 'src/doc/rust-by-example'. You may ask your reviewer for the
proper steps.
