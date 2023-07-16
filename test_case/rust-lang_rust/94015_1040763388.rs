plain
doc tests for: /checkout/src/doc/rustdoc/src/advanced-features.md
doc tests for: /checkout/src/doc/rustdoc/src/command-line-arguments.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustdoc/src/command-line-arguments.md" "--test-args" ""

stdout ----

running 2 tests
running 2 tests
test /checkout/src/doc/rustdoc/src/command-line-arguments.md - Unstable_command_line_arguments::When_this_flag_is_supplied__rustdoc_will_type_check_and_lint_your_code__but_will_not_generate_any (line 437) ... FAILED

failures:


---- /checkout/src/doc/rustdoc/src/command-line-arguments.md - Unstable_command_line_arguments::When_this_flag_is_supplied__rustdoc_will_type_check_and_lint_your_code__but_will_not_generate_any (line 437) stdout ----
error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `unstable`
  |
  |
3 | rustdoc -Z unstable-options --check src/lib.rs
  |            ^^^^^^^^ expected one of 8 possible tokens
error: aborting due to previous error

Couldn't compile the test.


failures:
    /checkout/src/doc/rustdoc/src/command-line-arguments.md - Unstable_command_line_arguments::When_this_flag_is_supplied__rustdoc_will_type_check_and_lint_your_code__but_will_not_generate_any (line 437)
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.16s


stderr ----
