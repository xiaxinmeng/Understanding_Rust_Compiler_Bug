plain
Testing error-index stage2
doc tests for: /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md" "--test-args" ""

stdout ----

running 1035 tests
---
  |
2 | #![feature(in_band_lifetimes)]
  |            ^^^^^^^^^^^^^^^^^ feature has been removed
  |
  = note: removed due to outstanding ergonomic questions and added lifetime resolution complexity
error[E0261]: use of undeclared lifetime name `'b`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13945:28
  |
  |
5 | fn foo<'a>(x: &'a u32, y: &'b u32) {}   // error!
  |        -                   ^^ undeclared lifetime
  |        |
  |        help: consider introducing lifetime `'b` here: `'b,`
error[E0261]: use of undeclared lifetime name `'a`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13949:10
  |
9 | impl Foo<'a> {
9 | impl Foo<'a> {
  |     -    ^^ undeclared lifetime
  |     |
  |     help: consider introducing lifetime `'a` here: `<'a>`
error[E0261]: use of undeclared lifetime name `'a`
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13950:20
   |
   |
10 |     fn bar<'b>(x: &'a u32, y: &'b u32, z: &'c u32) {}   // error!
   |                    ^^ undeclared lifetime
   |
help: consider introducing lifetime `'a` here
   |
9  | impl<'a> Foo<'a> {
   |     ++++
help: consider introducing lifetime `'a` here
   |
10 |     fn bar<'a, 'b>(x: &'a u32, y: &'b u32, z: &'c u32) {}   // error!

error[E0261]: use of undeclared lifetime name `'c`
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13950:44
   |
   |
10 |     fn bar<'b>(x: &'a u32, y: &'b u32, z: &'c u32) {}   // error!
   |                                            ^^ undeclared lifetime
   |
help: consider introducing lifetime `'c` here
   |
9  | impl<'c> Foo<'a> {
   |     ++++
help: consider introducing lifetime `'c` here
   |
10 |     fn bar<'c, 'b>(x: &'a u32, y: &'b u32, z: &'c u32) {}   // error!

error[E0261]: use of undeclared lifetime name `'a`
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13953:14
   |
   |
13 | impl<'b> Foo<'a> {  // error!
   |      -       ^^ undeclared lifetime
   |      |
   |      help: consider introducing lifetime `'a` here: `'a,`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0261, E0557.
For more information about an error, try `rustc --explain E0261`.
For more information about an error, try `rustc --explain E0261`.
Some expected error codes were not found: ["E0688"]
---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0687::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 13902) stdout ----
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13902:12
  |
2 | #![feature(in_band_lifetimes)]
  |            ^^^^^^^^^^^^^^^^^ feature has been removed
  |            ^^^^^^^^^^^^^^^^^ feature has been removed
  |
  = note: removed due to outstanding ergonomic questions and added lifetime resolution complexity
error[E0261]: use of undeclared lifetime name `'a`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13905:15
  |
  |
5 | fn foo(x: fn(&'a u32)) {} // error!
  |               ^^ undeclared lifetime
  |
  = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
help: consider introducing lifetime `'a` here
  |
5 | fn foo<'a>(x: fn(&'a u32)) {} // error!
  |       ++++
help: consider making the type lifetime-generic with a new `'a` lifetime
  |
5 | fn foo(x: for<'a> fn(&'a u32)) {} // error!

error[E0261]: use of undeclared lifetime name `'a`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13907:16
  |
  |
7 | fn bar(x: &Fn(&'a u32)) {} // error!
  |                ^^ undeclared lifetime
  |
  = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
help: consider introducing lifetime `'a` here
  |
7 | fn bar<'a>(x: &Fn(&'a u32)) {} // error!
  |       ++++
help: consider making the bound lifetime-generic with a new `'a` lifetime
  |
7 | fn bar(x: &for<'a> Fn(&'a u32)) {} // error!

warning: trait objects without an explicit `dyn` are deprecated
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13907:12
  |
  |
7 | fn bar(x: &Fn(&'a u32)) {} // error!
  |
  = note: `#[warn(bare_trait_objects)]` on by default
  = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
  = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
  = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
  |
7 - fn bar(x: &Fn(&'a u32)) {} // error!
7 + fn bar(x: &dyn Fn(&'a u32)) {} // error!

error[E0261]: use of undeclared lifetime name `'a`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13909:15
  |
  |
9 | fn baz(x: fn(&'a u32), y: &'a u32) {} // error!
  |               ^^ undeclared lifetime
  |
  = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
help: consider introducing lifetime `'a` here
  |
9 | fn baz<'a>(x: fn(&'a u32), y: &'a u32) {} // error!
  |       ++++
help: consider making the type lifetime-generic with a new `'a` lifetime
  |
9 | fn baz(x: for<'a> fn(&'a u32), y: &'a u32) {} // error!

error[E0261]: use of undeclared lifetime name `'a`
 --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13909:28
  |
  |
9 | fn baz(x: fn(&'a u32), y: &'a u32) {} // error!
  |       -                    ^^ undeclared lifetime
  |       |
  |       help: consider introducing lifetime `'a` here: `<'a>`
error[E0261]: use of undeclared lifetime name `'a`
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13913:10
   |
13 | impl Foo<'a> {
13 | impl Foo<'a> {
   |     -    ^^ undeclared lifetime
   |     |
   |     help: consider introducing lifetime `'a` here: `<'a>`
error[E0261]: use of undeclared lifetime name `'a`
  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:13914:26
   |
   |
14 |     fn bar(&self, x: fn(&'a u32)) {} // error!
   |                          ^^ undeclared lifetime
   |
   = note: for more information on higher-ranked polymorphism, visit https://doc.rust-lang.org/nomicon/hrtb.html
help: consider introducing lifetime `'a` here
   |
13 | impl<'a> Foo<'a> {
   |     ++++
help: consider introducing lifetime `'a` here
   |
14 |     fn bar<'a>(&self, x: fn(&'a u32)) {} // error!
   |           ++++
help: consider making the type lifetime-generic with a new `'a` lifetime
   |
14 |     fn bar(&self, x: for<'a> fn(&'a u32)) {} // error!

error: aborting due to 7 previous errors; 1 warning emitted

Some errors have detailed explanations: E0261, E0557.
Some errors have detailed explanations: E0261, E0557.
For more information about an error, try `rustc --explain E0261`.
Some expected error codes were not found: ["E0687"]
failures:
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0687::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 13902)
    /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0688::_::Note__this_error_code_is_no_longer_emitted_by_the_compiler_ (line 13942)

