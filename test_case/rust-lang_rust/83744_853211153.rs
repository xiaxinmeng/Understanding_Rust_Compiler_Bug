plain
..............................i.i................................................................... 11900/11939
.......................................
failures:

---- [ui] ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs stdout ----


1 error: `crate_type` within an `#![cfg_attr] attribute is deprecated`
-   --> /checkout/src/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs:5:18
+   --> $DIR/future-compat-crate-attributes-using-cfg_attr.rs:5:18
3    |
- LL | #![cfg_attr(foo, crate_type="bin")] //~ERROR `crate_type` within
+ LL | #![cfg_attr(foo, crate_type="bin")]
6    |
6    |
-    = note: `#[deny(deprecated_cfg_attr_crate_type_name)]` on by default
+ note: the lint level is defined here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+   --> $DIR/future-compat-crate-attributes-using-cfg_attr.rs:4:9
+    |
+ LL | #![deny(warnings)]
+    |         ^^^^^^^^
+    = note: `#[deny(deprecated_cfg_attr_crate_type_name)]` implied by `#[deny(warnings)]`
8    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
9    = note: for more information, see issue #XXXXX <https://github.com/rust-lang/rust/issues/XXXXX>


11 error: `crate_name` within an `#![cfg_attr] attribute is deprecated`
-   --> /checkout/src/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs:6:18
+   --> $DIR/future-compat-crate-attributes-using-cfg_attr.rs:6:18
13    |
- LL | #![cfg_attr(foo, crate_name="bar")] //~ERROR `crate_name` within
+ LL | #![cfg_attr(foo, crate_name="bar")]
16    |
17    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!

21 
21 
22 Future incompatibility report: Future breakage date: None, diagnostic:
23 error: `crate_type` within an `#![cfg_attr] attribute is deprecated`
-   --> /checkout/src/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs:5:18
+   --> $DIR/future-compat-crate-attributes-using-cfg_attr.rs:5:18
25    |
- LL | #![cfg_attr(foo, crate_type="bin")] //~ERROR `crate_type` within
+ LL | #![cfg_attr(foo, crate_type="bin")]
28    |
28    |
-    = note: `#[deny(deprecated_cfg_attr_crate_type_name)]` on by default
+ note: the lint level is defined here
+   --> $DIR/future-compat-crate-attributes-using-cfg_attr.rs:4:9
+    |
+ LL | #![deny(warnings)]
+    |         ^^^^^^^^
+    = note: `#[deny(deprecated_cfg_attr_crate_type_name)]` implied by `#[deny(warnings)]`
30    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
31    = note: for more information, see issue #XXXXX <https://github.com/rust-lang/rust/issues/XXXXX>

33 Future breakage date: None, diagnostic:
33 Future breakage date: None, diagnostic:
34 error: `crate_name` within an `#![cfg_attr] attribute is deprecated`
-   --> /checkout/src/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs:6:18
+   --> $DIR/future-compat-crate-attributes-using-cfg_attr.rs:6:18
36    |
- LL | #![cfg_attr(foo, crate_name="bar")] //~ERROR `crate_name` within
+ LL | #![cfg_attr(foo, crate_name="bar")]
39    |
40    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr/future-compat-crate-attributes-using-cfg_attr.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args cfg/future-compat-crate-attributes-using-cfg_attr.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--cfg" "foo" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `crate_type` within an `#![cfg_attr] attribute is deprecated`
  --> /checkout/src/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs:5:18
   |
LL | #![cfg_attr(foo, crate_type="bin")] //~ERROR `crate_type` within
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs:4:9
   |
   |
LL | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(deprecated_cfg_attr_crate_type_name)]` implied by `#[deny(warnings)]`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #XXXXX <https://github.com/rust-lang/rust/issues/XXXXX>

error: `crate_name` within an `#![cfg_attr] attribute is deprecated`
  --> /checkout/src/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs:6:18
   |
LL | #![cfg_attr(foo, crate_name="bar")] //~ERROR `crate_name` within
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #XXXXX <https://github.com/rust-lang/rust/issues/XXXXX>
error: aborting due to 2 previous errors


Future incompatibility report: Future breakage date: None, diagnostic:
error: `crate_type` within an `#![cfg_attr] attribute is deprecated`
  --> /checkout/src/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs:5:18
   |
LL | #![cfg_attr(foo, crate_type="bin")] //~ERROR `crate_type` within
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs:4:9
   |
   |
LL | #![deny(warnings)]
   |         ^^^^^^^^
   = note: `#[deny(deprecated_cfg_attr_crate_type_name)]` implied by `#[deny(warnings)]`
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #XXXXX <https://github.com/rust-lang/rust/issues/XXXXX>
Future breakage date: None, diagnostic:
Future breakage date: None, diagnostic:
error: `crate_name` within an `#![cfg_attr] attribute is deprecated`
  --> /checkout/src/test/ui/cfg/future-compat-crate-attributes-using-cfg_attr.rs:6:18
   |
LL | #![cfg_attr(foo, crate_name="bar")] //~ERROR `crate_name` within
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #XXXXX <https://github.com/rust-lang/rust/issues/XXXXX>

------------------------------------------


---
test result: FAILED. 11841 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 101.98s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:10:44
