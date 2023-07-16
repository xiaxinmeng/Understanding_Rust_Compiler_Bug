
[01:15:43] stderr:
[01:15:43] ------------------------------------------
[01:15:43] error: unexpected token: `0.0`
[01:15:43]   --> <anon>:98:15
[01:15:43]    |
[01:15:43] 98 |     let f = x.0.0;
[01:15:43]    |             --^^^
[01:15:43]    |             | |
[01:15:43]    |             | unexpected token
[01:15:43]    |             help: try parenthesizing the first index `(x.0).0`
[01:15:43] 
[01:15:43] error: unexpected token: `0.0`
[01:15:43]   --> <anon>:99:15
[01:15:43]    |
[01:15:43] 99 |     if c0 { x.0.0 = f; }
[01:15:43]    |             --^^^
[01:15:43]    |             | |
[01:15:43]    |             | unexpected token
[01:15:43]    |             help: try parenthesizing the first index `(x.0).0`
[01:15:43] 
[01:15:43] error: aborting due to previous error(s)
[01:15:43] 
[01:15:43] 
[01:15:43] ------------------------------------------
[01:15:43] 
[01:15:43] thread '[pretty] pretty/dynamic-drop.rs' panicked at 'explicit panic', /checkout/src/tools/compiletest/src/runtest.rs:2479
[01:15:43] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:15:43] 
[01:15:43] 
[01:15:43] failures:
[01:15:43]     [pretty] pretty/dynamic-drop.rs
[01:15:43] 
[01:15:43] test result: FAILED. 2669 passed; 1 failed; 36 ignored; 0 measured; 0 filtered out
[01:15:43] 
