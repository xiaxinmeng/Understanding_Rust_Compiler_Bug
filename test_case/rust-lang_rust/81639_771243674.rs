plain
30 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll/issue-62097.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args async-await/issues/issue-62097.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/issues/issue-62097.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/issues/issue-62097.nll/auxiliary"
------------------------------------------
------------------------------------------
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error[E0373]: closure may outlive the current function, but it borrows `self`, which is owned by the current function
   |
   |
LL |         foo(|| self.bar()).await;
   |             ^^ ---- `self` is borrowed here
   |             |
   |             may outlive borrowed value `self`
   |
note: function requires argument type to outlive `'static`
   |
   |
LL |         foo(|| self.bar()).await;
   |         ^^^^^^^^^^^^^^^^^^
help: to force the closure to take ownership of `self` (and any other referenced variables), use the `move` keyword
   |
LL |         foo(move || self.bar()).await;


error[E0521]: borrowed data escapes outside of associated function
   |
   |
LL |     pub async fn run_dummy_fn(&self) { //~ ERROR E0759
   |                               ----- `self` is a reference that is only valid in the associated function body
LL |         foo(|| self.bar()).await;
   |         ^^^^^^^^^^^^^^^^^^ `self` escapes the associated function body here
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0373, E0521.
For more information about an error, try `rustc --explain E0373`.
---
13 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/dyn-trait.nll/dyn-trait.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-header-lifetime-elision/dyn-trait.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-header-lifetime-elision/dyn-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/dyn-trait.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-header-lifetime-elision/dyn-trait.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0521]: borrowed data escapes outside of function
   |
   |
LL | fn with_dyn_debug_static<'a>(x: Box<dyn Debug + 'a>) {
   |                              - `x` is a reference that is only valid in the function body
LL |     static_val(x); //~ ERROR E0759
   |     ^^^^^^^^^^^^^ `x` escapes the function body here
   |
   = help: consider replacing `'a` with `'static`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0521`.

---
11 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16683.nll/issue-16683.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-16683.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16683.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16683.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16683.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0521]: borrowed data escapes outside of associated function
   |
LL |     fn b(&self) {
LL |     fn b(&self) {
   |          ----- `self` is a reference that is only valid in the associated function body
LL |         self.a(); //~ ERROR cannot infer
   |         ^^^^^^^^ `self` escapes the associated function body here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0521`.

---
11 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17758.nll/issue-17758.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-17758.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-17758.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17758.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-17758.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0521]: borrowed data escapes outside of associated function
   |
LL |     fn bar(&self) {
LL |     fn bar(&self) {
   |            ----- `self` is a reference that is only valid in the associated function body
LL |         self.foo();
   |         ^^^^^^^^^^ `self` escapes the associated function body here
error: aborting due to previous error

For more information about this error, try `rustc --explain E0521`.

---
25 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-bound-will-change-warning.nll/lifetime-bound-will-change-warning.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lifetimes/lifetime-bound-will-change-warning.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/lifetime-bound-will-change-warning.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-bound-will-change-warning.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/lifetime-bound-will-change-warning.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0521]: borrowed data escapes outside of function
   |
   |
LL | fn test2<'a>(x: &'a Box<dyn Fn() + 'a>) {
   |              - `x` is a reference that is only valid in the function body
LL |     // but ref_obj will not, so warn.
LL |     ref_obj(x) //~ ERROR mismatched types
   |     ^^^^^^^^^^ `x` escapes the function body here
   |
   = help: consider replacing `'a` with `'static`

error[E0521]: borrowed data escapes outside of function
   |
   |
LL | fn test2cc<'a>(x: &'a Box<dyn Fn() + 'a>) {
   |                - `x` is a reference that is only valid in the function body
LL |     // same as test2, but cross crate
LL |     lib::ref_obj(x) //~ ERROR mismatched types
   |     ^^^^^^^^^^^^^^^ `x` escapes the function body here
   |
   = help: consider replacing `'a` with `'static`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0521`.

---
26 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-mybox.nll/object-lifetime-default-mybox.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args object-lifetime/object-lifetime-default-mybox.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/object-lifetime/object-lifetime-default-mybox.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-mybox.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/object-lifetime/object-lifetime-default-mybox.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-mybox.rs:27:5
   |
LL | fn load1<'a,'b>(a: &'a MyBox<dyn SomeTrait>,
   |          -- -- lifetime `'b` defined here
   |          |
   |          lifetime `'a` defined here
...
LL |     a //~ ERROR lifetime mismatch
   |     ^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`

error[E0521]: borrowed data escapes outside of function
  --> /checkout/src/test/ui/object-lifetime/object-lifetime-default-mybox.rs:31:5
   |
LL | fn load2<'a>(ss: &MyBox<dyn SomeTrait + 'a>) -> MyBox<dyn SomeTrait + 'a> {
   |              -- `ss` is a reference that is only valid in the function body
LL |     load0(ss) //~ ERROR mismatched types
   |     ^^^^^^^^^ `ss` escapes the function body here
   |
   = help: consider replacing `'a` with `'static`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0521`.

---
14 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-invariant-static-error-reporting.nll/region-invariant-static-error-reporting.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/region-invariant-static-error-reporting.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/region-invariant-static-error-reporting.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-invariant-static-error-reporting.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/region-invariant-static-error-reporting.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0521]: borrowed data escapes outside of function
   |
   |
