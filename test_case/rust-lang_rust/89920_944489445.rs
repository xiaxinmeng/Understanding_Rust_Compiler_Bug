plain
doc tests for: /checkout/src/doc/unstable-book/src/compiler-flags/control-flow-guard.md
doc tests for: /checkout/src/doc/unstable-book/src/compiler-flags/debug_info_for_profiling.md
doc tests for: /checkout/src/doc/unstable-book/src/compiler-flags/emit-stack-sizes.md
doc tests for: /checkout/src/doc/unstable-book/src/compiler-flags/instrument-coverage.md
doc tests for: /checkout/src/doc/unstable-book/src/compiler-flags/location-detail.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/compiler-flags/location-detail.md" "--test-args" ""

stdout ----

running 1 test
running 1 test
test /checkout/src/doc/unstable-book/src/compiler-flags/location-detail.md - The_tracking_issue_for_this_feature_is__ (line 24) ... FAILED
failures:


---- /checkout/src/doc/unstable-book/src/compiler-flags/location-detail.md - The_tracking_issue_for_this_feature_is__ (line 24) stdout ----
error[E0762]: unterminated character literal
 --> /checkout/src/doc/unstable-book/src/compiler-flags/location-detail.md:25:39
  |
1 | panicked at 'Process blink had a fault', <redacted>:323:0

error: aborting due to previous error

For more information about this error, try `rustc --explain E0762`.
For more information about this error, try `rustc --explain E0762`.
Couldn't compile the test.

failures:
    /checkout/src/doc/unstable-book/src/compiler-flags/location-detail.md - The_tracking_issue_for_this_feature_is__ (line 24)
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


stderr ----
