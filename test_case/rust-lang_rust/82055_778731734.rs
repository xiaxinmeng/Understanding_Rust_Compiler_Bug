plain
.................................................................................................... 9200/11446
.................................................................................................... 9300/11446
.................................................................................................... 9400/11446
.....i......i....................................................................................... 9500/11446
...........................................iiiiiii..iiiiii.i........................................ 9600/11446
.................................................................................................... 9800/11446
.................................................................................................... 9900/11446
.................................................................................................... 10000/11446
.................................................................................................... 10100/11446
---

---- [ui] ui/const-generics/diagnostics.rs stdout ----
diff of stderr:

7 LL | impl Foo for A<N> {}
8    |                ^ help: a struct with a similar name exists: `A`
- error[E0747]: unresolved item provided when a constant was expected
- error[E0747]: unresolved item provided when a constant was expected
+ error[E0747]: unresolved item provided when a constan was expected
12    |
12    |
13 LL | impl Foo for A<N> {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/diagnostics/diagnostics.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/diagnostics/diagnostics.stderr
To update references, rerun the tests and pass the `--bless` flag
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
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

error[E0747]: unresolved item provided when a constan was expected
   |
   |
LL | impl Foo for A<N> {}
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces:
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
   |      help: consider changing this type paramater to a `const`-generic: `const N: u8`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0412, E0747.
For more information about an error, try `rustc --explain E0412`.
---
---- [ui] ui/const-generics/invalid-enum.rs stdout ----
diff of stderr:

25    |                  not a type
26    |                  help: try using the variant's enum: `CompileFlag`
- error[E0747]: unresolved item provided when a constant was expected
- error[E0747]: unresolved item provided when a constant was expected
+ error[E0747]: unresolved item provided when a constan was expected
30    |
30    |
31 LL |   let _: Example<CompileFlag::A, _> = Example { x: 0 };

47 LL |   let _: Example<{ Example::ASSOC_FLAG }, _> = Example { x: 0 };
48    |                  ^                     ^
- error[E0747]: unresolved item provided when a constant was expected
- error[E0747]: unresolved item provided when a constant was expected
+ error[E0747]: unresolved item provided when a constan was expected
52    |
52    |
53 LL |   test_1::<CompileFlag::A>();

58 LL |   test_1::<{ CompileFlag::A }>();
59    |            ^                ^
- error[E0747]: unresolved item provided when a constant was expected
- error[E0747]: unresolved item provided when a constant was expected
+ error[E0747]: unresolved item provided when a constan was expected
63    |
63    |
64 LL |   test_2::<_, CompileFlag::A>(0);

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-enum/invalid-enum.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-enum/invalid-enum.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/invalid-enum.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/invalid-enum.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-enum" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/invalid-enum/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0573]: expected type, found variant `CompileFlag::A`
   |
   |
LL |   test_1::<CompileFlag::A>();
   |            |
   |            not a type
   |            not a type
   |            help: try using the variant's enum: `CompileFlag`

error[E0573]: expected type, found variant `CompileFlag::A`
   |
   |
LL |   test_2::<_, CompileFlag::A>(0);
   |               |
   |               not a type
   |               not a type
   |               help: try using the variant's enum: `CompileFlag`

error[E0573]: expected type, found variant `CompileFlag::A`
   |
   |
LL |   let _: Example<CompileFlag::A, _> = Example { x: 0 };
   |                  |
   |                  not a type
   |                  not a type
   |                  help: try using the variant's enum: `CompileFlag`

error[E0747]: unresolved item provided when a constan was expected
   |
   |
LL |   let _: Example<CompileFlag::A, _> = Example { x: 0 };
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces:
   |
LL |   let _: Example<{ CompileFlag::A }, _> = Example { x: 0 };
   |                  ^                ^
error[E0747]: type provided when a constant was expected
  --> /checkout/src/test/ui/const-generics/invalid-enum.rs:33:18
   |
   |
LL |   let _: Example<Example::ASSOC_FLAG, _> = Example { x: 0 };
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces:
   |
LL |   let _: Example<{ Example::ASSOC_FLAG }, _> = Example { x: 0 };
   |                  ^                     ^

error[E0747]: unresolved item provided when a constan was expected
   |
   |
LL |   test_1::<CompileFlag::A>();
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces:
   |
LL |   test_1::<{ CompileFlag::A }>();
   |            ^                ^

error[E0747]: unresolved item provided when a constan was expected
   |
   |
LL |   test_2::<_, CompileFlag::A>(0);
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces:
   |
LL |   test_2::<_, { CompileFlag::A }>(0);
   |               ^                ^
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0573, E0747.
For more information about an error, try `rustc --explain E0573`.
---
test result: FAILED. 11351 passed; 2 failed; 93 ignored; 0 measured; 0 filtered out; finished in 135.42s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:22
