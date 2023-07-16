plain

---- [ui] ui/c-variadic/variadic-ffi-4.rs stdout ----
diff of stderr:

85 LL |     ap0 = &mut ap1;
86    |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
87    |
-    = note: requirement occurs because of a mutable pointer to VaListImpl<'_>
+    = note: requirement occurs because of a mutable reference to VaListImpl<'_>
89    = note: mutable references are invariant over their type parameter
90    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance


99 LL |     ap0 = &mut ap1;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
100    |     ^^^^^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
101    |
-    = note: requirement occurs because of a mutable pointer to VaListImpl<'_>
+    = note: requirement occurs because of a mutable reference to VaListImpl<'_>
103    = note: mutable references are invariant over their type parameter
104    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/variadic-ffi-4.stderr
To only update this specific test, also pass `--test-args c-variadic/variadic-ffi-4.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/c-variadic/variadic-ffi-4.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/c-variadic/variadic-ffi-4/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:8:5
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ function was supposed to return data with lifetime `'1` but it is returning data with lifetime `'f`
   |
   = note: requirement occurs because of the type VaListImpl<'_>, which makes the generic argument '_ invariant
   = note: the struct VaListImpl<'f> is invariant over the parameter 'f
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:8:5
   |
   |
LL | pub unsafe extern "C" fn no_escape0<'f>(_: usize, ap: ...) -> VaListImpl<'f> {
   |                                     --            -- has type `VaListImpl<'1>`
   |                                     |
   |                                     lifetime `'f` defined here
LL |     ap
   |     ^^ returning this value requires that `'1` must outlive `'f`
   |
   = note: requirement occurs because of the type VaListImpl<'_>, which makes the generic argument '_ invariant
   = note: the struct VaListImpl<'f> is invariant over the parameter 'f
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:14:5
   |
   |
LL | pub unsafe extern "C" fn no_escape1(_: usize, ap: ...) -> VaListImpl<'static> {
   |                                               -- has type `VaListImpl<'1>`
LL |     ap //~ ERROR: lifetime may not live long enough
   |     ^^ returning this value requires that `'1` must outlive `'static`
   |
   = note: requirement occurs because of the type VaListImpl<'_>, which makes the generic argument '_ invariant
   = note: the struct VaListImpl<'f> is invariant over the parameter 'f
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:18:31
   |
   |
LL |     let _ = ap.with_copy(|ap| ap); //~ ERROR: lifetime may not live long enough
   |                           --- ^^ returning this value requires that `'1` must outlive `'2`
   |                           | |
   |                           | return type of closure is VaList<'2, '_>
   |                           has type `VaList<'1, '_>`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1;
LL |     *ap0 = ap1;
   |     ^^^^ assignment requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of the type VaListImpl<'_>, which makes the generic argument '_ invariant
   = note: the struct VaListImpl<'f> is invariant over the parameter 'f
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:22:5
   |
   |
LL | pub unsafe extern "C" fn no_escape3(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1;
LL |     *ap0 = ap1;
   |     ^^^^ assignment requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of the type VaListImpl<'_>, which makes the generic argument '_ invariant
   = note: the struct VaListImpl<'f> is invariant over the parameter 'f
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:28:5
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     ap0 = &mut ap1;
   |     ^^^^^^^^^^^^^^ assignment requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of a mutable reference to VaListImpl<'_>
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:28:5
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     ap0 = &mut ap1;
   |     ^^^^^^^^^^^^^^ assignment requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of a mutable reference to VaListImpl<'_>
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

error[E0597]: `ap1` does not live long enough
   |
   |
LL | pub unsafe extern "C" fn no_escape4(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                                        - let's call the lifetime of this reference `'3`
LL |     ap0 = &mut ap1;
   |     |     |
   |     |     |
   |     |     borrowed value does not live long enough
   |     assignment requires that `ap1` is borrowed for `'3`
LL | }
LL | }
   | - `ap1` dropped here while still borrowed
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:35:12
   |
   |
LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1.clone();
   |            ^^^^^^^^^^^ argument requires that `'1` must outlive `'2`
   |
   = note: requirement occurs because of the type VaListImpl<'_>, which makes the generic argument '_ invariant
   = note: the struct VaListImpl<'f> is invariant over the parameter 'f
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/c-variadic/variadic-ffi-4.rs:35:12
   |
   |
