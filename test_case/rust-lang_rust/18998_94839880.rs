
$ cat main.rs
use std::io::Write;

fn foo(bar: *const u8) -> *const Write {
    bar as *const Write
}

fn main() {
  let _ = foo(&7);
}
$ RUST_BACKTRACE=1 rustc main.rs
error: internal compiler error: translating unsupported cast: *const u8 (cast_pointer) -> *const std::io::Write (cast_other)
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/tamird/src/rust/src/libsyntax/diagnostic.rs:230

stack backtrace:
   1:        0x10380853f - sys::backtrace::write::h8b9cc0e6b4558d6fnSs
   2:        0x103810ad2 - panicking::on_panic::hb84a54d9bdda0209CPw
   3:        0x1037c6cd5 - rt::unwind::begin_unwind_inner::h12cc96d4563962fclxw
   4:        0x102fce60e - rt::unwind::begin_unwind::h8554955808482497848
   5:        0x102fcedb2 - diagnostic::Handler::bug::hf9d658ba3ad60a681wB
   6:        0x100d57de7 - session::Session::bug::h996fc02cafac252bClp
   7:        0x10057116c - trans::expr::trans_imm_cast::hc380d2dff304ead8uoC
   8:        0x10056511e - trans::expr::trans_unadjusted::hda4f420c51313d0bbBA
   9:        0x1004991ef - trans::expr::trans::h9b4ecaab7176a08bieA
  10:        0x100539150 - trans::expr::trans_into::h054f76c51f01e5feZ7z
  11:        0x1004ba4b9 - trans::controlflow::trans_block::hcf83ff649913f4b923u
  12:        0x1004b8db6 - trans::base::trans_closure::hc90fb8352e6a23e9BDh
  13:        0x1004baaee - trans::base::trans_fn::h30ceafdd164c6b32jOh
  14:        0x1004be0e8 - trans::base::trans_item::hef31bbee38b11859vci
  15:        0x1004ccf30 - trans::base::trans_crate::ha0e1db7844d97546v1i
  16:        0x10032e2ae - driver::phase_4_translate_to_llvm::h62a23938e2c925f3nOa
  17:        0x100306579 - driver::compile_input::hb2bddda14f816e70Qba
  18:        0x1003cdd03 - run_compiler::h0b99cbf9200c123bF4b
  19:        0x1003cb82a - boxed::F.FnBox<A>::call_box::h12835505560322522005
  20:        0x1003cacc7 - rt::unwind::try::try_fn::h9284484278680574813
  21:        0x10389d028 - rust_try_inner
  22:        0x10389d015 - rust_try
  23:        0x1003cafa0 - boxed::F.FnBox<A>::call_box::h15674394682208383701
  24:        0x10380f63d - sys::thread::create::thread_start::hdaf0bf4a4fc759cbeVv
  25:     0x7fff8825a898 - _pthread_body
  26:     0x7fff8825a729 - _pthread_start
