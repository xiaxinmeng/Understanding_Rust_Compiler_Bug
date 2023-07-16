plain
.........ii......................................................................................... 7600/11249
.............................i...................................................................... 7700/11249
..............i..................................................................................... 7800/11249
.......i............................................................................................ 7900/11249
................F...........................F..............................................F........ 8000/11249
.................................................................................................... 8200/11249
...........................................................................................i........ 8300/11249
.................................................................................................... 8400/11249
.................................................................................................... 8500/11249
---
.................................................................................................... 9000/11249
.................................................................................................... 9100/11249
.................................................................................................... 9200/11249
..........................................i.......i................................................. 9300/11249
................................................................................iiiiii..iiiiii.i.... 9400/11249
.................................................................................................... 9600/11249
.................................................................................................... 9700/11249
.................................................................................................... 9800/11249
.................................................................................................... 9900/11249
---
failures:

---- [ui] ui/macros/issue-40469.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-40469.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-40469/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-40469/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: couldn't read /checkout/src/test/ui/macros/auxiliary/issue-40469.rs: No such file or directory (os error 2)
   |
   |
LL | include!("auxiliary/issue-40469.rs");
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---

---- [ui] ui/parser/issue-21146.rs stdout ----
diff of stderr:

- error: expected one of `!` or `::`, found `<eof>`
-   --> $DIR/auxiliary/issue-21146-inc.rs:3:1
+ error: couldn't read $DIR/auxiliary/issue-21146-inc.rs: No such file or directory (os error 2)
3    |
- LL | parse_error
- LL | parse_error
-    | ^^^^^^^^^^^ expected one of `!` or `::`
+ LL | include!("auxiliary/issue-21146-inc.rs");
+    |
+    = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
6 
7 error: aborting due to previous error
7 error: aborting due to previous error
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-21146/issue-21146.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-21146.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-21146.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-21146" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-21146/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: couldn't read /checkout/src/test/ui/parser/auxiliary/issue-21146-inc.rs: No such file or directory (os error 2)
   |
   |
LL | include!("auxiliary/issue-21146-inc.rs");
   |
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error
---

pub struct Struct {
    pub a: usize,
}
//~^ ERROR expected item, found `;`
fn main() {}




The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-46186/issue-46186.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-46186.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-46186.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-46186" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-46186/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected item, found `;`
  --> /checkout/src/test/ui/parser/issue-46186.rs:5:2
   |
LL | };
   |  ^ help: remove this semicolon
   |
   = help: braced struct declarations are not followed by a semicolon
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/parser/issue-76597.rs stdout ----
normalized fixed:
// run-rustfix

#![allow(dead_code)]
#![allow(unused_variables)]
                                     x: u8,
                                     y: u8,
) {}
) {}
//~^^ ERROR: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `y`
fn main() {}




The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-76597/issue-76597.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-76597.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issue-76597.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-76597" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-76597/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `!`, `(`, `)`, `+`, `,`, `::`, or `<`, found `y`
   |
LL | ...                   x: u8
   |                            -
   |                            |
   |                            |
   |                            expected one of 7 possible tokens
   |                            help: missing `,`
LL | ...                   y: u8,
   |                       ^ unexpected token
error: aborting due to previous error


------------------------------------------
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:27