LL | fn unify<'a>(x: Option<Invariant<'a>>, f: fn(Invariant<'a>)) {
   |              - `x` is a reference that is only valid in the function body
LL |     let bad = if x.is_some() {
LL |         x.unwrap()
   |         ^^^^^^^^^^ `x` escapes the function body here
   |
   = help: consider replacing `'a` with `'static`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0521`.

---
16 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters-trait-bound.nll/regions-bounded-method-type-parameters-trait-bound.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-bounded-method-type-parameters-trait-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-bounded-method-type-parameters-trait-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters-trait-bound.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-bounded-method-type-parameters-trait-bound.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0521]: borrowed data escapes outside of function
   |
   |
LL | fn caller2<'a,'b,F:Foo<'a>>(a: Inv<'a>, b: Inv<'b>, f: F) {
   |                             -           - `b` is a reference that is only valid in the function body
   |                             |
   |                             `a` declared here, outside of the function body
LL |     // Here the value provided for 'y is 'b, and hence 'b:'a does not hold.
LL |     f.method(b); //~ ERROR lifetime mismatch [E0623]
   |     ^^^^^^^^^^^ `b` escapes the function body here
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0521`.

---
54 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns.nll/regions-nested-fns.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-nested-fns.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-nested-fns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-nested-fns.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0521]: borrowed data escapes outside of closure
   |
   |
LL |     let mut ay = &y; //~ ERROR E0495
   |         ------ `ay` declared here, outside of the closure body
LL | 
LL |     ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
   |                                                           - `z` is a reference that is only valid in the closure body
...
LL |         ay = z;
   |         ^^^^^^ `z` escapes the closure body here

error[E0597]: `y` does not live long enough
   |
   |
LL |     let mut ay = &y; //~ ERROR E0495
   |                  ^^ borrowed value does not live long enough
...
LL |         if false { return ay; }
   |                           -- returning this value requires that `y` is borrowed for `'static`
LL | }
LL | }
   | - `y` dropped here while still borrowed

error[E0597]: `y` does not live long enough
   |
   |
LL |     ignore::<Box<dyn for<'z> FnMut(&'z isize)>>(Box::new(|z| {
   |                                                          --- value captured here
LL |         ay = x;
LL |         ay = &y;
   |               ^ borrowed value does not live long enough
...
LL |         if false { return ay; }
   |                           -- returning this value requires that `y` is borrowed for `'static`
LL | }
LL | }
   | - `y` dropped here while still borrowed
error: lifetime may not live long enough
  --> /checkout/src/test/ui/regions/regions-nested-fns.rs:14:27
   |
   |
LL | fn nested<'x>(x: &'x isize) {
   |           -- lifetime `'x` defined here
...
LL |         if false { return x; } //~ ERROR E0312
   |                           ^ returning this value requires that `'x` must outlive `'static`
   |
   = help: consider replacing `'x` with `'static`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0521, E0597.
For more information about an error, try `rustc --explain E0521`.
---
43 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound.nll/impl-on-dyn-trait-with-implicit-static-bound.nll.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args suggestions/impl-on-dyn-trait-with-implicit-static-bound.rs`
error: 1 errors occurred comparing output.
status: exit code: 1
status: exit code: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/impl-on-dyn-trait-with-implicit-static-bound.nll/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0521]: borrowed data escapes outside of function
   |
   |
LL |     fn use_it<'a, T>(val: &'a dyn ObjectTrait<T>) -> impl OtherTrait<'a> + 'a {
   |                      --- `val` is a reference that is only valid in the function body
LL |         val.use_self::<T>() //~ ERROR E0759
   |         ^^^^^^^^^^^^^^^^^^^ `val` escapes the function body here
   |
   = help: consider replacing `'a` with `'static`

error[E0521]: borrowed data escapes outside of function
   |
   |
LL |     fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> + 'a {
   |                   --- `val` is a reference that is only valid in the function body
LL |         val.use_self() //~ ERROR E0772
   |         ^^^^^^^^^^^^^^ `val` escapes the function body here
   |
   = help: consider replacing `'a` with `'static`

error[E0521]: borrowed data escapes outside of function
   |
   |
LL |     fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> {
   |                   --- `val` is a reference that is only valid in the function body
LL |         val.use_self() //~ ERROR E0759
   |         ^^^^^^^^^^^^^^ `val` escapes the function body here
   |
   = help: consider replacing `'a` with `'static`

error[E0521]: borrowed data escapes outside of function
   |
   |
LL |     fn use_it<'a>(val: &'a dyn ObjectTrait) -> impl OtherTrait<'a> + 'a {
   |                   --- `val` is a reference that is only valid in the function body
LL |         MyTrait::use_self(val) //~ ERROR E0759
   |         ^^^^^^^^^^^^^^^^^^^^^^ `val` escapes the function body here
   |
   = help: consider replacing `'a` with `'static`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0521`.

---
test result: FAILED. 11206 passed; 10 failed; 116 ignored; 0 measured; 0 filtered out; finished in 117.58s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/bin/FileCheck" "--host-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -Cdebuginfo=0 -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--llvm-version" "11.0.1-rust-1.51.0-nightly" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions frontendopenmp fuzzmutate globalisel gtest gtest_main hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target testingsupport textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xray" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always" "--compare-mode" "nll"


failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test
Build completed unsuccessfully in 0:23:49
