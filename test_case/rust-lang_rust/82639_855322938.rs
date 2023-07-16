plain
diff of stderr:

1 error[E0463]: can't find crate for `std`
2    |
3    = note: the `thumbv6m-none-eabi` target may not be installed
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    = help: consider downloading the target with `rustup target add thumbv6m-none-eabi`
6 error: aborting due to previous error
7 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37131/issue-37131.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-37131.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37131.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37131" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target=thumbv6m-none-eabi" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37131/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `std`
   |
   = note: the `thumbv6m-none-eabi` target may not be installed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.

---
diff of stderr:

1 error[E0463]: can't find crate for `core`
2    |
3    = note: the `thumbv7em-none-eabihf` target may not be installed
-    = help: consider downloading the target with `rustup target add thumbv7em-none-eabihf`
6 error: aborting due to previous error
7 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49851/compiler-builtins-error/compiler-builtins-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-49851/compiler-builtins-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-49851/compiler-builtins-error.rs" "-Zthreads=1" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49851/compiler-builtins-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target" "thumbv7em-none-eabihf" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-49851/compiler-builtins-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0463]: can't find crate for `core`
   |
   = note: the `thumbv7em-none-eabihf` target may not be installed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0463`.


------------------------------------------


---- [ui] ui/lint/issue-83477.rs stdout ----
normalized stderr:
warning: unknown lint: `rustc::foo::bar::default_hash_types`
   |
   |
LL | #[allow(rustc::foo::bar::default_hash_types)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `rustc::default_hash_types`
   = note: `#[warn(unknown_lints)]` on by default

warning: unknown lint: `rustc::foo::default_hash_types`
  --> $DIR/issue-83477.rs:9:9
  --> $DIR/issue-83477.rs:9:9
   |
LL | #[allow(rustc::foo::default_hash_types)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `rustc::default_hash_types`

warning: Prefer FxHashMap over HashMap, it has better performance
   |
   |
LL |     let _ = std::collections::HashMap::<String, String>::new();
   |                               ^^^^^^^ help: use: `FxHashMap`
note: the lint level is defined here
  --> $DIR/issue-83477.rs:3:9
   |
LL | #![warn(rustc::internal)]
LL | #![warn(rustc::internal)]
   |         ^^^^^^^^^^^^^^^
   = note: `#[warn(rustc::default_hash_types)]` implied by `#[warn(rustc::internal)]`
   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
warning: 3 warnings emitted



---
To only update this specific test, also pass `--test-args lint/issue-83477.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/issue-83477.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-83477" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/issue-83477/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: unknown lint: `rustc::foo::bar::default_hash_types`
   |
   |
LL | #[allow(rustc::foo::bar::default_hash_types)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `rustc::default_hash_types`
   = note: `#[warn(unknown_lints)]` on by default

warning: unknown lint: `rustc::foo::default_hash_types`
  --> /checkout/src/test/ui/lint/issue-83477.rs:9:9
  --> /checkout/src/test/ui/lint/issue-83477.rs:9:9
   |
LL | #[allow(rustc::foo::default_hash_types)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: did you mean: `rustc::default_hash_types`

warning: Prefer FxHashMap over HashMap, it has better performance
   |
   |
LL |     let _ = std::collections::HashMap::<String, String>::new();
   |                               ^^^^^^^ help: use: `FxHashMap`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/issue-83477.rs:3:9
   |
LL | #![warn(rustc::internal)]
LL | #![warn(rustc::internal)]
   |         ^^^^^^^^^^^^^^^
   = note: `#[warn(rustc::default_hash_types)]` implied by `#[warn(rustc::internal)]`
   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
warning: 3 warnings emitted


------------------------------------------
---
test result: FAILED. 11856 passed; 3 failed; 98 ignored; 0 measured; 0 filtered out; finished in 116.76s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:11:33
