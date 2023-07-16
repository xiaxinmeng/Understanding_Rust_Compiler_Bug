
   Compiling something v0.1.0 (file:///something/something)
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', src/libcore/option.rs:323
stack backtrace:
   1:      0x3be2ba97c9c - <unknown>
   2:      0x3be2baa636e - <unknown>
   3:      0x3be2baa5f03 - <unknown>
   4:      0x3be2baa680b - std::panicking::rust_panic_with_hook::h9f3930ca8cee8a65
   5:      0x3be2baa66a4 - <unknown>
   6:      0x3be2baa65c9 - std::panicking::begin_panic_fmt::h6073f869f9b775fa
   7:      0x3be2baa6557 - rust_begin_unwind
   8:      0x3be2bafcfbd - core::panicking::panic_fmt::hc8432e9fe5639d04
   9:      0x3be2bafcef4 - core::panicking::panic::hd491630d7a1ea17f
  10:      0x3be297902a6 - <unknown>
  11:      0x3be297973bf - <rustc_metadata::creader::CrateLoader<'a> as rustc::middle::cstore::CrateLoader>::process_item::h3d7a10bdafa59007
  12:      0x3be28fb50c0 - <unknown>
  13:      0x3be28fbbe8f - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit::Visitor<'a>>::visit_item::h2eab4919824bc032
  14:      0x3be28fbc38f - <rustc_resolve::build_reduced_graph::BuildReducedGraphVisitor<'a, 'b> as syntax::visit::Visitor<'a>>::visit_item::h2eab4919824bc032
  15:      0x3be28fafdcf - rustc_resolve::macros::<impl syntax::ext::base::Resolver for rustc_resolve::Resolver<'a>>::visit_expansion::h6a021c2cdf355ad5
  16:      0x3be254ffa32 - <unknown>
  17:      0x3be254fd677 - <unknown>
  18:      0x3be254fd135 - syntax::ext::expand::MacroExpander::expand_crate::h9b5d9e839a2028cc
  19:      0x3be2be4bc0c - <unknown>
  20:      0x3be2be43325 - <unknown>
  21:      0x3be2be3bf62 - rustc_driver::driver::compile_input::h9ff46639bcd36041
  22:      0x3be2be88394 - rustc_driver::run_compiler::hb5d68fffb8e95cee
  23:      0x3be2bd945cb - <unknown>
  24:      0x3be2baaf67a - __rust_maybe_catch_panic
  25:      0x3be2bdbc802 - <unknown>
  26:      0x3be2baa51b4 - <unknown>
  27:      0x3be246342e6 - start_thread
  28:      0x3be2b76554e - clone
  29:                0x0 - <unknown>

error: Could not compile `something`.

To learn more, run the command again with --verbose.
