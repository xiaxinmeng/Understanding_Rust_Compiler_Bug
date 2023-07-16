plain
.................................................................................................... 6200/11532
.............................................ii.ii.......i...i...................................... 6300/11532
...................................................................................i....i........... 6400/11532
....................i..........................i.................................................... 6500/11532
....................................................................i..F............................ 6600/11532
.....................F.............................................................................. 6700/11532
.....i.............................................................................................. 6900/11532
.................................................................................................... 7000/11532
.......................................................................................i............ 7100/11532
.......................................................................................ii........... 7200/11532
---
.................................................................................................... 9300/11532
.................................................................................................... 9400/11532
.................................................................................i......i........... 9500/11532
.................................................................................................... 9600/11532
....................iiiiiii..iiiiii.i............................................................... 9700/11532
.................................................................................................... 9900/11532
.................................................................................................... 10000/11532
.................................................................................................... 10100/11532
.................................................................................................... 10200/11532
---

---- [ui] ui/lint/lint-unnecessary-parens.rs stdout ----
diff of stderr:

52 LL |     bar((true));
53    |         ^^^^^^ help: remove these parentheses
54 
- error: unnecessary parentheses around `if` condition
-   --> $DIR/lint-unnecessary-parens.rs:51:8
- LL |     if (true) {}
-    |        ^^^^^^ help: remove these parentheses
- 
- 
61 error: unnecessary parentheses around `while` condition
63    |


106 LL |     _a += (1);
107    |           ^^^ help: remove these parentheses
- error: aborting due to 17 previous errors
+ error: aborting due to 16 previous errors
110 
111 
111 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unnecessary-parens/lint-unnecessary-parens.stderr
diff of fixed:

