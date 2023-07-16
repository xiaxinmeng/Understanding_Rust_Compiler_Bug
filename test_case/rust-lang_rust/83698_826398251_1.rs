
$ rustc +stage1base -C lto -O test.rs --target wasm32-unknown-unknown && wc -c test.wasm
6873 test.wasm
$ rustc +stage1 -C lto -O test.rs --target wasm32-unknown-unknown && wc -c test.wasm
139666 test.wasm

$ rustc +stage1base -C lto -O test.rs && wc -c libtest.so
1534160 libtest.so
$ rustc +stage1 -C lto -O test.rs && wc -c libtest.so
1536632 libtest.so
