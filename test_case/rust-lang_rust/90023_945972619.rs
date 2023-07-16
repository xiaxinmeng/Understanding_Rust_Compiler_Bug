plain
.................................................................................................... 1500/12312
...............................i.....i................i............................................. 1600/12312
.................................................................................................... 1700/12312
.....................................................i.............................................. 1800/12312
..................F.................................................................F............... 1900/12312
..........................FF........................................................................ 2000/12312
.............i.................................F.................................................... 2100/12312
.................................................................................................... 2300/12312
.................................................................................................... 2400/12312
.................................................................................................... 2500/12312
.................................................................................................... 2600/12312
---
failures:

---- [ui] ui/const-generics/const-argument-non-static-lifetime.rs#full stdout ----

error in revision `full`: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/const-argument-non-static-lifetime.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "full" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-non-static-lifetime.full/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/const-argument-non-static-lifetime.full/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: broken MIR in DefId(0:5 ~ const_argument_non_static_lifetime[a8be]::wow) (CanonicalUserTypeAnnotation { user_ty: Canonical { max_universe: U0, variables: [], value: TypeOf(DefId(0:3 ~ const_argument_non_static_lifetime[a8be]::test), UserSubsts { substs: [Const { ty: usize, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:7 ~ const_argument_non_static_lifetime[a8be]::wow::{constant#0}), const_param_did: Some(DefId(0:4 ~ const_argument_non_static_lifetime[a8be]::test::N)) }, substs_: None, promoted: None }) }], user_self_ty: None }) }, span: /checkout/src/test/ui/const-generics/const-argument-non-static-lifetime.rs:12:5: 15:7 (#0), inferred_ty: fn() {test::<{
        let _: &'a ();
        3
    }>} }): bad user type AscribeUserType(fn() {test::<3_usize>}, DefId(0:3 ~ const_argument_non_static_lifetime[a8be]::test) UserSubsts { substs: [Const { ty: usize, val: Unevaluated(Unevaluated { def: WithOptConstParam { did: DefId(0:7 ~ const_argument_non_static_lifetime[a8be]::wow::{constant#0}), const_param_did: Some(DefId(0:4 ~ const_argument_non_static_lifetime[a8be]::test::N)) }, substs_: None, promoted: None }) }], user_self_ty: None }, type_of=fn() {test::<N>}): NoSolution
   = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:330:27


thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', compiler/rustc_errors/src/lib.rs:1165:13

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.58.0-nightly (e68e3810c 2021-10-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
end of query stack

------------------------------------------
------------------------------------------


---- [ui] ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs:27:9
   |
LL |     let mut _q = Default::default();
   |         ^^^^^^ consider giving `_q` a type
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/const-generics/generic_const_exprs/issue-80742.rs stdout ----
diff of stderr:

- error[E0080]: evaluation of `Inline::<dyn std::fmt::Debug>::{constant#0}` failed
-   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
- LL |     intrinsics::size_of::<T>()
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |     |
-    |     |
-    |     size_of called on unsized type `dyn Debug`
-    |     inside `std::mem::size_of::<dyn Debug>` at $SRC_DIR/core/src/mem/mod.rs:LL:COL
-   ::: $DIR/issue-80742.rs:22:10
-    |
-    |
- LL |     [u8; size_of::<T>() + 1]: ,
-    |          -------------- inside `Inline::<dyn Debug>::{constant#0}` at $DIR/issue-80742.rs:22:10
- 
15 error[E0599]: the function or associated item `new` exists for struct `Inline<dyn Debug>`, but its trait bounds were not satisfied
17    |

35    = note: the following trait bounds were not satisfied:
35    = note: the following trait bounds were not satisfied:
36            `dyn Debug: Sized`
37 
- error[E0080]: evaluation of `Inline::<dyn std::fmt::Debug>::{constant#0}` failed
-   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
- LL |     intrinsics::size_of::<T>()
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
-    |     |
-    |     |
-    |     size_of called on unsized type `dyn Debug`
-    |     inside `std::mem::size_of::<dyn Debug>` at $SRC_DIR/core/src/mem/mod.rs:LL:COL
-   ::: $DIR/issue-80742.rs:14:10
-    |
-    |
- LL |     [u8; size_of::<T>() + 1]: ,
-    |          -------------- inside `Inline::<dyn Debug>::{constant#0}` at $DIR/issue-80742.rs:14:10
52 error[E0277]: the size for values of type `dyn Debug` cannot be known at compilation time
53   --> $DIR/issue-80742.rs:30:15
54    |


66 LL | struct Inline<T: ?Sized>
68 
- error: aborting due to 4 previous errors
+ error: unconstrained generic constant
+   --> $DIR/issue-80742.rs:30:15
+   --> $DIR/issue-80742.rs:30:15
+    |
+ LL |     let dst = Inline::<dyn Debug>::new(0);
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); size_of::<T>() + 1]:`
+ note: required by a bound in `Inline`
+    |
+    |
+ LL | struct Inline<T>
+    |        ------ required by a bound in this
+ LL | where
+ LL |     [u8; size_of::<T>() + 1]: ,
+    |          ^^^^^^^^^^^^^^^^^^ required by this bound in `Inline`
- Some errors have detailed explanations: E0080, E0277, E0599.
- For more information about an error, try `rustc --explain E0080`.
+ error: aborting due to 3 previous errors
+ 
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-80742.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80742" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-80742/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0599]: the function or associated item `new` exists for struct `Inline<dyn Debug>`, but its trait bounds were not satisfied
   |
   |
LL | / struct Inline<T>
LL | | where
LL | |     [u8; size_of::<T>() + 1]: ,
LL | | {
LL | |     _phantom: PhantomData<T>,
LL | |     buf: [u8; size_of::<T>() + 1],
LL | | }
   | |_- function or associated item `new` not found for this
...
LL |       let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |                                      ^^^ function or associated item cannot be called on `Inline<dyn Debug>` due to unsatisfied trait bounds
  ::: /checkout/library/core/src/fmt/mod.rs:623:1
   |
LL |   pub trait Debug {
LL |   pub trait Debug {
   |   --------------- doesn't satisfy `dyn Debug: Sized`
   = note: the following trait bounds were not satisfied:
   = note: the following trait bounds were not satisfied:
           `dyn Debug: Sized`
error[E0277]: the size for values of type `dyn Debug` cannot be known at compilation time
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:30:15
   |
   |
LL |     let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |
   = help: the trait `Sized` is not implemented for `dyn Debug`
   = help: the trait `Sized` is not implemented for `dyn Debug`
note: required by a bound in `Inline`
   |
LL | struct Inline<T>
LL | struct Inline<T>
   |               ^ required by this bound in `Inline`
help: consider relaxing the implicit `Sized` restriction
   |
LL | struct Inline<T: ?Sized>

error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-80742.rs:30:15
   |
   |
LL |     let dst = Inline::<dyn Debug>::new(0); //~ ERROR
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); size_of::<T>() + 1]:`
note: required by a bound in `Inline`
   |
LL | struct Inline<T>
   |        ------ required by a bound in this
LL | where
LL | where
LL |     [u8; size_of::<T>() + 1]: ,
   |          ^^^^^^^^^^^^^^^^^^ required by this bound in `Inline`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0599.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/const-generics/generic_const_exprs/issue-85848.rs stdout ----
diff of stderr:

43 LL | fn writes_to_specific_path<C: Delegates<()>>(cap: &C) {}
44    |                               ^^^^^^^^^^^^^ required by this bound in `writes_to_specific_path`
- error: aborting due to 2 previous errors
+ error[E0308]: mismatched types
+   --> $DIR/issue-85848.rs:24:5
+    |
+    |
+ LL |     writes_to_specific_path(&cap);
+    |     ^^^^^^^^^^^^^^^^^^^^^^^ expected `true`, found `{ contains::<T, U>() }`
+    = note: expected type `true`
+    = note: expected type `true`
+               found type `{ contains::<T, U>() }`
- For more information about this error, try `rustc --explain E0277`.
+ error: aborting due to 3 previous errors
+ 
+ Some errors have detailed explanations: E0277, E0308.
---
To only update this specific test, also pass `--test-args const-generics/generic_const_exprs/issue-85848.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-85848" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-85848/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `(): _Contains<&C>` is not satisfied
   |
   |
LL |     writes_to_specific_path(&cap);
   |     ^^^^^^^^^^^^^^^^^^^^^^^ the trait `_Contains<&C>` is not implemented for `()`
   |
note: required because of the requirements on the impl of `Contains<(), true>` for `&C`
   |
   |
LL | impl<T, U> Contains<T, { contains::<T, U>() }> for U where T: _Contains<U> {}
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^
note: required because of the requirements on the impl of `Delegates<()>` for `&C`
   |
   |
LL | impl<T, U> Delegates<U> for T where T: Contains<U, true> {}
   |            ^^^^^^^^^^^^     ^
note: required by a bound in `writes_to_specific_path`
   |
   |
LL | fn writes_to_specific_path<C: Delegates<()>>(cap: &C) {}
   |                               ^^^^^^^^^^^^^ required by this bound in `writes_to_specific_path`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs:24:5
   |
   |
LL |     writes_to_specific_path(&cap);
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { contains::<T, U>() }]:`
note: required because of the requirements on the impl of `Contains<(), true>` for `&C`
   |
   |
LL | impl<T, U> Contains<T, { contains::<T, U>() }> for U where T: _Contains<U> {}
   |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^     ^
note: required because of the requirements on the impl of `Delegates<()>` for `&C`
   |
   |
LL | impl<T, U> Delegates<U> for T where T: Contains<U, true> {}
   |            ^^^^^^^^^^^^     ^
note: required by a bound in `writes_to_specific_path`
   |
   |
LL | fn writes_to_specific_path<C: Delegates<()>>(cap: &C) {}
   |                               ^^^^^^^^^^^^^ required by this bound in `writes_to_specific_path`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-85848.rs:24:5
   |
   |
LL |     writes_to_specific_path(&cap);
   |     ^^^^^^^^^^^^^^^^^^^^^^^ expected `true`, found `{ contains::<T, U>() }`
   = note: expected type `true`
   = note: expected type `true`
              found type `{ contains::<T, U>() }`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
For more information about an error, try `rustc --explain E0277`.

------------------------------------------


---- [ui] ui/const-generics/issues/issue-86530.rs stdout ----
diff of stderr:

15 LL |     T: X,
16    |        ^ required by this bound in `z`
- error: aborting due to previous error
+ error: unconstrained generic constant
+   --> $DIR/issue-86530.rs:16:5
+    |
+    |
+ LL |     z(" ");
+    |
+    |
+    = help: try adding a `where` bound using this expression: `where [(); T::Y]:`
+ note: required by a bound in `z`
+    |
+    |
+ LL | fn z<T>(t: T)
+    |    - required by a bound in this
+ ...
+ LL |     [(); T::Y]: ,
+    |          ^^^^ required by this bound in `z`
+ error: aborting due to 2 previous errors
19 
20 For more information about this error, try `rustc --explain E0277`.
21 
---
To only update this specific test, also pass `--test-args const-generics/issues/issue-86530.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-86530.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86530" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-86530/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `&str: X` is not satisfied
   |
   |
LL |     z(" ");
   |     - ^^^ the trait `X` is not implemented for `&str`
   |     required by a bound introduced by this call
   |
note: required by a bound in `z`
  --> /checkout/src/test/ui/const-generics/issues/issue-86530.rs:10:8
  --> /checkout/src/test/ui/const-generics/issues/issue-86530.rs:10:8
   |
LL | fn z<T>(t: T)
   |    - required by a bound in this
LL | where
LL |     T: X,
   |        ^ required by this bound in `z`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/issues/issue-86530.rs:16:5
   |
   |
LL |     z(" ");
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); T::Y]:`
note: required by a bound in `z`
   |
   |
LL | fn z<T>(t: T)
   |    - required by a bound in this
...
LL |     [(); T::Y]: ,
   |          ^^^^ required by this bound in `z`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.

---
test result: FAILED. 12190 passed; 5 failed; 117 ignored; 0 measured; 0 filtered out; finished in 132.35s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:07
