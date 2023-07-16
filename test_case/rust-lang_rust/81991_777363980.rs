plain
---- [ui] ui/suggestions/match-prev-arm-needing-semi.rs stdout ----
diff of stderr:

24    |
25 LL |         false => async_dummy().await,
26    |                               ^^^^^^
- help: consider removing this semicolon and boxing the expressions
+ help: consider removing this semicolon
28    |
- LL |             Box::new(async_dummy())
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- LL |
- LL |         }
- LL |         false => Box::new(async_dummy()),
-    |
+ LL |             async_dummy()
34 
34 
35 error[E0308]: `match` arms have incompatible types


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/match-prev-arm-needing-semi.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/match-prev-arm-needing-semi.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/match-prev-arm-needing-semi.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/match-prev-arm-needing-semi/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: `match` arms have incompatible types
   |
   |
LL |   async fn async_dummy() {} //~ NOTE the `Output` of this `async fn`'s found opaque type
   |                          - the `Output` of this `async fn`'s found opaque type
...
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
LL | |         true => {
LL | |         true => {
LL | |             async_dummy(); //~ NOTE this is found to be
   | |             -------------- this is found to be of type `()`
LL | |             //~^ HELP consider removing this semicolon
LL | |         }
LL | |         false => async_dummy(), //~ ERROR `match` arms have incompatible types
   | |                  ^^^^^^^^^^^^^ expected `()`, found opaque type
...  |
LL | |         //~| HELP consider `await`ing on the `Future`
LL | |     };
   | |_____- `match` arms have incompatible types
   = note:     expected type `()`
   = note:     expected type `()`
           found opaque type `impl Future`
help: consider `await`ing on the `Future`
   |
LL |         false => async_dummy().await, //~ ERROR `match` arms have incompatible types
help: consider removing this semicolon
   |
   |
LL |             async_dummy() //~ NOTE this is found to be


error[E0308]: `match` arms have incompatible types
   |
   |
LL |   async fn async_dummy2() {} //~ NOTE the `Output` of this `async fn`'s found opaque type
   |                           - the `Output` of this `async fn`'s found opaque type
...
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
LL | |         true => {
LL | |         true => {
LL | |             async_dummy(); //~ NOTE this is found to be
   | |             -------------- this is found to be of type `()`
LL | |             //~^ HELP consider removing this semicolon
LL | |         }
LL | |         false => async_dummy2(), //~ ERROR `match` arms have incompatible types
   | |                  ^^^^^^^^^^^^^^ expected `()`, found opaque type
...  |
LL | |         //~| HELP consider `await`ing on the `Future`
LL | |     };
   | |_____- `match` arms have incompatible types
   = note:     expected type `()`
   = note:     expected type `()`
           found opaque type `impl Future`
help: consider `await`ing on the `Future`
   |
LL |         false => async_dummy2().await, //~ ERROR `match` arms have incompatible types
   |                                ^^^^^^
help: consider removing this semicolon and boxing the expressions
   |
LL |             Box::new(async_dummy()) //~ NOTE this is found to be
LL |             //~^ HELP consider removing this semicolon
LL |         }
LL |         false => Box::new(async_dummy2()), //~ ERROR `match` arms have incompatible types


error[E0308]: `match` arms have incompatible types
   |
   |
LL |   async fn async_dummy2() {} //~ NOTE the `Output` of this `async fn`'s found opaque type
   |                           - the `Output` of this `async fn`'s found opaque type
...
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
   |  _____________-
LL | |         true => async_dummy(), //~ NOTE this is found to be
   | |                 ------------- this is found to be of type `impl Future`
LL | |         //~| HELP consider `await`ing on both `Future`s
LL | |         false => async_dummy2(), //~ ERROR `match` arms have incompatible types
   | |                  ^^^^^^^^^^^^^^ expected opaque type, found a different opaque type
...  |
LL | |         //~| NOTE distinct uses of `impl Trait` result in different opaque types
LL | |     };
   | |_____- `match` arms have incompatible types
   |
   = note:     expected type `impl Future` (opaque type at </checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:16:24>)
           found opaque type `impl Future` (opaque type at </checkout/src/test/ui/suggestions/match-prev-arm-needing-semi.rs:17:25>)
   = note: distinct uses of `impl Trait` result in different opaque types
help: consider `await`ing on both `Future`s
   |
LL |         true => async_dummy().await, //~ NOTE this is found to be
LL |         //~| HELP consider `await`ing on both `Future`s
LL |         false => async_dummy2().await, //~ ERROR `match` arms have incompatible types


error[E0308]: `match` arms have incompatible types
   |
   |
LL |       let _ = match true { //~ NOTE `match` arms have incompatible types
LL | |         true => {
LL | |         true => {
LL | |             dummy(); //~ NOTE this is found to be
   | |             |      |
   | |             |      help: consider removing this semicolon
   | |             this is found to be of type `()`
   | |             this is found to be of type `()`
LL | |             //~^ HELP consider removing this semicolon
LL | |         }
LL | |         false => dummy(), //~ ERROR `match` arms have incompatible types
   | |                  ^^^^^^^ expected `()`, found `i32`
LL | |         //~^ NOTE expected `()`, found `i32`
LL | |     };
   | |_____- `match` arms have incompatible types
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 11346 passed; 1 failed; 92 ignored; 0 measured; 0 filtered out; finished in 137.65s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:15:17
