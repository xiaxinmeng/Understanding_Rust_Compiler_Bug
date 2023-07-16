plain
running 38 tests
iiiiiiii.iiii.ii.ii.i......F..........
failures:

---- src/fn_ctxt/checks.rs - fn_ctxt::checks::FnCtxt<'a,'tcx>::point_at_specific_expr_if_possible_for_derived_predicate_obligation (line 1984) stdout ----
error[E0405]: cannot find trait `Delicious` in this scope
 --> src/fn_ctxt/checks.rs:1988:27
  |
6 | impl <Filling: Delicious> Delicious for Burrito<Filling> {}


error[E0405]: cannot find trait `Delicious` in this scope
 --> src/fn_ctxt/checks.rs:1988:16
  |
6 | impl <Filling: Delicious> Delicious for Burrito<Filling> {}


error[E0405]: cannot find trait `Delicious` in this scope
 --> src/fn_ctxt/checks.rs:1989:29
  |
7 | fn eat_delicious_food<Food: Delicious>(food: Food) {}


error[E0425]: cannot find value `Kale` in this scope
  --> src/fn_ctxt/checks.rs:1992:41
   |
10 |   eat_delicious_food(Burrito { filling: Kale });

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0405, E0425.
Some errors have detailed explanations: E0405, E0425.
For more information about an error, try `rustc --explain E0405`.
Couldn't compile the test.

failures:
    src/fn_ctxt/checks.rs - fn_ctxt::checks::FnCtxt<'a,'tcx>::point_at_specific_expr_if_possible_for_derived_predicate_obligation (line 1984)
test result: FAILED. 20 passed; 1 failed; 17 ignored; 0 measured; 0 filtered out; finished in 0.30s


error: doctest failed, to rerun pass `-p rustc_hir_typeck --doc`
