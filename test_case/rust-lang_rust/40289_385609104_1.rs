 console
$ cargo build
$ nm -C target/debug/libfoo.rlib

foo-d351a64747b10042.18cd33xhzunvqw3h.rcgu.o:
0000000000000000 r STATIC

$ cargo build --release
$ nm -C target/release/libfoo.rlib

foo-f1e69902e2c50980.foo0-f8ffab837addfc521cde53270051d9b7.rs.rcgu.o:
0000000000000000 r STATIC
