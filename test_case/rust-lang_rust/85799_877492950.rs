plain

---- [ui] ui/trait-bounds/unsized-bound.rs stdout ----
diff of stderr:

14    |             ^ required by this bound in `Trait`
15 help: consider removing the `?Sized` bound to make the type parameter `Sized`
16    |
- LL | impl<A, B> Trait<(A, B)> for (A, B) where A: ?Sized, {}
-    |                                                   --
+ LL | impl<A, B, {}
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |         --
19 help: consider relaxing the implicit `Sized` restriction
20    |
21 LL | trait Trait<A: ?Sized> {}

32    = note: only the last element of a tuple may have a dynamically sized type
33 help: consider removing the `?Sized` bound to make the type parameter `Sized`
34    |
- LL | impl<A, B> Trait<(A, B)> for (A, B) where B: ?Sized, {}
-    |                                          --
+ LL | impl<A, B: ?Sized, {}
37 
38 error[E0277]: the size for values of type `C` cannot be known at compilation time
39   --> $DIR/unsized-bound.rs:5:31


69    = note: only the last element of a tuple may have a dynamically sized type
70 help: consider removing the `?Sized` bound to make the type parameter `Sized`
71    |
- LL | impl<A, B: ?Sized, C: ?Sized> Trait<(A, B, C)> for (A, B, C)  {}
-    |                                                             --
+ LL | impl<A, {}
74 
75 error[E0277]: the size for values of type `B` cannot be known at compilation time
76   --> $DIR/unsized-bound.rs:5:52


138    |              ^ required by this bound in `Trait3`
139 help: consider removing the `?Sized` bound to make the type parameter `Sized`
140    |
- LL | impl<A> Trait3<A> for A  {}
-    |                        --
+ LL | impl<A {}
+    |      --
143 help: consider relaxing the implicit `Sized` restriction
144    |
145 LL | trait Trait3<A: ?Sized> {}

182    |              ^ required by this bound in `Trait5`
183 help: consider removing the `?Sized` bound to make the type parameter `Sized`
184    |
- LL | impl<X, Y> Trait5<X, Y> for X  {}
-    |                              --
+ LL | impl<X {}
+    |      --
187 help: consider relaxing the implicit `Sized` restriction
188    |
189 LL | trait Trait5<A: ?Sized, B> {}

226    |                 ^ required by this bound in `Trait7`
227 help: consider removing the `?Sized` bound to make the type parameter `Sized`
228    |
- LL | impl<X, Y> Trait7<X, Y> for X  {}
-    |                              --
+ LL | impl<X, Y {}
+    |         --
231 help: consider relaxing the implicit `Sized` restriction
232    |
233 LL | trait Trait7<A, B: ?Sized> {}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/unsized-bound/unsized-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args trait-bounds/unsized-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/trait-bounds/unsized-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/unsized-bound" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/trait-bounds/unsized-bound/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the size for values of type `B` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:2:12
   |
LL | impl<A, B> Trait<(A, B)> for (A, B) where A: ?Sized, B: ?Sized, {}
   |         -  ^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |         this type parameter needs to be `std::marker::Sized`
   |
   = note: required because it appears within the type `(A, B)`
note: type parameters have an implicit `Sized` obligation
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:1:13
   |
LL | trait Trait<A> {}
   |             ^ required by this bound in `Trait`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A, B, {}
   |         --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait<A: ?Sized> {}


error[E0277]: the size for values of type `A` cannot be known at compilation time
   |
   |
LL | impl<A, B> Trait<(A, B)> for (A, B) where A: ?Sized, B: ?Sized, {}
   |      -                       ^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
   |
   = note: only the last element of a tuple may have a dynamically sized type
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A, B: ?Sized, {}

error[E0277]: the size for values of type `C` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:5:31
   |
   |
LL | impl<A, B: ?Sized, C: ?Sized> Trait<(A, B, C)> for (A, B, C) where A: ?Sized, {}
   |                    -          ^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |                    this type parameter needs to be `std::marker::Sized`
   |
   = note: required because it appears within the type `(A, B, C)`
note: type parameters have an implicit `Sized` obligation
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:1:13
   |
LL | trait Trait<A> {}
   |             ^ required by this bound in `Trait`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A, B: ?Sized, C> Trait<(A, B, C)> for (A, B, C) where A: ?Sized, {}
   |                    --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait<A: ?Sized> {}


error[E0277]: the size for values of type `A` cannot be known at compilation time
   |
   |
LL | impl<A, B: ?Sized, C: ?Sized> Trait<(A, B, C)> for (A, B, C) where A: ?Sized, {}
   |      -                                             ^^^^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
   |
   = note: only the last element of a tuple may have a dynamically sized type
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A, {}

