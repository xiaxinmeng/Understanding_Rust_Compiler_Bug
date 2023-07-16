
error: `Bar` cannot be used in patterns
  --> src/main.rs:15:9
   |
15 |         LEAK_FREE => (),
   |         ^^^^^^^^^

warning: unreachable pattern
  --> src/main.rs:16:9
   |
15 |         LEAK_FREE => (),
   |         --------- matches any value
16 |         _ => (),
   |         ^ unreachable pattern
   |
   = note: `#[warn(unreachable_patterns)]` on by default

warning: `playground` (bin "playground") generated 1 warning
error: could not compile `playground` due to previous error; 1 warning emitted
