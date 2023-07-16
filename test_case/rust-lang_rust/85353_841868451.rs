plain
.................................................................................................... 3900/11878
.................................................................................................... 4000/11878
.................................................................................................... 4100/11878
.................................................................................................... 4200/11878
...........................................................................................FF..F.... 4300/11878
................................................................................................ii.. 4500/11878
.....................................................................................i.............. 4600/11878
.................................................................................................... 4700/11878
.................................................................................................... 4800/11878
---
.................................................................................................... 9500/11878
.................................................................................................... 9600/11878
...............................................................................i......i............. 9700/11878
.................................................................................................... 9800/11878
..........................iiiiiiiiiiiii.i........................................................... 9900/11878
.................................................................................................... 10100/11878
.................................................................................................... 10200/11878
.................................................................................................... 10300/11878
.................................................................................................... 10400/11878
---

---- [ui] ui/consts/async-block.rs#without_feature stdout ----
diff of stderr:

4 LL | const _: i32 = { core::mem::ManuallyDrop::new(async { 0 }); 4 };
6    |
+    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
+    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
7    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
8 
9 error[E0658]: `async` blocks are not allowed in statics

12 LL | static _FUT: &(dyn Future<Output = ()> + Sync) = &async {};
14    |
+    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
15    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
17 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature/async-block.without_feature.stderr
To only update this specific test, also pass `--test-args consts/async-block.rs`


error in revision `without_feature`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/async-block.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "without_feature" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/async-block.without_feature/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `async` blocks are not allowed in constants
   |
   |
LL | const _: i32 = { core::mem::ManuallyDrop::new(async { 0 }); 4 };
   |
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable

error[E0658]: `async` blocks are not allowed in statics
   |
   |
LL | static _FUT: &(dyn Future<Output = ()> + Sync) = &async {};
   |
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.


------------------------------------------


---- [ui] ui/impl-trait/issues/issue-78721.rs stdout ----
diff of stderr:

13 LL |         let f: impl core::future::Future<Output = u8> = async { 1 };
15    |
+    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
+    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
16    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
17 
18 error[E0493]: destructors cannot be evaluated at compile-time

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78721/issue-78721.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/issues/issue-78721.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-78721.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78721" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78721/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `impl_trait_in_bindings` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(impl_trait_in_bindings)]
   |            ^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information

error[E0658]: `async` blocks are not allowed in constants
   |
   |
LL |         let f: impl core::future::Future<Output = u8> = async { 1 };
   |
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable

error[E0493]: destructors cannot be evaluated at compile-time
   |
   |
LL |         let f: impl core::future::Future<Output = u8> = async { 1 };
   |             ^ constants cannot evaluate destructors
LL |     }],
   |     - value is dropped here

error: aborting due to 2 previous errors; 1 warning emitted
---

---- [ui] ui/impl-trait/issues/issue-78722.rs#full_tait stdout ----
diff of stderr:

21 LL |         let f: F = async { 1 };
23    |
+    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
+    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
24    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
25 
26 error[E0493]: destructors cannot be evaluated at compile-time

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722.full_tait/issue-78722.full_tait.stderr
To only update this specific test, also pass `--test-args impl-trait/issues/issue-78722.rs`


error in revision `full_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-78722.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722.full_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722.full_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `type_alias_impl_trait` is incomplete and may not be safe to use and/or cause compiler crashes
   |
   |
LL | #![cfg_attr(full_tait, feature(type_alias_impl_trait))]
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63063 <https://github.com/rust-lang/rust/issues/63063> for more information


warning: the feature `impl_trait_in_bindings` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(impl_trait_in_bindings)]
   |            ^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information

error[E0658]: `async` blocks are not allowed in constants
   |
   |
LL |         let f: F = async { 1 };
   |
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable

error[E0493]: destructors cannot be evaluated at compile-time
   |
   |
LL |         let f: F = async { 1 };
   |             ^ constants cannot evaluate destructors
LL |     }],
   |     - value is dropped here

error: aborting due to 2 previous errors; 2 warnings emitted
---

---- [ui] ui/impl-trait/issues/issue-78722.rs#min_tait stdout ----
diff of stderr:

13 LL |         let f: F = async { 1 };
15    |
+    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
+    = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
16    = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable
17 
18 error[E0493]: destructors cannot be evaluated at compile-time

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722.min_tait/issue-78722.min_tait.stderr
To only update this specific test, also pass `--test-args impl-trait/issues/issue-78722.rs`


error in revision `min_tait`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-78722.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min_tait" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722.min_tait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-78722.min_tait/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: the feature `impl_trait_in_bindings` is incomplete and may not be safe to use and/or cause compiler crashes
   |
LL | #![feature(impl_trait_in_bindings)]
   |            ^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: `#[warn(incomplete_features)]` on by default
   = note: see issue #63065 <https://github.com/rust-lang/rust/issues/63065> for more information

error[E0658]: `async` blocks are not allowed in constants
   |
   |
LL |         let f: F = async { 1 };
   |
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = note: see issue #85368 <https://github.com/rust-lang/rust/issues/85368> for more information
   = help: add `#![feature(const_async_blocks)]` to the crate attributes to enable

error[E0493]: destructors cannot be evaluated at compile-time
   |
   |
LL |         let f: F = async { 1 };
   |             ^ constants cannot evaluate destructors
LL |     }],
   |     - value is dropped here

error: aborting due to 2 previous errors; 1 warning emitted
---
test result: FAILED. 11777 passed; 4 failed; 97 ignored; 0 measured; 0 filtered out; finished in 122.40s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:47
