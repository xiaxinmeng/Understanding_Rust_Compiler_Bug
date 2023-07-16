plain
.................................................................................................... 9400/11770
.................................................................................................... 9500/11770
...................................................................................................i 9600/11770
......i............................................................................................. 9700/11770
.............................................iiiiiii..iiiiii.i...................................... 9800/11770
.................................................................................................... 10000/11770
.................................................................................................... 10100/11770
.................................................................................................... 10200/11770
.................................................................................................... 10300/11770
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.195 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii..........i....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.61s

 finished in 2.702 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free  --target=i686-unknown-linux-gnu atomic_lock_free.rs
nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free/libatomic_lock_free.rlib" | "/checkout/src/etc/cat-and-grep.sh" -v __atomic_fetch_add
[[[ begin stdout ]]]

atomic_lock_free.atomic_lock_free.3a1fbbbh-cgu.0.rcgu.o:
00000000 T _ZN16atomic_lock_free10atomic_i1617h2737b9e1ef3d4a42E
00000000 T _ZN16atomic_lock_free10atomic_i3217h566726fa298ec94dE
00000000 T _ZN16atomic_lock_free10atomic_i6417h01d0495986c7c927E
00000000 T _ZN16atomic_lock_free10atomic_u1617h8535c341e6d19bafE
00000000 T _ZN16atomic_lock_free10atomic_u3217h7ef8b455042c3069E
00000000 T _ZN16atomic_lock_free10atomic_u6417h989894f8987a6ff0E
00000000 T _ZN16atomic_lock_free12atomic_isize17hcb7b7ece920ca70aE
00000000 T _ZN16atomic_lock_free12atomic_usize17hd376c0e7025ccab1E
00000000 T _ZN16atomic_lock_free9atomic_i817hd24e7880f305b4f7E
00000000 T _ZN16atomic_lock_free9atomic_u817h776c4379edcabbbcE
lib.rmeta:


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free  --target=x86_64-unknown-linux-gnu atomic_lock_free.rs
nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free/libatomic_lock_free.rlib" | "/checkout/src/etc/cat-and-grep.sh" -v __atomic_fetch_add
[[[ begin stdout ]]]

atomic_lock_free.atomic_lock_free.3a1fbbbh-cgu.0.rcgu.o:
0000000000000000 T _ZN16atomic_lock_free10atomic_i1617h2737b9e1ef3d4a42E
0000000000000000 T _ZN16atomic_lock_free10atomic_i3217h566726fa298ec94dE
0000000000000000 T _ZN16atomic_lock_free10atomic_i6417h01d0495986c7c927E
0000000000000000 T _ZN16atomic_lock_free10atomic_u1617h8535c341e6d19bafE
0000000000000000 T _ZN16atomic_lock_free10atomic_u3217h7ef8b455042c3069E
0000000000000000 T _ZN16atomic_lock_free10atomic_u6417h989894f8987a6ff0E
0000000000000000 T _ZN16atomic_lock_free12atomic_isize17hcb7b7ece920ca70aE
0000000000000000 T _ZN16atomic_lock_free12atomic_usize17hd376c0e7025ccab1E
0000000000000000 T _ZN16atomic_lock_free9atomic_i817hd24e7880f305b4f7E
0000000000000000 T _ZN16atomic_lock_free9atomic_u817h776c4379edcabbbcE
lib.rmeta:


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free  --target=arm-unknown-linux-gnueabi atomic_lock_free.rs
nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free/libatomic_lock_free.rlib" | "/checkout/src/etc/cat-and-grep.sh" -v __atomic_fetch_add
[[[ begin stdout ]]]

atomic_lock_free.atomic_lock_free.3a1fbbbh-cgu.0.rcgu.o:
00000000 t $a.0
00000000 t $a.1
00000000 t $a.2
00000000 t $a.3
00000000 t $a.4
00000000 t $a.5
00000000 t $a.6
00000000 t $a.7
00000000 t $a.8
00000000 t $a.9
00000000 T _ZN16atomic_lock_free10atomic_i1617h2737b9e1ef3d4a42E
00000000 T _ZN16atomic_lock_free10atomic_i3217h566726fa298ec94dE
00000000 T _ZN16atomic_lock_free10atomic_i6417h01d0495986c7c927E
00000000 T _ZN16atomic_lock_free10atomic_u1617h8535c341e6d19bafE
00000000 T _ZN16atomic_lock_free10atomic_u3217h7ef8b455042c3069E
00000000 T _ZN16atomic_lock_free10atomic_u6417h989894f8987a6ff0E
00000000 T _ZN16atomic_lock_free12atomic_isize17hcb7b7ece920ca70aE
00000000 T _ZN16atomic_lock_free12atomic_usize17hd376c0e7025ccab1E
00000000 T _ZN16atomic_lock_free9atomic_i817hd24e7880f305b4f7E
00000000 T _ZN16atomic_lock_free9atomic_u817h776c4379edcabbbcE
         U __aeabi_unwind_cpp_pr0
         U __aeabi_unwind_cpp_pr1
