plain
doc tests for: /checkout/src/doc/rustdoc/src/unstable-features.md
doc tests for: /checkout/src/doc/rustdoc/src/website-features.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustdoc/src/website-features.md" "--test-args" ""

stdout ----

running 2 tests
running 2 tests
test /checkout/src/doc/rustdoc/src/website-features.md - Website_features::Custom_search_engines (line 12) ... FAILED
test /checkout/src/doc/rustdoc/src/website-features.md - Website_features::Custom_search_engines (line 17) ... FAILED
failures:


---- /checkout/src/doc/rustdoc/src/website-features.md - Website_features::Custom_search_engines (line 12) stdout ----
error: expected expression, found `}`
 --> /checkout/src/doc/rustdoc/src/website-features.md:14:1
  |
2 | fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_12_0() {
  |                                                                                                                - while parsing this struct
3 | https://doc.rust-lang.org/stable/std/?search=%s
4 | } _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_12_0() }


error: struct literal body without path
 --> /checkout/src/doc/rustdoc/src/website-features.md:12:112
  |
2 |   fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_12_0() {
  |  ________________________________________________________________________________________________________________^
3 | | https://doc.rust-lang.org/stable/std/?search=%s
4 | | } _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_12_0() }
  |
  |
help: you might have forgotten to add the struct literal inside the block
  |
2 | fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_12_0() { SomeStruct {
3 | https://doc.rust-lang.org/stable/std/?search=%s
4 | } } _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_12_0() }

error: aborting due to 2 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/rustdoc/src/website-features.md - Website_features::Custom_search_engines (line 17) stdout ----
error: expected expression, found `}`
 --> /checkout/src/doc/rustdoc/src/website-features.md:19:1
  |
2 | fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_17_0() {
  |                                                                                                                - while parsing this struct
3 | https://doc.rust-lang.org/stable/std/?search=%s&go_to_first=true
4 | } _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_17_0() }


error: struct literal body without path
 --> /checkout/src/doc/rustdoc/src/website-features.md:17:112
  |
2 |   fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_17_0() {
  |  ________________________________________________________________________________________________________________^
3 | | https://doc.rust-lang.org/stable/std/?search=%s&go_to_first=true
4 | | } _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_17_0() }
  |
  |
help: you might have forgotten to add the struct literal inside the block
  |
2 | fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_17_0() { SomeStruct {
3 | https://doc.rust-lang.org/stable/std/?search=%s&go_to_first=true
4 | } } _doctest_main__checkout_src_doc_rustdoc_src_website_features_md_17_0() }

error: aborting due to 2 previous errors

Couldn't compile the test.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustdoc/src/website-features.md - Website_features::Custom_search_engines (line 12)
    /checkout/src/doc/rustdoc/src/website-features.md - Website_features::Custom_search_engines (line 17)
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s


stderr ----
