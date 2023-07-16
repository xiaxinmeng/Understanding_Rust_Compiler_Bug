
[01:04:01] ---- [pretty] run-pass/borrowck/two-phase-bin-ops.rs stdout ----
[01:04:01] 	
[01:04:01] error in revision `lxl`: pretty-printing failed in round 1 revision Some("lxl")
[01:04:01] status: exit code: 101
[01:04:01] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "-" "-Z" "unpretty=normal" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass/borrowck/two-phase-bin-ops.stage2-x86_64-unknown-linux-gnu.pretty.aux" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
[01:04:01] stdout:
[01:04:01] ------------------------------------------
[01:04:01] 
[01:04:01] ------------------------------------------
[01:04:01] stderr:
[01:04:01] ------------------------------------------
[01:04:01] error: found invalid character; only `#` is allowed in raw string delimitation: (
[01:04:01]   --> <anon>:25:16
[01:04:01]    |
[01:04:01] 25 | trivial_binop! r#(AddAssign , add_assign);
