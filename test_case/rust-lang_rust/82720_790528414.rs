plain
.................................................................................................... 9300/11525
.................................................................................................... 9400/11525
.........................................................................i......i................... 9500/11525
.................................................................................................... 9600/11525
............iiiiiii..iiiiii.i....................................................................... 9700/11525
.................................................................................................... 9900/11525
.................................................................................................... 10000/11525
.................................................................................................... 10100/11525
.................................................................................................... 10200/11525
---
diff of stderr:

85   --> $DIR/typeck_type_placeholder_item.rs:18:15
86    |
87 LL | static TEST5: (_, _) = (1, 2);
-    |               |
-    |               not allowed in type signatures
-    |               not allowed in type signatures
-    |               help: provide a type for the item: `TEST5: [type error]`
+    |               ^^^^^^ not allowed in type signatures
92 
93 error[E0121]: the type placeholder `_` is not allowed within types on item signatures

210   --> $DIR/typeck_type_placeholder_item.rs:76:15
211    |
211    |
212 LL |     static C: Option<_> = Some(42);
-    |               |
-    |               not allowed in type signatures
-    |               not allowed in type signatures
-    |               help: provide a type for the item: `C: [type error]`
+    |               ^^^^^^^^^ not allowed in type signatures
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
218 error[E0121]: the type placeholder `_` is not allowed within types on item signatures

256   --> $DIR/typeck_type_placeholder_item.rs:91:22
257    |
257    |
258 LL |     static FN_TEST5: (_, _) = (1, 2);
-    |                      |
-    |                      not allowed in type signatures
-    |                      not allowed in type signatures
-    |                      help: provide a type for the item: `FN_TEST5: [type error]`
+    |                      ^^^^^^ not allowed in type signatures
263 
264 error[E0121]: the type placeholder `_` is not allowed within types on item signatures

630   --> $DIR/typeck_type_placeholder_item.rs:205:14
631    |
632 LL |     const C: _;
632 LL |     const C: _;
-    |              ^
-    |              |
-    |              not allowed in type signatures
-    |              help: provide a type for the item: `C: [type error]`
+    |              ^ not allowed in type signatures
637 
638 error[E0121]: the type placeholder `_` is not allowed within types on item signatures


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item/typeck_type_placeholder_item.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item/typeck_type_placeholder_item.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/typeck_type_placeholder_item.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/typeck_type_placeholder_item/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:154:18
   |
LL | struct BadStruct<_>(_);
   |                  ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:157:16
   |
   |
LL | trait BadTrait<_> {}
   |                ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:167:19
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                   ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:167:22
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                      ^ expected identifier, found reserved identifier
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:172:19
   |
   |
LL | struct BadStruct2<_, T>(_, T);
   |                   ^ expected identifier, found reserved identifier

error: associated constant in `impl` without body
   |
LL |     const C: _;
   |     ^^^^^^^^^^-
   |               |
   |               |
   |               help: provide a definition for the constant: `= <expr>;`

