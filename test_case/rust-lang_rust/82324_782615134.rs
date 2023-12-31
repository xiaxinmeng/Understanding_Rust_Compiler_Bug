plain
.................................................................................................... 9200/11471
.................................................................................................... 9300/11471
.................................................................................................... 9400/11471
.............................i......i............................................................... 9500/11471
....................................................................iiiiiii..iiiiii.i............... 9600/11471
.................................................................................................... 9800/11471
.................................................................................................... 9900/11471
.................................................................................................... 10000/11471
.................................................................................................... 10100/11471
---

---- [ui] ui/const-generics/diagnostics.rs stdout ----
diff of stderr:

33 LL | impl<N> Foo for B<N> {}
34    |      -            ^
35    |      |
-    |      help: consider changing this type paramater to a `const`-generic: `const N: u8`
+    |      help: consider changing this type parameter to a `const`-generic: `const N: u8`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
38 error[E0747]: unresolved item provided when a constant was expected
39   --> $DIR/diagnostics.rs:16:32



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/diagnostics/diagnostics.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/diagnostics.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/diagnostics.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/diagnostics" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/diagnostics/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0412]: cannot find type `N` in this scope
  --> /checkout/src/test/ui/const-generics/diagnostics.rs:7:16
   |
LL | struct A<const N: u8>;
   | ---------------------- similarly named struct `A` defined here
LL | trait Foo {}
LL | impl Foo for A<N> {}
   |                ^ help: a struct with a similar name exists: `A`
error[E0412]: cannot find type `T` in this scope
  --> /checkout/src/test/ui/const-generics/diagnostics.rs:16:32
   |
   |
LL | struct A<const N: u8>;
   | ---------------------- similarly named struct `A` defined here
...
LL | impl<const N: u8> Foo for C<N, T> {}
   |                                ^ help: a struct with a similar name exists: `A`
error[E0747]: unresolved item provided when a constant was expected
  --> /checkout/src/test/ui/const-generics/diagnostics.rs:7:16
   |
   |
LL | impl Foo for A<N> {}
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
LL | impl Foo for A<{ N }> {}
   |                ^   ^
error[E0747]: type provided when a constant was expected
  --> /checkout/src/test/ui/const-generics/diagnostics.rs:12:19
   |
   |
LL | impl<N> Foo for B<N> {}
   |      -            ^
   |      |
   |      help: consider changing this type parameter to a `const`-generic: `const N: u8`
error[E0747]: unresolved item provided when a constant was expected
  --> /checkout/src/test/ui/const-generics/diagnostics.rs:16:32
   |
   |
LL | impl<const N: u8> Foo for C<N, T> {}
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
LL | impl<const N: u8> Foo for C<N, { T }> {}
   |                                ^   ^
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0412, E0747.
For more information about an error, try `rustc --explain E0412`.
---
test result: FAILED. 11377 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 141.06s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:24
