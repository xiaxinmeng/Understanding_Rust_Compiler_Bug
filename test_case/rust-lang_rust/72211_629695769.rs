
rustc +nightly --crate-type=dylib -O test.rs && objdump -d -Mintel libtest.so | awk -v RS='' '/foo/' 
