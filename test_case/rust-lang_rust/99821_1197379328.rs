plain
 finished in 0.607 seconds
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 157 tests
......................................F..................F.............................. 88/157
Some tests failed in compiletest suite=incremental mode=incremental host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:

---- [incremental] src/test/incremental/hashes/enum_defs.rs stdout ----


error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/enum_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/enum_defs.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/enum_defs/auxiliary"
stdout: none
--- stderr -------------------------------
error: `generics_of(EnumAddLifetimeBoundToParameter)` should be dirty but is not
   |
   |
LL | enum EnumAddLifetimeBoundToParameter<'a, T: 'a> {


error: `generics_of(EnumAddLifetimeBoundToParameterWhere)` should be dirty but is not
   |
   |
LL | enum EnumAddLifetimeBoundToParameterWhere<'a, T> where T: 'a {

error: aborting due to 2 previous errors
------------------------------------------



---- [incremental] src/test/incremental/hashes/trait_defs.rs stdout ----

error in revision `cfail2`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/trait_defs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail2" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/trait_defs.inc" "-Z" "incremental-verify-ich" "-O" "--error-format" "json" "--json" "future-incompat" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-O" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/trait_defs/auxiliary"
stdout: none
--- stderr -------------------------------
error: `generics_of(TraitAddLifetimeBoundToTypeParameterOfTrait)` should be dirty but is not
   |
   |
LL | trait TraitAddLifetimeBoundToTypeParameterOfTrait<'a, T: 'a> { }


error: `generics_of(TraitAddSecondLifetimeBoundToTypeParameterOfTrait)` should be dirty but is not
   |
   |
LL | trait TraitAddSecondLifetimeBoundToTypeParameterOfTrait<'a, 'b, T: 'a + 'b> { }


error: `generics_of(TraitAddLifetimeBoundToTypeParameterOfTraitWhere)` should be dirty but is not
   |
   |
LL | trait TraitAddLifetimeBoundToTypeParameterOfTraitWhere<'a, T> where T: 'a { }


error: `generics_of(TraitAddSecondLifetimeBoundToTypeParameterOfTraitWhere)` should be dirty but is not
   |
   |
LL | trait TraitAddSecondLifetimeBoundToTypeParameterOfTraitWhere<'a, 'b, T> where T: 'a + 'b { }

error: aborting due to 4 previous errors
------------------------------------------

