plain
failures:

---- [ui] src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found keyword `await`
   |
   |
LL |     pub mod await { //~ ERROR expected identifier, found keyword `await`
   |
   |
help: escape `await` to use it as an identifier
   |
LL |     pub mod r#await { //~ ERROR expected identifier, found keyword `await`

error: expected identifier, found keyword `await`
  --> /checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs:7:20
   |
   |
LL |         pub struct await; //~ ERROR expected identifier, found keyword `await`
   |
   |
help: escape `await` to use it as an identifier
   |
LL |         pub struct r#await; //~ ERROR expected identifier, found keyword `await`

error: expected identifier, found keyword `await`
  --> /checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs:10:22
   |
   |
LL | use self::outer_mod::await::await; //~ ERROR expected identifier, found keyword `await`
   |
   |
help: escape `await` to use it as an identifier
   |
LL | use self::outer_mod::r#await::await; //~ ERROR expected identifier, found keyword `await`

error: expected identifier, found keyword `await`
  --> /checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs:10:29
   |
   |
LL | use self::outer_mod::await::await; //~ ERROR expected identifier, found keyword `await`
   |
   |
help: escape `await` to use it as an identifier
   |
LL | use self::outer_mod::await::r#await; //~ ERROR expected identifier, found keyword `await`

error: internal compiler error: the following error was constructed but not emitted

error: expected identifier, found keyword `await`
error: expected identifier, found keyword `await`
  --> /checkout/src/test/ui/async-await/await-keyword/2018-edition-error-in-non-macro-position.rs:13:14
   |
LL | struct Foo { await: () }
   |
   |
help: escape `await` to use it as an identifier
   |
LL | struct Foo { r#await: () }

thread 'rustc' panicked at 'error was constructed but not emitted', compiler/rustc_errors/src/diagnostic_builder.rs:608:21
stack backtrace:
   0:     0x7f43f417453e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1c997130205068a9
   0:     0x7f43f417453e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h1c997130205068a9
   1:     0x7f43f41dd4e8 - core::fmt::write::hbc97408713459ab4
   2:     0x7f43f4165371 - std::io::Write::write_fmt::h91a12f12c8981e87
   3:     0x7f43f41775ee - std::panicking::default_hook::{{closure}}::hed186f1a76ad6eaf
   4:     0x7f43f41772ae - std::panicking::default_hook::h2156c81985f62f50
   5:     0x7f43f4b279b4 - rustc_driver[6dbd923c17f806cf]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f43f4177dad - std::panicking::rust_panic_with_hook::h8e93a276d045c73e
   7:     0x7f43f4177b89 - std::panicking::begin_panic_handler::{{closure}}::hb6f9ca69c61e5110
   8:     0x7f43f4174a74 - std::sys_common::backtrace::__rust_end_short_backtrace::h01813bcdbe1151d0
   9:     0x7f43f4177892 - rust_begin_unwind
  10:     0x7f43f4126f03 - core::panicking::panic_fmt::h83325d96fde1893b
  11:     0x7f43f78ae32f - <rustc_errors[e55928990a1d2ea2]::diagnostic_builder::DiagnosticBuilderInner as core[ecf400fcca37ab18]::ops::drop::Drop>::drop
  12:     0x7f43f6f22f26 - <rustc_parse[f3070ff06beed6ca]::parser::Parser>::parse_field_ident
  13:     0x7f43f6f211ee - <rustc_parse[f3070ff06beed6ca]::parser::Parser>::parse_field_def::{closure#0}
  14:     0x7f43f6f1e8ce - <rustc_parse[f3070ff06beed6ca]::parser::Parser>::parse_record_struct_body
  15:     0x7f43f6f1def3 - <rustc_parse[f3070ff06beed6ca]::parser::Parser>::parse_item_struct
  16:     0x7f43f6f10fd9 - <rustc_parse[f3070ff06beed6ca]::parser::Parser>::parse_item_common
  17:     0x7f43f6f0d37e - <rustc_parse[f3070ff06beed6ca]::parser::Parser>::parse_mod
  18:     0x7f43f6f0cfb8 - <rustc_parse[f3070ff06beed6ca]::parser::Parser>::parse_crate_mod
  19:     0x7f43f6f7265d - rustc_parse[f3070ff06beed6ca]::parse_crate_from_file
  20:     0x7f43f4c7c4f2 - rustc_interface[8fe2259424532a65]::passes::parse
  21:     0x7f43f4c572c5 - <rustc_interface[8fe2259424532a65]::queries::Queries>::parse
  22:     0x7f43f4b2b06b - rustc_interface[8fe2259424532a65]::interface::create_compiler_and_run::<core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>, rustc_driver[6dbd923c17f806cf]::run_compiler::{closure#1}>
  23:     0x7f43f4b96bc2 - <scoped_tls[1cf5fc7ebb1bf2b6]::ScopedKey<rustc_span[c8b130719d9aaa55]::SessionGlobals>>::set::<rustc_interface[8fe2259424532a65]::interface::run_compiler<core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>, rustc_driver[6dbd923c17f806cf]::run_compiler::{closure#1}>::{closure#0}, core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>>
  24:     0x7f43f4b8beb9 - std[9dd068e91d873d65]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8fe2259424532a65]::util::run_in_thread_pool_with_globals<rustc_interface[8fe2259424532a65]::interface::run_compiler<core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>, rustc_driver[6dbd923c17f806cf]::run_compiler::{closure#1}>::{closure#0}, core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>>::{closure#0}, core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>>
  25:     0x7f43f4b2cdde - std[9dd068e91d873d65]::panic::catch_unwind::<core[ecf400fcca37ab18]::panic::unwind_safe::AssertUnwindSafe<<std[9dd068e91d873d65]::thread::Builder>::spawn_unchecked_<rustc_interface[8fe2259424532a65]::util::run_in_thread_pool_with_globals<rustc_interface[8fe2259424532a65]::interface::run_compiler<core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>, rustc_driver[6dbd923c17f806cf]::run_compiler::{closure#1}>::{closure#0}, core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>>::{closure#0}, core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>>
  26:     0x7f43f4b8fcb2 - <<std[9dd068e91d873d65]::thread::Builder>::spawn_unchecked_<rustc_interface[8fe2259424532a65]::util::run_in_thread_pool_with_globals<rustc_interface[8fe2259424532a65]::interface::run_compiler<core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>, rustc_driver[6dbd923c17f806cf]::run_compiler::{closure#1}>::{closure#0}, core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>>::{closure#0}, core[ecf400fcca37ab18]::result::Result<(), rustc_errors[e55928990a1d2ea2]::ErrorGuaranteed>>::{closure#1} as core[ecf400fcca37ab18]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f43f4184bf5 - std::sys::unix::thread::Thread::new::thread_start::hf298edc70f895198
  28:     0x7f43f3f1fb43 - <unknown>
  29:     0x7f43f3fb1a00 - <unknown>
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (007361b77 2022-09-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 6 previous errors
------------------------------------------
---

13 LL | trait T {
14    | ^^^^^ expected identifier, found keyword
15    |
-    = help: the let keyword is not allowed in struct field definitions
-    = help: see https://doc.rust-lang.org/book/ch05-01-defining-structs.html for more information
+    = help: see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information
+ help: remove the let, the `let` keyword is not allowed in struct field definitions
+    |
+ LL - trait T {
+ LL +  T {
18 
18 
- error: expected `:`, found `T`
-   --> $DIR/missing-close-brace-in-struct.rs:4:7
+ error: expected `:`, found `{`
21    |
22 LL | trait T {
-    |       ^ expected `:`
+    |         ^ expected `:`
---
To only update this specific test, also pass `--test-args parser/mismatched-braces/missing-close-brace-in-struct.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-struct" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/mismatched-braces/missing-close-brace-in-struct/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs:13:65
   |
   |
LL | pub(crate) struct Bar<T> {
...
...
LL | fn main() {} //~ ERROR this file contains an unclosed delimiter

error: expected identifier, found keyword `trait`
  --> /checkout/src/test/ui/parser/mismatched-braces/missing-close-brace-in-struct.rs:4:1
   |
   |
LL | trait T { //~ ERROR expected identifier, found keyword `trait`
   |
   |
   = help: see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information
help: remove the let, the `let` keyword is not allowed in struct field definitions
   |
LL - trait T { //~ ERROR expected identifier, found keyword `trait`
LL +  T { //~ ERROR expected identifier, found keyword `trait`


error: expected `:`, found `{`
   |
   |
LL | trait T { //~ ERROR expected identifier, found keyword `trait`
   |         ^ expected `:`
error: aborting due to 3 previous errors
------------------------------------------


---
6    |
-    = help: the let keyword is not allowed in struct field definitions
-    = help: see https://doc.rust-lang.org/book/ch05-01-defining-structs.html for more information
- 
- error: expected `:`, found `foo`
-   --> $DIR/removed-syntax-field-let.rs:2:9
+    = help: see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information
+ help: remove the let, the `let` keyword is not allowed in struct field definitions
- LL |     let foo: (),
-    |         ^^^ expected `:`
-    |         ^^^ expected `:`
+ LL -     let foo: (),
+ LL +      foo: (),
15 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
17 
---
To only update this specific test, also pass `--test-args parser/removed-syntax-field-let.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/removed-syntax-field-let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/removed-syntax-field-let" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/removed-syntax-field-let/auxiliary"
stdout: none
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
--- stderr -------------------------------
error: expected identifier, found keyword `let`
   |
LL |     let foo: (),
   |     ^^^ expected identifier, found keyword
   |
   |
   = help: see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information
help: remove the let, the `let` keyword is not allowed in struct field definitions
LL -     let foo: (),
LL +      foo: (),
   |

