plain

---- [ui (nll)] ui/match/match-ref-mut-invariance.rs stdout ----
diff of stderr:

9    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'b`
10    |
11    = help: consider adding the following bound: `'a: 'b`
+    = note: requirement occurs because of a mutable reference to &i32
+    = note: mutable references are invariant over their type parameter
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
13 error: aborting due to previous error
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-invariance.nll/match-ref-mut-invariance.nll.stderr
To only update this specific test, also pass `--test-args match/match-ref-mut-invariance.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/match-ref-mut-invariance.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-invariance.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-invariance.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/match/match-ref-mut-invariance.rs:10:9
   |
LL | impl<'b> S<'b> {
   |      -- lifetime `'b` defined here
LL |     fn bar<'a>(&'a mut self) -> &'a mut &'a i32 {
   |            -- lifetime `'a` defined here
LL |         match self.0 { ref mut x => x } //~ ERROR mismatched types
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to &i32
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui (nll)] ui/match/match-ref-mut-let-invariance.rs stdout ----
diff of stderr:

10    |         ^ returning this value requires that `'a` must outlive `'b`
11    |
12    = help: consider adding the following bound: `'a: 'b`
+    = note: requirement occurs because of a mutable reference to &i32
+    = note: mutable references are invariant over their type parameter
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
14 error: aborting due to previous error
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-let-invariance.nll/match-ref-mut-let-invariance.nll.stderr
To only update this specific test, also pass `--test-args match/match-ref-mut-let-invariance.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/match/match-ref-mut-let-invariance.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-let-invariance.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/match/match-ref-mut-let-invariance.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/match/match-ref-mut-let-invariance.rs:11:9
   |
LL | impl<'b> S<'b> {
   |      -- lifetime `'b` defined here
LL |     fn bar<'a>(&'a mut self) -> &'a mut &'a i32 {
   |            -- lifetime `'a` defined here
LL |         let ref mut x = self.0;
LL |         x //~ ERROR mismatched types
   |         ^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to &i32
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui (nll)] ui/regions/region-lifetime-bounds-on-fns-where-clause.rs stdout ----
diff of stderr:

23    |     ^^^^^^^ argument requires that `'b` must outlive `'a`
24    |
25    = help: consider adding the following bound: `'b: 'a`
+    = note: requirement occurs because of a mutable reference to &isize
+    = note: mutable references are invariant over their type parameter
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
27 error: higher-ranked subtype error
28   --> $DIR/region-lifetime-bounds-on-fns-where-clause.rs:20:12



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.nll/region-lifetime-bounds-on-fns-where-clause.nll.stderr
To only update this specific test, also pass `--test-args regions/region-lifetime-bounds-on-fns-where-clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs:8:5
   |
LL | fn b<'a, 'b>(x: &mut &'a isize, y: &mut &'b isize) {
   |      --  -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
LL |     // Illegal now because there is no `'b:'a` declaration.
LL |     *x = *y; //~ ERROR E0623
   |     ^^^^^^^ assignment requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs:14:5
   |
   |
LL | fn c<'a,'b>(x: &mut &'a isize, y: &mut &'b isize) {
   |      -- -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
...
LL |     a(x, y); //~ ERROR lifetime mismatch [E0623]
   |     ^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference to &isize
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: higher-ranked subtype error
  --> /checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs:20:12
   |
   |
LL |     let _: fn(&mut &isize, &mut &isize) = a; //~ ERROR mismatched types

error: higher-ranked subtype error
  --> /checkout/src/test/ui/regions/region-lifetime-bounds-on-fns-where-clause.rs:20:12
   |
   |
LL |     let _: fn(&mut &isize, &mut &isize) = a; //~ ERROR mismatched types

error: aborting due to 4 previous errors



------------------------------------------


---- [ui (nll)] ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs stdout ----
diff of stderr:

23    |     ^^^^^^^^^^ argument requires that `'b` must outlive `'a`
24    |
25    = help: consider adding the following bound: `'b: 'a`
+    = note: requirement occurs because of a mutable reference to &isize
+    = note: mutable references are invariant over their type parameter
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
27 error: higher-ranked subtype error
28   --> $DIR/region-multiple-lifetime-bounds-on-fns-where-clause.rs:22:12



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.nll/region-multiple-lifetime-bounds-on-fns-where-clause.nll.stderr
To only update this specific test, also pass `--test-args regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:9:5
   |
LL | fn b<'a, 'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
   |      --  -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
LL |     // Illegal now because there is no `'b:'a` declaration.
LL |     *x = *y; //~ ERROR E0623
   |     ^^^^^^^ assignment requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:16:5
   |
   |
