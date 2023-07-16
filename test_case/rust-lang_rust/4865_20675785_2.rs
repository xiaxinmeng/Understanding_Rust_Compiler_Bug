
% rustc --lib foo.rs
warning: missing crate link meta `name`, using `foo` as default
warning: missing crate link meta `vers`, using `0.0` as default
% rustc -L. bar.rs
error: linking with `cc` failed with code 1
note: cc arguments: -L/home/philipp/programming/rust/x86_64-unknown-linux-gnu/stage2/lib/rustc/x86_64-unknown-linux-gnu/lib -m64 -o bar bar.o -L/home/philipp/programming/rust/x86_64-unknown-linux-gnu/stage2/lib/rustc/x86_64-unknown-linux-gnu/lib -lstd-6c65cf4b443341b1-0.8-pre -L. -lfoo-15fb3a718ea23983-0.0 -lrustrt -lrt -lpthread -L. -lrt -ldl -lm -lmorestack -lrustrt -Wl,-rpath,$ORIGIN/../../programming/rust/x86_64-unknown-linux-gnu/stage2/lib/rustc/x86_64-unknown-linux-gnu/lib -Wl,-rpath,$ORIGIN/. -Wl,-rpath,/home/philipp/programming/rust/x86_64-unknown-linux-gnu/stage2/lib/rustc/x86_64-unknown-linux-gnu/lib -Wl,-rpath,/home/philipp/tmp/6477/. -Wl,-rpath,/usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib
note: bar.o: In function `main::_c6c3cd618d2fde1::_0$x2e0':
bar.rc:(.text+0x2c): undefined reference to `foo::foo::_15bec6257b8bff78::_0$x2e0'
collect2: Fehler: ld gab 1 als Ende-Status zurück

error: aborting due to previous error
