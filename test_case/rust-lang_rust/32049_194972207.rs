
$ cargo rustc --target arm-unknown-linux-gnueabihf -- -O --emit asm -C save-temps
PromoteMemoryToRegister.cpp:531: void {anonymous}::PromoteMem2Reg::run(): Assertion `isAllocaPromotable(AI) && "Cannot promote non-promotable alloca!"' failed.
$ ls -lh target/arm-unknown-linux-gnueabihf/debug/*.bc
298K image.0.no-opt.bc
