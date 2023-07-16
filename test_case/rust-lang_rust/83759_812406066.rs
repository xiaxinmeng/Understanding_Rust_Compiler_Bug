plain
.................................................................................................... 9400/11731
.................................................................................................... 9500/11731
.........................................................................i.....i.................... 9600/11731
.................................................................................................... 9700/11731
..................iiiiiii..i.iiiiii................................................................. 9800/11731
.................................................................................................... 10000/11731
.................................................................................................... 10100/11731
.................................................................................................... 10200/11731
.................................................................................................... 10300/11731
---

---- [ui] ui/generics/wrong-number-of-args.rs stdout ----
diff of stderr:

412 LL |         trait GenericLifetimeAT<'a> {
414 
+ error[E0106]: missing lifetime specifier
+   --> $DIR/wrong-number-of-args.rs:163:44
+    |
+    |
+ LL |         type C = Box<dyn GenericLifetimeAT<(), AssocTy=()>>;
+    |                                            ^ expected named lifetime parameter
+    |
+ help: consider introducing a named lifetime parameter
+    |
+ LL |         type C<'a> = Box<dyn GenericLifetimeAT<'a, (), AssocTy=()>>;
+    |               ^^^^                             ^^^
+ 
415 error[E0107]: this trait takes 0 type arguments but 1 type argument was supplied
417    |


426 LL |         trait GenericLifetimeAT<'a> {
428 
- error[E0106]: missing lifetime specifier
-   --> $DIR/wrong-number-of-args.rs:163:44
-    |
-    |
- LL |         type C = Box<dyn GenericLifetimeAT<(), AssocTy=()>>;
-    |                                            ^ expected named lifetime parameter
-    |
- help: consider introducing a named lifetime parameter
-    |
- LL |         type C<'a> = Box<dyn GenericLifetimeAT<'a, (), AssocTy=()>>;
-    |               ^^^^                             ^^^
- 
440 error[E0107]: this trait takes 1 type argument but 0 type arguments were supplied
442    |


497 LL |         type C = Box<dyn GenericTypeAT<'static, A, AssocTy=()>>;
499 
+ error[E0106]: missing lifetime specifier
+   --> $DIR/wrong-number-of-args.rs:195:48
+    |
+    |
+ LL |         type A = Box<dyn GenericLifetimeTypeAT<AssocTy=()>>;
+    |                                                ^ expected named lifetime parameter
+    |
+ help: consider introducing a named lifetime parameter
+    |
+ LL |         type A<'a> = Box<dyn GenericLifetimeTypeAT<'a, AssocTy=()>>;
+    |               ^^^^                                 ^^^
+ 
500 error[E0107]: this trait takes 1 type argument but 0 type arguments were supplied
502    |


513 LL |         type A = Box<dyn GenericLifetimeTypeAT<A, AssocTy=()>>;
515 
- error[E0106]: missing lifetime specifier
-   --> $DIR/wrong-number-of-args.rs:195:48
-    |
-    |
- LL |         type A = Box<dyn GenericLifetimeTypeAT<AssocTy=()>>;
-    |                                                ^ expected named lifetime parameter
-    |
- help: consider introducing a named lifetime parameter
-    |
- LL |         type A<'a> = Box<dyn GenericLifetimeTypeAT<'a, AssocTy=()>>;
-    |               ^^^^                                 ^^^
- 
527 error[E0107]: this trait takes 1 type argument but 0 type arguments were supplied
529    |


581 LL |         type D<'a> = Box<dyn GenericLifetimeTypeAT<'a, (), AssocTy=()>>;
582    |               ^^^^                                 ^^^
+ error[E0106]: missing lifetime specifier
+   --> $DIR/wrong-number-of-args.rs:215:48
+    |
+    |
+ LL |         type E = Box<dyn GenericLifetimeTypeAT<(), (), AssocTy=()>>;
+    |                                                ^ expected named lifetime parameter
+    |
+ help: consider introducing a named lifetime parameter
+    |
+ LL |         type E<'a> = Box<dyn GenericLifetimeTypeAT<'a, (), (), AssocTy=()>>;
+    |               ^^^^                                 ^^^
+ 
584 error[E0107]: this trait takes 1 type argument but 2 type arguments were supplied
586    |


