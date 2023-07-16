plain
................................................F................................................... 3300/11231
.................................................................................................... 3400/11231
.................................................................................................... 3500/11231
.................................................................................................... 3600/11231
.....................................................................................F.........F.... 3700/11231
.................................................................................................... 3900/11231
.................................................................................................... 4000/11231
.................................................................................................... 4100/11231
.......................ii........................................................................... 4200/11231
.......................ii........................................................................... 4200/11231
.........i.......................................................................................... 4300/11231
.................................................................................................... 4400/11231
...................................i................................................................ 4500/11231
......................................................................F............................. 4600/11231
.................................................................................................... 4700/11231
.................................................................................................... 4800/11231
.................................................................................................... 4900/11231
...................................................................................i.Fi............. 5000/11231
.................................................................................................... 5200/11231
.................................................................................................... 5300/11231
.................................................................................................... 5400/11231
.................................................................................................... 5500/11231
---
.................................................................................................... 9000/11231
.................................................................................................... 9100/11231
.................................................................................................... 9200/11231
..................i......i.......................................................................... 9300/11231
.........................................................iiiiii..iiiiii.i........................... 9400/11231
.................................................................................................... 9600/11231
.................................................................................................... 9700/11231
.................................................................................................... 9800/11231
.................................................................................................... 9900/11231
---
failures:

---- [ui] ui/const-generics/defaults/const-default.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/defaults/const-default.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/const-default" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/defaults/const-default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0277]: the trait bound `[u32; N]: Default` is not satisfied
   |
   |
LL |   items: [u32; N]
   |   ^^^^^^^^^^^^^^^ the trait `Default` is not implemented for `[u32; N]`
   = help: the following implementations were found:
   = help: the following implementations were found:
             <&[T] as Default>
             <&mut [T] as Default>
             <[T; 0] as Default>
             <[T; 10] as Default>
           and 31 others
   = note: required by `std::default::Default::default`
   = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/const-generics/min_const_generics/default_function_param.rs stdout ----
diff of stderr:

- error: expected one of `!`, `(`, `+`, `,`, `::`, `<`, or `>`, found `=`
-   --> $DIR/default_function_param.rs:1:26
+ error[E0282]: type annotations needed
3    |
3    |
4 LL | fn foo<const SIZE: usize = 5>() {}
-    |                          ^ expected one of 7 possible tokens
+    |                            ^ cannot infer type for type `{integer}`
7 error: aborting due to previous error
8 

+ For more information about this error, try `rustc --explain E0282`.
+ For more information about this error, try `rustc --explain E0282`.
9 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/default_function_param/default_function_param.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args const-generics/min_const_generics/default_function_param.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/default_function_param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/default_function_param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/default_function_param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0282]: type annotations needed
  --> /checkout/src/test/ui/const-generics/min_const_generics/default_function_param.rs:5:28
   |
LL | fn foo<const SIZE: usize = 5>() {}
   |                            ^ cannot infer type for type `{integer}`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.


------------------------------------------


---- [ui] ui/const-generics/min_const_generics/default_trait_param.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit code: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/min_const_generics/default_trait_param.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/default_trait_param" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/min_const_generics/default_trait_param/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_middle/src/ty/subst.rs:555:17: expected const for `KIND/#1` (Const { ty: bool, val: Param(KIND/#1) }/1) but found Type(bool) when substituting substs=[Self, bool]
thread 'rustc' panicked at 'Box<Any>', /checkout/compiler/rustc_errors/src/lib.rs:904:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.51.0-nightly (fc43358b0 2020-12-31) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z unstable-options -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [check_item_well_formed] checking that `Foo` is well-formed
#1 [analysis] running analysis passes on this crate
end of query stack


------------------------------------------



---- [ui] ui/error-codes/E0128.rs stdout ----
diff of stderr:

