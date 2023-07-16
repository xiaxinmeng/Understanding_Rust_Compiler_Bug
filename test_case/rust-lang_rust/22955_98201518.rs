
$ RUST_BACKTRACE=1 LD_LIBRARY_PATH=$HOME/prefix/lib/ $HOME/prefix/bin/rustc 1.rs

1.rs:7:13: 7:21 warning: unused variable: `data_vec`, #[warn(unused_variables)] on by default
1.rs:7         let data_vec = data as *mut [u8];
                   ^~~~~~~~
error: internal compiler error: translating unsupported cast: *mut libc::types::common::c95::c_void (cast_pointer) -> *mut [u8] (cast_other)
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /home/asdf/dev/rust/src/libsyntax/diagnostic.rs:230

stack backtrace:
   1:     0x7f73c3005417 - sys::backtrace::write::hfb8b5b4dfa655b67qYr
                        at src/libstd/sys/unix/backtrace.rs:158
   2:     0x7f73c3030364 - panicking::on_panic::hf27ec90cc572c53alow
                        at src/libstd/panicking.rs:47
   3:     0x7f73c2f68e51 - rt::unwind::begin_unwind_inner::h3a3f805bb467efa2v3v
                        at src/libstd/rt/unwind.rs:569
   4:     0x7f73bea35b87 - rt::unwind::begin_unwind::h4664568208419770541
                        at src/libstd/rt/unwind.rs:522
   5:     0x7f73bea36941 - diagnostic::Handler::bug::hf6c874b0f2e6f03fOKB
                        at /home/asdf/dev/rust/<std macros>:3
   6:     0x7f73bfb20bf6 - session::Session::bug::h7ed65b4fa6bc9b00lzq
                        at src/librustc/session/mod.rs:193
   7:     0x7f73c161541b - trans::expr::trans_imm_cast::h1fa93edfd9165f96vpC
                        at src/librustc_trans/trans/expr.rs:2053
   8:     0x7f73c160f717 - trans::expr::trans_datum_unadjusted::h2a79b1ba683448a0IGA
                        at src/librustc_trans/trans/expr.rs:607
   9:     0x7f73c160750d - trans::expr::trans_unadjusted::ha952dc0116ec79cfcCA
                        at src/librustc_trans/trans/expr.rs:496
  10:     0x7f73c158b03a - trans::expr::trans_into::hcdb747bbcae3909808z
                        at src/librustc_trans/trans/expr.rs:188
  11:     0x7f73c1709ad0 - trans::_match::store_local::closure.53415
                        at src/librustc_trans/trans/_match.rs:1553
  12:     0x7f73c1709930 - trans::_match::mk_binding_alloca::h17107593058856989663
                        at src/librustc_trans/trans/_match.rs:1639
  13:     0x7f73c1391d3f - trans::_match::store_local::hc5e4ea77bbef8a463hI
                        at src/librustc_trans/trans/_match.rs:1551
  14:     0x7f73c13918fe - trans::base::init_local::h4f033efca6ba182d3Yg
                        at src/librustc_trans/trans/base.rs:854
  15:     0x7f73c1589df3 - trans::controlflow::trans_stmt::hdc919aefa7c8d6e2I1u
                        at src/librustc_trans/trans/controlflow.rs:64
  16:     0x7f73c13c456a - trans::controlflow::trans_block::h83c3cf9971c5ae67n6u
                        at src/librustc_trans/trans/controlflow.rs:113
  17:     0x7f73c13c12e1 - trans::base::trans_closure::h0c12b821f3101919IFh
                        at src/librustc_trans/trans/base.rs:1575
  18:     0x7f73c13c5271 - trans::base::trans_fn::h616bc9a7e6602788qQh
                        at src/librustc_trans/trans/base.rs:1626
  19:     0x7f73c13d3032 - trans::base::trans_item::h9c92c602e06681f1Cei
                        at src/librustc_trans/trans/base.rs:1974
  20:     0x7f73c13dedb4 - trans::base::trans_mod::h90f3d10fe0d34a1bdli
                        at src/librustc_trans/trans/base.rs:2072
  21:     0x7f73c13fc1ca - trans::base::trans_crate::h5b74408b5e06dd9ap3i
                        at src/librustc_trans/trans/base.rs:2678
  22:     0x7f73c387cd2b - driver::phase_4_translate_to_llvm::closure.23715
                        at src/librustc_driver/driver.rs:726
  23:     0x7f73c387c5ea - util::common::time::h17075660751370174737
                        at src/librustc/util/common.rs:39
  24:     0x7f73c36e31ed - driver::phase_4_translate_to_llvm::hfc84bb0501adeb47nOa
                        at src/librustc_driver/driver.rs:725
  25:     0x7f73c366114a - driver::compile_input::heca1e822607288f0Qba
                        at src/librustc_driver/driver.rs:141
  26:     0x7f73c38f74dc - run_compiler::ha5179f722f30471365b
                        at src/librustc_driver/lib.rs:156
  27:     0x7f73c38f3cac - run::closure.26490
                        at src/librustc_driver/lib.rs:99
  28:     0x7f73c38f2dd8 - monitor::closure.26462
                        at src/librustc_driver/lib.rs:813
  29:     0x7f73c38f2ca1 - boxed::F.FnBox<A>::call_box::h430058624187413269
                        at src/liballoc/boxed.rs:369
  30:     0x7f73c38f2857 - boxed::Box<FnBox<A, Output $u3d$$u20$R$GT$$u2b$$u20$Send$u20$$u2b$$u20$$u27$a$GT$.FnOnce$LT$A$GT$::call_once::h17097062596038723674
                        at src/liballoc/boxed.rs:385
  31:     0x7f73c38f1f13 - thread::Builder::spawn_inner::closure.26437
                        at src/libstd/thread/mod.rs:346
  32:     0x7f73c38f1e9e - rt::unwind::try::try_fn::__rust_abi::h9889257790502695346
                        at src/libstd/rt/unwind.rs:139
  33:     0x7f73c38f1e39 - rt::unwind::try::try_fn::h9889257790502695346
  34:     0x7f73c3164fd8 - rust_try_inner
  35:     0x7f73c3164fc5 - rust_try
  36:     0x7f73c38f132a - rt::unwind::try::h11333728198225189581
                        at src/libstd/rt/unwind.rs:125
  37:     0x7f73c38f10f0 - thread::Builder::spawn_inner::closure.26369
                        at src/libstd/thread/mod.rs:346
  38:     0x7f73c38f2acd - boxed::F.FnBox<A>::call_box::h6100598235081829283
                        at src/liballoc/boxed.rs:369
  39:     0x7f73c2ffca87 - boxed::Box<FnBox<A, Output $u3d$$u20$R$GT$$u2b$$u20$$u27$a$GT$.FnOnce$LT$A$GT$::call_once::h5868424356375581454
                        at src/liballoc/boxed.rs:377
  40:     0x7f73c2ffc987 - sys_common::thread::start_thread::h3330a160aae3ecc2o0q
                        at src/libstd/sys/common/thread.rs:30
  41:     0x7f73c302be94 - sys::thread::Thread::new::thread_start::__rust_abi
                        at src/libstd/sys/unix/thread.rs:77
  42:     0x7f73c302be74 - sys::thread::Thread::new::thread_start::hc1d63bdcdee02d59R9u
  43:     0x7f73bb6bf0a4 - start_thread
  44:     0x7f73c2bcb84c - clone
  45:                0x0 - <unknown>
