plain
 finished in 9.635 seconds
doc tests for: /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md" "--test-args" ""

stdout ----

running 15 tests
running 15 tests
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Vendor_specific_suffix (line 1015) ... FAILED
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Lifetime (line 609) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Backref (line 931) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Symbol_path::Path__Nested_path (line 384) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Symbol_path::Path__Impl (line 286) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Symbol_path::Path__Crate_root (line 180) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Symbol_path::Path__Trait_impl (line 247) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Instantiating_crate (line 976) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Symbol_path::Path__Inherent_impl (line 213) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Symbol_path::Path__Trait_definition (line 332) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Symbol_path::Path__Generic_arguments (line 440) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Const (line 676) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Identifier::Punycode_identifiers (line 526) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Type (line 862) ... ok
test /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Symbol_name (line 114) ... ok
failures:


---- /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Vendor_specific_suffix (line 1015) stdout ----
error[E0412]: cannot find type `RefCell` in this scope
 --> /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md:1017:25
  |
4 |     pub static EXAMPLE: RefCell<u32> = RefCell::new(1);
  |
help: consider importing this struct
  |
2 | use std::cell::RefCell;
2 | use std::cell::RefCell;
  |

error[E0433]: failed to resolve: use of undeclared type `RefCell`
 --> /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md:1017:40
  |
4 |     pub static EXAMPLE: RefCell<u32> = RefCell::new(1);
  |
help: consider importing this struct
  |
2 | use std::cell::RefCell;
---
For more information about an error, try `rustc --explain E0412`.
Couldn't compile the test.

failures:
    /checkout/src/doc/rustc/src/codegen-options/symbol-mangling.md - Symbol_Mangling::v0_mangling_format::Vendor_specific_suffix (line 1015)
test result: FAILED. 14 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s


stderr ----
