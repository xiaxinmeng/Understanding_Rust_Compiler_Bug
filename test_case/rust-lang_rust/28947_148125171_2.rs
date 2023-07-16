
$ make clean
...
$ rm -rf x86_64-unknown-linux-gnu
...
$ ./configure --prefix=/home/zazdxscf/build/1nonpkgs/rust/usr/local --disable-rpath --enable-manage-submodules --disable-clang --enable-ccache --enable-dist-host-only --disable-valgrind --disable-helgrind --disable-valgrind-rpass --python=/usr/bin/python2 --enable-optimize --enable-optimize-cxx --enable-optimize-llvm --enable-debug --enable-debuginfo --enable-debug-assertions --enable-debuginfo-tests --enable-llvm-assertions --enable-debug-jemalloc --disable-local-rust --enable-local-rust --local-rust-root=/usr --llvm-root=/usr
...
$ make -j1 -- all NO_REBUILD=1 TIME_PASSES=1 TIME_LLVM_PASSES=1 'RUSTFLAGS=--verbose -C llvm-args=-verify-each'
...
time: 0.073; rss: 172MB stability checking
time: 0.000; rss: 172MB unused lib feature checking
src/libcore/lib.rs:82:12: 82:29 warning: unused or unknown feature, #[warn(unused_features)] on by default
src/libcore/lib.rs:82 #![feature(unwind_attributes)]
                                 ^~~~~~~~~~~~~~~~~
src/libcore/panicking.rs:65:9: 65:18 warning: unused attribute, #[warn(unused_attributes)] on by default
src/libcore/panicking.rs:65         #[unwind]
                                    ^~~~~~~~~
time: 1.496; rss: 172MB lint checking
time: 0.000; rss: 172MB resolving dependency formats
rustc: Unknown command line argument '-verify-each'.  Try: 'rustc -help'
rustc: Did you mean '-verify-scev'?
/home/zazdxscf/build/1nonpkgs/rust/rust/mk/target.mk:164: recipe for target 'x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.core' failed
make: *** [x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/lib/stamp.core] Error 1
