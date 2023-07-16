plain
doc tests for: /checkout/src/doc/unstable-book/src/language-features/doc-notable-trait.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md" "--test-args" ""

stdout ----

running 3 tests
---
---- /checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md - The_tracking_issue_for_this_feature_is__ (line 24) stdout ----
error: unknown start of token: `
 --> /checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md:25:62
  |
3 | error[E0632]: cannot provide explicit generic arguments when `impl Trait` is used in argument position
  |
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
3 | error[E0632]: cannot provide explicit generic arguments when 'impl Trait` is used in argument position

error: unknown start of token: `
 --> /checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md:25:73
  |
  |
3 | error[E0632]: cannot provide explicit generic arguments when `impl Trait` is used in argument position
  |
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
3 | error[E0632]: cannot provide explicit generic arguments when `impl Trait' is used in argument position


error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `provide`
 --> /checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md:25:22
  |
3 | error[E0632]: cannot provide explicit generic arguments when `impl Trait` is used in argument position
  |             -        ^^^^^^^ expected one of 8 possible tokens
  |             |
  |             tried to parse a type due to this type ascription
  |
  = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: aborting due to 3 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md - The_tracking_issue_for_this_feature_is__ (line 14) stdout ----
error[E0632]: cannot provide explicit generic arguments when `impl Trait` is used in argument position
 --> /checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md:18:11
  |
5 |     foo::<str>("".to_string());
  |           ^^^ explicit generic argument not allowed
error: aborting due to previous error

Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md - The_tracking_issue_for_this_feature_is__ (line 38) stdout ----
---- /checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md - The_tracking_issue_for_this_feature_is__ (line 38) stdout ----
error[E0107]: this function takes at most 1 generic argument but 2 generic arguments were supplied
  --> /checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md:49:5
   |
12 |     foo::<str, String>("".to_string()); // Error, you cannot specify `impl Trait` explicitly
   |     ^^^        ------ help: remove this generic argument
   |     expected at most 1 generic argument
   |
   |
note: function defined here, with at most 1 generic parameter: `T`
  --> /checkout/src/doc/unstable-book/src/language-features/explicit-generic-args-with-impl-trait.md:41:4
   |
4  | fn foo<T: ?Sized>(_f: impl AsRef<T>) {}

error: aborting due to previous error

For more information about this error, try `rustc --explain E0107`.