595 LL |         trait GenericLifetimeTypeAT<'a, A> {
596    |               ^^^^^^^^^^^^^^^^^^^^^     -
- error[E0106]: missing lifetime specifier
-   --> $DIR/wrong-number-of-args.rs:215:48
-    |
-    |
- LL |         type E = Box<dyn GenericLifetimeTypeAT<(), (), AssocTy=()>>;
-    |                                                ^ expected named lifetime parameter
-    |
- help: consider introducing a named lifetime parameter
-    |
- LL |         type E<'a> = Box<dyn GenericLifetimeTypeAT<'a, (), (), AssocTy=()>>;
-    |               ^^^^                                 ^^^
- 
609 error[E0107]: this trait takes 1 lifetime argument but 2 lifetime arguments were supplied
611    |


739 LL |         type B = Box<dyn GenericLifetimeLifetimeAT<'static, 'b, AssocTy=()>>;
741 
+ error[E0106]: missing lifetime specifiers
+   --> $DIR/wrong-number-of-args.rs:273:56
+    |
+    |
+ LL |         type A = Box<dyn GenericLifetimeLifetimeTypeAT<AssocTy=()>>;
+    |                                                        ^ expected 2 lifetime parameters
+    |
+ help: consider introducing a named lifetime parameter
+    |
+ LL |         type A<'a> = Box<dyn GenericLifetimeLifetimeTypeAT<'a, 'a, AssocTy=()>>;
+    |               ^^^^                                         ^^^^^^^
+ 
742 error[E0107]: this trait takes 1 type argument but 0 type arguments were supplied
744    |

754    |
754    |
755 LL |         type A = Box<dyn GenericLifetimeLifetimeTypeAT<A, AssocTy=()>>;
- 
- error[E0106]: missing lifetime specifiers
-   --> $DIR/wrong-number-of-args.rs:273:56
-    |
-    |
- LL |         type A = Box<dyn GenericLifetimeLifetimeTypeAT<AssocTy=()>>;
-    |                                                        ^ expected 2 lifetime parameters
-    |
- help: consider introducing a named lifetime parameter
-    |
- LL |         type A<'a> = Box<dyn GenericLifetimeLifetimeTypeAT<'a, 'a, AssocTy=()>>;
-    |               ^^^^                                         ^^^^^^^
768 
769 error[E0107]: this trait takes 2 lifetime arguments but only 1 lifetime argument was supplied


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/wrong-number-of-args/wrong-number-of-args.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/wrong-number-of-args/wrong-number-of-args.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generics/wrong-number-of-args.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generics/wrong-number-of-args.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/wrong-number-of-args" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/wrong-number-of-args/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0107]: this struct takes 0 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     type B = Ty<'static>;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |              ^^--------- help: remove these generics
   |              expected 0 lifetime arguments
   |
   |
note: struct defined here, with 0 lifetime parameters
   |
LL |     struct Ty;
   |            ^^


error[E0107]: this struct takes 0 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     type C = Ty<'static, usize>;
   |              ^^ --------- help: remove this lifetime argument
   |              expected 0 lifetime arguments
   |
   |
note: struct defined here, with 0 lifetime parameters
   |
LL |     struct Ty;
   |            ^^


error[E0107]: this struct takes 0 type arguments but 1 type argument was supplied
   |
   |
LL |     type C = Ty<'static, usize>;
   |              ^^        ------- help: remove this type argument
   |              expected 0 type arguments
   |
   |
note: struct defined here, with 0 type parameters
   |
LL |     struct Ty;
   |            ^^


error[E0107]: this struct takes 0 lifetime arguments but 1 lifetime argument was supplied
   |
   |
LL |     type D = Ty<'static, usize, { 0 }>;
   |              ^^ --------- help: remove this lifetime argument
   |              expected 0 lifetime arguments
   |
   |
note: struct defined here, with 0 lifetime parameters
   |
