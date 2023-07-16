plain
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s


running 14 tests
test [ui] ui-internal/if_chain_style.rs ... ok
test [ui] ui-internal/invalid_paths.rs ... ok
test [ui] ui-internal/unnecessary_def_path_hardcoded_path.rs ... FAILED
test [ui] ui-internal/default_deprecation_reason.rs ... ok
test [ui] ui-internal/default_lint.rs ... ok
test [ui] ui-internal/lint_without_lint_pass.rs ... ok
test [ui] ui-internal/check_clippy_version_attribute.rs ... ok
test [ui] ui-internal/interning_defined_symbol.rs ... ok
test [ui] ui-internal/unnecessary_symbol_str.rs ... ok
test [ui] ui-internal/invalid_msrv_attr_impl.rs ... ok
test [ui] ui-internal/outer_expn_data.rs ... ok
test [ui] ui-internal/custom_ice_message.rs ... FAILED
test [ui] ui-internal/unnecessary_def_path.rs ... ok
test [ui] ui-internal/collapsible_span_lint_calls.rs ... ok
failures:

failures:
    [ui] ui-internal/custom_ice_message.rs
---

-error: hardcoded path to a language item
-  --> $DIR/unnecessary_def_path_hardcoded_path.rs:11:40
-   |
-LL |     const DEREF_MUT_TRAIT: [&str; 4] = ["core", "ops", "deref", "DerefMut"];
-   |
-   |
-   = help: convert all references to use `LangItem::DerefMut`
-   = note: `-D clippy::unnecessary-def-path` implied by `-D warnings`
-
 error: hardcoded path to a diagnostic item
   --> $DIR/unnecessary_def_path_hardcoded_path.rs:10:36
    |
    |
 LL |     const DEREF_TRAIT: [&str; 4] = ["core", "ops", "deref", "Deref"];
    |
    = help: convert all references to use `sym::Deref`
    = help: convert all references to use `sym::Deref`
+   = note: `-D clippy::unnecessary-def-path` implied by `-D warnings`
 error: hardcoded path to a diagnostic item
   --> $DIR/unnecessary_def_path_hardcoded_path.rs:12:43
    |
    |
 LL |     const DEREF_TRAIT_METHOD: [&str; 5] = ["core", "ops", "deref", "Deref", "deref"];
    |
    |
    = help: convert all references to use `sym::deref_method`
+error: hardcoded path to a language item
+  --> $DIR/unnecessary_def_path_hardcoded_path.rs:11:40
+   |
+   |
+LL |     const DEREF_MUT_TRAIT: [&str; 4] = ["core", "ops", "deref", "DerefMut"];
+   |
+   |
+   = help: convert all references to use `LangItem::DerefMut`
 error: aborting due to 3 previous errors
 
 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-internal/unnecessary_def_path_hardcoded_path.stage-id.stderr
