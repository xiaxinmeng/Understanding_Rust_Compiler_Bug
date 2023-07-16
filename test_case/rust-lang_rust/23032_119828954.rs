
#0  0x00007ffff3a41ee0 in LLVMTypeOf () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_llvm-7e44814b.so
#1  0x00007ffff635d99d in trans::consts::const_expr::h8acb773187e50bb5Pss () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_trans-7e44814b.so
#2  0x00007ffff63b0c79 in trans::consts::const_expr_unadjusted::h06541bee33f5159cZIs ()
   from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_trans-7e44814b.so
#3  0x00007ffff635d420 in trans::consts::const_expr::h8acb773187e50bb5Pss () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_trans-7e44814b.so
#4  0x00007ffff6355dc0 in trans::base::get_item_val::hc5b38e5ad51952cbVNi () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_trans-7e44814b.so
#5  0x00007ffff6352b4f in trans::base::trans_item::h48925c6cb6dfeb22Oqi () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_trans-7e44814b.so
#6  0x00007ffff63600df in trans::base::trans_crate::h8696565b15c10d31Lej () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_trans-7e44814b.so
#7  0x00007ffff7af8667 in driver::phase_4_translate_to_llvm::h2d179cc6784b73795Oa ()
   from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-7e44814b.so
#8  0x00007ffff7af1f35 in driver::phase_3_run_analysis_passes::closure.15599 () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-7e44814b.so
#9  0x00007ffff7aec1f5 in driver::phase_3_run_analysis_passes::h8865956255442467876 ()
   from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-7e44814b.so
#10 0x00007ffff7ad2643 in driver::compile_input::hcad48c94ba42de22Tba () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-7e44814b.so
#11 0x00007ffff7b9d0b4 in run_compiler::h81ca4dc28455e870k6b () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-7e44814b.so
#12 0x00007ffff7b9abee in boxed::F.FnBox$LT$A$GT$::call_box::h3765534436168023650 ()
   from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-7e44814b.so
#13 0x00007ffff7b9a4da in rt::unwind::try::try_fn::h10948129613882328285 () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-7e44814b.so
#14 0x00007ffff7610419 in rust_try_inner () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/libstd-7e44814b.so
#15 0x00007ffff7610406 in rust_try () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/libstd-7e44814b.so
#16 0x00007ffff7579198 in rt::unwind::try::inner_try::hc2015e0398f2bdedIyw () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/libstd-7e44814b.so
#17 0x00007ffff7b9a71b in boxed::F.FnBox$LT$A$GT$::call_box::h16611359455106826858 ()
   from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/librustc_driver-7e44814b.so
#18 0x00007ffff758c78d in sys::thread::Thread::new::thread_start::h825939a3e780a437vIv () from /home/ncameron/rust2/x86_64-unknown-linux-gnu/stage1/lib/libstd-7e44814b.so
#19 0x00007ffff1dcd6aa in start_thread (arg=0x7fffefbff700) at pthread_create.c:333
#20 0x00007ffff71cdeed in clone () at ../sysdeps/unix/sysv/linux/x86_64/clone.S:109
