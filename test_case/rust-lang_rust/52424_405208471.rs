
(gdb) run
Starting program: /home/pi/lanlord

Program received signal SIGSEGV, Segmentation fault.
clone<std::thread::Inner> () at /checkout/src/liballoc/sync.rs:718
718	/checkout/src/liballoc/sync.rs: No such file or directory.
(gdb) bt
#0  clone<std::thread::Inner> () at /checkout/src/liballoc/sync.rs:718
#1  clone () at libstd/thread/mod.rs:1001
#2  {{closure}} () at libstd/sys_common/thread_info.rs:39
#3  {{closure}}<std::thread::Thread,closure> () at libstd/sys_common/thread_info.rs:33
#4  _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::try_with::h12e294fc1498f32b ()
    at libstd/thread/local.rs:294
#5  0x003d796c in std::panicking::rust_panic_with_hook::h2e0e3adddefcec58 ()
    at libstd/sys_common/thread_info.rs:26
#6  0x003d7674 in std::panicking::begin_panic::h024edfc32ac8659c ()
    at libstd/panicking.rs:409
#7  0x003e5da8 in {{closure}} () at libstd/sys_common/thread_info.rs:47
#8  try_with<core::cell::RefCell<core::option::Option<std::sys_common::thread_info::ThreadInfo>>,closure,()> () at libstd/thread/local.rs:294
#9  _$LT$std..thread..local..LocalKey$LT$T$GT$$GT$::with::hd042109b71d60bfc ()
    at libstd/thread/local.rs:248
#10 0x004d67f0 in main ()
