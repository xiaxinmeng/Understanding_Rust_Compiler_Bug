plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:055e3b93d15803815fe6f9cbc1b02b11be094e54)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
    Finished release [optimized] target(s) in 24.60s
   Compiling cc v1.0.77
    Checking core v0.0.0 (/checkout/library/core)
   Compiling compiler_builtins v0.1.85
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1459:79
   0:     0x7efeed8ad8b5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he103eeb2bbb684e0
   1:     0x7efeed91df48 - core::fmt::write::hffc1f7dc98740e11
   2:     0x7efeed89f761 - std::io::Write::write_fmt::h09caeec5f3c075fc
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   4:     0x7efeed8b0aa4 - std::panicking::default_hook::{{closure}}::hb11e2e0340a28ed4
   5:     0x7efeed8b076a - std::panicking::default_hook::h480165ce5b7d0aef
   6:     0x7efeee34c812 - rustc_driver[650d564c7ad3dd1f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efeed8b1211 - std::panicking::rust_panic_with_hook::hd7fcb25083b61e45
   8:     0x7efeed8b0f79 - std::panicking::begin_panic_handler::{{closure}}::h041b00ac53f6a5dc
   9:     0x7efeed8addd4 - std::sys_common::backtrace::__rust_end_short_backtrace::h18845e3014e66c2d
  10:     0x7efeed8b0c22 - rust_begin_unwind
  11:     0x7efeed85ffe3 - core::panicking::panic_fmt::h637520fb6ac3c163
  12:     0x7efeeedaa388 - <rustc_data_structures[51da988687265f1f]::steal::Steal<rustc_middle[ee929c9ad1643fe]::mir::Body>>::borrow
  13:     0x7efeeed9ecbe - rustc_monomorphize[3508bc0439feaf57]::collector::collect_neighbours
  14:     0x7efeeed9ac38 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  15:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  16:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  17:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  18:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  19:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  20:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  21:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  22:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  23:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  24:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  25:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  26:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  27:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  28:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  29:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  30:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  31:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  32:     0x7efeeedb11b1 - <core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once
  33:     0x7efeeeda8fb5 - std[793ec1942fc995f8]::panicking::try::<(), core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  34:     0x7efeeedd2e34 - <rustc_session[8e07f96b924def24]::session::Session>::time::<(), rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}>
  35:     0x7efeeed979d7 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items
  36:     0x7efeeede455a - rustc_monomorphize[3508bc0439feaf57]::partitioning::collect_crate_mono_items_for_check
  37:     0x7efef0225f39 - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::try_execute_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt>
  38:     0x7efef02de2de - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::get_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt, rustc_middle[ee929c9ad1643fe]::dep_graph::dep_node::DepKind>
  39:     0x7efeeffe8a4a - <rustc_query_impl[3b6a55090664e5b0]::Queries as rustc_middle[ee929c9ad1643fe]::ty::query::QueryEngine>::collect_crate_mono_items_for_check
  40:     0x7efeee3d7820 - <rustc_interface[99b99fa288b2f649]::passes::QueryContext>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  41:     0x7efeee3ba68d - <rustc_interface[99b99fa288b2f649]::interface::Compiler>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}, core[de783dfd7be79b4e]::result::Result<core[de783dfd7be79b4e]::option::Option<rustc_interface[99b99fa288b2f649]::queries::Linker>, rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  42:     0x7efeee33a3a7 - rustc_span[9dd1deed9f8beba4]::with_source_map::<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  43:     0x7efeee3b4a5f - std[793ec1942fc995f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  44:     0x7efeee3aff08 - std[793ec1942fc995f8]::panic::catch_unwind::<core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  45:     0x7efeee35a2a7 - <<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1} as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  46:     0x7efeed8be17e - std::sys::unix::thread::Thread::new::thread_start::h47297d3d7ca257ac
  47:     0x7efeed651b43 - <unknown>
  48:     0x7efeed6e3a00 - <unknown>
  49:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (25b48e698 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph
end of query stack
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1459:79
   0:     0x7efeed8ad8b5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he103eeb2bbb684e0
   1:     0x7efeed91df48 - core::fmt::write::hffc1f7dc98740e11
   2:     0x7efeed89f761 - std::io::Write::write_fmt::h09caeec5f3c075fc
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   4:     0x7efeed8b0aa4 - std::panicking::default_hook::{{closure}}::hb11e2e0340a28ed4
   5:     0x7efeed8b076a - std::panicking::default_hook::h480165ce5b7d0aef
   6:     0x7efeee34c812 - rustc_driver[650d564c7ad3dd1f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efeed8b1211 - std::panicking::rust_panic_with_hook::hd7fcb25083b61e45
   8:     0x7efeed8b0f79 - std::panicking::begin_panic_handler::{{closure}}::h041b00ac53f6a5dc
   9:     0x7efeed8addd4 - std::sys_common::backtrace::__rust_end_short_backtrace::h18845e3014e66c2d
  10:     0x7efeed8b0c22 - rust_begin_unwind
  11:     0x7efeed85ffe3 - core::panicking::panic_fmt::h637520fb6ac3c163
  12:     0x7efeeedaa388 - <rustc_data_structures[51da988687265f1f]::steal::Steal<rustc_middle[ee929c9ad1643fe]::mir::Body>>::borrow
  13:     0x7efeeed9ecbe - rustc_monomorphize[3508bc0439feaf57]::collector::collect_neighbours
  14:     0x7efeeed9ac38 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  15:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  16:     0x7efeeedb11b1 - <core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once
  17:     0x7efeeeda8fb5 - std[793ec1942fc995f8]::panicking::try::<(), core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  18:     0x7efeeedd2e34 - <rustc_session[8e07f96b924def24]::session::Session>::time::<(), rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}>
  19:     0x7efeeed979d7 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items
  20:     0x7efeeede455a - rustc_monomorphize[3508bc0439feaf57]::partitioning::collect_crate_mono_items_for_check
  21:     0x7efef0225f39 - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::try_execute_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt>
  22:     0x7efef02de2de - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::get_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt, rustc_middle[ee929c9ad1643fe]::dep_graph::dep_node::DepKind>
  23:     0x7efeeffe8a4a - <rustc_query_impl[3b6a55090664e5b0]::Queries as rustc_middle[ee929c9ad1643fe]::ty::query::QueryEngine>::collect_crate_mono_items_for_check
  24:     0x7efeee3d7820 - <rustc_interface[99b99fa288b2f649]::passes::QueryContext>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  25:     0x7efeee3ba68d - <rustc_interface[99b99fa288b2f649]::interface::Compiler>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}, core[de783dfd7be79b4e]::result::Result<core[de783dfd7be79b4e]::option::Option<rustc_interface[99b99fa288b2f649]::queries::Linker>, rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  26:     0x7efeee33a3a7 - rustc_span[9dd1deed9f8beba4]::with_source_map::<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  27:     0x7efeee3b4a5f - std[793ec1942fc995f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  28:     0x7efeee3aff08 - std[793ec1942fc995f8]::panic::catch_unwind::<core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  29:     0x7efeee35a2a7 - <<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1} as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7efeed8be17e - std::sys::unix::thread::Thread::new::thread_start::h47297d3d7ca257ac
  31:     0x7efeed651b43 - <unknown>
  32:     0x7efeed6e3a00 - <unknown>
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (25b48e698 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph
end of query stack
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1459:79
   0:     0x7efeed8ad8b5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he103eeb2bbb684e0
   1:     0x7efeed91df48 - core::fmt::write::hffc1f7dc98740e11
   2:     0x7efeed89f761 - std::io::Write::write_fmt::h09caeec5f3c075fc
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   4:     0x7efeed8b0aa4 - std::panicking::default_hook::{{closure}}::hb11e2e0340a28ed4
   5:     0x7efeed8b076a - std::panicking::default_hook::h480165ce5b7d0aef
   6:     0x7efeee34c812 - rustc_driver[650d564c7ad3dd1f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efeed8b1211 - std::panicking::rust_panic_with_hook::hd7fcb25083b61e45
   8:     0x7efeed8b0f79 - std::panicking::begin_panic_handler::{{closure}}::h041b00ac53f6a5dc
   9:     0x7efeed8addd4 - std::sys_common::backtrace::__rust_end_short_backtrace::h18845e3014e66c2d
  10:     0x7efeed8b0c22 - rust_begin_unwind
  11:     0x7efeed85ffe3 - core::panicking::panic_fmt::h637520fb6ac3c163
  12:     0x7efeeedaa388 - <rustc_data_structures[51da988687265f1f]::steal::Steal<rustc_middle[ee929c9ad1643fe]::mir::Body>>::borrow
  13:     0x7efeeed9ecbe - rustc_monomorphize[3508bc0439feaf57]::collector::collect_neighbours
  14:     0x7efeeed9ac38 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  15:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  16:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  17:     0x7efeeedb11b1 - <core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once
  18:     0x7efeeeda8fb5 - std[793ec1942fc995f8]::panicking::try::<(), core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  19:     0x7efeeedd2e34 - <rustc_session[8e07f96b924def24]::session::Session>::time::<(), rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}>
  20:     0x7efeeed979d7 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items
  21:     0x7efeeede455a - rustc_monomorphize[3508bc0439feaf57]::partitioning::collect_crate_mono_items_for_check
  22:     0x7efef0225f39 - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::try_execute_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt>
  23:     0x7efef02de2de - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::get_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt, rustc_middle[ee929c9ad1643fe]::dep_graph::dep_node::DepKind>
  24:     0x7efeeffe8a4a - <rustc_query_impl[3b6a55090664e5b0]::Queries as rustc_middle[ee929c9ad1643fe]::ty::query::QueryEngine>::collect_crate_mono_items_for_check
  25:     0x7efeee3d7820 - <rustc_interface[99b99fa288b2f649]::passes::QueryContext>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  26:     0x7efeee3ba68d - <rustc_interface[99b99fa288b2f649]::interface::Compiler>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}, core[de783dfd7be79b4e]::result::Result<core[de783dfd7be79b4e]::option::Option<rustc_interface[99b99fa288b2f649]::queries::Linker>, rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  27:     0x7efeee33a3a7 - rustc_span[9dd1deed9f8beba4]::with_source_map::<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  28:     0x7efeee3b4a5f - std[793ec1942fc995f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  29:     0x7efeee3aff08 - std[793ec1942fc995f8]::panic::catch_unwind::<core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  30:     0x7efeee35a2a7 - <<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1} as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7efeed8be17e - std::sys::unix::thread::Thread::new::thread_start::h47297d3d7ca257ac
  32:     0x7efeed651b43 - <unknown>
  33:     0x7efeed6e3a00 - <unknown>
  34:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (25b48e698 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph
end of query stack
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1459:79
   0:     0x7efeed8ad8b5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he103eeb2bbb684e0
   1:     0x7efeed91df48 - core::fmt::write::hffc1f7dc98740e11
   2:     0x7efeed89f761 - std::io::Write::write_fmt::h09caeec5f3c075fc
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   4:     0x7efeed8b0aa4 - std::panicking::default_hook::{{closure}}::hb11e2e0340a28ed4
   5:     0x7efeed8b076a - std::panicking::default_hook::h480165ce5b7d0aef
   6:     0x7efeee34c812 - rustc_driver[650d564c7ad3dd1f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efeed8b1211 - std::panicking::rust_panic_with_hook::hd7fcb25083b61e45
   8:     0x7efeed8b0f79 - std::panicking::begin_panic_handler::{{closure}}::h041b00ac53f6a5dc
   9:     0x7efeed8addd4 - std::sys_common::backtrace::__rust_end_short_backtrace::h18845e3014e66c2d
  10:     0x7efeed8b0c22 - rust_begin_unwind
  11:     0x7efeed85ffe3 - core::panicking::panic_fmt::h637520fb6ac3c163
  12:     0x7efeeedaa388 - <rustc_data_structures[51da988687265f1f]::steal::Steal<rustc_middle[ee929c9ad1643fe]::mir::Body>>::borrow
  13:     0x7efeeed9ecbe - rustc_monomorphize[3508bc0439feaf57]::collector::collect_neighbours
  14:     0x7efeeed9ac38 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  15:     0x7efeeedb11b1 - <core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once
  16:     0x7efeeeda8fb5 - std[793ec1942fc995f8]::panicking::try::<(), core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  17:     0x7efeeedd2e34 - <rustc_session[8e07f96b924def24]::session::Session>::time::<(), rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}>
  18:     0x7efeeed979d7 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items
  19:     0x7efeeede455a - rustc_monomorphize[3508bc0439feaf57]::partitioning::collect_crate_mono_items_for_check
  20:     0x7efef0225f39 - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::try_execute_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt>
  21:     0x7efef02de2de - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::get_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt, rustc_middle[ee929c9ad1643fe]::dep_graph::dep_node::DepKind>
  22:     0x7efeeffe8a4a - <rustc_query_impl[3b6a55090664e5b0]::Queries as rustc_middle[ee929c9ad1643fe]::ty::query::QueryEngine>::collect_crate_mono_items_for_check
  23:     0x7efeee3d7820 - <rustc_interface[99b99fa288b2f649]::passes::QueryContext>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  24:     0x7efeee3ba68d - <rustc_interface[99b99fa288b2f649]::interface::Compiler>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}, core[de783dfd7be79b4e]::result::Result<core[de783dfd7be79b4e]::option::Option<rustc_interface[99b99fa288b2f649]::queries::Linker>, rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  25:     0x7efeee33a3a7 - rustc_span[9dd1deed9f8beba4]::with_source_map::<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  26:     0x7efeee3b4a5f - std[793ec1942fc995f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  27:     0x7efeee3aff08 - std[793ec1942fc995f8]::panic::catch_unwind::<core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  28:     0x7efeee35a2a7 - <<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1} as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  29:     0x7efeed8be17e - std::sys::unix::thread::Thread::new::thread_start::h47297d3d7ca257ac
  30:     0x7efeed651b43 - <unknown>
  31:     0x7efeed6e3a00 - <unknown>
  32:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (25b48e698 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph
end of query stack
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1459:79
   0:     0x7efeed8ad8b5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he103eeb2bbb684e0
   1:     0x7efeed91df48 - core::fmt::write::hffc1f7dc98740e11
   2:     0x7efeed89f761 - std::io::Write::write_fmt::h09caeec5f3c075fc
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   4:     0x7efeed8b0aa4 - std::panicking::default_hook::{{closure}}::hb11e2e0340a28ed4
   5:     0x7efeed8b076a - std::panicking::default_hook::h480165ce5b7d0aef
   6:     0x7efeee34c812 - rustc_driver[650d564c7ad3dd1f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efeed8b1211 - std::panicking::rust_panic_with_hook::hd7fcb25083b61e45
   8:     0x7efeed8b0f79 - std::panicking::begin_panic_handler::{{closure}}::h041b00ac53f6a5dc
   9:     0x7efeed8addd4 - std::sys_common::backtrace::__rust_end_short_backtrace::h18845e3014e66c2d
  10:     0x7efeed8b0c22 - rust_begin_unwind
  11:     0x7efeed85ffe3 - core::panicking::panic_fmt::h637520fb6ac3c163
  12:     0x7efeeedaa388 - <rustc_data_structures[51da988687265f1f]::steal::Steal<rustc_middle[ee929c9ad1643fe]::mir::Body>>::borrow
  13:     0x7efeeed9ecbe - rustc_monomorphize[3508bc0439feaf57]::collector::collect_neighbours
  14:     0x7efeeed9ac38 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  15:     0x7efeeedb11b1 - <core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once
  16:     0x7efeeeda8fb5 - std[793ec1942fc995f8]::panicking::try::<(), core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  17:     0x7efeeedd2e34 - <rustc_session[8e07f96b924def24]::session::Session>::time::<(), rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}>
  18:     0x7efeeed979d7 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items
  19:     0x7efeeede455a - rustc_monomorphize[3508bc0439feaf57]::partitioning::collect_crate_mono_items_for_check
  20:     0x7efef0225f39 - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::try_execute_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt>
  21:     0x7efef02de2de - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::get_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt, rustc_middle[ee929c9ad1643fe]::dep_graph::dep_node::DepKind>
  22:     0x7efeeffe8a4a - <rustc_query_impl[3b6a55090664e5b0]::Queries as rustc_middle[ee929c9ad1643fe]::ty::query::QueryEngine>::collect_crate_mono_items_for_check
  23:     0x7efeee3d7820 - <rustc_interface[99b99fa288b2f649]::passes::QueryContext>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  24:     0x7efeee3ba68d - <rustc_interface[99b99fa288b2f649]::interface::Compiler>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}, core[de783dfd7be79b4e]::result::Result<core[de783dfd7be79b4e]::option::Option<rustc_interface[99b99fa288b2f649]::queries::Linker>, rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  25:     0x7efeee33a3a7 - rustc_span[9dd1deed9f8beba4]::with_source_map::<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  26:     0x7efeee3b4a5f - std[793ec1942fc995f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  27:     0x7efeee3aff08 - std[793ec1942fc995f8]::panic::catch_unwind::<core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  28:     0x7efeee35a2a7 - <<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1} as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  29:     0x7efeed8be17e - std::sys::unix::thread::Thread::new::thread_start::h47297d3d7ca257ac
  30:     0x7efeed651b43 - <unknown>
  31:     0x7efeed6e3a00 - <unknown>
  32:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (25b48e698 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph
end of query stack
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1459:79
   0:     0x7efeed8ad8b5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he103eeb2bbb684e0
   1:     0x7efeed91df48 - core::fmt::write::hffc1f7dc98740e11
   2:     0x7efeed89f761 - std::io::Write::write_fmt::h09caeec5f3c075fc
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   4:     0x7efeed8b0aa4 - std::panicking::default_hook::{{closure}}::hb11e2e0340a28ed4
   5:     0x7efeed8b076a - std::panicking::default_hook::h480165ce5b7d0aef
   6:     0x7efeee34c812 - rustc_driver[650d564c7ad3dd1f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efeed8b1211 - std::panicking::rust_panic_with_hook::hd7fcb25083b61e45
   8:     0x7efeed8b0f79 - std::panicking::begin_panic_handler::{{closure}}::h041b00ac53f6a5dc
   9:     0x7efeed8addd4 - std::sys_common::backtrace::__rust_end_short_backtrace::h18845e3014e66c2d
  10:     0x7efeed8b0c22 - rust_begin_unwind
  11:     0x7efeed85ffe3 - core::panicking::panic_fmt::h637520fb6ac3c163
  12:     0x7efeeedaa388 - <rustc_data_structures[51da988687265f1f]::steal::Steal<rustc_middle[ee929c9ad1643fe]::mir::Body>>::borrow
  13:     0x7efeeed9ecbe - rustc_monomorphize[3508bc0439feaf57]::collector::collect_neighbours
  14:     0x7efeeed9ac38 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  15:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  16:     0x7efeeedb11b1 - <core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once
  17:     0x7efeeeda8fb5 - std[793ec1942fc995f8]::panicking::try::<(), core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  18:     0x7efeeedd2e34 - <rustc_session[8e07f96b924def24]::session::Session>::time::<(), rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}>
  19:     0x7efeeed979d7 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items
  20:     0x7efeeede455a - rustc_monomorphize[3508bc0439feaf57]::partitioning::collect_crate_mono_items_for_check
  21:     0x7efef0225f39 - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::try_execute_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt>
  22:     0x7efef02de2de - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::get_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt, rustc_middle[ee929c9ad1643fe]::dep_graph::dep_node::DepKind>
  23:     0x7efeeffe8a4a - <rustc_query_impl[3b6a55090664e5b0]::Queries as rustc_middle[ee929c9ad1643fe]::ty::query::QueryEngine>::collect_crate_mono_items_for_check
  24:     0x7efeee3d7820 - <rustc_interface[99b99fa288b2f649]::passes::QueryContext>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  25:     0x7efeee3ba68d - <rustc_interface[99b99fa288b2f649]::interface::Compiler>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}, core[de783dfd7be79b4e]::result::Result<core[de783dfd7be79b4e]::option::Option<rustc_interface[99b99fa288b2f649]::queries::Linker>, rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  26:     0x7efeee33a3a7 - rustc_span[9dd1deed9f8beba4]::with_source_map::<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  27:     0x7efeee3b4a5f - std[793ec1942fc995f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  28:     0x7efeee3aff08 - std[793ec1942fc995f8]::panic::catch_unwind::<core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  29:     0x7efeee35a2a7 - <<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1} as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7efeed8be17e - std::sys::unix::thread::Thread::new::thread_start::h47297d3d7ca257ac
  31:     0x7efeed651b43 - <unknown>
  32:     0x7efeed6e3a00 - <unknown>
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (25b48e698 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph
end of query stack
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1459:79
   0:     0x7efeed8ad8b5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he103eeb2bbb684e0
   1:     0x7efeed91df48 - core::fmt::write::hffc1f7dc98740e11
   2:     0x7efeed89f761 - std::io::Write::write_fmt::h09caeec5f3c075fc
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   4:     0x7efeed8b0aa4 - std::panicking::default_hook::{{closure}}::hb11e2e0340a28ed4
   5:     0x7efeed8b076a - std::panicking::default_hook::h480165ce5b7d0aef
   6:     0x7efeee34c812 - rustc_driver[650d564c7ad3dd1f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efeed8b1211 - std::panicking::rust_panic_with_hook::hd7fcb25083b61e45
   8:     0x7efeed8b0f79 - std::panicking::begin_panic_handler::{{closure}}::h041b00ac53f6a5dc
   9:     0x7efeed8addd4 - std::sys_common::backtrace::__rust_end_short_backtrace::h18845e3014e66c2d
  10:     0x7efeed8b0c22 - rust_begin_unwind
  11:     0x7efeed85ffe3 - core::panicking::panic_fmt::h637520fb6ac3c163
  12:     0x7efeeedaa388 - <rustc_data_structures[51da988687265f1f]::steal::Steal<rustc_middle[ee929c9ad1643fe]::mir::Body>>::borrow
  13:     0x7efeeed9ecbe - rustc_monomorphize[3508bc0439feaf57]::collector::collect_neighbours
  14:     0x7efeeed9ac38 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  15:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  16:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  17:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  18:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  19:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  20:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  21:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  22:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  23:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  24:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  25:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  26:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  27:     0x7efeeedb11b1 - <core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once
  28:     0x7efeeeda8fb5 - std[793ec1942fc995f8]::panicking::try::<(), core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  29:     0x7efeeedd2e34 - <rustc_session[8e07f96b924def24]::session::Session>::time::<(), rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}>
  30:     0x7efeeed979d7 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items
  31:     0x7efeeede455a - rustc_monomorphize[3508bc0439feaf57]::partitioning::collect_crate_mono_items_for_check
  32:     0x7efef0225f39 - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::try_execute_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt>
  33:     0x7efef02de2de - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::get_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt, rustc_middle[ee929c9ad1643fe]::dep_graph::dep_node::DepKind>
  34:     0x7efeeffe8a4a - <rustc_query_impl[3b6a55090664e5b0]::Queries as rustc_middle[ee929c9ad1643fe]::ty::query::QueryEngine>::collect_crate_mono_items_for_check
  35:     0x7efeee3d7820 - <rustc_interface[99b99fa288b2f649]::passes::QueryContext>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  36:     0x7efeee3ba68d - <rustc_interface[99b99fa288b2f649]::interface::Compiler>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}, core[de783dfd7be79b4e]::result::Result<core[de783dfd7be79b4e]::option::Option<rustc_interface[99b99fa288b2f649]::queries::Linker>, rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  37:     0x7efeee33a3a7 - rustc_span[9dd1deed9f8beba4]::with_source_map::<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  38:     0x7efeee3b4a5f - std[793ec1942fc995f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  39:     0x7efeee3aff08 - std[793ec1942fc995f8]::panic::catch_unwind::<core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  40:     0x7efeee35a2a7 - <<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1} as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7efeed8be17e - std::sys::unix::thread::Thread::new::thread_start::h47297d3d7ca257ac
  42:     0x7efeed651b43 - <unknown>
  43:     0x7efeed6e3a00 - <unknown>
  44:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (25b48e698 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph
end of query stack
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1459:79
   0:     0x7efeed8ad8b5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he103eeb2bbb684e0
   1:     0x7efeed91df48 - core::fmt::write::hffc1f7dc98740e11
   2:     0x7efeed89f761 - std::io::Write::write_fmt::h09caeec5f3c075fc
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   4:     0x7efeed8b0aa4 - std::panicking::default_hook::{{closure}}::hb11e2e0340a28ed4
   5:     0x7efeed8b076a - std::panicking::default_hook::h480165ce5b7d0aef
   6:     0x7efeee34c812 - rustc_driver[650d564c7ad3dd1f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efeed8b1211 - std::panicking::rust_panic_with_hook::hd7fcb25083b61e45
   8:     0x7efeed8b0f79 - std::panicking::begin_panic_handler::{{closure}}::h041b00ac53f6a5dc
   9:     0x7efeed8addd4 - std::sys_common::backtrace::__rust_end_short_backtrace::h18845e3014e66c2d
  10:     0x7efeed8b0c22 - rust_begin_unwind
  11:     0x7efeed85ffe3 - core::panicking::panic_fmt::h637520fb6ac3c163
  12:     0x7efeeedaa388 - <rustc_data_structures[51da988687265f1f]::steal::Steal<rustc_middle[ee929c9ad1643fe]::mir::Body>>::borrow
  13:     0x7efeeed9ecbe - rustc_monomorphize[3508bc0439feaf57]::collector::collect_neighbours
  14:     0x7efeeed9ac38 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  15:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  16:     0x7efeeedb11b1 - <core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once
  17:     0x7efeeeda8fb5 - std[793ec1942fc995f8]::panicking::try::<(), core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  18:     0x7efeeedd2e34 - <rustc_session[8e07f96b924def24]::session::Session>::time::<(), rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}>
  19:     0x7efeeed979d7 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items
  20:     0x7efeeede455a - rustc_monomorphize[3508bc0439feaf57]::partitioning::collect_crate_mono_items_for_check
  21:     0x7efef0225f39 - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::try_execute_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt>
  22:     0x7efef02de2de - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::get_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt, rustc_middle[ee929c9ad1643fe]::dep_graph::dep_node::DepKind>
  23:     0x7efeeffe8a4a - <rustc_query_impl[3b6a55090664e5b0]::Queries as rustc_middle[ee929c9ad1643fe]::ty::query::QueryEngine>::collect_crate_mono_items_for_check
  24:     0x7efeee3d7820 - <rustc_interface[99b99fa288b2f649]::passes::QueryContext>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  25:     0x7efeee3ba68d - <rustc_interface[99b99fa288b2f649]::interface::Compiler>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}, core[de783dfd7be79b4e]::result::Result<core[de783dfd7be79b4e]::option::Option<rustc_interface[99b99fa288b2f649]::queries::Linker>, rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  26:     0x7efeee33a3a7 - rustc_span[9dd1deed9f8beba4]::with_source_map::<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  27:     0x7efeee3b4a5f - std[793ec1942fc995f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  28:     0x7efeee3aff08 - std[793ec1942fc995f8]::panic::catch_unwind::<core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  29:     0x7efeee35a2a7 - <<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1} as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  30:     0x7efeed8be17e - std::sys::unix::thread::Thread::new::thread_start::h47297d3d7ca257ac
  31:     0x7efeed651b43 - <unknown>
  32:     0x7efeed6e3a00 - <unknown>
  33:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (25b48e698 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [collect_crate_mono_items_for_check] monomorphize the crate graph
end of query stack
thread 'rustc' panicked at 'attempted to read from stolen value: rustc_middle::mir::Body', compiler/rustc_monomorphize/src/collector.rs:1459:79
   0:     0x7efeed8ad8b5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he103eeb2bbb684e0
   1:     0x7efeed91df48 - core::fmt::write::hffc1f7dc98740e11
   2:     0x7efeed89f761 - std::io::Write::write_fmt::h09caeec5f3c075fc
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   3:     0x7efeed8ad6c1 - std::sys_common::backtrace::print::hf0a1710da6041581
   4:     0x7efeed8b0aa4 - std::panicking::default_hook::{{closure}}::hb11e2e0340a28ed4
   5:     0x7efeed8b076a - std::panicking::default_hook::h480165ce5b7d0aef
   6:     0x7efeee34c812 - rustc_driver[650d564c7ad3dd1f]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7efeed8b1211 - std::panicking::rust_panic_with_hook::hd7fcb25083b61e45
   8:     0x7efeed8b0f79 - std::panicking::begin_panic_handler::{{closure}}::h041b00ac53f6a5dc
   9:     0x7efeed8addd4 - std::sys_common::backtrace::__rust_end_short_backtrace::h18845e3014e66c2d
  10:     0x7efeed8b0c22 - rust_begin_unwind
  11:     0x7efeed85ffe3 - core::panicking::panic_fmt::h637520fb6ac3c163
  12:     0x7efeeedaa388 - <rustc_data_structures[51da988687265f1f]::steal::Steal<rustc_middle[ee929c9ad1643fe]::mir::Body>>::borrow
  13:     0x7efeeed9ecbe - rustc_monomorphize[3508bc0439feaf57]::collector::collect_neighbours
  14:     0x7efeeed9ac38 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  15:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  16:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  17:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  18:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  19:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  20:     0x7efeeed9bbd0 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_items_rec
  21:     0x7efeeedb11b1 - <core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}> as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once
  22:     0x7efeeeda8fb5 - std[793ec1942fc995f8]::panicking::try::<(), core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[51da988687265f1f]::sync::par_for_each_in<alloc[f8ae7fcebb1a15fe]::vec::Vec<rustc_middle[ee929c9ad1643fe]::mir::mono::MonoItem>, rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}::{closure#0}>::{closure#0}::{closure#0}>>
  23:     0x7efeeedd2e34 - <rustc_session[8e07f96b924def24]::session::Session>::time::<(), rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items::{closure#1}>
  24:     0x7efeeed979d7 - rustc_monomorphize[3508bc0439feaf57]::collector::collect_crate_mono_items
  25:     0x7efeeede455a - rustc_monomorphize[3508bc0439feaf57]::partitioning::collect_crate_mono_items_for_check
  26:     0x7efef0225f39 - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::try_execute_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt>
  27:     0x7efef02de2de - rustc_query_system[f535aa3e5c3893f5]::query::plumbing::get_query::<rustc_query_impl[3b6a55090664e5b0]::queries::collect_crate_mono_items_for_check, rustc_query_impl[3b6a55090664e5b0]::plumbing::QueryCtxt, rustc_middle[ee929c9ad1643fe]::dep_graph::dep_node::DepKind>
  28:     0x7efeeffe8a4a - <rustc_query_impl[3b6a55090664e5b0]::Queries as rustc_middle[ee929c9ad1643fe]::ty::query::QueryEngine>::collect_crate_mono_items_for_check
  29:     0x7efeee3d7820 - <rustc_interface[99b99fa288b2f649]::passes::QueryContext>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  30:     0x7efeee3ba68d - <rustc_interface[99b99fa288b2f649]::interface::Compiler>::enter::<rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}::{closure#2}, core[de783dfd7be79b4e]::result::Result<core[de783dfd7be79b4e]::option::Option<rustc_interface[99b99fa288b2f649]::queries::Linker>, rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  31:     0x7efeee33a3a7 - rustc_span[9dd1deed9f8beba4]::with_source_map::<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  32:     0x7efeee3b4a5f - std[793ec1942fc995f8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  33:     0x7efeee3aff08 - std[793ec1942fc995f8]::panic::catch_unwind::<core[de783dfd7be79b4e]::panic::unwind_safe::AssertUnwindSafe<<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>
  34:     0x7efeee35a2a7 - <<std[793ec1942fc995f8]::thread::Builder>::spawn_unchecked_<rustc_interface[99b99fa288b2f649]::util::run_in_thread_pool_with_globals<rustc_interface[99b99fa288b2f649]::interface::run_compiler<core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>, rustc_driver[650d564c7ad3dd1f]::run_compiler::{closure#1}>::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[de783dfd7be79b4e]::result::Result<(), rustc_errors[fb3a15d20253aff9]::ErrorGuaranteed>>::{closure#1} as core[de783dfd7be79b4e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  35:     0x7efeed8be17e - std::sys::unix::thread::Thread::new::thread_start::h47297d3d7ca257ac
  36:     0x7efeed651b43 - <unknown>
  37:     0x7efeed6e3a00 - <unknown>
  38:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (25b48e698 2023-01-31) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
