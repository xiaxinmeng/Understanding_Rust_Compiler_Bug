plain
....................iii................................................................. 14168/14211
...........................................
failures:

---- [ui] checkout/tests/ui/traits/new-solver/pointer-sized.rs stdout ----
error: ui test compiled successfully!
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/traits/new-solver/pointer-sized.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/pointer-sized" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/new-solver/pointer-sized/auxiliary"
stdout: none
stderr: none


failures:
    [ui] checkout/tests/ui/traits/new-solver/pointer-sized.rs
