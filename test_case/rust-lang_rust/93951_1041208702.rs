
warning: unreachable expression
  --> /Users/cormac/src/rust/src/test/ui/let-else/let-else-run-pass.rs:33:5
   |
LL |     let Some(1) = Some(2) else { return };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^------^^^
   |     |                            |
   |     |                            any code following this expression is unreachable
   |     unreachable expression
