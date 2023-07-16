
thread 'rustc' panicked at 'assertion failed: index < LLVMCountParams(llfn)', /Users/mark/external/rust/src/librustc_llvm/lib.rs:2216

stack backtrace:
   1:        0x106e3cb5f - sys::backtrace::write::hb71c0fe5c1369667t1r
   2:        0x106e45192 - panicking::on_panic::hbb1dc9652116e98atZv
   3:        0x106e01b15 - rt::unwind::begin_unwind_inner::h574b1d468dd56a51cHv
   4:        0x104b86bbf - rt::unwind::begin_unwind::h3614243639685568826
   5:        0x104b86b31 - get_param::h862e3f7f0f51b243dMc
   6:        0x103e264e5 - trans::base::create_datums_for_fn_args::hf0f3ad9707ccc7e3iqh
   7:        0x103e2971f - trans::base::trans_closure::h81642f970ec36bb6LCh
   8:        0x103e2d0be - trans::base::trans_fn::hd09832dd639fc1d5tNh
   9:        0x103e31149 - trans::base::trans_item::h6f9b7deff3559934Fbi
  10:        0x103e3f472 - trans::base::trans_crate::hf3ed54ad7ac3e76cF0i
  11:        0x1038c753e - driver::phase_4_translate_to_llvm::h2764d79a94c634c7hOa
  12:        0x10389f4c4 - driver::compile_input::h67c8d1668241c8b7Qba
  13:        0x103967983 - run_compiler::h76479bdf3aa57ee3z4b
  14:        0x1039654aa - boxed::F.FnBox<A>::call_box::h18367457670158931534
  15:        0x103964947 - rt::unwind::try::try_fn::h9857878285588690876
  16:        0x106ec9568 - rust_try_inner
  17:        0x106ec9555 - rust_try
  18:        0x103964c20 - boxed::F.FnBox<A>::call_box::h17442177368717038387
  19:        0x106e43cdd - sys::thread::create::thread_start::h43644ebd14c7598dL4u
  20:     0x7fff8c37f267 - _pthread_body
  21:     0x7fff8c37f1e4 - _pthread_start
