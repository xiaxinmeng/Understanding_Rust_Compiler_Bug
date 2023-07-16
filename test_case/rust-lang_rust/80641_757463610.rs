plain
.................................................................................................... 9000/11246
.................................................................................................... 9100/11246
.................................................................................................... 9200/11246
..........................................i......i.................................................. 9300/11246
.................................................................................iiiiii..iiiiii.i... 9400/11246
.................................................................................................... 9600/11246
.................................................................................................... 9700/11246
.................................................................................................... 9800/11246
.................................................................................................... 9900/11246
---

---- [ui] ui/internal/internal-unstable.rs stdout ----
diff of stderr:

1 error[E0658]: use of unstable library feature 'function'
-   --> $DIR/internal-unstable.rs:40:25
3    |
3    |
4 LL |     pass_through_allow!(internal_unstable::unstable());


7    = help: add `#![feature(function)]` to the crate attributes to enable
8 
9 error[E0658]: use of unstable library feature 'function'
-   --> $DIR/internal-unstable.rs:42:27
11    |
11    |
12 LL |     pass_through_noallow!(internal_unstable::unstable());


15    = help: add `#![feature(function)]` to the crate attributes to enable
16 
17 error[E0658]: use of unstable library feature 'function'
-   --> $DIR/internal-unstable.rs:46:22
19    |
19    |
20 LL |     println!("{:?}", internal_unstable::unstable());


23    = help: add `#![feature(function)]` to the crate attributes to enable
24 
25 error[E0658]: use of unstable library feature 'function'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/internal-unstable.rs:48:10
27    |
27    |
28 LL |     bar!(internal_unstable::unstable());


31    = help: add `#![feature(function)]` to the crate attributes to enable
32 
33 error[E0658]: use of unstable library feature 'function'
-   --> $DIR/internal-unstable.rs:18:9
35    |
36 LL |         internal_unstable::unstable();
37    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/internal/internal-unstable/internal-unstable.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args internal/internal-unstable.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/internal/internal-unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/internal/internal-unstable" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/internal/internal-unstable/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: use of unstable library feature 'function'
   |
   |
LL |     pass_through_allow!(internal_unstable::unstable()); //~ ERROR use of unstable
   |
   |
   = help: add `#![feature(function)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'function'
   |
   |
LL |     pass_through_noallow!(internal_unstable::unstable()); //~ ERROR use of unstable
   |
   |
   = help: add `#![feature(function)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'function'
   |
   |
LL |     println!("{:?}", internal_unstable::unstable()); //~ ERROR use of unstable
   |
   |
   = help: add `#![feature(function)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'function'
   |
   |
LL |     bar!(internal_unstable::unstable()); //~ ERROR use of unstable
   |
   |
   = help: add `#![feature(function)]` to the crate attributes to enable

error[E0658]: use of unstable library feature 'function'
   |
   |
LL |         internal_unstable::unstable(); //~ ERROR use of unstable
...
...
LL |     bar!(internal_unstable::unstable()); //~ ERROR use of unstable
   |
   |
   = help: add `#![feature(function)]` to the crate attributes to enable
   = note: this error originates in a macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0658`.

---
test result: FAILED. 11159 passed; 1 failed; 86 ignored; 0 measured; 0 filtered out; finished in 143.95s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:15
