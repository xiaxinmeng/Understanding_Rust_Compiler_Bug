plain
doc tests for: /checkout/src/doc/rustdoc/src/lints.md
doc tests for: /checkout/src/doc/rustdoc/src/scraped-examples.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustdoc/src/scraped-examples.md" "--test-args" ""

stdout ----

running 2 tests
running 2 tests
test /checkout/src/doc/rustdoc/src/scraped-examples.md - Scraped_examples (line 12) ... FAILED
test /checkout/src/doc/rustdoc/src/scraped-examples.md - Scraped_examples (line 5) ... ok
failures:


---- /checkout/src/doc/rustdoc/src/scraped-examples.md - Scraped_examples (line 12) stdout ----
error[E0433]: failed to resolve: use of undeclared crate or module `a_crate`
 --> /checkout/src/doc/rustdoc/src/scraped-examples.md:15:3
  |
4 |   a_crate::a_func();
  |   ^^^^^^^ use of undeclared crate or module `a_crate`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustdoc/src/scraped-examples.md - Scraped_examples (line 12)
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s


stderr ----
