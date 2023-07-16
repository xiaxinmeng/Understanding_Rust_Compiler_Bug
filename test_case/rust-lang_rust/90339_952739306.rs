plain
doc tests for: /checkout/src/doc/rustdoc/src/lints.md
doc tests for: /checkout/src/doc/rustdoc/src/the-doc-attribute.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustdoc/src/the-doc-attribute.md" "--test-args" ""

stdout ----

running 18 tests
---
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_crate_level::This_form_of_the_ (line 72) - compile ... ok
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_ (line 40) - compile ... ok
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_crate_level::By_default__ (line 125) - compile ... ok
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_crate_level::This_form_of_the_ (line 101) - compile ... ok
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_item_level::This_attribute_adds_an_alias_in_the_search_index_ (line 233) - compile ... ok
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_crate_level::By_default__ (line 136) - compile ... ok
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_crate_level::This_form_of_the_ (line 86) - compile ... ok
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_crate_level::This_form_of_the_ (line 86) - compile ... ok
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_item_level::This_attribute_adds_an_alias_in_the_search_index_ (line 254) - compile ... FAILED
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_item_level::_ (line 177) - compile ... ok
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_item_level::_ (line 161) - compile ... ok
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_item_level::_ (line 189) - compile ... ok
test /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_item_level::_ (line 205) - compile ... ok
failures:


---- /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_item_level::This_attribute_adds_an_alias_in_the_search_index_ (line 254) stdout ----
error[E0433]: failed to resolve: use of undeclared crate or module `ffi`
 --> /checkout/src/doc/rustdoc/src/the-doc-attribute.md:256:17
4 |     inner: *mut ffi::Obj,
  |                 ^^^ use of undeclared crate or module `ffi`

error[E0433]: failed to resolve: use of undeclared crate or module `ffi`
error[E0433]: failed to resolve: use of undeclared crate or module `ffi`
 --> /checkout/src/doc/rustdoc/src/the-doc-attribute.md:261:18
  |
9 |         unsafe { ffi::lib_name_do_something(self.inner) }
  |                  ^^^ use of undeclared crate or module `ffi`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0433`.
Couldn't compile the test.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustdoc/src/the-doc-attribute.md - The_::At_the_item_level::This_attribute_adds_an_alias_in_the_search_index_ (line 254)
test result: FAILED. 17 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s


stderr ----
