plain
test [ui] tests/ui/transmutability/primitives/numbers.rs#current ... ok

failures:

---- [ui] tests/ui/extern-flag/force-extern.rs stdout ----
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args extern-flag/force-extern.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/extern-flag/force-extern.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/force-extern" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/force-extern/auxiliary" "-Zunstable-options" "--crate-type" "dylib" "--edition=2018" "--extern" "force:panic_handler=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/force-extern/auxiliary/libpanic_handler.rlib"
stdout: none
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

warning: 1 warning emitted
------------------------------------------
------------------------------------------


---- [ui] tests/ui/extern-flag/redundant-force-extern.rs stdout ----
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

warning: 1 warning emitted

---
To only update this specific test, also pass `--test-args extern-flag/redundant-force-extern.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/extern-flag/redundant-force-extern.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=wasm32-unknown-unknown" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/redundant-force-extern" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/redundant-force-extern/auxiliary" "-Zunstable-options" "--crate-type" "dylib" "--edition=2018" "--extern" "force:panic_handler=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern-flag/redundant-force-extern/auxiliary/libpanic_handler.rlib"
stdout: none
warning: dropping unsupported crate type `dylib` for target `wasm32-unknown-unknown`

warning: 1 warning emitted
------------------------------------------
