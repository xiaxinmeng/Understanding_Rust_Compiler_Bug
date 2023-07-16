plain
---- [ui] src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs stdout ----
diff of stderr:

68    |
69 LL - /// Link to [derive@m]
70 LL + /// Link to [m!]
+    |
72 
73 error: unresolved link to `m`
74   --> $DIR/disambiguator-mismatch.rs:46:14
74   --> $DIR/disambiguator-mismatch.rs:46:14

124    |
125 LL - /// Link to [c()]
126 LL + /// Link to [const@c]
+    |
128 
129 error: incompatible link kind for `f`
130   --> $DIR/disambiguator-mismatch.rs:72:14
130   --> $DIR/disambiguator-mismatch.rs:72:14

136    |
137 LL - /// Link to [const@f]
138 LL + /// Link to [f()]
+    |
140 
141 error: unresolved link to `std`
142   --> $DIR/disambiguator-mismatch.rs:77:14
---
To only update this specific test, also pass `--test-args intra-doc/disambiguator-mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/disambiguator-mismatch" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/disambiguator-mismatch/auxiliary"
stdout: none
--- stderr -------------------------------
error: incompatible link kind for `S`
   |
   |
LL | /// Link to [struct@S]
   |              ^^^^^^^^ this link resolved to an enum, which is not a struct
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:1:9
   |
   |
LL | #![deny(rustdoc::broken_intra_doc_links)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: to link to the enum, prefix with `enum@`
   |
LL | /// Link to [enum@S]

error: incompatible link kind for `S`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:21:14
   |
   |
LL | /// Link to [mod@S]
   |              ^^^^^ this link resolved to an enum, which is not a module
   |
help: to link to the enum, prefix with `enum@`
   |
LL | /// Link to [enum@S]

error: incompatible link kind for `S`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:26:14
   |
   |
LL | /// Link to [union@S]
   |              ^^^^^^^ this link resolved to an enum, which is not a union
   |
help: to link to the enum, prefix with `enum@`
   |
LL | /// Link to [enum@S]

error: incompatible link kind for `S`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:31:14
   |
   |
LL | /// Link to [trait@S]
   |              ^^^^^^^ this link resolved to an enum, which is not a trait
   |
help: to link to the enum, prefix with `enum@`
   |
LL | /// Link to [enum@S]

error: incompatible link kind for `T`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:36:14
   |
   |
LL | /// Link to [struct@T]
   |              ^^^^^^^^ this link resolved to a trait, which is not a struct
   |
help: to link to the trait, prefix with `trait@`
   |
LL | /// Link to [trait@T]

error: incompatible link kind for `m`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:41:14
   |
   |
LL | /// Link to [derive@m]
   |              ^^^^^^^^ this link resolved to a macro, which is not a derive macro
   |
help: to link to the macro, add an exclamation mark
   |
LL - /// Link to [derive@m]
LL + /// Link to [m!]

error: unresolved link to `m`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:46:14
   |
   |
LL | /// Link to [m()]
   |              ^^^ this link resolves to the macro `m`, which is not in the value namespace
   |
help: to link to the macro, add an exclamation mark
   |
LL | /// Link to [m!()]

error: incompatible link kind for `s`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:52:14
   |
   |
LL | /// Link to [const@s]
   |              ^^^^^^^ this link resolved to a static, which is not a constant
   |
help: to link to the static, prefix with `static@`
   |
LL | /// Link to [static@s]

error: incompatible link kind for `c`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:57:14
   |
   |
LL | /// Link to [static@c]
   |              ^^^^^^^^ this link resolved to a constant, which is not a static
help: to link to the constant, prefix with `const@`
   |
   |
LL | /// Link to [const@c]

error: incompatible link kind for `c`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:62:14
   |
   |
LL | /// Link to [fn@c]
   |              ^^^^ this link resolved to a constant, which is not a function
help: to link to the constant, prefix with `const@`
   |
   |
LL | /// Link to [const@c]

error: incompatible link kind for `c`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:67:14
   |
   |
LL | /// Link to [c()]
   |              ^^^ this link resolved to a constant, which is not a function
help: to link to the constant, prefix with `const@`
   |
   |
LL - /// Link to [c()]
LL + /// Link to [const@c]

error: incompatible link kind for `f`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:72:14
   |
   |
LL | /// Link to [const@f]
   |              ^^^^^^^ this link resolved to a function, which is not a constant
help: to link to the function, add parentheses
   |
   |
LL - /// Link to [const@f]
LL + /// Link to [f()]

error: unresolved link to `std`
  --> /checkout/src/test/rustdoc-ui/intra-doc/disambiguator-mismatch.rs:77:14
   |
   |
LL | /// Link to [fn@std]
   |              ^^^^^^ this link resolves to the crate `std`, which is not in the value namespace
help: to link to the crate, prefix with `mod@`
   |
   |
LL | /// Link to [mod@std]

error: aborting due to 13 previous errors
------------------------------------------



---- [ui] src/test/rustdoc-ui/intra-doc/errors.rs stdout ----
diff of stderr:

98    |
99 LL - /// [type@Vec::into_iter]
100 LL + /// [Vec::into_iter()]
+    |
102 
103 error: unresolved link to `S`
104   --> $DIR/errors.rs:68:6
104   --> $DIR/errors.rs:68:6

110    |
111 LL - /// [S!]
112 LL + /// [struct@S]
+    |
114 
114 
115 error: unresolved link to `S::h`

122    |
122    |
123 LL - /// [type@S::h]
124 LL + /// [S::h()]
+    |
126 
126 
127 error: unresolved link to `T::g`

134    |
134    |
135 LL - /// [type@T::g]
136 LL + /// [T::g()]
+    |
138 
138 
139 error: unresolved link to `T::h`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/errors/errors.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/errors/errors.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intra-doc/errors.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/intra-doc/errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/errors" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/intra-doc/errors/auxiliary"
stdout: none
--- stderr -------------------------------
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
  --> /checkout/src/test/rustdoc-ui/intra-doc/errors.rs:63:6
   |
   |
LL | /// [type@Vec::into_iter]
   |      ^^^^^^^^^^^^^^^^^^^ this link resolves to the associated function `into_iter`, which is not in the type namespace
help: to link to the associated function, add parentheses
   |
   |
LL - /// [type@Vec::into_iter]
LL + /// [Vec::into_iter()]

error: unresolved link to `S`
  --> /checkout/src/test/rustdoc-ui/intra-doc/errors.rs:68:6
   |
   |
LL | /// [S!]
   |      ^^ this link resolves to the struct `S`, which is not in the macro namespace
   |
help: to link to the struct, prefix with `struct@`
   |
LL - /// [S!]
LL + /// [struct@S]


error: unresolved link to `S::h`
   |
   |
LL | /// [type@S::h]
   |      ^^^^^^^^^ this link resolves to the associated function `h`, which is not in the type namespace
help: to link to the associated function, add parentheses
   |
   |
LL - /// [type@S::h]
LL + /// [S::h()]


error: unresolved link to `T::g`
   |
   |
LL | /// [type@T::g]
   |      ^^^^^^^^^ this link resolves to the associated function `g`, which is not in the type namespace
help: to link to the associated function, add parentheses
   |
   |
LL - /// [type@T::g]
LL + /// [T::g()]


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
   |      ^^^ this link resolves to the macro `m`, which is not in the value namespace
   |
help: to link to the macro, add an exclamation mark
   |
LL | /// [m!()]

error: aborting due to 20 previous errors
------------------------------------------



---- [ui] src/test/rustdoc-ui/test-compile-fail1.rs stdout ----
diff of stderr:

3   |
4 6 | pub fn f() {}
5   | ---------- previous definition of the value `f` here
- 7 | 
7 8 | pub fn f() {}
7 8 | pub fn f() {}
8   | ^^^^^^^^^^ `f` redefined here


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/test-compile-fail1/test-compile-fail1.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/test-compile-fail1/test-compile-fail1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args test-compile-fail1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/test-compile-fail1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/test-compile-fail1" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/test-compile-fail1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0428]: the name `f` is defined multiple times
  |
6 | pub fn f() {}
6 | pub fn f() {}
  | ---------- previous definition of the value `f` here
8 | pub fn f() {}
8 | pub fn f() {}
  | ^^^^^^^^^^ `f` redefined here
  |
  = note: `f` must be defined only once in the value namespace of this module
error: aborting due to previous error

For more information about this error, try `rustc --explain E0428`.
------------------------------------------
