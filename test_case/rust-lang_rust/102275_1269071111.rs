plain
doc tests for: /checkout/src/doc/unstable-book/src/language-features/generators.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/half-open-range-patterns-in-slices.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/language-features/half-open-range-patterns-in-slices.md" "--test-args" ""

stdout ----

running 1 test
running 1 test
test /checkout/src/doc/unstable-book/src/language-features/half-open-range-patterns-in-slices.md - The_tracking_issue_for_this_feature_is__ (line 13) ... FAILED

failures:

---- /checkout/src/doc/unstable-book/src/language-features/half-open-range-patterns-in-slices.md - The_tracking_issue_for_this_feature_is__ (line 13) stdout ----
error[E0005]: refutable pattern in local binding: `[i32::MIN..=2_i32, ..]` not covered
  |
  |
7 |     let [a @ 3.., b @ ..3, c @ 4..6, ..] = xs;
  |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `[i32::MIN..=2_i32, ..]` not covered
  |
  = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
  = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
  = note: the matched value is of type `[i32; 8]`
help: you might want to use `if let` to ignore the variant that isn't matched
  |
7 |     let (a, b, c) = if let [a @ 3.., b @ ..3, c @ 4..6, ..] = xs { (a, b, c) } else { todo!() };
  |     ++++++++++++++++++                                           ++++++++++++++++++++++++++++++
help: alternatively, you might want to use let else to handle the variant that isn't matched
  |
7 |     let [a @ 3.., b @ ..3, c @ 4..6, ..] = xs else { todo!() };

error: aborting due to previous error

For more information about this error, try `rustc --explain E0005`.