LL | pub unsafe extern "C" fn no_escape5(_: usize, mut ap0: &mut VaListImpl, mut ap1: ...) {
   |                                               -------                   ------- has type `VaListImpl<'2>`
   |                                               has type `&mut VaListImpl<'1>`
   |                                               has type `&mut VaListImpl<'1>`
LL |     *ap0 = ap1.clone();
   |            ^^^^^^^^^^^ argument requires that `'2` must outlive `'1`
   |
   = note: requirement occurs because of the type VaListImpl<'_>, which makes the generic argument '_ invariant
   = note: the struct VaListImpl<'f> is invariant over the parameter 'f
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to 11 previous errors

For more information about this error, try `rustc --explain E0597`.


------------------------------------------


---- [ui] ui/nll/type-check-pointer-comparisons.rs stdout ----
diff of stderr:

9    |     ^ requires that `'a` must outlive `'b`
10    |
11    = help: consider adding the following bound: `'a: 'b`
-    = note: requirement occurs because of a mutable pointer to &i32
+    = note: requirement occurs because of a mutable reference to &i32
13    = note: mutable references are invariant over their type parameter
14    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance


24    |          ^ requires that `'b` must outlive `'a`
25    |
26    = help: consider adding the following bound: `'b: 'a`
-    = note: requirement occurs because of a mutable pointer to &i32
+    = note: requirement occurs because of a mutable reference to &i32
28    = note: mutable references are invariant over their type parameter
29    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance


73    |     ^ requires that `'a` must outlive `'b`
74    |
75    = help: consider adding the following bound: `'a: 'b`
-    = note: requirement occurs because of a mutable pointer to &i32
+    = note: requirement occurs because of a mutable reference to &i32
77    = note: mutable references are invariant over their type parameter
78    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance


88    |          ^ requires that `'b` must outlive `'a`
89    |
90    = help: consider adding the following bound: `'b: 'a`
-    = note: requirement occurs because of a mutable pointer to &i32
+    = note: requirement occurs because of a mutable reference to &i32
92    = note: mutable references are invariant over their type parameter
93    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons/type-check-pointer-comparisons.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons/type-check-pointer-comparisons.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/type-check-pointer-comparisons.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/type-check-pointer-comparisons.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/type-check-pointer-comparisons/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:6:5
   |
LL | fn compare_const<'a, 'b>(x: *const &mut &'a i32, y: *const &mut &'b i32) {
   |                  --  -- lifetime `'b` defined here
   |                  |
   |                  lifetime `'a` defined here
LL |     x == y;
   |     ^ requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to &i32
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:6:10
   |
   |
LL | fn compare_const<'a, 'b>(x: *const &mut &'a i32, y: *const &mut &'b i32) {
   |                  --  -- lifetime `'b` defined here
   |                  |
   |                  lifetime `'a` defined here
LL |     x == y;
   |          ^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference to &i32
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:12:5
   |
   |
LL | fn compare_mut<'a, 'b>(x: *mut &'a i32, y: *mut &'b i32) {
   |                --  -- lifetime `'b` defined here
   |                |
   |                lifetime `'a` defined here
LL |     x == y;
   |     ^ requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable pointer to &i32
   = note: mutable pointers are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:12:10
   |
   |
LL | fn compare_mut<'a, 'b>(x: *mut &'a i32, y: *mut &'b i32) {
   |                --  -- lifetime `'b` defined here
   |                |
   |                lifetime `'a` defined here
LL |     x == y;
   |          ^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable pointer to &i32
   = note: mutable pointers are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:18:5
   |
   |
LL | fn compare_fn_ptr<'a, 'b, 'c>(f: fn(&'c mut &'a i32), g: fn(&'c mut &'b i32)) {
   |                   --  -- lifetime `'b` defined here
   |                   |
   |                   lifetime `'a` defined here
LL |     f == g;
   |     ^ requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to &i32
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/type-check-pointer-comparisons.rs:18:10
   |
   |
LL | fn compare_fn_ptr<'a, 'b, 'c>(f: fn(&'c mut &'a i32), g: fn(&'c mut &'b i32)) {
   |                   --  -- lifetime `'b` defined here
   |                   |
   |                   lifetime `'a` defined here
LL |     f == g;
   |          ^ requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference to &i32
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance

help: `'a` and `'b` must be the same: replace one with the other
error: aborting due to 6 previous errors


------------------------------------------
---
test result: FAILED. 12105 passed; 2 failed; 115 ignored; 0 measured; 0 filtered out; finished in 134.85s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:25
