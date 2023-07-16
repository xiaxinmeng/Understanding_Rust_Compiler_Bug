plain
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.73
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `NodeStart(57)`,
 right: `NoNode`', compiler/rustc_metadata/src/rmeta/encoder.rs:397:9
   0:     0x7f6416676e22 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb97170048eb2feed
   0:     0x7f6416676e22 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb97170048eb2feed
   1:     0x7f64166dea58 - core::fmt::write::hddd81543d4c11162
   2:     0x7f6416667231 - std::io::Write::write_fmt::hb10c7c89d9d85e51
   3:     0x7f641667a139 - std::panicking::default_hook::{{closure}}::hf26c6e572bb06902
   4:     0x7f6416679dda - std::panicking::default_hook::h5e275631e30996f3
   5:     0x7f6417182664 - rustc_driver[49a19c9f287af04e]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f641667a99f - std::panicking::rust_panic_with_hook::h88a63d00b2da51a4
   7:     0x7f641667a7e7 - std::panicking::begin_panic_handler::{{closure}}::h896476355c1c691e
   8:     0x7f64166773c4 - std::sys_common::backtrace::__rust_end_short_backtrace::hd0faa25817600199
   9:     0x7f641667a4d9 - rust_begin_unwind
  10:     0x7f641662e203 - core::panicking::panic_fmt::h8ee02022fd7b4670
  11:     0x7f64166db338 - core::panicking::assert_failed_inner::h3caa3bfaa14f944f
  12:     0x7f64170949ab - core[990e9aac2c65c03a]::panicking::assert_failed::<rustc_metadata[9c6581cb287475aa]::rmeta::LazyState, rustc_metadata[9c6581cb287475aa]::rmeta::LazyState>
  13:     0x7f64190e4199 - <rustc_metadata[9c6581cb287475aa]::rmeta::encoder::EncodeContext>::lazy::<alloc[b31f836c82ae9902]::string::String, alloc[b31f836c82ae9902]::string::String>
  14:     0x7f641912beb0 - <rustc_span[4929ef0fa1883a40]::symbol::Symbol as rustc_serialize[2bae28866586257a]::serialize::Encodable<rustc_metadata[9c6581cb287475aa]::rmeta::encoder::EncodeContext>>::encode
  15:     0x7f64191ba322 - <core[990e9aac2c65c03a]::iter::adapters::map::Map<alloc[b31f836c82ae9902]::vec::into_iter::IntoIter<(rustc_span[4929ef0fa1883a40]::symbol::Symbol, core[990e9aac2c65c03a]::option::Option<rustc_span[4929ef0fa1883a40]::symbol::Symbol>)>, <rustc_metadata[9c6581cb287475aa]::rmeta::encoder::EncodeContext>::lazy_array<(rustc_span[4929ef0fa1883a40]::symbol::Symbol, core[990e9aac2c65c03a]::option::Option<rustc_span[4929ef0fa1883a40]::symbol::Symbol>), alloc[b31f836c82ae9902]::vec::Vec<(rustc_span[4929ef0fa1883a40]::symbol::Symbol, core[990e9aac2c65c03a]::option::Option<rustc_span[4929ef0fa1883a40]::symbol::Symbol>)>, (rustc_span[4929ef0fa1883a40]::symbol::Symbol, core[990e9aac2c65c03a]::option::Option<rustc_span[4929ef0fa1883a40]::symbol::Symbol>)>::{closure#0}> as core[990e9aac2c65c03a]::iter::traits::iterator::Iterator>::fold::<usize, <core[990e9aac2c65c03a]::iter::adapters::map::Map<alloc[b31f836c82ae9902]::vec::into_iter::IntoIter<(rustc_span[4929ef0fa1883a40]::symbol::Symbol, core[990e9aac2c65c03a]::option::Option<rustc_span[4929ef0fa1883a40]::symbol::Symbol>)>, <rustc_metadata[9c6581cb287475aa]::rmeta::encoder::EncodeContext>::lazy_array<(rustc_span[4929ef0fa1883a40]::symbol::Symbol, core[990e9aac2c65c03a]::option::Option<rustc_span[4929ef0fa1883a40]::symbol::Symbol>), alloc[b31f836c82ae9902]::vec::Vec<(rustc_span[4929ef0fa1883a40]::symbol::Symbol, core[990e9aac2c65c03a]::option::Option<rustc_span[4929ef0fa1883a40]::symbol::Symbol>)>, (rustc_span[4929ef0fa1883a40]::symbol::Symbol, core[990e9aac2c65c03a]::option::Option<rustc_span[4929ef0fa1883a40]::symbol::Symbol>)>::{closure#0}> as core[990e9aac2c65c03a]::iter::traits::iterator::Iterator>::count::{closure#0}>
  16:     0x7f64190e6664 - <rustc_metadata[9c6581cb287475aa]::rmeta::encoder::EncodeContext>::encode_crate_root
  17:     0x7f64190ff3fe - rustc_metadata[9c6581cb287475aa]::rmeta::encoder::encode_metadata_impl
  18:     0x7f64191b1ad1 - rustc_data_structures[a05911148c4af6d]::sync::join::<rustc_metadata[9c6581cb287475aa]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[9c6581cb287475aa]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[9c6581cb287475aa]::rmeta::encoder::EncodedMetadata, ()>
  19:     0x7f64190fea0e - rustc_metadata[9c6581cb287475aa]::rmeta::encoder::encode_metadata
  20:     0x7f64172a4aad - <rustc_interface[b0a9b6e49911083a]::passes::QueryContext>::enter::<<rustc_interface[b0a9b6e49911083a]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[990e9aac2c65c03a]::result::Result<alloc[b31f836c82ae9902]::boxed::Box<dyn core[990e9aac2c65c03a]::any::Any>, rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>
  21:     0x7f6417298b4e - <rustc_interface[b0a9b6e49911083a]::queries::Queries>::ongoing_codegen
  22:     0x7f641719746f - <rustc_interface[b0a9b6e49911083a]::interface::Compiler>::enter::<rustc_driver[49a19c9f287af04e]::run_compiler::{closure#1}::{closure#2}, core[990e9aac2c65c03a]::result::Result<core[990e9aac2c65c03a]::option::Option<rustc_interface[b0a9b6e49911083a]::queries::Linker>, rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>
  23:     0x7f64171f3509 - rustc_span[4929ef0fa1883a40]::with_source_map::<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>, rustc_interface[b0a9b6e49911083a]::interface::create_compiler_and_run<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>, rustc_driver[49a19c9f287af04e]::run_compiler::{closure#1}>::{closure#1}>
  24:     0x7f641719868e - <scoped_tls[56ec63818e29d8c5]::ScopedKey<rustc_span[4929ef0fa1883a40]::SessionGlobals>>::set::<rustc_interface[b0a9b6e49911083a]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>, rustc_driver[49a19c9f287af04e]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>
  25:     0x7f64171f0009 - std[cf5d045d49351115]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[b0a9b6e49911083a]::util::run_in_thread_pool_with_globals<rustc_interface[b0a9b6e49911083a]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>, rustc_driver[49a19c9f287af04e]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>
  26:     0x7f64171e7d79 - <<std[cf5d045d49351115]::thread::Builder>::spawn_unchecked_<rustc_interface[b0a9b6e49911083a]::util::run_in_thread_pool_with_globals<rustc_interface[b0a9b6e49911083a]::interface::run_compiler<core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>, rustc_driver[49a19c9f287af04e]::run_compiler::{closure#1}>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>::{closure#0}, core[990e9aac2c65c03a]::result::Result<(), rustc_errors[ae89e244cd91783c]::ErrorGuaranteed>>::{closure#1} as core[990e9aac2c65c03a]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  27:     0x7f6416687363 - std::sys::unix::thread::Thread::new::thread_start::hfdb91e9761b815b8
  28:     0x7f6410bd7609 - start_thread
  29:     0x7f64164ea133 - clone
  30:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (583d33d75 2022-07-05) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
