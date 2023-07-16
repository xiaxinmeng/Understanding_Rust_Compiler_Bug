
   | |_______^
   |
   |
   = help: the code block will either not be tested if not marked as a rust one or the code will be wrapped inside a main function
error: aborting due to 15 previous errors


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/deny-missing-docs-macro.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-missing-docs-macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-macro" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-macro/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: missing documentation for macro
  --> /checkout/src/test/rustdoc-ui/deny-missing-docs-macro.rs:6:1
   |
LL | macro_rules! foo { //~ ERROR
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/deny-missing-docs-macro.rs:3:9
   |
   |
LL | #![deny(missing_docs)]

error: aborting due to previous error



------------------------------------------


---- [ui] rustdoc-ui/deny-intra-link-resolution-failure.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-intra-link-resolution-failure/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unresolved link to `v2`
  --> /checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs:3:6
   |
LL | /// [v2] //~ ERROR
   |      ^^ no item named `v2` in scope
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/deny-intra-link-resolution-failure.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: aborting due to previous error

---
---- [ui] rustdoc-ui/doc-spotlight.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-spotlight.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-spotlight" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-spotlight/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown `doc` attribute `spotlight`
   |
   |
LL | #[doc(spotlight)]
   |       ^^^^^^^^^ help: use `notable_trait` instead
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/doc-spotlight.rs:2:9
   |
LL | #![deny(warnings)]
LL | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(invalid_doc_attributes)]` implied by `#[deny(warnings)]`
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: for more information, see issue #82730 <https://github.com/rust-lang/rust/issues/82730>
   = note: `doc(spotlight)` was renamed to `doc(notable_trait)`
   = note: `doc(spotlight)` is now a no-op
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/doc-without-codeblock.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/doc-without-codeblock.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-without-codeblock/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: missing code example in this documentation
  --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:1:1
   |
LL | / #![deny(rustdoc::missing_doc_code_examples)] //~ ERROR missing code example in this documentation
LL | |
LL | | /// Some docs.
LL | | //~^ ERROR missing code example in this documentation
LL | |     pub fn bar() {}
LL | | }
   | |_^
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:1:9
   |
LL | #![deny(rustdoc::missing_doc_code_examples)] //~ ERROR missing code example in this documentation

error: missing code example in this documentation
  --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:7:1
   |
   |
LL | /// And then, the princess died.

error: missing code example in this documentation
  --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:10:5
   |
   |
LL |     /// Or maybe not because she saved herself!

error: missing code example in this documentation
  --> /checkout/src/test/rustdoc-ui/doc-without-codeblock.rs:3:1
   |
---
---- [ui] rustdoc-ui/intra-doc/ambiguity.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/ambiguity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/ambiguity" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/ambiguity/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `true` is both a module and a builtin type
   |
   |
LL | /// [true] //~ ERROR `true` is both a module and a builtin type
   |      ^^^^ ambiguous link
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/ambiguity.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to link to the module, prefix with `mod@`
   |
LL | /// [mod@true] //~ ERROR `true` is both a module and a builtin type
   |      ^^^^^^^^
help: to link to the builtin type, prefix with `prim@`
   |
LL | /// [prim@true] //~ ERROR `true` is both a module and a builtin type


error: `ambiguous` is both a struct and a function
   |
   |
LL | /// [`ambiguous`] is ambiguous. //~ERROR `ambiguous`
   |      ^^^^^^^^^^^ ambiguous link
   |
help: to link to the struct, prefix with `struct@`
   |
LL | /// [`struct@ambiguous`] is ambiguous. //~ERROR `ambiguous`
help: to link to the function, add parentheses
   |
   |
LL | /// [`ambiguous()`] is ambiguous. //~ERROR `ambiguous`


error: `ambiguous` is both a struct and a function
   |
   |
LL | /// [ambiguous] is ambiguous. //~ERROR ambiguous
   |      ^^^^^^^^^ ambiguous link
   |
help: to link to the struct, prefix with `struct@`
   |
LL | /// [struct@ambiguous] is ambiguous. //~ERROR ambiguous
help: to link to the function, add parentheses
   |
   |
LL | /// [ambiguous()] is ambiguous. //~ERROR ambiguous


error: `multi_conflict` is a struct, a function, and a macro
   |
   |
