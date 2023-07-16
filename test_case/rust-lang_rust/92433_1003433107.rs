plain

40 LL |     for i in 0..x {
41    |              ^^^^
42    |              |
-    |              calling non-const function `<std::ops::Range<usize> as IntoIterator>::into_iter`
+    |              calling non-const function `iter::range::<impl Iterator for std::ops::Range<usize>>::next`
44    |              inside `f` at $DIR/const-fn-error.rs:5:14
45 ...
46 LL |     let a : [i32; f(X)];

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-error/const-fn-error.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-fn-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-fn-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-fn-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0658]: `for` is not allowed in a `const fn`
   |
LL | /     for i in 0..x {
LL | /     for i in 0..x {
LL | |         //~^ ERROR mutable references
LL | |         //~| ERROR calls in constant functions
LL | |         //~| ERROR calls in constant functions
...  |
LL | |         sum += i;
LL | |     }
   |
   = note: see issue #87575 <https://github.com/rust-lang/rust/issues/87575> for more information
   = help: add `#![feature(const_for)]` to the crate attributes to enable


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   |
LL |     for i in 0..x {
   |              ^^^^

---
   |
   = note: see issue #57349 <https://github.com/rust-lang/rust/issues/57349> for more information
   = help: add `#![feature(const_mut_refs)]` to the crate attributes to enable

error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   |
LL |     for i in 0..x {
   |              ^^^^


error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-fn-error.rs:5:14
   |
LL |     for i in 0..x {
   |              ^^^^
   |              |
   |              calling non-const function `iter::range::<impl Iterator for std::ops::Range<usize>>::next`
   |              inside `f` at /checkout/src/test/ui/consts/const-fn-error.rs:5:14
...
LL |     let a : [i32; f(X)];
   |                   ---- inside `main::{constant#0}` at /checkout/src/test/ui/consts/const-fn-error.rs:18:19
error: aborting due to 5 previous errors

Some errors have detailed explanations: E0015, E0080, E0658.
For more information about an error, try `rustc --explain E0015`.
For more information about an error, try `rustc --explain E0015`.

------------------------------------------


---- [ui] ui/issues/issue-23966.rs stdout ----
diff of stderr:

10 note: required by a bound in `fold`
11   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
12    |
- LL |         F: FnMut(B, Self::Item) -> B,
-    |            ^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `fold`
+ LL |         F: ~const FnMut(B, Self::Item) -> B,
+    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `fold`
16 error: aborting due to previous error
17 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23966/issue-23966.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-23966.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23966.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23966" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23966/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: expected a `FnMut<(_, char)>` closure, found `()`
   |
   |
LL |     "".chars().fold(|_, _| (), ());
   |                ----            ^^ expected an `FnMut<(_, char)>` closure, found `()`
   |                required by a bound introduced by this call
   |
   |
   = help: the trait `FnMut<(_, char)>` is not implemented for `()`
note: required by a bound in `fold`
   |
   |
LL |         F: ~const FnMut(B, Self::Item) -> B,
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `fold`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.

---

11 note: associated function defined here
12   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
13    |
- LL |     fn fold<B, F>(mut self, init: B, mut f: F) -> B
+ LL |     fn fold<B, F>(self, init: B, mut f: F) -> B
16 
17 error: aborting due to previous error



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3044/issue-3044.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-3044.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3044.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3044" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3044/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> /checkout/src/test/ui/issues/issue-3044.rs:3:23
   |
LL |       needlesArr.iter().fold(|x, y| {
   |  _______________________^^^^_-
   | |                       expected 2 arguments
LL | |     });
   | |_____- supplied 1 argument
   |
   |
note: associated function defined here
  --> /checkout/library/core/src/iter/traits/iterator.rs:2270:8
   |
LL |     fn fold<B, F>(self, init: B, mut f: F) -> B

error: aborting due to previous error

For more information about this error, try `rustc --explain E0061`.
---

17 note: required by a bound in `collect`
18   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
19    |
- LL |     fn collect<B: FromIterator<Self::Item>>(self) -> B
-    |                   ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `collect`
+ LL |     fn collect<B: ~const FromIterator<Self::Item>>(self) -> B
+    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `collect`
23 error: aborting due to 2 previous errors
24 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34334/issue-34334.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-34334.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34334.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34334" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34334/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: expected one of `,`, `:`, or `>`, found `=`
   |
   |
LL |     let sr: Vec<(u32, _, _) = vec![];
   |         --                - ^ expected one of `,`, `:`, or `>`
   |         |                 |
   |         |                 maybe try to close unmatched angle bracket
   |         while parsing the type for `sr`

error[E0277]: a value of type `Vec<(u32, _, _)>` cannot be built from an iterator over elements of type `()`
   |
   |
LL |     let sr2: Vec<(u32, _, _)> = sr.iter().map(|(faction, th_sender, th_receiver)| {}).collect();
   |                                                                                       ^^^^^^^ value of type `Vec<(u32, _, _)>` cannot be built from `std::iter::Iterator<Item=()>`
   |
   = help: the trait `FromIterator<()>` is not implemented for `Vec<(u32, _, _)>`
note: required by a bound in `collect`
   |
   |
LL |     fn collect<B: ~const FromIterator<Self::Item>>(self) -> B
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `collect`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.

---

8 note: required by a bound in `collect`
9   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
10    |
- LL |     fn collect<B: FromIterator<Self::Item>>(self) -> B
-    |                   ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `collect`
+ LL |     fn collect<B: ~const FromIterator<Self::Item>>(self) -> B
+    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `collect`
13 
14 error[E0277]: a value of type `Vec<f64>` cannot be built from an iterator over elements of type `&f64`

21 note: required by a bound in `collect`
22   --> $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
23    |
23    |
- LL |     fn collect<B: FromIterator<Self::Item>>(self) -> B
-    |                   ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `collect`
+ LL |     fn collect<B: ~const FromIterator<Self::Item>>(self) -> B
+    |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `collect`
27 error: aborting due to 2 previous errors
28 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call/issue-66923-show-error-for-correct-call.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-66923-show-error-for-correct-call.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-66923-show-error-for-correct-call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-66923-show-error-for-correct-call/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: a value of type `Vec<f64>` cannot be built from an iterator over elements of type `&f64`
   |
   |
LL |     let x2: Vec<f64> = x1.into_iter().collect();
   |                                       ^^^^^^^ value of type `Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
   |
   = help: the trait `FromIterator<&f64>` is not implemented for `Vec<f64>`
note: required by a bound in `collect`
   |
   |
LL |     fn collect<B: ~const FromIterator<Self::Item>>(self) -> B
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `collect`

error[E0277]: a value of type `Vec<f64>` cannot be built from an iterator over elements of type `&f64`
   |
   |
LL |     let x3 = x1.into_iter().collect::<Vec<f64>>();
   |                             ^^^^^^^ value of type `Vec<f64>` cannot be built from `std::iter::Iterator<Item=&f64>`
   |
   = help: the trait `FromIterator<&f64>` is not implemented for `Vec<f64>`
note: required by a bound in `collect`
   |
   |
LL |     fn collect<B: ~const FromIterator<Self::Item>>(self) -> B
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `collect`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/rfc-2632-const-trait-impl/call-const-trait-method-fail.rs stdout ----
diff of stderr:

4 LL |     a.plus(b)
6 
- error: aborting due to previous error
- error: aborting due to previous error
+ error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+    |
+    |
+ LL |     a.plus(b)
+ 
+ error: aborting due to 2 previous errors
8 
9 For more information about this error, try `rustc --explain E0015`.
---
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/call-const-trait-method-fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-fail" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/call-const-trait-method-fail/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   |
   |
LL |     a.plus(b)


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   |
   |
LL |     a.plus(b)

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0015`.
---
4 LL |     NonConst.func();
5    |     ^^^^^^^^^^^^^^^
6 
- error: aborting due to previous error
+ error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
+    |
+ LL |     NonConst.func();
+    |     ^^^^^^^^^^^^^^^
+ 
+ 
+ error: aborting due to 2 previous errors
8 
9 For more information about this error, try `rustc --explain E0015`.
10 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/cross-crate.gated/cross-crate.gated.stderr
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/cross-crate.rs`


error in revision `gated`: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/cross-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "gated" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/cross-crate.gated" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/cross-crate.gated/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   |
LL |     NonConst.func();
   |     ^^^^^^^^^^^^^^^


error[E0015]: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   |
LL |     NonConst.func();
   |     ^^^^^^^^^^^^^^^

---

6    |
7   ::: $SRC_DIR/core/src/iter/traits/iterator.rs:LL:COL
8    |
- LL |         Self: Sized,
+ LL |         Self: Sized + ~const Drop,
10    |               ----- this has a `Sized` requirement
11    |
12    = note: you need `&mut dyn Iterator<Item = &u64>` instead of `&dyn Iterator<Item = &u64>`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/imm-ref-trait-object/imm-ref-trait-object.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/imm-ref-trait-object.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/imm-ref-trait-object.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/imm-ref-trait-object" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/imm-ref-trait-object/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: the `min` method cannot be invoked on a trait object
   |
   |
LL |      t.min().unwrap() //~ ERROR the `min` method cannot be invoked on a trait object
   |
  ::: /checkout/library/core/src/iter/traits/iterator.rs:2965:15
   |
   |
LL |         Self: Sized + ~const Drop,
   |               ----- this has a `Sized` requirement
   |
   = note: you need `&mut dyn Iterator<Item = &u64>` instead of `&dyn Iterator<Item = &u64>`
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 12401 passed; 8 failed; 119 ignored; 0 measured; 0 filtered out; finished in 156.24s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:13
