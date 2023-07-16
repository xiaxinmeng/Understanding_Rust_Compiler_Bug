plain
doc tests for: /checkout/src/doc/unstable-book/src/compiler-flags/emit-stack-sizes.md
doc tests for: /checkout/src/doc/unstable-book/src/compiler-flags/export-executable-symbols.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/compiler-flags/export-executable-symbols.md" "--test-args" ""

stdout ----

running 2 tests
running 2 tests
test /checkout/src/doc/unstable-book/src/compiler-flags/export-executable-symbols.md - The_tracking_issue_for_this_feature_is__ (line 30) ... FAILED
test /checkout/src/doc/unstable-book/src/compiler-flags/export-executable-symbols.md - The_tracking_issue_for_this_feature_is__ (line 10) ... ok

failures:

---- /checkout/src/doc/unstable-book/src/compiler-flags/export-executable-symbols.md - The_tracking_issue_for_this_feature_is__ (line 30) stdout ----
error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `Export`
  |
  |
3 | The Export Tables (interpreted .edata section contents)
  |     ^^^^^^ expected one of 8 possible tokens
error: aborting due to previous error

Couldn't compile the test.