LL | /// [`multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`
   |      ^^^^^^^^^^^^^^^^ ambiguous link
   |
help: to link to the struct, prefix with `struct@`
   |
LL | /// [`struct@multi_conflict`] is a three-way conflict. //~ERROR `multi_conflict`
help: to link to the function, add parentheses
   |
   |
LL | /// [`multi_conflict()`] is a three-way conflict. //~ERROR `multi_conflict`
   |      ^^^^^^^^^^^^^^^^^^
help: to link to the macro, add an exclamation mark
   |
LL | /// [`multi_conflict!`] is a three-way conflict. //~ERROR `multi_conflict`


error: `type_and_value` is both a module and a constant
   |
   |
LL | /// Ambiguous [type_and_value]. //~ERROR type_and_value
   |                ^^^^^^^^^^^^^^ ambiguous link
   |
help: to link to the module, prefix with `mod@`
   |
LL | /// Ambiguous [mod@type_and_value]. //~ERROR type_and_value
   |                ^^^^^^^^^^^^^^^^^^
help: to link to the constant, prefix with `const@`
   |
LL | /// Ambiguous [const@type_and_value]. //~ERROR type_and_value


error: `foo::bar` is both an enum and a function
   |
   |
LL | /// Ambiguous non-implied shortcut link [`foo::bar`]. //~ERROR `foo::bar`
   |                                          ^^^^^^^^^^ ambiguous link
   |
help: to link to the enum, prefix with `enum@`
   |
LL | /// Ambiguous non-implied shortcut link [`enum@foo::bar`]. //~ERROR `foo::bar`
help: to link to the function, add parentheses
   |
   |
LL | /// Ambiguous non-implied shortcut link [`foo::bar()`]. //~ERROR `foo::bar`

error: aborting due to 6 previous errors



------------------------------------------


---- [ui] rustdoc-ui/intra-doc/anchors.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/anchors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/anchors" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/anchors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `prim@usize#x` contains an anchor, but links to builtin types are already anchored
   |
   |
LL | /// [prim@usize#x]
   |                |
   |                invalid anchor
   |
note: the lint level is defined here
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/anchors.rs:1:9
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   = note: this restriction may be lifted in a future release
   = note: see https://github.com/rust-lang/rust/issues/83083 for more information


error: `Foo::f#hola` contains an anchor, but links to fields are already anchored
   |
   |
LL | /// Or maybe [Foo::f#hola].
   |                     |
   |                     invalid anchor


error: `hello#people#!` contains multiple anchors
   |
   |
LL | /// Another anchor error: [hello#people#!].
   |                                        |
   |                                        invalid anchor


error: `Enum::A#whatever` contains an anchor, but links to variants are already anchored
   |
   |
LL | /// Damn enum's variants: [Enum::A#whatever].
   |                                   |
   |                                   invalid anchor


error: `u32#hello` contains an anchor, but links to builtin types are already anchored
   |
   |
LL | /// [u32#hello]
   |      ^^^------
   |         invalid anchor
   |
   = note: this restriction may be lifted in a future release
   = note: see https://github.com/rust-lang/rust/issues/83083 for more information
---
---- [ui] rustdoc-ui/intra-doc/disambiguator-mismatch.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/disambiguator-mismatch" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/disambiguator-mismatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: incompatible link kind for `S`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:14:14
   |
LL | /// Link to [struct@S]
   |              ^^^^^^^^ help: to link to the enum, prefix with `enum@`: `enum@S`
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this link resolved to an enum, which is not a struct
error: incompatible link kind for `S`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:19:14
   |
   |
LL | /// Link to [mod@S]
   |              ^^^^^ help: to link to the enum, prefix with `enum@`: `enum@S`
   |
   = note: this link resolved to an enum, which is not a module
error: incompatible link kind for `S`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:24:14
   |
   |
LL | /// Link to [union@S]
   |              ^^^^^^^ help: to link to the enum, prefix with `enum@`: `enum@S`
   |
   = note: this link resolved to an enum, which is not a union
error: incompatible link kind for `S`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:29:14
   |
   |
LL | /// Link to [trait@S]
   |              ^^^^^^^ help: to link to the enum, prefix with `enum@`: `enum@S`
   |
   = note: this link resolved to an enum, which is not a trait
