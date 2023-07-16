`
thread 'rustc' panicked at 'called `Result::unwrap()` on an `Err` value: TryFromIntError(())', compiler/rustc_middle/src/query/on_disk_cache.rs:304:70
stack backtrace:
   0:     0x7f535b7f06e1 - std::backtrace_rs::backtrace::libunwind::trace::h4354896f1663baaf
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f535b7f06e1 - std::backtrace_rs::backtrace::trace_unsynchronized::h8ac49f89c23585dd
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f535b7f06e1 - std::sys_common::backtrace::_print_fmt::h9f5f16b3ef080000
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f535b7f06e1 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hcfed927151d1ad83
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f535b850bdf - core::fmt::rt::Argument::fmt::h12fb43eea2fe23a8
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/core/src/fmt/rt.rs:138:9
   5:     0x7f535b850bdf - core::fmt::write::hcf94a34baaaea06f
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/core/src/fmt/mod.rs:1094:21
   6:     0x7f535b7e3941 - std::io::Write::write_fmt::h60003491edc2e074
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/io/mod.rs:1712:15
   7:     0x7f535b7f04f5 - std::sys_common::backtrace::_print::h9a8311322b9d8667
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/sys_common/backtrace.rs:47:5
   8:     0x7f535b7f04f5 - std::sys_common::backtrace::print::h8503eaeeae92ea08
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/sys_common/backtrace.rs:34:9
   9:     0x7f535b7f3177 - std::panicking::default_hook::{{closure}}::h36a123b73b99c0f3
  10:     0x7f535b7f2f64 - std::panicking::default_hook::ha2efb9fc5f628e61
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/panicking.rs:288:9
  11:     0x7f535a6c87db - rustc_driver_impl[64b7dd1911194932]::install_ice_hook::{closure#0}
  12:     0x7f535b7f3897 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h250e911c4e8d011a
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/alloc/src/boxed.rs:1999:9
  13:     0x7f535b7f3897 - std::panicking::rust_panic_with_hook::h5d9e02f555bc48d2
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/panicking.rs:695:13
  14:     0x7f535b7f3617 - std::panicking::begin_panic_handler::{{closure}}::hbb00bd8fdd3417d9
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/panicking.rs:582:13
  15:     0x7f535b7f0b26 - std::sys_common::backtrace::__rust_end_short_backtrace::ha9ceea58b8cf1deb
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/sys_common/backtrace.rs:150:18
  16:     0x7f535b7f3382 - rust_begin_unwind
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/panicking.rs:578:5
  17:     0x7f535b84ce63 - core::panicking::panic_fmt::h1df8faa11491e0c5
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/core/src/panicking.rs:67:14
  18:     0x7f535b84d483 - core::result::unwrap_failed::h4175d949b4f9e8b9
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/core/src/result.rs:1651:5
  19:     0x7f5359cbfb1f - rustc_middle[f607de435e9a4418]::ty::context::tls::with_context::<<rustc_middle[f607de435e9a4418]::dep_graph::dep_node::DepKind as rustc_query_system[7835b3e98a87b812]::dep_graph::DepKind>::with_deps<<rustc_middle[f607de435e9a4418]::query::on_disk_cache::OnDiskCache>::serialize::{closure#0}, core[b4a8d54be4c63c12]::result::Result<usize, std[d9d2839bce1dd559]::io::error::Error>>::{closure#0}, core[b4a8d54be4c63c12]::result::Result<usize, std[d9d2839bce1dd559]::io::error::Error>>::{closure#0}
  20:     0x7f5359c8b521 - <rustc_middle[f607de435e9a4418]::ty::context::TyCtxt>::serialize_query_result_cache
  21:     0x7f5359c8b117 - <rustc_session[22fc273b1747149c]::session::Session>::time::<core[b4a8d54be4c63c12]::result::Result<usize, std[d9d2839bce1dd559]::io::error::Error>, rustc_incremental[ce295fd98e3725ed]::persist::save::encode_query_cache::{closure#0}>
  22:     0x7f5359c8a1ec - rustc_incremental[ce295fd98e3725ed]::persist::file_format::save_in::<rustc_incremental[ce295fd98e3725ed]::persist::save::save_dep_graph::{closure#0}::{closure#2}::{closure#0}::{closure#0}>
  23:     0x7f5359c89602 - <rustc_session[22fc273b1747149c]::session::Session>::time::<(), rustc_incremental[ce295fd98e3725ed]::persist::save::save_dep_graph::{closure#0}::{closure#2}::{closure#0}>
  24:     0x7f5359c6986f - rustc_data_structures[d67636fd6e7e7cea]::sync::join::<rustc_incremental[ce295fd98e3725ed]::persist::save::save_dep_graph::{closure#0}::{closure#2}, rustc_incremental[ce295fd98e3725ed]::persist::save::save_dep_graph::{closure#0}::{closure#3}, (), ()>
  25:     0x7f5359c633b6 - rustc_incremental[ce295fd98e3725ed]::persist::save::save_dep_graph
  26:     0x7f5359c63087 - <rustc_session[22fc273b1747149c]::session::Session>::time::<(), <rustc_interface[d75252421fea963]::interface::Compiler>::enter<rustc_driver_impl[64b7dd1911194932]::run_compiler::{closure#1}::{closure#2}, core[b4a8d54be4c63c12]::result::Result<core[b4a8d54be4c63c12]::option::Option<rustc_interface[d75252421fea963]::queries::Linker>, rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>::{closure#0}>
  27:     0x7f5359c60119 - <rustc_interface[d75252421fea963]::interface::Compiler>::enter::<rustc_driver_impl[64b7dd1911194932]::run_compiler::{closure#1}::{closure#2}, core[b4a8d54be4c63c12]::result::Result<core[b4a8d54be4c63c12]::option::Option<rustc_interface[d75252421fea963]::queries::Linker>, rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>
  28:     0x7f5359c5dcfd - <scoped_tls[36ad800287692b7a]::ScopedKey<rustc_span[f3562a9d228bc34b]::SessionGlobals>>::set::<rustc_interface[d75252421fea963]::interface::run_compiler<core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>, rustc_driver_impl[64b7dd1911194932]::run_compiler::{closure#1}>::{closure#0}, core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>
  29:     0x7f5359c5d156 - std[d9d2839bce1dd559]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d75252421fea963]::util::run_in_thread_pool_with_globals<rustc_interface[d75252421fea963]::interface::run_compiler<core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>, rustc_driver_impl[64b7dd1911194932]::run_compiler::{closure#1}>::{closure#0}, core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>
  30:     0x7f5359c5cf05 - <<std[d9d2839bce1dd559]::thread::Builder>::spawn_unchecked_<rustc_interface[d75252421fea963]::util::run_in_thread_pool_with_globals<rustc_interface[d75252421fea963]::interface::run_compiler<core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>, rustc_driver_impl[64b7dd1911194932]::run_compiler::{closure#1}>::{closure#0}, core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[b4a8d54be4c63c12]::result::Result<(), rustc_span[f3562a9d228bc34b]::ErrorGuaranteed>>::{closure#1} as core[b4a8d54be4c63c12]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7f535b7fdd45 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h0ee7a0a44efbcb8b
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/alloc/src/boxed.rs:1985:9
  32:     0x7f535b7fdd45 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h8f984f8b1c075279
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/alloc/src/boxed.rs:1985:9
  33:     0x7f535b7fdd45 - std::sys::unix::thread::Thread::new::thread_start::h054937194df87b6d
                               at /rustc/18bfe5d8a9ca0e226171e98f8f4ef071790f3352/library/std/src/sys/unix/thread.rs:108:17
  34:     0x7f5357465bb5 - <unknown>
  35:     0x7f53574e7d90 - <unknown>
  36:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (18bfe5d8a 2023-05-14) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type bin -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

query stack during panic:
end of query stack
[1]    555227 killed     rustc bad.rs --crate-type bin -C embed-bitcode=no -C split-debuginfo=unpacked
