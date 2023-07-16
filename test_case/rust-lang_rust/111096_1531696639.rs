plain
........................................................................................  9152/14898
........................................................................................  9240/14898
........................................................................................  9328/14898
........................................................................................  9416/14898
...............................i..i..................................i............F..F..  9504/14898
........................................................................................  9680/14898
.............................................................................i..........  9768/14898
...............................i........................................................  9856/14898
...............i........................................................................  9944/14898
---
..........................

failures:

---- [ui] tests/ui/numbers-arithmetic/overflow-attribute-works-1.rs stdout ----
error: ui test compiled successfully!
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/numbers-arithmetic/overflow-attribute-works-1.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflow-attribute-works-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflow-attribute-works-1/auxiliary" "-C" "overflow_checks=true"
stdout: none
stderr: none


---- [ui] tests/ui/numbers-arithmetic/overflow-attribute-works-2.rs stdout ----
error: ui test compiled successfully!
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/numbers-arithmetic/overflow-attribute-works-2.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflow-attribute-works-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/numbers-arithmetic/overflow-attribute-works-2/auxiliary" "-C" "overflow_checks=false"
stdout: none
stderr: none


failures:
    [ui] tests/ui/numbers-arithmetic/overflow-attribute-works-1.rs
