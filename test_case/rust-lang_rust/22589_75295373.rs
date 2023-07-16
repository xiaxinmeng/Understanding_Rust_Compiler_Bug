
use std::old_io;
fn main()
{
    let s = old_io::stdin().read_line().ok().expect("");
    if let Result<n2,_> = s.trim().parse() {};
}

bug01.rs:5:12: 5:18 error: internal compiler error: ident only path should have been covered already
bug01.rs:5     if let Result<n2,_> = s.trim().parse() {};
                      ^~~~~~
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:        0x1109aebe3 - sys::backtrace::write::hc8e3cee73e646c590nC
   2:        0x1109dcbe5 - panicking::on_panic::h00b47941f5bc8a02HOL
   3:        0x110906418 - rt::unwind::begin_unwind_inner::h539538ef7f909326UvL
   4:        0x10ffbadcf - rt::unwind::begin_unwind::h8012941227422632356
   5:        0x10ffbad7c - diagnostic::SpanHandler::span_bug::hef0e2599b36810bavYE
   6:        0x10ffe3b15 - parse::parser::Parser<'a>::parse_pat::hb3cc0e8f43f24f14wLL
   7:        0x10fff58b1 - parse::parser::Parser<'a>::parse_if_let_expr::h2bc85cad05578cb3dsL
   8:        0x10ffec629 - parse::parser::Parser<'a>::parse_bottom_expr::hfdb4e11f0fe78545KwK
   9:        0x10fff1613 - parse::parser::Parser<'a>::parse_dot_or_call_expr::h29c18bf2a027a407LPK
  10:        0x10fff45e6 - parse::parser::Parser<'a>::parse_prefix_expr::h8fbc8afb82429f45pcL
  11:        0x10fff4cf3 - parse::parser::Parser<'a>::parse_binops::hfed7b8b660ee0d91shL
  12:        0x10fff537a - parse::parser::Parser<'a>::parse_assign_expr::hcc1281fbca4ab686imL
  13:        0x10fff7bd1 - parse::parser::Parser<'a>::parse_stmt::h63ef22dacc2ade27v4L
  14:        0x1100015c0 - parse::parser::Parser<'a>::parse_block_tail_::hb6267ed63d0a6ad7YfM
  15:        0x10ffde668 - parse::parser::Parser<'a>::parse_inner_attrs_and_block::h04ad46fd719258bcieM
  16:        0x110005cf0 - parse::parser::Parser<'a>::parse_item_fn::h76bb729755ce656afVM
  17:        0x10fffbe9a - parse::parser::Parser<'a>::parse_item_::hc402e07115996d8fzUN
  18:        0x11000c751 - parse::parser::Parser<'a>::parse_mod_items::h12f6ab79c6414614PjN
  19:        0x1100183b3 - parse::parser::Parser<'a>::parse_crate_mod::h66d7caf303a06621FlO
  20:        0x11002ce53 - parse::parse_crate_from_file::hd404c1ea18c1aca1vNV
  21:        0x10ccce166 - driver::phase_1_parse_input::closure.16949
  22:        0x10cca8113 - driver::phase_1_parse_input::h9979f32fb2edc149Qra
  23:        0x10cca4d79 - driver::compile_input::h99719101b033d8dfGba
  24:        0x10cd7e8a7 - run_compiler::hbb4536200a8af2f8Zbc
  25:        0x10cd7bd21 - thunk::F.Invoke<A, R>::invoke::h2149495261217397749
  26:        0x10cd7a980 - rt::unwind::try::try_fn::h9879759663745361221
  27:        0x110a567e9 - rust_try_inner
  28:        0x110a567d6 - rust_try
  29:        0x10cd7b0e5 - thunk::F.Invoke<A, R>::invoke::h14352669848700948155
  30:        0x1109c5ac3 - sys::thread::thread_start::h9e3555425c6999b2e4G
  31:     0x7fff91e71899 - _pthread_body
  32:     0x7fff91e7172a - _pthread_struct_init

