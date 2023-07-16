plain

---- [ui] ui/inference/inference_unstable.rs stdout ----
diff of stderr:

7    = note: `#[warn(unstable_name_collisions)]` on by default
8    = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
9    = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
-    = note: AssocItem { def_id: DefId(17:4 ~ inference_unstable_itertools[8787]::IpuItertools::ipu_flatten), ident: ipu_flatten#0, kind: Fn, vis: Public, defaultness: Default { has_value: true }, container: TraitContainer(DefId(17:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: true } Some(AssocItem { def_id: DefId(17:4 ~ inference_unstable_itertools[8787]::IpuItertools::ipu_flatten), ident: ipu_flatten#0, kind: Fn, vis: Public, defaultness: Default { has_value: true }, container: TraitContainer(DefId(17:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: true })
+    = note: AssocItem { def_id: DefId(19:4 ~ inference_unstable_itertools[8787]::IpuItertools::ipu_flatten), ident: ipu_flatten#0, kind: Fn, vis: Public, defaultness: Default { has_value: true }, container: TraitContainer(DefId(19:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: true } Some(AssocItem { def_id: DefId(19:4 ~ inference_unstable_itertools[8787]::IpuItertools::ipu_flatten), ident: ipu_flatten#0, kind: Fn, vis: Public, defaultness: Default { has_value: true }, container: TraitContainer(DefId(19:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: true })
11    = help: call with fully qualified syntax `inference_unstable_itertools::IpuItertools::ipu_flatten(...)` to keep using the current method
12    = help: add `#![feature(ipu_flatten)]` to the crate attributes to enable `inference_unstable_iterator::IpuIterator::ipu_flatten`

19    |
19    |
20    = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
21    = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
-    = note: AssocItem { def_id: DefId(17:5 ~ inference_unstable_itertools[8787]::IpuItertools::C), ident: C#0, kind: Const, vis: Public, defaultness: Default { has_value: false }, container: TraitContainer(DefId(17:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: false } Some(AssocItem { def_id: DefId(17:5 ~ inference_unstable_itertools[8787]::IpuItertools::C), ident: C#0, kind: Const, vis: Public, defaultness: Default { has_value: false }, container: TraitContainer(DefId(17:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: false })
+    = note: AssocItem { def_id: DefId(19:5 ~ inference_unstable_itertools[8787]::IpuItertools::C), ident: C#0, kind: Const, vis: Public, defaultness: Default { has_value: false }, container: TraitContainer(DefId(19:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: false } Some(AssocItem { def_id: DefId(19:5 ~ inference_unstable_itertools[8787]::IpuItertools::C), ident: C#0, kind: Const, vis: Public, defaultness: Default { has_value: false }, container: TraitContainer(DefId(19:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: false })
23    = help: add `#![feature(assoc_const_ipu_iter)]` to the crate attributes to enable `inference_unstable_iterator::IpuIterator::C`
25 warning: 2 warnings emitted


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable/inference_unstable.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/inference_unstable.rs`
error: 1 errors occurred comparing output.
status: exit code: 0
status: exit code: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/inference_unstable.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/inference_unstable/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: an associated function with this name may be added to the standard library in the future
  --> /checkout/src/test/ui/inference/inference_unstable.rs:16:20
   |
LL |     assert_eq!('x'.ipu_flatten(), 1);
   |
   = note: `#[warn(unstable_name_collisions)]` on by default
   = note: `#[warn(unstable_name_collisions)]` on by default
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: AssocItem { def_id: DefId(19:4 ~ inference_unstable_itertools[8787]::IpuItertools::ipu_flatten), ident: ipu_flatten#0, kind: Fn, vis: Public, defaultness: Default { has_value: true }, container: TraitContainer(DefId(19:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: true } Some(AssocItem { def_id: DefId(19:4 ~ inference_unstable_itertools[8787]::IpuItertools::ipu_flatten), ident: ipu_flatten#0, kind: Fn, vis: Public, defaultness: Default { has_value: true }, container: TraitContainer(DefId(19:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: true })
   = help: call with fully qualified syntax `inference_unstable_itertools::IpuItertools::ipu_flatten(...)` to keep using the current method
   = help: add `#![feature(ipu_flatten)]` to the crate attributes to enable `inference_unstable_iterator::IpuIterator::ipu_flatten`
warning: an associated constant with this name may be added to the standard library in the future
  --> /checkout/src/test/ui/inference/inference_unstable.rs:19:16
   |
   |
LL |     assert_eq!(char::C, 1);
   |                ^^^^^^^ help: use the fully qualified path to the associated const: `<char as IpuItertools>::C`
   |
   = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = note: AssocItem { def_id: DefId(19:5 ~ inference_unstable_itertools[8787]::IpuItertools::C), ident: C#0, kind: Const, vis: Public, defaultness: Default { has_value: false }, container: TraitContainer(DefId(19:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: false } Some(AssocItem { def_id: DefId(19:5 ~ inference_unstable_itertools[8787]::IpuItertools::C), ident: C#0, kind: Const, vis: Public, defaultness: Default { has_value: false }, container: TraitContainer(DefId(19:3 ~ inference_unstable_itertools[8787]::IpuItertools)), fn_has_self_parameter: false })
   = help: add `#![feature(assoc_const_ipu_iter)]` to the crate attributes to enable `inference_unstable_iterator::IpuIterator::C`
warning: 2 warnings emitted


------------------------------------------
---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:34
