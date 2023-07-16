plain

---- [ui] ui/stability-attribute/stability-attribute-sanity.rs stdout ----
diff of stderr:

40 LL |     #[unstable(feature = "b")]
42 
- error[E0546]: missing 'feature'
-   --> $DIR/stability-attribute-sanity.rs:31:5
-    |
-    |
- LL |     #[stable(since = "a")]
- 
49 error[E0542]: missing 'since'
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
50   --> $DIR/stability-attribute-sanity.rs:36:5
50   --> $DIR/stability-attribute-sanity.rs:36:5
51    |

120 LL | #[rustc_deprecated(since = "a", reason = "text")]
122 
- error: aborting due to 19 previous errors
+ error: aborting due to 18 previous errors
124 
---
To only update this specific test, also pass `--test-args stability-attribute/stability-attribute-sanity.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stability-attribute/stability-attribute-sanity/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0541]: unknown meta item 'reason'
   |
   |
LL |     #[stable(feature = "a", since = "b", reason)] //~ ERROR unknown meta item 'reason' [E0541]
   |                                          ^^^^^^ expected one of `since`, `note`
error[E0539]: incorrect meta item
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:11:29
   |
   |
LL |     #[stable(feature = "a", since)] //~ ERROR incorrect meta item [E0539]

error[E0539]: incorrect meta item
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:14:14
   |
   |
LL |     #[stable(feature, since = "a")] //~ ERROR incorrect meta item [E0539]

error[E0539]: incorrect meta item
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:17:29
   |
   |
LL |     #[stable(feature = "a", since(b))] //~ ERROR incorrect meta item [E0539]

error[E0539]: incorrect meta item
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:20:14
   |
   |
LL |     #[stable(feature(b), since = "a")] //~ ERROR incorrect meta item [E0539]

error[E0546]: missing 'feature'
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:25:5
   |
   |
LL |     #[unstable(issue = "none")] //~ ERROR missing 'feature' [E0546]

error[E0547]: missing 'issue'
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:28:5
   |
   |
LL |     #[unstable(feature = "b")] //~ ERROR missing 'issue' [E0547]

error[E0542]: missing 'since'
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:36:5
   |
   |
LL |     #[stable(feature = "a")] //~ ERROR missing 'since' [E0542]

error[E0542]: missing 'since'
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:40:5
   |
   |
LL |     #[rustc_deprecated(reason = "a")] //~ ERROR missing 'since' [E0542]

error[E0543]: missing 'reason'
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:44:5
   |
   |
LL |     #[rustc_deprecated(since = "a")] //~ ERROR missing 'reason' [E0543]

error[E0544]: multiple stability levels
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:49:1
   |
   |
LL | #[stable(feature = "a", since = "b")] //~ ERROR multiple stability levels [E0544]

error[E0544]: multiple stability levels
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:53:1
   |
   |
LL | #[unstable(feature = "b", issue = "none")] //~ ERROR multiple stability levels [E0544]

error[E0544]: multiple stability levels
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:57:1
   |
   |
LL | #[stable(feature = "a", since = "b")] //~ ERROR multiple stability levels [E0544]

error[E0550]: multiple deprecated attributes
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:62:1
   |
   |
LL | #[rustc_deprecated(since = "b", reason = "text")]
   | ------------------------------------------------- first deprecation attribute
LL | #[rustc_deprecated(since = "b", reason = "text")] //~ ERROR multiple deprecated attributes
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ repeated deprecation attribute
error[E0544]: multiple stability levels
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:64:1
   |
   |
LL | #[rustc_const_unstable(feature = "d", issue = "none")] //~ ERROR multiple stability levels

error: invalid stability version found
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:60:1
   |
   |
LL | #[stable(feature = "a", since = "b")] //~ ERROR invalid stability version found
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid stability version
LL | pub const fn multiple4() { }
   | ---------------------------- the stability attribute annotates this item

error: invalid deprecation version found
error: invalid deprecation version found
  --> /checkout/src/test/ui/stability-attribute/stability-attribute-sanity.rs:67:1
   |
LL | #[stable(feature = "a", since = "1.0.0")] //~ ERROR invalid deprecation version found
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ invalid deprecation version
LL | #[rustc_deprecated(since = "invalid", reason = "text")]
LL | fn invalid_deprecation_version() {}


error[E0549]: rustc_deprecated attribute must be paired with either stable or unstable attribute
   |
   |
LL | #[rustc_deprecated(since = "a", reason = "text")]

error: aborting due to 18 previous errors

Some errors have detailed explanations: E0539, E0541, E0542, E0543, E0544, E0546, E0547, E0549, E0550.
---
test result: FAILED. 11988 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 130.14s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:07
