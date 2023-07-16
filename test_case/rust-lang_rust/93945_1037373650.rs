plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.67
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'source slice length (8) does not match destination slice length (4)', compiler/rustc_metadata/src/rmeta/encoder.rs:2236:32
stack backtrace:
   0:     0x7fb521d4772c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc18baf0e5ad62c0e
   1:     0x7fb521db5e9e - core::fmt::write::h04f3cb9c5bd3369c
   2:     0x7fb521d366e1 - std::io::Write::write_fmt::h73ff4fde244178ae
   3:     0x7fb521d4755b - std::sys_common::backtrace::print::hc84114397f416d7d
   4:     0x7fb521d4bd24 - std::panicking::default_hook::{{closure}}::hf05675e5828388ac
   5:     0x7fb521d4b906 - std::panicking::default_hook::h05e4c1dbbc633a4e
   6:     0x7fb52281f801 - rustc_driver[b4ef4919aa58b87e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fb521d4c443 - std::panicking::rust_panic_with_hook::h96c2153e7808e938
   8:     0x7fb521d4c257 - std::panicking::begin_panic_handler::{{closure}}::hc36987e05b06ca89
   9:     0x7fb521d47c44 - std::sys_common::backtrace::__rust_end_short_backtrace::h30c4ddf35a5c2752
  10:     0x7fb521d4bf19 - rust_begin_unwind
  11:     0x7fb521d029d3 - core::panicking::panic_fmt::h7802ba5043cb2ca5
  12:     0x7fb521d02e22 - core::slice::<impl [T]>::copy_from_slice::len_mismatch_fail::ha2f5fba5d6466dd4
  13:     0x7fb524578537 - <[u8]>::copy_from_slice
  14:     0x7fb52452b03b - rustc_metadata[2dbd8235e3c3c141]::rmeta::encoder::encode_metadata_impl
  15:     0x7fb5245a3974 - rustc_data_structures[1eb754226c267db1]::sync::join::<rustc_metadata[2dbd8235e3c3c141]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[2dbd8235e3c3c141]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[2dbd8235e3c3c141]::rmeta::encoder::EncodedMetadata, ()>
  16:     0x7fb52452a27b - rustc_metadata[2dbd8235e3c3c141]::rmeta::encoder::encode_metadata
  17:     0x7fb5229ccc5b - <rustc_interface[c015e0d6d1a2e32c]::passes::QueryContext>::enter::<<rustc_interface[c015e0d6d1a2e32c]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<alloc[2897730bb73bbd5c]::boxed::Box<dyn core[ba5cb6be30a93795]::any::Any>, rustc_errors[5cad097c964025d7]::ErrorReported>>
  18:     0x7fb5229ab0de - <rustc_interface[c015e0d6d1a2e32c]::queries::Queries>::ongoing_codegen
  19:     0x7fb5227d0e15 - <rustc_interface[c015e0d6d1a2e32c]::interface::Compiler>::enter::<rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}::{closure#2}, core[ba5cb6be30a93795]::result::Result<core[ba5cb6be30a93795]::option::Option<rustc_interface[c015e0d6d1a2e32c]::queries::Linker>, rustc_errors[5cad097c964025d7]::ErrorReported>>
  20:     0x7fb52280eb69 - rustc_span[519afcfa7470336f]::with_source_map::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_interface[c015e0d6d1a2e32c]::interface::create_compiler_and_run<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>::{closure#1}>
  21:     0x7fb5227cefc9 - rustc_interface[c015e0d6d1a2e32c]::interface::create_compiler_and_run::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>
  22:     0x7fb5227ac11e - <scoped_tls[9ef8146b5f47b671]::ScopedKey<rustc_span[519afcfa7470336f]::SessionGlobals>>::set::<rustc_interface[c015e0d6d1a2e32c]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c015e0d6d1a2e32c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>
  23:     0x7fb5227a98a5 - std[eec5ef45012f6570]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c015e0d6d1a2e32c]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c015e0d6d1a2e32c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>
  24:     0x7fb522811e41 - std[eec5ef45012f6570]::panicking::try::<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, core[ba5cb6be30a93795]::panic::unwind_safe::AssertUnwindSafe<<std[eec5ef45012f6570]::thread::Builder>::spawn_unchecked_<rustc_interface[c015e0d6d1a2e32c]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c015e0d6d1a2e32c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#1}::{closure#0}>>
  25:     0x7fb5227abd92 - <<std[eec5ef45012f6570]::thread::Builder>::spawn_unchecked_<rustc_interface[c015e0d6d1a2e32c]::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface[c015e0d6d1a2e32c]::interface::run_compiler<core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>, rustc_driver[b4ef4919aa58b87e]::run_compiler::{closure#1}>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#0}, core[ba5cb6be30a93795]::result::Result<(), rustc_errors[5cad097c964025d7]::ErrorReported>>::{closure#1} as core[ba5cb6be30a93795]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  26:     0x7fb521d5b213 - std::sys::unix::thread::Thread::new::thread_start::he83677990b8dc5fd
  27:     0x7fb51c0cd609 - start_thread
  28:     0x7fb521bc4293 - clone
  29:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (c29688dc2 2022-02-12) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C symbol-mangling-version=legacy -Z unstable-options -Z split-metadata -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
