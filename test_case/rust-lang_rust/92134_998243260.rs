plain
doc tests for: /checkout/src/doc/rustc/src/json.md
doc tests for: /checkout/src/doc/rustc/src/linker-plugin-lto.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustc/src/linker-plugin-lto.md" "--test-args" ""

stdout ----

running 1 test
running 1 test
test /checkout/src/doc/rustc/src/linker-plugin-lto.md - Linker_plugin_LTO::Usage::Usage_with_clang_cl_and___target_x86_64_pc_windows_msvc (line 107) ... FAILED
failures:


---- /checkout/src/doc/rustc/src/linker-plugin-lto.md - Linker_plugin_LTO::Usage::Usage_with_clang_cl_and___target_x86_64_pc_windows_msvc (line 107) stdout ----
 --> /checkout/src/doc/rustc/src/linker-plugin-lto.md:108:63
  |
  |
3 | error: Linker plugin based LTO is not supported together with `-C prefer-dynamic` when
  |
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
3 | error: Linker plugin based LTO is not supported together with '-C prefer-dynamic` when

error: unknown start of token: `
 --> /checkout/src/doc/rustc/src/linker-plugin-lto.md:108:81
  |
  |
3 | error: Linker plugin based LTO is not supported together with `-C prefer-dynamic` when
  |
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
3 | error: Linker plugin based LTO is not supported together with `-C prefer-dynamic' when


error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `plugin`
 --> /checkout/src/doc/rustc/src/linker-plugin-lto.md:108:15
  |
3 | error: Linker plugin based LTO is not supported together with `-C prefer-dynamic` when
  |      -        ^^^^^^ expected one of 8 possible tokens
  |      |
  |      tried to parse a type due to this type ascription
  |
  = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: aborting due to 3 previous errors

Couldn't compile the test.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustc/src/linker-plugin-lto.md - Linker_plugin_LTO::Usage::Usage_with_clang_cl_and___target_x86_64_pc_windows_msvc (line 107)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


stderr ----
