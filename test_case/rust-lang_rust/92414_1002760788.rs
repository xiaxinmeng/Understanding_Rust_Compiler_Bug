plain
Suite("src/test/pretty") not skipped for "bootstrap::test::Pretty" -- not in ["src/tools/tidy"]
Check compiletest suite=pretty mode=pretty (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 71 tests
............................................2021-12-29T20:15:01.073353Z ERROR compiletest::runtest: fatal error, panic: "pretty-printed source does not match expected source\nexpected:\n------------------------------------------\n// Check that nested items have their visibility and `default`nesses in the right order.\n\n// pp-exact\n\nfn main() {}\n\n#[cfg(FALSE)]\nextern \"C\" {\n    static X: u8 ;\n    type X;\n    fn foo();\n    pub static X: u8 ;\n    pub type X;\n    pub fn foo();\n}\n\n#[cfg(FALSE)]\ntrait T {\n    const X: u8 ;\n    type X;\n    fn foo();\n    default const X: u8 ;\n    default type X;\n    default fn foo();\n    pub const X: u8 ;\n    pub type X;\n    pub fn foo();\n    pub default const X: u8 ;\n    pub default type X;\n    pub default fn foo();\n}\n\n#[cfg(FALSE)]\nimpl T for S {\n    const X: u8 ;\n    type X;\n    fn foo();\n    default const X: u8 ;\n    default type X;\n    default fn foo();\n    pub const X: u8 ;\n    pub type X;\n    pub fn foo();\n    pub default const X: u8 ;\n    pub default type X;\n    pub default fn foo();\n}\n\n------------------------------------------\nactual:\n------------------------------------------\n// Check that nested items have their visibility and `default`nesses in the right order.\n\n// pp-exact\n\nfn main() {}\n\n#[cfg(FALSE)]\nextern \"C\" {\n    static X: u8;\n    type X;\n    fn foo();\n    pub static X: u8;\n    pub type X;\n    pub fn foo();\n}\n\n#[cfg(FALSE)]\ntrait T {\n    const X: u8;\n    type X;\n    fn foo();\n    default const X: u8;\n    default type X;\n    default fn foo();\n    pub const X: u8;\n    pub type X;\n    pub fn foo();\n    pub default const X: u8;\n    pub default type X;\n    pub default fn foo();\n}\n\n#[cfg(FALSE)]\nimpl T for S {\n    const X: u8;\n    type X;\n    fn foo();\n    default const X: u8;\n    default type X;\n    default fn foo();\n    pub const X: u8;\n    pub type X;\n    pub fn foo();\n    pub default const X: u8;\n    pub default type X;\n    pub default fn foo();\n}\n\n------------------------------------------\ndiff:\n------------------------------------------\n6\t\n7\t#[cfg(FALSE)]\n8\textern \"C\" {\n-\t    static X: u8 ;\n+\t    static X: u8;\n10\t    type X;\n11\t    fn foo();\n-\t    pub static X: u8 ;\n+\t    pub static X: u8;\n13\t    pub type X;\n14\t    pub fn foo();\n15\t}\n\n16\t\n17\t#[cfg(FALSE)]\n18\ttrait T {\n-\t    const X: u8 ;\n+\t    const X: u8;\n20\t    type X;\n21\t    fn foo();\n-\t    default const X: u8 ;\n+\t    default const X: u8;\n23\t    default type X;\n24\t    default fn foo();\n-\t    pub const X: u8 ;\n+\t    pub const X: u8;\n26\t    pub type X;\n27\t    pub fn foo();\n-\t    pub default const X: u8 ;\n+\t    pub default const X: u8;\n29\t    pub default type X;\n30\t    pub default fn foo();\n31\t}\n\n32\t\n33\t#[cfg(FALSE)]\n34\timpl T for S {\n-\t    const X: u8 ;\n+\t    const X: u8;\n36\t    type X;\n37\t    fn foo();\n-\t    default const X: u8 ;\n+\t    default const X: u8;\n39\t    default type X;\n40\t    default fn foo();\n-\t    pub const X: u8 ;\n+\t    pub const X: u8;\n42\t    pub type X;\n43\t    pub fn foo();\n-\t    pub default const X: u8 ;\n+\t    pub default const X: u8;\n45\t    pub default type X;\n46\t    pub default fn foo();\n47\t}\n\n\n"
F..........................
Some tests failed in compiletest suite=pretty mode=pretty host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

---- [pretty] pretty/nested-item-vis-defaultness.rs stdout ----


error: pretty-printed source does not match expected source
expected:
------------------------------------------
// Check that nested items have their visibility and `default`nesses in the right order.

// pp-exact
fn main() {}

#[cfg(FALSE)]
extern "C" {
extern "C" {
    static X: u8 ;
    type X;
    fn foo();
    pub static X: u8 ;
    pub type X;
    pub fn foo();

#[cfg(FALSE)]
trait T {
    const X: u8 ;
---
    pub default type X;
    pub default fn foo();
}

#[cfg(FALSE)]
impl T for S {
    const X: u8 ;
    type X;
    fn foo();
    default const X: u8 ;
    default type X;
    default fn foo();
    pub const X: u8 ;
    pub type X;
    pub fn foo();
    pub default const X: u8 ;
    pub default type X;
    pub default fn foo();

------------------------------------------
actual:
------------------------------------------
------------------------------------------
// Check that nested items have their visibility and `default`nesses in the right order.

// pp-exact
fn main() {}

#[cfg(FALSE)]
extern "C" {
---
    pub default type X;
    pub default fn foo();
}

#[cfg(FALSE)]
impl T for S {
    const X: u8;
    type X;
    fn foo();
    default const X: u8;
    default type X;
    default fn foo();
    pub const X: u8;
    pub type X;
    pub fn foo();
    pub default const X: u8;
    pub default type X;
    pub default fn foo();

------------------------------------------
diff:
------------------------------------------
------------------------------------------
6 
7 #[cfg(FALSE)]
8 extern "C" {
-     static X: u8 ;
+     static X: u8;
10     type X;
11     fn foo();
-     pub static X: u8 ;
+     pub static X: u8;
13     pub type X;
14     pub fn foo();

16 
17 #[cfg(FALSE)]
18 trait T {
---
30     pub default fn foo();
31 }

32 
33 #[cfg(FALSE)]
34 impl T for S {
-     const X: u8 ;
+     const X: u8;
36     type X;
37     fn foo();
-     default const X: u8 ;
+     default const X: u8;
39     default type X;
40     default fn foo();
-     pub const X: u8 ;
+     pub const X: u8;
42     pub type X;
43     pub fn foo();
-     pub default const X: u8 ;
+     pub default const X: u8;
45     pub default type X;
46     pub default fn foo();




thread '[pretty] pretty/nested-item-vis-defaultness.rs' panicked at 'fatal error', src/tools/compiletest/src/runtest.rs:2086:9


failures:
    [pretty] pretty/nested-item-vis-defaultness.rs
    [pretty] pretty/nested-item-vis-defaultness.rs

test result: FAILED. 70 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.58s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/pretty" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/pretty" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "pretty" "--mode" "pretty" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:20
