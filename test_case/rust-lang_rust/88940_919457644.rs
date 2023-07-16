plain
   Compiling itertools v0.9.0
[RUSTC-TIMING] build_helper test:false 0.355
[RUSTC-TIMING] lock_api test:false 0.270
   Compiling getopts v0.2.21
   Compiling unic-char-property v0.9.0
[RUSTC-TIMING] unic_char_property test:false 0.085
   Compiling unic-ucd-version v0.9.0
[RUSTC-TIMING] unic_ucd_version test:false 0.039
[RUSTC-TIMING] fixedbitset test:false 0.659
   Compiling crossbeam-utils v0.7.2
[RUSTC-TIMING] ansi_term test:false 0.900
   Compiling memoffset v0.5.5
---
[RUSTC-TIMING] build_script_build test:false 0.275
[RUSTC-TIMING] unicode_script test:false 1.048
[RUSTC-TIMING] bitflags test:false 0.045
[RUSTC-TIMING] maybe_uninit test:false 0.040
   Compiling unic-emoji-char v0.9.0
   Compiling psm v0.1.16
[RUSTC-TIMING] build_script_build test:false 0.310
   Compiling stacker v0.1.14
[RUSTC-TIMING] build_script_build test:false 0.309
[RUSTC-TIMING] build_script_build test:false 0.309
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
[RUSTC-TIMING] log test:false 0.495
[RUSTC-TIMING] build_script_build test:false 0.380
[RUSTC-TIMING] build_script_build test:false 0.438
[RUSTC-TIMING] unic_emoji_char test:false 0.560
[RUSTC-TIMING] tracing_core test:false 1.656
   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
[RUSTC-TIMING] sharded_slab test:false 1.629
[RUSTC-TIMING] itertools test:false 1.556
---
   Compiling itertools v0.9.0
[RUSTC-TIMING] build_helper test:false 0.519
   Compiling getopts v0.2.21
[RUSTC-TIMING] lock_api test:false 0.491
   Compiling unic-ucd-version v0.9.0
[RUSTC-TIMING] unic_ucd_version test:false 0.042
   Compiling unic-char-property v0.9.0
   Compiling indexmap v1.7.0
[RUSTC-TIMING] unic_char_property test:false 0.116
   Compiling crossbeam-utils v0.7.2
[RUSTC-TIMING] termcolor test:false 1.598
---
[RUSTC-TIMING] maybe_uninit test:false 0.036
[RUSTC-TIMING] build_script_build test:false 0.229
[RUSTC-TIMING] build_script_build test:false 0.317
[RUSTC-TIMING] bitflags test:false 0.044
   Compiling unic-emoji-char v0.9.0
[RUSTC-TIMING] cc test:false 2.765
[RUSTC-TIMING] unicode_script test:false 1.721
[RUSTC-TIMING] log test:false 0.691
[RUSTC-TIMING] log test:false 0.691
[RUSTC-TIMING] unic_emoji_char test:false 0.741
[RUSTC-TIMING] crc32fast test:false 0.522
   Compiling rustc_apfloat v0.0.0 (/checkout/compiler/rustc_apfloat)
[RUSTC-TIMING] tracing_core test:false 2.372
   Compiling psm v0.1.16
---

---- [ui] ui/traits/item-privacy.rs stdout ----
diff of stderr:

140    |               - this trait cannot be made into an object...
141 LL |         const C: u8 = 0;
142    |               ^ ...because it contains this associated `const`
-    = help: consider moving `B` to another trait
144    = help: consider moving `A` to another trait
+    = help: consider moving `B` to another trait
145    = help: consider moving `C` to another trait
147 error[E0223]: ambiguous associated type


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/item-privacy/item-privacy.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/item-privacy.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/item-privacy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/item-privacy" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/item-privacy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: no method named `a` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- method `a` not found for this
...
LL |     S.a(); //~ ERROR no method named `a` found
   |       ^ method not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `method::A` defines an item `a`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no method named `b` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- method `b` not found for this
