plain
   Compiling once_cell v1.7.2
   Compiling difference v2.0.0
   Compiling expect-test v1.0.1
   Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0599]: no method named `as_slice` found for struct `url_parts_builder::UrlPartsBuilder` in the current scope
  --> src/librustdoc/html/tests.rs:7:70
   |
7  |     assert_eq!(expected, href_relative_parts(&fqp, &relative_to_fqp).as_slice());
   |                                                                      ^^^^^^^^ method not found in `url_parts_builder::UrlPartsBuilder`
  ::: src/librustdoc/html/url_parts_builder.rs:11:1
   |
   |
11 | crate struct UrlPartsBuilder {
   | ---------------------------- method `as_slice` not found for this
For more information about this error, try `rustc --explain E0599`.
error: could not compile `rustdoc` due to previous error



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "-p" "rustdoc:0.0.0" "--" "--quiet"


Build completed unsuccessfully in 0:34:48
