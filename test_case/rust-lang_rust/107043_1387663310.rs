plain
---- [ui] checkout/tests/ui/numbers-arithmetic/next-power-of-two-overflow-debug.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/numbers-arithmetic/next-power-of-two-overflow-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/next-power-of-two-overflow-debug/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/next-power-of-two-overflow-debug/auxiliary" "-C" "debug_assertions=true"
stdout: none
--- stderr -------------------------------
error: incorrect value `true` for codegen option `debug_assertions` - one of: `y`, `yes`, `on`, `n`, `no`, or `off` was expected


---- [ui] checkout/tests/ui/rfc-2091-track-caller/call-chain.rs#default stdout ----


error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/rfc-2091-track-caller/call-chain.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/call-chain.default/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2091-track-caller/call-chain.default/auxiliary" "-Zinline-mir=false"
stdout: none
--- stderr -------------------------------
error: incorrect value `false` for unstable option `inline-mir` - one of: `y`, `yes`, `on`, `n`, `no`, or `off` was expected



failures:
