
[01:38:26] ---- [mir-opt] mir-opt/match_false_edges.rs stdout ----
[01:38:26] 	thread '[mir-opt] mir-opt/match_false_edges.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:38:26] Currnt block:  bb9: { // binding1 and guard
[01:38:26] Expected Line: "        _6 = const guard() -> bb10;      // scope 2 at /checkout/src/test/mir-opt/match_false_edges.rs:25:20: 25:27"
[01:38:26] Actual Line: "     _6 = const guard() -> [return: bb10, unwind: bb1];"
...
[01:38:26] ---- [mir-opt] mir-opt/validate_2.rs stdout ----
[01:38:26] 	thread '[mir-opt] mir-opt/validate_2.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:38:26] Currnt block: None
[01:38:26] Expected Line: "        drop(_1) -> bb2;                 // scope 0 at /checkout/src/test/mir-opt/validate_2.rs:16:2: 16:2"
[01:38:26] Actual Line: "        drop(_1) -> [return: bb2, unwind: bb3];"
