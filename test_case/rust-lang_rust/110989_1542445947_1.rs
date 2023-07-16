
error: expected one of `->`, `where`, or `{`, found `<eof>`
  --> /home/uweigand/rust/tests/rustdoc-ui/ice-bug-report-url.rs:13:10
   |
LL | fn wrong()
   |          ^ expected one of `->`, `where`, or `{`

thread 'rustc' panicked at 'aborting due to `-Z treat-err-as-bug=1`', compiler/rustc_errors/src/lib.rs:1711:30
stack backtrace:
   0:      0x3ffec4992a2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h413b3b0f9de3326b
   1:      0x3ffec56739e - core::fmt::write::h84c7cedca7777db7
   2:      0x3ffec4ade78 - std::panicking::default_hook::h89c76c4bb766adfe
   3:      0x3ffed577204 - rustc_driver_impl[b094fb3383cea06f]::install_ice_hook::{closure#0}
   4:      0x3ffec4af758 - std::panicking::rust_panic_with_hook::h3b0cf6703ee96eed
   5:      0x3ffec499854 - std::panicking::begin_panic_handler::{{closure}}::h36b73881ffb76e47
   6:      0x3ffec4995d0 - std::sys_common::backtrace::__rust_end_short_backtrace::h9f7578507492eb4f
   7:      0x3ffec4af2ae - rust_begin_unwind
   8:      0x3ffec47058c - core::panicking::panic_fmt::h9c206364a104a065
   9:      0x3fff4944962 - <rustc_errors[23e277f92d39e05]::HandlerInner>::emit_diagnostic::{closure#2}
  10:      0x3fff4943c28 - <rustc_errors[23e277f92d39e05]::HandlerInner>::emit_diagnostic
  11:      0x3fff49b3be6 - <rustc_span[fa50c18a7876c42a]::ErrorGuaranteed as rustc_errors[23e277f92d39e05]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  12:      0x3ffed6acba2 - <rustc_interface[6001cdef06e51c04]::queries::Queries>::pre_configure
  13:      0x3ffed6ad5f8 - <rustc_interface[6001cdef06e51c04]::queries::Queries>::global_ctxt
  14:      0x2aa0048f836 - <rustc_interface[6001cdef06e51c04]::interface::Compiler>::enter::<rustdoc[9110c6921b1a8d2a]::main_args::{closure#1}::{closure#0}, core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>>
  15:      0x2aa001f4318 - rustc_span[fa50c18a7876c42a]::set_source_map::<core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>, rustc_interface[6001cdef06e51c04]::interface::run_compiler<core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>, rustdoc[9110c6921b1a8d2a]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  16:      0x2aa004915d8 - <scoped_tls[de53d4204225384f]::ScopedKey<rustc_span[fa50c18a7876c42a]::SessionGlobals>>::set::<rustc_interface[6001cdef06e51c04]::interface::run_compiler<core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>, rustdoc[9110c6921b1a8d2a]::main_args::{closure#1}>::{closure#0}, core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>>
  17:      0x2aa000f1cf8 - std[4f5ed56b8f53302d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[6001cdef06e51c04]::util::run_in_thread_pool_with_globals<rustc_interface[6001cdef06e51c04]::interface::run_compiler<core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>, rustdoc[9110c6921b1a8d2a]::main_args::{closure#1}>::{closure#0}, core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>>
  18:      0x2aa000f35ae - std[4f5ed56b8f53302d]::panicking::try::do_call::<core[1e85ed23d5535b8b]::panic::unwind_safe::AssertUnwindSafe<<std[4f5ed56b8f53302d]::thread::Builder>::spawn_unchecked_<rustc_interface[6001cdef06e51c04]::util::run_in_thread_pool_with_globals<rustc_interface[6001cdef06e51c04]::interface::run_compiler<core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>, rustdoc[9110c6921b1a8d2a]::main_args::{closure#1}>::{closure#0}, core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>>
  19:      0x2aa0014c090 - __rust_try.llvm.11562562457469458502
  20:      0x2aa003315aa - <<std[4f5ed56b8f53302d]::thread::Builder>::spawn_unchecked_<rustc_interface[6001cdef06e51c04]::util::run_in_thread_pool_with_globals<rustc_interface[6001cdef06e51c04]::interface::run_compiler<core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>, rustdoc[9110c6921b1a8d2a]::main_args::{closure#1}>::{closure#0}, core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1e85ed23d5535b8b]::result::Result<(), rustc_span[fa50c18a7876c42a]::ErrorGuaranteed>>::{closure#1} as core[1e85ed23d5535b8b]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  21:      0x3ffec4fe0b0 - std::sys::unix::thread::Thread::new::thread_start::ha67c2f2eda1f7e3b
  22:      0x3ffec09f5c8 - start_thread
                               at /usr/src/debug/glibc-2.36-9.1.ibm.fc37.s390x/nptl/pthread_create.c:442:8
  23:      0x3ffec11b51e - <unknown>
                               at /usr/src/debug/glibc-2.36-9.1.ibm.fc37.s390x/misc/../sysdeps/unix/sysv/linux/s390/s390-64/clone.S:66
  24:                0x0 - <unknown>

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-rustdoc&template=ice.md

note: rustc 1.71.0-dev running on s390x-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C debuginfo=0 -Z treat-err-as-bug

query stack during panic:
end of query stack
