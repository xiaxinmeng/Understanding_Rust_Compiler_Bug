
thread '<main>' panicked at 'explicit panic', panic.rs:2
stack backtrace:
   1:     0x7fbdbdb2d5f9 - sys::backtrace::write::h6bf89e75ba594056cRC
   2:     0x7fbdbdb308f9 - panicking::on_panic::hbf9e41a2aefa0bc6ghJ
   3:     0x7fbdbdb2ad62 - rt::unwind::begin_unwind_inner::h41f8448b15c8a007oWI
   4:     0x7fbdbdb2a16e - rt::unwind::begin_unwind::h17882462255754141092
                        at /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/rt/unwind.rs:522
   5:     0x7fbdbdb2a0ba - foo::h846a49565d377979eaa
                        at /home/huon/projects/test-rust/<std macros>:3
   6:     0x7fbdbdb2a4bd - bar::h20543e655da0f511Eaa
                        at /home/huon/projects/test-rust/panic.rs:5
   7:     0x7fbdbdb2a4ed - main::h32ccc23e635fe73eJaa
                        at /home/huon/projects/test-rust/panic.rs:8
   8:     0x7fbdbdb34148 - rust_try_inner
   9:     0x7fbdbdb34135 - rust_try
  10:     0x7fbdbdb31f9b - rt::lang_start::hba3ed105db79dd3fJbJ
  11:     0x7fbdbdb2a634 - main
  12:     0x7fbdbcd3bb44 - __libc_start_main
  13:     0x7fbdbdb29f58 - <unknown>
  14:                0x0 - <unknown>
