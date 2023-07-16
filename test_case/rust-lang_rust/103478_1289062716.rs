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
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
help: remove the parentheses
   |
   |
LL - fn foo2(_: &dyn (Drop + AsRef<str>)) {} //~ ERROR incorrect braces around trait bounds
LL + fn foo2(_: &dyn Drop + AsRef<str>) {} //~ ERROR incorrect braces around trait bounds

error: expected parameter name, found `{`
  --> /checkout/src/test/ui/parser/trait-object-delimiters.rs:8:17
   |
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
   0:     0x7f1114df05de - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h17daff433a1ff788
   1:     0x7f1114e59ad8 - core::fmt::write::hc0fd288bf2ceb977
   2:     0x7f1114de1c51 - std::io::Write::write_fmt::h49be0f18253c0ef3
   3:     0x7f1114df03e1 - std::sys_common::backtrace::print::h317705132ee4178a
   4:     0x7f1114df3554 - std::panicking::default_hook::{{closure}}::h8ce38fddfee1f10b
   5:     0x7f1114df3219 - std::panicking::default_hook::h86799dd00af80e6b
   6:     0x7f11157e0ca4 - rustc_driver[2703185df305da9e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f1114df3ca4 - std::panicking::rust_panic_with_hook::h3ed259353bc1f706
   8:     0x7f1114df39c9 - std::panicking::begin_panic_handler::{{closure}}::hdbce1f04822f2659
   9:     0x7f1114df0b14 - std::sys_common::backtrace::__rust_end_short_backtrace::h86067e3d6e1d9313
  10:     0x7f1114df36d2 - rust_begin_unwind
  11:     0x7f1114da4aa3 - core::panicking::panic_fmt::hac7f5d0aa93f5ec8
  12:     0x7f111869cfbf - <rustc_errors[8b39a9b26a05cfac]::diagnostic_builder::DiagnosticBuilderInner as core[3bde5789d4361c5f]::ops::drop::Drop>::drop
  13:     0x7f1117cf08ee - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::expected_ident_found
  14:     0x7f1117d750c8 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_ident_common
  15:     0x7f1117d5ea85 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_path_segment
  16:     0x7f1117d5e742 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_path_segments
  17:     0x7f1117d5e3cf - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_path_inner
  18:     0x7f1117d71af1 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_generic_bounds_common
  19:     0x7f1117d70c35 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_dyn_ty
  20:     0x7f1117d6db79 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_ty_common
  21:     0x7f1117d6fd79 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_borrowed_pointee
  22:     0x7f1117d6c728 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_ty_common
  23:     0x7f1117d4fef6 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_param_general::{closure#0}
  24:     0x7f1117d4dfe2 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_fn_params::{closure#0}
  25:     0x7f1117d4d518 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_fn_decl
  26:     0x7f1117d4ae99 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_fn
  27:     0x7f1117d350ac - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_item_common
  28:     0x7f1117d3413e - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_mod
  29:     0x7f1117d33d58 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_crate_mod
  30:     0x7f1117dd3fdd - rustc_parse[5c71ceb78daebaa5]::parse_crate_from_file
  31:     0x7f111590dc35 - <rustc_session[f5c7a819ad4b9a5c]::session::Session>::time::<core[3bde5789d4361c5f]::result::Result<rustc_ast[16861c8d093ee98e]::ast::Crate, rustc_errors[8b39a9b26a05cfac]::diagnostic_builder::DiagnosticBuilder<rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>, rustc_interface[ef6ae0326aa47169]::passes::parse::{closure#0}>
  32:     0x7f111593b657 - rustc_interface[ef6ae0326aa47169]::passes::parse
  33:     0x7f111592b595 - <rustc_interface[ef6ae0326aa47169]::queries::Queries>::parse
  34:     0x7f11158515aa - <rustc_interface[ef6ae0326aa47169]::interface::Compiler>::enter::<rustc_driver[2703185df305da9e]::run_compiler::{closure#1}::{closure#2}, core[3bde5789d4361c5f]::result::Result<core[3bde5789d4361c5f]::option::Option<rustc_interface[ef6ae0326aa47169]::queries::Linker>, rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>
  35:     0x7f11157e23ee - rustc_span[115ae281b71134e8]::with_source_map::<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_interface[ef6ae0326aa47169]::interface::run_compiler<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_driver[2703185df305da9e]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  36:     0x7f111584516c - <scoped_tls[ea6ac4ff9a002ba7]::ScopedKey<rustc_span[115ae281b71134e8]::SessionGlobals>>::set::<rustc_interface[ef6ae0326aa47169]::interface::run_compiler<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_driver[2703185df305da9e]::run_compiler::{closure#1}>::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>
  37:     0x7f1115801939 - std[e4e4ae12747e695c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ef6ae0326aa47169]::util::run_in_thread_pool_with_globals<rustc_interface[ef6ae0326aa47169]::interface::run_compiler<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_driver[2703185df305da9e]::run_compiler::{closure#1}>::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>
  38:     0x7f111584a438 - std[e4e4ae12747e695c]::panic::catch_unwind::<core[3bde5789d4361c5f]::panic::unwind_safe::AssertUnwindSafe<<std[e4e4ae12747e695c]::thread::Builder>::spawn_unchecked_<rustc_interface[ef6ae0326aa47169]::util::run_in_thread_pool_with_globals<rustc_interface[ef6ae0326aa47169]::interface::run_compiler<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_driver[2703185df305da9e]::run_compiler::{closure#1}>::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>
  39:     0x7f11157f241a - <<std[e4e4ae12747e695c]::thread::Builder>::spawn_unchecked_<rustc_interface[ef6ae0326aa47169]::util::run_in_thread_pool_with_globals<rustc_interface[ef6ae0326aa47169]::interface::run_compiler<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_driver[2703185df305da9e]::run_compiler::{closure#1}>::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>::{closure#1} as core[3bde5789d4361c5f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  40:     0x7f1114e003ce - std::sys::unix::thread::Thread::new::thread_start::h6c270bdaa6e48575
  41:     0x7f1114b9ab43 - <unknown>
  42:     0x7f1114c2ca00 - <unknown>
  43:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (8798974b7 2022-10-24) running on x86_64-unknown-linux-gnu

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
   0:     0x7ff36a74e5de - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h17daff433a1ff788
   1:     0x7ff36a7b7ad8 - core::fmt::write::hc0fd288bf2ceb977
   2:     0x7ff36a73fc51 - std::io::Write::write_fmt::h49be0f18253c0ef3
   3:     0x7ff36a74e3e1 - std::sys_common::backtrace::print::h317705132ee4178a
   4:     0x7ff36a751554 - std::panicking::default_hook::{{closure}}::h8ce38fddfee1f10b
   5:     0x7ff36a751219 - std::panicking::default_hook::h86799dd00af80e6b
   6:     0x7ff36b13eca4 - rustc_driver[2703185df305da9e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7ff36a751ca4 - std::panicking::rust_panic_with_hook::h3ed259353bc1f706
   8:     0x7ff36a7519c9 - std::panicking::begin_panic_handler::{{closure}}::hdbce1f04822f2659
   9:     0x7ff36a74eb14 - std::sys_common::backtrace::__rust_end_short_backtrace::h86067e3d6e1d9313
  10:     0x7ff36a7516d2 - rust_begin_unwind
  11:     0x7ff36a702aa3 - core::panicking::panic_fmt::hac7f5d0aa93f5ec8
  12:     0x7ff36dffafbf - <rustc_errors[8b39a9b26a05cfac]::diagnostic_builder::DiagnosticBuilderInner as core[3bde5789d4361c5f]::ops::drop::Drop>::drop
  13:     0x7ff36d64e8ee - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::expected_ident_found
  14:     0x7ff36d6d30c8 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_ident_common
  15:     0x7ff36d6bca85 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_path_segment
  16:     0x7ff36d6bc84c - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_path_segments
  17:     0x7ff36d6bc3cf - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_path_inner
  18:     0x7ff36d66b523 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_assoc_op_cast
  19:     0x7ff36d665374 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_assoc_expr_with
  20:     0x7ff36d6c613d - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_local
  21:     0x7ff36d6c1dd0 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_stmt_without_recovery
  22:     0x7ff36d6c838a - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_full_stmt
  23:     0x7ff36d6c804f - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_block_tail
  24:     0x7ff36d6c7f37 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_block_common
  25:     0x7ff36d6a9018 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_fn
  26:     0x7ff36d6930ac - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_item_common
  27:     0x7ff36d69213e - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_mod
  28:     0x7ff36d691d58 - <rustc_parse[5c71ceb78daebaa5]::parser::Parser>::parse_crate_mod
  29:     0x7ff36d731fdd - rustc_parse[5c71ceb78daebaa5]::parse_crate_from_file
  30:     0x7ff36b26bc35 - <rustc_session[f5c7a819ad4b9a5c]::session::Session>::time::<core[3bde5789d4361c5f]::result::Result<rustc_ast[16861c8d093ee98e]::ast::Crate, rustc_errors[8b39a9b26a05cfac]::diagnostic_builder::DiagnosticBuilder<rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>, rustc_interface[ef6ae0326aa47169]::passes::parse::{closure#0}>
  31:     0x7ff36b299657 - rustc_interface[ef6ae0326aa47169]::passes::parse
  32:     0x7ff36b289595 - <rustc_interface[ef6ae0326aa47169]::queries::Queries>::parse
  33:     0x7ff36b1af5aa - <rustc_interface[ef6ae0326aa47169]::interface::Compiler>::enter::<rustc_driver[2703185df305da9e]::run_compiler::{closure#1}::{closure#2}, core[3bde5789d4361c5f]::result::Result<core[3bde5789d4361c5f]::option::Option<rustc_interface[ef6ae0326aa47169]::queries::Linker>, rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>
  34:     0x7ff36b1403ee - rustc_span[115ae281b71134e8]::with_source_map::<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_interface[ef6ae0326aa47169]::interface::run_compiler<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_driver[2703185df305da9e]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  35:     0x7ff36b1a316c - <scoped_tls[ea6ac4ff9a002ba7]::ScopedKey<rustc_span[115ae281b71134e8]::SessionGlobals>>::set::<rustc_interface[ef6ae0326aa47169]::interface::run_compiler<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_driver[2703185df305da9e]::run_compiler::{closure#1}>::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>
  36:     0x7ff36b15f939 - std[e4e4ae12747e695c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ef6ae0326aa47169]::util::run_in_thread_pool_with_globals<rustc_interface[ef6ae0326aa47169]::interface::run_compiler<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_driver[2703185df305da9e]::run_compiler::{closure#1}>::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>
  37:     0x7ff36b1a8438 - std[e4e4ae12747e695c]::panic::catch_unwind::<core[3bde5789d4361c5f]::panic::unwind_safe::AssertUnwindSafe<<std[e4e4ae12747e695c]::thread::Builder>::spawn_unchecked_<rustc_interface[ef6ae0326aa47169]::util::run_in_thread_pool_with_globals<rustc_interface[ef6ae0326aa47169]::interface::run_compiler<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_driver[2703185df305da9e]::run_compiler::{closure#1}>::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>
  38:     0x7ff36b15041a - <<std[e4e4ae12747e695c]::thread::Builder>::spawn_unchecked_<rustc_interface[ef6ae0326aa47169]::util::run_in_thread_pool_with_globals<rustc_interface[ef6ae0326aa47169]::interface::run_compiler<core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>, rustc_driver[2703185df305da9e]::run_compiler::{closure#1}>::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[3bde5789d4361c5f]::result::Result<(), rustc_errors[8b39a9b26a05cfac]::ErrorGuaranteed>>::{closure#1} as core[3bde5789d4361c5f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  39:     0x7ff36a75e3ce - std::sys::unix::thread::Thread::new::thread_start::h6c270bdaa6e48575
  40:     0x7ff36a4f8b43 - <unknown>
  41:     0x7ff36a58aa00 - <unknown>
  42:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (8798974b7 2022-10-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 2 previous errors
------------------------------------------
