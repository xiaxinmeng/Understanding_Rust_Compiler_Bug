plain
---- [ui] src/test/ui/mir/mir_let_chains_drop_order.rs stdout ----

error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_let_chains_drop_order/a.js"
stdout: none
--- stderr -------------------------------
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
exception thrown: RuntimeError: function signature mismatch,RuntimeError: function signature mismatch
    at dynCall_vi (<anonymous>:wasm-function[382]:0x135bf)
    at invoke_vi (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_let_chains_drop_order/a.js:4651:5)
    at <anonymous>:wasm-function[63]:0x1d1e
    at dynCall_v (<anonymous>:wasm-function[43]:0x96f)
    at <anonymous>:wasm-function[44]:0x97a
    at dynCall_ii (<anonymous>:wasm-function[386]:0x135f3)
    at Module.dynCall_ii (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_let_chains_drop_order/a.js:4609:76)
    at invoke_ii (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/mir_let_chains_drop_order/a.js:4739:12)
    at <anonymous>:wasm-function[157]:0x5d74
    at main (<anonymous>:wasm-function[65]:0x23b5)


---- [ui] src/test/ui/rfcs/rfc1857-drop-order.rs stdout ----


error: test run failed!
status: exit status: 1
command: "/emsdk-portable/node/latest/bin/node" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'this panic is caught :D', /checkout/src/test/ui/rfcs/rfc1857-drop-order.rs:61:10
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
exception thrown: RuntimeError: function signature mismatch,RuntimeError: function signature mismatch
    at dynCall_vi (<anonymous>:wasm-function[385]:0x16eea)
    at invoke_vi (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:4651:5)
    at <anonymous>:wasm-function[67]:0x1a0d
    at dynCall_v (<anonymous>:wasm-function[43]:0x998)
    at <anonymous>:wasm-function[44]:0x9a3
    at dynCall_ii (<anonymous>:wasm-function[389]:0x16f20)
    at Module.dynCall_ii (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:4609:76)
    at invoke_ii (/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfcs/rfc1857-drop-order/a.js:4728:12)
    at <anonymous>:wasm-function[159]:0x9682
    at main (<anonymous>:wasm-function[68]:0x5cd1)



failures:
