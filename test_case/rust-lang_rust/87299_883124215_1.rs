
error: internal compiler error: failed to process buffered lint here (dummy = false)
  --> C:\Users\owner\.cargo\registry\src\github.com-1ecc6299db9ec823\wgpu-core-0.7.1\build.rs:7:5
   |
7  | /     cfg_aliases::cfg_aliases! {
8  | |         // Vendors/systems
9  | |         wasm: { target_arch = "wasm32" },
10 | |         apple: { any(target_os = "ios", target_os = "macos") },
...  |
18 | |         gl: { any(wasm, unix_wo_apple) },
19 | |     }
   | |_____^
   |
   = note: delayed at /rustc/014026d1a7ca991f82f12efa95ef4dffb29dc8af\compiler\rustc_lint\src\early.rs:402:18
   = note: this error: internal compiler error originates in the macro `$crate::cfg_aliases` (in Nightly builds, run with -Z macro-backtrace for more info)

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler\rustc_errors\src\lib.rs:1050:13
stack backtrace:
   0:     0x7ffaa12f72cf - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf8f9ccaaf737268c
   1:     0x7ffaa132030a - core::fmt::write::h7e004fee938dc3bd
   2:     0x7ffaa12eaa58 - <std::io::IoSlice as core::fmt::Debug>::fmt::h5e8f50613d775ee8
   3:     0x7ffaa12faef6 - std::panicking::take_hook::hb09930640d1f28d1
   4:     0x7ffaa12fa9d9 - std::panicking::take_hook::hb09930640d1f28d1
   5:     0x7ffa93b3210e - <md5::Md5 as std::io::Write>::flush::h740848eb8d315998
   6:     0x7ffaa12fb7f0 - std::panicking::rust_panic_with_hook::ha77067ca1c3aa04d
   7:     0x7ffaa12fb2b1 - rust_begin_unwind
   8:     0x7ffaa12f7c1f - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf8f9ccaaf737268c
   9:     0x7ffaa12fb209 - rust_begin_unwind
  10:     0x7ffaa135599c - std::panicking::begin_panic_fmt::hd0b34d17ecdda70b
  11:     0x7ffa97e115cf - rustc_errors::HandlerInner::delay_as_bug::had7a1fc16257907d
  12:     0x7ffa97e0d052 - <rustc_errors::HandlerInner as core::ops::drop::Drop>::drop::h6ab627992737d6aa
  13:     0x7ffa93bb5374 - <rustc_driver::args::Error as core::fmt::Debug>::fmt::h5df0c4d8fb3d3c5e
  14:     0x7ffa93bbce5a - <rustc_driver::args::Error as core::fmt::Debug>::fmt::h5df0c4d8fb3d3c5e
  15:     0x7ffa93b4fe1d - <rustc_hir::intravisit::ErasedMap as rustc_hir::intravisit::Map>::foreign_item::h82aae2bf47c2b2ad
  16:     0x7ffa93b4bcee - chalk_engine::TableIndex::increment::h1dc89da32ec272e2
  17:     0x7ffa93b89359 - <rustc_driver::args::Error as core::fmt::Debug>::fmt::h5df0c4d8fb3d3c5e
  18:     0x7ffa93b5687a - <rustc_middle::ty::SymbolName as core::fmt::Display>::fmt::h5079114a276318b8
  19:     0x7ffa93b53518 - rustc_driver::pretty::print_after_hir_lowering::h5170af9d47465e28
  20:     0x7ffa93b46d6d - <rustc_driver::Compilation as core::fmt::Debug>::fmt::h64cba24964385564
  21:     0x7ffaa130933c - std::sys::windows::thread::Thread::new::h0a6e657a03ae2a48
  22:     0x7ffaf2957034 - BaseThreadInitThunk
  23:     0x7ffaf3ba2651 - RtlUserThreadStart
