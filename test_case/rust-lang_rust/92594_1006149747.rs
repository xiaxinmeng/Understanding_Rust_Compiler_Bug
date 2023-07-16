plain
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 230 tests
..........i.ii....ii................................................................................ 100/230
.................i...................iiiiiii...F..i...................iii........................... 200/230
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [run-make] run-make-fulldeps/pretty-expanded stdout ----


error: make failed
status: exit status: 2
command: "make"
stdout:
------------------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-expanded/pretty-expanded:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-expanded/pretty-expanded -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-expanded/pretty-expanded  -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/pretty-expanded/pretty-expanded/input.expanded.rs -Zunpretty=expanded input.rs
------------------------------------------
stderr:
------------------------------------------
error: an inner attribute is not permitted following an outer attribute
error: an inner attribute is not permitted following an outer attribute
 --> input.rs:4:1
  |
1 | #[crate_type="lib"]
  | ------------------- previous outer attribute
4 | #![feature(rustc_encodable_decodable)]
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not permitted following an outer attribute
...
7 | extern crate rustc_serialize;
7 | extern crate rustc_serialize;
  | ----------------------------- the inner attribute doesn't annotate this extern crate
  |
  = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
help: to annotate the extern crate, change the attribute from inner to outer style
4 - #![feature(rustc_encodable_decodable)]
4 - #![feature(rustc_encodable_decodable)]
4 + #[feature(rustc_encodable_decodable)]


error[E0658]: use of unstable library feature 'rustc_encodable_decodable': RustcDecodable cannot be used stably
  |
  |
9 | #[derive(RustcEncodable)] pub struct A;
  |
  = help: add `#![feature(rustc_encodable_decodable)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'rustc_encodable_decodable': RustcDecodable cannot be used stably
   |
   |
10 | #[derive(RustcEncodable)] pub struct B(isize);
   |
   = help: add `#![feature(rustc_encodable_decodable)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'rustc_encodable_decodable': RustcDecodable cannot be used stably
   |
   |
11 | #[derive(RustcEncodable)] pub struct C { x: isize }
   |
   = help: add `#![feature(rustc_encodable_decodable)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'rustc_encodable_decodable': RustcDecodable cannot be used stably
   |
   |
12 | #[derive(RustcEncodable)] pub enum D {}
   |
   = help: add `#![feature(rustc_encodable_decodable)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'rustc_encodable_decodable': RustcDecodable cannot be used stably
   |
   |
13 | #[derive(RustcEncodable)] pub enum E { y }
   |
   = help: add `#![feature(rustc_encodable_decodable)]` to the crate attributes to enable


error[E0658]: use of unstable library feature 'rustc_encodable_decodable': RustcDecodable cannot be used stably
   |
   |
14 | #[derive(RustcEncodable)] pub enum F { z(isize) }
   |
   = help: add `#![feature(rustc_encodable_decodable)]` to the crate attributes to enable

warning: ignoring --out-dir flag due to -o flag
warning: ignoring --out-dir flag due to -o flag

warning: use of deprecated macro `RustcEncodable`: rustc-serialize is deprecated and no longer supported
  |
  |
9 | #[derive(RustcEncodable)] pub struct A;
  |
  = note: `#[warn(deprecated)]` on by default


warning: use of deprecated macro `RustcEncodable`: rustc-serialize is deprecated and no longer supported
   |
   |
10 | #[derive(RustcEncodable)] pub struct B(isize);


warning: use of deprecated macro `RustcEncodable`: rustc-serialize is deprecated and no longer supported
   |
   |
11 | #[derive(RustcEncodable)] pub struct C { x: isize }


warning: use of deprecated macro `RustcEncodable`: rustc-serialize is deprecated and no longer supported
   |
   |
12 | #[derive(RustcEncodable)] pub enum D {}


warning: use of deprecated macro `RustcEncodable`: rustc-serialize is deprecated and no longer supported
   |
   |
13 | #[derive(RustcEncodable)] pub enum E { y }


warning: use of deprecated macro `RustcEncodable`: rustc-serialize is deprecated and no longer supported
   |
   |
14 | #[derive(RustcEncodable)] pub enum F { z(isize) }


warning: variant `y` should have an upper camel case name
   |
   |
13 | #[derive(RustcEncodable)] pub enum E { y }
   |                                        ^ help: convert the identifier to upper camel case (notice the capitalization): `Y`
   = note: `#[warn(non_camel_case_types)]` on by default


warning: variant `z` should have an upper camel case name
   |
   |
14 | #[derive(RustcEncodable)] pub enum F { z(isize) }
   |                                        ^ help: convert the identifier to upper camel case (notice the capitalization): `Z`
error: aborting due to 7 previous errors; 9 warnings emitted

For more information about this error, try `rustc --explain E0658`.
For more information about this error, try `rustc --explain E0658`.
make: *** [Makefile:4: all] Error 1
------------------------------------------




failures:
    [run-make] run-make-fulldeps/pretty-expanded

test result: FAILED. 210 passed; 1 failed; 19 ignored; 0 measured; 0 filtered out; finished in 31.11s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--rustdoc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "--rust-demangler-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" "--src-base" "/checkout/src/test/run-make-fulldeps" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "run-make-fulldeps" "--mode" "run-make" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--llvm-bin-dir" "/usr/lib/llvm-12/bin" "--cc" "cc" "--cxx" "c++" "--cflags" "-ffunction-sections -fdata-sections -fPIC -m64" "--ar" "ar" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:32:20
