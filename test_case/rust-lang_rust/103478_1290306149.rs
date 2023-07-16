plain
failures:

---- [ui] src/test/ui/parser/trait-object-delimiters.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/trait-object-delimiters.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-delimiters" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/trait-object-delimiters/auxiliary"
stdout: none
--- stderr -------------------------------
error: ambiguous `+` in a type
   |
   |
LL | fn foo1(_: &dyn Drop + AsRef<str>) {} //~ ERROR ambiguous `+` in a type
   |             ^^^^^^^^^^^^^^^^^^^^^ help: use parentheses to disambiguate: `(dyn Drop + AsRef<str>)`
error: incorrect braces around trait bounds
  --> /checkout/src/test/ui/parser/trait-object-delimiters.rs:6:17
   |
   |
LL | fn foo2(_: &dyn (Drop + AsRef<str>)) {} //~ ERROR incorrect braces around trait bounds
   |                 ^                 ^
help: remove the parentheses
   |
   |
LL - fn foo2(_: &dyn (Drop + AsRef<str>)) {} //~ ERROR incorrect braces around trait bounds
LL + fn foo2(_: &dyn Drop + AsRef<str>) {} //~ ERROR incorrect braces around trait bounds

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: expected parameter name, found `{`
  --> /checkout/src/test/ui/parser/trait-object-delimiters.rs:8:17
  --> /checkout/src/test/ui/parser/trait-object-delimiters.rs:8:17
   |
LL | fn foo3(_: &dyn {Drop + AsRef<str>}) {} //~ ERROR expected parameter name, found `{`
   |                 ^ expected parameter name

error: expected one of `!`, `(`, `)`, `*`, `,`, `?`, `for`, `~`, lifetime, or path, found `{`
   |
   |
LL | fn foo3(_: &dyn {Drop + AsRef<str>}) {} //~ ERROR expected parameter name, found `{`
   |                -^ expected one of 10 possible tokens
   |                help: missing `,`

error: internal compiler error: the following error was constructed but not emitted


error: expected one of `,`, `:`, `=`, or `>`, found `+`
   |
   |
LL | fn foo4(_: &dyn <Drop + AsRef<str>>) {} //~ ERROR expected identifier, found `<`
   |                       ^ expected one of `,`, `:`, `=`, or `>`
