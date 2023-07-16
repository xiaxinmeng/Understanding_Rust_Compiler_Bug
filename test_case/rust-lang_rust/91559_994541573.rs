plain
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/alloc-no-oom-handling/alloc-no-oom-handling  --edition=2018 --crate-type=rlib ../../../../library/alloc/src/lib.rs --cfg feature=\"external_crate\" --cfg no_global_oom_handling
------------------------------------------
stderr:
------------------------------------------
error[E0432]: unresolved import `self::spec_extend`
error[E0432]: unresolved import `self::spec_extend`
   --> ../../../../library/alloc/src/vec/mod.rs:147:11
    |
147 | use self::spec_extend::TrySpecExtend;
    |           ^^^^^^^^^^^ could not find `spec_extend` in `self`
error[E0433]: failed to resolve: use of undeclared type `AllocInit`
   --> ../../../../library/alloc/src/raw_vec.rs:159:41
    |
    |
159 |         Self::try_allocate_in(capacity, AllocInit::Uninitialized, alloc)
    |                                         ^^^^^^^^^ use of undeclared type `AllocInit`
error[E0433]: failed to resolve: use of undeclared type `AllocInit`
   --> ../../../../library/alloc/src/raw_vec.rs:229:17
    |
    |
229 |                 AllocInit::Uninitialized => alloc.allocate(layout),
    |                 ^^^^^^^^^ use of undeclared type `AllocInit`
error[E0433]: failed to resolve: use of undeclared type `AllocInit`
   --> ../../../../library/alloc/src/raw_vec.rs:230:17
    |
    |
230 |                 AllocInit::Zeroed => alloc.allocate_zeroed(layout),
    |                 ^^^^^^^^^ use of undeclared type `AllocInit`
error[E0433]: failed to resolve: use of undeclared type `SetLenOnDrop`
    --> ../../../../library/alloc/src/vec/mod.rs:2576:33
     |
     |
2576 |             let mut local_len = SetLenOnDrop::new(&mut self.len);
     |                                 ^^^^^^^^^^^^ use of undeclared type `SetLenOnDrop`
error[E0412]: cannot find type `AllocInit` in this scope
   --> ../../../../library/alloc/src/raw_vec.rs:215:15
    |
215 |         init: AllocInit,
215 |         init: AllocInit,
    |               ^^^^^^^^^ not found in this scope

error[E0425]: cannot find value `Global` in this scope
   --> ../../../../library/alloc/src/slice.rs:549:28
    |
549 |         self.try_to_vec_in(Global)
    |
help: consider importing this unit struct
    |
85  | use crate::alloc::Global;
85  | use crate::alloc::Global;
    |

error[E0412]: cannot find type `Vec` in this scope
    --> ../../../../library/alloc/src/sync.rs:2527:32
     |
2527 |     pub fn try_from_vec(mut v: Vec<T>) -> Result<Self, TryReserveError> {
     |
help: consider importing this struct
     |
7    | use crate::vec::Vec;
7    | use crate::vec::Vec;
     |

error[E0599]: no function or associated item named `try_with_capacity` found for struct `RawVec` in the current scope
   --> ../../../../library/alloc/src/boxed.rs:629:27
    |
629 |             match RawVec::try_with_capacity(len) {
    |                           |
    |                           function or associated item not found in `RawVec<_, _>`
    |                           help: there is an associated function with a similar name: `try_with_capacity_in`
    |
    |
   ::: ../../../../library/alloc/src/raw_vec.rs:52:1
    |
52  | pub(crate) struct RawVec<T, A: Allocator = Global> {
    | -------------------------------------------------- function or associated item `try_with_capacity` not found for this

error[E0599]: no function or associated item named `try_with_capacity_zeroed` found for struct `RawVec` in the current scope
   --> ../../../../library/alloc/src/boxed.rs:659:27
    |
659 |             match RawVec::try_with_capacity_zeroed(len) {
    |                           |
    |                           function or associated item not found in `RawVec<_, _>`
    |                           help: there is an associated function with a similar name: `try_with_capacity_in`
    |
    |
   ::: ../../../../library/alloc/src/raw_vec.rs:52:1
    |
52  | pub(crate) struct RawVec<T, A: Allocator = Global> {
    | -------------------------------------------------- function or associated item `try_with_capacity_zeroed` not found for this

error[E0599]: no method named `reserve_for_push` found for struct `RawVec` in the current scope
    --> ../../../../library/alloc/src/vec/mod.rs:1905:22
     |
1905 |             self.buf.reserve_for_push(self.len);
     |                      ^^^^^^^^^^^^^^^^ method not found in `RawVec<T, A>`
    ::: ../../../../library/alloc/src/raw_vec.rs:52:1
     |
     |
52   | pub(crate) struct RawVec<T, A: Allocator = Global> {
     | -------------------------------------------------- method `reserve_for_push` not found for this

error[E0599]: no method named `try_spec_extend` found for mutable reference `&mut Vec<T, A>` in the current scope
    --> ../../../../library/alloc/src/vec/mod.rs:2476:14
     |
2476 |         self.try_spec_extend(other.iter())
     |              ^^^^^^^^^^^^^^^ method not found in `&mut Vec<T, A>`
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0412, E0425, E0432, E0433, E0599.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.
make: *** [Makefile:4: all] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/alloc-no-oom-handling

test result: FAILED. 209 passed; 1 failed; 18 ignored; 0 measured; 0 filtered out; finished in 27.87s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-12/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:30:17