error[E0277]: the size for values of type `B` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:5:52
   |
   |
LL | impl<A, B: ?Sized, C: ?Sized> Trait<(A, B, C)> for (A, B, C) where A: ?Sized, {}
   |         -                                          ^^^^^^^^^ doesn't have a size known at compile-time
   |         this type parameter needs to be `std::marker::Sized`
   |
   |
   = note: only the last element of a tuple may have a dynamically sized type
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A, B, C: ?Sized> Trait<(A, B, C)> for (A, B, C) where A: ?Sized, {}

error[E0277]: the size for values of type `B` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:10:28
   |
   |
LL | impl<A: ?Sized, B: ?Sized> Trait2<(A, B)> for (A, B) {}
   |                 -          ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |                 this type parameter needs to be `std::marker::Sized`
   |
   = note: required because it appears within the type `(A, B)`
note: type parameters have an implicit `Sized` obligation
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:9:14
   |
LL | trait Trait2<A> {}
   |              ^ required by this bound in `Trait2`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A: ?Sized, B> Trait2<(A, B)> for (A, B) {}
   |                 --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait2<A: ?Sized> {}


error[E0277]: the size for values of type `A` cannot be known at compilation time
   |
   |
LL | impl<A: ?Sized, B: ?Sized> Trait2<(A, B)> for (A, B) {}
   |      -                                        ^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
   |
   = note: only the last element of a tuple may have a dynamically sized type
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A, B: ?Sized> Trait2<(A, B)> for (A, B) {}


error[E0277]: the size for values of type `A` cannot be known at compilation time
   |
   |
LL | impl<A> Trait3<A> for A where A: ?Sized {}
   |      -  ^^^^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:13:14
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:13:14
   |
LL | trait Trait3<A> {}
   |              ^ required by this bound in `Trait3`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A {}
   |      --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait3<A: ?Sized> {}


error[E0277]: the size for values of type `A` cannot be known at compilation time
   |
   |
LL | impl<A: ?Sized> Trait4<A> for A {}
   |      -          ^^^^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:16:14
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:16:14
   |
LL | trait Trait4<A> {}
   |              ^ required by this bound in `Trait4`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<A> Trait4<A> for A {}
   |      --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait4<A: ?Sized> {}

error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:20:12
   |
   |
LL | impl<X, Y> Trait5<X, Y> for X where X: ?Sized {}
   |      -     ^^^^^^^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:19:14
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:19:14
   |
LL | trait Trait5<A, B> {}
   |              ^ required by this bound in `Trait5`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<X {}
   |      --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait5<A: ?Sized, B> {}

error[E0277]: the size for values of type `X` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:23:20
   |
   |
LL | impl<X: ?Sized, Y> Trait6<X, Y> for X {}
   |      -             ^^^^^^^^^^^^ doesn't have a size known at compile-time
   |      this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:22:14
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:22:14
   |
LL | trait Trait6<A, B> {}
   |              ^ required by this bound in `Trait6`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<X, Y> Trait6<X, Y> for X {}
   |      --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait6<A: ?Sized, B> {}

error[E0277]: the size for values of type `Y` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:26:12
   |
   |
LL | impl<X, Y> Trait7<X, Y> for X where Y: ?Sized {}
   |         -  ^^^^^^^^^^^^ doesn't have a size known at compile-time
   |         this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:25:17
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:25:17
   |
LL | trait Trait7<A, B> {}
   |                 ^ required by this bound in `Trait7`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<X, Y {}
   |         --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait7<A, B: ?Sized> {}

error[E0277]: the size for values of type `Y` cannot be known at compilation time
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:29:20
   |
   |
LL | impl<X, Y: ?Sized> Trait8<X, Y> for X {}
   |         -          ^^^^^^^^^^^^ doesn't have a size known at compile-time
   |         this type parameter needs to be `std::marker::Sized`
   |
note: type parameters have an implicit `Sized` obligation
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:28:17
  --> /checkout/src/test/ui/trait-bounds/unsized-bound.rs:28:17
   |
LL | trait Trait8<A, B> {}
   |                 ^ required by this bound in `Trait8`
help: consider removing the `?Sized` bound to make the type parameter `Sized`
   |
LL | impl<X, Y> Trait8<X, Y> for X {}
   |         --
help: consider relaxing the implicit `Sized` restriction
   |
LL | trait Trait8<A, B: ?Sized> {}

error: aborting due to 13 previous errors

For more information about this error, try `rustc --explain E0277`.
---
test result: FAILED. 11970 passed; 1 failed; 100 ignored; 0 measured; 0 filtered out; finished in 119.60s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:50
