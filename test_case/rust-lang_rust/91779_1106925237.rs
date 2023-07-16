plain
doc tests for: /checkout/src/doc/unstable-book/src/language-features/custom-test-frameworks.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/debugger-visualizer.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/language-features/debugger-visualizer.md" "--test-args" ""

stdout ----

running 1 test
running 1 test
test /checkout/src/doc/unstable-book/src/language-features/debugger-visualizer.md - The_tracking_issue_for_this_feature_is__::Examples (line 14) ... FAILED

failures:

---- /checkout/src/doc/unstable-book/src/language-features/debugger-visualizer.md - The_tracking_issue_for_this_feature_is__::Examples (line 14) stdout ----
error: /checkout/src/doc/unstable-book/src/language-features/foo.natvis wasn't a valid file
 --> /checkout/src/doc/unstable-book/src/language-features/debugger-visualizer.md:15:1
  |
3 | #![debugger_visualizer(natvis_file = "foo.natvis")]

error: aborting due to previous error

Couldn't compile the test.
