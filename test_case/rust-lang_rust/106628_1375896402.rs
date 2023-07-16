plain
---- [rustdoc] src/test/rustdoc/intra-doc/libstd-re-export.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/libstd-re-export/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/intra-doc/libstd-re-export" "--deny" "warnings" "/checkout/src/test/rustdoc/intra-doc/libstd-re-export.rs"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: value <= 0xFFFF_FF00', /checkout/compiler/rustc_span/src/def_id.rs:192:1
stack backtrace:
   0:     0x7effa0813215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h10e4db1ec3f355da
   1:     0x7effa0883468 - core::fmt::write::h1090ae415765daf7
   2:     0x7effa08052d1 - std::io::Write::write_fmt::h39f5ecb0d26672be
   3:     0x7effa0813021 - std::sys_common::backtrace::print::h73e77945ed0b05cf
   4:     0x7effa0816404 - std::panicking::default_hook::{{closure}}::hac26a6aa9712d493
   5:     0x7effa08160ca - std::panicking::default_hook::h5a018c7ac75b9f75
   6:     0x7effa1262052 - rustc_driver[5e5c2919ff35889]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7effa0816b74 - std::panicking::rust_panic_with_hook::h297673d33afd75ef
   8:     0x7effa081689a - std::panicking::begin_panic_handler::{{closure}}::h7c7c9bab0f1a7fa4
   9:     0x7effa0813734 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7d590a98b9be244
  10:     0x7effa0816582 - rust_begin_unwind
  11:     0x7effa07c6fd3 - core::panicking::panic_fmt::hb629c9ddd0e58892
  12:     0x7effa07c706d - core::panicking::panic::h09adfb5f8eabd29c
  13:     0x55e28924cbac - rustdoc[de13de2484d26160]::clean::inline::build_module_items
  14:     0x55e28924c5d3 - rustdoc[de13de2484d26160]::clean::inline::build_module
  15:     0x55e289245471 - rustdoc[de13de2484d26160]::clean::inline::try_inline
  16:     0x55e28924ca5d - rustdoc[de13de2484d26160]::clean::inline::build_module_items
  17:     0x55e289246076 - rustdoc[de13de2484d26160]::clean::inline::try_inline_glob
  18:     0x55e28905914e - rustdoc[de13de2484d26160]::clean::clean_use_statement
  19:     0x55e289045979 - <&mut rustdoc[de13de2484d26160]::clean::clean_doc_module::{closure#3} as core[f3d2147ea7d94ff]::ops::function::FnOnce<(&(&rustc_hir[48e0ac5cf7b25fb3]::hir::Item, core[f3d2147ea7d94ff]::option::Option<rustc_span[e9024fe076906892]::symbol::Symbol>, core[f3d2147ea7d94ff]::option::Option<rustc_hir[48e0ac5cf7b25fb3]::hir_id::HirId>),)>>::call_once
  20:     0x55e28913be5e - <alloc[b52485c4c3259fff]::vec::Vec<rustdoc[de13de2484d26160]::clean::types::Item> as alloc[b52485c4c3259fff]::vec::spec_extend::SpecExtend<rustdoc[de13de2484d26160]::clean::types::Item, core[f3d2147ea7d94ff]::iter::adapters::flatten::FlatMap<core[f3d2147ea7d94ff]::slice::iter::Iter<(&rustc_hir[48e0ac5cf7b25fb3]::hir::Item, core[f3d2147ea7d94ff]::option::Option<rustc_span[e9024fe076906892]::symbol::Symbol>, core[f3d2147ea7d94ff]::option::Option<rustc_hir[48e0ac5cf7b25fb3]::hir_id::HirId>)>, alloc[b52485c4c3259fff]::vec::Vec<rustdoc[de13de2484d26160]::clean::types::Item>, rustdoc[de13de2484d26160]::clean::clean_doc_module::{closure#3}>>>::spec_extend
  21:     0x55e28904a456 - rustdoc[de13de2484d26160]::clean::clean_doc_module
  22:     0x55e28924f901 - rustdoc[de13de2484d26160]::clean::utils::krate
  23:     0x55e288fc1d74 - <rustc_session[f6740c4190efb142]::session::Session>::time::<rustdoc[de13de2484d26160]::clean::types::Crate, rustdoc[de13de2484d26160]::core::run_global_ctxt::{closure#4}>
  24:     0x55e288f9af2a - rustdoc[de13de2484d26160]::core::run_global_ctxt
  25:     0x55e288fc4d20 - <rustc_interface[89b05f6e7e544cbd]::passes::QueryContext>::enter::<rustdoc[de13de2484d26160]::main_args::{closure#1}::{closure#0}::{closure#1}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>
  26:     0x55e2891ad822 - <rustc_interface[89b05f6e7e544cbd]::interface::Compiler>::enter::<rustdoc[de13de2484d26160]::main_args::{closure#1}::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>
  27:     0x55e288f6e4ae - rustc_span[e9024fe076906892]::with_source_map::<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustc_interface[89b05f6e7e544cbd]::interface::run_compiler<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustdoc[de13de2484d26160]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  28:     0x55e288ed7174 - <scoped_tls[8c8eef33dd19b896]::ScopedKey<rustc_span[e9024fe076906892]::SessionGlobals>>::set::<rustc_interface[89b05f6e7e544cbd]::interface::run_compiler<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustdoc[de13de2484d26160]::main_args::{closure#1}>::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>
  29:     0x55e288f7d999 - std[115b1322c35af2b3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[89b05f6e7e544cbd]::util::run_in_thread_pool_with_globals<rustc_interface[89b05f6e7e544cbd]::interface::run_compiler<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustdoc[de13de2484d26160]::main_args::{closure#1}>::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>
  30:     0x55e288f27d18 - std[115b1322c35af2b3]::panic::catch_unwind::<core[f3d2147ea7d94ff]::panic::unwind_safe::AssertUnwindSafe<<std[115b1322c35af2b3]::thread::Builder>::spawn_unchecked_<rustc_interface[89b05f6e7e544cbd]::util::run_in_thread_pool_with_globals<rustc_interface[89b05f6e7e544cbd]::interface::run_compiler<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustdoc[de13de2484d26160]::main_args::{closure#1}>::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>
  31:     0x55e2890c92b7 - <<std[115b1322c35af2b3]::thread::Builder>::spawn_unchecked_<rustc_interface[89b05f6e7e544cbd]::util::run_in_thread_pool_with_globals<rustc_interface[89b05f6e7e544cbd]::interface::run_compiler<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustdoc[de13de2484d26160]::main_args::{closure#1}>::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>::{closure#1} as core[f3d2147ea7d94ff]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7effa082397e - std::sys::unix::thread::Thread::new::thread_start::h7972d211b8c18e2d
  33:     0x7effa04afb43 - <unknown>
  34:     0x7effa0541a00 - <unknown>
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

---
---- [rustdoc] src/test/rustdoc/primitive-reexport.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-reexport/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/primitive-reexport" "--deny" "warnings" "/checkout/src/test/rustdoc/primitive-reexport.rs" "--extern" "foo" "--edition" "2018"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'assertion failed: value <= 0xFFFF_FF00', /checkout/compiler/rustc_span/src/def_id.rs:192:1
   0:     0x7f7225422215 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h10e4db1ec3f355da
   1:     0x7f7225492468 - core::fmt::write::h1090ae415765daf7
   2:     0x7f72254142d1 - std::io::Write::write_fmt::h39f5ecb0d26672be
   3:     0x7f7225422021 - std::sys_common::backtrace::print::h73e77945ed0b05cf
   3:     0x7f7225422021 - std::sys_common::backtrace::print::h73e77945ed0b05cf
   4:     0x7f7225425404 - std::panicking::default_hook::{{closure}}::hac26a6aa9712d493
   5:     0x7f72254250ca - std::panicking::default_hook::h5a018c7ac75b9f75
   6:     0x7f7225e71052 - rustc_driver[5e5c2919ff35889]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f7225425b74 - std::panicking::rust_panic_with_hook::h297673d33afd75ef
   8:     0x7f722542589a - std::panicking::begin_panic_handler::{{closure}}::h7c7c9bab0f1a7fa4
   9:     0x7f7225422734 - std::sys_common::backtrace::__rust_end_short_backtrace::hf7d590a98b9be244
  10:     0x7f7225425582 - rust_begin_unwind
  11:     0x7f72253d5fd3 - core::panicking::panic_fmt::hb629c9ddd0e58892
  12:     0x7f72253d606d - core::panicking::panic::h09adfb5f8eabd29c
  13:     0x56039a100bac - rustdoc[de13de2484d26160]::clean::inline::build_module_items
  14:     0x56039a0fa076 - rustdoc[de13de2484d26160]::clean::inline::try_inline_glob
  15:     0x560399f0d14e - rustdoc[de13de2484d26160]::clean::clean_use_statement
  16:     0x560399ef9979 - <&mut rustdoc[de13de2484d26160]::clean::clean_doc_module::{closure#3} as core[f3d2147ea7d94ff]::ops::function::FnOnce<(&(&rustc_hir[48e0ac5cf7b25fb3]::hir::Item, core[f3d2147ea7d94ff]::option::Option<rustc_span[e9024fe076906892]::symbol::Symbol>, core[f3d2147ea7d94ff]::option::Option<rustc_hir[48e0ac5cf7b25fb3]::hir_id::HirId>),)>>::call_once
  17:     0x560399fefe5e - <alloc[b52485c4c3259fff]::vec::Vec<rustdoc[de13de2484d26160]::clean::types::Item> as alloc[b52485c4c3259fff]::vec::spec_extend::SpecExtend<rustdoc[de13de2484d26160]::clean::types::Item, core[f3d2147ea7d94ff]::iter::adapters::flatten::FlatMap<core[f3d2147ea7d94ff]::slice::iter::Iter<(&rustc_hir[48e0ac5cf7b25fb3]::hir::Item, core[f3d2147ea7d94ff]::option::Option<rustc_span[e9024fe076906892]::symbol::Symbol>, core[f3d2147ea7d94ff]::option::Option<rustc_hir[48e0ac5cf7b25fb3]::hir_id::HirId>)>, alloc[b52485c4c3259fff]::vec::Vec<rustdoc[de13de2484d26160]::clean::types::Item>, rustdoc[de13de2484d26160]::clean::clean_doc_module::{closure#3}>>>::spec_extend
  18:     0x560399efe456 - rustdoc[de13de2484d26160]::clean::clean_doc_module
  19:     0x560399ef8d0d - <&mut rustdoc[de13de2484d26160]::clean::clean_doc_module::{closure#1} as core[f3d2147ea7d94ff]::ops::function::FnMut<(&rustdoc[de13de2484d26160]::visit_ast::Module,)>>::call_mut
  20:     0x560399feedce - <alloc[b52485c4c3259fff]::vec::Vec<rustdoc[de13de2484d26160]::clean::types::Item> as alloc[b52485c4c3259fff]::vec::spec_extend::SpecExtend<rustdoc[de13de2484d26160]::clean::types::Item, core[f3d2147ea7d94ff]::iter::adapters::filter_map::FilterMap<core[f3d2147ea7d94ff]::slice::iter::Iter<rustdoc[de13de2484d26160]::visit_ast::Module>, rustdoc[de13de2484d26160]::clean::clean_doc_module::{closure#1}>>>::spec_extend
  21:     0x560399efe32c - rustdoc[de13de2484d26160]::clean::clean_doc_module
  22:     0x56039a103901 - rustdoc[de13de2484d26160]::clean::utils::krate
  23:     0x560399e75d74 - <rustc_session[f6740c4190efb142]::session::Session>::time::<rustdoc[de13de2484d26160]::clean::types::Crate, rustdoc[de13de2484d26160]::core::run_global_ctxt::{closure#4}>
  24:     0x560399e4ef2a - rustdoc[de13de2484d26160]::core::run_global_ctxt
  25:     0x560399e78d20 - <rustc_interface[89b05f6e7e544cbd]::passes::QueryContext>::enter::<rustdoc[de13de2484d26160]::main_args::{closure#1}::{closure#0}::{closure#1}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>
  26:     0x56039a061822 - <rustc_interface[89b05f6e7e544cbd]::interface::Compiler>::enter::<rustdoc[de13de2484d26160]::main_args::{closure#1}::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>
  27:     0x560399e224ae - rustc_span[e9024fe076906892]::with_source_map::<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustc_interface[89b05f6e7e544cbd]::interface::run_compiler<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustdoc[de13de2484d26160]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  28:     0x560399d8b174 - <scoped_tls[8c8eef33dd19b896]::ScopedKey<rustc_span[e9024fe076906892]::SessionGlobals>>::set::<rustc_interface[89b05f6e7e544cbd]::interface::run_compiler<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustdoc[de13de2484d26160]::main_args::{closure#1}>::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>
  29:     0x560399e31999 - std[115b1322c35af2b3]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[89b05f6e7e544cbd]::util::run_in_thread_pool_with_globals<rustc_interface[89b05f6e7e544cbd]::interface::run_compiler<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustdoc[de13de2484d26160]::main_args::{closure#1}>::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>
  30:     0x560399ddbd18 - std[115b1322c35af2b3]::panic::catch_unwind::<core[f3d2147ea7d94ff]::panic::unwind_safe::AssertUnwindSafe<<std[115b1322c35af2b3]::thread::Builder>::spawn_unchecked_<rustc_interface[89b05f6e7e544cbd]::util::run_in_thread_pool_with_globals<rustc_interface[89b05f6e7e544cbd]::interface::run_compiler<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustdoc[de13de2484d26160]::main_args::{closure#1}>::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>
  31:     0x560399f7d2b7 - <<std[115b1322c35af2b3]::thread::Builder>::spawn_unchecked_<rustc_interface[89b05f6e7e544cbd]::util::run_in_thread_pool_with_globals<rustc_interface[89b05f6e7e544cbd]::interface::run_compiler<core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>, rustdoc[de13de2484d26160]::main_args::{closure#1}>::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[f3d2147ea7d94ff]::result::Result<(), rustc_errors[bb970c7044ef841d]::ErrorGuaranteed>>::{closure#1} as core[f3d2147ea7d94ff]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  32:     0x7f722543297e - std::sys::unix::thread::Thread::new::thread_start::h7972d211b8c18e2d
  33:     0x7f72250beb43 - <unknown>
  34:     0x7f7225150a00 - <unknown>
  35:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

