plain
-    |
- LL |     v
-    |     ^ lifetime mismatch
-    |
-    = note: expected struct `std::collections::btree_map::IterMut<'_, &'new (), _>`
-               found struct `std::collections::btree_map::IterMut<'_, &'static (), _>`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- note: the lifetime `'new` as defined on the function body at 3:21...
-   --> $DIR/variance-btree-invariant-types.rs:3:21
-    |
- LL | fn iter_cov_key<'a, 'new>(v: IterMut<'a, &'static (), ()>) -> IterMut<'a, &'new (), ()> {
-    |                     ^^^^
-    = note: ...does not necessarily outlive the static lifetime
- error[E0308]: mismatched types
-   --> $DIR/variance-btree-invariant-types.rs:7:5
-    |
- LL |     v
- LL |     v
-    |     ^ lifetime mismatch
-    |
-    = note: expected struct `std::collections::btree_map::IterMut<'_, _, &'new ()>`
-               found struct `std::collections::btree_map::IterMut<'_, _, &'static ()>`
- note: the lifetime `'new` as defined on the function body at 6:21...
-   --> $DIR/variance-btree-invariant-types.rs:6:21
-    |
- LL | fn iter_cov_val<'a, 'new>(v: IterMut<'a, (), &'static ()>) -> IterMut<'a, (), &'new ()> {
-    |                     ^^^^
-    = note: ...does not necessarily outlive the static lifetime
- error[E0308]: mismatched types
32   --> $DIR/variance-btree-invariant-types.rs:10:5
33    |
34 LL |     v
34 LL |     v

178    |                       ^^^^
179    = note: ...does not necessarily outlive the static lifetime
- error: aborting due to 12 previous errors
+ error: aborting due to 10 previous errors
182 
183 For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args variance/variance-btree-invariant-types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/variance/variance-btree-invariant-types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-btree-invariant-types" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/variance/variance-btree-invariant-types/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:10:5
   |
LL |     v //~ ERROR mismatched types
   |     ^ lifetime mismatch
   |
   = note: expected struct `std::collections::btree_map::IterMut<'_, &'static (), _>`
              found struct `std::collections::btree_map::IterMut<'_, &'new (), _>`
note: the lifetime `'new` as defined on the function body at 9:24...
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:9:24
   |
LL | fn iter_contra_key<'a, 'new>(v: IterMut<'a, &'new (), ()>) -> IterMut<'a, &'static (), ()> {
   |                        ^^^^
   = note: ...does not necessarily outlive the static lifetime
error[E0308]: mismatched types
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:13:5
   |
   |
LL |     v //~ ERROR mismatched types
   |     ^ lifetime mismatch
   |
   = note: expected struct `std::collections::btree_map::IterMut<'_, _, &'static ()>`
              found struct `std::collections::btree_map::IterMut<'_, _, &'new ()>`
note: the lifetime `'new` as defined on the function body at 12:24...
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:12:24
   |
LL | fn iter_contra_val<'a, 'new>(v: IterMut<'a, (), &'new ()>) -> IterMut<'a, (), &'static ()> {
   |                        ^^^^
   = note: ...does not necessarily outlive the static lifetime
error[E0308]: mismatched types
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:18:5
   |
   |
LL |     v //~ ERROR mismatched types
   |     ^ lifetime mismatch
   |
   = note: expected struct `std::collections::btree_map::OccupiedEntry<'_, &'new (), _>`
              found struct `std::collections::btree_map::OccupiedEntry<'_, &'static (), _>`
note: the lifetime `'new` as defined on the function body at 16:20...
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:16:20
   |
LL | fn occ_cov_key<'a, 'new>(v: OccupiedEntry<'a, &'static (), ()>)
   |                    ^^^^
   = note: ...does not necessarily outlive the static lifetime
error[E0308]: mismatched types
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:22:5
   |
   |
LL |     v //~ ERROR mismatched types
   |     ^ lifetime mismatch
   |
   = note: expected struct `std::collections::btree_map::OccupiedEntry<'_, _, &'new ()>`
              found struct `std::collections::btree_map::OccupiedEntry<'_, _, &'static ()>`
note: the lifetime `'new` as defined on the function body at 20:20...
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:20:20
   |
LL | fn occ_cov_val<'a, 'new>(v: OccupiedEntry<'a, (), &'static ()>)
   |                    ^^^^
   = note: ...does not necessarily outlive the static lifetime
error[E0308]: mismatched types
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:26:5
   |
   |
LL |     v //~ ERROR mismatched types
   |     ^ lifetime mismatch
   |
   = note: expected struct `std::collections::btree_map::OccupiedEntry<'_, &'static (), _>`
              found struct `std::collections::btree_map::OccupiedEntry<'_, &'new (), _>`
note: the lifetime `'new` as defined on the function body at 24:23...
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:24:23
   |
LL | fn occ_contra_key<'a, 'new>(v: OccupiedEntry<'a, &'new (), ()>)
   |                       ^^^^
   = note: ...does not necessarily outlive the static lifetime
error[E0308]: mismatched types
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:30:5
   |
   |
LL |     v //~ ERROR mismatched types
   |     ^ lifetime mismatch
   |
   = note: expected struct `std::collections::btree_map::OccupiedEntry<'_, _, &'static ()>`
              found struct `std::collections::btree_map::OccupiedEntry<'_, _, &'new ()>`
note: the lifetime `'new` as defined on the function body at 28:23...
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:28:23
   |
LL | fn occ_contra_val<'a, 'new>(v: OccupiedEntry<'a, (), &'new ()>)
   |                       ^^^^
   = note: ...does not necessarily outlive the static lifetime
error[E0308]: mismatched types
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:35:5
   |
   |
LL |     v //~ ERROR mismatched types
   |     ^ lifetime mismatch
   |
   = note: expected struct `std::collections::btree_map::VacantEntry<'_, &'new (), _>`
              found struct `std::collections::btree_map::VacantEntry<'_, &'static (), _>`
note: the lifetime `'new` as defined on the function body at 33:20...
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:33:20
   |
LL | fn vac_cov_key<'a, 'new>(v: VacantEntry<'a, &'static (), ()>)
   |                    ^^^^
   = note: ...does not necessarily outlive the static lifetime
error[E0308]: mismatched types
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:39:5
   |
   |
LL |     v //~ ERROR mismatched types
   |     ^ lifetime mismatch
   |
   = note: expected struct `std::collections::btree_map::VacantEntry<'_, _, &'new ()>`
              found struct `std::collections::btree_map::VacantEntry<'_, _, &'static ()>`
note: the lifetime `'new` as defined on the function body at 37:20...
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:37:20
   |
LL | fn vac_cov_val<'a, 'new>(v: VacantEntry<'a, (), &'static ()>)
   |                    ^^^^
   = note: ...does not necessarily outlive the static lifetime
error[E0308]: mismatched types
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:43:5
   |
   |
LL |     v //~ ERROR mismatched types
   |     ^ lifetime mismatch
   |
   = note: expected struct `std::collections::btree_map::VacantEntry<'_, &'static (), _>`
              found struct `std::collections::btree_map::VacantEntry<'_, &'new (), _>`
note: the lifetime `'new` as defined on the function body at 41:23...
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:41:23
   |
LL | fn vac_contra_key<'a, 'new>(v: VacantEntry<'a, &'new (), ()>)
   |                       ^^^^
   = note: ...does not necessarily outlive the static lifetime
error[E0308]: mismatched types
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:47:5
   |
   |
LL |     v //~ ERROR mismatched types
   |     ^ lifetime mismatch
   |
   = note: expected struct `std::collections::btree_map::VacantEntry<'_, _, &'static ()>`
              found struct `std::collections::btree_map::VacantEntry<'_, _, &'new ()>`
note: the lifetime `'new` as defined on the function body at 45:23...
  --> /checkout/src/test/ui/variance/variance-btree-invariant-types.rs:45:23
   |
LL | fn vac_contra_val<'a, 'new>(v: VacantEntry<'a, (), &'new ()>)
   |                       ^^^^
   = note: ...does not necessarily outlive the static lifetime
error: aborting due to 10 previous errors

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 11857 passed; 1 failed; 98 ignored; 0 measured; 0 filtered out; finished in 124.28s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:12
