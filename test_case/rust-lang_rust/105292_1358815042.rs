plain
...i...........ii....................................................................... 1936/14029
........................................................................................ 2024/14029
............................................................................i........... 2112/14029
........................................................................................ 2200/14029
.........................................................FF.F.........F.F............... 2288/14029
.........F.........F.....FF.F.........F................................................. 2376/14029
......................................................................................F. 2464/14029
........................................................................................ 2640/14029
........................................................................................ 2728/14029
........................................................................................ 2816/14029
........................................................................................ 2904/14029
---
failures:

---- [ui] src/test/ui/const-generics/generic_const_exprs/cross_crate.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/const-generics/generic_const_exprs/auxiliary/const_evaluatable_lib.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/auxiliary/const_evaluatable_lib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/cross_crate/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/cross_crate/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/auxiliary/const_evaluatable_lib.rs:6:10
   |
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
LL | where
LL | where
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/auxiliary/const_evaluatable_lib.rs:6:10
   |
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/cross_crate_predicate.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui/const-generics/generic_const_exprs/auxiliary/const_evaluatable_lib.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/auxiliary/const_evaluatable_lib.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/cross_crate_predicate/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/cross_crate_predicate/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/auxiliary/const_evaluatable_lib.rs:6:10
   |
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
note: required by a bound in `test1`
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
LL | where
LL | where
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/auxiliary/const_evaluatable_lib.rs:6:10
   |
   |
LL |     [u8; std::mem::size_of::<T>() - 1]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>() - 1]:`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
note: required by a bound in `test1`
   |
   |
LL | pub fn test1<T>() -> [u8; std::mem::size_of::<T>() - 1]
   |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test1`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs:20:13
   |
   |
LL |     (): Foo<{ N + 1 }>,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<T, const N: usize>(_: T) -> <() as Foo<{ N + 1 }>>::Assoc
LL | where
LL | where
LL |     (): Foo<{ N + 1 }>,
   |             ^^^^^^^^^ required by this bound in `foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs:20:13
   |
   |
LL |     (): Foo<{ N + 1 }>,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
note: required by a bound in `foo`
   |
   |
LL | fn foo<T, const N: usize>(_: T) -> <() as Foo<{ N + 1 }>>::Assoc
   |                                               ^^^^^^^^^ required by this bound in `foo`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs:20:9
   |
   |
LL |     (): Foo<{ N + 1 }>,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/const_eval_resolve_canonical.rs:18:36
   |
   |
LL | fn foo<T, const N: usize>(_: T) -> <() as Foo<{ N + 1 }>>::Assoc
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N + 1 }]:`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/fn_call.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/fn_call/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/fn_call/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:15:10
   |
   |
LL |     [u8; std::mem::size_of::<T>()]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>()]:`
note: required by a bound in `test_simple`
   |
   |
LL | fn test_simple<T>() -> [u8; std::mem::size_of::<T>()]
LL | where
LL | where
LL |     [u8; std::mem::size_of::<T>()]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_simple`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:15:10
   |
   |
LL |     [u8; std::mem::size_of::<T>()]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); std::mem::size_of::<T>()]:`
note: required by a bound in `test_simple`
   |
   |
LL | fn test_simple<T>() -> [u8; std::mem::size_of::<T>()]
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_simple`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:22:10
   |
   |
LL |     [u8; test_me::<T>(N, N + 1) + N]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); test_me::<T>(N, N + 1) + N]:`
note: required by a bound in `test_with_args`
   |
   |
LL | fn test_with_args<T, const N: usize>() -> [u8; test_me::<T>(N, N + 1) + N]
LL | where
LL | where
LL |     [u8; test_me::<T>(N, N + 1) + N]: Sized,
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_with_args`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/fn_call.rs:22:10
   |
   |
LL |     [u8; test_me::<T>(N, N + 1) + N]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); test_me::<T>(N, N + 1) + N]:`
note: required by a bound in `test_with_args`
   |
   |
LL | fn test_with_args<T, const N: usize>() -> [u8; test_me::<T>(N, N + 1) + N]
   |                                                ^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `test_with_args`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/infer-too-generic/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/infer-too-generic/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs:9:9
   |
   |
LL |     [T; N - 1]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `split_first`
   |
   |
LL | fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; N - 1])
LL | where
LL | where
LL |     [T; N - 1]: Sized,
   |         ^^^^^ required by this bound in `split_first`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/infer-too-generic.rs:9:9
   |
   |
LL |     [T; N - 1]: Sized,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); N - 1]:`
note: required by a bound in `split_first`
   |
   |
LL | fn split_first<T, const N: usize>(arr: [T; N]) -> (T, [T; N - 1])
   |                                                           ^^^^^ required by this bound in `split_first`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/issue-84408.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-84408" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/issue-84408/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:28:5
   |
   |