LL |     struct Ty;
   |            ^^


error[E0107]: this struct takes 0 generic arguments but 2 generic arguments were supplied
   |
   |
LL |     type D = Ty<'static, usize, { 0 }>;
   |              ^^        -------------- help: remove these generic arguments
   |              expected 0 generic arguments
   |
   |
note: struct defined here, with 0 generic parameters
   |
LL |     struct Ty;
   |            ^^


error[E0107]: missing generics for struct `type_and_type::Ty`
  --> /checkout/src/test/ui/generics/wrong-number-of-args.rs:26:14
   |
LL |     type A = Ty;
   |              ^^ expected 2 type arguments
   |
note: struct defined here, with 2 type parameters: `A`, `B`
   |
   |
LL |     struct Ty<A, B>;
   |            ^^ -  -
help: use angle brackets to add missing type arguments
   |
LL |     type A = Ty<A, B>;


error[E0107]: this struct takes 2 type arguments but only 1 type argument was supplied
   |
   |
LL |     type B = Ty<usize>;
   |              ^^ ----- supplied 1 type argument
   |              expected 2 type arguments
   |
   |
note: struct defined here, with 2 type parameters: `A`, `B`
   |
   |
LL |     struct Ty<A, B>;
   |            ^^ -  -
help: add missing type argument
   |
LL |     type B = Ty<usize, B>;


error[E0107]: this struct takes 2 type arguments but 3 type arguments were supplied
   |
   |
LL |     type D = Ty<usize, String, char>;
   |              ^^              ------ help: remove this type argument
   |              expected 2 type arguments
   |
   |
note: struct defined here, with 2 type parameters: `A`, `B`
   |
   |
LL |     struct Ty<A, B>;
   |            ^^ -  -

error[E0107]: this struct takes 2 type arguments but 0 type arguments were supplied
   |
   |
LL |     type E = Ty<>;
   |              ^^ expected 2 type arguments
   |
note: struct defined here, with 2 type parameters: `A`, `B`
   |
   |
LL |     struct Ty<A, B>;
   |            ^^ -  -
help: add missing type arguments
   |
LL |     type E = Ty<A, B>;


error[E0107]: missing generics for struct `lifetime_and_type::Ty`
   |
LL |     type A = Ty;
   |              ^^ expected 1 type argument
   |
   |
note: struct defined here, with 1 type parameter: `T`
   |
   |
LL |     struct Ty<'a, T>;
   |            ^^     -
help: use angle brackets to add missing type argument
   |
LL |     type A = Ty<T>;

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/generics/wrong-number-of-args.rs:48:14
   |
   |
LL |     type A = Ty;
   |              ^^ expected named lifetime parameter
   |
help: consider introducing a named lifetime parameter
   |
LL |     type A<'a> = Ty<'a>;
   |           ^^^^   ^^^^^^

error[E0107]: this struct takes 1 type argument but 0 type arguments were supplied
   |
   |
LL |     type B = Ty<'static>;
   |              ^^ expected 1 type argument
   |
note: struct defined here, with 1 type parameter: `T`
   |
   |
LL |     struct Ty<'a, T>;
   |            ^^     -
help: add missing type argument
   |
LL |     type B = Ty<'static, T>;

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/generics/wrong-number-of-args.rs:58:17
   |
   |
LL |     type C = Ty<usize>;
   |                 ^ expected named lifetime parameter
   |
help: consider introducing a named lifetime parameter
   |
LL |     type C<'a> = Ty<'a, usize>;
   |           ^^^^      ^^^

error[E0107]: this struct takes 1 type argument but 0 type arguments were supplied
   |
   |
LL |     type E = Ty<>;
   |              ^^ expected 1 type argument
   |
note: struct defined here, with 1 type parameter: `T`
   |
   |
LL |     struct Ty<'a, T>;
   |            ^^     -
help: add missing type argument
   |
LL |     type E = Ty<T>;

error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/generics/wrong-number-of-args.rs:64:16
   |
   |
LL |     type E = Ty<>;
   |                ^- expected named lifetime parameter
   |
