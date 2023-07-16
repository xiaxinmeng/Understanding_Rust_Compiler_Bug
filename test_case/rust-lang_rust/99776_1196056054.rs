plain
test [ui] src/test/ui/suggestions/dont-suggest-ref/simple.rs ... ok
test [ui] src/test/ui/structs-enums/variant-structs-trivial.rs ... ok
test [ui] src/test/ui/structs-enums/type-sizes.rs ... ok
test [ui] src/test/ui/structs-enums/unit-like-struct.rs ... ok
test [ui] src/test/ui/suggestions/dont-suggest-doc-hidden-variant-for-enum/hidden-parent.rs ... ok
test [ui] src/test/ui/suggestions/fn-ctor-passed-as-arg-where-it-should-have-been-called.rs ... ok
test [ui] src/test/ui/suggestions/fn-needing-specified-return-type-param.rs ... ok
test [ui] src/test/ui/suggestions/fn-missing-lifetime-in-item.rs ... ok
test [ui] src/test/ui/suggestions/field-has-method.rs ... ok
test [ui] src/test/ui/suggestions/field-has-method.rs ... ok
test [ui] src/test/ui/suggestions/dont-suggest-doc-hidden-variant-for-enum/hidden-child.rs ... ok
test [ui] src/test/ui/structs/large-records.rs ... ok
test [ui] src/test/ui/suggestions/if-let-typo.rs ... ok
test [ui] src/test/ui/suggestions/field-access.rs ... ok
test [ui] src/test/ui/suggestions/imm-ref-trait-object.rs ... ok
---
---- [ui] src/test/ui/cfg/crt-static-off-works.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cfg/crt-static-off-works.rs" "-Zthreads=1" "--target=wasm32-unknown-emscripten" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/crt-static-off-works/a.js" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-C" "target-feature=-crt-static" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/crt-static-off-works/auxiliary"
stdout: none
--- stderr -------------------------------
WARN rustc_codegen_ssa::back::linker Building dynamic executable with -sMAIN_MODULE=2. Dead code elimination may break things. See https://emscripten.org/docs/compiling/Dynamic-Linking.html?highlight=main_module#code-size 
error: linking with `emcc` failed: exit status: 1
   |
   = note: "emcc" "-s" "EXPORTED_FUNCTIONS=[\"_main\"]" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/crt-static-off-works/a.crt_static_off_works.9a386833-cgu.0.rcgu.o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/crt-static-off-works/a.2t30dcus8l95j891.rcgu.o" "-L" "/checkout/obj/build/wasm32-unknown-emscripten/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/crt-static-off-works/auxiliary" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd-0ac7839583e0074b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libpanic_unwind-696b6eaa34188a6f.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_demangle-34a2893d86851892.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libstd_detect-1e3b162d3dc5a21c.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libhashbrown-da057e8abfe679c7.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libminiz_oxide-7a6670a49c7068de.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libadler-ba5ac73899ee68d1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_alloc-deae52698d2735a3.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libunwind-b1560c864fd3a4a1.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcfg_if-a0fbea172501d8a2.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liblibc-297353a3c10661a4.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/liballoc-b71b12ce2301795a.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/librustc_std_workspace_core-2a3aad12c7daf07b.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcore-658f099c4bc7f037.rlib" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/libcompiler_builtins-f302f376a525e7f3.rlib" "-l" "c" "-s" "DISABLE_EXCEPTION_CATCHING=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-emscripten/lib/self-contained" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/crt-static-off-works/a.js" "-sMAIN_MODULE=2" "-O2" "-g0" "-sABORTING_MALLOC=0" "-Wl,--fatal-warnings"
   = note: cache:INFO: generating system library: libc.a... (this will be cached in "/emsdk-portable/upstream/emscripten/cache/wasm-pic/libc.a" for subsequent builds)
           cache:INFO:  - ok
           cache:INFO: generating system library: libcompiler_rt.a... (this will be cached in "/emsdk-portable/upstream/emscripten/cache/wasm-pic/libcompiler_rt.a" for subsequent builds)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-emscripten
           cache:INFO:  - ok
           cache:INFO: generating system library: libc-wasm.a... (this will be cached in "/emsdk-portable/upstream/emscripten/cache/wasm-pic/libc-wasm.a" for subsequent builds)
           cache:INFO:  - ok
           cache:INFO: generating system library: libc++.a... (this will be cached in "/emsdk-portable/upstream/emscripten/cache/wasm-pic/libc++.a" for subsequent builds)
           cache:INFO:  - ok
           cache:INFO: generating system library: libc++abi.a... (this will be cached in "/emsdk-portable/upstream/emscripten/cache/wasm-pic/libc++abi.a" for subsequent builds)
           cache:INFO:  - ok
           cache:INFO: generating system library: libdlmalloc.a... (this will be cached in "/emsdk-portable/upstream/emscripten/cache/wasm-pic/libdlmalloc.a" for subsequent builds)
           cache:INFO:  - ok
           cache:INFO: generating system library: libpthread_stub.a... (this will be cached in "/emsdk-portable/upstream/emscripten/cache/wasm-pic/libpthread_stub.a" for subsequent builds)
           cache:INFO:  - ok
           cache:INFO: generating system library: libc_rt_wasm.a... (this will be cached in "/emsdk-portable/upstream/emscripten/cache/wasm-pic/libc_rt_wasm.a" for subsequent builds)
           cache:INFO:  - ok
           cache:INFO: generating system library: libsockets.a... (this will be cached in "/emsdk-portable/upstream/emscripten/cache/wasm-pic/libsockets.a" for subsequent builds)
           cache:INFO:  - ok
           cache:INFO: generating system asset: generated_struct_info.json... (this will be cached in "/emsdk-portable/upstream/emscripten/cache/wasm-pic/generated_struct_info.json" for subsequent builds)
           missing function: iprintf
           -1
           exception thrown: RuntimeError: abort(-1). Build with -s ASSERTIONS=1 for more info.,RuntimeError: abort(-1). Build with -s ASSERTIONS=1 for more info.
               at abort (/tmp/tmp6fh1tjp2.js:1561:11)
               at _iprintf (/tmp/tmp6fh1tjp2.js:1801:37)
               at <anonymous>:wasm-function[5]:0x16d
               at main (<anonymous>:wasm-function[6]:0xaf43)
               at Module._main (/tmp/tmp6fh1tjp2.js:1943:60)
               at callMain (/tmp/tmp6fh1tjp2.js:2101:15)
               at doRun (/tmp/tmp6fh1tjp2.js:2162:23)
               at run (/tmp/tmp6fh1tjp2.js:2177:5)
               at runCaller (/tmp/tmp6fh1tjp2.js:2086:19)
               at removeRunDependency (/tmp/tmp6fh1tjp2.js:1536:7)
           emcc: error: '/emsdk-portable/node/14.18.2_64bit/bin/node /tmp/tmp6fh1tjp2.js' failed (1)

error: aborting due to previous error
------------------------------------------

