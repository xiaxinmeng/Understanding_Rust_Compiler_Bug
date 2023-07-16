plain
.................................................................................................... 9200/11472
.................................................................................................... 9300/11472
.................................................................................................... 9400/11472
.............................i.......i.............................................................. 9500/11472
....................................................................iiiiiii..iiiiii.i............... 9600/11472
.................................................................................................... 9800/11472
.................................................................................................... 9900/11472
.................................................................................................... 10000/11472
.................................................................................................... 10100/11472
---
diff of stderr:

13   --> $DIR/generator-desc.rs:12:16
14    |
15 LL | async fn one() {}
-    |                - the `Output` of this `async fn`'s expected opaque type
+    |                - checked the `Output` of this `async fn`, expected opaque type
17 LL | async fn two() {}
-    |                - the `Output` of this `async fn`'s found opaque type
+    |                - checked the `Output` of this `async fn`, found opaque type
19 ...
20 LL |     fun(one(), two());
21    |                ^^^^^ expected opaque type, found a different opaque type
22    |
22    |
+    = note: while checking the return type of the `async fn`
+    = note: while checking the return type of the `async fn`
23    = note: expected opaque type `impl Future` (opaque type at <$DIR/generator-desc.rs:5:16>)
24               found opaque type `impl Future` (opaque type at <$DIR/generator-desc.rs:6:16>)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
25    = help: consider `await`ing on both `Future`s

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/generator-desc.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/generator-desc.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/generator-desc.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/generator-desc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/generator-desc/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:10:25
   |
LL |     fun(async {}, async {});
   |               --        ^^ expected `async` block, found a different `async` block
   |               |
   |               the expected `async` block
   |
   = note: expected `async` block `[static generator@/checkout/src/test/ui/async-await/generator-desc.rs:10:15: 10:17]`
              found `async` block `[static generator@/checkout/src/test/ui/async-await/generator-desc.rs:10:25: 10:27]`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:12:16
   |
   |
LL | async fn one() {}
   |                - checked the `Output` of this `async fn`, expected opaque type
LL | async fn two() {}
   |                - checked the `Output` of this `async fn`, found opaque type
...
LL |     fun(one(), two());
   |                ^^^^^ expected opaque type, found a different opaque type
   |
   = note: while checking the return type of the `async fn`
   = note: while checking the return type of the `async fn`
   = note: expected opaque type `impl Future` (opaque type at </checkout/src/test/ui/async-await/generator-desc.rs:5:16>)
              found opaque type `impl Future` (opaque type at </checkout/src/test/ui/async-await/generator-desc.rs:6:16>)
   = help: consider `await`ing on both `Future`s
   = note: distinct uses of `impl Trait` result in different opaque types
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/generator-desc.rs:14:26
   |
   |
LL |     fun((async || {})(), (async || {})());
   |                   --     ^^^^^^^^^^^^^^^ expected `async` closure body, found a different `async` closure body
   |                   |
   |                   the expected `async` closure body
  ::: /checkout/library/core/src/future/mod.rs:61:43
   |
   |
LL | pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
   |                                           |
   |                                           the expected opaque type
   |                                           the found opaque type
   |
   |
   = note: expected opaque type `impl Future` (`async` closure body)
              found opaque type `impl Future` (`async` closure body)
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/suggestions/issue-81839.rs stdout ----
diff of stderr:

17   ::: $DIR/auxiliary/issue-81839.rs:6:49
18    |
19 LL |       pub async fn answer_str(&self, _s: &str) -> Test {
-    |                                                   ---- the `Output` of this `async fn`'s found opaque type
+    |                                                   ---- checked the `Output` of this `async fn`, found opaque type
21    |
+    = note: while checking the return type of the `async fn`
22    = note:     expected type `()`
23            found opaque type `impl Future`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-81839/issue-81839.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-81839/issue-81839.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/issue-81839.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/issue-81839.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-81839" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/issue-81839/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: `match` arms have incompatible types
   |
   |
LL | /     match num {
LL | |         1 => {
LL | |             cx.answer_str("hi");
   | |             |                  |
   | |             |                  help: consider removing this semicolon
   | |             this is found to be of type `()`
LL | |         }
LL | |         }
LL | |         _ => cx.answer_str("hi"), //~ `match` arms have incompatible types
   | |              ^^^^^^^^^^^^^^^^^^^ expected `()`, found opaque type
LL | |     }
   | |_____- `match` arms have incompatible types
  ::: /checkout/src/test/ui/suggestions/auxiliary/issue-81839.rs:6:49
   |
   |
LL |       pub async fn answer_str(&self, _s: &str) -> Test {
   |                                                   ---- checked the `Output` of this `async fn`, found opaque type
   |
   = note: while checking the return type of the `async fn`
   = note:     expected type `()`
           found opaque type `impl Future`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 11377 passed; 2 failed; 93 ignored; 0 measured; 0 filtered out; finished in 139.28s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:56