LL | fn c<'a,'b, 'c>(x: &mut &'a isize, y: &mut &'b isize, z: &mut &'c isize) {
   |      -- -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
...
LL |     a(x, y, z); //~ ERROR lifetime mismatch [E0623]
   |     ^^^^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference to &isize
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: higher-ranked subtype error
  --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:22:12
   |
   |
LL |     let _: fn(&mut &isize, &mut &isize, &mut &isize) = a; //~ ERROR E0308

error: higher-ranked subtype error
  --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:22:12
   |
   |
LL |     let _: fn(&mut &isize, &mut &isize, &mut &isize) = a; //~ ERROR E0308

error: higher-ranked subtype error
  --> /checkout/src/test/ui/regions/region-multiple-lifetime-bounds-on-fns-where-clause.rs:22:12
   |
   |
LL |     let _: fn(&mut &isize, &mut &isize, &mut &isize) = a; //~ ERROR E0308

error: aborting due to 5 previous errors



------------------------------------------


---- [ui (nll)] ui/regions/regions-lifetime-bounds-on-fns.rs stdout ----
diff of stderr:

23    |     ^^^^^^^ argument requires that `'b` must outlive `'a`
24    |
25    = help: consider adding the following bound: `'b: 'a`
+    = note: requirement occurs because of a mutable reference to &isize
+    = note: mutable references are invariant over their type parameter
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
27 error: higher-ranked subtype error
28   --> $DIR/regions-lifetime-bounds-on-fns.rs:20:12



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-lifetime-bounds-on-fns.nll/regions-lifetime-bounds-on-fns.nll.stderr
To only update this specific test, also pass `--test-args regions/regions-lifetime-bounds-on-fns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-lifetime-bounds-on-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-lifetime-bounds-on-fns.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-lifetime-bounds-on-fns.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-lifetime-bounds-on-fns.rs:8:5
   |
LL | fn b<'a, 'b>(x: &mut &'a isize, y: &mut &'b isize) {
   |      --  -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
LL |     // Illegal now because there is no `'b:'a` declaration.
LL |     *x = *y; //~ ERROR E0623
   |     ^^^^^^^ assignment requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-lifetime-bounds-on-fns.rs:14:5
   |
   |
LL | fn c<'a,'b>(x: &mut &'a isize, y: &mut &'b isize) {
   |      -- -- lifetime `'b` defined here
   |      |
   |      lifetime `'a` defined here
...
LL |     a(x, y); //~ ERROR lifetime mismatch [E0623]
   |     ^^^^^^^ argument requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference to &isize
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: higher-ranked subtype error
  --> /checkout/src/test/ui/regions/regions-lifetime-bounds-on-fns.rs:20:12
   |
   |
LL |     let _: fn(&mut &isize, &mut &isize) = a; //~ ERROR E0308

error: higher-ranked subtype error
  --> /checkout/src/test/ui/regions/regions-lifetime-bounds-on-fns.rs:20:12
   |
   |
LL |     let _: fn(&mut &isize, &mut &isize) = a; //~ ERROR E0308

error: aborting due to 4 previous errors



------------------------------------------


---- [ui (nll)] ui/regions/regions-trait-object-subtyping.rs stdout ----
diff of stderr:

10    |     ^ returning this value requires that `'a` must outlive `'b`
11    |
12    = help: consider adding the following bound: `'a: 'b`
+    = note: requirement occurs because of a mutable reference to dyn Dummy
+    = note: mutable references are invariant over their type parameter
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
14 error: lifetime may not live long enough
15   --> $DIR/regions-trait-object-subtyping.rs:22:5


23    |     ^ returning this value requires that `'b` must outlive `'a`
24    |
25    = help: consider adding the following bound: `'b: 'a`
+    = note: requirement occurs because of a mutable reference to dyn Dummy
+    = note: mutable references are invariant over their type parameter
+    = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
27 error: aborting due to 2 previous errors
28 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-trait-object-subtyping.nll/regions-trait-object-subtyping.nll.stderr
To only update this specific test, also pass `--test-args regions/regions-trait-object-subtyping.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-trait-object-subtyping.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-trait-object-subtyping.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-trait-object-subtyping.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-trait-object-subtyping.rs:15:5
   |
LL | fn foo3<'a,'b>(x: &'a mut dyn Dummy) -> &'b mut dyn Dummy {
   |         -- -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
LL |     // Without knowing 'a:'b, we can't coerce
LL |     x //~ ERROR lifetime bound not satisfied
   |     ^ returning this value requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
   = note: requirement occurs because of a mutable reference to dyn Dummy
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-trait-object-subtyping.rs:22:5
   |
   |
LL | fn foo4<'a:'b,'b>(x: Wrapper<&'a mut dyn Dummy>) -> Wrapper<&'b mut dyn Dummy> {
   |         --    -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
LL |     // We can't coerce because it is packed in `Wrapper`
LL |     x //~ ERROR mismatched types
   |     ^ returning this value requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
   = note: requirement occurs because of a mutable reference to dyn Dummy
   = note: mutable references are invariant over their type parameter
   = help: see <https://doc.rust-lang.org/nomicon/subtyping.html> for more information about variance
error: aborting due to 2 previous errors


------------------------------------------
---
test result: FAILED. 11827 passed; 6 failed; 124 ignored; 0 measured; 0 filtered out; finished in 96.46s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "12.0.1-rust-1.54.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:20:14
