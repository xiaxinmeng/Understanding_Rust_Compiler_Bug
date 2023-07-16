
% RUST_BACKTRACE=1 rustc regex-empty-str.rs
error: internal compiler error: unexpected failure
note: the compiler hit an unexpected failure path. this is a bug.
note: we would appreciate a bug report: http://static.rust-lang.org/doc/master/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' failed at 'index out of bounds: the len is 0 but the index is 0', /build/rust-git/src/rust/src/libregex/lib.rs:1
stack backtrace:
   1:     0x7f2f857062f0 - rt::backtrace::imp::write::h288a001cb8ab0467Mwa::v0.11.pre
   2:     0x7f2f85657960 - rt::unwind::begin_unwind_inner::h21d0e74b81447276R69::v0.11.pre
   3:     0x7f2f856578d0 - <unknown>
   4:     0x7f2f85705f00 - rt::unwind::begin_unwind_raw::hcae126cecf9ac08b039::v0.11.pre
   5:     0x7f2f85656c00 - rt::unwind::fail_::hb350ad41fcb04968E19::v0.11.pre
   6:     0x7f2f85705f70 - <unknown>
   7:     0x7f2f85658b90 - rt::unwind::fail_bounds_check::h77cb92ae44655be1119::v0.11.pre
   8:     0x7f2f81af2710 - <unknown>
   9:     0x7f2f81b018a0 - re::Regex::new::h7515f9e2f6385e4exZg::v0.11.pre
  10:     0x7f2f81d458e0 - <unknown>
  11:     0x7f2f845f43d0 - ext::base::BasicMacroExpander.MacroExpander::expand::hd67bc89fd9d9a56eQbR::v0.11.pre
  12:     0x7f2f84605490 - ext::expand::expand_expr::ha374fe02226e64benIR::v0.11.pre
  13:     0x7f2f8460dd70 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_expr::h677bb239787cfda7RyS::v0.11.pre
  14:     0x7f2f8462cb80 - ext::expand::expand_stmt::hcdb4c2679a8ed298FhS::v0.11.pre
  15:     0x7f2f84648610 - <unknown>
  16:     0x7f2f8461a840 - ext::expand::expand_block_elts::h5c64eab4a957c3e9OuS::v0.11.pre
  17:     0x7f2f84648530 - ext::expand::expand_block::h454ad0775b85e8908tS::v0.11.pre
  18:     0x7f2f846175b0 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_block::h96e96e3fb41ac8e10zS::v0.11.pre
  19:     0x7f2f84620ae0 - <unknown>
  20:     0x7f2f8461b600 - ext::expand::expand_item::h94bc4908c0c8fdf5TVR::v0.11.pre
  21:     0x7f2f84623bf0 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_item::hfe58429eae993e278yS::v0.11.pre
  22:     0x7f2f84623b90 - <unknown>
  23:     0x7f2f846235d0 - <unknown>
  24:     0x7f2f84622ea0 - <unknown>
  25:     0x7f2f84648d50 - ext::expand::expand_crate::h8377624133b73814MAS::v0.11.pre
  26:     0x7f2f86b61a00 - <unknown>
  27:     0x7f2f86b61260 - <unknown>
  28:     0x7f2f86b5eaf0 - driver::driver::phase_2_configure_and_expand::he42a57583b18ce58YSe::v0.11.pre
  29:     0x7f2f86ba1550 - driver::driver::compile_input::hffffe7f389caa620Amf::v0.11.pre
  30:     0x7f2f86bc6aa0 - run_compiler::hfcd48299addf77dfeQm::v0.11.pre
  31:     0x7f2f86be2f10 - <unknown>
  32:     0x7f2f86be1410 - <unknown>
  33:     0x7f2f86bdbc80 - <unknown>
  34:     0x7f2f85d9e340 - <unknown>
  35:     0x7f2f856fbda0 - <unknown>
  36:     0x7f2f8570c5c0 - rust_try
  37:     0x7f2f856fbbe0 - rt::task::Task::run::h93895fb04573c54dUW7::v0.11.pre
  38:     0x7f2f85d9e110 - <unknown>
  39:     0x7f2f85704e30 - <unknown>
  40:     0x7f2f833ef060 - start_thread
  41:     0x7f2f8532a489 - __clone
  42:                0x0 - <unknown>