48     foo();
49     bar(true); //~ ERROR unnecessary parentheses around function argument
50 
-     if true {} //~ ERROR unnecessary parentheses around `if` condition
+     if (true) {} //~ ERROR unnecessary parentheses around `if` condition
52     while true {} //~ ERROR unnecessary parentheses around `while` condition
53     match true { //~ ERROR unnecessary parentheses around `match` scrutinee expression


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unnecessary-parens/lint-unnecessary-parens.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-unnecessary-parens.rs`
error: 2 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unnecessary-parens.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unnecessary-parens" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unnecessary-parens/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unnecessary parentheses around `return` value
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:13:12
   |
LL |     return (1); //~ ERROR unnecessary parentheses around `return` value
   |            ^^^ help: remove these parentheses
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:3:9
   |
   |
LL | #![deny(unused_parens)]

error: unnecessary parentheses around `return` value
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:16:12
   |
   |
LL |     return (X { y }); //~ ERROR unnecessary parentheses around `return` value
   |            ^^^^^^^^^ help: remove these parentheses
error: unnecessary parentheses around type
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:19:46
   |
   |
LL | pub fn unused_parens_around_return_type() -> (u32) { //~ ERROR unnecessary parentheses around type
   |                                              ^^^^^ help: remove these parentheses
error: unnecessary parentheses around block return value
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:25:9
   |
   |
LL |         (5) //~ ERROR unnecessary parentheses around block return value
   |         ^^^ help: remove these parentheses
error: unnecessary parentheses around block return value
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:27:5
   |
   |
LL |     (5) //~ ERROR unnecessary parentheses around block return value
   |     ^^^ help: remove these parentheses
error: unnecessary parentheses around assigned value
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:44:31
   |
   |
LL | pub const CONST_ITEM: usize = (10); //~ ERROR unnecessary parentheses around assigned value
   |                               ^^^^ help: remove these parentheses
error: unnecessary parentheses around assigned value
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:45:33
   |
   |
LL | pub static STATIC_ITEM: usize = (10); //~ ERROR unnecessary parentheses around assigned value
   |                                 ^^^^ help: remove these parentheses
error: unnecessary parentheses around function argument
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:49:9
   |
   |
LL |     bar((true)); //~ ERROR unnecessary parentheses around function argument
   |         ^^^^^^ help: remove these parentheses

error: unnecessary parentheses around `while` condition
   |
   |
LL |     while (true) {} //~ ERROR unnecessary parentheses around `while` condition
   |           ^^^^^^ help: remove these parentheses

error: unnecessary parentheses around `match` scrutinee expression
   |
   |
LL |     match (true) { //~ ERROR unnecessary parentheses around `match` scrutinee expression
   |           ^^^^^^ help: remove these parentheses

error: unnecessary parentheses around `let` scrutinee expression
   |
   |
LL |     if let 1 = (1) {} //~ ERROR unnecessary parentheses around `let` scrutinee expression
   |                ^^^ help: remove these parentheses

error: unnecessary parentheses around `let` scrutinee expression
   |
   |
LL |     while let 1 = (2) {} //~ ERROR unnecessary parentheses around `let` scrutinee expression
   |                   ^^^ help: remove these parentheses
error: unnecessary parentheses around method argument
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:73:24
   |
   |
LL |     X { y: false }.foo((true)); //~ ERROR unnecessary parentheses around method argument
   |                        ^^^^^^ help: remove these parentheses
error: unnecessary parentheses around assigned value
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:75:18
   |
   |
LL |     let mut _a = (0); //~ ERROR unnecessary parentheses around assigned value
   |                  ^^^ help: remove these parentheses
error: unnecessary parentheses around assigned value
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:76:10
   |
   |
LL |     _a = (0); //~ ERROR unnecessary parentheses around assigned value
   |          ^^^ help: remove these parentheses
error: unnecessary parentheses around assigned value
  --> /checkout/src/test/ui/lint/lint-unnecessary-parens.rs:77:11
   |
   |
LL |     _a += (1); //~ ERROR unnecessary parentheses around assigned value
   |           ^^^ help: remove these parentheses
error: aborting due to 16 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/lint/unused_parens_remove_json_suggestion.rs stdout ----
diff of stderr:

- {"message":"unnecessary parentheses around `if` condition","code":{"code":"unused_parens","explanation":null},"level":"error","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":500,"byte_end":504,"line_start":17,"line_end":17,"column_start":8,"column_end":12,"is_primary":true,"text":[{"text":"    if (_b) {
-   --> $DIR/unused_parens_remove_json_suggestion.rs:17:8
+ {"message":"unnecessary parentheses around `while` condition","code":{"code":"unused_parens","explanation":null},"level":"error","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":793,"byte_end":808,"line_start":36,"line_end":36,"column_start":11,"column_end":26,"is_primary":true,"text":[{"text":"    while (false && true){","highlight_start":11,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"the lint level is defined here","code":null,"level":"note","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":414,"byte_end":427,"line_start":10,"line_end":10,"column_start":9,"column_end":22,"is_primary":true,"text":[{"text":"#![deny(unused_parens)]","highlight_start":9,"highlight_end":22}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"remove these parentheses","code":null,"level":"help","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":793,"byte_end":808,"line_start":36,"line_end":36,"column_start":11,"column_end":26,"is_primary":true,"text":[{"text":"    while (false && true){","highlight_start":11,"highlight_end":26}],"label":null,"suggested_replacement":"false && true ","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary parentheses around `while` condition
+   --> $DIR/unused_parens_remove_json_suggestion.rs:36:11
3    |
- LL |     if (_b) {
-    |        ^^^^ help: remove these parentheses
+ LL |     while (false && true){
+    |           ^^^^^^^^^^^^^^^ help: remove these parentheses
7 note: the lint level is defined here
8   --> $DIR/unused_parens_remove_json_suggestion.rs:10:9

11    |         ^^^^^^^^^^^^^
11    |         ^^^^^^^^^^^^^
12 
13 "}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- {"message":"unnecessary parentheses around `if` condition","code":{"code":"unused_parens","explanation":null},"level":"error","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":631,"byte_end":634,"line_start":28,"line_end":28,"column_start":7,"column_end":10,"is_primary":true,"text":[{"text":"    if(c) {
-   --> $DIR/unused_parens_remove_json_suggestion.rs:28:7
-    |
- LL |     if(c) {
-    |       ^^^ help: remove these parentheses
- "}
- "}
- {"message":"unnecessary parentheses around `if` condition","code":{"code":"unused_parens","explanation":null},"level":"error","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":711,"byte_end":714,"line_start":32,"line_end":32,"column_start":8,"column_end":11,"is_primary":true,"text":[{"text":"    if (c){
-   --> $DIR/unused_parens_remove_json_suggestion.rs:32:8
- LL |     if (c){
-    |        ^^^ help: remove these parentheses
- 
- "}
- "}
- {"message":"unnecessary parentheses around `while` condition","code":{"code":"unused_parens","explanation":null},"level":"error","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":793,"byte_end":808,"line_start":36,"line_end":36,"column_start":11,"column_end":26,"is_primary":true,"text":[{"text":"    while (false && true){","highlight_start":11,"highlight_end":26}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"remove these parentheses","code":null,"level":"help","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":793,"byte_end":808,"line_start":36,"line_end":36,"column_start":11,"column_end":26,"is_primary":true,"text":[{"text":"    while (false && true){","highlight_start":11,"highlight_end":26}],"label":null,"suggested_replacement":"false && true ","suggestion_applicability":"MachineApplicable","expansion":null}],"children":[],"rendered":null}],"rendered":"error: unnecessary parentheses around `while` condition
-   --> $DIR/unused_parens_remove_json_suggestion.rs:36:11
- LL |     while (false && true){
-    |           ^^^^^^^^^^^^^^^ help: remove these parentheses
- 
- "}
- "}
- {"message":"unnecessary parentheses around `if` condition","code":{"code":"unused_parens","explanation":null},"level":"error","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":821,"byte_end":824,"line_start":37,"line_end":37,"column_start":12,"column_end":15,"is_primary":true,"text":[{"text":"        if (c) {
-   --> $DIR/unused_parens_remove_json_suggestion.rs:37:12
- LL |         if (c) {
-    |            ^^^ help: remove these parentheses
- 
- "}
- "}
42 {"message":"unnecessary parentheses around `while` condition","code":{"code":"unused_parens","explanation":null},"level":"error","spans":[{"file_name":"$DIR/unused_parens_remove_json_suggestion.rs","byte_start":918,"byte_end":933,"line_start":43,"line_end":43,"column_start":10,"column_end":25,"is_primary":true,"text":[{"text":"    while(true && false) {
43   --> $DIR/unused_parens_remove_json_suggestion.rs:43:10

67    |               ^^^^^^^^^^^^^^^ help: remove these parentheses
68 
69 "}
---
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_remove_json_suggestion/unused_parens_remove_json_suggestion.stderr
diff of fixed:

14 
15     let _b = false;
16 
-     if _b { //~ ERROR unnecessary parentheses
+     if (_b) { //~ ERROR unnecessary parentheses
18         println!("hello");
20 

25 fn f() -> bool {
26     let c = false;
26     let c = false;
27 
-     if c { //~ ERROR unnecessary parentheses
+     if(c) { //~ ERROR unnecessary parentheses
29         println!("next");
31 


-     if c { //~ ERROR unnecessary parentheses
+     if (c){ //~ ERROR unnecessary parentheses
33         println!("prev");
35 

36     while false && true {
36     while false && true {
-         if c { //~ ERROR unnecessary parentheses
+         if (c) { //~ ERROR unnecessary parentheses
38             println!("norm");
40 


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_remove_json_suggestion/unused_parens_remove_json_suggestion.fixed
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/unused_parens_remove_json_suggestion.rs`
error: 2 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/unused_parens_remove_json_suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_remove_json_suggestion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--error-format" "json" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/unused_parens_remove_json_suggestion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: unnecessary parentheses around `while` condition
   |
LL |     while (false && true){
   |           ^^^^^^^^^^^^^^^ help: remove these parentheses
   |
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/unused_parens_remove_json_suggestion.rs:10:9
   |
LL | #![deny(unused_parens)]


error: unnecessary parentheses around `while` condition
   |
   |
LL |     while(true && false) { //~ ERROR unnecessary parentheses
   |          ^^^^^^^^^^^^^^^ help: remove these parentheses

error: unnecessary parentheses around `for` iterator expression
   |
   |
LL |         for _ in (0 .. 3){ //~ ERROR unnecessary parentheses
   |                  ^^^^^^^^ help: remove these parentheses

error: unnecessary parentheses around `for` iterator expression
   |
   |
LL |     for _ in (0 .. 3) { //~ ERROR unnecessary parentheses
   |              ^^^^^^^^ help: remove these parentheses

error: unnecessary parentheses around `while` condition
   |
   |
LL |         while (true && false) { //~ ERROR unnecessary parentheses
   |               ^^^^^^^^^^^^^^^ help: remove these parentheses
error: aborting due to 5 previous errors


------------------------------------------
---
test result: FAILED. 11437 passed; 2 failed; 93 ignored; 0 measured; 0 filtered out; finished in 131.15s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:27
