plain
........................................................................i........................... 6000/11258
.................................................................................................... 6100/11258
.......................ii.ii.......i...i............................................................ 6200/11258
.................................................................................i.................. 6300/11258
.......i....................................................................................F..F.... 6400/11258
..........F.........i............................................................................... 6500/11258
.ii..........................................i...................................................... 6700/11258
.................................................................................................... 6800/11258
.................................................................................................... 6900/11258
.i.................................................................................................. 7000/11258
---

---- [ui] ui/lint/lint-removed-cmdline.rs stdout ----
diff of stderr:

- warning: lint `raw_pointer_derive` has been removed: `using derive with raw pointers is ok`
+ warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
2    |
3    = note: requested on the command line with `-D raw_pointer_derive`


- warning: lint `raw_pointer_derive` has been removed: `using derive with raw pointers is ok`
+ warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
6    |
7    = note: requested on the command line with `-D raw_pointer_derive`


- warning: lint `raw_pointer_derive` has been removed: `using derive with raw pointers is ok`
+ warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
10    |
11    = note: requested on the command line with `-D raw_pointer_derive`


- warning: lint `raw_pointer_derive` has been removed: `using derive with raw pointers is ok`
+ warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
14    |
15    = note: requested on the command line with `-D raw_pointer_derive`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-removed-cmdline/lint-removed-cmdline.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-removed-cmdline/lint-removed-cmdline.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-removed-cmdline.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-removed-cmdline.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-removed-cmdline" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-D" "raw_pointer_derive" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-removed-cmdline/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
   |
   = note: requested on the command line with `-D raw_pointer_derive`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
   |
   = note: requested on the command line with `-D raw_pointer_derive`

warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
   |
   = note: requested on the command line with `-D raw_pointer_derive`

warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
   |
   = note: requested on the command line with `-D raw_pointer_derive`

error: unused variable: `unused`
   |
   |
LL | fn main() { let unused = (); }
   |                 ^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-removed-cmdline.rs:11:8
   |
LL | #[deny(warnings)]
---

---- [ui] ui/lint/lint-removed.rs stdout ----
diff of stderr:

- warning: lint `raw_pointer_derive` has been removed: `using derive with raw pointers is ok`
+ warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
3    |
3    |
4 LL | #[deny(raw_pointer_derive)]

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-removed/lint-removed.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-removed/lint-removed.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-removed.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-removed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-removed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-removed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint `raw_pointer_derive` has been removed: using derive with raw pointers is ok
   |
   |
LL | #[deny(raw_pointer_derive)] //~ WARN `raw_pointer_derive` has been removed
   |
   = note: `#[warn(renamed_and_removed_lints)]` on by default


error: unused variable: `unused`
   |
   |
LL | fn main() { let unused = (); } //~ ERROR unused
   |                 ^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused`
note: the lint level is defined here
  --> /checkout/src/test/ui/lint/lint-removed.rs:7:8
   |
   |
LL | #[deny(unused_variables)]

error: aborting due to previous error; 1 warning emitted



------------------------------------------


---- [ui] ui/lint/lint-unexported-no-mangle.rs stdout ----
diff of stderr:

- warning: lint `private_no_mangle_fns` has been removed: `no longer a warning, `#[no_mangle]` functions always exported`
+ warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
2    |
3    = note: requested on the command line with `-F private_no_mangle_fns`


- warning: lint `private_no_mangle_statics` has been removed: `no longer a warning, `#[no_mangle]` statics always exported`
+ warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
6    |
7    = note: requested on the command line with `-F private_no_mangle_statics`


- warning: lint `private_no_mangle_fns` has been removed: `no longer a warning, `#[no_mangle]` functions always exported`
+ warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
10    |
11    = note: requested on the command line with `-F private_no_mangle_fns`


- warning: lint `private_no_mangle_statics` has been removed: `no longer a warning, `#[no_mangle]` statics always exported`
+ warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
14    |
15    = note: requested on the command line with `-F private_no_mangle_statics`


- warning: lint `private_no_mangle_fns` has been removed: `no longer a warning, `#[no_mangle]` functions always exported`
+ warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
18    |
19    = note: requested on the command line with `-F private_no_mangle_fns`


- warning: lint `private_no_mangle_statics` has been removed: `no longer a warning, `#[no_mangle]` statics always exported`
+ warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
22    |
23    = note: requested on the command line with `-F private_no_mangle_statics`


- warning: lint `private_no_mangle_fns` has been removed: `no longer a warning, `#[no_mangle]` functions always exported`
+ warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
26    |
27    = note: requested on the command line with `-F private_no_mangle_fns`


- warning: lint `private_no_mangle_statics` has been removed: `no longer a warning, `#[no_mangle]` statics always exported`
+ warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
30    |
31    = note: requested on the command line with `-F private_no_mangle_statics`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unexported-no-mangle/lint-unexported-no-mangle.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unexported-no-mangle/lint-unexported-no-mangle.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lint/lint-unexported-no-mangle.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/lint-unexported-no-mangle.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unexported-no-mangle" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-F" "private_no_mangle_fns" "-F" "no_mangle_const_items" "-F" "private_no_mangle_statics" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/lint-unexported-no-mangle/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
   |
   = note: requested on the command line with `-F private_no_mangle_fns`

warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
   |
   = note: requested on the command line with `-F private_no_mangle_statics`

warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
   |
   = note: requested on the command line with `-F private_no_mangle_fns`

warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
   |
   = note: requested on the command line with `-F private_no_mangle_statics`

warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
   |
   = note: requested on the command line with `-F private_no_mangle_fns`

warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
   |
   = note: requested on the command line with `-F private_no_mangle_statics`

warning: lint `private_no_mangle_fns` has been removed: no longer a warning, `#[no_mangle]` functions always exported
   |
   = note: requested on the command line with `-F private_no_mangle_fns`

warning: lint `private_no_mangle_statics` has been removed: no longer a warning, `#[no_mangle]` statics always exported
   |
   = note: requested on the command line with `-F private_no_mangle_statics`

error: const items should never be `#[no_mangle]`
   |
   |
LL | const FOO: u64 = 1; //~ ERROR const items should never be `#[no_mangle]`
   | |
   | |
   | help: try a static value: `pub static`
   |
   = note: requested on the command line with `-F no-mangle-const-items`

error: const items should never be `#[no_mangle]`
   |
   |
LL | pub const PUB_FOO: u64 = 1; //~ ERROR const items should never be `#[no_mangle]`
   | |
   | |
   | help: try a static value: `pub static`
error: aborting due to 2 previous errors; 8 warnings emitted


------------------------------------------
---
test result: FAILED. 11168 passed; 3 failed; 87 ignored; 0 measured; 0 filtered out; finished in 116.79s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:12:59
