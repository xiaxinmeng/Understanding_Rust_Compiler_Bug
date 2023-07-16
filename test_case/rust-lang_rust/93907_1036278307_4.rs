
stack backtrace:
   0:     0x7ffded817d30 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he09221c2d444dfe1
   1:     0x7ffded84779a - core::fmt::write::h7c00eda1e723c654
   2:     0x7ffded809059 - <std::io::IoSlice as core::fmt::Debug>::fmt::h793e0aeaf1fec88d
   3:     0x7ffded81b4a2 - std::panicking::default_hook::h608d4dde448f0c29
   4:     0x7ffded81b063 - std::panicking::default_hook::h608d4dde448f0c29
   5:     0x7ffdd649ec09 - <rustc_driver[416e5eba3e41cb3]::args::Error as core[984dcdc6a6b66ef3]::fmt::Debug>::fmt
   6:     0x7ffded81bdd2 - std::panicking::rust_panic_with_hook::h1b80c96e85e80f64
   7:     0x7ffded81bb1d - <std::panicking::begin_panic_handler::StrPanicPayload as core::panic::BoxMeUp>::get::hba826596c7be7c9e
   8:     0x7ffded818687 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he09221c2d444dfe1
   9:     0x7ffded81b799 - rust_begin_unwind
  10:     0x7ffded87a985 - core::panicking::panic_fmt::h987a325189bd1697
  11:     0x7ffddabfc7e7 - <rustc_errors[c29ebb7dd1d8d943]::ErrorReported as core[984dcdc6a6b66ef3]::fmt::Debug>::fmt
  12:     0x7ffddabf94b8 - <rustc_errors[c29ebb7dd1d8d943]::HandlerInner>::delay_as_bug
  13:     0x7ffddabf49c2 - <rustc_errors[c29ebb7dd1d8d943]::HandlerInner as core[984dcdc6a6b66ef3]::ops::drop::Drop>::drop
  14:     0x7ffdd6497224 - <rustc_driver[416e5eba3e41cb3]::args::Error as core[984dcdc6a6b66ef3]::fmt::Debug>::fmt
  15:     0x7ffdd649b427 - <rustc_driver[416e5eba3e41cb3]::args::Error as core[984dcdc6a6b66ef3]::fmt::Debug>::fmt
  16:     0x7ffdd64339dd - <rustc_middle[36d3a5273b30a1f]::ty::SymbolName as core[984dcdc6a6b66ef3]::fmt::Debug>::fmt
  17:     0x7ffdd6437ff0 - <rustc_driver[416e5eba3e41cb3]::args::Error as core[984dcdc6a6b66ef3]::fmt::Debug>::fmt
  18:     0x7ffdd642d53a - <rustc_codegen_ssa[71a279e59bdb6602]::back::linker::MsvcLinker as rustc_codegen_ssa[71a279e59bdb6602]::back::linker::Linker>::optimize
  19:     0x7ffdd640e383 - <rustc_codegen_ssa[71a279e59bdb6602]::back::linker::MsvcLinker as rustc_codegen_ssa[71a279e59bdb6602]::back::linker::Linker>::optimize
  20:     0x7ffdd64b5138 - rustc_driver[416e5eba3e41cb3]::pretty::print_after_hir_lowering
  21:     0x7ffded82c73c - std::sys::windows::thread::Thread::new::h5fb28379b94c9474
  22:     0x7ffe7f1854e0 - BaseThreadInitThunk
  23:     0x7ffe7fee485b - RtlUserThreadStart

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (e646f3d2a 2022-02-10) running on x86_64-pc-windows-msvc

note: compiler flags: --crate-type rlib -C embed-bitcode=no -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `rocket`
