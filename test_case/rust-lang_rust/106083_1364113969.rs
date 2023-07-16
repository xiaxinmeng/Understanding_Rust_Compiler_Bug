plain
failures:

---- [ui] src/test/ui/intrinsics/panic-uninitialized-zeroed.rs#default stdout ----

error in revision `default`: test run failed!
status: exit status: 101
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/panic-uninitialized-zeroed.default/a.js"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 6, kind: WouldBlock, message: "Resource temporarily unavailable" }', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:111:9
------------------------------------------


---- [ui] src/test/ui/intrinsics/panic-uninitialized-zeroed.rs#strict stdout ----
---- [ui] src/test/ui/intrinsics/panic-uninitialized-zeroed.rs#strict stdout ----

error in revision `strict`: test run failed!
status: exit status: 101
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/panic-uninitialized-zeroed.strict/a.js"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 6, kind: WouldBlock, message: "Resource temporarily unavailable" }', /checkout/src/test/ui/intrinsics/panic-uninitialized-zeroed.rs:111:9
------------------------------------------



