
stack backtrace:
   1:     0x7fe8541e9540 - sys::backtrace::write::h327eaaaf8d06f8cfpRt
   2:     0x7fe85420c3d0 - failure::on_fail::ha67ff6264214772164z
   3:     0x7fe854177610 - rt::unwind::begin_unwind_inner::he9a71b1eb010e0b8ZJz
   4:     0x7fe8536290c0 - rt::unwind::begin_unwind::h7390311291878927563
   5:     0x7fe85364e7e0 - std..collections..hash..table..RawTable<syntax..ast..Name, (*>::glue_drop.14264::hd720afd492e3b274
   6:     0x7fe853661540 - std..collections..hash..table..RawTable<u32, std..collections..hash..set..HashSet<u32, std..collections..hash..state..DefaultState<rustc..util..nodemap..FnvHasher>>>::glue_drop.14543::hf7a3bf470f821dfd
   7:     0x7fe853698840 - Resolver::glue_drop.15449::haa86d9eeaa46612e
   8:     0x7fe853698040 - resolve_crate::h166b19f32b64ffdaByi
   9:     0x7fe854744cf0 - driver::phase_3_run_analysis_passes::h13374e85a1a62731EEa
  10:     0x7fe854732a20 - driver::compile_input::h271893b1aec24c35Aba
  11:     0x7fe8547f8b50 - run_compiler::hdec9687b5f4662f2e5b
  12:     0x7fe8547f72c0 - thunk::F.Invoke<A, R>::invoke::h15654788930951088931
  13:     0x7fe8547f61c0 - rt::unwind::try::try_fn::h10306438865730164293
  14:     0x7fe85427c770 - rust_try_inner
  15:     0x7fe85427c760 - rust_try
  16:     0x7fe8547f6470 - thunk::F.Invoke<A, R>::invoke::h3452119925705501931
  17:     0x7fe8541f9e10 - sys::thread::thread_start::h60a75d433610076acJw
  18:     0x7fe84f123250 - start_thread
  19:     0x7fe853e27219 - clone
  20:                0x0 - <unknown>
