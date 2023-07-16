plain
failures:

---- [ui] ui/dyn-keyword/issue-56327-dyn-trait-in-macro-is-okay.rs stdout ----
normalized stderr:
warning: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped
   |
   |
LL |             let _x: Box<dyn Drop>;
...
...
LL | foo!();
   |
   |
   = note: `#[warn(dyn_drop)]` on by default
   = note: this warning originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 1 warning emitted





The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-keyword/issue-56327-dyn-trait-in-macro-is-okay/issue-56327-dyn-trait-in-macro-is-okay.stderr
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args dyn-keyword/issue-56327-dyn-trait-in-macro-is-okay.rs`

error: 1 errors occurred comparing output.
error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/dyn-keyword/issue-56327-dyn-trait-in-macro-is-okay.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-keyword/issue-56327-dyn-trait-in-macro-is-okay" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2015" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/dyn-keyword/issue-56327-dyn-trait-in-macro-is-okay/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped
  --> /checkout/src/test/ui/dyn-keyword/issue-56327-dyn-trait-in-macro-is-okay.rs:17:29
   |
LL |             let _x: Box<dyn Drop>;
...
...
LL | foo!();
   |
   |
   = note: `#[warn(dyn_drop)]` on by default
   = note: this warning originates in the macro `foo` (in Nightly builds, run with -Z macro-backtrace for more info)
warning: 1 warning emitted


------------------------------------------
---
38    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
39    = note: for more information, see issue #56484 <https://github.com/rust-lang/rust/issues/56484>
40 
- warning: 3 warnings emitted
+ warning: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped
+    |
+    |
+ LL | unsafe impl Trait for dyn (::std::ops::Drop) + Send { }
+    |
+    |
+    = note: `#[warn(dyn_drop)]` on by default
+ 
+ warning: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped
+    |
+    |
+ LL | unsafe impl Trait for dyn (::std::ops::Drop) + Sync { }
+ 
+ 
+ warning: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped
+    |
+    |
+ LL | unsafe impl Trait for dyn (::std::ops::Drop) + Send + Sync { }
+ 
+ warning: 6 warnings emitted
42 
43 
---
To only update this specific test, also pass `--test-args traits/object/issue-33140-traitobject-crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/object/issue-33140-traitobject-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/issue-33140-traitobject-crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/object/issue-33140-traitobject-crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`: (E0119)
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Send) + Sync { }
   | ------------------------------------------------------ first implementation here
LL | unsafe impl Trait for dyn (::std::marker::Send) + Send + Sync { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
note: the lint level is defined here
  --> /checkout/src/test/ui/traits/object/issue-33140-traitobject-crate.rs:3:9
   |
LL | #![warn(order_dependent_trait_objects)]
LL | #![warn(order_dependent_trait_objects)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #56484 <https://github.com/rust-lang/rust/issues/56484>

warning: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`: (E0119)
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Send) + Send + Sync { }
   | ------------------------------------------------------------- first implementation here
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #56484 <https://github.com/rust-lang/rust/issues/56484>


warning: conflicting implementations of trait `Trait` for type `(dyn std::marker::Send + std::marker::Sync + 'static)`: (E0119)
   |
   |
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send { }
   | ------------------------------------------------------ first implementation here
...
LL | unsafe impl Trait for dyn (::std::marker::Sync) + Send + Sync { }
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ conflicting implementation for `(dyn std::marker::Send + std::marker::Sync + 'static)`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #56484 <https://github.com/rust-lang/rust/issues/56484>


warning: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped
   |
   |
LL | unsafe impl Trait for dyn (::std::ops::Drop) + Send { }
   |
   |
   = note: `#[warn(dyn_drop)]` on by default

warning: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped
   |
   |
LL | unsafe impl Trait for dyn (::std::ops::Drop) + Sync { }


warning: types that do not implement `Drop` can still have drop glue, consider instead using `std::mem::needs_drop` to detect whether a type is trivially dropped
   |
   |
LL | unsafe impl Trait for dyn (::std::ops::Drop) + Send + Sync { }

warning: 6 warnings emitted


---
test result: FAILED. 11945 passed; 2 failed; 97 ignored; 0 measured; 0 filtered out; finished in 99.40s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:09:57
