
Thread 2 (Thread 0x7fffea5c8700 (LWP 29952)):
#0  0x0000000000000001 in ?? ()
#1  0x0000000000000008 in ?? ()
#2  0x00007fffea5b5328 in ?? ()
#3  0x00007fffea5b5328 in ?? ()
#4  0x0000000000000000 in ?? ()

Thread 1 (Thread 0x7ffff7fb1a00 (LWP 29947)):
#0  0x00007fffebbfe7dd in pthread_join (threadid=140737125320448, thread_return=0x0) at pthread_join.c:90
#1  0x00007ffff713a353 in std::sys::thread::{{impl}}::join (self=...) at /home/shum/src/rust/rust/src/libstd/sys/unix/thread.rs:152
#2  0x00007ffff772dd96 in std::thread::{{impl}}::join<()> (self=0x7fffffffcf30) at /home/shum/src/rust/rust/src/libstd/thread/mod.rs:627
#3  0x00007ffff772dcee in std::thread::{{impl}}::join<()> (self=...) at /home/shum/src/rust/rust/src/libstd/thread/mod.rs:687
#4  0x00007ffff7726cdc in rustc_driver::monitor<closure> (f=...) at /home/shum/src/rust/rust/src/librustc_driver/lib.rs:1050
#5  0x00007ffff76febcb in rustc_driver::run (args=...) at /home/shum/src/rust/rust/src/librustc_driver/lib.rs:136
#6  0x00007ffff778d8c8 in rustc_driver::main () at /home/shum/src/rust/rust/src/librustc_driver/lib.rs:1115
#7  0x000055555555493b in driver::main () at /home/shum/src/rust/rust/src/driver/driver.rs:21
#8  0x00007ffff713d5b6 in fn$LP$$RP$::fn_pointer_shim.32324::h35d87cadf14a8183 () at /home/shum/src/rust/rust/src/libcore/option.rs:323
#9  0x00007ffff713d2f3 in std::panicking::try::{{closure}}::{{closure}}<(),fn()> () at /home/shum/src/rust/rust/src/libstd/panicking.rs:250
#10 0x00007ffff713d5fd in std::panicking::try::call<closure> (f=0x7fffffffdb20) at /home/shum/src/rust/rust/src/libstd/panicking.rs:282
#11 0x00007ffff7167858 in __rust_try () from /home/shum/src/rust/rust/x86_64-unknown-linux-gnu/stage1/lib/libstd-f53fb285.so
#12 0x00007ffff71677a8 in panic_unwind::__rust_maybe_catch_panic (f=0x7ffff713d5e0 <std::panicking::try::call<closure>>, data=0x7fffffffdb20 "\340\334\377\377\377\177", data_ptr=0x7fffffffdb08, 
    vtable_ptr=0x7fffffffdb00) at /home/shum/src/rust/rust/src/libpanic_unwind/lib.rs:91
#13 0x00007ffff713d21b in std::panicking::try::{{closure}}<(),fn()> (s=0x7ffff7fb19e0) at /home/shum/src/rust/rust/src/libstd/panicking.rs:257
#14 0x00007ffff713d796 in std::thread::local::{{impl}}::with<core::cell::Cell<usize>,closure,core::result::Result<(), Box<Any>>> (
    self=0x7ffff74e9598 <std::panicking::PANIC_COUNT::h99b79c8949805cd2>, f=...) at /home/shum/src/rust/rust/src/libstd/thread/local.rs:245
#15 0x00007ffff713d11c in std::panicking::try<(),fn()> (f=0x555555554930 <driver::main>) at /home/shum/src/rust/rust/src/libstd/panicking.rs:245
#16 0x00007ffff713d06b in std::panic::catch_unwind<fn(),()> (f=0x555555554930 <driver::main>) at /home/shum/src/rust/rust/src/libstd/panic.rs:312
#17 0x00007ffff713cf69 in std::rt::lang_start (main=0x555555554930 <driver::main> "UH\211\345", <incomplete sequence \353>, argc=24, argv=0x7fffffffdf78)
    at /home/shum/src/rust/rust/src/libstd/rt.rs:58
#18 0x000055555555497a in main ()
