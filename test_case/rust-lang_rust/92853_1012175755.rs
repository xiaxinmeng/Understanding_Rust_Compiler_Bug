plain
...................................................i................................................ 6100/12546
.................................................................................................... 6200/12546
................................i................................................................... 6300/12546
.................................................................................................i.. 6400/12546
................................................ii.ii........i..Fi.................................. 6500/12546
..................................i....i........................................i................i.. 6700/12546
...........i....................................................i................................... 6800/12546
....................................................................i............................... 6900/12546
.................................................................................................... 7000/12546
---

---- [ui] ui/iterators/into-iter-on-arrays-lint.rs stdout ----
diff of stderr:

1 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:10:11
3    |
4 LL |     small.into_iter();
5    |           ^^^^^^^^^


17    |     ++++++++++++++++++++++++     ~
18 
19 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:13:12
21    |
21    |
22 LL |     [1, 2].into_iter();

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
34    |     ++++++++++++++++++++++++      ~
35 
35 
36 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:16:9
38    |
39 LL |     big.into_iter();
40    |         ^^^^^^^^^


51    |     ++++++++++++++++++++++++   ~
52 
53 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:19:15
55    |
55    |
56 LL |     [0u8; 33].into_iter();

68    |     ++++++++++++++++++++++++         ~
69 
69 
70 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:23:21
72    |
72    |
73 LL |     Box::new(small).into_iter();
74    |                     ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
77    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
78 
78 
79 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:26:22
81    |
81    |
82 LL |     Box::new([1, 2]).into_iter();
83    |                      ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
86    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
87 
87 
88 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:29:19
90    |
90    |
91 LL |     Box::new(big).into_iter();
92    |                   ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
95    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
96 
96 
97 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:32:25
99    |
99    |
100 LL |     Box::new([0u8; 33]).into_iter();
101    |                         ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
104    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
105 
105 
106 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:36:31
108    |
108    |
109 LL |     Box::new(Box::new(small)).into_iter();
110    |                               ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
113    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
114 
114 
115 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:39:32
117    |
117    |
118 LL |     Box::new(Box::new([1, 2])).into_iter();
119    |                                ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
122    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
123 
123 
124 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:42:29
126    |
126    |
127 LL |     Box::new(Box::new(big)).into_iter();
128    |                             ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
131    = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
132 
132 
133 warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
-   --> $DIR/into-iter-on-arrays-lint.rs:45:35
135    |
135    |
136 LL |     Box::new(Box::new([0u8; 33])).into_iter();
137    |                                   ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/into-iter-on-arrays-lint.stderr
diff of fixed:
diff of fixed:

2 // run-rustfix
3 // rustfix-only-machine-applicable
4 
+ #[allow(unused_must_use)]
5 fn main() {
6     let small = [1, 2];
7     let big = [0u8; 33];

The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/into-iter-on-arrays-lint.fixed
To only update this specific test, also pass `--test-args iterators/into-iter-on-arrays-lint.rs`

error: 2 errors occurred comparing output.
status: exit status: 0
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/iterators/into-iter-on-arrays-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/iterators/into-iter-on-arrays-lint/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
LL |     small.into_iter();
   |           ^^^^^^^^^
   |
   |
   = note: `#[warn(array_into_iter)]` on by default
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
LL |     small.iter();
   |           ~~~~
   |           ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
LL |     IntoIterator::into_iter(small);
   |     ++++++++++++++++++++++++     ~


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     [1, 2].into_iter();
   |
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     [1, 2].iter();
   |            ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
LL |     IntoIterator::into_iter([1, 2]);
   |     ++++++++++++++++++++++++      ~


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
LL |     big.into_iter();
   |         ^^^^^^^^^
   |
   |
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     big.iter();
   |         ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
LL |     IntoIterator::into_iter(big);
   |     ++++++++++++++++++++++++   ~


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     [0u8; 33].into_iter();
   |
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>
help: use `.iter()` instead of `.into_iter()` to avoid ambiguity
   |
LL |     [0u8; 33].iter();
   |               ~~~~
help: or use `IntoIterator::into_iter(..)` instead of `.into_iter()` to explicitly iterate by value
   |
LL |     IntoIterator::into_iter([0u8; 33]);
   |     ++++++++++++++++++++++++         ~

warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(small).into_iter();
   |                     ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new([1, 2]).into_iter();
   |                      ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(big).into_iter();
   |                   ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new([0u8; 33]).into_iter();
   |                         ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new(small)).into_iter();
   |                               ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new([1, 2])).into_iter();
   |                                ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new(big)).into_iter();
   |                             ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>


warning: this method call resolves to `<&[T; N] as IntoIterator>::into_iter` (due to backwards compatibility), but will resolve to <[T; N] as IntoIterator>::into_iter in Rust 2021
   |
   |
LL |     Box::new(Box::new([0u8; 33])).into_iter();
   |                                   ^^^^^^^^^ help: use `.iter()` instead of `.into_iter()` to avoid ambiguity: `iter`
   = warning: this changes meaning in Rust 2021
   = note: for more information, see <https://doc.rust-lang.org/nightly/edition-guide/rust-2021/IntoIterator-for-arrays.html>

warning: 12 warnings emitted
---
test result: FAILED. 12426 passed; 1 failed; 119 ignored; 0 measured; 0 filtered out; finished in 149.57s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:03
