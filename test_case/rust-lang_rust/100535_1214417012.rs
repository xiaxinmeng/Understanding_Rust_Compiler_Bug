plain
test [run-make] src/test/run-make/thumb-none-cortex-m ... ignored
test [run-make] src/test/run-make/thumb-none-qemu ... ignored
test [run-make] src/test/run-make/issue-85019-moved-src-dir ... ok
test [run-make] src/test/run-make/rustc-macro-dep-files ... ok
test [run-make] src/test/run-make/rlib-format-packed-bundled-libs-2 ... ok
test [run-make] src/test/run-make/unstable-flag-required ... ok
test [run-make] src/test/run-make/remap-path-prefix-dwarf ... ok
test [run-make] src/test/run-make/track-path-dep-info ... ok
test [run-make] src/test/run-make/wasm-custom-sections-opt ... ok
---
test [run-make] src/test/run-make/wasm-symbols-different-module ... ok

failures:

---- [run-make] src/test/run-make/rlib-format-packed-bundled-libs stdout ----
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
--- stdout -------------------------------
clang -ffunction-sections -fdata-sections -fPIC --target=wasm32-unknown-unknown -v -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o native_dep_1.c
--- stderr -------------------------------
/bin/dash: 1: clang: not found
/bin/dash: 1: clang: not found
make: *** [../../run-make-fulldeps/tools.mk:176: /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make/rlib-format-packed-bundled-libs/rlib-format-packed-bundled-libs/libnative_dep_1.o] Error 127



failures:
failures:
    [run-make] src/test/run-make/rlib-format-packed-bundled-libs
test result: FAILED. 43 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out; finished in 5.22s

Build completed unsuccessfully in 0:12:28
