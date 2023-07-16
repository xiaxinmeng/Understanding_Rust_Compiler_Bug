
% ./build/x86_64-unknown-linux-gnu/stage1/bin/rustc -Ccodegen-units=1 -C target-feature=-sse2 issue-65844.rs  --emit=llvm-bc
warning: unused variable: `a`
 --> issue-65844.rs:2:10
  |
2 |     let (a, b) = get_pair();
  |          ^ help: consider prefixing with an underscore: `_a`
  |
  = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `b`
 --> issue-65844.rs:2:13
  |
2 |     let (a, b) = get_pair();
  |             ^ help: consider prefixing with an underscore: `_b`

% ./build/x86_64-unknown-linux-gnu/llvm/bin/llc issue-65844.bc
error: <unknown>:0:0: in function _ZN11issue_658448get_pair17h78e1eb5d988d8ff6E { double, double } (): SSE2 register return with SSE2 disabled

error: <unknown>:0:0: in function _ZN11issue_658448get_pair17h78e1eb5d988d8ff6E { double, double } (): SSE2 register return with SSE2 disabled
