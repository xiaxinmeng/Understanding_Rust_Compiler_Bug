
$ echo "type T = Iterator::Item;" > panic.rs
$ RUST_BACKTRACE=1 rustc --crate-type=lib panic.rs
ERROR:rbml::reader: failed to find block with tag 7
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'explicit panic', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librbml/lib.rs:262

stack backtrace:
   1:     0x7f9ce7fb75e0 - sys::backtrace::write::h4d3bbe08a44dee4aRbu
   2:     0x7f9ce7fdace0 - failure::on_fail::h53efd1909aa8a1f2jmB
   3:     0x7f9ce7f47050 - rt::unwind::begin_unwind_inner::h0e24af2cd7543451k1A
   4:     0x7f9ce12bb430 - rt::unwind::begin_unwind::h2286129482268768168
   5:     0x7f9ce12ba1c0 - reader::get_doc::h6fcf164d61a0eed4nLa
   6:     0x7f9ce62aba20 - metadata::decoder::item_type::h124d99db0047034alTg
   7:     0x7f9ce62bc430 - metadata::decoder::get_type::h03c0f48119e223b8u5g
   8:     0x7f9ce626c150 - metadata::csearch::get_type::hcd92c9a503a2a41ckWj
   9:     0x7f9ce77da340 - collect::CollectCtxt<'a, 'tcx>.AstConv<'tcx>::get_item_type_scheme::h4e70e9be93fc772fIgu
  10:     0x7f9ce77c9230 - astconv::ast_path_to_ty::heb9bbc4a061d898eKSs
  11:     0x7f9ce77ca700 - astconv::ast_ty_to_ty::unboxed_closure.30768
  12:     0x7f9ce7765290 - astconv::ast_ty_to_ty::h84248d43c3bae556ujt
  13:     0x7f9ce77e86a0 - collect::ty_of_item::hd540332b324b20cfn6u
  14:     0x7f9ce77dfc40 - collect::convert::hbe3857c09eac11a1fIu
  15:     0x7f9ce78197d0 - check_crate::unboxed_closure.31906
  16:     0x7f9ce7817710 - check_crate::hc9b4c474446351a3YMy
  17:     0x7f9ce853b080 - driver::phase_3_run_analysis_passes::he5f3b198079d8d2eVFa
  18:     0x7f9ce85230b0 - driver::compile_input::ha4b908a84d39061dBba
  19:     0x7f9ce85e6d70 - run_compiler::h9744adc14d83069fR8b
  20:     0x7f9ce85e54e0 - thunk::F.Invoke<A, R>::invoke::h4743467232418998746
  21:     0x7f9ce85e4440 - rt::unwind::try::try_fn::h14171547017825018159
  22:     0x7f9ce8046040 - rust_try_inner
  23:     0x7f9ce8046030 - rust_try
  24:     0x7f9ce85e46f0 - thunk::F.Invoke<A, R>::invoke::h16899136376829984014
  25:     0x7f9ce7fc74d0 - sys::thread::thread_start::h7e7f9bf2088d1750X6w
  26:     0x7f9ce2234fe0 - start_thread
  27:     0x7f9ce7bd0c99 - __clone
  28:                0x0 - <unknown>
