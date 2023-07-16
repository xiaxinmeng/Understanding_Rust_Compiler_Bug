
thread '<main>' panicked at 'assertion failed: `(left == right) && (right == left)` (left: `22`, right: `0`)', /home/mneumann/disk/neu/rust-cross-dragonfly/stage2-linux/rust/src/libstd/sys/unix/mutex.rs:53
stack backtrace:
   1:          0x1050730 - sys::backtrace::write::h6c2cb1f1132defcbQft
   2:          0x105e7a0 - failure::on_fail::h52cb2223a000b31akgz
   3:          0x102ce30 - rt::unwind::begin_unwind_inner::h5cadd3b8e3af94baoUy
   4:          0x102d780 - rt::unwind::begin_unwind_fmt::hdcce387453c77319VSy
   5:          0x1028240 - sys::mutex::Mutex::destroy::h944d68132516e13eymu
   6:          0x1028200 - sys_common::mutex::Mutex::destroy::h05060b7894b378a0U8w
   7:          0x10281c0 - sync::mutex::Mutex<T>.Drop::drop::h9736437216186091782
   8:          0x1028130 - std..sync..mutex..Mutex<bool>::glue_drop.1740::hc29669eec6ad0cb3
   9:          0x10280d0 - std..thread..Inner::glue_drop.1737::h852294eb2538a0c8
  10:          0x10280a0 - mem::drop::h5042620272860910571
  11:          0x1027da0 - arc::Arc<T>.Drop::drop::h2578377223763874141
  12:          0x1027d70 - alloc..arc..Arc<std..thread..Inner>::glue_drop.1734::hdefd71314919f852
  13:          0x1027d40 - std..thread..Thread::glue_drop.1731::h5e295f37c79aa596
  14:          0x102c900 - std..thread..JoinGuard<(*>::glue_drop.2140::h772f4f23477015a3
  15:          0x102c7e0 - thread::JoinGuard<T>::join::h10970293053447468842
  16:          0x1025630 - main::he5cc945d2fe13fb6faa
  17:          0x1062540 - rt::lang_start::unboxed_closure.33933
  18:          0x10624d0 - rt::unwind::try::try_fn::__rust_abi::h1667473091729999914
  19:          0x1062490 - rt::unwind::try::try_fn::h1667473091729999914
  20:          0x1070180 - rust_try_inner
  21:          0x1070170 - rust_try
  22:          0x1062060 - rt::unwind::try::h17451077907674403930
  23:          0x1061d30 - rt::lang_start::h35714ae31d7a9360V9y
  24:          0x10257b0 - main
  25:                0x0 - <unknown>
