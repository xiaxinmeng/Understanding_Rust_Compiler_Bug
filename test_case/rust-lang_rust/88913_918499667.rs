plain
   Compiling opaque-debug v0.3.0
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
   Compiling cpuid-bool v0.1.2
   Compiling unicode-width v0.1.8
   Compiling unic-common v0.9.0
   Compiling unic-char-range v0.9.0
   Compiling termcolor v1.1.2
   Compiling serde_derive v1.0.125
   Compiling serde v1.0.125
   Compiling annotate-snippets v0.8.0
---
   Compiling sharded-slab v0.1.1
   Compiling thread_local v1.0.1
   Compiling itertools v0.9.0
   Compiling getopts v0.2.21
   Compiling unic-ucd-version v0.9.0
   Compiling unic-char-property v0.9.0
   Compiling crossbeam-utils v0.7.2
   Compiling memoffset v0.5.5
   Compiling crossbeam-epoch v0.8.2
   Compiling num-traits v0.2.12
   Compiling num-traits v0.2.12
   Compiling num-integer v0.1.43
   Compiling generic-array v0.14.4
   Compiling unicode-normalization v0.1.13
   Compiling unic-emoji-char v0.9.0
   Compiling stacker v0.1.14
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
   Compiling ena v0.14.0
---
   Compiling stable_deref_trait v1.2.0
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
   Compiling cpuid-bool v0.1.2
   Compiling unicode-width v0.1.8
   Compiling unic-char-range v0.9.0
   Compiling unic-common v0.9.0
   Compiling serde_derive v1.0.125
   Compiling termcolor v1.1.2
   Compiling serde v1.0.125
   Compiling annotate-snippets v0.8.0
---
   Compiling tracing-core v0.1.17
   Compiling sharded-slab v0.1.1
   Compiling thread_local v1.0.1
   Compiling itertools v0.9.0
   Compiling unic-ucd-version v0.9.0
   Compiling getopts v0.2.21
   Compiling unic-char-property v0.9.0
   Compiling crossbeam-utils v0.7.2
   Compiling memoffset v0.5.5
   Compiling crossbeam-epoch v0.8.2
   Compiling num-traits v0.2.12
   Compiling num-traits v0.2.12
   Compiling num-integer v0.1.43
   Compiling generic-array v0.14.4
   Compiling unicode-normalization v0.1.13
   Compiling unic-emoji-char v0.9.0
   Compiling psm v0.1.16
   Compiling stacker v0.1.14
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
   Compiling ena v0.14.0
---
....................i.i............................................................................. 12100/12132
................................
failures:

---- [ui] ui/parser/emoji-identifiers.rs stdout ----


18 LL |     let _ = i_like_to_😄_a_lot() ➖ 4;
19    |             ^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `i_like_to_😅_a_lot`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- error: identifiers cannot contain emoji: `i_like_to_😄_a_lot`
-   --> $DIR/emoji-identifiers.rs:13:13
-    |
- LL |     let _ = i_like_to_😄_a_lot() ➖ 4;
- 
- 
- error: identifiers cannot contain emoji: `full_of_✨`
-   --> $DIR/emoji-identifiers.rs:4:8
-    |
- LL |     fn full_of_✨() -> 👀 {
- 
- 
33 error: identifiers cannot contain emoji: `full_of✨`
34   --> $DIR/emoji-identifiers.rs:9:8

64    |
64    |
65 LL | struct ABig👩👩👧👧Family;
+ 
+ 
+ error: identifiers cannot contain emoji: `i_like_to_😄_a_lot`
+   --> $DIR/emoji-identifiers.rs:13:13
+    |
+ LL |     let _ = i_like_to_😄_a_lot() ➖ 4;
+ 
+ 
+ error: identifiers cannot contain emoji: `full_of_✨`
+   --> $DIR/emoji-identifiers.rs:4:8
+    |
+ LL |     fn full_of_✨() -> 👀 {
67 
67 
68 error[E0599]: no function or associated item named `full_of✨` found for struct `👀` in the current scope
69   --> $DIR/emoji-identifiers.rs:9:8

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers/emoji-identifiers.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/emoji-identifiers.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/emoji-identifiers.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/emoji-identifiers/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unknown start of token: \u{2796}
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:13:33
   |
LL |     let _ = i_like_to_😄_a_lot() ➖ 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope
   |
   |
help: Unicode character '➖' (Heavy Minus Sign) looks like '-' (Minus/Hyphen), but it is not
   |
LL |     let _ = i_like_to_😄_a_lot() - 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope


error[E0425]: cannot find function `i_like_to_😄_a_lot` in this scope
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:13:13
   |
LL | fn i_like_to_😅_a_lot() -> 👀 { //~ ERROR identifiers cannot contain emoji
   | ----------------------------- similarly named function `i_like_to_😅_a_lot` defined here
...
LL |     let _ = i_like_to_😄_a_lot() ➖ 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope
   |             ^^^^^^^^^^^^^^^^^^ help: a function with a similar name exists: `i_like_to_😅_a_lot`

error: identifiers cannot contain emoji: `full_of✨`
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:9:8
   |
LL |     👀::full_of✨() //~ ERROR no function or associated item named `full_of✨` found for struct `👀`


error: identifiers cannot contain emoji: `👀`
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:2:8
   |
LL | struct 👀; //~ ERROR identifiers cannot contain emoji
LL | impl 👀 {
   |      ^^
   |      ^^
LL |     fn full_of_✨() -> 👀 { //~ ERROR identifiers cannot contain emoji
LL |         👀
   |         ^^
...
...
LL | fn i_like_to_😅_a_lot() -> 👀 { //~ ERROR identifiers cannot contain emoji
   |                            ^^
LL |     👀::full_of✨() //~ ERROR no function or associated item named `full_of✨` found for struct `👀`


error: identifiers cannot contain emoji: `i_like_to_😅_a_lot`
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:8:4
   |
LL | fn i_like_to_😅_a_lot() -> 👀 { //~ ERROR identifiers cannot contain emoji


error: identifiers cannot contain emoji: `ABig👩👩👧👧Family`
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:1:8
   |
LL | struct ABig👩👩👧👧Family; //~ ERROR identifiers cannot contain emoji


error: identifiers cannot contain emoji: `i_like_to_😄_a_lot`
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:13:13
   |
LL |     let _ = i_like_to_😄_a_lot() ➖ 4; //~ ERROR cannot find function `i_like_to_😄_a_lot` in this scope


error: identifiers cannot contain emoji: `full_of_✨`
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:4:8
   |
LL |     fn full_of_✨() -> 👀 { //~ ERROR identifiers cannot contain emoji


error[E0599]: no function or associated item named `full_of✨` found for struct `👀` in the current scope
  --> /checkout/src/test/ui/parser/emoji-identifiers.rs:9:8
   |
LL | struct 👀; //~ ERROR identifiers cannot contain emoji
   | ---------- function or associated item `full_of✨` not found for this
...
LL |     👀::full_of✨() //~ ERROR no function or associated item named `full_of✨` found for struct `👀`
   |         |
   |         function or associated item not found in `👀`
   |         function or associated item not found in `👀`
   |         help: there is an associated function with a similar name: `full_of_✨`
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0425, E0599.
For more information about an error, try `rustc --explain E0425`.
For more information about an error, try `rustc --explain E0425`.

------------------------------------------



failures:
    [ui] ui/parser/emoji-identifiers.rs
test result: FAILED. 12029 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 130.54s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:30