To only update this specific test, also pass `--test-args unnecessary_def_path_hardcoded_path.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui-internal/unnecessary_def_path_hardcoded_path.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-internal" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-internal/unnecessary_def_path_hardcoded_path.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-d4b0c12069551999.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a009da3a546d2d45.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-f03750e49676bd17.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-internal/unnecessary_def_path_hardcoded_path.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
{"message":"hardcoded path to a diagnostic item","code":{"code":"clippy::unnecessary_def_path","explanation":null},"level":"error","spans":[{"file_name":"tests/ui-internal/unnecessary_def_path_hardcoded_path.rs","byte_start":183,"byte_end":216,"line_start":10,"line_end":10,"column_start":36,"column_end":69,"is_primary":true,"text":[{"text":"    const DEREF_TRAIT: [&str; 4] = [\"core\", \"ops\", \"deref\", \"Deref\"];","highlight_start":36,"highlight_end":69}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"convert all references to use `sym::Deref`","code":null,"level":"help","spans":[],"children":[],"rendered":null},{"message":"`-D clippy::unnecessary-def-path` implied by `-D warnings`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error: hardcoded path to a diagnostic item\n  --> tests/ui-internal/unnecessary_def_path_hardcoded_path.rs:10:36\n   |\nLL |     const DEREF_TRAIT: [&str; 4] = [\"core\", \"ops\", \"deref\", \"Deref\"];\n   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: convert all references to use `sym::Deref`\n   = note: `-D clippy::unnecessary-def-path` implied by `-D warnings`\n\n"}
{"message":"hardcoded path to a diagnostic item","code":{"code":"clippy::unnecessary_def_path","explanation":null},"level":"error","spans":[{"file_name":"tests/ui-internal/unnecessary_def_path_hardcoded_path.rs","byte_start":337,"byte_end":379,"line_start":12,"line_end":12,"column_start":43,"column_end":85,"is_primary":true,"text":[{"text":"    const DEREF_TRAIT_METHOD: [&str; 5] = [\"core\", \"ops\", \"deref\", \"Deref\", \"deref\"];","highlight_start":43,"highlight_end":85}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"convert all references to use `sym::deref_method`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: hardcoded path to a diagnostic item\n  --> tests/ui-internal/unnecessary_def_path_hardcoded_path.rs:12:43\n   |\nLL |     const DEREF_TRAIT_METHOD: [&str; 5] = [\"core\", \"ops\", \"deref\", \"Deref\", \"deref\"];\n   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: convert all references to use `sym::deref_method`\n\n"}
{"message":"hardcoded path to a language item","code":{"code":"clippy::unnecessary_def_path","explanation":null},"level":"error","spans":[{"file_name":"tests/ui-internal/unnecessary_def_path_hardcoded_path.rs","byte_start":257,"byte_end":293,"line_start":11,"line_end":11,"column_start":40,"column_end":76,"is_primary":true,"text":[{"text":"    const DEREF_MUT_TRAIT: [&str; 4] = [\"core\", \"ops\", \"deref\", \"DerefMut\"];","highlight_start":40,"highlight_end":76}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"convert all references to use `LangItem::DerefMut`","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error: hardcoded path to a language item\n  --> tests/ui-internal/unnecessary_def_path_hardcoded_path.rs:11:40\n   |\nLL |     const DEREF_MUT_TRAIT: [&str; 4] = [\"core\", \"ops\", \"deref\", \"DerefMut\"];\n   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^\n   |\n   = help: convert all references to use `LangItem::DerefMut`\n\n"}

------------------------------------------

diff of stderr:
diff of stderr:

 thread '<unnamed>' panicked at 'Would you like some help with that?', clippy_lints/src/utils/internal_lints/produce_ice.rs:28:9
 
 error: internal compiler error: unexpected panic
 
 note: the compiler unexpectedly panicked. this is a bug.
 note: the compiler unexpectedly panicked. this is a bug.
 
 note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new
 
 note: Clippy version: foo
 
 query stack during panic:
 end of query stack
+thread '<unnamed>' panicked at 'Found a `push` without a `pop`.', compiler/rustc_lint/src/levels.rs:504:9
+stack backtrace:
+   0:     0x7f37023a4936 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf200c65b63063574
+   1:     0x7f3702412648 - core::fmt::write::h3ce1ce510a1434ee
+   2:     0x7f37023959c1 - std::io::Write::write_fmt::hc63c1603904c0576
+   3:     0x7f37023a4705 - std::sys_common::backtrace::print::h55827d8a9d5330a4
+   4:     0x7f37023a7ba7 - std::panicking::default_hook::{{closure}}::h84f4a3a1cd18964d
+   5:     0x7f37023a7906 - std::panicking::default_hook::h5e37b84e642d32d6
+   6:     0x5563289a75ff - clippy_driver[22aade832c9a811b]::ICE_HOOK::{closure#0}::{closure#0}
+   7:     0x7f37023a84d3 - std::panicking::rust_panic_with_hook::hbe93a99cd042be47
+   8:     0x7f37023a81c2 - std::panicking::begin_panic_handler::{{closure}}::h49f6cd0aae68bbd2
+   9:     0x7f37023a4edc - std::sys_common::backtrace::__rust_end_short_backtrace::h36a744de5c67bd92
+  10:     0x7f37023a7eb2 - rust_begin_unwind
+  11:     0x7f370235a133 - core::panicking::panic_fmt::hba328d3b0c424f87
+  12:     0x7f3705a1abe9 - <rustc_lint[461e14cb210291a]::levels::BuilderPush as core[aeda7df16bc3840c]::ops::drop::Drop>::drop
+  13:     0x7f3702fd1acd - <rustc_lint[461e14cb210291a]::early::EarlyContextAndPass<rustc_lint[461e14cb210291a]::early::EarlyLintPassObjects> as rustc_ast[4a96e9100c3088bc]::visit::Visitor>::visit_item
+  14:     0x7f3702f73fbc - rustc_ast[4a96e9100c3088bc]::visit::walk_crate::<rustc_lint[461e14cb210291a]::early::EarlyContextAndPass<rustc_lint[461e14cb210291a]::early::EarlyLintPassObjects>>
+  15:     0x7f3702fcfb4b - rustc_lint[461e14cb210291a]::early::early_lint_node::<rustc_lint[461e14cb210291a]::early::EarlyLintPassObjects, &rustc_ast[4a96e9100c3088bc]::ast::Crate>
+  16:     0x7f3702fcc4d9 - rustc_lint[461e14cb210291a]::early::check_ast_node::<rustc_lint[461e14cb210291a]::BuiltinCombinedEarlyLintPass, &rustc_ast[4a96e9100c3088bc]::ast::Crate>
+  17:     0x7f3702eeee6a - <rustc_session[ab24442121b565b4]::session::Session>::time::<(), rustc_interface[bbe20c2c484be1e6]::passes::configure_and_expand::{closure#8}>
+  18:     0x7f3702f23676 - rustc_interface[bbe20c2c484be1e6]::passes::configure_and_expand
+  19:     0x7f3702fa3a9d - <rustc_interface[bbe20c2c484be1e6]::queries::Queries>::expansion
+  20:     0x7f3702ddc6b5 - <rustc_interface[bbe20c2c484be1e6]::interface::Compiler>::enter::<rustc_driver[ccc124f1c83dbe2]::run_compiler::{closure#1}::{closure#2}, core[aeda7df16bc3840c]::result::Result<core[aeda7df16bc3840c]::option::Option<rustc_interface[bbe20c2c484be1e6]::queries::Linker>, rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>
+  21:     0x7f3702d566a1 - rustc_span[6f141704eba446bb]::with_source_map::<core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>, rustc_interface[bbe20c2c484be1e6]::interface::run_compiler<core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>, rustc_driver[ccc124f1c83dbe2]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
+  22:     0x7f3702ddd604 - <scoped_tls[900ea7f55d1ff37f]::ScopedKey<rustc_span[6f141704eba446bb]::SessionGlobals>>::set::<rustc_interface[bbe20c2c484be1e6]::interface::run_compiler<core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>, rustc_driver[ccc124f1c83dbe2]::run_compiler::{closure#1}>::{closure#0}, core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>
+  23:     0x7f3702d85bd0 - std[a1338642064fe5b1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bbe20c2c484be1e6]::util::run_in_thread_pool_with_globals<rustc_interface[bbe20c2c484be1e6]::interface::run_compiler<core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>, rustc_driver[ccc124f1c83dbe2]::run_compiler::{closure#1}>::{closure#0}, core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>
+  24:     0x7f3702d6b1f3 - <<std[a1338642064fe5b1]::thread::Builder>::spawn_unchecked_<rustc_interface[bbe20c2c484be1e6]::util::run_in_thread_pool_with_globals<rustc_interface[bbe20c2c484be1e6]::interface::run_compiler<core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>, rustc_driver[ccc124f1c83dbe2]::run_compiler::{closure#1}>::{closure#0}, core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>::{closure#1} as core[aeda7df16bc3840c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+  25:     0x7f37023b51ee - std::sys::unix::thread::Thread::new::thread_start::hb6ac43646bdea9ca
+  26:     0x7f3702042b43 - <unknown>
+  27:     0x7f37020d4a00 - <unknown>
+  28:                0x0 - <unknown>
+error: internal compiler error: unexpected panic
+
+note: the compiler unexpectedly panicked. this is a bug.
+
---
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args custom_ice_message.rs`