- error[E0128]: type parameters with a default cannot use forward declared identifiers
+ error[E0128]: generic parameters with a default cannot use forward declared identifiers
2   --> $DIR/E0128.rs:1:14
3    |
4 LL | struct Foo<T=U, U=()> {

-    |              ^ defaulted type parameters cannot be forward declared
+    |              ^ defaulted generic parameters cannot be forward declared
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0128/E0128.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0128.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0128.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0128" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0128/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0128]: generic parameters with a default cannot use forward declared identifiers
   |
   |
LL | struct Foo<T=U, U=()> { //~ ERROR E0128
   |              ^ defaulted generic parameters cannot be forward declared
error: aborting due to previous error

For more information about this error, try `rustc --explain E0128`.


------------------------------------------


---- [ui] ui/feature-gate/feature-gate-const_generic_defaults.rs stdout ----
diff of stderr:

4 LL | struct A<const N: usize = 3>;
6    |
6    |
-    = note: to enable them use #![feature(const_generic_defaults)]
+    = help: add `#![feature(const_generic_defaults)]` to the crate attributes to enable
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/feature-gate-const_generic_defaults/feature-gate-const_generic_defaults.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args feature-gate/feature-gate-const_generic_defaults.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/feature-gate/feature-gate-const_generic_defaults.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/feature-gate-const_generic_defaults" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/feature-gate/feature-gate-const_generic_defaults/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: default values for const generic parameters are unstable
   |
   |
LL | struct A<const N: usize = 3>;
   |
   = help: add `#![feature(const_generic_defaults)]` to the crate attributes to enable

error: aborting due to previous error
---

---- [ui] ui/generics/generic-non-trailing-defaults.rs stdout ----
diff of stderr:

10 LL | struct Foo<A, B = Vec<C>, C>(A, B, C);
12 
12 
- error[E0128]: type parameters with a default cannot use forward declared identifiers
-   --> $DIR/generic-non-trailing-defaults.rs:6:23
-    |
- LL | struct Foo<A, B = Vec<C>, C>(A, B, C);
-    |                       ^ defaulted type parameters cannot be forward declared
+ error: aborting due to 2 previous errors
- error: aborting due to 3 previous errors
- 
- For more information about this error, try `rustc --explain E0128`.
22 
22 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/generic-non-trailing-defaults/generic-non-trailing-defaults.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generics/generic-non-trailing-defaults.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generics/generic-non-trailing-defaults.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/generic-non-trailing-defaults" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/generic-non-trailing-defaults/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: type parameters with a default must be trailing
   |
   |
LL | struct Vec<A = Heap, T>(A, T);


error: type parameters with a default must be trailing
   |
   |
LL | struct Foo<A, B = Vec<C>, C>(A, B, C);

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/generics/generic-type-params-forward-mention.rs stdout ----
diff of stderr:

- error[E0128]: type parameters with a default cannot use forward declared identifiers
+ error[E0128]: generic parameters with a default cannot use forward declared identifiers
3    |
3    |
4 LL | struct Foo<T = Option<U>, U = bool>(T, U);

-    |                       ^ defaulted type parameters cannot be forward declared
+    |                       ^ defaulted generic parameters cannot be forward declared
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/generic-type-params-forward-mention/generic-type-params-forward-mention.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generics/generic-type-params-forward-mention.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generics/generic-type-params-forward-mention.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/generic-type-params-forward-mention" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generics/generic-type-params-forward-mention/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0128]: generic parameters with a default cannot use forward declared identifiers
   |
   |
LL | struct Foo<T = Option<U>, U = bool>(T, U);
   |                       ^ defaulted generic parameters cannot be forward declared
error: aborting due to previous error

For more information about this error, try `rustc --explain E0128`.


------------------------------------------


---- [ui] ui/issues/issue-18183.rs stdout ----
diff of stderr:

- error[E0128]: type parameters with a default cannot use forward declared identifiers
+ error[E0128]: generic parameters with a default cannot use forward declared identifiers
3    |
3    |
4 LL | pub struct Foo<Bar=Bar>(Bar);

-    |                    ^^^ defaulted type parameters cannot be forward declared
+    |                    ^^^ defaulted generic parameters cannot be forward declared
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18183/issue-18183.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-18183.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-18183.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18183" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-18183/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0128]: generic parameters with a default cannot use forward declared identifiers
   |
   |
LL | pub struct Foo<Bar=Bar>(Bar); //~ ERROR E0128
   |                    ^^^ defaulted generic parameters cannot be forward declared
error: aborting due to previous error

For more information about this error, try `rustc --explain E0128`.


------------------------------------------


---- [ui] ui/issues/issue-26812.rs stdout ----
diff of stderr:

- error[E0128]: type parameters with a default cannot use forward declared identifiers
+ error[E0128]: generic parameters with a default cannot use forward declared identifiers
3    |
3    |
4 LL | fn avg<T=T::Item>(_: T) {}

-    |          ^^^^^^^ defaulted type parameters cannot be forward declared
+    |          ^^^^^^^ defaulted generic parameters cannot be forward declared
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26812/issue-26812.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-26812.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-26812.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26812" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-26812/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0128]: generic parameters with a default cannot use forward declared identifiers
   |
   |
LL | fn avg<T=T::Item>(_: T) {}
   |          ^^^^^^^ defaulted generic parameters cannot be forward declared
error: aborting due to previous error

For more information about this error, try `rustc --explain E0128`.

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-9/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "9.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver engine executionengine fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:14:53
