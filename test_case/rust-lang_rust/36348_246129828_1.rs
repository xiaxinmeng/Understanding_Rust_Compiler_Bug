
$ cat ./recomp-gdb.sh 
#!/bin/sh
exec gdb -d "../llvm-toolchain-3.8-3.8.1" -ex "set debug-file-directory $HOME/vroot/usr/lib/debug" -ex "run ./recomp.sh" sh

$ cat ./recomp.sh 
#!/bin/sh
export RUSTC_BOOTSTRAP_KEY=39b92f95
export RUST_BACKTRACE=1
export LD_LIBRARY_PATH=/home/infinity0/vroot/usr/lib/llvm-3.8/lib
# the below is what the test claims to be running, c+p from its output:
exec aarch64-unknown-linux-gnu/stage2/bin/rustc \
/home/infinity0/rustc-1.11.0+dfsg1/src/test/debuginfo/type-names.rs \
-L aarch64-unknown-linux-gnu/test/debuginfo-gdb/ \
--target=aarch64-unknown-linux-gnu \
-L aarch64-unknown-linux-gnu/test/debuginfo-gdb/type-names.stage2-aarch64-unknown-linux-gnu.debuginfo-gdb.libaux \
-C prefer-dynamic \
-o aarch64-unknown-linux-gnu/test/debuginfo-gdb/type-names.stage2-aarch64-unknown-linux-gnu \
-C link-args=-Wl,-z,relro --cfg rtopt -C rpath -L aarch64-unknown-linux-gnu/rt -g
