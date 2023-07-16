
$ RUST_BACKTRACE=1 rustc thing.rs
thing.rs:2:15: 2:16 error: internal compiler error: ident only path should have been covered already
thing.rs:2     for thing(x[]) {
                         ^
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:130

stack backtrace:
   1:     0x7f70f09cb849 - sys::backtrace::write::h0f507cf2b119bca1VvD
   2:     0x7f70f09f62ac - panicking::on_panic::h04ebb6dcfaadcf921LJ
   3:     0x7f70f091c423 - rt::unwind::begin_unwind_inner::h43d0ccf768b9e081IrJ
   4:     0x7f70edd1760d - rt::unwind::begin_unwind::h826077241771358402
   5:     0x7f70edd175b3 - diagnostic::SpanHandler::span_bug::h80ed710d6b368e45ccB
   6:     0x7f70edd3e7ca - parse::parser::Parser<'a>::parse_pat::h4c0d7251e28e428a3qH
   7:     0x7f70edd52bfd - parse::parser::Parser<'a>::parse_unspanned_seq::h3679275454620717133
   8:     0x7f70edd3dc46 - parse::parser::Parser<'a>::parse_pat::h4c0d7251e28e428a3qH
   9:     0x7f70edd4b0e6 - parse::parser::Parser<'a>::parse_for_expr::hc1b9eb2dc219fc8epbH
  10:     0x7f70edd46d55 - parse::parser::Parser<'a>::parse_bottom_expr::h362d99c42b2176e7GfG
  11:     0x7f70edd4d57d - parse::parser::Parser<'a>::parse_dot_or_call_expr::hd16d2c5d19125e72EyG
  12:     0x7f70edd503ab - parse::parser::Parser<'a>::parse_prefix_expr::h96aa457c8b8ebc6a0SG
  13:     0x7f70edd509fd - parse::parser::Parser<'a>::parse_binops::hb1d80895ae4d5b970XG
  14:     0x7f70edd512c6 - parse::parser::Parser<'a>::parse_assign_expr::h6e06d25a36ee4c45H2G
  15:     0x7f70edd540e6 - parse::parser::Parser<'a>::parse_stmt_::h14eccd81314aa067uLH
  16:     0x7f70edd4c75a - parse::parser::Parser<'a>::parse_block_tail::ha53e37ba005c5052OVH
  17:     0x7f70edd37e7d - parse::parser::Parser<'a>::parse_inner_attrs_and_block::he0fab8c5122aa9faHUH
  18:     0x7f70edd6045a - parse::parser::Parser<'a>::parse_item_fn::h2a46e5272077d7cfSvI
  19:     0x7f70edd56ff6 - parse::parser::Parser<'a>::parse_item_::h35a25dbfb6a95a1cMlJ
  20:     0x7f70edd6616a - parse::parser::Parser<'a>::parse_mod_items::heb665d27e9a3b4e8RRI
  21:     0x7f70edd6c845 - parse::parser::Parser<'a>::parse_crate_mod::h75221af8ec569051AKJ
  22:     0x7f70edd81555 - parse::parse_crate_from_file::hb8f7065228fc9b9fZxQ
  23:     0x7f70f1069758 - driver::phase_1_parse_input::closure.15497
  24:     0x7f70f1040963 - driver::phase_1_parse_input::hb46b4058e9a3285cNqa
  25:     0x7f70f103cdc7 - driver::compile_input::hef973a4d6868aed2Qba
  26:     0x7f70f10f5d55 - run_compiler::h7f8cb65ce2d89ffap2b
  27:     0x7f70f10f380a - thunk::F.Invoke<A, R>::invoke::h2974104229743277102
  28:     0x7f70f10f2a59 - rt::unwind::try::try_fn::h13662642278760376923
  29:     0x7f70f0a70768 - rust_try_inner
  30:     0x7f70f0a70755 - rust_try
  31:     0x7f70f10f2db7 - thunk::F.Invoke<A, R>::invoke::h8690323944732705129
  32:     0x7f70f09e1291 - sys::thread::create::thread_start::hf9ba913050908365ulI
  33:     0x7f70ea7fe181 - start_thread
  34:     0x7f70f058347c - __clone
  35:                0x0 - <unknown>