thread 'rustc' panicked at 'error was constructed but not emitted', compiler/rustc_errors/src/diagnostic_builder.rs:671:21
stack backtrace:
stack backtrace:
   0:     0x7fb9d1d435de - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf13649e211d1dece
   1:     0x7fb9d1dacaf8 - core::fmt::write::h611eb9a0b804be2c
   2:     0x7fb9d1d34cf1 - std::io::Write::write_fmt::h6f7257b9da5598c1
   3:     0x7fb9d1d433e1 - std::sys_common::backtrace::print::h8bc06fad78de09d9
   4:     0x7fb9d1d46554 - std::panicking::default_hook::{{closure}}::h3322cd3e8c315a5f
   5:     0x7fb9d1d46219 - std::panicking::default_hook::h14050209e942d7e5
   6:     0x7fb9d2734be4 - rustc_driver[61cdcd7c422ac744]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fb9d1d46ca4 - std::panicking::rust_panic_with_hook::hb51ea536c2b885df
   8:     0x7fb9d1d469c9 - std::panicking::begin_panic_handler::{{closure}}::h277ce3a9ec748808
   9:     0x7fb9d1d43b14 - std::sys_common::backtrace::__rust_end_short_backtrace::hb1064a5f3a086921
  10:     0x7fb9d1d466d2 - rust_begin_unwind
  11:     0x7fb9d1cf7aa3 - core::panicking::panic_fmt::hbab9518fe514d61c
  12:     0x7fb9d55f672f - <rustc_errors[2a002e8cbda2c840]::diagnostic_builder::DiagnosticBuilderInner as core[f78a9a4743325546]::ops::drop::Drop>::drop
  13:     0x7fb9d4c4e7ae - <rustc_parse[9aa0de9253274d95]::parser::Parser>::expected_ident_found
  14:     0x7fb9d4cced28 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_ident_common
  15:     0x7fb9d4cb8b05 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_path_segment
  16:     0x7fb9d4cb87c2 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_path_segments
  17:     0x7fb9d4cb844f - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_path_inner
  18:     0x7fb9d4ccb751 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_generic_bounds_common
  19:     0x7fb9d4cca895 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_dyn_ty
  20:     0x7fb9d4cc7897 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_ty_common
  21:     0x7fb9d4cc99d9 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_borrowed_pointee
  22:     0x7fb9d4cc63c4 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_ty_common
  23:     0x7fb9d4ca9f26 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_param_general::{closure#0}
  24:     0x7fb9d4ca7ff2 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_fn_params::{closure#0}
  25:     0x7fb9d4ca7528 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_fn_decl
  26:     0x7fb9d4ca4ea9 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_fn
  27:     0x7fb9d4c8f05c - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_item_common
  28:     0x7fb9d4c8e0ee - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_mod
  29:     0x7fb9d4c8dd08 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_crate_mod
  30:     0x7fb9d4d2df4d - rustc_parse[9aa0de9253274d95]::parse_crate_from_file
  31:     0x7fb9d2861175 - <rustc_session[48f4bdc7c1cab0f3]::session::Session>::time::<core[f78a9a4743325546]::result::Result<rustc_ast[15d33fb3ce2d735e]::ast::Crate, rustc_errors[2a002e8cbda2c840]::diagnostic_builder::DiagnosticBuilder<rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>, rustc_interface[85d830dfb3cd14ec]::passes::parse::{closure#0}>
  32:     0x7fb9d288ecf7 - rustc_interface[85d830dfb3cd14ec]::passes::parse
  33:     0x7fb9d287ec35 - <rustc_interface[85d830dfb3cd14ec]::queries::Queries>::parse
  34:     0x7fb9d27a54da - <rustc_interface[85d830dfb3cd14ec]::interface::Compiler>::enter::<rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}::{closure#2}, core[f78a9a4743325546]::result::Result<core[f78a9a4743325546]::option::Option<rustc_interface[85d830dfb3cd14ec]::queries::Linker>, rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>
  35:     0x7fb9d273632e - rustc_span[25b079398ffd9b12]::with_source_map::<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_interface[85d830dfb3cd14ec]::interface::run_compiler<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  36:     0x7fb9d27990bc - <scoped_tls[b6e8d73e7a6b41ba]::ScopedKey<rustc_span[25b079398ffd9b12]::SessionGlobals>>::set::<rustc_interface[85d830dfb3cd14ec]::interface::run_compiler<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}>::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>
  37:     0x7fb9d2755879 - std[217bdc2af5db14e1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[85d830dfb3cd14ec]::util::run_in_thread_pool_with_globals<rustc_interface[85d830dfb3cd14ec]::interface::run_compiler<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}>::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>
  38:     0x7fb9d279a1e8 - std[217bdc2af5db14e1]::panic::catch_unwind::<core[f78a9a4743325546]::panic::unwind_safe::AssertUnwindSafe<<std[217bdc2af5db14e1]::thread::Builder>::spawn_unchecked_<rustc_interface[85d830dfb3cd14ec]::util::run_in_thread_pool_with_globals<rustc_interface[85d830dfb3cd14ec]::interface::run_compiler<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}>::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>
  39:     0x7fb9d274635a - <<std[217bdc2af5db14e1]::thread::Builder>::spawn_unchecked_<rustc_interface[85d830dfb3cd14ec]::util::run_in_thread_pool_with_globals<rustc_interface[85d830dfb3cd14ec]::interface::run_compiler<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}>::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>::{closure#1} as core[f78a9a4743325546]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7fb9d1d533ce - std::sys::unix::thread::Thread::new::thread_start::h459971eaa2b6a111
  41:     0x7fb9d1aedb43 - <unknown>
  42:     0x7fb9d1b7fa00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (972111925 2022-10-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 6 previous errors
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/rust-2018/dyn-trait-compatibility.rs stdout ----
diff of stderr:

9 LL | type A1 = dyn::r#dyn;
11 
+ error: expected identifier, found keyword `dyn`
+   --> $DIR/dyn-trait-compatibility.rs:5:15
+    |
+    |
+ LL | type A2 = dyn<dyn, dyn>;
+    |
+    |
+ help: escape `dyn` to use it as an identifier
+    |
+ LL | type A2 = dyn<r#dyn, dyn>;
+ 
+ error: expected identifier, found keyword `dyn`
+   --> $DIR/dyn-trait-compatibility.rs:5:20
+    |
+    |
+ LL | type A2 = dyn<dyn, dyn>;
+ 
12 error: expected identifier, found `<`
13   --> $DIR/dyn-trait-compatibility.rs:5:14
14    |
14    |

15 LL | type A2 = dyn<dyn, dyn>;
+    |
+    |
+ help: help: place the generic parameter list after the function name:
+    |
+ LL | type A2 = dyn ;<dyn, dyn>
17 
- error: aborting due to 2 previous errors
+ error: aborting due to 4 previous errors
19 
---
To only update this specific test, also pass `--test-args rust-2018/dyn-trait-compatibility.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rust-2018/dyn-trait-compatibility.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/dyn-trait-compatibility" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rust-2018/dyn-trait-compatibility/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found keyword `dyn`
   |
   |
LL | type A1 = dyn::dyn; //~ERROR expected identifier, found keyword `dyn`
   |
help: escape `dyn` to use it as an identifier
   |
   |
LL | type A1 = dyn::r#dyn; //~ERROR expected identifier, found keyword `dyn`

error: expected identifier, found keyword `dyn`
  --> /checkout/src/test/ui/rust-2018/dyn-trait-compatibility.rs:5:15
   |
   |
LL | type A2 = dyn<dyn, dyn>; //~ERROR expected identifier, found `<`
   |
help: escape `dyn` to use it as an identifier
   |
   |
LL | type A2 = dyn<r#dyn, dyn>; //~ERROR expected identifier, found `<`

error: expected identifier, found keyword `dyn`
  --> /checkout/src/test/ui/rust-2018/dyn-trait-compatibility.rs:5:20
   |
   |
LL | type A2 = dyn<dyn, dyn>; //~ERROR expected identifier, found `<`

error: expected identifier, found `<`
  --> /checkout/src/test/ui/rust-2018/dyn-trait-compatibility.rs:5:14
   |
   |
LL | type A2 = dyn<dyn, dyn>; //~ERROR expected identifier, found `<`
   |
   |
help: help: place the generic parameter list after the function name:
   |
LL | type A2 = dyn ;<dyn, dyn> //~ERROR expected identifier, found `<`

error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/suggestions/type-ascription-instead-of-path-2.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/type-ascription-instead-of-path-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/type-ascription-instead-of-path-2/auxiliary"
stdout: none
--- stderr -------------------------------


error: expected one of `,`, `:`, `=`, or `>`, found `<`
   |
   |
LL |     let _ = vec![Ok(2)].into_iter().collect:<Result<Vec<_>,_>>()?;
   |                                            -       ^ expected one of `,`, `:`, `=`, or `>`
   |                                            |
   |                                            help: maybe write a path separator here: `::`
   |
   = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`
thread 'rustc' panicked at 'error was constructed but not emitted', compiler/rustc_errors/src/diagnostic_builder.rs:671:21
stack backtrace:
stack backtrace:
   0:     0x7f05d83fe5de - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf13649e211d1dece
   1:     0x7f05d8467af8 - core::fmt::write::h611eb9a0b804be2c
   2:     0x7f05d83efcf1 - std::io::Write::write_fmt::h6f7257b9da5598c1
   3:     0x7f05d83fe3e1 - std::sys_common::backtrace::print::h8bc06fad78de09d9
   4:     0x7f05d8401554 - std::panicking::default_hook::{{closure}}::h3322cd3e8c315a5f
   5:     0x7f05d8401219 - std::panicking::default_hook::h14050209e942d7e5
   6:     0x7f05d8defbe4 - rustc_driver[61cdcd7c422ac744]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f05d8401ca4 - std::panicking::rust_panic_with_hook::hb51ea536c2b885df
   8:     0x7f05d84019c9 - std::panicking::begin_panic_handler::{{closure}}::h277ce3a9ec748808
   9:     0x7f05d83feb14 - std::sys_common::backtrace::__rust_end_short_backtrace::hb1064a5f3a086921
  10:     0x7f05d84016d2 - rust_begin_unwind
  11:     0x7f05d83b2aa3 - core::panicking::panic_fmt::hbab9518fe514d61c
  12:     0x7f05dbcb172f - <rustc_errors[2a002e8cbda2c840]::diagnostic_builder::DiagnosticBuilderInner as core[f78a9a4743325546]::ops::drop::Drop>::drop
  13:     0x7f05db3097ae - <rustc_parse[9aa0de9253274d95]::parser::Parser>::expected_ident_found
  14:     0x7f05db389d28 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_ident_common
  15:     0x7f05db373b05 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_path_segment
  16:     0x7f05db3738cc - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_path_segments
  17:     0x7f05db37344f - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_path_inner
  18:     0x7f05db321253 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_assoc_op_cast
  19:     0x7f05db31f657 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_assoc_expr_with
  20:     0x7f05db37d22d - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_local
  21:     0x7f05db378e50 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_stmt_without_recovery
  22:     0x7f05db37f47a - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_full_stmt
  23:     0x7f05db37f13f - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_block_tail
  24:     0x7f05db37f027 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_block_common
  25:     0x7f05db360028 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_fn
  26:     0x7f05db34a05c - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_item_common
  27:     0x7f05db3490ee - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_mod
  28:     0x7f05db348d08 - <rustc_parse[9aa0de9253274d95]::parser::Parser>::parse_crate_mod
  29:     0x7f05db3e8f4d - rustc_parse[9aa0de9253274d95]::parse_crate_from_file
  30:     0x7f05d8f1c175 - <rustc_session[48f4bdc7c1cab0f3]::session::Session>::time::<core[f78a9a4743325546]::result::Result<rustc_ast[15d33fb3ce2d735e]::ast::Crate, rustc_errors[2a002e8cbda2c840]::diagnostic_builder::DiagnosticBuilder<rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>, rustc_interface[85d830dfb3cd14ec]::passes::parse::{closure#0}>
  31:     0x7f05d8f49cf7 - rustc_interface[85d830dfb3cd14ec]::passes::parse
  32:     0x7f05d8f39c35 - <rustc_interface[85d830dfb3cd14ec]::queries::Queries>::parse
  33:     0x7f05d8e604da - <rustc_interface[85d830dfb3cd14ec]::interface::Compiler>::enter::<rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}::{closure#2}, core[f78a9a4743325546]::result::Result<core[f78a9a4743325546]::option::Option<rustc_interface[85d830dfb3cd14ec]::queries::Linker>, rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>
  34:     0x7f05d8df132e - rustc_span[25b079398ffd9b12]::with_source_map::<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_interface[85d830dfb3cd14ec]::interface::run_compiler<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  35:     0x7f05d8e540bc - <scoped_tls[b6e8d73e7a6b41ba]::ScopedKey<rustc_span[25b079398ffd9b12]::SessionGlobals>>::set::<rustc_interface[85d830dfb3cd14ec]::interface::run_compiler<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}>::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>
  36:     0x7f05d8e10879 - std[217bdc2af5db14e1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[85d830dfb3cd14ec]::util::run_in_thread_pool_with_globals<rustc_interface[85d830dfb3cd14ec]::interface::run_compiler<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}>::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>
  37:     0x7f05d8e551e8 - std[217bdc2af5db14e1]::panic::catch_unwind::<core[f78a9a4743325546]::panic::unwind_safe::AssertUnwindSafe<<std[217bdc2af5db14e1]::thread::Builder>::spawn_unchecked_<rustc_interface[85d830dfb3cd14ec]::util::run_in_thread_pool_with_globals<rustc_interface[85d830dfb3cd14ec]::interface::run_compiler<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}>::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>
  38:     0x7f05d8e0135a - <<std[217bdc2af5db14e1]::thread::Builder>::spawn_unchecked_<rustc_interface[85d830dfb3cd14ec]::util::run_in_thread_pool_with_globals<rustc_interface[85d830dfb3cd14ec]::interface::run_compiler<core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>, rustc_driver[61cdcd7c422ac744]::run_compiler::{closure#1}>::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f78a9a4743325546]::result::Result<(), rustc_errors[2a002e8cbda2c840]::ErrorGuaranteed>>::{closure#1} as core[f78a9a4743325546]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7f05d840e3ce - std::sys::unix::thread::Thread::new::thread_start::h459971eaa2b6a111
  40:     0x7f05d81a8b43 - <unknown>
  41:     0x7f05d823aa00 - <unknown>
  42:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (972111925 2022-10-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 2 previous errors
------------------------------------------
