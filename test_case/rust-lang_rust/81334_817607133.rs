
(gdb) r --crate-name cfg_if --edition=2018 /home/glaubitz/rustc/rustc-1.50.0+dfsg1/vendor/cfg-if-0.1.10/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -Cembed-bitcode=no -C debuginfo=2 -C metadata=144fa4f9a0a129e5 -C extra-filename=-144fa4f9a0a129e5 --out-dir /home/glaubitz/rustc/rustc-1.50.0+dfsg1/build/bootstrap/debug/deps -L dependency=/home/glaubitz/rustc/rustc-1.50.0+dfsg1/build/bootstrap/debug/deps --cap-lints warn -Cdebuginfo=2 -C linker=powerpc-linux-gnu-gcc -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings
Starting program: /home/glaubitz/tmp/usr/local/bin/rustc --crate-name cfg_if --edition=2018 /home/glaubitz/rustc/rustc-1.50.0+dfsg1/vendor/cfg-if-0.1.10/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -Cembed-bitcode=no -C debuginfo=2 -C metadata=144fa4f9a0a129e5 -C extra-filename=-144fa4f9a0a129e5 --out-dir /home/glaubitz/rustc/rustc-1.50.0+dfsg1/build/bootstrap/debug/deps -L dependency=/home/glaubitz/rustc/rustc-1.50.0+dfsg1/build/bootstrap/debug/deps --cap-lints warn -Cdebuginfo=2 -C linker=powerpc-linux-gnu-gcc -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/lib/powerpc-linux-gnu/libthread_db.so.1".
[New Thread 0xede3f0f0 (LWP 256221)]
{"artifact":"/home/glaubitz/rustc/rustc-1.50.0+dfsg1/build/bootstrap/debug/deps/cfg_if-144fa4f9a0a129e5.d","emit":"dep-info"}

Thread 2 "rustc" received signal SIGILL, Illegal instruction.
[Switching to Thread 0xede3f0f0 (LWP 256221)]
0xf4f59bbc in std::thread::local::fast::Key$LT$T$GT$::try_initialize::h23c87cba01958f05 ()
   from /home/glaubitz/tmp/usr/local/bin/../lib/librustc_driver-9cf50d6324ac9732.so
(gdb) bt
#0  0xf4f59bbc in std::thread::local::fast::Key$LT$T$GT$::try_initialize::h23c87cba01958f05 ()
   from /home/glaubitz/tmp/usr/local/bin/../lib/librustc_driver-9cf50d6324ac9732.so
Backtrace stopped: previous frame identical to this frame (corrupt stack?)
(gdb)
