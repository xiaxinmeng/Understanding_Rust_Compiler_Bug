plain
.................................................................................................... 9400/11843
.................................................................................................... 9500/11843
.................................................................................................... 9600/11843
.....................................................i......i....................................... 9700/11843
..................................................................................................ii 9800/11843
iiiii..iiiiii.i..................................................................................... 9900/11843
................................................F.FF............F................................... 10000/11843
.................................................................................................... 10200/11843
.................................................................................................... 10300/11843
.................................................................................................... 10400/11843
.................................................................................................... 10500/11843
---
failures:

---- [ui] ui/issues/issue-38074.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-38074.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38074/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-38074/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |     let r: u64x2 = unsafe { simd_shuffle2(a, a, [0-0, 0-0]) };

error: aborting due to previous error



------------------------------------------


---- [ui] ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |     simd_shuffle2(Simd2(10, 11), Simd2(12, 13), [0, 3])

error: aborting due to previous error



------------------------------------------


---- [ui] ui/simd-intrinsic/simd-intrinsic-generic-elements.rs stdout ----
diff of stderr:

- error[E0511]: invalid monomorphization of `simd_insert` intrinsic: expected SIMD input type, found non-SIMD `i32`
-   --> $DIR/simd-intrinsic-generic-elements.rs:46:9
-    |
- LL |         simd_insert(0, 0, 0);
- 
- 
- error[E0511]: invalid monomorphization of `simd_insert` intrinsic: expected inserted type `i32` (element of input `i32x4`), found `f64`
-   --> $DIR/simd-intrinsic-generic-elements.rs:48:9
-    |
- LL |         simd_insert(x, 0, 1.0);
- 
- 
- error[E0511]: invalid monomorphization of `simd_extract` intrinsic: expected return type `i32` (element of input `i32x4`), found `f32`
-   --> $DIR/simd-intrinsic-generic-elements.rs:50:9
-    |
- LL |         simd_extract::<_, f32>(x, 0);
- 
- 
- error[E0511]: invalid monomorphization of `simd_shuffle2` intrinsic: expected SIMD input type, found non-SIMD `i32`
+ error: last argument of `simd_shuffle` is required to be a `const` item
21    |
21    |
22 LL |         simd_shuffle2::<i32, i32>(0, 0, [0; 2]);
23    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
24 
24 
- error[E0511]: invalid monomorphization of `simd_shuffle4` intrinsic: expected SIMD input type, found non-SIMD `i32`
+ error: last argument of `simd_shuffle` is required to be a `const` item
27    |
27    |
28 LL |         simd_shuffle4::<i32, i32>(0, 0, [0; 4]);
29    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
30 
30 
- error[E0511]: invalid monomorphization of `simd_shuffle8` intrinsic: expected SIMD input type, found non-SIMD `i32`
+ error: last argument of `simd_shuffle` is required to be a `const` item
33    |
33    |
34 LL |         simd_shuffle8::<i32, i32>(0, 0, [0; 8]);
35    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
36 
36 
- error[E0511]: invalid monomorphization of `simd_shuffle2` intrinsic: expected return element type `i32` (element of input `i32x4`), found `f32x2` with element type `f32`
+ error: last argument of `simd_shuffle` is required to be a `const` item
39    |
39    |
40 LL |         simd_shuffle2::<_, f32x2>(x, x, [0; 2]);
41    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
42 
42 
- error[E0511]: invalid monomorphization of `simd_shuffle4` intrinsic: expected return element type `i32` (element of input `i32x4`), found `f32x4` with element type `f32`
+ error: last argument of `simd_shuffle` is required to be a `const` item
45    |
45    |
46 LL |         simd_shuffle4::<_, f32x4>(x, x, [0; 4]);
47    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
48 
48 
- error[E0511]: invalid monomorphization of `simd_shuffle8` intrinsic: expected return element type `i32` (element of input `i32x4`), found `f32x8` with element type `f32`
+ error: last argument of `simd_shuffle` is required to be a `const` item
51    |
51    |
52 LL |         simd_shuffle8::<_, f32x8>(x, x, [0; 8]);
53    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
54 
54 
- error[E0511]: invalid monomorphization of `simd_shuffle2` intrinsic: expected return type of length 2, found `i32x8` with length 8
+ error: last argument of `simd_shuffle` is required to be a `const` item
57    |
57    |
58 LL |         simd_shuffle2::<_, i32x8>(x, x, [0; 2]);
59    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
60 
60 
- error[E0511]: invalid monomorphization of `simd_shuffle4` intrinsic: expected return type of length 4, found `i32x8` with length 8
+ error: last argument of `simd_shuffle` is required to be a `const` item
63    |
63    |
64 LL |         simd_shuffle4::<_, i32x8>(x, x, [0; 4]);
65    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
66 
66 
- error[E0511]: invalid monomorphization of `simd_shuffle8` intrinsic: expected return type of length 8, found `i32x2` with length 2
+ error: last argument of `simd_shuffle` is required to be a `const` item
69    |
69    |
70 LL |         simd_shuffle8::<_, i32x2>(x, x, [0; 8]);
71    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
72 
- error: aborting due to 12 previous errors
+ error: aborting due to 9 previous errors
---
To only update this specific test, also pass `--test-args simd-intrinsic/simd-intrinsic-generic-elements.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-generic-elements.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-generic-elements" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-generic-elements/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         simd_shuffle2::<i32, i32>(0, 0, [0; 2]);


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         simd_shuffle4::<i32, i32>(0, 0, [0; 4]);


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         simd_shuffle8::<i32, i32>(0, 0, [0; 8]);


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         simd_shuffle2::<_, f32x2>(x, x, [0; 2]);


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         simd_shuffle4::<_, f32x4>(x, x, [0; 4]);


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         simd_shuffle8::<_, f32x8>(x, x, [0; 8]);


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         simd_shuffle2::<_, i32x8>(x, x, [0; 2]);


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         simd_shuffle4::<_, i32x8>(x, x, [0; 4]);


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         simd_shuffle8::<_, i32x2>(x, x, [0; 8]);

error: aborting due to 9 previous errors



------------------------------------------


---- [ui] ui/simd-intrinsic/simd-intrinsic-inlining-issue67557.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zmir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd-intrinsic/simd-intrinsic-inlining-issue67557/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         let p_res: Simd2 = simd_shuffle2(Simd2(10, 11), Simd2(12, 13), [0, 1]);


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |     simd_shuffle2(Simd2(10, 11), Simd2(12, 13), [0, 3])

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/simd/simd-intrinsic-generic-elements.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/simd/simd-intrinsic-generic-elements.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/simd-intrinsic-generic-elements/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/simd/simd-intrinsic-generic-elements/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         all_eq!(simd_shuffle2(x2, y2, [3, 0]), i32x2(121, 20));


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         all_eq!(simd_shuffle4(x2, y2, [3, 0, 1, 2]), i32x4(121, 20, 21, 120));


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         all_eq!(simd_shuffle8(x2, y2, [3, 0, 1, 2, 1, 2, 3, 0]),


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         all_eq!(simd_shuffle2(x4, y4, [7, 2]), i32x2(143, 42));


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         all_eq!(simd_shuffle4(x4, y4, [7, 2, 5, 0]), i32x4(143, 42, 141, 40));


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         all_eq!(simd_shuffle8(x4, y4, [7, 2, 5, 0, 3, 6, 4, 1]),


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         all_eq!(simd_shuffle2(x8, y8, [11, 5]), i32x2(183, 85));


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         all_eq!(simd_shuffle4(x8, y8, [11, 5, 15, 0]), i32x4(183, 85, 187, 80));


error: last argument of `simd_shuffle` is required to be a `const` item
   |
   |
LL |         all_eq!(simd_shuffle8(x8, y8, [11, 5, 15, 0, 3, 8, 12, 1]),

error: aborting due to 9 previous errors


---
test result: FAILED. 11742 passed; 5 failed; 96 ignored; 0 measured; 0 filtered out; finished in 120.86s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:12
