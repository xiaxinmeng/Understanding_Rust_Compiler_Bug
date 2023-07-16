
Starting program: /tmp/winapi-rs/lib/kernel32/target/release/build/kernel32-sys-70200ca6b7279963/build-script-build 
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Program received signal SIGILL, Illegal instruction.
0x0000555555559946 in core::result::unwrap_failed::h98e39d97d47cde50 ()
(gdb) bt
#0  0x0000555555559946 in core::result::unwrap_failed::h98e39d97d47cde50 ()
#1  0x000055555555a0c8 in build::link::h504eeff8ebb5a460 ()
#2  0x000055555556d47b in panic_unwind::__rust_maybe_catch_panic () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libpanic_unwind/lib.rs:97
#3  0x000055555556366b in std::panicking::try<(),fn()> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panicking.rs:332
#4  std::panic::catch_unwind<fn(),()> () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/panic.rs:351
#5  std::rt::lang_start () at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/../src/libstd/rt.rs:57
#6  0x00007ffff7424291 in __libc_start_main () from /usr/lib/libc.so.6
#7  0x00005555555597da in _start ()