error: incompatible link kind for `T`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:34:14
   |
   |
LL | /// Link to [struct@T]
   |              ^^^^^^^^ help: to link to the trait, prefix with `trait@`: `trait@T`
   |
   = note: this link resolved to a trait, which is not a struct
error: incompatible link kind for `m`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:39:14
   |
   |
LL | /// Link to [derive@m]
   |              ^^^^^^^^ help: to link to the macro, add an exclamation mark: `m!`
   |
   = note: this link resolved to a macro, which is not a derive macro
error: incompatible link kind for `s`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:44:14
   |
   |
LL | /// Link to [const@s]
   |              ^^^^^^^ help: to link to the static, prefix with `static@`: `static@s`
   = note: this link resolved to a static, which is not a constant

error: incompatible link kind for `c`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:49:14
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:49:14
   |
LL | /// Link to [static@c]
   |              ^^^^^^^^ help: to link to the constant, prefix with `const@`: `const@c`
   = note: this link resolved to a constant, which is not a static

error: incompatible link kind for `c`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:54:14
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:54:14
   |
LL | /// Link to [fn@c]
   |              ^^^^ help: to link to the constant, prefix with `const@`: `const@c`
   = note: this link resolved to a constant, which is not a function

error: incompatible link kind for `c`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:59:14
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:59:14
   |
LL | /// Link to [c()]
   |              ^^^ help: to link to the constant, prefix with `const@`: `const@c`
   = note: this link resolved to a constant, which is not a function

error: incompatible link kind for `f`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:64:14
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:64:14
   |
LL | /// Link to [const@f]
   |              ^^^^^^^ help: to link to the function, add parentheses: `f()`
   |
   = note: this link resolved to a function, which is not a constant
error: aborting due to 11 previous errors


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/intra-doc/email-address-localhost.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/email-address-localhost.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/email-address-localhost" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/email-address-localhost/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown disambiguator `hello`
   |
   |
LL | //! Email me at <hello@localhost>.
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/email-address-localhost.rs:2:9
   |
   |
LL | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(rustdoc::broken_intra_doc_links)]` implied by `#[deny(warnings)]`
   = note: see https://doc.rust-lang.org/nightly/rustdoc/linking-to-items-by-name.html#namespaces-and-disambiguators for more info about disambiguators
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/intra-doc/alias-ice.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/alias-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/alias-ice" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/alias-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unresolved link to `TypeAlias::hoge`
   |
   |
LL | /// [broken cross-reference](TypeAlias::hoge) //~ ERROR
   |                              ^^^^^^^^^^^^^^^ the type alias `TypeAlias` has no associated item named `hoge`
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/alias-ice.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]

error: aborting due to previous error



------------------------------------------


---- [ui] rustdoc-ui/intra-doc/errors.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/errors" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/errors/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unresolved link to `path::to::nonexistent::module`
   |
   |
LL | /// [path::to::nonexistent::module]
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `path` in scope
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/errors.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]


error: unresolved link to `path::to::nonexistent::macro`
   |
   |
LL | /// [path::to::nonexistent::macro!]
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `path` in scope

error: unresolved link to `path::to::nonexistent::type`
   |
   |
LL | /// [type@path::to::nonexistent::type]
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no item named `path` in scope

error: unresolved link to `std::io::not::here`
   |
   |
LL | /// [std::io::not::here]
   |      ^^^^^^^^^^^^^^^^^^ no item named `not` in module `io`

error: unresolved link to `std::io::not::here`
   |
   |
LL | /// [type@std::io::not::here]
   |      ^^^^^^^^^^^^^^^^^^^^^^^ no item named `not` in module `io`

error: unresolved link to `std::io::Error::x`
   |
   |
LL | /// [std::io::Error::x]
   |      ^^^^^^^^^^^^^^^^^ the struct `Error` has no field or associated item named `x`

error: unresolved link to `std::io::ErrorKind::x`
   |
   |
LL | /// [std::io::ErrorKind::x]
   |      ^^^^^^^^^^^^^^^^^^^^^ the enum `ErrorKind` has no variant or associated item named `x`

error: unresolved link to `f::A`
   |
   |
LL | /// [f::A]
   |      ^^^^ `f` is a function, not a module or type, and cannot have associated items

