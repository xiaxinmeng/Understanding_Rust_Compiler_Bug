plain
.................................................................................................... 9200/11452
.................................................................................................... 9300/11452
.................................................................................................... 9400/11452
...........i......i................................................................................. 9500/11452
.................................................iiiiiii...iiiiiii.................................. 9600/11452
.................................................................................................... 9800/11452
.................................................................................................... 9900/11452
.................................................................................................... 10000/11452
.................................................................................................... 10100/11452
---

---- [ui] ui/const-generics/const-param-shadowing.rs stdout ----
diff of stderr:

4 LL | fn test<const N: usize>() -> Foo<N> {
6    |
6    |
- help: if this generic argument was intended as a const parameter, surround it with braces:
+ help: if this generic argument was intended as a const parameter, surround it with braces
8    |
9 LL | fn test<const N: usize>() -> Foo<{ N }> {
10    |                                  ^   ^

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-shadowing/const-param-shadowing.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-shadowing/const-param-shadowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/const-param-shadowing.rs`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-param-shadowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-shadowing" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-param-shadowing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0747]: type provided when a constant was expected
  --> /checkout/src/test/ui/const-generics/const-param-shadowing.rs:3:34
   |
LL | fn test<const N: usize>() -> Foo<N> { //~ ERROR type provided when
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
LL | fn test<const N: usize>() -> Foo<{ N }> { //~ ERROR type provided when
   |                                  ^   ^
error: aborting due to previous error

For more information about this error, try `rustc --explain E0747`.


------------------------------------------


---- [ui] ui/const-generics/diagnostics.rs stdout ----
diff of stderr:

22 LL | impl Foo for A<N> {}
24    |
24    |
- help: if this generic argument was intended as a const parameter, surround it with braces:
+ help: if this generic argument was intended as a const parameter, surround it with braces
26    |
27 LL | impl Foo for A<{ N }> {}
28    |                ^   ^
43    |
43    |
44    = note: type arguments must be provided before constant arguments
45    = help: reorder the arguments: consts: `<C, N>`
- help: if this generic argument was intended as a const parameter, surround it with braces:
+ help: if this generic argument was intended as a const parameter, surround it with braces
47    |
48 LL | impl<const N: u8> Foo for C<N, { T }> {}
49    |                                ^   ^

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/diagnostics/diagnostics.stderr
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
   |      help: consider changing this type paramater to a `const`-generic: `const N: u8`
error[E0747]: unresolved item provided when a constant was expected
  --> /checkout/src/test/ui/const-generics/diagnostics.rs:16:32
   |
   |
LL | impl<const N: u8> Foo for C<N, T> {}
   |
   |
   = note: type arguments must be provided before constant arguments
   = help: reorder the arguments: consts: `<C, N>`
help: if this generic argument was intended as a const parameter, surround it with braces
   |
LL | impl<const N: u8> Foo for C<N, { T }> {}
   |                                ^   ^
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0412, E0747.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.

------------------------------------------


---- [ui] ui/const-generics/invalid-enum.rs stdout ----
diff of stderr:

31 LL |   let _: Example<CompileFlag::A, _> = Example { x: 0 };
33    |
33    |
- help: if this generic argument was intended as a const parameter, surround it with braces:
+ help: if this generic argument was intended as a const parameter, surround it with braces
35    |
36 LL |   let _: Example<{ CompileFlag::A }, _> = Example { x: 0 };
37    |                  ^                ^

42 LL |   let _: Example<Example::ASSOC_FLAG, _> = Example { x: 0 };
44    |
44    |
- help: if this generic argument was intended as a const parameter, surround it with braces:
+ help: if this generic argument was intended as a const parameter, surround it with braces
46    |
47 LL |   let _: Example<{ Example::ASSOC_FLAG }, _> = Example { x: 0 };
48    |                  ^                     ^

53 LL |   test_1::<CompileFlag::A>();
55    |
55    |
- help: if this generic argument was intended as a const parameter, surround it with braces:
+ help: if this generic argument was intended as a const parameter, surround it with braces
57    |
58 LL |   test_1::<{ CompileFlag::A }>();
59    |            ^                ^

64 LL |   test_2::<_, CompileFlag::A>(0);
66    |
66    |
- help: if this generic argument was intended as a const parameter, surround it with braces:
+ help: if this generic argument was intended as a const parameter, surround it with braces
68    |
69 LL |   test_2::<_, { CompileFlag::A }>(0);
70    |               ^                ^

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
error[E0747]: unresolved item provided when a constant was expected
  --> /checkout/src/test/ui/const-generics/invalid-enum.rs:29:18
   |
   |
LL |   let _: Example<CompileFlag::A, _> = Example { x: 0 };
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
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
help: if this generic argument was intended as a const parameter, surround it with braces
   |
LL |   let _: Example<{ Example::ASSOC_FLAG }, _> = Example { x: 0 };
   |                  ^                     ^
error[E0747]: unresolved item provided when a constant was expected
  --> /checkout/src/test/ui/const-generics/invalid-enum.rs:21:12
   |
   |
LL |   test_1::<CompileFlag::A>();
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
LL |   test_1::<{ CompileFlag::A }>();
   |            ^                ^
error[E0747]: unresolved item provided when a constant was expected
  --> /checkout/src/test/ui/const-generics/invalid-enum.rs:25:15
   |
   |
LL |   test_2::<_, CompileFlag::A>(0);
   |
   |
help: if this generic argument was intended as a const parameter, surround it with braces
   |
LL |   test_2::<_, { CompileFlag::A }>(0);
   |               ^                ^
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0573, E0747.
For more information about an error, try `rustc --explain E0573`.
---
test result: FAILED. 11356 passed; 3 failed; 93 ignored; 0 measured; 0 filtered out; finished in 132.99s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:41
