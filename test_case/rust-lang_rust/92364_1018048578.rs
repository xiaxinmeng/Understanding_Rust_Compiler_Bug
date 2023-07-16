plain

running 229 tests
..........i.ii....ii................................................................................ 100/229
.................i...................iiiiiii......i...................iii........................... 200/229
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
...........ii.F..............

---- [run-make] run-make-fulldeps/type-mismatch-same-crate-name stdout ----

error: make failed
error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
# compile two different versions of crateA
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name  --crate-type=rlib crateA.rs -C metadata=-1 -C extra-filename=-1
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name  --crate-type=rlib crateA.rs -C metadata=-2 -C extra-filename=-2
# make crateB depend on version 1 of crateA
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name  --crate-type=rlib crateB.rs --extern crateA=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name/libcrateA-1.rlib
# make crateC depend on version 2 of crateA
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name  crateC.rs --extern crateA=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/type-mismatch-same-crate-name/type-mismatch-same-crate-name/libcrateA-2.rlib 2>&1 | \
 tr -d '\r\n' | "/checkout/src/etc/cat-and-grep.sh" -e \
"mismatched types.*\
crateB::try_foo\(foo2\);.*\
expected struct \`crateA::foo::Foo\`, found struct \`Foo\`.*\
different versions of crate \`crateA\`.*\
mismatched types.*\
crateB::try_bar\(bar2\);.*\
expected trait \`crateA::bar::Bar\`, found trait \`Bar\`.*\
different versions of crate \`crateA\`"
[[[ begin stdout ]]]
error[E0308]: arguments to this function are incorrect  --> crateC.rs:22:9   |22 |         crateB::try_foo(foo2);   |         ^^^^^^^^^^^^^^^ ---- expected struct `crateA::foo::Foo`, found struct `Foo`   |   = note: perhaps two different versions of crate `crateA` are being used?error[E0308]: arguments to this function are incorrect  --> crateC.rs:23:9   |23 |         crateB::try_bar(bar2);   |         ^^^^^^^^^^^^^^^ ---- expected trait `crateA::bar::Bar`, found trait `Bar`   |   = note: expected struct `Box<(dyn crateA::bar::Bar + 'static)>`              found struct `Box<dyn Bar>`   = note: perhaps two different versions of crate `crateA` are being used?error: aborting due to 2 previous errorsFor more information about this error, try `rustc --explain E0308`.
[[[ end stdout ]]]
Error: cannot match: mismatched types.*crateB::try_foo\(foo2\);.*expected struct `crateA::foo::Foo`, found struct `Foo`.*different versions of crate `crateA`.*mismatched types.*crateB::try_bar\(bar2\);.*expected trait `crateA::bar::Bar`, found trait `Bar`.*different versions of crate `crateA`
------------------------------------------
stderr:
------------------------------------------
warning: trait objects without an explicit `dyn` are deprecated
warning: trait objects without an explicit `dyn` are deprecated
 --> crateA.rs:8:25
8 |     pub fn bar() -> Box<Bar> {
  |                         ^^^
  |
  = note: `#[warn(bare_trait_objects)]` on by default
---

warning: 1 warning emitted

warning: trait objects without an explicit `dyn` are deprecated
 --> crateA.rs:8:25
8 |     pub fn bar() -> Box<Bar> {
  |                         ^^^
  |
  = note: `#[warn(bare_trait_objects)]` on by default
---

warning: trait objects without an explicit `dyn` are deprecated
 --> crateB.rs:4:23
  |
4 | pub fn try_bar(x: Box<crateA::Bar>){}
  |
  = note: `#[warn(bare_trait_objects)]` on by default
  = warning: this is accepted in the current edition (Rust 2015) but is a hard error in Rust 2021!
  = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
  = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/warnings-promoted-to-error.html>
help: use `dyn`
  |
4 - pub fn try_bar(x: Box<crateA::Bar>){}
4 + pub fn try_bar(x: Box<dyn crateA::Bar>){}

warning: unused variable: `x`
 --> crateB.rs:3:16
  |
  |
3 | pub fn try_foo(x: crateA::Foo){}
  |                ^ help: if this is intentional, prefix it with an underscore: `_x`
  = note: `#[warn(unused_variables)]` on by default

warning: unused variable: `x`
 --> crateB.rs:4:16
 --> crateB.rs:4:16
  |
4 | pub fn try_bar(x: Box<crateA::Bar>){}
  |                ^ help: if this is intentional, prefix it with an underscore: `_x`
warning: 3 warnings emitted


make: *** [Makefile:10: all] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/type-mismatch-same-crate-name

test result: FAILED. 209 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out; finished in 35.74s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-12/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:35:17
