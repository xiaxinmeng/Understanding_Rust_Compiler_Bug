plain
diff of stderr:

83 
84 For more information about this error, try `rustc --explain E0412`.
85 Future incompatibility report: Future breakage diagnostic:
- warning: cannot find type `FromOutside` in this scope
+ error: cannot find type `FromOutside` in this scope
88    |
88    |
89 LL | #[derive(generate_mod::CheckDerive)]
90    |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
91    |
-    = note: `#[warn(proc_macro_derive_resolution_fallback)]` on by default
-    = note: `#[warn(proc_macro_derive_resolution_fallback)]` on by default
+    = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
94    = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
94    = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
-    = note: this warning originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
97 Future breakage diagnostic:
97 Future breakage diagnostic:
- warning: cannot find type `OuterDerive` in this scope
+ error: cannot find type `OuterDerive` in this scope
100    |
100    |
101 LL | #[derive(generate_mod::CheckDerive)]
103    |
104    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
105    = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
105    = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
-    = note: this warning originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
108 Future breakage diagnostic:
108 Future breakage diagnostic:
- warning: cannot find type `FromOutside` in this scope
+ error: cannot find type `FromOutside` in this scope
111    |
111    |
112 LL |     #[derive(generate_mod::CheckDerive)]
114    |
115    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
116    = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
116    = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
-    = note: this warning originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
119 Future breakage diagnostic:
119 Future breakage diagnostic:
- warning: cannot find type `OuterDerive` in this scope
+ error: cannot find type `OuterDerive` in this scope
122    |
122    |
123 LL |     #[derive(generate_mod::CheckDerive)]
125    |
126    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
127    = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
127    = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
-    = note: this warning originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
130 Future breakage diagnostic:
130 Future breakage diagnostic:
131 warning: cannot find type `FromOutside` in this scope

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod/generate-mod.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args proc-macro/generate-mod.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/generate-mod.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/generate-mod/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0412]: cannot find type `FromOutside` in this scope
   |
   |
LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
   |
   = note: consider importing this struct:
           FromOutside
           FromOutside
   = note: this error originates in the macro `generate_mod::check` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0412]: cannot find type `Outer` in this scope
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:9:1
   |
   |
LL | generate_mod::check!(); //~ ERROR cannot find type `FromOutside` in this scope
   |
   = note: consider importing this struct:
           Outer
           Outer
   = note: this error originates in the macro `generate_mod::check` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0412]: cannot find type `FromOutside` in this scope
   |
   |
LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
   |
   = note: consider importing this struct:
           FromOutside
           FromOutside
   = note: this error originates in the attribute macro `generate_mod::check_attr` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0412]: cannot find type `OuterAttr` in this scope
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:12:1
   |
   |
LL | #[generate_mod::check_attr] //~ ERROR cannot find type `FromOutside` in this scope
   |
   = note: consider importing this struct:
           OuterAttr
           OuterAttr
   = note: this error originates in the attribute macro `generate_mod::check_attr` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find type `FromOutside` in this scope
   |
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   |
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find type `OuterDerive` in this scope
   |
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find type `FromOutside` in this scope
   |
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)

error: cannot find type `OuterDerive` in this scope
   |
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0412`.
For more information about this error, try `rustc --explain E0412`.
Future incompatibility report: Future breakage diagnostic:
error: cannot find type `FromOutside` in this scope
   |
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   |
   = note: `#[deny(proc_macro_derive_resolution_fallback)]` on by default
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
Future breakage diagnostic:
error: cannot find type `OuterDerive` in this scope
   |
   |
LL | #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
Future breakage diagnostic:
error: cannot find type `FromOutside` in this scope
   |
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
Future breakage diagnostic:
error: cannot find type `OuterDerive` in this scope
   |
   |
LL |     #[derive(generate_mod::CheckDerive)] //~ ERROR cannot find type `FromOutside` in this scope
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this error originates in the derive macro `generate_mod::CheckDerive` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
Future breakage diagnostic:
warning: cannot find type `FromOutside` in this scope
   |
   |
LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
note: the lint level is defined here
  --> /checkout/src/test/ui/proc-macro/generate-mod.rs:30:10
   |
   |
LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this warning originates in the derive macro `generate_mod::CheckDeriveLint` (in Nightly builds, run with -Z macro-backtrace for more info)
Future breakage diagnostic:
Future breakage diagnostic:
warning: cannot find type `OuterDeriveLint` in this scope
   |
   |
LL | #[derive(generate_mod::CheckDeriveLint)] // OK, lint is suppressed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ names from parent modules are not accessible without an explicit import
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: for more information, see issue #83583 <https://github.com/rust-lang/rust/issues/83583>
   = note: this warning originates in the derive macro `generate_mod::CheckDeriveLint` (in Nightly builds, run with -Z macro-backtrace for more info)

------------------------------------------


---
test result: FAILED. 12188 passed; 1 failed; 117 ignored; 0 measured; 0 filtered out; finished in 133.23s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:54