error: unresolved link to `f::A`
   |
   |
LL | /// [f::A!]
   |      ^^^^^ `f` is a function, not a module or type, and cannot have associated items

error: unresolved link to `S::A`
   |
   |
LL | /// [S::A]
   |      ^^^^ the struct `S` has no field or associated item named `A`

error: unresolved link to `S::fmt`
   |
   |
LL | /// [S::fmt]
   |      ^^^^^^ the struct `S` has no field or associated item named `fmt`

error: unresolved link to `E::D`
   |
   |
LL | /// [E::D]
   |      ^^^^ the enum `E` has no variant or associated item named `D`

error: unresolved link to `u8::not_found`
   |
   |
LL | /// [u8::not_found]
   |      ^^^^^^^^^^^^^ the builtin type `u8` has no associated item named `not_found`

error: unresolved link to `std::primitive::u8::not_found`
   |
   |
LL | /// [std::primitive::u8::not_found]
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the builtin type `u8` has no associated item named `not_found`

error: unresolved link to `Vec::into_iter`
   |
   |
LL | /// [type@Vec::into_iter]
   |      |
   |      |
   |      this link resolves to the associated function `into_iter`, which is not in the type namespace
   |      help: to link to the associated function, add parentheses: `Vec::into_iter()`
error: unresolved link to `S`
  --> /checkout/src/test/rustdoc-ui/intra-doc/errors.rs:68:6
   |
   |
LL | /// [S!]
   |      |
   |      |
   |      this link resolves to the struct `S`, which is not in the macro namespace
   |      help: to link to the struct, prefix with `struct@`: `struct@S`

error: unresolved link to `S::h`
   |
   |
LL | /// [type@S::h]
   |      |
   |      |
   |      this link resolves to the associated function `h`, which is not in the type namespace
   |      help: to link to the associated function, add parentheses: `S::h()`

error: unresolved link to `T::g`
   |
   |
LL | /// [type@T::g]
   |      |
   |      |
   |      this link resolves to the associated function `g`, which is not in the type namespace
   |      help: to link to the associated function, add parentheses: `T::g()`

error: unresolved link to `T::h`
   |
   |
LL | /// [T::h!]
   |      ^^^^^ the trait `T` has no macro named `h`
error: unresolved link to `m`
  --> /checkout/src/test/rustdoc-ui/intra-doc/errors.rs:98:6
   |
   |
LL | /// [m()]
   |      |
   |      |
   |      this link resolves to the macro `m`, which is not in the value namespace
   |      help: to link to the macro, add an exclamation mark: `m!`
error: aborting due to 20 previous errors


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/intra-doc/field-ice.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/field-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/field-ice" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/field-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: incompatible link kind for `Foo::bar`
   |
   |
LL | /// [`Foo::bar()`]
   |      ^^^^^^^^^^^^ help: to link to the field, remove the disambiguator: ``Foo::bar``
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/field-ice.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: this link resolved to a field, which is not a function
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] rustdoc-ui/intra-doc/incompatible-primitive-disambiguator.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/incompatible-primitive-disambiguator.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/incompatible-primitive-disambiguator" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/incompatible-primitive-disambiguator/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: incompatible link kind for `u8::MIN`
   |
   |
LL | //! [static@u8::MIN]
   |      ^^^^^^^^^^^^^^ help: to link to the associated constant, prefix with `const@`: `const@u8::MIN`
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/incompatible-primitive-disambiguator.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   = note: this link resolved to an associated constant, which is not a static

error: aborting due to previous error

---
---- [ui] rustdoc-ui/intra-doc/malformed-generics.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/malformed-generics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/malformed-generics" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/malformed-generics/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unresolved link to `Vec<`
   |
   |
LL | //! [Vec<] //~ ERROR
   |      ^^^^ unbalanced angle brackets
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/malformed-generics.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]


error: unresolved link to `Vec<Box<T`
   |
   |
LL | //! [Vec<Box<T] //~ ERROR
   |      ^^^^^^^^^ unbalanced angle brackets

error: unresolved link to `Vec<Box<T>`
   |
   |
LL | //! [Vec<Box<T>] //~ ERROR
   |      ^^^^^^^^^^ unbalanced angle brackets

