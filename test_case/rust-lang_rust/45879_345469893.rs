
[00:57:59] ---- [mir-opt] mir-opt/validate_1.rs stdout ----
[00:57:59] 	
[00:57:59] error: compilation failed!
[00:57:59] status: exit code: 101
[00:57:59] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/mir-opt/validate_1.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt" "--target=x86_64-unknown-linux-gnu" "-Zdump-mir=all" "-Zmir-opt-level=3" "-Zdump-mir-exclude-pass-number" "-Zdump-mir-dir=/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/validate_1" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/validate_1.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "verbose" "-Z" "mir-emit-validate=1" "-Z" "span_free_formats" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/mir-opt/validate_1.stage2-x86_64-unknown-linux-gnu.aux"
[00:57:59] stdout:
[00:57:59] ------------------------------------------
[00:57:59] 
[00:57:59] ------------------------------------------
[00:57:59] stderr:
[00:57:59] ------------------------------------------
[00:57:59] error: internal compiler error: /checkout/src/librustc_mir/transform/inline.rs:715: Arg operand `1` is (_10.0: &ReErased mut i32), not local
