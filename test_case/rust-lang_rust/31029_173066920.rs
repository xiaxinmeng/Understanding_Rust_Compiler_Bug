
        thread 'btree::map::test_basic_large' panicked at 'assertion failed: idx <= node.len()', src/libcollections/btree/node.rs:700
stack backtrace:
   1:      0x375d13bf428 - sys::backtrace::tracing::imp::write::h04d113a23f92dc85kju
   2:      0x375d13c7af7 - panicking::default_handler::h80171f446718b158mwy
   3:      0x375d138670e - sys_common::unwind::begin_unwind_inner::h996123888c11f344Rbt
   4:      0x375d1387798 - sys_common::unwind::begin_unwind_fmt::hb94bf4aab76db4d7Xat
   5:      0x375d13bcbb1 - rust_begin_unwind
   6:      0x375d14236ef - panicking::panic_fmt::hd38cddc432a305fbRZL
   7:      0x375d1423008 - panicking::panic::h32607c42eb190b3boYL
   8:       0x36484f1a66 - btree::map::BTreeMap<K, V>::remove::h11058900178034209718
   9:       0x36484ef838 - btree::map::test_basic_large::he236ebd0d3bf9d02Xyb
  10:      0x375d1ad28c6 - boxed::F.FnBox<A>::call_box::h13039866824781652362
  11:      0x375d1ad534b - sys_common::unwind::try::try_fn::h8239963937927191743
  12:      0x375d13bcb48 - __rust_try
  13:      0x375d13b10db - sys_common::unwind::try::inner_try::hb1d5bbff41ba6227p8s
  14:      0x375d1ad56ca - boxed::F.FnBox<A>::call_box::h6628856905804677820
  15:      0x375d13c5a63 - sys::thread::Thread::new::thread_start::h0fc09666d697035citx
  16:      0x375d10d34a3 - start_thread
  17:      0x375d0bf513c - clone
  18:                0x0 - <unknown>
