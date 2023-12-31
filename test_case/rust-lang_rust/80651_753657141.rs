plain
.................................................................................................... 9000/11241
.................................................................................................... 9100/11241
.................................................................................................... 9200/11241
.....................................i......i....................................................... 9300/11241
............................................................................iiiiii..iiiiii.i........ 9400/11241
.................................................................................................... 9600/11241
.................................................................................................... 9700/11241
.................................................................................................... 9800/11241
.................................................................................................... 9900/11241
---
diff of stderr:

23    |
24    = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
25    = note: the matched value is of type `&Void`
+    = note: references are always considered inhabited
26 
27 error[E0004]: non-exhaustive patterns: type `(Void,)` is non-empty


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated/uninhabited-matches-feature-gated.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated/uninhabited-matches-feature-gated.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args uninhabited/uninhabited-matches-feature-gated.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/uninhabited/uninhabited-matches-feature-gated.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/uninhabited/uninhabited-matches-feature-gated/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
------------------------------------------
stderr:
------------------------------------------
error[E0004]: non-exhaustive patterns: `Err(_)` not covered
   |
   |
LL |     let _ = match x {   //~ ERROR non-exhaustive
   |                   ^ pattern `Err(_)` not covered
  ::: /checkout/library/core/src/result.rs:250:5
   |
   |
LL |     Err(#[stable(feature = "rust1", since = "1.0.0")] E),
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `std::result::Result<u32, &Void>`

error[E0004]: non-exhaustive patterns: type `&Void` is non-empty
   |
   |
LL | enum Void {}
   | ------------ `Void` defined here
...
LL |     let _ = match x {}; //~ ERROR non-exhaustive
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `&Void`
   = note: the matched value is of type `&Void`
   = note: references are always considered inhabited

error[E0004]: non-exhaustive patterns: type `(Void,)` is non-empty
   |
   |
LL |     let _ = match x {}; //~ ERROR non-exhaustive
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `(Void,)`

error[E0004]: non-exhaustive patterns: type `[Void; 1]` is non-empty
   |
   |
LL |     let _ = match x {}; //~ ERROR non-exhaustive
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `[Void; 1]`

error[E0004]: non-exhaustive patterns: `&[_, ..]` not covered
   |
   |
LL |     let _ = match x {   //~ ERROR non-exhaustive
   |                   ^ pattern `&[_, ..]` not covered
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `&[Void]`

error[E0004]: non-exhaustive patterns: `Err(_)` not covered
   |
   |
LL |     let _ = match x {   //~ ERROR non-exhaustive
   |                   ^ pattern `Err(_)` not covered
  ::: /checkout/library/core/src/result.rs:250:5
   |
   |
LL |     Err(#[stable(feature = "rust1", since = "1.0.0")] E),
   |
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `std::result::Result<u32, Void>`

error[E0005]: refutable pattern in local binding: `Err(_)` not covered
   |
   |
LL |     let Ok(x) = x;
   |         ^^^^^ pattern `Err(_)` not covered
  ::: /checkout/library/core/src/result.rs:250:5
   |
   |
LL |     Err(#[stable(feature = "rust1", since = "1.0.0")] E),
   |
   |
   = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
   = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
   = note: the matched value is of type `std::result::Result<u32, Void>`
help: you might want to use `if let` to ignore the variant that isn't matched
   |
LL |     if let Ok(x) = x { /* */ }

error: aborting due to 7 previous errors

Some errors have detailed explanations: E0004, E0005.
---
test result: FAILED. 11155 passed; 1 failed; 85 ignored; 0 measured; 0 filtered out; finished in 136.85s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:03
