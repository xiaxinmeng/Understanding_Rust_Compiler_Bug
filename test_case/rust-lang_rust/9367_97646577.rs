 sh
RUSTLIB=/usr/local/lib/rustlib/x86_64-apple-darwin/lib
rustc --emit obj -Z no-landing-pads -C no-stack-check test.rs
$LLVM/lld -flavor darwin -arch x86_64 test.o $RUSTLIB/libmorestack.a -L $RUSTLIB -lstd-4e7c5e5c -lSystem -lcompiler-rt -o test