error: 1 errors occurred comparing output.
status: signal: 6 (SIGABRT) (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui-internal/custom_ice_message.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-internal" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-internal/custom_ice_message.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-d4b0c12069551999.rlib" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a009da3a546d2d45.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-f03750e49676bd17.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-internal/custom_ice_message.stage-id.aux"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
thread 'rustc' panicked at 'Would you like some help with that?', src/tools/clippy/clippy_lints/src/utils/internal_lints/produce_ice.rs:28:9

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.67 (4d35d878 2022-11-25)
query stack during panic:
end of query stack
end of query stack
thread 'rustc' panicked at 'Found a `push` without a `pop`.', compiler/rustc_lint/src/levels.rs:504:9
stack backtrace:
   0:     0x7f37023a4936 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf200c65b63063574
   1:     0x7f3702412648 - core::fmt::write::h3ce1ce510a1434ee
   2:     0x7f37023959c1 - std::io::Write::write_fmt::hc63c1603904c0576
   3:     0x7f37023a4705 - std::sys_common::backtrace::print::h55827d8a9d5330a4
   4:     0x7f37023a7ba7 - std::panicking::default_hook::{{closure}}::h84f4a3a1cd18964d
   5:     0x7f37023a7906 - std::panicking::default_hook::h5e37b84e642d32d6
   6:     0x5563289a75ff - clippy_driver[22aade832c9a811b]::ICE_HOOK::{closure#0}::{closure#0}
   7:     0x7f37023a84d3 - std::panicking::rust_panic_with_hook::hbe93a99cd042be47
   8:     0x7f37023a81c2 - std::panicking::begin_panic_handler::{{closure}}::h49f6cd0aae68bbd2
   9:     0x7f37023a4edc - std::sys_common::backtrace::__rust_end_short_backtrace::h36a744de5c67bd92
  10:     0x7f37023a7eb2 - rust_begin_unwind
  11:     0x7f370235a133 - core::panicking::panic_fmt::hba328d3b0c424f87
  12:     0x7f3705a1abe9 - <rustc_lint[461e14cb210291a]::levels::BuilderPush as core[aeda7df16bc3840c]::ops::drop::Drop>::drop
  13:     0x7f3702fd1acd - <rustc_lint[461e14cb210291a]::early::EarlyContextAndPass<rustc_lint[461e14cb210291a]::early::EarlyLintPassObjects> as rustc_ast[4a96e9100c3088bc]::visit::Visitor>::visit_item
  14:     0x7f3702f73fbc - rustc_ast[4a96e9100c3088bc]::visit::walk_crate::<rustc_lint[461e14cb210291a]::early::EarlyContextAndPass<rustc_lint[461e14cb210291a]::early::EarlyLintPassObjects>>
  15:     0x7f3702fcfb4b - rustc_lint[461e14cb210291a]::early::early_lint_node::<rustc_lint[461e14cb210291a]::early::EarlyLintPassObjects, &rustc_ast[4a96e9100c3088bc]::ast::Crate>
  16:     0x7f3702fcc4d9 - rustc_lint[461e14cb210291a]::early::check_ast_node::<rustc_lint[461e14cb210291a]::BuiltinCombinedEarlyLintPass, &rustc_ast[4a96e9100c3088bc]::ast::Crate>
  17:     0x7f3702eeee6a - <rustc_session[ab24442121b565b4]::session::Session>::time::<(), rustc_interface[bbe20c2c484be1e6]::passes::configure_and_expand::{closure#8}>
  18:     0x7f3702f23676 - rustc_interface[bbe20c2c484be1e6]::passes::configure_and_expand
  19:     0x7f3702fa3a9d - <rustc_interface[bbe20c2c484be1e6]::queries::Queries>::expansion
  20:     0x7f3702ddc6b5 - <rustc_interface[bbe20c2c484be1e6]::interface::Compiler>::enter::<rustc_driver[ccc124f1c83dbe2]::run_compiler::{closure#1}::{closure#2}, core[aeda7df16bc3840c]::result::Result<core[aeda7df16bc3840c]::option::Option<rustc_interface[bbe20c2c484be1e6]::queries::Linker>, rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>
  21:     0x7f3702d566a1 - rustc_span[6f141704eba446bb]::with_source_map::<core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>, rustc_interface[bbe20c2c484be1e6]::interface::run_compiler<core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>, rustc_driver[ccc124f1c83dbe2]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  22:     0x7f3702ddd604 - <scoped_tls[900ea7f55d1ff37f]::ScopedKey<rustc_span[6f141704eba446bb]::SessionGlobals>>::set::<rustc_interface[bbe20c2c484be1e6]::interface::run_compiler<core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>, rustc_driver[ccc124f1c83dbe2]::run_compiler::{closure#1}>::{closure#0}, core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>
  23:     0x7f3702d85bd0 - std[a1338642064fe5b1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[bbe20c2c484be1e6]::util::run_in_thread_pool_with_globals<rustc_interface[bbe20c2c484be1e6]::interface::run_compiler<core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>, rustc_driver[ccc124f1c83dbe2]::run_compiler::{closure#1}>::{closure#0}, core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>
  24:     0x7f3702d6b1f3 - <<std[a1338642064fe5b1]::thread::Builder>::spawn_unchecked_<rustc_interface[bbe20c2c484be1e6]::util::run_in_thread_pool_with_globals<rustc_interface[bbe20c2c484be1e6]::interface::run_compiler<core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>, rustc_driver[ccc124f1c83dbe2]::run_compiler::{closure#1}>::{closure#0}, core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[aeda7df16bc3840c]::result::Result<(), rustc_errors[174272472a4cdf32]::ErrorGuaranteed>>::{closure#1} as core[aeda7df16bc3840c]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7f37023b51ee - std::sys::unix::thread::Thread::new::thread_start::hb6ac43646bdea9ca
  26:     0x7f3702042b43 - <unknown>
  27:     0x7f37020d4a00 - <unknown>
  28:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.67 (4d35d878 2022-11-25)
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