lib.rmeta:


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free  --target=arm-unknown-linux-gnueabihf atomic_lock_free.rs
nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free/libatomic_lock_free.rlib" | "/checkout/src/etc/cat-and-grep.sh" -v __atomic_fetch_add
[[[ begin stdout ]]]

atomic_lock_free.atomic_lock_free.3a1fbbbh-cgu.0.rcgu.o:
00000000 t $a.0
00000000 t $a.1
00000000 t $a.2
00000000 t $a.3
00000000 t $a.4
00000000 t $a.5
00000000 t $a.6
00000000 t $a.7
00000000 t $a.8
00000000 t $a.9
00000000 T _ZN16atomic_lock_free10atomic_i1617h2737b9e1ef3d4a42E
00000000 T _ZN16atomic_lock_free10atomic_i3217h566726fa298ec94dE
00000000 T _ZN16atomic_lock_free10atomic_i6417h01d0495986c7c927E
00000000 T _ZN16atomic_lock_free10atomic_u1617h8535c341e6d19bafE
00000000 T _ZN16atomic_lock_free10atomic_u3217h7ef8b455042c3069E
00000000 T _ZN16atomic_lock_free10atomic_u6417h989894f8987a6ff0E
00000000 T _ZN16atomic_lock_free12atomic_isize17hcb7b7ece920ca70aE
00000000 T _ZN16atomic_lock_free12atomic_usize17hd376c0e7025ccab1E
00000000 T _ZN16atomic_lock_free9atomic_i817hd24e7880f305b4f7E
00000000 T _ZN16atomic_lock_free9atomic_u817h776c4379edcabbbcE
         U __aeabi_unwind_cpp_pr0
         U __aeabi_unwind_cpp_pr1
lib.rmeta:


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free  --target=armv7-unknown-linux-gnueabihf atomic_lock_free.rs
nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free/libatomic_lock_free.rlib" | "/checkout/src/etc/cat-and-grep.sh" -v __atomic_fetch_add
[[[ begin stdout ]]]

atomic_lock_free.atomic_lock_free.3a1fbbbh-cgu.0.rcgu.o:
00000000 t $a.0
00000000 t $a.1
00000000 t $a.2
00000000 t $a.3
00000000 t $a.4
00000000 t $a.5
00000000 t $a.6
00000000 t $a.7
00000000 t $a.8
00000000 t $a.9
00000000 T _ZN16atomic_lock_free10atomic_i1617h2737b9e1ef3d4a42E
00000000 T _ZN16atomic_lock_free10atomic_i3217h566726fa298ec94dE
00000000 T _ZN16atomic_lock_free10atomic_i6417h01d0495986c7c927E
00000000 T _ZN16atomic_lock_free10atomic_u1617h8535c341e6d19bafE
00000000 T _ZN16atomic_lock_free10atomic_u3217h7ef8b455042c3069E
00000000 T _ZN16atomic_lock_free10atomic_u6417h989894f8987a6ff0E
00000000 T _ZN16atomic_lock_free12atomic_isize17hcb7b7ece920ca70aE
00000000 T _ZN16atomic_lock_free12atomic_usize17hd376c0e7025ccab1E
00000000 T _ZN16atomic_lock_free9atomic_i817hd24e7880f305b4f7E
00000000 T _ZN16atomic_lock_free9atomic_u817h776c4379edcabbbcE
         U __aeabi_unwind_cpp_pr0
         U __aeabi_unwind_cpp_pr1
lib.rmeta:


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free  --target=thumbv7neon-unknown-linux-gnueabihf atomic_lock_free.rs
nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free/libatomic_lock_free.rlib" | "/checkout/src/etc/cat-and-grep.sh" -v __atomic_fetch_add
[[[ begin stdout ]]]

