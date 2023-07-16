plain
failures:

---- [rustdoc] src/test/rustdoc/recursive-deref.rs stdout ----

error: rustdoc failed!
status: signal: 6 (core dumped)
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursive-deref/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/recursive-deref" "--deny" "warnings" "/checkout/src/test/rustdoc/recursive-deref.rs"
stdout: none
--- stderr -------------------------------
thread 'rustc' has overflowed its stack
fatal runtime error: stack overflow



failures:
