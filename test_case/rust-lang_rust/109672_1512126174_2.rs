bash
rustc -O test.rs --crate-type staticlib
gcc test.c libtest.a
./a.out
