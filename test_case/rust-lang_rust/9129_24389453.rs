
% x86_64-apple-darwin/stage2/bin/rustc --cfg stage2   --cfg debug -Z no-debug-borrows \
  --target=x86_64-apple-darwin  --lib \
  -o x86_64-apple-darwin/stage2/lib/rustc/x86_64-apple-darwin/lib/librun_pass_stage2.dylib \
  ../src/test/run-pass/issue-2216.rs
Segmentation fault: 11
