plain
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

91   --> $DIR/deref-suggestion.rs:48:20
92    |
93 LL |     let r = R { i: i2 };
-    |                    ^ expected `u32`, found `&{integer}`
+    |                    ^^ expected `u32`, found `&{integer}`
96 help: consider dereferencing the borrow
97    |



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deref-suggestion/deref-suggestion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args deref-suggestion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/deref-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deref-suggestion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/deref-suggestion/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:8:9
   |
LL |     foo(s);
   |         ^- help: try using a conversion method: `.to_string()`
   |         |
   |         expected struct `String`, found `&String`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:14:10
   |
LL |     foo3(u);
LL |     foo3(u);
   |          ^ expected `u32`, found `&u32`
help: consider dereferencing the borrow
   |
   |
LL |     foo3(*u);

error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:30:9
   |
   |
LL |     foo(&"aaa".to_owned());
   |         ^^^^^^^^^^^^^^^^^ expected struct `String`, found `&String`
help: consider removing the borrow
   |
   |
LL |     foo("aaa".to_owned());

error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:32:9
   |
   |
LL |     foo(&mut "aaa".to_owned());
   |         ^^^^^^^^^^^^^^^^^^^^^ expected struct `String`, found `&mut String`
help: consider removing the borrow
   |
   |
LL |     foo("aaa".to_owned());

error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:2:20
   |
   |
LL |     ($x:expr) => { &$x } //~ ERROR mismatched types
   |                    ^^^ expected `u32`, found `&{integer}`
...
LL |     foo3(borrow!(0));
   |
   |
   = note: this error originates in the macro `borrow` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:36:5
   |
   |
LL |     assert_eq!(3i32, &3i32);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ expected `i32`, found `&i32`
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:39:17
   |
   |
LL |     let s = S { u };
   |                 |
   |                 |
   |                 expected `&u32`, found integer
   |                 help: consider borrowing here: `u: &u`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:42:20
   |
   |
LL |     let s = S { u: u2 };
   |                    |
   |                    |
   |                    expected `&u32`, found integer
   |                    help: consider borrowing here: `&u2`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:45:17
   |
   |
LL |     let r = R { i };
   |                 ^ expected `u32`, found `&{integer}`
help: consider dereferencing the borrow
   |
   |
LL |     let r = R { i: *i };

error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:48:20
   |
   |
LL |     let r = R { i: i2 };
   |                    ^^ expected `u32`, found `&{integer}`
help: consider dereferencing the borrow
   |
   |
LL |     let r = R { i: *i2 };

error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:57:9
   |
   |
LL |         b
   |         ^ expected `i32`, found `&{integer}`
help: consider dereferencing the borrow
   |
LL |         *b
   |         ^
   |         ^

error[E0308]: mismatched types
  --> /checkout/src/test/ui/deref-suggestion.rs:65:9
   |
LL |         b
   |         ^ expected `i32`, found `&{integer}`
help: consider dereferencing the borrow
   |
LL |         *b
   |         ^
   |         ^

error[E0308]: `if` and `else` have incompatible types
  --> /checkout/src/test/ui/deref-suggestion.rs:70:12
LL |        let val = if true {
   |   _______________-
LL |  |         *a
   |  |         -- expected because of this
   |  |         -- expected because of this
LL |  |     } else if true {
   |  |____________^
LL | ||     //~^ ERROR incompatible types
LL | ||         b
LL | ||     } else {
LL | ||         &0
LL | ||     };
   | ||_____|
   | ||_____|
   | |______`if` and `else` have incompatible types
   |        expected `i32`, found `&{integer}`
error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0308`.

---
test result: FAILED. 11976 passed; 1 failed; 103 ignored; 0 measured; 0 filtered out; finished in 124.43s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:07
