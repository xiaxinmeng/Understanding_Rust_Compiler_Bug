
compile_and_link: x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/bin/rustc
error: linking with `cc` failed: exit code: 1
note: cc arguments: -m64 -L/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -o x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/bin/rustc x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/bin/rustc.o -Wl,--as-needed -Wl,-O1 -L/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lstd-6425b930ca146ae9-0.9-pre -L/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lrustuv-a13edc95d75df17-0.9-pre -L/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lextra-aaa96aab146eb38e-0.9-pre -L/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lsyntax-2bb2d559d93ae8f0-0.9-pre -L/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -lrustc-8581899a03b03e-0.9-pre -Lx86_64-unknown-linux-gnu/rt/uv_support -Lx86_64-unknown-linux-gnu/rt/libuv -L/home/keegan/proj/rust/rust/.rust -L/home/keegan/proj/rust/rust -lmorestack -Wl,-rpath,$ORIGIN/../lib -Wl,-rpath,/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib -Wl,-rpath,/usr/local/lib/rustc/x86_64-unknown-linux-gnu/lib
note: /home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libextra-aaa96aab146eb38e-0.9-pre.so: error: undefined reference to 'ceil'
/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libextra-aaa96aab146eb38e-0.9-pre.so: error: undefined reference to 'log10'
/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libextra-aaa96aab146eb38e-0.9-pre.so: error: undefined reference to 'fmod'
/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libextra-aaa96aab146eb38e-0.9-pre.so: error: undefined reference to 'floor'
/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libextra-aaa96aab146eb38e-0.9-pre.so: error: undefined reference to 'trunc'
/home/keegan/proj/rust/rust/x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/lib/libextra-aaa96aab146eb38e-0.9-pre.so: error: undefined reference to 'pow'
collect2: error: ld returned 1 exit status

error: aborting due to previous error
task 'rustc' failed at 'explicit failure', /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/libsyntax/diagnostic.rs:102
task '<main>' failed at 'explicit failure', /home/rustbuild/src/rust-buildbot/slave/snap3-linux/build/src/librustc/lib.rs:396
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustc/x86_64-unknown-linux-gnu/bin/rustc] Error 101
