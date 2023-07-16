
---- engine::actor::new_0 stdout ----
    error: linking with `cc` failed: exit code: 1
note: cc '-Wl,--as-needed' '-m64' 
'-L' '/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib' 
'-o' '/tmp/rustdoctest.NXfYvYNp7cWR/rust-out' '/tmp/rustdoctest.NXfYvYNp7cWR/rust_out.o' 
'-Wl,--whole-archive' 
'-lmorestack' 
'-Wl,--no-whole-archive' 
'-Wl,--gc-sections' '-pie' 
'-nodefaultlibs' 
'/home/jonny/code/rslike/target/librslike-33665a348a9c0d5a.rlib' '/home/jonny/code/rslike/target/deps/libtcod-672eed4d6de26d21.rlib' '/home/jonny/code/rslike/target/deps/libbitflags-57b03d5337bba57b.rlib' '/home/jonny/code/rslike/target/deps/libtcod-sys-62882c83ff6a5d9c.rlib' 
'-L' '/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib' 
'-lstd-4e7c5e5c' '-L' '/home/jonny/code/rslike/target' 
'-L' '/home/jonny/code/rslike/target/deps' 
'-L' '/usr/local/lib/rustlib/x86_64-unknown-linux-gnu/lib' 
'-L' '/home/jonny/code/rslike/.rust/lib/x86_64-unknown-linux-gnu' 
'-L' '/home/jonny/code/rslike/lib/x86_64-unknown-linux-gnu' 
'-Wl,--whole-archive' 
'-Wl,-Bstatic' 
'-Wl,--no-whole-archive' 
'-Wl,-Bdynamic' 
'-ltcod' 
'-ltcod' 
'-ltcod' 
'-ldl' 
'-lpthread' 
'-lrt' 
'-lgcc_s' 
'-lpthread' 
'-lc' 
'-lm' 
'-lcompiler-rt'
note: /usr/bin/ld: cannot find -ltcod
/usr/bin/ld: cannot find -ltcod
/usr/bin/ld: cannot find -ltcod
collect2: error: ld returned 1 exit status

error: aborting due to previous error
thread 'engine::actor::new_0' panicked at 'Box<Any>', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libsyntax/diagnostic.rs:151
