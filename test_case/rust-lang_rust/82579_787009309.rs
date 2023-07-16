plain
---- [ui] ui/did_you_mean/issue-40396.rs stdout ----
diff of stderr:

69    |
70 LL |         std::collections::HashMap<i128, i128>::new(1, 2);
71    |                                       ^ expected one of 8 possible tokens
-    |
- help: use `::<...>` instead of `<...>` to specify type or const arguments
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |
- LL |         std::collections::HashMap::<i128, i128>::new(1, 2);
77 
78 error[E0308]: mismatched types
79   --> $DIR/issue-40396.rs:13:17


99    |                |
100    |                expected due to this
101 
- error[E0308]: mismatched types
-   --> $DIR/issue-40396.rs:27:21
-    |
- LL |         let x: () = 32;
-    |                --   ^^ expected `()`, found integer
-    |                expected due to this
- 
- error: aborting due to 11 previous errors
+ error: aborting due to 10 previous errors
+ error: aborting due to 10 previous errors
111 
112 For more information about this error, try `rustc --explain E0308`.
113 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40396/issue-40396.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args did_you_mean/issue-40396.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/did_you_mean/issue-40396.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40396" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/did_you_mean/issue-40396/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: comparison operators cannot be chained
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:2:20
   |
LL |     (0..13).collect<Vec<i32>>();
   |                    ^   ^
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
   |
LL |     (0..13).collect::<Vec<i32>>();

error: comparison operators cannot be chained
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:5:8
   |
   |
LL |     Vec<i32>::new();
   |        ^   ^
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
LL |     Vec::<i32>::new();
   |        ^^

error: comparison operators cannot be chained
error: comparison operators cannot be chained
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:8:20
   |
LL |     (0..13).collect<Vec<i32>();
   |                    ^   ^
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
   |
LL |     (0..13).collect::<Vec<i32>();


error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, or an operator, found `,`
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:11:43
   |
LL |     let x = std::collections::HashMap<i128, i128>::new(); //~ ERROR expected one of
   |                                           ^ expected one of 7 possible tokens
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
   |
LL |     let x = std::collections::HashMap::<i128, i128>::new(); //~ ERROR expected one of


error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `,`
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:15:39
   |
LL |         std::collections::HashMap<i128, i128>::new() //~ ERROR expected one of
   |                                       ^ expected one of 8 possible tokens
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
   |
LL |         std::collections::HashMap::<i128, i128>::new() //~ ERROR expected one of


error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `,`
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:20:39
   |
LL |         std::collections::HashMap<i128, i128>::new(); //~ ERROR expected one of
   |                                       ^ expected one of 8 possible tokens
   |
help: use `::<...>` instead of `<...>` to specify type or const arguments
   |
LL |         std::collections::HashMap::<i128, i128>::new(); //~ ERROR expected one of


error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `,`
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:25:39
   |
LL |         std::collections::HashMap<i128, i128>::new(1, 2); //~ ERROR expected one of
   |                                       ^ expected one of 8 possible tokens
error[E0308]: mismatched types
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:13:17
   |
   |
LL |     let x: () = 42; //~ ERROR mismatched types
   |            --   ^^ expected `()`, found integer
   |            expected due to this

error[E0308]: mismatched types
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:18:17
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:18:17
   |
LL |     let x: () = 42; //~ ERROR mismatched types
   |            --   ^^ expected `()`, found integer
   |            expected due to this

error[E0308]: mismatched types
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:22:21
  --> /checkout/src/test/ui/did_you_mean/issue-40396.rs:22:21
   |
LL |         let x: () = 42; //~ ERROR mismatched types
   |                --   ^^ expected `()`, found integer
   |                expected due to this

error: aborting due to 10 previous errors

---
test result: FAILED. 11413 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 132.14s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:58
