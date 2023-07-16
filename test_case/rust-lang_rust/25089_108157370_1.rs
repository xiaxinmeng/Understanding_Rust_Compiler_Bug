
$ rustc foo.rs && ./foo
thread '<main>' panicked at 'test panic', foo.rs:18
thread '<main>' panicked at 'assertion failed: !DROPPED', foo.rs:7
stack backtrace:
   1:        0x10798ee45 - sys::backtrace::write::h6ede916059f8deceM9r
   2:        0x1079920c7 - panicking::on_panic::ha49f13d0ffb0f10e2uw
   3:        0x10798d448 - rt::unwind::begin_unwind_inner::hf0bdcaef6fa2453akdw
   4:        0x10798bade - rt::unwind::begin_unwind::h13318957588857980774
   5:        0x10798ba3e - Foo.Drop::drop::h2c1b207a3b565791kaa
   6:        0x10798bf32 - Foo::drop.886::haa7535a0227f2840
   7:        0x10798c02c - main::hdaf364a19f487aa9Dba
   8:        0x107992f58 - rust_try_inner
   9:        0x107992f45 - rust_try
  10:        0x107992750 - rt::lang_start::h5cc5e69c1f0d126e6pw
  11:        0x10798c08e - main
thread panicked while panicking. aborting.
zsh: illegal hardware instruction  ./foo