error[E0403]: the name `_` is already used for a generic parameter in this item's generic parameters
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                   -  ^ already used
   |                   first use of `_`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test() -> _ { 5 }
   |              |
   |              not allowed in type signatures
   |              help: replace with the correct return type: `i32`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test2() -> (_, _) { (5, 5) }
   |               -^--^-
   |               ||  |
   |               ||  not allowed in type signatures
   |               |not allowed in type signatures
   |               help: replace with the correct return type: `(i32, i32)`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static TEST3: _ = "test";
   |               |
   |               not allowed in type signatures
   |               not allowed in type signatures
   |               help: replace `_` with the correct type: `&str`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static TEST4: _ = 145;
   |               |
   |               not allowed in type signatures
   |               not allowed in type signatures
   |               help: replace `_` with the correct type: `i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | static TEST5: (_, _) = (1, 2);
   |               ^^^^^^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test6(_: _) { }
   |             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6<T>(_: T) { }
   |         ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test6_b<T>(_: _, _: T) { }
   |                  ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6_b<T, U>(_: U, _: T) { }
   |             ^^^     ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test6_c<T, K, L, A, B>(_: _, _: (T, K, L, A, B)) { }
   |                              ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test6_c<T, K, L, A, B, U>(_: U, _: (T, K, L, A, B)) { }
   |                         ^^^     ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test7(x: _) { let _x: usize = x; }
   |             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test7<T>(x: T) { let _x: usize = x; }
   |         ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test8(_f: fn() -> _) { }
   |                      |
   |                      not allowed in type signatures
   |                      help: use type parameters instead: `T`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test8(_f: fn() -> _) { }
   |                      ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | fn test8<T>(_f: fn() -> T) { }
   |         ^^^             ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn test11(x: &usize) -> &_ {
   |                         -^
   |                         ||
   |                         |not allowed in type signatures
   |                         help: replace with the correct return type: `&'static &'static usize`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | unsafe fn test12(x: *const usize) -> *const *const _ {
   |                                      |             |
   |                                      |             not allowed in type signatures
   |                                      |             not allowed in type signatures
   |                                      help: replace with the correct return type: `*const *const usize`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
LL |     a: _,
   |        ^ not allowed in type signatures
   |        ^ not allowed in type signatures
LL |     //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |     b: (_, _),
   |         ^  ^ not allowed in type signatures
   |         not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL | struct Test10<T> {
LL |     a: T,
LL |     //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |     b: (T, T),


error: missing type for `static` item
   |
LL |     static A = 42;
LL |     static A = 42;
   |            ^ help: provide a type for the item: `A: i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static B: _ = 42;
   |               |
   |               not allowed in type signatures
   |               not allowed in type signatures
   |               help: replace `_` with the correct type: `i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static C: Option<_> = Some(42);
   |               ^^^^^^^^^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test() -> _ { 5 }
   |                     |
   |                     not allowed in type signatures
   |                     help: replace with the correct return type: `i32`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test2() -> (_, _) { (5, 5) }
   |                      -^--^-
   |                      ||  |
   |                      ||  not allowed in type signatures
   |                      |not allowed in type signatures
   |                      help: replace with the correct return type: `(i32, i32)`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static FN_TEST3: _ = "test";
   |                      |
   |                      not allowed in type signatures
   |                      not allowed in type signatures
   |                      help: replace `_` with the correct type: `&str`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static FN_TEST4: _ = 145;
   |                      |
   |                      not allowed in type signatures
   |                      not allowed in type signatures
   |                      help: replace `_` with the correct type: `i32`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     static FN_TEST5: (_, _) = (1, 2);
   |                      ^^^^^^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test6(_: _) { }
   |                    ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test6<T>(_: T) { }
   |                ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test7(x: _) { let _x: usize = x; }
   |                    ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test7<T>(x: T) { let _x: usize = x; }
   |                ^^^    ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test8(_f: fn() -> _) { }
   |                             |
   |                             not allowed in type signatures
   |                             help: use type parameters instead: `T`


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test8(_f: fn() -> _) { }
   |                             ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL |     fn fn_test8<T>(_f: fn() -> T) { }
   |                ^^^             ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
LL |         a: _,
   |            ^ not allowed in type signatures
   |            ^ not allowed in type signatures
LL |         //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |         b: (_, _),
   |             ^  ^ not allowed in type signatures
   |             not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL |     struct FnTest10<T> {
LL |         a: T,
LL |         //~^ ERROR the type placeholder `_` is not allowed within types on item signatures
LL |         b: (T, T),

error[E0282]: type annotations needed
  --> /checkout/src/test/ui/typeck/typeck_type_placeholder_item.rs:128:18
   |
   |
LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
   |                  ^ cannot infer type

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test11(_: _) -> (_, _) { panic!() }
   |                            ^  ^ not allowed in type signatures
   |                            not allowed in type signatures


error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test12(x: i32) -> (_, _) { (x, x) }
   |                             -^--^-
   |                             ||  |
   |                             ||  not allowed in type signatures
   |                             |not allowed in type signatures
   |                             help: replace with the correct return type: `(i32, i32)`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn fn_test13(x: _) -> (i32, _) { (x, x) }
   |                           ------^-
   |                           |     not allowed in type signatures
   |                           |     not allowed in type signatures
   |                           help: replace with the correct return type: `(i32, i32)`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | struct BadStruct<_>(_);
   |                     ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct BadStruct<T>(T);
   |                  ^  ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | impl BadTrait<_> for BadStruct<_> {}
   |               ^                ^ not allowed in type signatures
   |               not allowed in type signatures
   |
help: use type parameters instead
   |
   |
LL | impl<T> BadTrait<T> for BadStruct<T> {}
   |     ^^^          ^                ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn impl_trait() -> impl BadTrait<_> {
   |                                  ^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | struct BadStruct1<_, _>(_);
   |                         ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct BadStruct1<T, _>(T);
   |                   ^     ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | struct BadStruct2<_, T>(_, T);
   |                         ^ not allowed in type signatures
help: use type parameters instead
   |
   |
LL | struct BadStruct2<U, T>(U, T);
   |                   ^     ^

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | type X = Box<_>;
   |              ^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | type Y = impl Trait<_>;
   |                     ^ not allowed in type signatures

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | fn value() -> Option<&'static _> {
   |               ----------------^-
   |               |               not allowed in type signatures
   |               |               not allowed in type signatures
   |               help: replace with the correct return type: `Option<&'static u8>`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL | const _: Option<_> = map(value);
   |          |
   |          not allowed in type signatures
   |          not allowed in type signatures
   |          help: replace `_` with the correct type: `Option<u8>`

error[E0121]: the type placeholder `_` is not allowed within types on item signatures
   |
   |
LL |     fn method_test1(&self, x: _);
---
test result: FAILED. 11431 passed; 1 failed; 93 ignored; 0 measured; 0 filtered out; finished in 133.50s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:13:11
