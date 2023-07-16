plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between 41e0363055ade59584cff667c79f64937e6ef3f9 and 3e812cc745400bd9edd227d181ad866aca6b1881
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s


running 14 tests
test [ui] ui-internal/invalid_paths.rs ... ok
test [ui] ui-internal/if_chain_style.rs ... ok
test [ui] ui-internal/unnecessary_def_path_hardcoded_path.rs ... ok
test [ui] ui-internal/default_deprecation_reason.rs ... ok
test [ui] ui-internal/default_lint.rs ... ok
test [ui] ui-internal/lint_without_lint_pass.rs ... ok
test [ui] ui-internal/check_clippy_version_attribute.rs ... ok
test [ui] ui-internal/interning_defined_symbol.rs ... ok
test [ui] ui-internal/unnecessary_symbol_str.rs ... ok
test [ui] ui-internal/custom_ice_message.rs ... FAILED
test [ui] ui-internal/invalid_msrv_attr_impl.rs ... ok
test [ui] ui-internal/collapsible_span_lint_calls.rs ... ok
test [ui] ui-internal/outer_expn_data.rs ... ok
test [ui] ui-internal/unnecessary_def_path.rs ... ok
failures:

failures:
    [ui] ui-internal/custom_ice_message.rs
---

---- compile_test stdout ----
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
+   0:     0x7fbe2c719936 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf5c53e27eabb6c92
+   1:     0x7fbe2c787648 - core::fmt::write::h9adcac22ca1ada35
+   2:     0x7fbe2c70a9c1 - std::io::Write::write_fmt::hfc1b8644e676f68a
+   3:     0x7fbe2c719705 - std::sys_common::backtrace::print::h324c23e4df4fd21b
+   4:     0x7fbe2c71cba7 - std::panicking::default_hook::{{closure}}::h0d9f2773bf75edd6
+   5:     0x7fbe2c71c906 - std::panicking::default_hook::h0296363bafb07503
+   6:     0x56119fabd5df - clippy_driver[8a5d313b41b41c68]::ICE_HOOK::{closure#0}::{closure#0}
+   7:     0x7fbe2c71d4d3 - std::panicking::rust_panic_with_hook::h5b6dcf711341e085
+   8:     0x7fbe2c71d1c2 - std::panicking::begin_panic_handler::{{closure}}::hc0bcdaae2dbdd5e4
+   9:     0x7fbe2c719edc - std::sys_common::backtrace::__rust_end_short_backtrace::hc4161379984d54e2
+  10:     0x7fbe2c71ceb2 - rust_begin_unwind
+  11:     0x7fbe2c6cf133 - core::panicking::panic_fmt::h03752014e655c065
+  12:     0x7fbe2fd933a9 - <rustc_lint[d9358ae84c43d93b]::levels::BuilderPush as core[68faba315474d546]::ops::drop::Drop>::drop
+  13:     0x7fbe2d34692d - <rustc_lint[d9358ae84c43d93b]::early::EarlyContextAndPass<rustc_lint[d9358ae84c43d93b]::early::EarlyLintPassObjects> as rustc_ast[a41204963f2f886f]::visit::Visitor>::visit_item
+  14:     0x7fbe2d2e900c - rustc_ast[a41204963f2f886f]::visit::walk_crate::<rustc_lint[d9358ae84c43d93b]::early::EarlyContextAndPass<rustc_lint[d9358ae84c43d93b]::early::EarlyLintPassObjects>>
+  15:     0x7fbe2d3449ab - rustc_lint[d9358ae84c43d93b]::early::early_lint_node::<rustc_lint[d9358ae84c43d93b]::early::EarlyLintPassObjects, &rustc_ast[a41204963f2f886f]::ast::Crate>
+  16:     0x7fbe2d341339 - rustc_lint[d9358ae84c43d93b]::early::check_ast_node::<rustc_lint[d9358ae84c43d93b]::BuiltinCombinedEarlyLintPass, &rustc_ast[a41204963f2f886f]::ast::Crate>
+  17:     0x7fbe2d263cda - <rustc_session[255004e973edb88b]::session::Session>::time::<(), rustc_interface[521b6230467a61d]::passes::configure_and_expand::{closure#8}>
+  18:     0x7fbe2d2986c6 - rustc_interface[521b6230467a61d]::passes::configure_and_expand
+  19:     0x7fbe2d318b5d - <rustc_interface[521b6230467a61d]::queries::Queries>::expansion
+  20:     0x7fbe2d1516d5 - <rustc_interface[521b6230467a61d]::interface::Compiler>::enter::<rustc_driver[59be94657ff5e7cd]::run_compiler::{closure#1}::{closure#2}, core[68faba315474d546]::result::Result<core[68faba315474d546]::option::Option<rustc_interface[521b6230467a61d]::queries::Linker>, rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>
+  21:     0x7fbe2d0cb651 - rustc_span[e4d44162dc051c57]::with_source_map::<core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>, rustc_interface[521b6230467a61d]::interface::run_compiler<core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>, rustc_driver[59be94657ff5e7cd]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
+  22:     0x7fbe2d152624 - <scoped_tls[a9f52a12973999a]::ScopedKey<rustc_span[e4d44162dc051c57]::SessionGlobals>>::set::<rustc_interface[521b6230467a61d]::interface::run_compiler<core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>, rustc_driver[59be94657ff5e7cd]::run_compiler::{closure#1}>::{closure#0}, core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>
+  23:     0x7fbe2d0fab40 - std[97d4460a24a895d5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[521b6230467a61d]::util::run_in_thread_pool_with_globals<rustc_interface[521b6230467a61d]::interface::run_compiler<core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>, rustc_driver[59be94657ff5e7cd]::run_compiler::{closure#1}>::{closure#0}, core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>
+  24:     0x7fbe2d0e0173 - <<std[97d4460a24a895d5]::thread::Builder>::spawn_unchecked_<rustc_interface[521b6230467a61d]::util::run_in_thread_pool_with_globals<rustc_interface[521b6230467a61d]::interface::run_compiler<core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>, rustc_driver[59be94657ff5e7cd]::run_compiler::{closure#1}>::{closure#0}, core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>::{closure#1} as core[68faba315474d546]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+  25:     0x7fbe2c72a1ee - std::sys::unix::thread::Thread::new::thread_start::h583a6f15dd367f1e
+  26:     0x7fbe2c3b7b43 - <unknown>
+  27:     0x7fbe2c449a00 - <unknown>
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
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/clippy-driver" "tests/ui-internal/custom_ice_message.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-internal" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-internal/custom_ice_message.stage-id" "-A" "unused" "--emit=metadata" "-Dwarnings" "-Zui-testing" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps" "-L" "dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps" "--extern" "syn=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libsyn-7b5cc279bd04eca8.rlib" "--extern" "tokio=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libtokio-e0524b7e2611e851.rlib" "--extern" "itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libitertools-b6f83e8bf7b1d2e3.rlib" "--extern" "rustc_semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/librustc_semver-963bbd3f89834643.rlib" "--extern" "parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libparking_lot-f03750e49676bd17.rlib" "--extern" "clippy_lints=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_lints-d4b0c12069551999.rlib" "--extern" "derive_new=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-a2f771a227dafeba.so" "--extern" "clippy_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libclippy_utils-a009da3a546d2d45.rlib" "--extern" "if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libif_chain-03f75cdc6d4d3afc.rlib" "--extern" "futures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libfutures-7a940bbcc1286cdc.rlib" "--extern" "serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libserde-edad3f22f7291185.rlib" "--extern" "serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-f7c308c5f8e7b09c.so" "--extern" "regex=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libregex-619ac20e364f2b2c.rlib" "--extern" "quote=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/libquote-e39b631048351f36.rlib" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/test/ui-internal/custom_ice_message.stage-id.aux"
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

note: Clippy version: clippy 0.1.67 (3e812cc7 2022-11-25)
query stack during panic:
end of query stack
end of query stack
thread 'rustc' panicked at 'Found a `push` without a `pop`.', compiler/rustc_lint/src/levels.rs:504:9
stack backtrace:
   0:     0x7fbe2c719936 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf5c53e27eabb6c92
   1:     0x7fbe2c787648 - core::fmt::write::h9adcac22ca1ada35
   2:     0x7fbe2c70a9c1 - std::io::Write::write_fmt::hfc1b8644e676f68a
   3:     0x7fbe2c719705 - std::sys_common::backtrace::print::h324c23e4df4fd21b
   4:     0x7fbe2c71cba7 - std::panicking::default_hook::{{closure}}::h0d9f2773bf75edd6
   5:     0x7fbe2c71c906 - std::panicking::default_hook::h0296363bafb07503
   6:     0x56119fabd5df - clippy_driver[8a5d313b41b41c68]::ICE_HOOK::{closure#0}::{closure#0}
   7:     0x7fbe2c71d4d3 - std::panicking::rust_panic_with_hook::h5b6dcf711341e085
   8:     0x7fbe2c71d1c2 - std::panicking::begin_panic_handler::{{closure}}::hc0bcdaae2dbdd5e4
   9:     0x7fbe2c719edc - std::sys_common::backtrace::__rust_end_short_backtrace::hc4161379984d54e2
  10:     0x7fbe2c71ceb2 - rust_begin_unwind
  11:     0x7fbe2c6cf133 - core::panicking::panic_fmt::h03752014e655c065
  12:     0x7fbe2fd933a9 - <rustc_lint[d9358ae84c43d93b]::levels::BuilderPush as core[68faba315474d546]::ops::drop::Drop>::drop
  13:     0x7fbe2d34692d - <rustc_lint[d9358ae84c43d93b]::early::EarlyContextAndPass<rustc_lint[d9358ae84c43d93b]::early::EarlyLintPassObjects> as rustc_ast[a41204963f2f886f]::visit::Visitor>::visit_item
  14:     0x7fbe2d2e900c - rustc_ast[a41204963f2f886f]::visit::walk_crate::<rustc_lint[d9358ae84c43d93b]::early::EarlyContextAndPass<rustc_lint[d9358ae84c43d93b]::early::EarlyLintPassObjects>>
  15:     0x7fbe2d3449ab - rustc_lint[d9358ae84c43d93b]::early::early_lint_node::<rustc_lint[d9358ae84c43d93b]::early::EarlyLintPassObjects, &rustc_ast[a41204963f2f886f]::ast::Crate>
  16:     0x7fbe2d341339 - rustc_lint[d9358ae84c43d93b]::early::check_ast_node::<rustc_lint[d9358ae84c43d93b]::BuiltinCombinedEarlyLintPass, &rustc_ast[a41204963f2f886f]::ast::Crate>
  17:     0x7fbe2d263cda - <rustc_session[255004e973edb88b]::session::Session>::time::<(), rustc_interface[521b6230467a61d]::passes::configure_and_expand::{closure#8}>
  18:     0x7fbe2d2986c6 - rustc_interface[521b6230467a61d]::passes::configure_and_expand
  19:     0x7fbe2d318b5d - <rustc_interface[521b6230467a61d]::queries::Queries>::expansion
  20:     0x7fbe2d1516d5 - <rustc_interface[521b6230467a61d]::interface::Compiler>::enter::<rustc_driver[59be94657ff5e7cd]::run_compiler::{closure#1}::{closure#2}, core[68faba315474d546]::result::Result<core[68faba315474d546]::option::Option<rustc_interface[521b6230467a61d]::queries::Linker>, rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>
  21:     0x7fbe2d0cb651 - rustc_span[e4d44162dc051c57]::with_source_map::<core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>, rustc_interface[521b6230467a61d]::interface::run_compiler<core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>, rustc_driver[59be94657ff5e7cd]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  22:     0x7fbe2d152624 - <scoped_tls[a9f52a12973999a]::ScopedKey<rustc_span[e4d44162dc051c57]::SessionGlobals>>::set::<rustc_interface[521b6230467a61d]::interface::run_compiler<core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>, rustc_driver[59be94657ff5e7cd]::run_compiler::{closure#1}>::{closure#0}, core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>
  23:     0x7fbe2d0fab40 - std[97d4460a24a895d5]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[521b6230467a61d]::util::run_in_thread_pool_with_globals<rustc_interface[521b6230467a61d]::interface::run_compiler<core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>, rustc_driver[59be94657ff5e7cd]::run_compiler::{closure#1}>::{closure#0}, core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>
  24:     0x7fbe2d0e0173 - <<std[97d4460a24a895d5]::thread::Builder>::spawn_unchecked_<rustc_interface[521b6230467a61d]::util::run_in_thread_pool_with_globals<rustc_interface[521b6230467a61d]::interface::run_compiler<core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>, rustc_driver[59be94657ff5e7cd]::run_compiler::{closure#1}>::{closure#0}, core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[68faba315474d546]::result::Result<(), rustc_errors[18491cffa1bdadc5]::ErrorGuaranteed>>::{closure#1} as core[68faba315474d546]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7fbe2c72a1ee - std::sys::unix::thread::Thread::new::thread_start::h583a6f15dd367f1e
  26:     0x7fbe2c3b7b43 - <unknown>
  27:     0x7fbe2c449a00 - <unknown>
  28:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust-clippy/issues/new

note: Clippy version: clippy 0.1.67 (3e812cc7 2022-11-25)
query stack during panic:
end of query stack
thread panicked while panicking. aborting.

