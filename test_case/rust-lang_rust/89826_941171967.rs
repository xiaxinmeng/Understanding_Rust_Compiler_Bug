plain
........................................iiiiiiiiiii............i..........ii....ii.................. 200/12275
.................................................................................................... 300/12275
.................................................................................................... 400/12275
.................................................................................................... 500/12275
............................................................................F........F.............. 600/12275
..................................................ii................................................ 800/12275
.................................................................................................... 900/12275
.................................................................................................... 1000/12275
.................................................................................................... 1100/12275
---

---- [ui] ui/async-await/issue-64130-non-send-future-diags.rs stdout ----
diff of stderr:

+ warning: `MutexGuard` held across a suspend point, but should not be
+    |
+    |
+ LL |     let g = x.lock().unwrap();
+    |         ^
+ LL |     baz().await;
+    |     ----------- the value is held across this suspend point
+    |
+    = note: `#[warn(must_not_suspend)]` on by default
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+ note: holding a MutexGuard across suspend points can cause deadlocks, delays, and cause Futures to not implement `Send`
+    |
+    |
+ LL |     let g = x.lock().unwrap();
+    |         ^
+ help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
+    |
+    |
+ LL |     let g = x.lock().unwrap();
+ 
1 error: future cannot be sent between threads safely
2   --> $DIR/issue-64130-non-send-future-diags.rs:21:5
3    |
3    |

20 LL | fn is_send<T: Send>(t: T) { }
21    |               ^^^^ required by this bound in `is_send`
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
24 
25 
---
To only update this specific test, also pass `--test-args async-await/issue-64130-non-send-future-diags.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-64130-non-send-future-diags.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-64130-non-send-future-diags" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-64130-non-send-future-diags/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `MutexGuard` held across a suspend point, but should not be
   |
   |
LL |     let g = x.lock().unwrap();
   |         ^
LL |     baz().await;
   |     ----------- the value is held across this suspend point
   |
   = note: `#[warn(must_not_suspend)]` on by default
note: holding a MutexGuard across suspend points can cause deadlocks, delays, and cause Futures to not implement `Send`
   |
   |
LL |     let g = x.lock().unwrap();
   |         ^
help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
   |
   |
LL |     let g = x.lock().unwrap();

error: future cannot be sent between threads safely
  --> /checkout/src/test/ui/async-await/issue-64130-non-send-future-diags.rs:21:5
   |
   |
LL |     is_send(foo());
   |     ^^^^^^^ future returned by `foo` is not `Send`
   |
   = help: within `impl Future`, the trait `Send` is not implemented for `MutexGuard<'_, u32>`
note: future is not `Send` as this value is used across an await
   |
   |
LL |     let g = x.lock().unwrap();
   |         - has type `MutexGuard<'_, u32>` which is not `Send`
LL |     baz().await;
   |     ^^^^^^^^^^^ await occurs here, with `g` maybe used later
LL | }
   | - `g` is later dropped here
note: required by a bound in `is_send`
   |
   |
LL | fn is_send<T: Send>(t: T) { }
   |               ^^^^ required by this bound in `is_send`
error: aborting due to previous error; 1 warning emitted


------------------------------------------
------------------------------------------


---- [ui] ui/async-await/issue-71137.rs stdout ----
diff of stderr:

+ warning: `MutexGuard` held across a suspend point, but should not be
+    |
+    |
+ LL |     let mut guard = m.lock().unwrap();
+    |         ^^^^^^^^^
+ LL |     (async { "right"; }).await;
+    |     -------------------------- the value is held across this suspend point
+    |
+    = note: `#[warn(must_not_suspend)]` on by default
+ note: holding a MutexGuard across suspend points can cause deadlocks, delays, and cause Futures to not implement `Send`
+    |
+    |
+ LL |     let mut guard = m.lock().unwrap();
+    |         ^^^^^^^^^
+ help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
+    |
+    |
+ LL |     let mut guard = m.lock().unwrap();
+ 
1 error: future cannot be sent between threads safely
2   --> $DIR/issue-71137.rs:20:3
3    |
3    |

21 LL | fn fake_spawn<F: Future + Send + 'static>(f: F) { }
22    |                           ^^^^ required by this bound in `fake_spawn`
- error: aborting due to previous error
+ error: aborting due to previous error; 1 warning emitted
25 
26 
---
To only update this specific test, also pass `--test-args async-await/issue-71137.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issue-71137.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-71137" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issue-71137/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: `MutexGuard` held across a suspend point, but should not be
   |
   |
LL |     let mut guard = m.lock().unwrap();
   |         ^^^^^^^^^
LL |     (async { "right"; }).await;
   |     -------------------------- the value is held across this suspend point
   |
   = note: `#[warn(must_not_suspend)]` on by default
note: holding a MutexGuard across suspend points can cause deadlocks, delays, and cause Futures to not implement `Send`
   |
   |
LL |     let mut guard = m.lock().unwrap();
   |         ^^^^^^^^^
help: consider using a block (`{ ... }`) to shrink the value's scope, ending before the suspend point
   |
   |
LL |     let mut guard = m.lock().unwrap();

error: future cannot be sent between threads safely
  --> /checkout/src/test/ui/async-await/issue-71137.rs:20:3
   |
   |
LL |   fake_spawn(wrong_mutex()); //~ Error future cannot be sent between threads safely
   |   ^^^^^^^^^^ future returned by `wrong_mutex` is not `Send`
   |
   = help: within `impl Future`, the trait `Send` is not implemented for `MutexGuard<'_, i32>`
note: future is not `Send` as this value is used across an await
   |
   |
LL |     let mut guard = m.lock().unwrap();
   |         --------- has type `MutexGuard<'_, i32>` which is not `Send`
LL |     (async { "right"; }).await;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ await occurs here, with `mut guard` maybe used later
LL |     *guard += 1;
LL |   }
   |   - `mut guard` is later dropped here
note: required by a bound in `fake_spawn`
   |
   |
LL | fn fake_spawn<F: Future + Send + 'static>(f: F) { }
   |                           ^^^^ required by this bound in `fake_spawn`
error: aborting due to previous error; 1 warning emitted


------------------------------------------
---
test result: FAILED. 12156 passed; 2 failed; 117 ignored; 0 measured; 0 filtered out; finished in 127.02s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:12:34