atomic_lock_free.atomic_lock_free.3a1fbbbh-cgu.0.rcgu.o:
00000000 t $t.0
00000000 t $t.1
00000000 t $t.2
00000000 t $t.3
00000000 t $t.4
00000000 t $t.5
00000000 t $t.6
00000000 t $t.7
00000000 t $t.8
00000000 t $t.9
00000001 T _ZN16atomic_lock_free10atomic_i1617h2737b9e1ef3d4a42E
00000001 T _ZN16atomic_lock_free10atomic_i3217h566726fa298ec94dE
00000001 T _ZN16atomic_lock_free10atomic_i6417h01d0495986c7c927E
00000001 T _ZN16atomic_lock_free10atomic_u1617h8535c341e6d19bafE
00000001 T _ZN16atomic_lock_free10atomic_u3217h7ef8b455042c3069E
00000001 T _ZN16atomic_lock_free10atomic_u6417h989894f8987a6ff0E
00000001 T _ZN16atomic_lock_free12atomic_isize17hcb7b7ece920ca70aE
00000001 T _ZN16atomic_lock_free12atomic_usize17hd376c0e7025ccab1E
00000001 T _ZN16atomic_lock_free9atomic_i817hd24e7880f305b4f7E
00000001 T _ZN16atomic_lock_free9atomic_u817h776c4379edcabbbcE
         U __aeabi_unwind_cpp_pr0
lib.rmeta:


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free  --target=aarch64-unknown-linux-gnu atomic_lock_free.rs
nm "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free/libatomic_lock_free.rlib" | "/checkout/src/etc/cat-and-grep.sh" -v __atomic_fetch_add
[[[ begin stdout ]]]

atomic_lock_free.atomic_lock_free.3a1fbbbh-cgu.0.rcgu.o:
0000000000000000 n $d.12
0000000000000000 r $d.13
0000000000000000 t $x.0
0000000000000000 t $x.1
0000000000000000 t $x.10
0000000000000000 t $x.11
0000000000000000 t $x.2
0000000000000000 t $x.3
0000000000000000 t $x.4
0000000000000000 t $x.5
0000000000000000 t $x.6
0000000000000000 t $x.7
0000000000000000 t $x.8
0000000000000000 t $x.9
0000000000000000 T _ZN16atomic_lock_free10atomic_i1617h2737b9e1ef3d4a42E
0000000000000000 T _ZN16atomic_lock_free10atomic_i3217h566726fa298ec94dE
0000000000000000 T _ZN16atomic_lock_free10atomic_i6417h01d0495986c7c927E
0000000000000000 T _ZN16atomic_lock_free10atomic_u1617h8535c341e6d19bafE
0000000000000000 T _ZN16atomic_lock_free10atomic_u3217h7ef8b455042c3069E
0000000000000000 T _ZN16atomic_lock_free10atomic_u6417h989894f8987a6ff0E
0000000000000000 T _ZN16atomic_lock_free11atomic_i12817h70056d81fc065880E
0000000000000000 T _ZN16atomic_lock_free11atomic_u12817he28a56875e88f7d1E
0000000000000000 T _ZN16atomic_lock_free12atomic_isize17hcb7b7ece920ca70aE
0000000000000000 T _ZN16atomic_lock_free12atomic_usize17hd376c0e7025ccab1E
0000000000000000 T _ZN16atomic_lock_free9atomic_i817hd24e7880f305b4f7E
0000000000000000 T _ZN16atomic_lock_free9atomic_u817h776c4379edcabbbcE
lib.rmeta:


[[[ end stdout ]]]
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/atomic-lock-free/atomic-lock-free  --target=mips-unknown-linux-gnu atomic_lock_free.rs
Makefile:9: recipe for target 'all' failed
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
nm: lib.rmeta: no symbols
nm: lib.rmeta: no symbols
nm: lib.rmeta: no symbols
nm: lib.rmeta: no symbols
nm: lib.rmeta: no symbols
nm: lib.rmeta: no symbols
nm: lib.rmeta: no symbols
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /cargo/registry/src/github.com-1ecc6299db9ec823/object-0.22.0/src/write/elf.rs:144:61

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (9038c15d1 2021-04-22) running on x86_64-unknown-linux-gnu

query stack during panic:
end of query stack
make: *** [all] Error 101
------------------------------------------




failures:
    [run-make] run-make-fulldeps/atomic-lock-free

test result: FAILED. 200 passed; 1 failed; 20 ignored; 0 measured; 0 filtered out; finished in 24.83s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-10/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:34:25
