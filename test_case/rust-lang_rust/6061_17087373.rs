
% RUST_LOG=rustc=1,::rt::backtrace make -j1 VERBOSE=1
cfg: build triple x86_64-apple-darwin
cfg: host triples x86_64-apple-darwin
cfg: target triples x86_64-apple-darwin
cfg: disabling rustc optimization (CFG_DISABLE_OPTIMIZE)
cfg: host for x86_64-apple-darwin is x86_64
cfg: os for x86_64-apple-darwin is apple-darwin
cfg: using clang
x86_64-apple-darwin/stage1/bin/rustc --cfg stage1    --target=x86_64-apple-darwin  --cfg rustc -o x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/bin/rustc /Users/fklock/Dev/Mozilla/rust.git/src/driver/driver.rs
rust: task 7fca11d00000 ran out of stack
cp x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/bin/rustc x86_64-apple-darwin/stage2/bin/rustc
cp: x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/bin/rustc: No such file or directory
make: *** [x86_64-apple-darwin/stage2/bin/rustc] Error 1