error: unresolved link to `Vec<Box<T>>>`
   |
   |
LL | //! [Vec<Box<T>>>] //~ ERROR
   |      ^^^^^^^^^^^^ unbalanced angle brackets

error: unresolved link to `Vec<T>>>`
   |
   |
LL | //! [Vec<T>>>] //~ ERROR
   |      ^^^^^^^^ unbalanced angle brackets

error: unresolved link to `<Vec`
   |
   |
LL | //! [<Vec] //~ ERROR
   |      ^^^^ unbalanced angle brackets

error: unresolved link to `Vec::<`
   |
   |
LL | //! [Vec::<] //~ ERROR
   |      ^^^^^^ unbalanced angle brackets

error: unresolved link to `<T>`
   |
   |
LL | //! [<T>] //~ ERROR
   |      ^^^ missing type for generic parameters

error: unresolved link to `<invalid syntax>`
   |
   |
LL | //! [<invalid syntax>] //~ ERROR
   |      ^^^^^^^^^^^^^^^^ missing type for generic parameters

error: unresolved link to `Vec:<T>:new`
   |
   |
LL | //! [Vec:<T>:new()] //~ ERROR
   |      ^^^^^^^^^^^^^ has invalid path separator

error: unresolved link to `Vec<<T>>`
   |
   |
LL | //! [Vec<<T>>] //~ ERROR
   |      ^^^^^^^^ too many angle brackets

error: unresolved link to `Vec<>`
   |
   |
LL | //! [Vec<>] //~ ERROR
   |      ^^^^^ empty angle brackets

error: unresolved link to `Vec<<>>`
   |
   |
LL | //! [Vec<<>>] //~ ERROR
   |      ^^^^^^^ too many angle brackets

error: unresolved link to `<Vec as IntoIterator>::into_iter`
   |
   |
LL | //! [<Vec as IntoIterator>::into_iter] //~ ERROR
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ fully-qualified syntax is unsupported
   = note: see https://github.com/rust-lang/rust/issues/74563 for more information


error: unresolved link to `<Vec<T> as IntoIterator>::iter`
   |
   |
LL | //! [<Vec<T> as IntoIterator>::iter] //~ ERROR
   |      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ fully-qualified syntax is unsupported
   = note: see https://github.com/rust-lang/rust/issues/74563 for more information

error: aborting due to 15 previous errors

---
---- [ui] rustdoc-ui/intra-doc/non-path-primitives.rs stdout ----

error: ui test compiled successfully!
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/non-path-primitives" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/non-path-primitives/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unresolved link to `T`
  --> /checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs:12:7
   |
LL | //! [[T]::rotate_left] //~ ERROR unresolved link to `T`
   |       ^ no item named `T` in scope
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Z`
  --> /checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs:14:5
   |
   |
LL | //![Z]([T; N]::map) //~ ERROR unresolved link to `Z`
   |     ^ no item named `Z` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Z`
  --> /checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs:17:6
   |
   |
LL | //! [Z][] //~ ERROR unresolved link to `Z`
   |      ^ no item named `Z` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`
error: unresolved link to `Z`
  --> /checkout/src/test/rustdoc-ui/intra-doc/non-path-primitives.rs:19:6
   |
   |
LL | //! [Z]: [T; N]::map //~ ERROR unresolved link to `Z`
   |      ^ no item named `Z` in scope
   |
   = help: to escape `[` and `]` characters, add '\' before them like `\[` or `\]`

error: unresolved link to `unit::eq`
   |
   |
LL | //! [unit::eq] //~ ERROR unresolved
   |      ^^^^^^^^ the builtin type `unit` has no associated item named `eq`

error: unresolved link to `tuple::eq`
   |
   |
LL | //! [tuple::eq] //~ ERROR unresolved
   |      ^^^^^^^^^ the builtin type `tuple` has no associated item named `eq`

error: unresolved link to `fn::eq`
   |
   |
LL | //! [fn::eq] //~ ERROR unresolved
   |      ^^^^^^ the builtin type `fn` has no associated item named `eq`

error: unresolved link to `never::eq`
   |
   |
LL | //! [never::eq] //~ ERROR unresolved


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--src-base" "/checkout/src/test/rustdoc-ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "rustdoc-ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:25:33
