
(gdb) bt
#0  0x00007ffff6290877 in raise () from /lib64/libc.so.6
#1  0x00007ffff6291f68 in abort () from /lib64/libc.so.6
#2  0x00007ffff62897d6 in __assert_fail_base () from /lib64/libc.so.6
#3  0x00007ffff6289882 in __assert_fail () from /lib64/libc.so.6
#4  0x00007ffff4b97e16 in llvm::InvokeInst::init(llvm::Value*, llvm::BasicBlock*, llvm::BasicBlock*, llvm::ArrayRef<llvm::Value*>, llvm::Twine const&) ()
   from /home/bkoropoff/Software/rust/bin/../lib/./librustc_llvm-4e7c5e5c.so
#5  0x00007ffff4af2e26 in LLVMBuildInvoke () from /home/bkoropoff/Software/rust/bin/../lib/./librustc_llvm-4e7c5e5c.so
#6  0x00007ffff70cfd3b in middle::trans::builder::Builder$LT$$x27a$C$$x20$x27tcx$GT$::invoke::h36c87f24c8cb5e332Id ()
   from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#7  0x00007ffff70cf5fa in middle::trans::build::Invoke::heaf58bcd916273adJAc () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#8  0x00007ffff7067b93 in middle::trans::base::invoke::h02d93537d1fbb810l2f () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#9  0x00007ffff70561c8 in middle::trans::callee::trans_call_inner::h529356bd69297c3fc33 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#10 0x00007ffff709086f in middle::trans::expr::trans_overloaded_op::hc334cc867ac912aegj7 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#11 0x00007ffff708f73c in middle::trans::expr::trans_index::h0683c76cbef6a9ca7Q5 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#12 0x00007ffff7086e0f in middle::trans::expr::trans_unadjusted::h60f233b15d0ee0be4x5 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#13 0x00007ffff704e94c in middle::trans::expr::trans::h9668e97995c3f9db0Q4 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#14 0x00007ffff7092025 in middle::trans::expr::trans_addr_of::h926e1c2021bdb1b1lZ6 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#15 0x00007ffff7087305 in middle::trans::expr::trans_unadjusted::h60f233b15d0ee0be4x5 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#16 0x00007ffff704e94c in middle::trans::expr::trans::h9668e97995c3f9db0Q4 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#17 0x00007ffff7092025 in middle::trans::expr::trans_addr_of::h926e1c2021bdb1b1lZ6 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#18 0x00007ffff7087305 in middle::trans::expr::trans_unadjusted::h60f233b15d0ee0be4x5 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#19 0x00007ffff704d4ab in middle::trans::expr::trans_into::h39a29d37016f65e86M4 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#20 0x00007ffff709acb0 in middle::trans::expr::trans_adt::hd3f0bde1a953799fFJ6 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#21 0x00007ffff708898d in middle::trans::expr::trans_rvalue_dps_unadjusted::h01c9585b496c4288vb6 ()
   from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#22 0x00007ffff7086aa9 in middle::trans::expr::trans_unadjusted::h60f233b15d0ee0be4x5 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#23 0x00007ffff704e94c in middle::trans::expr::trans::h9668e97995c3f9db0Q4 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#24 0x00007ffff7098c52 in middle::trans::_match::trans_match::h41c8b17632059349prk () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#25 0x00007ffff7088327 in middle::trans::expr::trans_rvalue_dps_unadjusted::h01c9585b496c4288vb6 ()
   from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#26 0x00007ffff704d48d in middle::trans::expr::trans_into::h39a29d37016f65e86M4 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#27 0x00007ffff704c80a in middle::trans::controlflow::trans_stmt_semi::hdc58fc6ebef4d6faT20 ()
   from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#28 0x00007ffff704be63 in middle::trans::controlflow::trans_stmt::h397895233afab202GY0 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#29 0x00007ffff704d6f9 in middle::trans::controlflow::trans_block::hb6681c04599bdda3M30 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#30 0x00007ffff70efe74 in middle::trans::base::trans_closure::h882e4d42d869331eZah () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#31 0x00007ffff7040cb8 in middle::trans::base::trans_fn::h665ac863fbdf3d77pmh () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#32 0x00007ffff703e273 in middle::trans::base::trans_item::ha94ea1421e357df5LFh () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#33 0x00007ffff70f8fd8 in middle::trans::base::trans_crate::h82a182542ea2b73dRDi () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
---Type <return> to continue, or q <return> to quit---
#34 0x00007ffff7533115 in driver::driver::phase_4_translate_to_llvm::h18a5a35c6c98ab34cXA ()
   from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#35 0x00007ffff752b303 in driver::driver::compile_input::hd53b9c320149dc7a6tA () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#36 0x00007ffff75adbb6 in driver::run_compiler::h5a2f74c8147703b6KgE () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#37 0x00007ffff75abbec in driver::run::closure.145379 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#38 0x00007ffff6d063b8 in task::TaskBuilder$LT$S$GT$::try_future::closure.103636 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#39 0x00007ffff6d062c3 in task::TaskBuilder$LT$S$GT$::spawn_internal::closure.103607 () from /home/bkoropoff/Software/rust/bin/../lib/librustc-4e7c5e5c.so
#40 0x00007ffff69d1c22 in task::NativeSpawner.Spawner::spawn::closure.8435 () from /home/bkoropoff/Software/rust/bin/../lib/libnative-4e7c5e5c.so
#41 0x00007ffff66dfa0c in rust_try_inner () from /home/bkoropoff/Software/rust/bin/../lib/librustrt-4e7c5e5c.so
#42 0x00007ffff66df9f6 in rust_try () from /home/bkoropoff/Software/rust/bin/../lib/librustrt-4e7c5e5c.so
#43 0x00007ffff66831b3 in unwind::try::hb5e64b370a12855dxGd () from /home/bkoropoff/Software/rust/bin/../lib/librustrt-4e7c5e5c.so
#44 0x00007ffff668307c in task::Task::run::h015667fe60aef08bnMc () from /home/bkoropoff/Software/rust/bin/../lib/librustrt-4e7c5e5c.so
#45 0x00007ffff69d1a27 in task::NativeSpawner.Spawner::spawn::closure.8373 () from /home/bkoropoff/Software/rust/bin/../lib/libnative-4e7c5e5c.so
#46 0x00007ffff66848e5 in thread::thread_start::hdd3d902ef54656f2C7c () from /home/bkoropoff/Software/rust/bin/../lib/librustrt-4e7c5e5c.so
#47 0x00007ffff176fee5 in start_thread () from /lib64/libpthread.so.0
#48 0x00007ffff634fb8d in clone () from /lib64/libc.so.6
