plain
test [ui] ui/abi/numbers-arithmetic/i128-ffi.rs ... ignored
test [ui] ui/abi/segfault-no-out-of-stack.rs ... ignored
test [ui] ui/abi/stack-probes-lto.rs ... ignored
test [ui] ui/abi/stack-probes.rs ... ignored
test [ui] ui/abi/stack-protector.rs#no-ssp ... ignored
test [ui] ui/abi/stack-protector.rs#ssp ... ignored
test [ui] ui/abi/struct-enums/struct-return.rs ... ignored
test [ui] ui/abi/union/union-c-interop.rs ... ignored
test [ui] ui/abi/variadic-ffi.rs ... ignored
test [ui] ui/abi/unsupported.rs#i686 ... ok
---
test [assembly] assembly/asm/powerpc-types.rs#powerpc64 ... ok
test [assembly] assembly/asm/mips-types.rs#mips64 ... ok
test [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#basic ... FAILED
test [assembly] assembly/asm/mips-types.rs#mips32 ... ok
test [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#strong ... FAILED
test [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#all ... FAILED
test [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#none ... FAILED
test [assembly] assembly/stack-protector/stack-protector-target-support.rs#r1 ... ok
test [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#missing ... FAILED
test [assembly] assembly/stack-protector/stack-protector-target-support.rs#r10 ... ok
test [assembly] assembly/stack-protector/stack-protector-target-support.rs#r11 ... ok
test [assembly] assembly/stack-protector/stack-protector-target-support.rs#r12 ... ok
test [assembly] assembly/stack-protector/stack-protector-target-support.rs#r13 ... ok
test [assembly] assembly/stack-protector/stack-protector-target-support.rs#r16 ... ok
test [assembly] assembly/stack-protector/stack-protector-target-support.rs#r14 ... ok
test [assembly] assembly/stack-protector/stack-protector-target-support.rs#r15 ... ok
---
test [assembly] assembly/target-feature-multiple.rs#TWOFLAGS ... ok
test [assembly] assembly/static-relocation-model.rs#x64 ... ok
test [assembly] assembly/target-feature-multiple.rs#SINGLEFLAG ... ok
test [assembly] assembly/asm/x86-types.rs#i686 ... ok
Some tests failed in compiletest suite=assembly mode=assembly host=x86_64-unknown-linux-gnu target=nvptx64-nvidia-cuda

failures:


---- [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#basic stdout ----

error in revision `basic`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "--cfg" "basic" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.basic/stack-protector-heuristics-effect.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-Z" "stack-protector=basic" "-C" "opt-level=2" "-Z" "merge-functions=disabled" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.basic/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `-Z stack-protector=basic` is not supported for target nvptx64-nvidia-cuda and will be ignored
error[E0463]: can't find crate for `std`
  |
  |
  = note: the `nvptx64-nvidia-cuda` target may not support the standard library
  = note: `std` is required by `<unknown>` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0463`.


------------------------------------------


---- [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#strong stdout ----

error in revision `strong`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "--cfg" "strong" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.strong/stack-protector-heuristics-effect.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-Z" "stack-protector=strong" "-C" "opt-level=2" "-Z" "merge-functions=disabled" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.strong/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `-Z stack-protector=strong` is not supported for target nvptx64-nvidia-cuda and will be ignored
error[E0463]: can't find crate for `std`
  |
  |
  = note: the `nvptx64-nvidia-cuda` target may not support the standard library
  = note: `std` is required by `<unknown>` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0463`.


------------------------------------------


---- [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#all stdout ----

error in revision `all`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "--cfg" "all" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.all/stack-protector-heuristics-effect.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-Z" "stack-protector=all" "-C" "opt-level=2" "-Z" "merge-functions=disabled" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.all/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `-Z stack-protector=all` is not supported for target nvptx64-nvidia-cuda and will be ignored
error[E0463]: can't find crate for `std`
  |
  |
  = note: the `nvptx64-nvidia-cuda` target may not support the standard library
  = note: `std` is required by `<unknown>` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: aborting due to previous error; 1 warning emitted

For more information about this error, try `rustc --explain E0463`.


------------------------------------------


---- [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#none stdout ----

error in revision `none`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "--cfg" "none" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.none/stack-protector-heuristics-effect.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-Z" "stack-protector=none" "-C" "opt-level=2" "-Z" "merge-functions=disabled" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.none/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
  |
  = note: the `nvptx64-nvidia-cuda` target may not support the standard library
  = note: `std` is required by `<unknown>` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.


------------------------------------------


---- [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#missing stdout ----

error in revision `missing`: compilation failed!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/assembly/stack-protector/stack-protector-heuristics-effect.rs" "-Zthreads=1" "--target=nvptx64-nvidia-cuda" "--cfg" "missing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.missing/stack-protector-heuristics-effect.s" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "-C" "opt-level=2" "-Z" "merge-functions=disabled" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly/stack-protector/stack-protector-heuristics-effect.missing/auxiliary" "--emit=asm"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
  |
  = note: the `nvptx64-nvidia-cuda` target may not support the standard library
  = note: `std` is required by `<unknown>` because it does not declare `#![no_std]`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.


------------------------------------------



failures:
    [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#all
    [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#basic
    [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#missing
    [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#none
    [assembly] assembly/stack-protector/stack-protector-heuristics-effect.rs#strong
test result: FAILED. 108 passed; 5 failed; 15 ignored; 0 measured; 0 filtered out; finished in 0.48s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/nvptx64-nvidia-cuda/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/assembly" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/assembly" "--stage-id" "stage2-nvptx64-nvidia-cuda" "--suite" "assembly" "--mode" "assembly" "--target" "nvptx64-nvidia-cuda" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--nodejs" "/node-v15.14.0-linux-x64/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/nvptx64-nvidia-cuda/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.57.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:00:33
