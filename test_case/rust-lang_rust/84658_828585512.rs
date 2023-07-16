plain
Successfully built f3a375416b92
Successfully tagged rust-ci:latest
Built container sha256:f3a375416b92ebaf129c8b87437506c5f403bfccb5c5456a6b13b4de0d89bc91
Uploading finished image to https://ci-caches.rust-lang.org/docker/b3d0ae34838021305b6fcbdeafa478fd95ab56ec1e0ac46bba89978ceea5671f3703b98515be144dc1842984f7ff09550c50d4f8ee787f51a796bbc2315ff174
upload failed: - to s3://rust-lang-ci-sccache2/docker/b3d0ae34838021305b6fcbdeafa478fd95ab56ec1e0ac46bba89978ceea5671f3703b98515be144dc1842984f7ff09550c50d4f8ee787f51a796bbc2315ff174 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-10]
---
.................................................................................................... 9400/11804
.................................................................................................... 9500/11804
.................................................................................................... 9600/11804
............................i......i................................................................ 9700/11804
..........................................................................iiiiiii..iiiiii.i......... 9800/11804
.................................................................................................... 10000/11804
.................................................................................................... 10100/11804
.................................................................................................... 10200/11804
.................................................................................................... 10300/11804
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 35 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....

 finished in 0.155 seconds
Suite("src/test/incremental") not skipped for "bootstrap::test::Incremental" -- not in ["src/tools/tidy"]
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........ii.ii........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 2.39s

 finished in 2.460 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
---- [pretty] pretty/asm.rs stdout ----

error: pretty-printing failed in round 0 revision None
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/pretty/asm.rs" "-Z" "unpretty=expanded" "--target" "x86_64-unknown-linux-gnu" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty/asm/auxiliary.pretty" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers"
------------------------------------------
#![feature(prelude_import)]
#![no_std]
#![feature(asm)]
#![feature(asm)]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;


// pretty-mode:expanded
// pp-exact:asm.pp
// only-x86_64
pub fn main() {
    let a: i32;
    let mut b = 4i32;
    unsafe {
    unsafe {
        asm!("");
        asm!("");
        asm!("", options(nomem, nostack));
        asm!("{0}", in(reg) 4);
        asm!("{0}", out(reg) a);
        asm!("{0}", inout(reg) b);
        asm!("{0} {1}", out(reg) _, inlateout(reg) b => _);
        asm!("", out("al") _, lateout("rbx") _);
        asm!("inst1\ninst2");
        asm!("inst1 {0}, 42\ninst2 {1}, 24", in(reg) a, out(reg) b);
        asm!("inst2 {1}, 24\ninst1 {0}, 42", in(reg) a, out(reg) b);
        asm!("inst1 {0}, 42\ninst2 {1}, 24", in(reg) a, out(reg) b);
        asm!("inst1\ninst2");
        asm!("inst1\ninst2");
        asm!("inst1\n\tinst2");
        asm!("inst1\ninst2\ninst3\ninst4");
}

------------------------------------------
stderr:
stderr:
------------------------------------------
error: invalid register `rbx`: rbx is used internally by LLVM and cannot be used as an operand for inline asm
   |
   |
18 |         asm!("", out("al") _, lateout("rbx") _);

error: aborting due to previous error


---
test result: FAILED. 68 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.60s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "pretty" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:50
