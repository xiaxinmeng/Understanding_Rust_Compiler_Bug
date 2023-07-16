plain
test [ui] ui/save-analysis/issue-65590.rs ... ok
test [ui] ui/save-analysis/issue-73020.rs ... ok
test [ui] ui/self/arbitrary-self-types-not-object-safe.rs#curr ... ok
test [ui] ui/self/arbitrary-self-types-not-object-safe.rs#object_safe_for_dispatch ... ok
test [ui] ui/rt-explody-panic-payloads.rs ... FAILED
test [ui] ui/self/arbitrary_self_types_pin_lifetime-async.rs ... ok
test [ui] ui/self/arbitrary_self_types_pin_lifetime_impl_trait.rs ... ok
test [ui] ui/self/arbitrary_self_types_pin_lifetime.rs ... ok
test [ui] ui/self/arbitrary_self_types_pin_lifetime_mismatch-async.rs ... ok
---
test [ui] ui/wrong-hashset-issue-42918.rs ... ok

failures:

---- [ui] ui/rt-explody-panic-payloads.rs stdout ----
error: test run failed!
status: exit status: 101
status: exit status: 101
command: "/node-v15.14.0-linux-x64/bin/node" "/checkout/src/etc/wasm32-shim.js" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rt-explody-panic-payloads/a.wasm"
------------------------------------------

------------------------------------------
stderr:
stderr:
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=wasm32-unknown-unknown
------------------------------------------
RuntimeError: unreachable
    at __rust_start_panic (<anonymous>:wasm-function[104]:0x5627)
    at rust_panic (<anonymous>:wasm-function[98]:0x53fc)
    at _ZN3std9panicking20rust_panic_with_hook17h5ccb5bdcd34631a8E (<anonymous>:wasm-function[91]:0x50c1)
    at _ZN3std9panicking11begin_panic28_$u7b$$u7b$closure$u7d$$u7d$17hd96be7bc5324d8daE (<anonymous>:wasm-function[2]:0x23d)
    at _ZN3std10sys_common9backtrace26__rust_end_short_backtrace17hd88c1e7ea83709a4E (<anonymous>:wasm-function[1]:0x20a)
    at _ZN3std9panicking11begin_panic17hfff309e421a58feeE (<anonymous>:wasm-function[6]:0x2ce)
    at _ZN3std5panic9panic_any17h20f6d662a393fa05E (<anonymous>:wasm-function[5]:0x2bc)
    at _ZN25rt_explody_panic_payloads4main17h9754a1d5adad1c6eE (<anonymous>:wasm-function[21]:0x9cf)
    at _ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hf472dc553149c653E (<anonymous>:wasm-function[3]:0x25c)
    at _ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h4e7f1e0d07d35e3dE (<anonymous>:wasm-function[4]:0x290)
    at _ZN3std2rt19lang_start_internal17he10975b331fcd316E (<anonymous>:wasm-function[99]:0x54bb)
    at main (<anonymous>:wasm-function[22]:0xa4a)
    at Object.<anonymous> (/checkout/src/etc/wasm32-shim.js:20:20)
    at Module._compile (node:internal/modules/cjs/loader:1092:14)
    at Object.Module._extensions..js (node:internal/modules/cjs/loader:1121:10)
    at Module.load (node:internal/modules/cjs/loader:972:32)
    at Function.Module._load (node:internal/modules/cjs/loader:813:14)
    at Function.executeUserEntryPoint [as runMain] (node:internal/modules/run_main:76:12)
    at node:internal/main/run_main_module:17:47
------------------------------------------




failures:
    [ui] ui/rt-explody-panic-payloads.rs
test result: FAILED. 11377 passed; 1 failed; 611 ignored; 0 measured; 0 filtered out; finished in 98.63s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/wasm32-unknown-unknown/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-wasm32-unknown-unknown" "--suite" "ui" "--mode" "ui" "--target" "wasm32-unknown-unknown" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v15.14.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/wasm32-unknown-unknown/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.55.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --host= --target wasm32-unknown-unknown src/test/run-make src/test/ui src/test/mir-opt src/test/codegen-units library/core
Build completed unsuccessfully in 0:18:41
