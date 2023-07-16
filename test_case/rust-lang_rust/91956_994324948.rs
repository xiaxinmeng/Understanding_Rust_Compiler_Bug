plain

81    | 
82 
83 "}
- {"message":"unnecessary parentheses around `for` iterator expression","code":{"code":"unused_parens","explanation":null},"level":"error","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":968,"byte_end":969,"line_start":44,"line_end":44,"column_start":18,"column_end":19,"is_primary":true,"text":[{"text":"        for _ in (0 .. 3){
-   --> $DIR/unused_parens_remove_json_suggestion.rs:44:18
-    |
- LL |         for _ in (0 .. 3){
-    |                  ^      ^
- help: remove these parentheses
-    |
-    |
- LL -         for _ in (0 .. 3){
- LL +         for _ in 0 .. 3 {
- 
- "}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- {"message":"unnecessary parentheses around `for` iterator expression","code":{"code":"unused_parens","explanation":null},"level":"error","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":1069,"byte_end":1070,"line_start":49,"line_end":49,"column_start":14,"column_end":15,"is_primary":true,"text":[{"text":"    for _ in (0 .. 3) {
-   --> $DIR/unused_parens_remove_json_suggestion.rs:49:14
-    |
- LL |     for _ in (0 .. 3) {
-    |              ^      ^
- help: remove these parentheses
-    |
-    |
- LL -     for _ in (0 .. 3) {
- LL +     for _ in 0 .. 3 {
- 
- "}
- "}
110 {"message":"unnecessary parentheses around `while` condition","code":{"code":"unused_parens","explanation":null},"level":"error","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":1128,"byte_end":1129,"line_start":50,"line_end":50,"column_start":15,"column_end":16,"is_primary":true,"text":[{"text":"        while (true && false) {
111   --> $DIR/unused_parens_remove_json_suggestion.rs:50:15

120    | 
121 
122 "}
---
diff of fixed:

41     }
42 
43     while true && false { //~ ERROR unnecessary parentheses
-         for _ in 0 .. 3 { //~ ERROR unnecessary parentheses
+         for _ in (0 .. 3){ //~ ERROR unnecessary parentheses
45             println!("e~")
47     }

48 
48 
-     for _ in 0 .. 3 { //~ ERROR unnecessary parentheses
+     for _ in (0 .. 3) { //~ ERROR unnecessary parentheses
50         while true && false { //~ ERROR unnecessary parentheses
51             println!("e~")


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_remove_json_suggestion/unused_parens_remove_json_suggestion.fixed
To only update this specific test, also pass `--test-args lint/unused_parens_remove_json_suggestion.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused_parens_remove_json_suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_remove_json_suggestion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "json" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_remove_json_suggestion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unnecessary parentheses around `if` condition
  --> /checkout/src/test/ui/lint/unused_parens_remove_json_suggestion.rs:17:8
   |
LL |     if (_b) { //~ ERROR unnecessary parentheses
   |        ^  ^
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused_parens_remove_json_suggestion.rs:10:9
   |
   |
LL | #![deny(unused_parens)]
help: remove these parentheses
   |
   |
LL -     if (_b) { //~ ERROR unnecessary parentheses
LL +     if _b { //~ ERROR unnecessary parentheses

error: unnecessary parentheses around `if` condition
  --> /checkout/src/test/ui/lint/unused_parens_remove_json_suggestion.rs:28:7
   |
   |
LL |     if(c) { //~ ERROR unnecessary parentheses
   |       ^ ^
help: remove these parentheses
   |
   |
LL -     if(c) { //~ ERROR unnecessary parentheses
LL +     if c { //~ ERROR unnecessary parentheses

error: unnecessary parentheses around `if` condition
  --> /checkout/src/test/ui/lint/unused_parens_remove_json_suggestion.rs:32:8
   |
   |
LL |     if (c){ //~ ERROR unnecessary parentheses
   |        ^ ^
help: remove these parentheses
   |
   |
LL -     if (c){ //~ ERROR unnecessary parentheses
LL +     if c { //~ ERROR unnecessary parentheses


error: unnecessary parentheses around `while` condition
   |
LL |     while (false && true){
   |           ^             ^
   |
---

error: unnecessary parentheses around `if` condition
  --> /checkout/src/test/ui/lint/unused_parens_remove_json_suggestion.rs:37:12
   |
LL |         if (c) { //~ ERROR unnecessary parentheses
   |            ^ ^
help: remove these parentheses
   |
   |
LL -         if (c) { //~ ERROR unnecessary parentheses
LL +         if c { //~ ERROR unnecessary parentheses


error: unnecessary parentheses around `while` condition
   |
   |
LL |     while(true && false) { //~ ERROR unnecessary parentheses
   |          ^             ^
help: remove these parentheses
   |
   |
LL -     while(true && false) { //~ ERROR unnecessary parentheses
LL +     while true && false { //~ ERROR unnecessary parentheses


error: unnecessary parentheses around `while` condition
   |
   |
LL |         while (true && false) { //~ ERROR unnecessary parentheses
   |               ^             ^
help: remove these parentheses
   |
   |
LL -         while (true && false) { //~ ERROR unnecessary parentheses
LL +         while true && false { //~ ERROR unnecessary parentheses

error: aborting due to 7 previous errors


---
test result: FAILED. 12366 passed; 1 failed; 119 ignored; 0 measured; 0 filtered out; finished in 142.79s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:49