LL |         fn b(&self) { }
LL |         fn b(&self) { }
   |            - the method is available for `S` here
...
LL |     S.b(); //~ ERROR no method named `b` found
   |       ^ method not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use method::B;
LL | use method::B;
   |

error[E0624]: associated function `a` is private
   |
LL |         fn a(&self) { }
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |         ----------- private associated function defined here
   |         ----------- private associated function defined here
...
LL |     c.a(); //~ ERROR associated function `a` is private
   |       ^ private associated function

error[E0599]: no function or associated item named `a` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- function or associated item `a` not found for this
...
LL |     S::a(&S);
   |        ^ function or associated item not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `method::A` defines an item `a`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no function or associated item named `b` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- function or associated item `b` not found for this
...
LL |     S::b(&S);
   |        ^ function or associated item not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use method::B;
LL | use method::B;
   |

error[E0624]: associated function `a` is private
   |
LL |         fn a(&self) { }
   |         ----------- private associated function defined here
...
...
LL |     <dyn C>::a(&S); //~ ERROR associated function `a` is private
   |              ^ private associated function

error[E0599]: no associated item named `A` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- associated item `A` not found for this
...
LL |     S::A; //~ ERROR no associated item named `A` found
   |        ^ associated item not found in `S`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `assoc_const::A` defines an item `A`, perhaps you need to implement it
   |
LL |     trait A {
   |     ^^^^^^^


error[E0599]: no associated item named `B` found for struct `S` in the current scope
   |
LL | struct S;
LL | struct S;
   | --------- associated item `B` not found for this
...
LL |     S::B; //~ ERROR no associated item named `B` found
   |        ^ associated item not found in `S`
   = help: items from traits can only be used if the trait is in scope
help: the following trait is implemented but not in scope; perhaps add a `use` for it:
   |
LL | use assoc_const::B;
---
   |
LL |         const A: u8 = 0;
   |         ---------------- private associated constant defined here
...
LL |     <dyn C>::A; //~ ERROR associated constant `A` is private
   |              ^ private associated constant

error[E0038]: the trait `assoc_const::C` cannot be made into an object
   |
   |
LL |     <dyn C>::A; //~ ERROR associated constant `A` is private
   |      ^^^^^ `assoc_const::C` cannot be made into an object
note: for a trait to be "object safe" it needs to allow building a vtable to allow the call to be resolvable dynamically; for more information visit <https://doc.rust-lang.org/reference/items/traits.html#object-safety>
  --> /checkout/src/test/ui/traits/item-privacy.rs:25:15
   |
LL |         const A: u8 = 0;
LL |         const A: u8 = 0;
   |               ^ ...because it contains this associated `const`
LL |         const B: u8 = 0;
LL |         const B: u8 = 0;
   |               ^ ...because it contains this associated `const`
...
LL |     pub trait C: A + B {
   |               - this trait cannot be made into an object...
LL |         const C: u8 = 0;
   |               ^ ...because it contains this associated `const`
   = help: consider moving `A` to another trait
   = help: consider moving `B` to another trait
   = help: consider moving `C` to another trait
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/item-privacy.rs:115:12
   |
   |
LL |     let _: S::A; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::A`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/item-privacy.rs:116:12
   |
   |
LL |     let _: S::B; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::B`
error[E0223]: ambiguous associated type
  --> /checkout/src/test/ui/traits/item-privacy.rs:117:12
   |
   |
LL |     let _: S::C; //~ ERROR ambiguous associated type
   |            ^^^^ help: use fully-qualified syntax: `<S as Trait>::C`

error: associated type `A` is private
   |
   |
LL |     let _: T::A; //~ ERROR associated type `A` is private


error: associated type `A` is private
   |
   |
LL |         A = u8, //~ ERROR associated type `A` is private

error: aborting due to 15 previous errors

Some errors have detailed explanations: E0038, E0223, E0599, E0624.
---
test result: FAILED. 12032 passed; 1 failed; 102 ignored; 0 measured; 0 filtered out; finished in 133.67s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:11
