plain
.................................................................................................... 9400/11803
.................................................................................................... 9500/11803
.................................................................................................... 9600/11803
............................i......i................................................................ 9700/11803
..........................................................................iiiiiii..iiiiii.i......... 9800/11803
.................................................................................................... 10000/11803
.................................................................................................... 10100/11803
.................................................................................................... 10200/11803
.................................................................................................... 10300/11803
---
.................................................................................................... 11800/11803
...
failures:

---- [ui] ui/feature-gates/feature-gate-unnamed_fields.rs stdout ----
normalized stderr:
error[E0658]: unnamed fields are not yet fully implemented
  --> $DIR/feature-gate-unnamed_fields.rs:3:8
   |
LL |       _: union {
   |  ________^
LL | |         bar: u8,
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
LL | |         baz: u16
LL | |     }
   |
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = note: see issue #49804 <https://github.com/rust-lang/rust/issues/49804> for more information
   = help: add `#![feature(unnamed_fields)]` to the crate attributes to enable

error[E0658]: unnamed fields are not yet fully implemented
  --> $DIR/feature-gate-unnamed_fields.rs:3:5
   |
LL |     _: union {
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
  --> $DIR/feature-gate-unnamed_fields.rs:11:5
LL |     _: struct {
   |     ^
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

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0658, E0740.
Some errors have detailed explanations: E0658, E0740.
For more information about an error, try `rustc --explain E0658`.



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unnamed_fields/feature-gate-unnamed_fields.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gates/feature-gate-unnamed_fields.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unnamed_fields" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gates/feature-gate-unnamed_fields/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
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
  --> /checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs:3:5
   |
LL |     _: union { //~ ERROR unnamed fields are not yet fully implemented
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
  --> /checkout/src/test/ui/feature-gates/feature-gate-unnamed_fields.rs:11:5
   |
LL |     _: struct { //~ ERROR unnamed fields are not yet fully implemented
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

error: aborting due to 6 previous errors

Some errors have detailed explanations: E0658, E0740.
---
test result: FAILED. 11705 passed; 1 failed; 97 ignored; 0 measured; 0 filtered out; finished in 119.86s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:56