LL |     [(); A + B]: ,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { A + B }]:`
note: required by a bound in `<Bar<A, B> as Melon<{ A + B }>>`
   |
   |
LL | impl<const A: usize, const B: usize> Melon<{ A + B }> for Bar<A, B>
   |                                            ^^^^^^^^^ required by this bound in `<Bar<A, B> as Melon<{ A + B }>>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:28:5
   |
   |
LL |     [(); A + B]: ,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `<Bar<A, B> as Melon<{ A + B }>>`
   |
   |
LL |     [(); A + B]: ,
   |          ^^^^^ required by this bound in `<Bar<A, B> as Melon<{ A + B }>>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:26:59
   |
   |
LL | impl<const A: usize, const B: usize> Melon<{ A + B }> for Bar<A, B>
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `Bar`
   |
   |
LL | struct Bar<const A: usize, const B: usize>([i32; A + B])
LL | where
LL | where
LL |     [(); A + B]: ;
   |          ^^^^^ required by this bound in `Bar`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:26:38
   |
   |
LL | impl<const A: usize, const B: usize> Melon<{ A + B }> for Bar<A, B>
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { A + B }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:28:5
   |
   |
LL |     [(); A + B]: ,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { A + B }]:`
note: required by a bound in `<Bar<A, B> as Melon<{ A + B }>>`
   |
   |
LL | impl<const A: usize, const B: usize> Melon<{ A + B }> for Bar<A, B>
   |                                            ^^^^^^^^^ required by this bound in `<Bar<A, B> as Melon<{ A + B }>>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:28:5
   |
   |
LL |     [(); A + B]: ,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `<Bar<A, B> as Melon<{ A + B }>>`
   |
   |
LL |     [(); A + B]: ,
   |          ^^^^^ required by this bound in `<Bar<A, B> as Melon<{ A + B }>>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:30:23
   |
   |
LL |     fn new(arr: [i32; A + B]) -> Self {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `<Bar<A, B> as Melon<{ A + B }>>::new`
   |
   |
LL |     fn new(arr: [i32; A + B]) -> Self {
   |                       ^^^^^ required by this bound in `<Bar<A, B> as Melon<{ A + B }>>::new`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:30:34
   |
   |
LL |     fn new(arr: [i32; A + B]) -> Self {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `Bar`
   |
   |
LL | struct Bar<const A: usize, const B: usize>([i32; A + B])
LL | where
LL | where
LL |     [(); A + B]: ;
   |          ^^^^^ required by this bound in `Bar`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:28:5
   |
   |
LL |     [(); A + B]: ,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { A + B }]:`
note: required by a bound in `<Bar<A, B> as Melon<{ A + B }>>`
   |
   |
LL | impl<const A: usize, const B: usize> Melon<{ A + B }> for Bar<A, B>
   |                                            ^^^^^^^^^ required by this bound in `<Bar<A, B> as Melon<{ A + B }>>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:28:5
   |
   |
LL |     [(); A + B]: ,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `<Bar<A, B> as Melon<{ A + B }>>`
   |
   |
LL |     [(); A + B]: ,
   |          ^^^^^ required by this bound in `<Bar<A, B> as Melon<{ A + B }>>`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:33:24
   |
   |
LL |     fn change<T: Melon<{ A + B }>>(self) -> T {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { A + B }]:`
note: required by a bound in `<Bar<A, B> as Melon<{ A + B }>>::change`
   |
   |
LL |     fn change<T: Melon<{ A + B }>>(self) -> T {
   |                        ^^^^^^^^^ required by this bound in `<Bar<A, B> as Melon<{ A + B }>>::change`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:33:36
   |
   |
LL |     fn change<T: Melon<{ A + B }>>(self) -> T {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); A + B]:`
note: required by a bound in `Bar`
   |
   |
LL | struct Bar<const A: usize, const B: usize>([i32; A + B])
LL | where
LL | where
LL |     [(); A + B]: ;
   |          ^^^^^ required by this bound in `Bar`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/issue-84408.rs:33:18
   |
   |
LL |     fn change<T: Melon<{ A + B }>>(self) -> T {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { A + B }]:`
error: aborting due to 13 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/less_than.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/less_than/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/less_than/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:7:56
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |                                                        ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:7:56
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
note: required by a bound in `test`
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |                                  ^^^^^^^^^^ required by this bound in `test`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:7:30
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
error: unconstrained generic constant
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/less_than.rs:7:69
   |
   |
LL | fn test<const N: usize>() -> Foo<{ N > 10 }> where Foo<{ N > 10 }>: Sized {
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { N > 10 }]:`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/const-generics/generic_const_exprs/nested_uneval_unification-1.rs:17:5
   |
   |
LL |     [(); { M * 2 } + 1]: ,
   |
   |
   = help: try adding a `where` bound using this expression: `where [(); { M * 2 }]:`
note: required by a bound in `substs2`
   |
   |
LL | fn substs2<const M: usize>() -> Substs1<{ M * 2 }>
   |                                         ^^^^^^^^^ required by this bound in `substs2`
error: unconstrained generic constant
