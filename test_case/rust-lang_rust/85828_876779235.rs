plain
..........................................................i.i....................................... 12000/12067
...................................................................
failures:

---- [ui] ui/intrinsics/intrinsic-raw_eq-const-padding.rs stdout ----

- error: any use of this value will cause an error
+ error[E0080]: evaluation of constant value failed
+ error[E0080]: evaluation of constant value failed
2   --> $DIR/intrinsic-raw_eq-const-padding.rs:6:5
3    |
- LL | / const BAD_RAW_EQ_CALL: bool = unsafe {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL | |     std::intrinsics::raw_eq(&(1_u8, 2_u16), &(1_u8, 2_u16))
-    | |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reading 4 bytes of memory starting at alloc2, but 1 byte is uninitialized starting at alloc2+0x1, and this operation requires initialized memory
- LL | |
- LL | |
- LL | | };
-    |
- note: the lint level is defined here
- note: the lint level is defined here
-   --> $DIR/intrinsic-raw_eq-const-padding.rs:3:9
-    |
- LL | #![deny(const_err)]
-    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
+ LL |     std::intrinsics::raw_eq(&(1_u8, 2_u16), &(1_u8, 2_u16))
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reading 4 bytes of memory starting at alloc2, but 1 byte is uninitialized starting at alloc2+0x1, and this operation requires initialized memory
20 error: aborting due to previous error
21 

+ For more information about this error, try `rustc --explain E0080`.
+ For more information about this error, try `rustc --explain E0080`.
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/intrinsic-raw_eq-const-padding/intrinsic-raw_eq-const-padding.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args intrinsics/intrinsic-raw_eq-const-padding.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/intrinsics/intrinsic-raw_eq-const-padding.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/intrinsic-raw_eq-const-padding" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/intrinsics/intrinsic-raw_eq-const-padding/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/intrinsics/intrinsic-raw_eq-const-padding.rs:6:5
   |
LL |     std::intrinsics::raw_eq(&(1_u8, 2_u16), &(1_u8, 2_u16))
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reading 4 bytes of memory starting at alloc2, but 1 byte is uninitialized starting at alloc2+0x1, and this operation requires initialized memory
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.


------------------------------------------



failures:
    [ui] ui/intrinsics/intrinsic-raw_eq-const-padding.rs
test result: FAILED. 11966 passed; 1 failed; 100 ignored; 0 measured; 0 filtered out; finished in 125.09s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:59
