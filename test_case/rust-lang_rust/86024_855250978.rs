plain
6 
7 error[E0452]: malformed lint attribute input
+   --> $DIR/reasons-erroneous.rs:3:49
+    |
+ LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
+    |                                                 ^^^^^^^^^^ reason in lint attribute must come last
+ 
+ error[E0452]: malformed lint attribute input
9    |
9    |
10 LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
11    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason must be a string literal
12 
13 error[E0452]: malformed lint attribute input
+   --> $DIR/reasons-erroneous.rs:10:31
+   --> $DIR/reasons-erroneous.rs:10:31
+    |
+ LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
+ 
+ error[E0452]: malformed lint attribute input
15    |
15    |
16 LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
73    |                                                          ^ reason must be a string literal
74 
75 error[E0452]: malformed lint attribute input
+   --> $DIR/reasons-erroneous.rs:3:49
+   --> $DIR/reasons-erroneous.rs:3:49
+    |
+ LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
+    |                                                 ^^^^^^^^^^ reason in lint attribute must come last
+ 
+ error[E0452]: malformed lint attribute input
77    |
77    |
78 LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
79    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason must be a string literal
80 
81 error[E0452]: malformed lint attribute input
+   --> $DIR/reasons-erroneous.rs:10:31
+   --> $DIR/reasons-erroneous.rs:10:31
+    |
+ LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
+ 
+ error[E0452]: malformed lint attribute input
83    |
83    |
84 LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
133    |                                                          ^ reason must be a string literal
134 
135 error[E0452]: malformed lint attribute input
+   --> $DIR/reasons-erroneous.rs:3:49
+   --> $DIR/reasons-erroneous.rs:3:49
+    |
+ LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
+    |                                                 ^^^^^^^^^^ reason in lint attribute must come last
+ 
+ error[E0452]: malformed lint attribute input
137    |
137    |
138 LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
139    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason must be a string literal
140 
141 error[E0452]: malformed lint attribute input
+   --> $DIR/reasons-erroneous.rs:10:31
+   --> $DIR/reasons-erroneous.rs:10:31
+    |
+ LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
+    |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
+ 
+ error[E0452]: malformed lint attribute input
143    |
143    |
144 LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]

186 LL | #![warn(keyword_idents, reason = "root in rubble", macro_use_extern_crate)]
187    |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
- error: aborting due to 30 previous errors; 1 warning emitted
+ error: aborting due to 36 previous errors; 1 warning emitted
190 
191 For more information about this error, try `rustc --explain E0452`.
---
To only update this specific test, also pass `--test-args lint/reasons-erroneous.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/reasons-erroneous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-erroneous" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/reasons-erroneous/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:3:58
   |
LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
   |                                                          ^ reason must be a string literal
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:3:49
   |
   |
LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
   |                                                 ^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:10:40
   |
   |
LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason must be a string literal
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:10:31
   |
   |
LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:17:29
   |
   |
LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:17:29
   |
   |
LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:30:23
   |
   |
LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:30:23
   |
   |
LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:43:36
   |
   |
LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:43:36
   |
   |
LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:56:44
   |
   |
LL | #![warn(ellipsis_inclusive_range_patterns, reason = "born barren", reason = "a freak growth")]
   |                                            ^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:63:25
   |
   |
LL | #![warn(keyword_idents, reason = "root in rubble", macro_use_extern_crate)]
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
warning: unknown lint: `reason`
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:70:39
   |
   |
LL | #![warn(missing_copy_implementations, reason)]
   |
   = note: `#[warn(unknown_lints)]` on by default

error[E0452]: malformed lint attribute input
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:3:58
   |
LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
   |                                                          ^ reason must be a string literal
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:3:49
   |
   |
LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
   |                                                 ^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:10:40
   |
   |
LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason must be a string literal
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:10:31
   |
   |
LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:17:29
   |
   |
LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:17:29
   |
   |
LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:30:23
   |
   |
LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:30:23
   |
   |
LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:43:36
   |
   |
LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:43:36
   |
   |
LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:56:44
   |
   |
LL | #![warn(ellipsis_inclusive_range_patterns, reason = "born barren", reason = "a freak growth")]
   |                                            ^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:63:25
   |
   |
LL | #![warn(keyword_idents, reason = "root in rubble", macro_use_extern_crate)]
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:3:58
   |
   |
LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
   |                                                          ^ reason must be a string literal
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:3:49
   |
   |
LL | #![warn(absolute_paths_not_starting_with_crate, reason = 0)]
   |                                                 ^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:10:40
   |
   |
LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason must be a string literal
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:10:31
   |
   |
LL | #![warn(anonymous_parameters, reason = b"consider these, for we have condemned them")]
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:17:29
   |
   |
LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:17:29
   |
   |
LL | #![warn(bare_trait_objects, reasons = "leaders to no sure land, guides their bearings lost")]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:30:23
   |
   |
LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:30:23
   |
   |
LL | #![warn(box_pointers, blerp = "or in league with robbers have reversed the signposts")]
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:43:36
   |
   |
LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:43:36
   |
   |
LL | #![warn(elided_lifetimes_in_paths, reason("disrespectful to ancestors", "irresponsible to heirs"))]
   |                                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ bad attribute argument
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:56:44
   |
   |
LL | #![warn(ellipsis_inclusive_range_patterns, reason = "born barren", reason = "a freak growth")]
   |                                            ^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
error[E0452]: malformed lint attribute input
  --> /checkout/src/test/ui/lint/reasons-erroneous.rs:63:25
   |
   |
LL | #![warn(keyword_idents, reason = "root in rubble", macro_use_extern_crate)]
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^ reason in lint attribute must come last
error: aborting due to 36 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0452`.

---
test result: FAILED. 11868 passed; 1 failed; 98 ignored; 0 measured; 0 filtered out; finished in 122.75s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:56