help: consider introducing a named lifetime parameter
   |
LL |     type E<'a> = Ty<'a>;
   |           ^^^^      ^^
error[E0107]: missing generics for struct `type_and_type_and_type::Ty`
  --> /checkout/src/test/ui/generics/wrong-number-of-args.rs:74:14
   |
LL |     type A = Ty;
LL |     type A = Ty;
   |              ^^ expected at least 2 type arguments
   |
note: struct defined here, with at least 2 type parameters: `A`, `B`
   |
   |
LL |     struct Ty<A, B, C = &'static str>;
   |            ^^ -  -
help: use angle brackets to add missing type arguments
   |
LL |     type A = Ty<A, B>;


error[E0107]: this struct takes at least 2 type arguments but only 1 type argument was supplied
   |
   |
LL |     type B = Ty<usize>;
   |              ^^ ----- supplied 1 type argument
   |              expected at least 2 type arguments
   |
   |
note: struct defined here, with at least 2 type parameters: `A`, `B`
   |
   |
LL |     struct Ty<A, B, C = &'static str>;
   |            ^^ -  -
help: add missing type argument
   |
LL |     type B = Ty<usize, B>;


error[E0107]: this struct takes at most 3 type arguments but 4 type arguments were supplied
   |
   |
LL |     type E = Ty<usize, String, char, f64>;
   |              ^^                    ----- help: remove this type argument
   |              expected at most 3 type arguments
   |
   |
note: struct defined here, with at most 3 type parameters: `A`, `B`, `C`
   |
   |
LL |     struct Ty<A, B, C = &'static str>;
   |            ^^ -  -  -

error[E0107]: this struct takes at least 2 type arguments but 0 type arguments were supplied
   |
LL |     type F = Ty<>;
   |              ^^ expected at least 2 type arguments
   |
   |
note: struct defined here, with at least 2 type parameters: `A`, `B`
   |
   |
LL |     struct Ty<A, B, C = &'static str>;
   |            ^^ -  -
help: add missing type arguments
   |
LL |     type F = Ty<A, B>;


error[E0107]: this trait takes 0 type arguments but 1 type argument was supplied
   |
   |
LL |     type A = Box<dyn NonGeneric<usize>>;
   |                      ^^^^^^^^^^------- help: remove these generics
   |                      expected 0 type arguments
   |
   |
note: trait defined here, with 0 type parameters
   |
LL |     trait NonGeneric {
   |           ^^^^^^^^^^


error[E0106]: missing lifetime specifier
  --> /checkout/src/test/ui/generics/wrong-number-of-args.rs:114:22
   |
LL |     type B = Box<dyn GenericLifetime>;
   |                      ^^^^^^^^^^^^^^^ expected named lifetime parameter
   |
help: consider introducing a named lifetime parameter
   |
LL |     type B<'a> = Box<dyn GenericLifetime<'a>>;
   |           ^^^^           ^^^^^^^^^^^^^^^^^^^

error[E0107]: this trait takes 1 lifetime argument but 2 lifetime arguments were supplied
   |
   |
LL |     type C = Box<dyn GenericLifetime<'static, 'static>>;
   |                      ^^^^^^^^^^^^^^^        --------- help: remove this lifetime argument
   |                      expected 1 lifetime argument
   |
   |
note: trait defined here, with 1 lifetime parameter: `'a`
   |
   |
LL |     trait GenericLifetime<'a> {


error[E0107]: missing generics for trait `GenericType`
   |
   |
LL |     type D = Box<dyn GenericType>;
   |                      ^^^^^^^^^^^ expected 1 type argument
   |
note: trait defined here, with 1 type parameter: `A`
   |
   |
LL |     trait GenericType<A> {
   |           ^^^^^^^^^^^ -
help: use angle brackets to add missing type argument
   |
LL |     type D = Box<dyn GenericType<A>>;


error[E0107]: this trait takes 1 type argument but 2 type arguments were supplied
   |
---
test result: FAILED. 11634 passed; 1 failed; 96 ignored; 0 measured; 0 filtered out; finished in 138.77s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:15
