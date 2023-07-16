plain
+ LL |     Foo() = 1,
+    |             ^
+    |
+    = note: see issue #60553 <https://github.com/rust-lang/rust/issues/60553> for more information
+    = help: add `#![feature(arbitrary_enum_discriminant)]` to the crate attributes to enable
+ error[E0658]: discriminants on non-unit variants are experimental
+   --> $DIR/issue-88621.rs:4:13
+    |
+    |
+ LL |     Bar{} = 2,
+    |
+    = note: see issue #60553 <https://github.com/rust-lang/rust/issues/60553> for more information
+    = note: see issue #60553 <https://github.com/rust-lang/rust/issues/60553> for more information
+    = help: add `#![feature(arbitrary_enum_discriminant)]` to the crate attributes to enable
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
+ error[E0658]: custom discriminant values are not allowed in enums with tuple or struct variants
+    |
+ LL |     Foo() = 1,
+    |     --------- tuple variant defined here
+    |     --------- tuple variant defined here
+ LL |     Bar{} = 2,
+    |     --------- struct variant defined here
+ LL |     Baz = 3,
+    |           ^ disallowed custom discriminant
+    = note: see issue #60553 <https://github.com/rust-lang/rust/issues/60553> for more information
+    = note: see issue #60553 <https://github.com/rust-lang/rust/issues/60553> for more information
+    = help: add `#![feature(arbitrary_enum_discriminant)]` to the crate attributes to enable
+ 
1 error[E0605]: non-primitive cast: `Kind2` as `u8`
3    |


4 LL |     let _ = Kind2::Foo() as u8;
5    |             ^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
- error: aborting due to previous error
+ error: aborting due to 4 previous errors
8 
- For more information about this error, try `rustc --explain E0605`.
---
To only update this specific test, also pass `--test-args cast/issue-88621.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cast/issue-88621.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/issue-88621" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=cc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cast/issue-88621/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
LL |     Foo() = 1,
   |             ^
   |
   = note: see issue #60553 <https://github.com/rust-lang/rust/issues/60553> for more information
   = help: add `#![feature(arbitrary_enum_discriminant)]` to the crate attributes to enable
error[E0658]: discriminants on non-unit variants are experimental
  --> /checkout/src/test/ui/cast/issue-88621.rs:4:13
   |
   |
LL |     Bar{} = 2,
   |
   = note: see issue #60553 <https://github.com/rust-lang/rust/issues/60553> for more information
   = note: see issue #60553 <https://github.com/rust-lang/rust/issues/60553> for more information
   = help: add `#![feature(arbitrary_enum_discriminant)]` to the crate attributes to enable

error[E0658]: custom discriminant values are not allowed in enums with tuple or struct variants
   |
LL |     Foo() = 1,
   |     --------- tuple variant defined here
   |     --------- tuple variant defined here
LL |     Bar{} = 2,
   |     --------- struct variant defined here
LL |     Baz = 3,
   |           ^ disallowed custom discriminant
   = note: see issue #60553 <https://github.com/rust-lang/rust/issues/60553> for more information
   = note: see issue #60553 <https://github.com/rust-lang/rust/issues/60553> for more information
   = help: add `#![feature(arbitrary_enum_discriminant)]` to the crate attributes to enable

error[E0605]: non-primitive cast: `Kind2` as `u8`
   |
   |
LL |     let _ = Kind2::Foo() as u8;
   |             ^^^^^^^^^^^^^^^^^^ an `as` expression can only be used to convert between primitive types or to coerce to a specific trait object
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0605, E0658.
For more information about an error, try `rustc --explain E0605`.
---
test result: FAILED. 12255 passed; 1 failed; 167 ignored; 0 measured; 0 filtered out; finished in 124.79s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/i586-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-i586-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "i586-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--linker" "cc" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "13.0.0-rust-1.59.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:17:35
