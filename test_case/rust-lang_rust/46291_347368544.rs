
[01:39:21] failures:
[01:39:21] 
[01:39:21] ---- [mir-opt] mir-opt/packed-struct-drop-aligned.rs stdout ----
[01:39:21] 	thread '[mir-opt] mir-opt/packed-struct-drop-aligned.rs' panicked at 'Did not find expected line, error: Mismatch in lines
[01:39:21] Currnt block:     bb0: {
[01:39:21] Expected Line: "        drop(_6) -> bb2;                 // scope 1 at /checkout/src/test/mir-opt/packed-struct-drop-aligned.rs:13:5: 13:8"
[01:39:21] Actual Line: "        drop(_6) -> [return: bb4, unwind: bb3];"
