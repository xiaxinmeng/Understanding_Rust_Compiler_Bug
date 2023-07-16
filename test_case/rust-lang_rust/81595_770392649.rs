plain
.................................................................................................... 1500/11329
.................................................................................................... 1600/11329
.................................................................................................... 1700/11329
...........................................................................i........................ 1800/11329
...........................................................F.F...................................... 1900/11329
.................................................................................................... 2100/11329
.................................................................................................... 2200/11329
.................................................................................................... 2300/11329
.................................................................................................... 2400/11329
---

---- [ui] ui/const-generics/issues/issue-69654.rs stdout ----
diff of stderr:

4 LL | impl<T> Bar<T> for [u8; T] {}
5    |                         ^ not a value
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
6 
- error[E0599]: no function or associated item named `foo` found for struct `Foo<{_: usize}>` in the current scope
+ error[E0599]: the function or associated item `foo` exists for struct `Foo<{_: usize}>`, but its trait bounds were not satisfied
9    |
9    |
10 LL | struct Foo<const N: usize> {}
11    | -------------------------- function or associated item `foo` not found for this
12 ...
13 LL |     Foo::foo();
13 LL |     Foo::foo();
-    |          ^^^ function or associated item not found in `Foo<{_: usize}>`
+    |          ^^^ function or associated item cannot be called on `Foo<{_: usize}>` due to unsatisfied trait bounds
15    |
-    = note: the method `foo` exists but the following trait bounds were not satisfied:
+    = note: the following trait bounds were not satisfied:
17            `[u8; _]: Bar<[(); _]>`
19 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654/issue-69654.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-69654.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-69654.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0423]: expected value, found type parameter `T`
   |
   |
LL | impl<T> Bar<T> for [u8; T] {}
   |                         ^ not a value

error[E0599]: the function or associated item `foo` exists for struct `Foo<{_: usize}>`, but its trait bounds were not satisfied
   |
   |
LL | struct Foo<const N: usize> {}
   | -------------------------- function or associated item `foo` not found for this
LL |     Foo::foo();
LL |     Foo::foo();
   |          ^^^ function or associated item cannot be called on `Foo<{_: usize}>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `[u8; _]: Bar<[(); _]>`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0423, E0599.
For more information about an error, try `rustc --explain E0423`.
For more information about an error, try `rustc --explain E0423`.

------------------------------------------


---- [ui] ui/const-generics/issues/issue-69654-run-pass.rs stdout ----
diff of stderr:

- error[E0599]: no function or associated item named `foo` found for struct `Foo<{_: usize}>` in the current scope
+ error[E0599]: the function or associated item `foo` exists for struct `Foo<{_: usize}>`, but its trait bounds were not satisfied
3    |
3    |
4 LL | struct Foo<const N: usize> {}
5    | -------------------------- function or associated item `foo` not found for this
6 ...
7 LL |     Foo::foo();
7 LL |     Foo::foo();
-    |          ^^^ function or associated item not found in `Foo<{_: usize}>`
+    |          ^^^ function or associated item cannot be called on `Foo<{_: usize}>` due to unsatisfied trait bounds
9    |
-    = note: the method `foo` exists but the following trait bounds were not satisfied:
+    = note: the following trait bounds were not satisfied:
11            `[u8; _]: Bar<[(); _]>`
13 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654-run-pass/issue-69654-run-pass.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/issues/issue-69654-run-pass.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-69654-run-pass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654-run-pass" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-69654-run-pass/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: the function or associated item `foo` exists for struct `Foo<{_: usize}>`, but its trait bounds were not satisfied
   |
   |
LL | struct Foo<const N: usize> {}
   | -------------------------- function or associated item `foo` not found for this
LL |     Foo::foo();
LL |     Foo::foo();
   |          ^^^ function or associated item cannot be called on `Foo<{_: usize}>` due to unsatisfied trait bounds
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `[u8; _]: Bar<[(); _]>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.

---
test result: FAILED. 11239 passed; 2 failed; 88 ignored; 0 measured; 0 filtered out; finished in 115.27s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:54
