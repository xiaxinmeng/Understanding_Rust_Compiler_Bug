
C:\Users\Davide\Dropbox\Code\CellularMaps-Rust\src\main.rs:6    for c in range(0u,(map.get_width())) {
                                                                    ^
C:\Users\Davide\Dropbox\Code\CellularMaps-Rust\src\main.rs:6:6: 6:7 error: internal compiler error: debuginfo::create_for_loop_var_metadata() - Referenced variable location is not an alloca!
C:\Users\Davide\Dropbox\Code\CellularMaps-Rust\src\main.rs:6    for c in range(0u,(map.get_width())) {
                                                                    ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', C:\bot\slave\nightly-win-64\build\src\libsyntax\diagnostic.rs:123

stack backtrace:
   1:         0x69bef7f8 - sys::backtrace::write::h5b0ba28ad71857afeQt
   2:         0x69c021e9 - rt::unwind::register::h91aefd9b472076aecGz
   3:         0x69b82fcb - rt::unwind::begin_unwind_inner::h04b772eae7a7fc7bJDz
   4:         0x6f8acd56 - diagnostic::SpanHandler::span_bug::h90408fcd1499494fwZF
   5:         0x6f8acd00 - diagnostic::SpanHandler::span_bug::h90408fcd1499494fwZF
   6:           0x4a1cea - session::Session::span_bug::h33a792ccd13ce896fon
   7:           0xf7cecf - trans::debuginfo::UniqueTypeId...std..fmt..Show::fmt::h2a94dd8eb7abda200OD
   8:         0x6f8791e1 - ast_util::walk_pat::h6509451e2c595f07fyC
   9:           0xeb1553 - trans::type_::Type...std..cmp..PartialEq::ne::h4b92644f40b08c44S1J
  10:           0xe6a133 - trans::cleanup::FunctionContext<$u{27}blk$C$$u{20}$u{27}tcx$GT$.CleanupMethods$LT$$u{27}blk$C$$u{20}$u{27}tcx$GT$::pop_and_trans_ast_cleanup_scope::h411a754a1c1fd541h4K
  11:           0xe6aa68 - trans::cleanup::FunctionContext<$u{27}blk$C$$u{20}$u{27}tcx$GT$.CleanupMethods$LT$$u{27}blk$C$$u{20}$u{27}tcx$GT$::pop_and_trans_ast_cleanup_scope::h411a754a1c1fd541h4K
  12:           0xf158ec - trans::base::IsUnboxedClosureFlag...std..clone..Clone::clone::h57e6afe7886161b1abu
  13:           0xe5ec48 - trans::context::CrateContext<$u{27}b$C$$u{20}$u{27}tcx$GT$::sess::hcfd66e5fd92d5809Ksm
  14:           0xe5a40e - trans::context::CrateContext<$u{27}b$C$$u{20}$u{27}tcx$GT$::stats::h05a9b3c7687a7fcemFm
  15:           0xf1c578 - trans::base::trans_crate::h397f6c9d3999aac9aEv
  16:         0x70b2d263 - driver::phase_4_translate_to_llvm::h3d4d3b1fa1113efa1Ca
  17:         0x70b03b22 - driver::compile_input::hd01226bf772688afvba
  18:         0x70ca3e95 - run::h9cdc49dc8c011aeedYb
  19:         0x70ca05b9 - run::h9cdc49dc8c011aeedYb
  20:         0x69c6648c - rust_try
  21:         0x69c66469 - rust_try
  22:         0x70ca0d0f - run::h9cdc49dc8c011aeedYb
  23:         0x69bf4ee7 - sys::tcp::TcpListener::bind::hb1b42652ff0a60d9Fqw
  24:     0x7ffcd12516ad - BaseThreadInitThunk

Could not compile `cellular_maps`.
