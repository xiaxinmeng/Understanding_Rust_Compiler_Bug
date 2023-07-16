
% time RUST_MAX_STACK=10000000 x86_64-apple-darwin/stage1/bin/rustc --cfg stage1    --target=x86_64-apple-darwin  --cfg rustc -o x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/bin/rustc /Users/fklock/Dev/Mozilla/rust.git/src/driver/driver.rs
rust: task 7fbb50d00000 ran out of stack

real    0m0.376s
user    0m0.364s
sys 0m0.011s
% time RUST_MAX_STACK=100000000 x86_64-apple-darwin/stage1/bin/rustc --cfg stage1    --target=x86_64-apple-darwin  --cfg rustc -o x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/bin/rustc /Users/fklock/Dev/Mozilla/rust.git/src/driver/driver.rs
rust: task 7f950ad00000 ran out of stack

real    0m3.734s
user    0m3.694s
sys 0m0.039s
% time RUST_MAX_STACK=1000000000 x86_64-apple-darwin/stage1/bin/rustc --cfg stage1    --target=x86_64-apple-darwin  --cfg rustc -o x86_64-apple-darwin/stage1/lib/rustc/x86_64-apple-darwin/bin/rustc /Users/fklock/Dev/Mozilla/rust.git/src/driver/driver.rs
rust: task 7fdde2d00000 ran out of stack

real    0m37.261s
user    0m36.967s
sys 0m0.295s
