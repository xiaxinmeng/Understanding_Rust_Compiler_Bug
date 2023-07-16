plain
.................................................................................................... 5900/12155
.................................................................................................... 6000/12155
.....i....................................................................F......................... 6100/12155
.................................................................................................... 6200/12155
.......i...F...................................................................i.................... 6300/12155
.........................................ii.ii.......i...i.......................................... 6500/12155
.........................................................................................i....i..... 6600/12155
....................................i..............i............i................................... 6700/12155
.................................................................................................... 6800/12155
---
.................................................................................................... 7700/12155
.................................................................................................... 7800/12155
.................................................................................................... 7900/12155
............................i..ii..............................................................ii... 8000/12155
...............................................................F.F.................................. 8100/12155
i................................i............................................................i..... 8300/12155
.................................................................................................... 8400/12155
.................................i.................................................................. 8500/12155
.................................................................................................... 8600/12155
---
1 error[E0308]: mismatched types
-   --> $DIR/unused-substs-5.rs:15:9
+   --> $DIR/unused-substs-5.rs:15:19
3    |
4 LL |     x = q::<_, N>(x);
-    |         ^^^^^^^^^^^^- help: try using a conversion method: `.to_vec()`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |         |
-    |         cyclic type of infinite size
+    |                   ^ cyclic type of infinite size
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-5/unused-substs-5.stderr
To only update this specific test, also pass `--test-args const-generics/occurs-check/unused-substs-5.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/occurs-check/unused-substs-5.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-5" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/occurs-check/unused-substs-5/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/occurs-check/unused-substs-5.rs:15:19
   |
LL |     x = q::<_, N>(x); //~ ERROR mismatched types
   |                   ^ cyclic type of infinite size
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---
diff of stderr:

2   --> $DIR/issue-72616.rs:20:30
3    |
4 LL |         if String::from("a") == "a".try_into().unwrap() {}
-    |                              ^^ -------------- this method call resolves to `Result<T, <Self as TryInto<T>>::Error>`
-    |                              cannot infer type
+    |                              ^^ cannot infer type
8    |
8    |
9    = note: cannot satisfy `String: PartialEq<_>`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616/issue-72616.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616/issue-72616.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/issue-72616.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/issue-72616.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-72616/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/inference/issue-72616.rs:20:30
   |
LL |         if String::from("a") == "a".try_into().unwrap() {}
   |                              ^^ cannot infer type
   |
   = note: cannot satisfy `String: PartialEq<_>`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0283`.

---
diff of stderr:

