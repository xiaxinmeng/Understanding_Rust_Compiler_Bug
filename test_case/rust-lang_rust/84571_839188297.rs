plain
.................................................................................................... 9500/11846
.................................................................................................... 9600/11846
......................................................i......i...................................... 9700/11846
.................................................................................................... 9800/11846
iiiiiii..iiiiii.i................................................................................... 9900/11846
.................................................................................................... 10100/11846
.................................................................................................... 10200/11846
.................................................................................................... 10300/11846
.................................................................................................... 10400/11846
---
.....................................i.i............................................................ 11800/11846
..............................................
failures:

---- [ui] ui/feature-gates/feature-gate-unnamed_fields.rs stdout ----
normalized stderr:
error[E0658]: unnamed fields are not yet fully implemented
  --> $DIR/feature-gate-unnamed_fields.rs:3:5
   |
LL |     _: union {
   |
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = help: add `#![feature(unnamed_fields)]` to the crate attributes to enable

error[E0658]: unnamed fields are not yet fully implemented
  --> $DIR/feature-gate-unnamed_fields.rs:3:8
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL |       _: union {
   |  ________^
LL | |         bar: u8,
LL | |         baz: u16
LL | |     }
   |
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = help: add `#![feature(unnamed_fields)]` to the crate attributes to enable

error[E0658]: unnamed fields are not yet fully implemented
  --> $DIR/feature-gate-unnamed_fields.rs:11:5
LL |     _: struct {
   |     ^
   |
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = help: add `#![feature(unnamed_fields)]` to the crate attributes to enable

error[E0658]: unnamed fields are not yet fully implemented
  --> $DIR/feature-gate-unnamed_fields.rs:11:8
LL |       _: struct {
   |  ________^
   |  ________^
LL | |         foobaz: u8,
LL | |         barbaz: u16
LL | |     }
   |
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = help: add `#![feature(unnamed_fields)]` to the crate attributes to enable

error[E0658]: unnamed fields are not yet fully implemented
  --> $DIR/feature-gate-unnamed_fields.rs:19:5
LL |     _: S
   |     ^
   |
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = help: add `#![feature(unnamed_fields)]` to the crate attributes to enable
error: anonymous unions are unimplemented
  --> $DIR/feature-gate-unnamed_fields.rs:3:8
   |
   |
LL |       _: union {
   |  ________^
LL | |         bar: u8,
LL | |         baz: u16
LL | |     }

error: anonymous structs are unimplemented
  --> $DIR/feature-gate-unnamed_fields.rs:11:8
   |
   |
LL |       _: struct {
   |  ________^
LL | |         foobaz: u8,
LL | |         barbaz: u16
LL | |     }


error[E0740]: unions may not contain fields that need dropping
  --> $DIR/feature-gate-unnamed_fields.rs:11:5
   |
LL | /     _: struct {
LL | |         foobaz: u8,
LL | |         barbaz: u16
LL | |     }
   |
   |
note: `std::mem::ManuallyDrop` can be used to wrap the type
  --> $DIR/feature-gate-unnamed_fields.rs:11:5
   |
LL | /     _: struct {
LL | |         foobaz: u8,
LL | |         barbaz: u16
LL | |     }

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0658, E0740.
---
To only update this specific test, also pass `--test-args feature-gates/feature-gate-unnamed_fields.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unnamed_fields" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unnamed_fields/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: unnamed fields are not yet fully implemented
  --> /checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs:3:5
   |
LL |     _: union { //~ ERROR unnamed fields are not yet fully implemented
   |
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = help: add `#![feature(unnamed_fields)]` to the crate attributes to enable

error[E0658]: unnamed fields are not yet fully implemented
  --> /checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs:3:8
   |
LL |       _: union { //~ ERROR unnamed fields are not yet fully implemented
   |  ________^
LL | |         bar: u8,
LL | |         baz: u16
LL | |     }
   |
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = help: add `#![feature(unnamed_fields)]` to the crate attributes to enable

error[E0658]: unnamed fields are not yet fully implemented
  --> /checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs:11:5
   |
LL |     _: struct { //~ ERROR unnamed fields are not yet fully implemented
   |
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = help: add `#![feature(unnamed_fields)]` to the crate attributes to enable

error[E0658]: unnamed fields are not yet fully implemented
  --> /checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs:11:8
   |
LL |       _: struct { //~ ERROR unnamed fields are not yet fully implemented
   |  ________^
LL | |         foobaz: u8,
LL | |         barbaz: u16
LL | |     }
   |
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = help: add `#![feature(unnamed_fields)]` to the crate attributes to enable

error[E0658]: unnamed fields are not yet fully implemented
  --> /checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs:19:5
   |
LL |     _: S //~ ERROR unnamed fields are not yet fully implemented
   |
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = help: add `#![feature(unnamed_fields)]` to the crate attributes to enable
error: anonymous unions are unimplemented
  --> /checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs:3:8
   |
   |
LL |       _: union { //~ ERROR unnamed fields are not yet fully implemented
   |  ________^
LL | |         bar: u8,
LL | |         baz: u16
LL | |     }

error: anonymous structs are unimplemented
  --> /checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs:11:8
   |
   |
LL |       _: struct { //~ ERROR unnamed fields are not yet fully implemented
   |  ________^
LL | |         foobaz: u8,
LL | |         barbaz: u16
LL | |     }


error[E0740]: unions may not contain fields that need dropping
  --> /checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs:11:5
   |
LL | /     _: struct { //~ ERROR unnamed fields are not yet fully implemented
LL | |         foobaz: u8,
LL | |         barbaz: u16
LL | |     }
   |
   |
note: `std::mem::ManuallyDrop` can be used to wrap the type
  --> /checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs:11:5
   |
LL | /     _: struct { //~ ERROR unnamed fields are not yet fully implemented
LL | |         foobaz: u8,
LL | |         barbaz: u16
LL | |     }

error: aborting due to 8 previous errors

Some errors have detailed explanations: E0658, E0740.
Some errors have detailed explanations: E0658, E0740.
For more information about an error, try `rustc --explain E0658`.

------------------------------------------


---- [ui] ui/unnamed_fields/prohibit_anonymous.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unnamed_fields/prohibit_anonymous.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unnamed_fields/prohibit_anonymous" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unnamed_fields/prohibit_anonymous/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: anonymous struct not allowed in return type
  --> /checkout/src/test/ui/unnamed_fields/prohibit_anonymous.rs:4:11
   |
LL | fn f() -> struct { field: u8 } {} // Should error
   |           ^^^^^^^^^^^^^^^^^^^^ anonymous struct outside of struct or union

error: anonymous struct not allowed with a named identifier
  --> /checkout/src/test/ui/unnamed_fields/prohibit_anonymous.rs:6:11
   |
LL | union G { field: struct { field: u8 } } // Should error
   |           |
   |           |
   |           help: replace with `_`: `_`

error: anonymous struct not allowed in tuple struct
  --> /checkout/src/test/ui/unnamed_fields/prohibit_anonymous.rs:10:1
   |
LL | struct I(struct { field: u8 }, u8); // Should error
   |          |
   |          anonymous struct outside of struct or union


thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', compiler/rustc_ast_passes/src/ast_validation.rs:234:35

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (830f521ec 2021-05-11) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack
error: aborting due to 3 previous errors



------------------------------------------



failures:
    [ui] ui/feature-gates/feature-gate-unnamed_fields.rs
    [ui] ui/unnamed_fields/prohibit_anonymous.rs
test result: FAILED. 11748 passed; 2 failed; 96 ignored; 0 measured; 0 filtered out; finished in 116.33s




command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:38
