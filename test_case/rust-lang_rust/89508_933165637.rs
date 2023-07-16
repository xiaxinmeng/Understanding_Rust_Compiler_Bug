plain
---- [ui] ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs stdout ----
diff of 32bit.stderr:

161    |
162 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
163    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
- help: skipping check for `const_panic` feature
-   --> $DIR/const_refers_to_static_cross_crate.rs:32:77
-    |
- LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
169 help: skipping check that does not even have a feature gate
170   --> $DIR/const_refers_to_static_cross_crate.rs:32:20
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i686-unknown-linux-gnu
171    |
171    |

172 LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
-    = note: this warning originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
175 
176 error: aborting due to 10 previous errors; 3 warnings emitted
177 
177 


The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/const_refers_to_static_cross_crate.32bit.stderr
To only update this specific test, also pass `--test-args consts/miri_unleashed/const_refers_to_static_cross_crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:12:1
   |
LL | / const SLICE_MUT: &[u8; 1] = { //~ ERROR undefined behavior to use this value
LL | | //~| encountered a reference pointing to a static variable
LL | |     unsafe { &static_cross_crate::ZERO }
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:40:9
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:40:9
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:17:1
   |
LL | / const U8_MUT: &u8 = { //~ ERROR undefined behavior to use this value
LL | | //~| encountered a reference pointing to a static variable
LL | |     unsafe { &static_cross_crate::ZERO[0] }
LL | | };
   | |__^ type validation failed: encountered a reference pointing to a static variable
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:49:9
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:49:9
   |
LL |         U8_MUT => true,

warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:25:15
   |
   |
LL | / const U8_MUT2: &u8 = {
LL | |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
   | |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
LL | |     //~^ WARN [const_err]
LL | |     //~| constant accesses static
LL | |     //~| WARN this was previously accepted by the compiler but is being phased out
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:23:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: could not evaluate constant pattern
error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:60:9
   |
LL |         U8_MUT2 => true,

warning: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
   |
LL | / const U8_MUT3: &u8 = {
LL | |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
   | |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constant accesses static
LL | |     //~^ WARN [const_err]
LL | |     //~| constant accesses static
LL | |     //~| WARN this was previously accepted by the compiler but is being phased out
LL | | };
   |
note: the lint level is defined here
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:30:8
   |
   |
LL | #[warn(const_err)]
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: could not evaluate constant pattern
error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:68:9
   |
LL |         U8_MUT3 => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:40:9
   |
   |
LL |         SLICE_MUT => true,
   |         ^^^^^^^^^

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:49:9
   |
LL |         U8_MUT => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:60:9
   |
   |
LL |         U8_MUT2 => true,

error: could not evaluate constant pattern
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:68:9
   |
   |
LL |         U8_MUT3 => true,

warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
---
   |               ^^^^^^^^^^^^^^^^^^^^^^^^
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:19:15
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:19:15
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:19:15
   |
   |
LL |     unsafe { &static_cross_crate::ZERO[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:25:17
   |
   |
LL |     unsafe { &(*static_cross_crate::ZERO_REF)[0] }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/const_refers_to_static_cross_crate.rs:32:20
   |
   |
LL |     unsafe { match static_cross_crate::OPT_ZERO { Some(ref u) => u, None => panic!() } }

error: aborting due to 10 previous errors; 3 warnings emitted

For more information about this error, try `rustc --explain E0080`.
---
test result: FAILED. 12074 passed; 1 failed; 163 ignored; 0 measured; 0 filtered out; finished in 79.09s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i686-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i686-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i686-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--pass" "check" "--nodejs" "/usr/bin/node" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:01:21
