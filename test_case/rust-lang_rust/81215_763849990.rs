plain
failures:

---- [ui] ui/consts/const-int-arithmetic.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-arithmetic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-arithmetic/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-arithmetic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL |                 const $label: $ty = $expr;
...
...
LL |         C41: 100i8.wrapping_div(10), 10;
   |              ^^^^^^^^^^^^^^^^^^^^^^ "calling extern function `core::num::<impl i8>::wrapping_div`" needs an rfc before being allowed inside constants
   |
   = note: `#[deny(const_err)]` on by default

error: any use of this value will cause an error
   |
   |
LL |                 const $label: $ty = $expr;
...
...
LL |         C42: (-128i8).wrapping_div(-1), -128;
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ "calling extern function `core::num::<impl i8>::wrapping_div`" needs an rfc before being allowed inside constants

error: any use of this value will cause an error
   |
   |
LL |                 const $label: $ty = $expr;
...
...
LL |         C43: 100i8.wrapping_rem(10), 0;
   |              ^^^^^^^^^^^^^^^^^^^^^^ "calling extern function `core::num::<impl i8>::wrapping_rem`" needs an rfc before being allowed inside constants

error: any use of this value will cause an error
   |
   |
LL |                 const $label: $ty = $expr;
...
...
LL |         C44: (-128i8).wrapping_rem(-1), 0;
   |              ^^^^^^^^^^^^^^^^^^^^^^^^^ "calling extern function `core::num::<impl i8>::wrapping_rem`" needs an rfc before being allowed inside constants
error: aborting due to 4 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/consts/const-size_of_val-align_of_val.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-size_of_val-align_of_val.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of_val-align_of_val/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-size_of_val-align_of_val/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL | const SIZE_OF_DANGLING: usize = unsafe { mem::size_of_val_raw(0x100 as *const i32) };
   |                                          |
   |                                          |
   |                                          "calling extern function `size_of_val_raw`" needs an rfc before being allowed inside constants
   |
   = note: `#[deny(const_err)]` on by default

error: any use of this value will cause an error
   |
   |
LL | const ALIGN_OF_DANGLING: usize = unsafe { mem::align_of_val_raw(0x100 as *const i16) };
   |                                           |
   |                                           |
   |                                           "calling extern function `align_of_val_raw`" needs an rfc before being allowed inside constants
error: aborting due to 2 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/consts/ptr_is_null.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/ptr_is_null.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_is_null" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type=lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/ptr_is_null/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: any use of this value will cause an error
   |
   |
LL | pub const _: () = assert!(!(FOO as *const usize).is_null());
   |                            |
   |                            |
   |                            "calling extern function `ptr::const_ptr::<impl *const T>::is_null`" needs an rfc before being allowed inside constants
   |
   = note: `#[deny(const_err)]` on by default

error: any use of this value will cause an error
   |
   |
LL | pub const _: () = assert!(!(42 as *const usize).is_null());
   |                            |
   |                            |
   |                            "calling extern function `ptr::const_ptr::<impl *const T>::is_null`" needs an rfc before being allowed inside constants

error: any use of this value will cause an error
   |
   |
LL | pub const _: () = assert!((0 as *const usize).is_null());
   |                           |
   |                           |
   |                           "calling extern function `ptr::const_ptr::<impl *const T>::is_null`" needs an rfc before being allowed inside constants

error: any use of this value will cause an error
   |
   |
LL | pub const _: () = assert!(std::ptr::null::<usize>().is_null());
   |                           |
   |                           |
   |                           "calling extern function `ptr::const_ptr::<impl *const T>::is_null`" needs an rfc before being allowed inside constants

error: any use of this value will cause an error
   |
   |
LL | pub const _: () = assert!(!("foo" as *const str).is_null());
   |                            |
   |                            |
   |                            "calling extern function `ptr::const_ptr::<impl *const T>::is_null`" needs an rfc before being allowed inside constants
error: aborting due to 5 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/generator/metadata-sufficient-for-layout.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/metadata-sufficient-for-layout.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/metadata-sufficient-for-layout" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/metadata-sufficient-for-layout/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_metadata/src/rmeta/decoder.rs:1183:17: get_optimized_mir: missing MIR for `DefId(18:7 ~ metadata_sufficient_for_layout[8787]::g::{closure#0})`
thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
note: rustc 1.51.0-nightly (0566609f2 2021-01-20) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [optimized_mir] optimizing MIR for `metadata_sufficient_for_layout::g::{closure#0}`
#1 [layout_raw] computing layout of `[generator@metadata_sufficient_for_layout::g::{closure#0} {()}]`
end of query stack


------------------------------------------

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:16:01
