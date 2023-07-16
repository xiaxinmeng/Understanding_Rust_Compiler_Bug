
failures:

---- [ui] ui/unsized-locals/by-value-trait-object-safety-rpass.rs stdout ----

error: test run failed!
status: signal: 6
command: "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-rpass/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------

thread 'main' has overflowed its stack
fatal runtime error: stack overflow

------------------------------------------


---- [ui] ui/unsized-locals/by-value-trait-object-safety-withdefault.rs stdout ----

error: test run failed!
status: signal: 6
command: "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/unsized-locals/by-value-trait-object-safety-withdefault/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------

thread 'main' has overflowed its stack
fatal runtime error: stack overflow

------------------------------------------


---- [ui] ui/unsized-locals/autoderef.rs stdout ----

error: test run failed!
status: signal: 6
command: "/checkout/obj/build/i686-unknown-linux-gnu/test/ui/unsized-locals/autoderef/a"
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------

thread 'main' has overflowed its stack
fatal runtime error: stack overflow

------------------------------------------



failures:
    [ui] ui/unsized-locals/autoderef.rs
    [ui] ui/unsized-locals/by-value-trait-object-safety-rpass.rs
    [ui] ui/unsized-locals/by-value-trait-object-safety-withdefault.rs

test result: FAILED. 10827 passed; 3 failed; 109 ignored; 0 measured; 0 filtered out
