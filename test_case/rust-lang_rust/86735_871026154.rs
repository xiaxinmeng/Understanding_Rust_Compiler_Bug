plain
...................i.i.............................................................................. 12000/12028
............................
failures:

---- [ui] ui/deriving/deriving-default-enum.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deriving/deriving-default-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-default-enum/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deriving/deriving-default-enum/auxiliary"
------------------------------------------

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
---

---- [ui] ui/error-codes/E0665.rs stdout ----
diff of stderr:

- error[E0665]: `Default` cannot be derived for enums, only structs
+ error: no default variant declared
2   --> $DIR/E0665.rs:1:10
4 LL | #[derive(Default)]

5    |          ^^^^^^^
6    |
6    |
+    = help: make a unit variant default by placing `#[default]` above it
7    = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)
9 error: aborting due to previous error

10 
- For more information about this error, try `rustc --explain E0665`.
---
To only update this specific test, also pass `--test-args error-codes/E0665.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0665.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0665" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0665/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: no default variant declared
  --> /checkout/src/test/ui/error-codes/E0665.rs:1:10
   |
LL | #[derive(Default)] //~ ERROR E0665
   |
   |
   = help: make a unit variant default by placing `#[default]` above it
   = note: this error originates in the derive macro `Default` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/macros/macros-nonfatal-errors.rs stdout ----

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:60: unexpected error: '60:5: 60:8: default variant must be exhaustive'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:58: unexpected error: '58:7: 58:14: cannot find attribute `default` in this scope'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:51: unexpected error: '51:7: 51:14: cannot find attribute `default` in this scope'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:41: unexpected error: '41:7: 41:14: cannot find attribute `default` in this scope'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:42: unexpected error: '42:7: 42:14: cannot find attribute `default` in this scope'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:43: unexpected error: '43:7: 43:14: cannot find attribute `default` in this scope'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:44: unexpected error: '44:7: 44:14: cannot find attribute `default` in this scope'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:33: unexpected error: '33:7: 33:14: cannot find attribute `default` in this scope'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:34: unexpected error: '34:7: 34:14: cannot find attribute `default` in this scope'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:27: unexpected error: '27:7: 27:14: cannot find attribute `default` in this scope'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:21: unexpected error: '21:7: 21:14: cannot find attribute `default` in this scope'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:19: unexpected error: '19:7: 19:14: cannot find attribute `default` in this scope'

error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:17: unexpected error: '17:7: 17:14: cannot find attribute `default` in this scope'
error: /checkout/src/test/ui/macros/macros-nonfatal-errors.rs:59: expected error not found: 

error: 13 unexpected errors found, 1 expected errors not found
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macros-nonfatal-errors.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macros-nonfatal-errors/auxiliary"
    Error {
        line_num: 60,
        kind: Some(
            Error,
            Error,
        ),
        msg: "60:5: 60:8: default variant must be exhaustive",
    Error {
        line_num: 58,
        kind: Some(
            Error,
---
test result: FAILED. 11928 passed; 3 failed; 97 ignored; 0 measured; 0 filtered out; finished in 119.11s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:23