2   --> $DIR/issue-51116.rs:6:13
3    |
4 LL |         for tile in row {
-    |                     --- the element type for this iterator is not specified
+    |             ---- consider giving `tile` a type
7 LL |             *tile = 0;
8    |             ^^^^^ cannot infer type



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51116/issue-51116.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-51116.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-51116.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51116" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-51116/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/issues/issue-51116.rs:6:13
   |
LL |         for tile in row {
   |             ---- consider giving `tile` a type
LL |             //~^ NOTE the element type for this iterator is not specified
LL |             *tile = 0;
   |
   = note: type must be known at this point

error: aborting due to previous error
---

---- [ui] ui/issues/issue-59494.rs stdout ----
diff of stderr:

- error[E0277]: expected a `Fn<(_,)>` closure, found `impl Fn<(((_, _), _),)>`
-   --> $DIR/issue-59494.rs:21:22
+ error[E0308]: mismatched types
3    |
3    |
- LL |     let t8 = t8n(t7, t7p(f, g));
-    |                      ^^^^^^^^^ expected an `Fn<(_,)>` closure, found `impl Fn<(((_, _), _),)>`
-    |
-    = help: the trait `Fn<(_,)>` is not implemented for `impl Fn<(((_, _), _),)>`
- note: required by a bound in `t8n`
-   --> $DIR/issue-59494.rs:5:45
-    |
- LL | fn t8n<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(A) -> C) -> impl Fn(A) -> (B, C)
-    |                                             ^^^^^^^^^^ required by this bound in `t8n`
+ LL |     let t7 = |env| |a| |b| t7p(f, g)(((env, a), b));
+    |                                        ^^^ cyclic type of infinite size
14 error: aborting due to previous error
15 

- For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args issues/issue-59494.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-59494.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59494" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-59494/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-59494.rs:20:40
   |
LL |     let t7 = |env| |a| |b| t7p(f, g)(((env, a), b));
   |                                        ^^^ cyclic type of infinite size
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---
1 error[E0308]: mismatched types
-   --> $DIR/occurs-check-3.rs:4:24
+   --> $DIR/occurs-check-3.rs:4:32
3    |
4 LL | fn main() { let c; c = Clam::A(c); match c { Clam::A::<isize>(_) => { } } }
-    |                        ^^^^^^^^^^ cyclic type of infinite size
+    |                                ^ cyclic type of infinite size
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/occurs-check-3/occurs-check-3.stderr
To only update this specific test, also pass `--test-args occurs-check-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/occurs-check-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/occurs-check-3" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/occurs-check-3/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/occurs-check-3.rs:4:32
   |
LL | fn main() { let c; c = Clam::A(c); match c { Clam::A::<isize>(_) => { } } }
   |                                ^ cyclic type of infinite size
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---
1 error[E0308]: mismatched types
-   --> $DIR/occurs-check-2.rs:7:9
+   --> $DIR/occurs-check-2.rs:6:9
3    |
- LL |     f = box g;
-    |         ^^^^^ cyclic type of infinite size
- help: try using a conversion method
-    |
-    |
- LL |     f = (box g).to_string();
-    |         +     +++++++++++++
+ LL |     g = f;
+    |         ^ cyclic type of infinite size
12 error: aborting due to previous error
13 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/occurs-check-2/occurs-check-2.stderr
To only update this specific test, also pass `--test-args occurs-check-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/occurs-check-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/occurs-check-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/occurs-check-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/occurs-check-2.rs:6:9
   |
LL |     g = f;
   |         ^ cyclic type of infinite size
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.

---
1 error[E0284]: type annotations needed
-   --> $DIR/question-mark-type-infer.rs:12:21
+   --> $DIR/question-mark-type-infer.rs:12:5
3    |
4 LL |     l.iter().map(f).collect()?
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^ cannot infer type
6    |
6    |
7    = note: cannot satisfy `<_ as Try>::Residual == _`
- help: consider specifying the type argument in the method call
-    |
- LL |     l.iter().map(f).collect::<B>()?
12 
13 error: aborting due to previous error
14 

---
To only update this specific test, also pass `--test-args question-mark-type-infer.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/question-mark-type-infer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/question-mark-type-infer" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/question-mark-type-infer/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0284]: type annotations needed
  --> /checkout/src/test/ui/question-mark-type-infer.rs:12:5
   |
LL |     l.iter().map(f).collect()? //~ ERROR type annotations needed
   |
   |
   = note: cannot satisfy `<_ as Try>::Residual == _`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0284`.

---
diff of stderr:

12   --> $DIR/issue-77982.rs:12:44
13    |
14 LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
-    |                                            ^^^^^^^^^ ----------- this method call resolves to `T`
-    |                                            |
-    |                                            cannot infer type for type parameter `T` declared on the trait `From`
+    |                                            ^^^^^^^^^ cannot infer type for type parameter `T` declared on the trait `From`
18    |
19    = note: cannot satisfy `u32: From<_>`
20 note: required by `from`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/issue-77982.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args traits/issue-77982.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/traits/issue-77982.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/traits/issue-77982/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:8:10
   |
LL |     opts.get(opt.as_ref()); //~ ERROR type annotations needed
   |          ^^^ ------------ this method call resolves to `&T`
   |          |
   |          cannot infer type for type parameter `Q` declared on the associated function `get`
   |
   = note: cannot satisfy `String: Borrow<_>`
error[E0283]: type annotations needed
  --> /checkout/src/test/ui/traits/issue-77982.rs:12:44
   |
   |
LL |     let ips: Vec<_> = (0..100_000).map(|_| u32::from(0u32.into())).collect();
   |                                            ^^^^^^^^^ cannot infer type for type parameter `T` declared on the trait `From`
   |
   = note: cannot satisfy `u32: From<_>`
note: required by `from`
   |
   |
LL |     fn from(_: T) -> Self;

error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:35:16
   |
   |
LL |     let _ = ().foo(); //~ ERROR type annotations needed
   |         -      ^^^ cannot infer type for type parameter `T` declared on the trait `Foo`
   |         |
   |         consider giving this pattern the explicit type `Box<T>`, where the type parameter `T` is specified
   |
   = note: cannot satisfy `(): Foo<'_, _>`
error[E0283]: type annotations needed for `Box<T>`
  --> /checkout/src/test/ui/traits/issue-77982.rs:39:19
   |
   |
LL |     let _ = (&()).bar(); //~ ERROR type annotations needed
   |         -         ^^^ cannot infer type for type parameter `T` declared on the trait `Bar`
   |         |
   |         consider giving this pattern the explicit type `Box<T>`, where the type parameter `T` is specified
   |
   = note: cannot satisfy `&(): Bar<'_, _>`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0283`.


------------------------------------------


---- [ui] ui/type-inference/sort_by_key.rs stdout ----
diff of stderr:

3    |
4 LL |     lst.sort_by_key(|&(v, _)| v.iter().sum());
5    |         ^^^^^^^^^^^ cannot infer type for type parameter `K` declared on the associated function `sort_by_key`
- help: consider specifying the type argument in the method call
-    |
-    |
- LL |     lst.sort_by_key(|&(v, _)| v.iter().sum::<S>());
11 
12 error: aborting due to previous error
13 

---
To only update this specific test, also pass `--test-args type-inference/sort_by_key.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type-inference/sort_by_key.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/sort_by_key" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-inference/sort_by_key/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/type-inference/sort_by_key.rs:3:9
   |
LL |     lst.sort_by_key(|&(v, _)| v.iter().sum()); //~ ERROR type annotations needed
   |         ^^^^^^^^^^^ cannot infer type for type parameter `K` declared on the associated function `sort_by_key`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.

---
test result: FAILED. 12045 passed; 9 failed; 101 ignored; 0 measured; 0 filtered out; finished in 123.75s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:11:46
