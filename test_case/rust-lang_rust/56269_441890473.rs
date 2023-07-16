
error: unsatisfied lifetime constraints
   --> src/librustc_mir/hair/pattern/_match.rs:341:9
    |
317 |   impl<'a, 'tcx> MatchCheckCtxt<'a, 'tcx> {
    |        -- lifetime `'a` defined here
...
335 |       fn lower_byte_str_pattern<'p>(&mut self, pat: &'p Pattern<'tcx>)
    |                                 -- lifetime `'p` defined here
...
341 | /         self.byte_array_map.entry(pat).or_insert_with(|| {
342 | |             match pat.kind {
343 | |                 box PatternKind::Constant {
344 | |                     value: const_val
...   |
372 | |             }
373 | |         }).clone()
    | |__________________^ returning this value requires that `'p` must outlive `'a`

error[E0597]: `wild_patterns_owned` does not live long enough
    --> src/librustc_mir/hair/pattern/_match.rs:1254:33
     |
1254 |     let wild_patterns: Vec<_> = wild_patterns_owned.iter().collect();
     |                                 ^^^^^^^^^^^^^^^^^^^ borrowed value does not live long enough
...
1258 |     match specialize(cx, v, &ctor, &wild_patterns) {
     |           ---------------------------------------- a temporary with access to the borrow is created here ...
...
1269 | }
     | -
     | |
     | `wild_patterns_owned` dropped here while still borrowed
     | ... and the borrow might be used here, when that temporary is dropped and runs the destructor for type `std::option::Option<smallvec::SmallVec<[&hair::pattern::Pattern<'_>; 2]>>`
     |
     = note: The temporary is part of an expression at the end of a block. Consider forcing this temporary to be dropped sooner, before the block's local variables are dropped. For example, you could save the expression's value in a new local variable `x` and then make `x` be the expression at the end of the block.

error[E0597]: `wild_pattern` does not live long enough
   --> src/librustc_mir/hair/pattern/check_match.rs:287:56
    |
287 |             let witness = match is_useful(cx, &pats, &[&wild_pattern], ConstructWitness) {
    |                                                        ^^^^^^^^^^^^^ borrowed value does not live long enough
...
309 |         });
    |         -
    |         |
    |         `wild_pattern` dropped here while still borrowed
    |         borrow might be used here, when `pats` is dropped and runs the destructor for type `hair::pattern::_match::Matrix<'_, '_>`
    |
    = note: values in a scope are dropped in the opposite order they are defined

error[E0597]: `wild_pattern` does not live long enough
   --> src/librustc_mir/hair/pattern/check_match.rs:475:35
    |
466 | fn check_exhaustive<'a, 'tcx>(cx: &mut MatchCheckCtxt<'a, 'tcx>,
    |                     -- lifetime `'a` defined here
...
475 |     match is_useful(cx, matrix, &[&wild_pattern], ConstructWitness) {
    |           ------------------------^^^^^^^^^^^^^--------------------
    |           |                       |
    |           |                       borrowed value does not live long enough
    |           argument requires that `wild_pattern` is borrowed for `'a`
...
514 | }
    | - `wild_pattern` dropped here while still borrowed
