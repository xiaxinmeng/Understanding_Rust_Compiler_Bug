plain
.................................................................................................... 9500/11880
.................................................................................................... 9600/11880
.................................................................................i......i........... 9700/11880
.................................................................................................... 9800/11880
..........................iiiiiii..iiiiii.i......................................................... 9900/11880
.................................................................................................... 10100/11880
.................................................................................................... 10200/11880
.................................................................................................... 10300/11880
.................................................................................................... 10400/11880
---
Suite("src/test/assembly") not skipped for "bootstrap::test::Assembly" -- not in ["src/tools/tidy"]
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 36 tests
iiiiiiiiiiiiiiiiiiiiiiiiiiiiiiii....
test result: ok. 4 passed; 0 failed; 32 ignored; 0 measured; 0 filtered out; finished in 0.11s

 finished in 0.178 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
Suite("src/test/debuginfo") not skipped for "bootstrap::test::Debuginfo" -- not in ["src/tools/tidy"]
Check compiletest suite=debuginfo mode=debuginfo (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 116 tests
iiiiiiiiii.i.i..i..i..ii....i.i....ii..........iiii.........i.....i...i.......ii.i.ii.....iiii.....i 100/116
test result: ok. 78 passed; 0 failed; 38 ignored; 0 measured; 0 filtered out; finished in 3.65s

 finished in 3.719 seconds
Suite("src/test/ui-fulldeps") not skipped for "bootstrap::test::UiFullDeps" -- not in ["src/tools/tidy"]
---
Suite("src/test/run-make") not skipped for "bootstrap::test::RunMake" -- not in ["src/tools/tidy"]
Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 25 tests
iiiiiiiiiiiii............

 finished in 3.093 seconds
Build completed successfully in 0:31:27
+ python2.7 ../x.py --stage 2 test src/test/mir-opt --host= --target=i686-unknown-linux-gnu
---
Check compiletest suite=ui mode=ui (x86_64-unknown-linux-gnu -> i686-unknown-linux-gnu)

running 11880 tests
......................i............................................................................. 100/11880
......................................iiiiiii..iii.iiiiiiiii........................................ 200/11880
.................................................................................................... 400/11880
.................................................................................................... 500/11880
..................................................................................................i. 600/11880
.................................................................................................... 700/11880
---
.................................................................................................... 9400/11880
.................................................................................................... 9500/11880
.................................................................................................... 9600/11880
.................................................................................i......i........... 9700/11880
..........................................iiiiiii................................................... 9800/11880
..........................iiiiiii..iiiiii.i......................................................... 9900/11880
.................................................................................................... 10100/11880
.................................................................................................... 10200/11880
.................................................................................................... 10300/11880
.................................................................................................... 10400/11880
---
........................i..............................................i.i.......................... 11800/11880
................................................................................
failures:

---- [ui] ui/proc-macro/issue-84428-rental-back-compat.rs stdout ----

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
3 PRINT-DERIVE INPUT (DEBUG): TokenStream [
4     Ident {
5         ident: "enum",
-         span: #0 bytes(10794903..10794907),
+         span: #0 bytes(10265830..10265834),
8     Ident {
8     Ident {
9         ident: "ProceduralMasqueradeDummyType",

-         span: #0 bytes(10794908..10794937),
+         span: #0 bytes(10265835..10265864),
12     Group {
13         delimiter: Brace,


14         stream: TokenStream [
15             Ident {
16                 ident: "Input",
-                 span: #0 bytes(10794944..10794949),
+                 span: #0 bytes(10265871..10265876),
19         ],
19         ],
-         span: #0 bytes(10794938..10794951),
+         span: #0 bytes(10265865..10265878),
22 ]
23 



The actual stdout differed from the expected stdout.
Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-84428-rental-back-compat/issue-84428-rental-back-compat.stdout
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/issue-84428-rental-back-compat.rs`
error: 1 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/issue-84428-rental-back-compat.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-84428-rental-back-compat" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/issue-84428-rental-back-compat/auxiliary"
------------------------------------------
------------------------------------------
PRINT-DERIVE INPUT (DISPLAY): enum ProceduralMasqueradeDummyType { Input, }
PRINT-DERIVE RE-COLLECTED (DISPLAY): enum ProceduralMasqueradeDummyType { Input }
PRINT-DERIVE INPUT (DEBUG): TokenStream [
    Ident {
        ident: "enum",
        span: #0 bytes(10265830..10265834),
    Ident {
    Ident {
        ident: "ProceduralMasqueradeDummyType",
        span: #0 bytes(10265835..10265864),
    Group {
        delimiter: Brace,
        delimiter: Brace,
        stream: TokenStream [
            Ident {
                ident: "Input",
                span: #0 bytes(10265871..10265876),
        ],
        ],
        span: #0 bytes(10265865..10265878),
]

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: using outdated version of `rental` crate
  --> /checkout/src/test/ui/proc-macro/pretty-print-compat-hack/rental-0.5.5.rs:7:6
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = note: `#[warn(proc_macro_back_compat)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: Versions of the `rental` crate below `0.5.6` will eventually stop compiling. Please update to the latest version of `rental`
warning: 1 warning emitted


Future incompatibility report: Future breakage date: None, diagnostic:
warning: using outdated version of `rental` crate
  --> /checkout/src/test/ui/proc-macro/pretty-print-compat-hack/rental-0.5.5.rs:7:6
   |
LL | enum ProceduralMasqueradeDummyType {
   |
   = note: `#[warn(proc_macro_back_compat)]` on by default
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: for more information, see issue #83125 <https://github.com/rust-lang/rust/issues/83125>
   = note: Versions of the `rental` crate below `0.5.6` will eventually stop compiling. Please update to the latest version of `rental`

------------------------------------------


---
test result: FAILED. 11743 passed; 1 failed; 136 ignored; 0 measured; 0 filtered out; finished in 73.22s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test src/test/ui --pass=check --host= --target=i686-unknown-linux-gnu
Build completed unsuccessfully in 0:01:15
